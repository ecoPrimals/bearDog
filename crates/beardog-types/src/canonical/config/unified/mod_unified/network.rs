//! # Network Configuration Domain
//!
//! Network and communication configuration for `BearDog` including ports,
//! timeouts, SSL settings, and connection management.

use crate::canonical::config::r#trait::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    /// Enable network services
    pub enabled: bool,
    /// Host address to bind to
    pub host: String,
    /// Port number for network services
    pub port: u16,
    /// Maximum concurrent connections allowed
    pub max_connections: usize,
    /// Network timeout in seconds
    pub timeout_seconds: u64,
    /// Enable TLS/SSL encryption
    pub tls_enabled: bool,
}

impl Default for NetworkConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            host: "0.0.0.0".to_string(),
            port: 8080,
            max_connections: 1000,
            timeout_seconds: 30,
            tls_enabled: false,
        }
    }
}

impl BearDogConfig for NetworkConfiguration {
    fn validate(&self) -> BearDogResult<()> {
        if self.port == 0 {
            return Err(BearDogError::system("Network port cannot be 0".to_string()));
        }
        
        if self.max_connections == 0 {
            return Err(BearDogError::system(
                "Maximum connections must be greater than 0".to_string()
            ));
        }

        if self.timeout_seconds == 0 {
            return Err(BearDogError::system(
                "Timeout must be greater than 0".to_string()
            ));
        }

        Ok(())
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize network config: {e}")))
    }

    fn from_env() -> BearDogResult<Self> {
        Ok(Self {
            enabled: std::env::var("BEARDOG_NETWORK_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            host: std::env::var("BEARDOG_NETWORK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("BEARDOG_NETWORK_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
            max_connections: std::env::var("BEARDOG_NETWORK_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            timeout_seconds: std::env::var("BEARDOG_NETWORK_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            tls_enabled: std::env::var("BEARDOG_NETWORK_TLS_ENABLED")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
        })
    }
    
    fn merge(&self, _other: &Self) -> BearDogResult<Self> { 
        Ok(self.clone()) 
    }
    
    fn domain() -> &'static str { 
        "network" 
    }
} 