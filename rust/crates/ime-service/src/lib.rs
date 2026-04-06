use std::path::{Path, PathBuf};

use anyhow::Result;
use ime_config::AppConfig;
use ime_core::{EngineResponse, ImeEngine, KeyEvent, SessionState};
use ime_db::{
    Database, ErrorLogEntry, HotkeyEntry, InputHistoryEntry, NewErrorLogEntry,
    NewUserDictionaryEntry, UserDictionaryEntry,
};
use ime_dict::MemoryDictionary;
use ime_ipc::RuntimeStatusResponse;
use ime_platform_api::InputMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TypingSnapshot {
    pub session: SessionState,
    pub response: EngineResponse,
}

#[derive(Debug, Clone)]
pub struct ServiceRuntime {
    db: Database,
}

impl ServiceRuntime {
    pub fn new(path: impl Into<PathBuf>) -> Result<Self> {
        let db = Database::new(path.into());
        db.initialize()?;
        Ok(Self { db })
    }

    pub fn get_config(&self) -> Result<AppConfig> {
        self.db.load_config()
    }

    pub fn update_config(&self, config: AppConfig) -> Result<RuntimeStatusResponse> {
        self.db.save_config(&config)?;
        Ok(self.runtime_status_from(&config))
    }

    pub fn reset_config(&self) -> Result<AppConfig> {
        let config = AppConfig::default();
        self.db.save_config(&config)?;
        Ok(config)
    }

    pub fn get_runtime_status(&self) -> Result<RuntimeStatusResponse> {
        let config = self.get_config()?;
        Ok(self.runtime_status_from(&config))
    }

    pub fn list_user_dictionary_entries(&self) -> Result<Vec<UserDictionaryEntry>> {
        self.db.list_user_dictionary_entries()
    }

    pub fn create_user_dictionary_entry(
        &self,
        entry: NewUserDictionaryEntry,
    ) -> Result<UserDictionaryEntry> {
        self.db.create_user_dictionary_entry(entry)
    }

    pub fn delete_user_dictionary_entry(&self, id: i64) -> Result<Vec<UserDictionaryEntry>> {
        self.db.delete_user_dictionary_entry(id)?;
        self.list_user_dictionary_entries()
    }

    pub fn list_input_history_entries(&self) -> Result<Vec<InputHistoryEntry>> {
        self.db.list_input_history_entries()
    }

    pub fn list_hotkeys(&self) -> Result<Vec<HotkeyEntry>> {
        self.db.list_hotkeys()
    }

    pub fn save_hotkey(&self, entry: HotkeyEntry) -> Result<Vec<HotkeyEntry>> {
        self.db.save_hotkey(entry)?;
        self.list_hotkeys()
    }

    pub fn delete_hotkey(&self, id: &str) -> Result<Vec<HotkeyEntry>> {
        self.db.delete_hotkey(id)?;
        self.list_hotkeys()
    }

    pub fn list_error_logs(&self) -> Result<Vec<ErrorLogEntry>> {
        self.db.list_error_logs()
    }

    pub fn append_error_log(&self, entry: NewErrorLogEntry) -> Result<ErrorLogEntry> {
        self.db.append_error_log(entry)
    }

    pub fn create_typing_session(&self) -> Result<SessionState> {
        let config = self.get_config()?;
        let input_mode = if config.input.english_mode_by_default {
            InputMode::English
        } else {
            InputMode::Chinese
        };

        Ok(SessionState {
            input_mode,
            ..SessionState::default()
        })
    }

    pub fn process_typing_key(
        &self,
        mut session: SessionState,
        event: KeyEvent,
    ) -> Result<TypingSnapshot> {
        let engine = ImeEngine::new(MemoryDictionary);
        let response = engine.handle_key_event(&mut session, event);

        Ok(TypingSnapshot { session, response })
    }

    fn runtime_status_from(&self, config: &AppConfig) -> RuntimeStatusResponse {
        RuntimeStatusResponse {
            service_status: "就绪".to_string(),
            active_platform: "Windows TSF 脚手架".to_string(),
            default_schema: config.input.default_schema.clone(),
        }
    }
}

impl From<&Path> for ServiceRuntime {
    fn from(value: &Path) -> Self {
        Self::new(value.to_path_buf()).expect("service runtime from path")
    }
}

#[cfg(test)]
mod tests {
    use crate::ServiceRuntime;
    use ime_core::KeyEvent;
    use ime_db::NewUserDictionaryEntry;
    use ime_platform_api::InputMode;

    #[test]
    fn update_config_persists_and_returns_runtime_snapshot() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");
        let mut config = service.get_config().expect("load config");
        config.input.default_schema = "wubi".to_string();

        let runtime = service.update_config(config.clone()).expect("save config");

        assert_eq!(runtime.default_schema, "wubi");

        let loaded = service.get_config().expect("reload config");
        assert_eq!(loaded.input.default_schema, "wubi");
    }

    #[test]
    fn reset_config_restores_defaults() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");
        let mut config = service.get_config().expect("load config");
        config.input.default_schema = "wubi".to_string();
        service.update_config(config).expect("save custom config");

        let reset = service.reset_config().expect("reset config");

        assert_eq!(reset, ime_config::AppConfig::default());
        let loaded = service.get_config().expect("reload config");
        assert_eq!(loaded, ime_config::AppConfig::default());
    }

    #[test]
    fn service_runtime_can_create_and_list_dictionary_entries() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");

        service
            .create_user_dictionary_entry(NewUserDictionaryEntry {
                schema_id: "pinyin".to_string(),
                code: "nihao".to_string(),
                word: "你好".to_string(),
                weight: 1.0,
                source: "manual".to_string(),
            })
            .expect("create entry");

        assert_eq!(
            service
                .list_user_dictionary_entries()
                .expect("list entries")
                .len(),
            1
        );
    }

    #[test]
    fn typing_session_respects_default_input_mode() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");
        let mut config = service.get_config().expect("load config");
        config.input.english_mode_by_default = true;
        service.update_config(config).expect("save config");

        let session = service
            .create_typing_session()
            .expect("create typing session");

        assert_eq!(session.input_mode, InputMode::English);
    }

    #[test]
    fn typing_key_processing_updates_session_and_commit_text() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");
        let session = service
            .create_typing_session()
            .expect("create typing session");

        let snapshot = service
            .process_typing_key(session, KeyEvent::Char('s'))
            .expect("process typing key");

        assert_eq!(snapshot.response.preedit.composition_text, "s");
        assert_eq!(snapshot.session.raw_keys, "s");

        let committed = service
            .process_typing_key(snapshot.session, KeyEvent::Number(1))
            .expect("process candidate selection");

        assert_eq!(committed.response.commit_text.as_deref(), Some("shurufa"));
        assert!(committed.session.raw_keys.is_empty());
    }

    #[test]
    fn service_runtime_can_save_and_delete_hotkeys() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");

        let saved = service
            .save_hotkey(ime_db::HotkeyEntry {
                id: "toggle-input-mode".to_string(),
                action: "toggleInputMode".to_string(),
                accelerator: "Ctrl+Space".to_string(),
                scope: "global".to_string(),
                enabled: true,
                updated_at: 0,
            })
            .expect("save hotkey");

        assert_eq!(saved.len(), 1);

        let deleted = service
            .delete_hotkey("toggle-input-mode")
            .expect("delete hotkey");

        assert!(deleted.is_empty());
    }

    #[test]
    fn service_runtime_can_append_and_list_error_logs() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");

        service
            .append_error_log(ime_db::NewErrorLogEntry {
                level: "error".to_string(),
                module: "ime-service".to_string(),
                message: "boom".to_string(),
                context_json: Some("{\"phase\":\"test\"}".to_string()),
            })
            .expect("append log");

        let logs = service.list_error_logs().expect("list logs");

        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].message, "boom");
        assert_eq!(logs[0].context_json.as_deref(), Some("{\"phase\":\"test\"}"));
    }

    #[test]
    fn toggling_input_mode_clears_existing_typing_session_state() {
        let file = tempfile::NamedTempFile::new().expect("temp db");
        let service = ServiceRuntime::new(file.path()).expect("service runtime");
        let session = service
            .create_typing_session()
            .expect("create typing session");

        let snapshot = service
            .process_typing_key(session, KeyEvent::Char('s'))
            .expect("process char");

        let toggled = service
            .process_typing_key(snapshot.session, KeyEvent::ToggleInputMode)
            .expect("toggle mode");

        assert_eq!(toggled.response.input_mode, InputMode::English);
        assert!(toggled.session.raw_keys.is_empty());
        assert!(toggled.response.preedit.composition_text.is_empty());
    }
}
