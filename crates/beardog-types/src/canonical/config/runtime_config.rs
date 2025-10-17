//! Runtime Configuration Management
//!
//! Centralized configuration for network endpoints, ports, and other runtime values
//! that were previously hardcoded. All values can be overridden via environment variables.

use serde::{Deserialize, Serialize};
use std::env;

/// Runtime network configuration with environment variable overrides
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeNetworkConfig {
    /// Discovery endpoint URL
    pub discovery_endpoint: String,
    /// API server host
    pub api_host: String,
    /// API server port
    pub api_port: u16,
    /// Metrics endpoint port
    pub metrics_port: u16,
    /// Health check port
    pub health_port: u16,
    /// WebSocket port
    pub ws_port: u16,
    /// gRPC port
    pub grpc_port: u16,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum connections
    pub max_connections: usize,
    /// Enable TLS
    pub enable_tls: bool,
}

impl Default for RuntimeNetworkConfig {
    fn default() -> Self {
        Self {
            discovery_endpoint: env::var("BEARDOG_DISCOVERY_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080/discover".to_string()),
            api_host: env::var("BEARDOG_API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            api_port: env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            metrics_port: env::var("BEARDOG_METRICS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9090),
            health_port: env::var("BEARDOG_HEALTH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8081),
            ws_port: env::var("BEARDOG_WS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            grpc_port: env::var("BEARDOG_GRPC_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(50051),
            timeout_seconds: env::var("BEARDOG_TIMEOUT_SECONDS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30),
            max_connections: env::var("BEARDOG_MAX_CONNECTIONS")
                .ok()
                .and_then(|m| m.parse().ok())
                .unwrap_or(1000),
            enable_tls: env::var("BEARDOG_ENABLE_TLS")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl RuntimeNetworkConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Get full API URL
    pub fn api_url(&self) -> String {
        let protocol = if self.enable_tls { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.api_host, self.api_port)
    }

    /// Get metrics URL
    pub fn metrics_url(&self) -> String {
        format!("http://{}:{}/metrics", self.api_host, self.metrics_port)
    }

    /// Get health check URL
    pub fn health_url(&self) -> String {
        format!("http://{}:{}/health", self.api_host, self.health_port)
    }

    /// Get WebSocket URL
    pub fn ws_url(&self) -> String {
        let protocol = if self.enable_tls { "wss" } else { "ws" };
        format!("{}://{}:{}", protocol, self.api_host, self.ws_port)
    }

    /// Get gRPC endpoint
    pub fn grpc_endpoint(&self) -> String {
        format!("{}:{}", self.api_host, self.grpc_port)
    }
}

/// Runtime HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeHsmConfig {
    /// PKCS#11 library path
    pub pkcs11_library_path: String,
    /// TPM device path
    pub tpm_device_path: String,
    /// Software HSM storage path
    pub software_hsm_storage: String,
    /// Enable hardware HSM
    pub enable_hardware_hsm: bool,
}

impl Default for RuntimeHsmConfig {
    fn default() -> Self {
        Self {
            pkcs11_library_path: env::var("BEARDOG_PKCS11_LIBRARY")
                .unwrap_or_else(|_| "/usr/lib/softhsm/libsofthsm2.so".to_string()),
            tpm_device_path: env::var("BEARDOG_TPM_DEVICE")
                .unwrap_or_else(|_| "/dev/tpm0".to_string()),
            software_hsm_storage: env::var("BEARDOG_HSM_STORAGE")
                .unwrap_or_else(|_| "/var/lib/beardog/hsm".to_string()),
            enable_hardware_hsm: env::var("BEARDOG_ENABLE_HARDWARE_HSM")
                .ok()
                .and_then(|e| e.parse().ok())
                .unwrap_or(false),
        }
    }
}

impl RuntimeHsmConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }
}

/// Complete runtime configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Network configuration
    pub network: RuntimeNetworkConfig,
    /// HSM configuration
    pub hsm: RuntimeHsmConfig,
    /// Environment (development, staging, production)
    pub environment: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            network: RuntimeNetworkConfig::default(),
            hsm: RuntimeHsmConfig::default(),
            environment: env::var("BEARDOG_ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}

impl RuntimeConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        Self::default()
    }

    /// Check if running in production
    pub fn is_production(&self) -> bool {
        self.environment.to_lowercase() == "production"
    }

    /// Check if running in development
    pub fn is_development(&self) -> bool {
        self.environment.to_lowercase() == "development"
    }

    /// Check if running in staging
    pub fn is_staging(&self) -> bool {
        self.environment.to_lowercase() == "staging"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_network_config() {
        let config = RuntimeNetworkConfig::default();
        assert_eq!(config.api_port, 8080);
        assert_eq!(config.metrics_port, 9090);
        assert!(config.enable_tls);
    }

    #[test]
    fn test_api_url_generation() {
        let config = RuntimeNetworkConfig {
            api_host: "example.com".to_string(),
            api_port: 8443,
            enable_tls: true,
            ..Default::default()
        };
        assert_eq!(config.api_url(), "https://example.com:8443");
    }

    #[test]
    fn test_api_url_without_tls() {
        let config = RuntimeNetworkConfig {
            api_host: "localhost".to_string(),
            api_port: 8080,
            enable_tls: false,
            ..Default::default()
        };
        assert_eq!(config.api_url(), "http://localhost:8080");
    }

    #[test]
    fn test_environment_detection() {
        let config = RuntimeConfig {
            environment: "production".to_string(),
            ..Default::default()
        };
        assert!(config.is_production());
        assert!(!config.is_development());
    }

    #[test]
    fn test_hsm_config_defaults() {
        let config = RuntimeHsmConfig::default();
        assert!(config.pkcs11_library_path.contains("softhsm"));
        assert_eq!(config.tpm_device_path, "/dev/tpm0");
        assert!(!config.enable_hardware_hsm);
    }

    #[test]
    fn test_url_helpers() {
        let config = RuntimeNetworkConfig {
            api_host: "localhost".to_string(),
            api_port: 8080,
            metrics_port: 9090,
            health_port: 8081,
            ws_port: 3000,
            grpc_port: 50051,
            enable_tls: false,
            ..Default::default()
        };

        assert_eq!(config.api_url(), "http://localhost:8080");
        assert_eq!(config.metrics_url(), "http://localhost:9090/metrics");
        assert_eq!(config.health_url(), "http://localhost:8081/health");
        assert_eq!(config.ws_url(), "ws://localhost:3000");
        assert_eq!(config.grpc_endpoint(), "localhost:50051");
    }
}
