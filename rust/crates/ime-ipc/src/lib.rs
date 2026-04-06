use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceCommand {
    GetConfig,
    GetRuntimeStatus,
    ReloadDictionary,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeStatusResponse {
    pub service_status: String,
    pub active_platform: String,
    pub default_schema: String,
}
