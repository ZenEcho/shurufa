use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InputMode {
    Chinese,
    English,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Candidate {
    pub id: String,
    pub text: String,
    pub annotation: Option<String>,
    pub hotkey: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidatePage {
    pub items: Vec<Candidate>,
    pub page_index: usize,
    pub has_next_page: bool,
}

impl CandidatePage {
    pub fn empty() -> Self {
        Self {
            items: Vec::new(),
            page_index: 0,
            has_next_page: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PreeditState {
    pub composition_text: String,
    pub cursor: usize,
}

impl PreeditState {
    pub fn empty() -> Self {
        Self {
            composition_text: String::new(),
            cursor: 0,
        }
    }
}

pub trait PlatformHost {
    fn update_preedit(&self, preedit: PreeditState);
    fn update_candidates(&self, candidates: CandidatePage);
    fn commit_text(&self, text: &str);
    fn clear_session(&self);
    fn set_mode_indicator(&self, mode: InputMode);
}
