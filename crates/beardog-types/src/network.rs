

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub address: String,
    pub port: u16,
    pub timeout_ms: u32,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            address: "localhost".to_string(),
            port: 8080,
            timeout_ms: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    WebSocket,
}

impl Default for NetworkProtocol {
    fn default() -> Self {
        Self::Http
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub protocol: NetworkProtocol,
    pub config: EndpointConfig,
    pub enabled: bool,
}

impl Default for ServiceEndpoint {
    fn default() -> Self {
        Self {
            protocol: NetworkProtocol::default(),
            config: EndpointConfig::default(),
            enabled: true,
        }
    }
}

