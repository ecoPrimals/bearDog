

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub name: String,
    pub enabled: bool,
    pub connection_timeout: Duration,
    pub retry_attempts: u32,
}

impl Default for AdapterConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            enabled: true,
            connection_timeout: Duration::from_secs(30),
            retry_attempts: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterPerformanceConfig {
    pub max_connections: u32,
    pub connection_pool_size: u32,
    pub request_timeout: Duration,
}

impl Default for AdapterPerformanceConfig {
    fn default() -> Self {
        Self {
            max_connections: 100,
            connection_pool_size: 10,
            request_timeout: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConnectionPooling {
    pub enabled: bool,
    pub min_connections: u32,
    pub max_connections: u32,
}

impl Default for AdapterConnectionPooling {
    fn default() -> Self {
        Self {
            enabled: true,
            min_connections: 1,
            max_connections: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdapterAuthMethod {
    None,
    Token,
    ApiKey,
}

impl Default for AdapterAuthMethod {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AdapterSecurityPolicy {
    pub auth_method: AdapterAuthMethod,
    pub allowed_hosts: Vec<String>,
}

