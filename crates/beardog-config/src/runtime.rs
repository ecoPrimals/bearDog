//! Runtime Configuration Management
//!
//! This module provides dynamic configuration loading from environment variables,
//! configuration files, and secure defaults, replacing hardcoded values throughout
//! the BearDog system.

// Removed unused imports
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Main runtime configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Network configuration
    pub network: NetworkConfig,
    /// Service endpoints configuration  
    pub endpoints: EndpointsConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Performance tuning configuration
    pub performance: PerformanceConfig,
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Host to bind to (default: "0.0.0.0" for production)
    pub bind_host: String,
    /// API port (configurable, default: 8080)
    pub api_port: u16,
    /// HTTPS port (configurable, default: 8443)
    pub https_port: u16,
    /// Admin port (configurable, default: 9092)
    pub admin_port: u16,
    /// Metrics port (configurable, default: 9091)
    pub metrics_port: u16,
    /// Trusted hosts for CORS
    pub trusted_hosts: Vec<String>,
    /// Enable TLS
    pub enable_tls: bool,
    /// TLS certificate path
    pub tls_cert_path: Option<PathBuf>,
    /// TLS key path
    pub tls_key_path: Option<PathBuf>,
}

/// Service endpoints configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointsConfig {
    /// SongBird service endpoint
    pub songbird_endpoint: String,
    /// NestGate service endpoint
    pub nestgate_endpoint: String,
    /// Squirrel service endpoint
    pub squirrel_endpoint: String,
    /// ToadStool service endpoint
    pub toadstool_endpoint: String,
    /// Webhook base URL
    pub webhook_base_url: String,
    /// External API base URL
    pub external_api_base_url: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Session timeout
    pub session_timeout: Duration,
    /// Maximum failed login attempts
    pub max_failed_attempts: u32,
    /// Account lockout duration
    pub lockout_duration: Duration,
    /// JWT secret key (should be loaded securely)
    pub jwt_secret: Option<String>,
    /// Enable HSM
    pub enable_hsm: bool,
    /// HSM library path
    pub hsm_library_path: Option<PathBuf>,
    /// Force HTTPS redirect
    pub force_https: bool,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Primary database URL
    pub database_url: String,
    /// Metrics database URL (separate for performance)
    pub metrics_database_url: Option<String>,
    /// Maximum database connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Enable connection pooling
    pub enable_pooling: bool,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Worker thread count
    pub worker_threads: Option<usize>,
    /// Enable request compression
    pub enable_compression: bool,
    /// Rate limiting per minute
    pub rate_limit_per_minute: u32,
}

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Log level
    pub log_level: String,
    /// Enable structured logging
    pub enable_structured_logging: bool,
    /// Maximum log file size
    pub max_log_file_size: usize,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            network: NetworkConfig::default(),
            endpoints: EndpointsConfig::default(),
            security: SecurityConfig::default(),
            database: DatabaseConfig::default(),
            performance: PerformanceConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind_host: "0.0.0.0".to_string(), // Bind to all interfaces for production
            api_port: 8080,
            https_port: 8443,
            admin_port: 9092,
            metrics_port: 9091,
            trusted_hosts: vec![
                "beardog.local".to_string(),
                "songbird.beardog.local".to_string(),
            ],
            enable_tls: true, // Default to secure
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

impl Default for EndpointsConfig {
    fn default() -> Self {
        Self {
            // Use internal service mesh names instead of localhost
            songbird_endpoint: "https://songbird.ecosystem.internal:8443".to_string(),
            nestgate_endpoint: "https://nestgate.ecosystem.internal:8443".to_string(),
            squirrel_endpoint: "https://squirrel.ecosystem.internal:8443".to_string(),
            toadstool_endpoint: "https://toadstool.ecosystem.internal:8443".to_string(),
            webhook_base_url: "https://api.beardog.local:8443".to_string(),
            external_api_base_url: "https://api.beardog.local:8443".to_string(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            session_timeout: Duration::from_secs(3600), // 1 hour
            max_failed_attempts: 5,
            lockout_duration: Duration::from_secs(1800), // 30 minutes
            jwt_secret: None,  // Must be provided via environment or config
            enable_hsm: false, // Default to software mode
            hsm_library_path: None,
            force_https: true, // Force HTTPS by default
        }
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            // Use SQLite by default for development, but should be configured for production
            database_url: "sqlite:///var/lib/beardog/beardog.db".to_string(),
            metrics_database_url: None,
            max_connections: 10,
            connection_timeout: Duration::from_secs(30),
            enable_pooling: true,
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            request_timeout: Duration::from_secs(30),
            max_connections: 1000,
            worker_threads: None, // Auto-detect
            enable_compression: true,
            rate_limit_per_minute: 60,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval: Duration::from_secs(60),
            log_level: "info".to_string(),
            enable_structured_logging: true,
            max_log_file_size: 10 * 1024 * 1024, // 10MB
        }
    }
}

impl RuntimeConfig {
    /// Load configuration from environment variables, config files, and defaults
    pub fn load() -> BearDogResult<Self> {
        let mut config = Self::default();

        // Load from environment variables with proper error handling
        config.load_from_environment()?;

        // Try to load from configuration file if specified
        if let Ok(config_file) = std::env::var("BEARDOG_CONFIG_FILE") {
            config.load_from_file(&config_file)?;
        }

        // Validate configuration
        config.validate()?;

        Ok(config)
    }

    /// Load configuration from environment variables
    fn load_from_environment(&mut self) -> BearDogResult<()> {
        // Network configuration
        if let Ok(bind_host) = std::env::var("BEARDOG_BIND_HOST") {
            self.network.bind_host = bind_host;
        }
        if let Ok(api_port) = std::env::var("BEARDOG_API_PORT") {
            self.network.api_port =
                api_port
                    .parse()
                    .map_err(|e| beardog_errors::BearDogError::Configuration {
                        message: format!("Invalid API port: {e}"),
                    })?;
        }
        if let Ok(enable_tls) = std::env::var("BEARDOG_ENABLE_TLS") {
            self.network.enable_tls =
                enable_tls
                    .parse()
                    .map_err(|e| beardog_errors::BearDogError::Configuration {
                        message: format!("Invalid TLS setting: {e}"),
                    })?;
        }

        // Endpoints configuration
        if let Ok(songbird_endpoint) = std::env::var("SONGBIRD_ENDPOINT") {
            self.endpoints.songbird_endpoint = songbird_endpoint;
        }
        if let Ok(nestgate_endpoint) = std::env::var("NESTGATE_ENDPOINT") {
            self.endpoints.nestgate_endpoint = nestgate_endpoint;
        }

        // Database configuration
        if let Ok(database_url) = std::env::var("BEARDOG_DATABASE_URL") {
            self.database.database_url = database_url;
        }
        if let Ok(metrics_db_url) = std::env::var("BEARDOG_METRICS_DATABASE_URL") {
            self.database.metrics_database_url = Some(metrics_db_url);
        }

        // Security configuration
        if let Ok(jwt_secret) = std::env::var("BEARDOG_JWT_SECRET") {
            self.security.jwt_secret = Some(jwt_secret);
        }
        if let Ok(force_https) = std::env::var("BEARDOG_FORCE_HTTPS") {
            self.security.force_https = force_https.parse().unwrap_or(true);
        }

        Ok(())
    }

    /// Load configuration from a file
    fn load_from_file(&mut self, path: &str) -> BearDogResult<()> {
        let content = std::fs::read_to_string(path).map_err(|e| {
            beardog_errors::BearDogError::Configuration {
                message: format!("Failed to read config file {path}: {e}"),
            }
        })?;

        // Support both TOML and JSON
        if path.ends_with(".toml") {
            *self = toml::from_str(&content).map_err(|e| {
                beardog_errors::BearDogError::Configuration {
                    message: format!("Failed to parse TOML config: {e}"),
                }
            })?;
        } else if path.ends_with(".json") {
            *self = serde_json::from_str(&content).map_err(|e| {
                beardog_errors::BearDogError::Configuration {
                    message: format!("Failed to parse JSON config: {e}"),
                }
            })?;
        } else {
            return Err(beardog_errors::BearDogError::Configuration {
                message: "Unsupported config file format. Use .toml or .json".to_string(),
            });
        }

        Ok(())
    }

    /// Validate the configuration
    fn validate(&self) -> BearDogResult<()> {
        // Validate ports are in valid range (u16 is always 0-65535, so just check it's not 0)
        if self.network.api_port == 0 {
            return Err(beardog_errors::BearDogError::Configuration {
                message: "API port cannot be 0".to_string(),
            });
        }

        // Validate database URL is not empty
        if self.database.database_url.is_empty() {
            return Err(beardog_errors::BearDogError::Configuration {
                message: "Database URL cannot be empty".to_string(),
            });
        }

        // Warn if using insecure defaults
        if !self.network.enable_tls {
            tracing::warn!("TLS is disabled - this is not recommended for production");
        }

        if !self.security.force_https {
            tracing::warn!("HTTPS redirect is disabled - this is not recommended for production");
        }

        Ok(())
    }

    /// Get the full API base URL
    pub fn api_base_url(&self) -> String {
        let protocol = if self.network.enable_tls {
            "https"
        } else {
            "http"
        };
        let port = if self.network.enable_tls {
            self.network.https_port
        } else {
            self.network.api_port
        };

        // Don't include standard ports in URLs
        if (protocol == "http" && port == 80) || (protocol == "https" && port == 443) {
            format!("{}://{}", protocol, self.network.bind_host)
        } else {
            format!("{}://{}:{}", protocol, self.network.bind_host, port)
        }
    }

    /// Get the metrics endpoint URL
    pub fn metrics_url(&self) -> String {
        format!("{}:{}", self.network.bind_host, self.network.metrics_port)
    }

    /// Get the admin endpoint URL  
    pub fn admin_url(&self) -> String {
        format!("{}:{}", self.network.bind_host, self.network.admin_port)
    }
}

/// Global configuration instance
static CONFIG: std::sync::OnceLock<RuntimeConfig> = std::sync::OnceLock::new();

/// Initialize global configuration
pub fn init_config() -> BearDogResult<()> {
    let config = RuntimeConfig::load()?;
    CONFIG
        .set(config)
        .map_err(|_| beardog_errors::BearDogError::Configuration {
            message: "Configuration already initialized".to_string(),
        })?;
    Ok(())
}

/// Get global configuration
pub fn get_config() -> &'static RuntimeConfig {
    CONFIG.get().unwrap_or_else(|| {
        // If configuration is not initialized, initialize it
        tracing::warn!("Configuration not initialized, attempting to initialize");
        init_config().unwrap_or_else(|e| {
            tracing::error!("Failed to initialize configuration: {}", e);
            // This is a programming error - we must have configuration to proceed
            std::process::exit(1);
        });
        CONFIG.get().expect(
            "Configuration must be initialized before use - this indicates a programming error",
        )
    })
}
