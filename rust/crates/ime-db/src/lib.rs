use std::path::{Path, PathBuf};

use anyhow::Result;
use ime_config::AppConfig;
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};

pub const DEFAULT_SCHEMA_SQL: &str = include_str!("../../../sql/0001_init.sql");
const APP_CONFIG_KEY: &str = "app";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserDictionaryEntry {
    pub id: i64,
    pub schema_id: String,
    pub code: String,
    pub word: String,
    pub weight: f64,
    pub source: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewUserDictionaryEntry {
    pub schema_id: String,
    pub code: String,
    pub word: String,
    pub weight: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputHistoryEntry {
    pub id: i64,
    pub schema_id: String,
    pub input_code: String,
    pub committed_text: String,
    pub usage_count: i64,
    pub last_used_at: i64,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotkeyEntry {
    pub id: String,
    pub action: String,
    pub accelerator: String,
    pub scope: String,
    pub enabled: bool,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ErrorLogEntry {
    pub id: i64,
    pub level: String,
    pub module: String,
    pub message: String,
    pub context_json: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewErrorLogEntry {
    pub level: String,
    pub module: String,
    pub message: String,
    pub context_json: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub path: PathBuf,
}

impl Database {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn initialize(&self) -> Result<()> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;

        if self.load_raw_config(&connection)?.is_none() {
            self.save_config_with_connection(&connection, &AppConfig::default())?;
        }

        Ok(())
    }

    pub fn load_config(&self) -> Result<AppConfig> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;

        match self.load_raw_config(&connection)? {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(AppConfig::default()),
        }
    }

    pub fn save_config(&self, config: &AppConfig) -> Result<()> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        self.save_config_with_connection(&connection, config)
    }

    pub fn list_user_dictionary_entries(&self) -> Result<Vec<UserDictionaryEntry>> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        let mut statement = connection.prepare(
            "SELECT id, schema_id, code, word, weight, source, created_at, updated_at
             FROM user_dictionary
             ORDER BY updated_at DESC, id DESC",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(UserDictionaryEntry {
                id: row.get(0)?,
                schema_id: row.get(1)?,
                code: row.get(2)?,
                word: row.get(3)?,
                weight: row.get(4)?,
                source: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn create_user_dictionary_entry(
        &self,
        entry: NewUserDictionaryEntry,
    ) -> Result<UserDictionaryEntry> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        connection.execute(
            "INSERT INTO user_dictionary (schema_id, code, word, weight, source, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, unixepoch(), unixepoch())",
            params![
                entry.schema_id,
                entry.code,
                entry.word,
                entry.weight,
                entry.source
            ],
        )?;

        let id = connection.last_insert_rowid();
        self.get_user_dictionary_entry(&connection, id)
    }

    pub fn delete_user_dictionary_entry(&self, id: i64) -> Result<()> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        connection.execute("DELETE FROM user_dictionary WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_input_history_entries(&self) -> Result<Vec<InputHistoryEntry>> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        let mut statement = connection.prepare(
            "SELECT id, schema_id, input_code, committed_text, usage_count, last_used_at, created_at
             FROM input_history
             ORDER BY last_used_at DESC, id DESC",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(InputHistoryEntry {
                id: row.get(0)?,
                schema_id: row.get(1)?,
                input_code: row.get(2)?,
                committed_text: row.get(3)?,
                usage_count: row.get(4)?,
                last_used_at: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;

        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn list_hotkeys(&self) -> Result<Vec<HotkeyEntry>> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        let mut statement = connection.prepare(
            "SELECT id, action, accelerator, scope, enabled, updated_at
             FROM hotkeys
             ORDER BY updated_at DESC, id ASC",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(HotkeyEntry {
                id: row.get(0)?,
                action: row.get(1)?,
                accelerator: row.get(2)?,
                scope: row.get(3)?,
                enabled: row.get::<_, i64>(4)? != 0,
                updated_at: row.get(5)?,
            })
        })?;

        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn save_hotkey(&self, entry: HotkeyEntry) -> Result<()> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        connection.execute(
            "INSERT INTO hotkeys (id, action, accelerator, scope, enabled, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, unixepoch())
             ON CONFLICT(id) DO UPDATE SET
               action = excluded.action,
               accelerator = excluded.accelerator,
               scope = excluded.scope,
               enabled = excluded.enabled,
               updated_at = excluded.updated_at",
            params![
                entry.id,
                entry.action,
                entry.accelerator,
                entry.scope,
                if entry.enabled { 1 } else { 0 }
            ],
        )?;
        Ok(())
    }

    pub fn delete_hotkey(&self, id: &str) -> Result<()> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        connection.execute("DELETE FROM hotkeys WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn list_error_logs(&self) -> Result<Vec<ErrorLogEntry>> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        let mut statement = connection.prepare(
            "SELECT id, level, module, message, context_json, created_at
             FROM error_logs
             ORDER BY created_at DESC, id DESC",
        )?;
        let rows = statement.query_map([], |row| {
            Ok(ErrorLogEntry {
                id: row.get(0)?,
                level: row.get(1)?,
                module: row.get(2)?,
                message: row.get(3)?,
                context_json: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    pub fn append_error_log(&self, entry: NewErrorLogEntry) -> Result<ErrorLogEntry> {
        let connection = self.open()?;
        connection.execute_batch(DEFAULT_SCHEMA_SQL)?;
        connection.execute(
            "INSERT INTO error_logs (level, module, message, context_json, created_at)
             VALUES (?1, ?2, ?3, ?4, unixepoch())",
            params![entry.level, entry.module, entry.message, entry.context_json],
        )?;

        let id = connection.last_insert_rowid();
        self.get_error_log_entry(&connection, id)
    }

    fn open(&self) -> Result<Connection> {
        Ok(Connection::open(&self.path)?)
    }

    fn load_raw_config(&self, connection: &Connection) -> Result<Option<String>> {
        let value = connection
            .query_row(
                "SELECT value_json FROM app_config WHERE key = ?1",
                params![APP_CONFIG_KEY],
                |row| row.get::<_, String>(0),
            )
            .optional()?;

        Ok(value)
    }

    fn save_config_with_connection(
        &self,
        connection: &Connection,
        config: &AppConfig,
    ) -> Result<()> {
        let json = serde_json::to_string(config)?;
        connection.execute(
            "INSERT INTO app_config (key, value_json, updated_at)
             VALUES (?1, ?2, unixepoch())
             ON CONFLICT(key) DO UPDATE SET
                 value_json = excluded.value_json,
                 updated_at = excluded.updated_at",
            params![APP_CONFIG_KEY, json],
        )?;

        Ok(())
    }

    fn get_user_dictionary_entry(
        &self,
        connection: &Connection,
        id: i64,
    ) -> Result<UserDictionaryEntry> {
        Ok(connection.query_row(
            "SELECT id, schema_id, code, word, weight, source, created_at, updated_at
             FROM user_dictionary WHERE id = ?1",
            params![id],
            |row| {
                Ok(UserDictionaryEntry {
                    id: row.get(0)?,
                    schema_id: row.get(1)?,
                    code: row.get(2)?,
                    word: row.get(3)?,
                    weight: row.get(4)?,
                    source: row.get(5)?,
                    created_at: row.get(6)?,
                    updated_at: row.get(7)?,
                })
            },
        )?)
    }

    fn get_error_log_entry(&self, connection: &Connection, id: i64) -> Result<ErrorLogEntry> {
        Ok(connection.query_row(
            "SELECT id, level, module, message, context_json, created_at
             FROM error_logs WHERE id = ?1",
            params![id],
            |row| {
                Ok(ErrorLogEntry {
                    id: row.get(0)?,
                    level: row.get(1)?,
                    module: row.get(2)?,
                    message: row.get(3)?,
                    context_json: row.get(4)?,
                    created_at: row.get(5)?,
                })
            },
        )?)
    }
}

impl From<&Path> for Database {
    fn from(path: &Path) -> Self {
        Self::new(path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::Database;
    use ime_config::AppConfig;
    use tempfile::NamedTempFile;

    #[test]
    fn initialize_creates_schema_and_default_config() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());

        db.initialize().expect("database initialized");
        let config = db.load_config().expect("config loaded");

        assert_eq!(config, AppConfig::default());
    }

    #[test]
    fn save_config_round_trips() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        let mut config = AppConfig::default();
        config.input.default_schema = "wubi".to_string();

        db.initialize().expect("database initialized");
        db.save_config(&config).expect("config saved");

        let loaded = db.load_config().expect("config loaded");
        assert_eq!(loaded.input.default_schema, "wubi");
    }

    #[test]
    fn user_dictionary_entries_can_be_created_listed_and_deleted() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        let created = db
            .create_user_dictionary_entry(super::NewUserDictionaryEntry {
                schema_id: "pinyin".to_string(),
                code: "nihao".to_string(),
                word: "你好".to_string(),
                weight: 1.0,
                source: "manual".to_string(),
            })
            .expect("entry created");

        let entries = db.list_user_dictionary_entries().expect("entries listed");
        assert_eq!(entries.len(), 1);

        db.delete_user_dictionary_entry(created.id)
            .expect("entry deleted");
        assert!(
            db.list_user_dictionary_entries()
                .expect("entries listed")
                .is_empty()
        );
    }

    #[test]
    fn hotkeys_can_be_saved_listed_and_deleted() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        db.save_hotkey(super::HotkeyEntry {
            id: "toggle-input-mode".to_string(),
            action: "toggleInputMode".to_string(),
            accelerator: "Ctrl+Space".to_string(),
            scope: "global".to_string(),
            enabled: true,
            updated_at: 0,
        })
        .expect("hotkey saved");

        assert_eq!(db.list_hotkeys().expect("list hotkeys").len(), 1);
        db.delete_hotkey("toggle-input-mode")
            .expect("delete hotkey");
        assert!(db.list_hotkeys().expect("list hotkeys").is_empty());
    }

    #[test]
    fn history_and_logs_can_be_listed() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        db.append_error_log(super::NewErrorLogEntry {
            level: "error".to_string(),
            module: "ime-service".to_string(),
            message: "sample".to_string(),
            context_json: None,
        })
        .expect("append log");

        assert_eq!(db.list_error_logs().expect("logs listed").len(), 1);
        assert!(
            db.list_input_history_entries()
                .expect("history listed")
                .is_empty()
        );
    }

    #[test]
    fn initialize_does_not_overwrite_existing_config() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        let mut config = AppConfig::default();
        config.input.default_schema = "wubi".to_string();
        db.save_config(&config).expect("config saved");

        db.initialize().expect("database re-initialized");

        let loaded = db.load_config().expect("config loaded");
        assert_eq!(loaded.input.default_schema, "wubi");
    }

    #[test]
    fn saving_hotkey_with_same_id_updates_existing_row() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        db.save_hotkey(super::HotkeyEntry {
            id: "toggle-input-mode".to_string(),
            action: "toggleInputMode".to_string(),
            accelerator: "Ctrl+Space".to_string(),
            scope: "global".to_string(),
            enabled: true,
            updated_at: 0,
        })
        .expect("first hotkey saved");

        db.save_hotkey(super::HotkeyEntry {
            id: "toggle-input-mode".to_string(),
            action: "toggleInputMode".to_string(),
            accelerator: "Alt+Space".to_string(),
            scope: "global".to_string(),
            enabled: false,
            updated_at: 0,
        })
        .expect("second hotkey saved");

        let hotkeys = db.list_hotkeys().expect("list hotkeys");
        assert_eq!(hotkeys.len(), 1);
        assert_eq!(hotkeys[0].accelerator, "Alt+Space");
        assert!(!hotkeys[0].enabled);
    }

    #[test]
    fn appended_error_logs_preserve_context_json() {
        let file = NamedTempFile::new().expect("temp file");
        let db = Database::new(file.path().to_path_buf());
        db.initialize().expect("database initialized");

        let created = db
            .append_error_log(super::NewErrorLogEntry {
                level: "warn".to_string(),
                module: "ime-service".to_string(),
                message: "with context".to_string(),
                context_json: Some("{\"code\":1}".to_string()),
            })
            .expect("append log");

        assert_eq!(created.context_json.as_deref(), Some("{\"code\":1}"));

        let logs = db.list_error_logs().expect("logs listed");
        assert_eq!(logs[0].context_json.as_deref(), Some("{\"code\":1}"));
    }
}
