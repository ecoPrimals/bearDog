

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalSystemConfig {
    pub name: String,
    pub enabled: bool,
    pub endpoint: String,
    pub timeout: Duration,
}

impl Default for ExternalSystemConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            endpoint: "http://localhost:8080".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSupportConfig {
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub websocket_enabled: bool,
}

impl Default for ProtocolSupportConfig {
    fn default() -> Self {
        Self {
            http_enabled: true,
            https_enabled: true,
            websocket_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageQueuingConfig {
    pub enabled: bool,
    pub queue_size: u32,
    pub batch_size: u32,
    pub flush_interval: Duration,
}

impl Default for MessageQueuingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            queue_size: 1000,
            batch_size: 100,
            flush_interval: Duration::from_secs(5),
        }
    }
}
