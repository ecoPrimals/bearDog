//! Network configuration for BearDog services
//!
//! Provides configurable network settings to replace hardcoded values throughout the codebase.

use serde::{Deserialize, Serialize};
use std::env;

/// Network configuration for all BearDog services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// API server configuration
    pub api: ApiConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// External service endpoints
    pub external_services: ExternalServicesConfig,
    /// Monitoring and metrics configuration
    pub monitoring: MonitoringConfig,
    /// Webhook configuration
    pub webhooks: WebhookConfig,
}

/// API server network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address for API server (default: "0.0.0.0")
    pub bind_address: String,
    /// API server port (default: 8080)
    pub port: u16,
    /// Full API base URL (constructed from bind_address and port)
    pub base_url: String,
}

/// Database connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database host (default: "localhost")
    pub host: String,
    /// Database port (default: 5432 for PostgreSQL)
    pub port: u16,
    /// Database name
    pub database: String,
    /// Connection URL (constructed or provided)
    pub url: String,
}

/// External service endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServicesConfig {
    /// Prometheus endpoint for metrics
    pub prometheus_url: String,
    /// Grafana endpoint for dashboards  
    pub grafana_url: String,
    /// Grafana port (default: 3000)
    pub grafana_port: u16,
    /// SongBird discovery service URL
    pub songbird_url: String,
    /// NestGate service URL
    pub nestgate_url: String,
    /// ToadStool compute provider URLs
    pub toadstool_urls: Vec<String>,
}

/// Monitoring and metrics configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Metrics bind address (default: "127.0.0.1")
    pub metrics_bind_address: String,
    /// Metrics port (default: 9090)
    pub metrics_port: u16,
    /// Health check endpoint port (default: 8081)
    pub health_port: u16,
    /// Dashboard port (default: 3000)
    pub dashboard_port: u16,
}

/// Webhook configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Base URL for webhook callbacks
    pub callback_base_url: String,
    /// Webhook server port (default: 8080)
    pub webhook_port: u16,
    /// External webhook endpoints
    pub external_webhooks: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            api: ApiConfig::default(),
            database: DatabaseConfig::default(),
            external_services: ExternalServicesConfig::default(),
            monitoring: MonitoringConfig::default(),
            webhooks: WebhookConfig::default(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        let bind_address = env::var("BEARDOG_API_BIND_ADDRESS")
            .unwrap_or_else(|_| "0.0.0.0".to_string());
        let port: u16 = env::var("BEARDOG_API_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);
        
        let base_url = if bind_address == "0.0.0.0" {
            format!("http://localhost:{}", port)
        } else {
            format!("http://{}:{}", bind_address, port)
        };

        Self {
            bind_address,
            port,
            base_url,
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let host = env::var("BEARDOG_DB_HOST")
            .unwrap_or_else(|_| "localhost".to_string());
        let port: u16 = env::var("BEARDOG_DB_PORT")
            .unwrap_or_else(|_| "5432".to_string())
            .parse()
            .unwrap_or(5432);
        let database = env::var("BEARDOG_DB_NAME")
            .unwrap_or_else(|_| "beardog".to_string());
        
        let url = format!("postgresql://{}:{}/{}", host, port, database);

        Self {
            host,
            port,
            database,
            url,
        }
    }
}

impl Default for ExternalServicesConfig {
    fn default() -> Self {
        Self {
            prometheus_url: env::var("BEARDOG_PROMETHEUS_URL")
                .unwrap_or_else(|_| "http://localhost:9090".to_string()),
            grafana_url: env::var("BEARDOG_GRAFANA_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            grafana_port: env::var("BEARDOG_GRAFANA_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
            songbird_url: env::var("BEARDOG_SONGBIRD_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            nestgate_url: env::var("BEARDOG_NESTGATE_URL")
                .unwrap_or_else(|_| "http://localhost:8088".to_string()),
            toadstool_urls: env::var("BEARDOG_TOADSTOOL_URLS")
                .unwrap_or_else(|_| "http://localhost:8080".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            metrics_bind_address: env::var("BEARDOG_METRICS_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            metrics_port: env::var("BEARDOG_METRICS_PORT")
                .unwrap_or_else(|_| "9090".to_string())
                .parse()
                .unwrap_or(9090),
            health_port: env::var("BEARDOG_HEALTH_PORT")
                .unwrap_or_else(|_| "8081".to_string())
                .parse()
                .unwrap_or(8081),
            dashboard_port: env::var("BEARDOG_DASHBOARD_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        }
    }
}

impl Default for WebhookConfig {
    fn default() -> Self {
        let webhook_port: u16 = env::var("BEARDOG_WEBHOOK_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080);

        Self {
            callback_base_url: env::var("BEARDOG_WEBHOOK_CALLBACK_URL")
                .unwrap_or_else(|_| format!("http://localhost:{}", webhook_port)),
            webhook_port,
            external_webhooks: env::var("BEARDOG_EXTERNAL_WEBHOOKS")
                .unwrap_or_else(|_| "".to_string())
                .split(',')
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
                .collect(),
        }
    }
}

impl NetworkConfig {
    /// Load configuration from environment variables and config files
    pub fn load() -> Self {
        // Start with defaults (which read from env vars)
        let mut config = Self::default();
        
        // Try to load from configuration file if specified
        if let Ok(config_path) = std::env::var("BEARDOG_CONFIG_FILE") {
            if let Ok(file_config) = Self::load_from_file(&config_path) {
                config = Self::merge_configs(config, file_config);
            } else {
                tracing::warn!("Failed to load config from file: {}", config_path);
            }
        }

        // Also check for network-specific config file
        if let Ok(network_config_path) = std::env::var("BEARDOG_NETWORK_CONFIG_FILE") {
            if let Ok(file_config) = Self::load_from_file(&network_config_path) {
                config = Self::merge_configs(config, file_config);
            } else {
                tracing::warn!("Failed to load network config from file: {}", network_config_path);
            }
        }

        config
    }

    /// Load configuration from a file (TOML or YAML)
    fn load_from_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path, e))?;

        if path.ends_with(".toml") {
            toml::from_str(&content)
                .map_err(|e| format!("Failed to parse TOML config: {}", e))
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            serde_yaml::from_str(&content)
                .map_err(|e| format!("Failed to parse YAML config: {}", e))
        } else if path.ends_with(".json") {
            serde_json::from_str(&content)
                .map_err(|e| format!("Failed to parse JSON config: {}", e))
        } else {
            Err("Unsupported config file format. Use .toml, .yaml/.yml, or .json".to_string())
        }
    }

    /// Merge two configurations, with file_config taking precedence
    fn merge_configs(env_config: Self, file_config: Self) -> Self {
        // For now, file config completely overrides env config
        // In a more sophisticated implementation, we might merge individual fields
        file_config
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate ports are in valid range
        if self.api.port == 0 || self.api.port > 65535 {
            return Err(format!("Invalid API port: {}", self.api.port));
        }

        // Validate database URL format
        if self.database.url.is_empty() {
            return Err("Database URL cannot be empty".to_string());
        }

        // Validate external service URLs
        if !self.external_services.prometheus_url.starts_with("http") {
            return Err("Prometheus URL must start with http:// or https://".to_string());
        }

        if !self.external_services.grafana_url.starts_with("http") {
            return Err("Grafana URL must start with http:// or https://".to_string());
        }

        Ok(())
    }

    /// Create example configuration files for users
    pub fn create_example_toml() -> String {
        r#"
# BearDog Network Configuration Example
# Save as beardog-network.toml and set BEARDOG_NETWORK_CONFIG_FILE

[api]
host = "0.0.0.0"
port = 8080
enable_tls = true
tls_cert_path = "/etc/beardog/tls/cert.pem"
tls_key_path = "/etc/beardog/tls/key.pem"

[database]
url = "postgresql://beardog:password@localhost:5432/beardog"
max_connections = 20
connection_timeout_ms = 5000

[external_services]
prometheus_url = "http://prometheus:9090"
grafana_url = "http://grafana:3000"
elasticsearch_url = "http://elasticsearch:9200"

[security]
enable_https_redirect = true
cors_enabled = true
cors_allowed_origins = ["https://app.beardog.com", "https://admin.beardog.com"]
rate_limit_requests_per_minute = 100
"#.to_string()
    }

    /// Create example YAML configuration
    pub fn create_example_yaml() -> String {
        r#"
# BearDog Network Configuration Example  
# Save as beardog-network.yaml and set BEARDOG_NETWORK_CONFIG_FILE

api:
  host: "0.0.0.0"
  port: 8080
  enable_tls: true
  tls_cert_path: "/etc/beardog/tls/cert.pem"
  tls_key_path: "/etc/beardog/tls/key.pem"

database:
  url: "postgresql://beardog:password@localhost:5432/beardog"
  max_connections: 20
  connection_timeout_ms: 5000

external_services:
  prometheus_url: "http://prometheus:9090"
  grafana_url: "http://grafana:3000"
  elasticsearch_url: "http://elasticsearch:9200"

security:
  enable_https_redirect: true
  cors_enabled: true
  cors_allowed_origins:
    - "https://app.beardog.com"
    - "https://admin.beardog.com"
  rate_limit_requests_per_minute: 100
"#.to_string()
    }

    /// Get the full API endpoint URL
    pub fn api_endpoint(&self) -> String {
        self.api.base_url.clone()
    }

    /// Get database connection URL
    pub fn database_url(&self) -> String {
        self.database.url.clone()
    }

    /// Get Prometheus metrics URL
    pub fn prometheus_endpoint(&self) -> String {
        self.external_services.prometheus_url.clone()
    }

    /// Get Grafana dashboard URL  
    pub fn grafana_endpoint(&self) -> String {
        self.external_services.grafana_url.clone()
    }
}
