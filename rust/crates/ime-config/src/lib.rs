use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneralConfig {
    pub startup_with_system: bool,
    pub locale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InputConfig {
    pub default_schema: String,
    pub english_mode_by_default: bool,
    pub candidate_page_size: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppearanceConfig {
    pub theme: String,
    pub font_size: u8,
    pub candidate_layout: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoggingConfig {
    pub level: String,
    pub redact_input_content: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub input: InputConfig,
    pub appearance: AppearanceConfig,
    pub logging: LoggingConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                startup_with_system: false,
                locale: "zh-CN".to_string(),
            },
            input: InputConfig {
                default_schema: "pinyin".to_string(),
                english_mode_by_default: false,
                candidate_page_size: 9,
            },
            appearance: AppearanceConfig {
                theme: "system".to_string(),
                font_size: 16,
                candidate_layout: "vertical".to_string(),
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                redact_input_content: true,
            },
        }
    }
}
