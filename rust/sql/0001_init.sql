CREATE TABLE IF NOT EXISTS app_config (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS input_schema (
  id TEXT PRIMARY KEY,
  type TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  config_json TEXT NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS user_dictionary (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  schema_id TEXT NOT NULL,
  code TEXT NOT NULL,
  word TEXT NOT NULL,
  weight REAL NOT NULL DEFAULT 1,
  source TEXT NOT NULL DEFAULT 'manual',
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_user_dictionary_lookup
ON user_dictionary(schema_id, code, weight DESC);

CREATE TABLE IF NOT EXISTS input_history (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  schema_id TEXT NOT NULL,
  input_code TEXT NOT NULL,
  committed_text TEXT NOT NULL,
  usage_count INTEGER NOT NULL DEFAULT 1,
  last_used_at INTEGER NOT NULL,
  created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_input_history_lookup
ON input_history(schema_id, input_code, last_used_at DESC);

CREATE TABLE IF NOT EXISTS snippets (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  trigger TEXT NOT NULL UNIQUE,
  content TEXT NOT NULL,
  description TEXT,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS hotkeys (
  id TEXT PRIMARY KEY,
  action TEXT NOT NULL,
  accelerator TEXT NOT NULL,
  scope TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,
  updated_at INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS error_logs (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  level TEXT NOT NULL,
  module TEXT NOT NULL,
  message TEXT NOT NULL,
  context_json TEXT,
  created_at INTEGER NOT NULL
);

