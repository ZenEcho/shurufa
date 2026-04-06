use ime_dict::DictionaryProvider;
use ime_platform_api::{Candidate, CandidatePage, InputMode, PreeditState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyEvent {
    Char(char),
    Backspace,
    Enter,
    Space,
    Escape,
    Number(u8),
    ToggleInputMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineResponse {
    pub consumed: bool,
    pub commit_text: Option<String>,
    pub preedit: PreeditState,
    pub candidates: CandidatePage,
    pub input_mode: InputMode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionState {
    pub raw_keys: String,
    pub composition_text: String,
    pub selected_index: usize,
    pub candidates: Vec<Candidate>,
    pub input_mode: InputMode,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            raw_keys: String::new(),
            composition_text: String::new(),
            selected_index: 0,
            candidates: Vec::new(),
            input_mode: InputMode::Chinese,
        }
    }
}

pub struct ImeEngine<D>
where
    D: DictionaryProvider,
{
    dictionary: D,
}

impl<D> ImeEngine<D>
where
    D: DictionaryProvider,
{
    pub fn new(dictionary: D) -> Self {
        Self { dictionary }
    }

    pub fn handle_key_event(&self, session: &mut SessionState, event: KeyEvent) -> EngineResponse {
        match event {
            KeyEvent::ToggleInputMode => {
                let next_mode = match session.input_mode {
                    InputMode::Chinese => InputMode::English,
                    InputMode::English => InputMode::Chinese,
                };

                self.clear_session(session);
                session.input_mode = next_mode;
                self.response(session, true, None)
            }
            KeyEvent::Escape => {
                if session.raw_keys.is_empty() && session.candidates.is_empty() {
                    return self.response(session, false, None);
                }

                self.clear_session(session);
                self.response(session, true, None)
            }
            KeyEvent::Backspace => {
                if session.raw_keys.is_empty() {
                    return self.response(session, false, None);
                }

                session.raw_keys.pop();
                self.refresh_candidates(session);
                self.response(session, true, None)
            }
            KeyEvent::Number(number) => self.select_candidate(session, number as usize),
            KeyEvent::Space | KeyEvent::Enter => self.commit_current_selection(session),
            KeyEvent::Char(ch) => self.handle_char_input(session, ch),
        }
    }

    pub fn handle_text_input(&self, session: &mut SessionState, text: &str) -> CandidatePage {
        for ch in text.chars() {
            let _ = self.handle_key_event(session, KeyEvent::Char(ch));
        }

        CandidatePage {
            items: session.candidates.clone(),
            page_index: 0,
            has_next_page: false,
        }
    }

    pub fn preedit(&self, session: &SessionState) -> PreeditState {
        if session.composition_text.is_empty() {
            return PreeditState::empty();
        }

        PreeditState {
            composition_text: session.composition_text.clone(),
            cursor: session.composition_text.len(),
        }
    }

    pub fn clear_session(&self, session: &mut SessionState) {
        session.raw_keys.clear();
        session.composition_text.clear();
        session.selected_index = 0;
        session.candidates.clear();
    }

    fn handle_char_input(&self, session: &mut SessionState, ch: char) -> EngineResponse {
        if session.input_mode == InputMode::English {
            return EngineResponse {
                consumed: true,
                commit_text: Some(ch.to_string()),
                preedit: PreeditState::empty(),
                candidates: CandidatePage::empty(),
                input_mode: session.input_mode.clone(),
            };
        }

        if !ch.is_ascii_alphanumeric() {
            return self.response(session, false, None);
        }

        session.raw_keys.push(ch.to_ascii_lowercase());
        self.refresh_candidates(session);
        self.response(session, true, None)
    }

    fn refresh_candidates(&self, session: &mut SessionState) {
        session.composition_text = session.raw_keys.clone();
        session.candidates = self.dictionary.search(&session.raw_keys);
        session.selected_index = 0;
    }

    fn select_candidate(&self, session: &mut SessionState, number: usize) -> EngineResponse {
        if number == 0 || number > session.candidates.len() {
            return self.response(session, false, None);
        }

        let commit_text = session.candidates[number - 1].text.clone();
        self.clear_session(session);
        self.response(session, true, Some(commit_text))
    }

    fn commit_current_selection(&self, session: &mut SessionState) -> EngineResponse {
        if let Some(first_candidate) = session.candidates.first() {
            let commit_text = first_candidate.text.clone();
            self.clear_session(session);
            return self.response(session, true, Some(commit_text));
        }

        if session.input_mode == InputMode::English || session.raw_keys.is_empty() {
            return self.response(session, false, None);
        }

        let fallback = session.raw_keys.clone();
        self.clear_session(session);
        self.response(session, true, Some(fallback))
    }

    fn response(
        &self,
        session: &SessionState,
        consumed: bool,
        commit_text: Option<String>,
    ) -> EngineResponse {
        EngineResponse {
            consumed,
            commit_text,
            preedit: self.preedit(session),
            candidates: CandidatePage {
                items: session.candidates.clone(),
                page_index: 0,
                has_next_page: false,
            },
            input_mode: session.input_mode.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ImeEngine, KeyEvent, SessionState};
    use ime_dict::MemoryDictionary;
    use ime_platform_api::InputMode;

    #[test]
    fn chinese_mode_builds_preedit_and_candidates() {
        let engine = ImeEngine::new(MemoryDictionary);
        let mut session = SessionState::default();

        let response = engine.handle_key_event(&mut session, KeyEvent::Char('S'));

        assert!(response.consumed);
        assert_eq!(response.preedit.composition_text, "s");
        assert_eq!(response.candidates.items.len(), 2);
    }

    #[test]
    fn number_key_commits_selected_candidate() {
        let engine = ImeEngine::new(MemoryDictionary);
        let mut session = SessionState::default();

        let _ = engine.handle_key_event(&mut session, KeyEvent::Char('s'));
        let response = engine.handle_key_event(&mut session, KeyEvent::Number(2));

        assert_eq!(response.commit_text.as_deref(), Some("settings-center"));
        assert!(response.preedit.composition_text.is_empty());
        assert!(response.candidates.items.is_empty());
    }

    #[test]
    fn toggle_mode_switches_to_english_and_commits_directly() {
        let engine = ImeEngine::new(MemoryDictionary);
        let mut session = SessionState::default();

        let response = engine.handle_key_event(&mut session, KeyEvent::ToggleInputMode);
        assert_eq!(response.input_mode, InputMode::English);

        let response = engine.handle_key_event(&mut session, KeyEvent::Char('x'));
        assert_eq!(response.commit_text.as_deref(), Some("x"));
        assert!(response.candidates.items.is_empty());
    }
}
