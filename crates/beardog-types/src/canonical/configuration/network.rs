

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkConfig {

    pub communication_mesh: CommunicationMeshConfig,

    pub security: NetworkSecurityConfig,

    pub ports: PortConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationMeshConfig {

    pub enabled: bool,

    pub nodes: Vec<String>,

    pub discovery_interval: Duration,}

impl Default for CommunicationMeshConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            nodes: vec!["localhost:8080".to_string(), "localhost:8081".to_string()],
            discovery_interval: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSecurityConfig {

    pub tls_enabled: bool,

    pub cert_path: Option<String>,

    pub key_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfig {

    pub http: u16,

    pub https: u16,

    pub grpc: u16,
}

impl Default for PortConfig {
    fn default() -> Self {
        Self {
            http: 8080,
            https: 8443,
            grpc: 9090,
        }
    }
}
