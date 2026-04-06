use crate::ContextHandle;
use crate::context_bridge::{TextExecutionPlan, TextExecutionStep};
use anyhow::Result;

#[cfg(windows)]
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use windows::Win32::Foundation::E_FAIL;
#[cfg(windows)]
use windows::Win32::UI::TextServices::{
    INSERT_TEXT_AT_SELECTION_FLAGS, ITfContext, ITfEditSession, ITfEditSession_Impl,
    ITfInsertAtSelection, TF_ES_READWRITE, TF_ES_SYNC, TF_IAS_NO_DEFAULT_COMPOSITION,
};
#[cfg(windows)]
use windows_core::{Error as WinError, Interface, Result as WinResult, implement};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TextExecutionReport {
    pub context_identity: usize,
    pub committed_texts: Vec<String>,
    pub skipped_steps: Vec<TextExecutionStep>,
}

impl TextExecutionReport {
    pub fn committed_count(&self) -> usize {
        self.committed_texts.len()
    }
}

pub fn execute_plan(
    context: &ContextHandle,
    client_id: u32,
    plan: &TextExecutionPlan,
) -> Result<TextExecutionReport> {
    #[cfg(windows)]
    {
        return execute_plan_windows(context, client_id, plan);
    }

    #[cfg(not(windows))]
    {
        let _ = (context, client_id);
        Ok(TextExecutionReport {
            context_identity: plan.context_identity,
            committed_texts: Vec::new(),
            skipped_steps: plan.steps.clone(),
        })
    }
}

#[cfg(windows)]
fn execute_plan_windows(
    context: &ContextHandle,
    client_id: u32,
    plan: &TextExecutionPlan,
) -> Result<TextExecutionReport> {
    let report = Arc::new(Mutex::new(TextExecutionReport {
        context_identity: plan.context_identity,
        committed_texts: Vec::new(),
        skipped_steps: Vec::new(),
    }));
    let session: ITfEditSession = TextEditSession::new(context.inner.clone(), plan.clone(), report.clone()).into();
    let session_result = unsafe {
        context
            .inner
            .RequestEditSession(client_id, &session, TF_ES_SYNC | TF_ES_READWRITE)?
    };
    session_result.ok()?;

    let report = report
        .lock()
        .expect("text execution report lock")
        .clone();
    Ok(report)
}

#[cfg(windows)]
#[implement(ITfEditSession)]
struct TextEditSession {
    context: ITfContext,
    plan: TextExecutionPlan,
    report: Arc<Mutex<TextExecutionReport>>,
}

#[cfg(windows)]
impl TextEditSession {
    fn new(
        context: ITfContext,
        plan: TextExecutionPlan,
        report: Arc<Mutex<TextExecutionReport>>,
    ) -> Self {
        Self {
            context,
            plan,
            report,
        }
    }
}

#[cfg(windows)]
impl ITfEditSession_Impl for TextEditSession_Impl {
    fn DoEditSession(&self, ec: u32) -> WinResult<()> {
        let insert_at_selection = self.context.cast::<ITfInsertAtSelection>().ok();
        let mut report = self.report.lock().expect("text execution report lock");

        for step in &self.plan.steps {
            match step {
                TextExecutionStep::CommitText {
                    text,
                    via_insert_at_selection,
                } if *via_insert_at_selection => {
                    let insert = insert_at_selection.as_ref().ok_or_else(|| WinError::from(E_FAIL))?;
                    let utf16: Vec<u16> = text.encode_utf16().collect();
                    unsafe {
                        insert.InsertTextAtSelection(
                            ec,
                            INSERT_TEXT_AT_SELECTION_FLAGS(
                                TF_IAS_NO_DEFAULT_COMPOSITION.0,
                            ),
                            &utf16,
                        )?;
                    }
                    report.committed_texts.push(text.clone());
                }
                TextExecutionStep::CommitText { .. }
                | TextExecutionStep::StartComposition { .. }
                | TextExecutionStep::UpdateComposition { .. }
                | TextExecutionStep::EndComposition => {
                    report.skipped_steps.push(step.clone());
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::TextExecutionReport;
    use crate::context_bridge::{TextExecutionPlan, TextExecutionStep};

    #[test]
    fn execution_report_counts_committed_texts() {
        let report = TextExecutionReport {
            context_identity: 1,
            committed_texts: vec!["书".to_string(), "入".to_string()],
            skipped_steps: Vec::new(),
        };

        assert_eq!(report.committed_count(), 2);
    }

    #[test]
    fn execution_report_can_carry_skipped_steps() {
        let report = TextExecutionReport {
            context_identity: 2,
            committed_texts: Vec::new(),
            skipped_steps: vec![TextExecutionStep::StartComposition {
                text: "s".to_string(),
                cursor: 1,
            }],
        };

        assert_eq!(report.context_identity, 2);
        assert_eq!(report.skipped_steps.len(), 1);
    }

    #[test]
    fn non_commit_steps_are_representable_in_plans() {
        let plan = TextExecutionPlan {
            context_identity: 3,
            steps: vec![
                TextExecutionStep::StartComposition {
                    text: "shu".to_string(),
                    cursor: 3,
                },
                TextExecutionStep::CommitText {
                    text: "书".to_string(),
                    via_insert_at_selection: true,
                },
            ],
        };

        assert_eq!(plan.steps.len(), 2);
    }
}
