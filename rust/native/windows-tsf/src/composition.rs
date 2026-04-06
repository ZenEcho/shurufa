use crate::response::HostAction;
use ime_platform_api::PreeditState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositionAction {
    Start { text: String, cursor: usize },
    Update { text: String, cursor: usize },
    Commit(String),
    End,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompositionState {
    pub active_text: Option<String>,
    pub cursor: usize,
}

impl CompositionState {
    pub fn apply_host_action(&mut self, action: &HostAction) -> Vec<CompositionAction> {
        match action {
            HostAction::UpdatePreedit(preedit) => self.apply_preedit(preedit),
            HostAction::CommitText(text) => self.apply_commit(text),
            HostAction::ClearSession => self.clear(),
            _ => Vec::new(),
        }
    }

    fn apply_preedit(&mut self, preedit: &PreeditState) -> Vec<CompositionAction> {
        if preedit.composition_text.is_empty() {
            return self.clear();
        }

        let action = if self.active_text.is_some() {
            CompositionAction::Update {
                text: preedit.composition_text.clone(),
                cursor: preedit.cursor,
            }
        } else {
            CompositionAction::Start {
                text: preedit.composition_text.clone(),
                cursor: preedit.cursor,
            }
        };

        self.active_text = Some(preedit.composition_text.clone());
        self.cursor = preedit.cursor;

        vec![action]
    }

    fn apply_commit(&mut self, text: &str) -> Vec<CompositionAction> {
        let mut actions = Vec::new();

        if self.active_text.is_some() {
            actions.push(CompositionAction::End);
        }

        actions.push(CompositionAction::Commit(text.to_string()));
        self.active_text = None;
        self.cursor = 0;

        actions
    }

    fn clear(&mut self) -> Vec<CompositionAction> {
        if self.active_text.take().is_some() {
            self.cursor = 0;
            return vec![CompositionAction::End];
        }

        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{CompositionAction, CompositionState};
    use crate::response::HostAction;
    use ime_platform_api::PreeditState;

    #[test]
    fn starts_composition_when_preedit_first_appears() {
        let mut state = CompositionState::default();

        let actions = state.apply_host_action(&HostAction::UpdatePreedit(PreeditState {
            composition_text: "ni".to_string(),
            cursor: 2,
        }));

        assert_eq!(
            actions,
            vec![CompositionAction::Start {
                text: "ni".to_string(),
                cursor: 2,
            }]
        );
    }

    #[test]
    fn updates_existing_composition_when_preedit_changes() {
        let mut state = CompositionState {
            active_text: Some("ni".to_string()),
            cursor: 2,
        };

        let actions = state.apply_host_action(&HostAction::UpdatePreedit(PreeditState {
            composition_text: "nihao".to_string(),
            cursor: 5,
        }));

        assert_eq!(
            actions,
            vec![CompositionAction::Update {
                text: "nihao".to_string(),
                cursor: 5,
            }]
        );
    }

    #[test]
    fn ends_composition_before_commit() {
        let mut state = CompositionState {
            active_text: Some("nihao".to_string()),
            cursor: 5,
        };

        let actions = state.apply_host_action(&HostAction::CommitText("你好".to_string()));

        assert_eq!(
            actions,
            vec![
                CompositionAction::End,
                CompositionAction::Commit("你好".to_string()),
            ]
        );
    }

    #[test]
    fn clear_session_ends_active_composition() {
        let mut state = CompositionState {
            active_text: Some("nihao".to_string()),
            cursor: 5,
        };

        let actions = state.apply_host_action(&HostAction::ClearSession);

        assert_eq!(actions, vec![CompositionAction::End]);
    }
}
