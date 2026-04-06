use crate::ContextHandle;
use crate::composition::CompositionAction;
use anyhow::Result;
#[cfg(windows)]
use windows::Win32::UI::TextServices::{ITfContextComposition, ITfInsertAtSelection};
#[cfg(windows)]
use windows_core::Interface;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextContextCapabilities {
    pub supports_context_composition: bool,
    pub supports_insert_at_selection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundTextContext {
    pub identity: usize,
    pub capabilities: TextContextCapabilities,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextWriteRequest {
    StartComposition { text: String, cursor: usize },
    UpdateComposition { text: String, cursor: usize },
    CommitText(String),
    EndComposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextExecutionStep {
    StartComposition {
        text: String,
        cursor: usize,
    },
    UpdateComposition {
        text: String,
        cursor: usize,
    },
    CommitText {
        text: String,
        via_insert_at_selection: bool,
    },
    EndComposition,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextExecutionPlan {
    pub context_identity: usize,
    pub steps: Vec<TextExecutionStep>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TsfTextContextBridge {
    bound_context: Option<BoundTextContext>,
    write_requests: Vec<TextWriteRequest>,
}

impl TsfTextContextBridge {
    pub fn bind_context(&mut self, context: &ContextHandle) -> Result<()> {
        self.bound_context = Some(BoundTextContext {
            identity: context.raw_identity(),
            capabilities: query_context_capabilities(context)?,
        });
        Ok(())
    }

    pub fn clear_binding(&mut self) {
        self.bound_context = None;
    }

    pub fn bound_context(&self) -> Option<&BoundTextContext> {
        self.bound_context.as_ref()
    }

    pub fn write_requests(&self) -> &[TextWriteRequest] {
        &self.write_requests
    }

    pub fn clear_requests(&mut self) {
        self.write_requests.clear();
    }

    pub fn build_execution_plan(&self) -> Option<TextExecutionPlan> {
        let bound_context = self.bound_context.as_ref()?;
        let steps = self
            .write_requests
            .iter()
            .map(|request| match request {
                TextWriteRequest::StartComposition { text, cursor } => {
                    TextExecutionStep::StartComposition {
                        text: text.clone(),
                        cursor: *cursor,
                    }
                }
                TextWriteRequest::UpdateComposition { text, cursor } => {
                    TextExecutionStep::UpdateComposition {
                        text: text.clone(),
                        cursor: *cursor,
                    }
                }
                TextWriteRequest::CommitText(text) => TextExecutionStep::CommitText {
                    text: text.clone(),
                    via_insert_at_selection: bound_context
                        .capabilities
                        .supports_insert_at_selection,
                },
                TextWriteRequest::EndComposition => TextExecutionStep::EndComposition,
            })
            .collect();

        Some(TextExecutionPlan {
            context_identity: bound_context.identity,
            steps,
        })
    }

    pub fn apply_composition_action(&mut self, action: CompositionAction) {
        if self.bound_context.is_none() {
            return;
        }

        match action {
            CompositionAction::Start { text, cursor } => {
                self.write_requests
                    .push(TextWriteRequest::StartComposition { text, cursor });
            }
            CompositionAction::Update { text, cursor } => {
                self.write_requests
                    .push(TextWriteRequest::UpdateComposition { text, cursor });
            }
            CompositionAction::Commit(text) => {
                self.write_requests.push(TextWriteRequest::CommitText(text));
            }
            CompositionAction::End => {
                self.write_requests.push(TextWriteRequest::EndComposition);
            }
        }
    }
}

#[cfg(windows)]
fn query_context_capabilities(context: &ContextHandle) -> Result<TextContextCapabilities> {
    Ok(TextContextCapabilities {
        supports_context_composition: context.inner.cast::<ITfContextComposition>().is_ok(),
        supports_insert_at_selection: context.inner.cast::<ITfInsertAtSelection>().is_ok(),
    })
}

#[cfg(not(windows))]
fn query_context_capabilities(_context: &ContextHandle) -> Result<TextContextCapabilities> {
    Ok(TextContextCapabilities {
        supports_context_composition: false,
        supports_insert_at_selection: false,
    })
}

#[cfg(test)]
mod tests {
    use super::{TextExecutionStep, TextWriteRequest, TsfTextContextBridge};
    use crate::{CompositionAction, WindowsTsfHost};

    #[test]
    fn bridge_tracks_bound_context_identity() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut bridge = TsfTextContextBridge::default();

        bridge.bind_context(&context).expect("bind context");

        assert_eq!(
            bridge.bound_context().map(|item| item.identity),
            Some(context.raw_identity())
        );
    }

    #[test]
    fn bridge_collects_write_requests_only_when_context_is_bound() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut bridge = TsfTextContextBridge::default();

        bridge.apply_composition_action(CompositionAction::Commit("commit".to_string()));
        assert!(bridge.write_requests().is_empty());

        bridge.bind_context(&context).expect("bind context");
        bridge.apply_composition_action(CompositionAction::Commit("commit".to_string()));

        assert_eq!(
            bridge.write_requests(),
            &[TextWriteRequest::CommitText("commit".to_string())]
        );
    }

    #[test]
    fn bridge_builds_execution_plan_in_request_order() {
        let host = WindowsTsfHost::bootstrap().expect("host bootstrap");
        let document = host
            .create_document_manager()
            .expect("document manager created");
        let context = host.create_context(&document).expect("context created");
        let mut bridge = TsfTextContextBridge::default();

        bridge.bind_context(&context).expect("bind context");
        bridge.apply_composition_action(CompositionAction::Start {
            text: "ni".to_string(),
            cursor: 2,
        });
        bridge.apply_composition_action(CompositionAction::Commit("commit".to_string()));

        let supports_insert_at_selection = bridge
            .bound_context()
            .expect("bound context")
            .capabilities
            .supports_insert_at_selection;
        let plan = bridge.build_execution_plan().expect("execution plan");

        assert_eq!(plan.context_identity, context.raw_identity());
        assert_eq!(
            plan.steps,
            vec![
                TextExecutionStep::StartComposition {
                    text: "ni".to_string(),
                    cursor: 2,
                },
                TextExecutionStep::CommitText {
                    text: "commit".to_string(),
                    via_insert_at_selection: supports_insert_at_selection,
                },
            ]
        );
    }
}
