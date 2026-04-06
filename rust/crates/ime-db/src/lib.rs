use std::path::{Path, PathBuf};

use anyhow::Result;
use ime_config::AppConfig;
use rusqlite::{Connection, OptionalExtension, params};

pub const DEFAULT_SCHEMA_SQL: &str = include_str!("../../../sql/0001_init.sql");
const APP_CONFIG_KEY: &str = "app";

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
}
