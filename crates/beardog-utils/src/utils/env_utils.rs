use beardog_errors::{BearDogError, BearDogResult};
use std::env;
use std::time::Duration;

/// Environment variable utilities for production deployments
pub struct EnvUtils;

impl EnvUtils {
    /// Get a required environment variable
    pub fn get_required(key: &str) -> BearDogResult<String> {
        env::var(key).map_err(|_| BearDogError::Configuration {
            message: format!("Required environment variable {key} not set"),
        })
    }

    /// Get an optional environment variable with default
    pub fn get_optional(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    /// Get a boolean environment variable with default
    pub fn get_bool(key: &str, default: bool) -> bool {
        env::var(key)
            .map(|v| v.to_lowercase() == "true" || v == "1")
            .unwrap_or(default)
    }

    /// Get a numeric environment variable with default
    pub fn get_u16(key: &str, default: u16) -> u16 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a numeric environment variable with default
    pub fn get_u32(key: &str, default: u32) -> u32 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a numeric environment variable with default  
    pub fn get_u64(key: &str, default: u64) -> u64 {
        env::var(key)
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }

    /// Get a duration from environment variable (in seconds)
    pub fn get_duration_secs(key: &str, default_secs: u64) -> Duration {
        let secs = Self::get_u64(key, default_secs);
        Duration::from_secs(secs)
    }

    /// Get a CSV list from environment variable
    pub fn get_csv_list(key: &str, default: Vec<&str>) -> Vec<String> {
        env::var(key)
            .map(|v| v.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_else(|_| default.into_iter().map(|s| s.to_string()).collect())
    }

    /// Validate required environment variables for production
    pub fn validate_production_env() -> BearDogResult<()> {
        let required_vars = [
            "BEARDOG_DATABASE_URL",
            "BEARDOG_SECRET_KEY",
            "BEARDOG_ENCRYPTION_KEY",
            "BEARDOG_API_BIND_ADDRESS",
        ];

        for var in &required_vars {
            Self::get_required(var)?;
        }

        // Validate optional but important vars
        let warnings = [
            ("BEARDOG_LOG_LEVEL", "INFO"),
            ("BEARDOG_NESTGATE_ENDPOINT", "https://nestgate.example.com"),
            ("BEARDOG_SONGBIRD_ENDPOINT", "https://songbird.example.com"),
            ("BEARDOG_SMTP_SERVER", "smtp.example.com"),
        ];

        for (var, example) in &warnings {
            if env::var(var).is_err() {
                tracing::warn!("Consider setting {}, example: {}", var, example);
            }
        }

        Ok(())
    }

    /// Get database configuration from environment
    pub fn get_database_config() -> BearDogResult<DatabaseConfig> {
        Ok(DatabaseConfig {
            url: Self::get_required("BEARDOG_DATABASE_URL")?,
            max_connections: Self::get_u32("BEARDOG_DB_MAX_CONNECTIONS", 10),
            connection_timeout: Self::get_duration_secs("BEARDOG_DB_CONNECTION_TIMEOUT", 30),
            idle_timeout: Self::get_duration_secs("BEARDOG_DB_IDLE_TIMEOUT", 600),
            enable_ssl: Self::get_bool("BEARDOG_DB_SSL", true),
        })
    }

    /// Get Redis configuration from environment  
    pub fn get_redis_config() -> Option<RedisConfig> {
        env::var("BEARDOG_REDIS_URL").ok().map(|url| RedisConfig {
            url,
            max_connections: Self::get_u32("BEARDOG_REDIS_MAX_CONNECTIONS", 10),
            connection_timeout: Self::get_duration_secs("BEARDOG_REDIS_CONNECTION_TIMEOUT", 5),
            key_prefix: Self::get_optional("BEARDOG_REDIS_KEY_PREFIX", "beardog:"),
        })
    }

    /// Get observability configuration
    pub fn get_observability_config() -> ObservabilityConfig {
        ObservabilityConfig {
            log_level: Self::get_optional("BEARDOG_LOG_LEVEL", "INFO"),
            enable_metrics: Self::get_bool("BEARDOG_ENABLE_METRICS", true),
            metrics_port: Self::get_u16("BEARDOG_METRICS_PORT", 9090),
            enable_tracing: Self::get_bool("BEARDOG_ENABLE_TRACING", true),
            jaeger_endpoint: env::var("BEARDOG_JAEGER_ENDPOINT").ok(),
            otlp_endpoint: env::var("BEARDOG_OTLP_ENDPOINT").ok(),
        }
    }

    /// Get security configuration
    pub fn get_security_config() -> BearDogResult<SecurityConfig> {
        Ok(SecurityConfig {
            secret_key: Self::get_required("BEARDOG_SECRET_KEY")?,
            encryption_key: Self::get_required("BEARDOG_ENCRYPTION_KEY")?,
            jwt_expiry: Self::get_duration_secs("BEARDOG_JWT_EXPIRY", 3600),
            rate_limit_requests: Self::get_u32("BEARDOG_RATE_LIMIT_REQUESTS", 100),
            rate_limit_window: Self::get_duration_secs("BEARDOG_RATE_LIMIT_WINDOW", 60),
            enable_mfa: Self::get_bool("BEARDOG_ENABLE_MFA", true),
            session_timeout: Self::get_duration_secs("BEARDOG_SESSION_TIMEOUT", 1800),
        })
    }
}

/// Database configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of concurrent database connections
    pub max_connections: u32,
    /// Connection timeout for database operations
    pub connection_timeout: Duration,
    /// Idle timeout for database connections
    pub idle_timeout: Duration,
    /// Whether to enable SSL for database connections
    pub enable_ssl: bool,
}

/// Redis configuration
#[derive(Debug, Clone)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Maximum number of concurrent Redis connections
    pub max_connections: u32,
    /// Connection timeout for Redis operations
    pub connection_timeout: Duration,
    /// Key prefix for Redis keys
    pub key_prefix: String,
}

/// Observability configuration
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    /// Log level for application logging
    pub log_level: String,
    /// Whether to enable metrics collection
    pub enable_metrics: bool,
    /// Port for metrics server
    pub metrics_port: u16,
    /// Whether to enable distributed tracing
    pub enable_tracing: bool,
    /// Jaeger endpoint for trace collection
    pub jaeger_endpoint: Option<String>,
    /// OpenTelemetry endpoint for trace collection
    pub otlp_endpoint: Option<String>,
}

/// Security configuration
#[derive(Debug, Clone)]
pub struct SecurityConfig {
    /// Secret key for cryptographic operations
    pub secret_key: String,
    /// Encryption key for data encryption
    pub encryption_key: String,
    /// JWT token expiry duration
    pub jwt_expiry: Duration,
    /// Number of requests allowed per rate limit window
    pub rate_limit_requests: u32,
    /// Time window for rate limiting
    pub rate_limit_window: Duration,
    /// Whether to enable multi-factor authentication
    pub enable_mfa: bool,
    /// Session timeout duration
    pub session_timeout: Duration,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_optional() {
        env::set_var("TEST_VAR", "test_value");
        assert_eq!(EnvUtils::get_optional("TEST_VAR", "default"), "test_value");
        assert_eq!(EnvUtils::get_optional("NON_EXISTENT", "default"), "default");
        env::remove_var("TEST_VAR");
    }

    #[test]
    fn test_get_bool() {
        env::set_var("TEST_BOOL_TRUE", "true");
        env::set_var("TEST_BOOL_FALSE", "false");
        env::set_var("TEST_BOOL_1", "1");

        assert!(EnvUtils::get_bool("TEST_BOOL_TRUE", false));
        assert!(!EnvUtils::get_bool("TEST_BOOL_FALSE", true));
        assert!(EnvUtils::get_bool("TEST_BOOL_1", false));
        assert!(EnvUtils::get_bool("NON_EXISTENT", true));

        env::remove_var("TEST_BOOL_TRUE");
        env::remove_var("TEST_BOOL_FALSE");
        env::remove_var("TEST_BOOL_1");
    }

    #[test]
    fn test_get_csv_list() {
        env::set_var("TEST_CSV", "item1,item2,item3");
        let result = EnvUtils::get_csv_list("TEST_CSV", vec!["default"]);
        assert_eq!(result, vec!["item1", "item2", "item3"]);

        let default_result = EnvUtils::get_csv_list("NON_EXISTENT", vec!["default1", "default2"]);
        assert_eq!(default_result, vec!["default1", "default2"]);

        env::remove_var("TEST_CSV");
    }
}
