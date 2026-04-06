use ime_core::EngineResponse;
use ime_platform_api::{CandidatePage, InputMode, PreeditState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostAction {
    CommitText(String),
    ClearSession,
    UpdatePreedit(PreeditState),
    UpdateCandidates(CandidatePage),
    SetModeIndicator(InputMode),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostActionPlan {
    pub actions: Vec<HostAction>,
}

impl HostActionPlan {
    pub fn from_response(response: &EngineResponse) -> Self {
        let mut actions = Vec::new();

        if let Some(commit) = &response.commit_text {
            actions.push(HostAction::CommitText(commit.clone()));
        } else if response.preedit.composition_text.is_empty()
            && response.candidates.items.is_empty()
        {
            actions.push(HostAction::ClearSession);
        } else {
            actions.push(HostAction::UpdatePreedit(response.preedit.clone()));
            actions.push(HostAction::UpdateCandidates(response.candidates.clone()));
        }

        actions.push(HostAction::SetModeIndicator(response.input_mode.clone()));

        Self { actions }
    }
}

#[cfg(test)]
mod tests {
    use super::{HostAction, HostActionPlan};
    use ime_core::EngineResponse;
    use ime_platform_api::{Candidate, CandidatePage, InputMode, PreeditState};

    #[test]
    fn response_plan_commits_text_before_mode_indicator() {
        let response = EngineResponse {
            consumed: true,
            commit_text: Some("你好".to_string()),
            preedit: PreeditState::empty(),
            candidates: CandidatePage::empty(),
            input_mode: InputMode::Chinese,
        };

        let plan = HostActionPlan::from_response(&response);

        assert_eq!(
            plan.actions,
            vec![
                HostAction::CommitText("你好".to_string()),
                HostAction::SetModeIndicator(InputMode::Chinese),
            ]
        );
    }

    #[test]
    fn response_plan_clears_when_preedit_and_candidates_are_empty() {
        let response = EngineResponse {
            consumed: true,
            commit_text: None,
            preedit: PreeditState::empty(),
            candidates: CandidatePage::empty(),
            input_mode: InputMode::English,
        };

        let plan = HostActionPlan::from_response(&response);

        assert_eq!(
            plan.actions,
            vec![
                HostAction::ClearSession,
                HostAction::SetModeIndicator(InputMode::English),
            ]
        );
    }

    #[test]
    fn response_plan_updates_preedit_and_candidates_when_composition_exists() {
        let response = EngineResponse {
            consumed: true,
            commit_text: None,
            preedit: PreeditState {
                composition_text: "nihao".to_string(),
                cursor: 5,
            },
            candidates: CandidatePage {
                items: vec![Candidate {
                    id: "1".to_string(),
                    text: "你好".to_string(),
                    annotation: Some("system".to_string()),
                    hotkey: Some("1".to_string()),
                }],
                page_index: 0,
                has_next_page: false,
            },
            input_mode: InputMode::Chinese,
        };

        let plan = HostActionPlan::from_response(&response);

        assert_eq!(plan.actions.len(), 3);
        assert_eq!(
            plan.actions[0],
            HostAction::UpdatePreedit(PreeditState {
                composition_text: "nihao".to_string(),
                cursor: 5,
            })
        );
    }
}
