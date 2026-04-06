use ime_config::AppConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceCommand {
    GetConfig,
    SetConfig { config: AppConfig },
    ResetConfig,
    GetRuntimeStatus,
    ReloadDictionary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConfigResponse {
    pub config: AppConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeStatusResponse {
    pub service_status: String,
    pub active_platform: String,
    pub default_schema: String,
}

#[cfg(test)]
mod tests {
    use super::ServiceCommand;
    use ime_config::AppConfig;

    #[test]
    fn config_command_serializes_with_payload() {
        let command = ServiceCommand::SetConfig {
            config: AppConfig::default(),
        };

        let json = serde_json::to_string(&command).expect("serialize command");

        assert!(json.contains("SetConfig"));
    }
}
