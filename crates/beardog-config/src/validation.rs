//! Configuration Validation
//!
//! This module provides comprehensive validation for all BearDog configuration
//! options, ensuring system stability and security in production environments.

use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashSet;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use url::Url;

/// Validate a complete BearDog configuration
pub fn validate_configuration(
    config: &crate::runtime::RuntimeConfig,
) -> BearDogResult<ValidationReport> {
    let mut validator = ConfigurationValidator::new();

    validator.validate_network_config(&config.network)?;
    validator.validate_endpoints_config(&config.endpoints)?;
    validator.validate_security_config(&config.security)?;
    validator.validate_database_config(&config.database)?;
    validator.validate_performance_config(&config.performance)?;
    validator.validate_monitoring_config(&config.monitoring)?;

    Ok(validator.into_report())
}

/// Configuration validation report
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Validation errors that prevent startup
    pub errors: Vec<String>,
    /// Warnings about suboptimal configuration
    pub warnings: Vec<String>,
    /// Informational messages about configuration
    pub info: Vec<String>,
    /// Overall validation success
    pub is_valid: bool,
}

impl ValidationReport {
    /// Create a new empty validation report
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            info: Vec::new(),
            is_valid: true,
        }
    }

    /// Add an error to the report
    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    /// Add a warning to the report
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Add an info message to the report
    pub fn add_info(&mut self, info: String) {
        self.info.push(info);
    }

    /// Print the validation report
    pub fn print(&self) {
        if !self.errors.is_empty() {
            eprintln!("❌ Configuration Errors:");
            for error in &self.errors {
                eprintln!("   • {error}");
            }
        }

        if !self.warnings.is_empty() {
            println!("⚠️  Configuration Warnings:");
            for warning in &self.warnings {
                println!("   • {warning}");
            }
        }

        if !self.info.is_empty() {
            println!("ℹ️  Configuration Info:");
            for info in &self.info {
                println!("   • {info}");
            }
        }

        if self.is_valid {
            println!("✅ Configuration validation passed!");
        } else {
            eprintln!("❌ Configuration validation failed!");
        }
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration validator with comprehensive checks
struct ConfigurationValidator {
    report: ValidationReport,
    used_ports: HashSet<u16>,
}

impl ConfigurationValidator {
    fn new() -> Self {
        Self {
            report: ValidationReport::new(),
            used_ports: HashSet::new(),
        }
    }

    fn into_report(self) -> ValidationReport {
        self.report
    }

    /// Validate network configuration
    fn validate_network_config(
        &mut self,
        config: &crate::runtime::NetworkConfig,
    ) -> BearDogResult<()> {
        // Validate bind host
        if config.bind_host.is_empty() {
            self.report
                .add_error("Bind host cannot be empty".to_string());
        } else {
            match config.bind_host.parse::<IpAddr>() {
                Ok(_) => {
                    self.report
                        .add_info(format!("Valid bind host: {}", config.bind_host));
                }
                Err(_) => {
                    self.report.add_error(format!(
                        "Invalid bind host IP address: {}",
                        config.bind_host
                    ));
                }
            }
        }

        // Validate and check for port conflicts
        self.validate_port("API", config.api_port);
        self.validate_port("HTTPS", config.https_port);
        self.validate_port("Admin", config.admin_port);
        self.validate_port("Metrics", config.metrics_port);

        // Security warnings
        if config.bind_host == "0.0.0.0" {
            self.report.add_info(
                "Binding to all interfaces (0.0.0.0) - good for containerized deployments"
                    .to_string(),
            );
        }

        if !config.enable_tls {
            self.report
                .add_warning("TLS is disabled - not recommended for production".to_string());
        }

        // Validate TLS configuration if enabled
        if config.enable_tls {
            if let Some(ref cert_path) = config.tls_cert_path {
                if !cert_path.exists() {
                    self.report
                        .add_error(format!("TLS certificate file not found: {cert_path:?}"));
                }
            } else {
                self.report
                    .add_warning("TLS enabled but no certificate path specified".to_string());
            }

            if let Some(ref key_path) = config.tls_key_path {
                if !key_path.exists() {
                    self.report
                        .add_error(format!("TLS key file not found: {key_path:?}"));
                }
            } else {
                self.report
                    .add_warning("TLS enabled but no key path specified".to_string());
            }
        }

        Ok(())
    }

    /// Validate endpoints configuration
    fn validate_endpoints_config(
        &mut self,
        config: &crate::runtime::EndpointsConfig,
    ) -> BearDogResult<()> {
        self.validate_url("SongBird", &config.songbird_endpoint);
        self.validate_url("NestGate", &config.nestgate_endpoint);
        self.validate_url("Squirrel", &config.squirrel_endpoint);
        self.validate_url("ToadStool", &config.toadstool_endpoint);
        self.validate_url("Webhook Base", &config.webhook_base_url);
        self.validate_url("External API Base", &config.external_api_base_url);

        Ok(())
    }

    /// Validate security configuration
    fn validate_security_config(
        &mut self,
        config: &crate::runtime::SecurityConfig,
    ) -> BearDogResult<()> {
        // Validate JWT configuration
        if let Some(jwt_secret) = &config.jwt_secret {
            if jwt_secret.is_empty() {
                self.report
                    .add_error("JWT secret key is required for production".to_string());
            } else if jwt_secret.len() < 32 {
                self.report.add_warning(
                    "JWT secret key should be at least 32 characters long".to_string(),
                );
            }
        } else {
            self.report
                .add_error("JWT secret key is required for production".to_string());
        }

        // Check security settings
        if !config.force_https {
            self.report.add_warning(
                "HTTPS redirect is disabled - not recommended for production".to_string(),
            );
        }

        // Note: CORS and encryption key validation removed as they're not in the current SecurityConfig struct

        Ok(())
    }

    /// Validate database configuration
    fn validate_database_config(
        &mut self,
        config: &crate::runtime::DatabaseConfig,
    ) -> BearDogResult<()> {
        // Validate database URL
        if config.database_url.is_empty() {
            self.report
                .add_error("Database URL is required".to_string());
        } else {
            match Url::parse(&config.database_url) {
                Ok(url) => match url.scheme() {
                    "postgresql" | "postgres" | "sqlite" | "mysql" => {
                        self.report.add_info(format!(
                            "Valid database URL scheme: {scheme}",
                            scheme = url.scheme()
                        ));
                    }
                    scheme => {
                        self.report
                            .add_warning(format!("Unusual database scheme: {scheme}"));
                    }
                },
                Err(e) => {
                    self.report.add_error(format!("Invalid database URL: {e}"));
                }
            }
        }

        // Validate connection pool settings
        if config.max_connections == 0 {
            self.report
                .add_error("Maximum connections must be greater than 0".to_string());
        } else if config.max_connections > 1000 {
            self.report.add_warning(
                "Very high max_connections setting - ensure database can handle this load"
                    .to_string(),
            );
        }

        if config.connection_timeout.is_zero() {
            self.report
                .add_error("Connection timeout must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Validate performance configuration
    fn validate_performance_config(
        &mut self,
        config: &crate::runtime::PerformanceConfig,
    ) -> BearDogResult<()> {
        // Validate worker threads
        if let Some(worker_threads) = config.worker_threads {
            if worker_threads == 0 {
                self.report
                    .add_error("Worker threads must be greater than 0".to_string());
            } else {
                let cpu_cores = num_cpus::get();
                if worker_threads > cpu_cores * 2 {
                    self.report.add_warning(format!(
                        "Worker threads ({worker_threads}) exceed 2x CPU cores ({cpu_cores})"
                    ));
                }
            }
        }

        // Validate connection limits
        if config.max_connections == 0 {
            self.report
                .add_error("Max concurrent connections must be greater than 0".to_string());
        } else if config.max_connections > 100_000 {
            self.report.add_warning(
                "Very high concurrent connection limit - ensure system resources are adequate"
                    .to_string(),
            );
        }

        // Validate timeout settings
        if config.request_timeout.is_zero() {
            self.report
                .add_error("Request timeout must be greater than 0".to_string());
        } else if config.request_timeout > Duration::from_secs(300) {
            self.report.add_warning(
                "Very high request timeout - may cause resource exhaustion".to_string(),
            );
        }

        Ok(())
    }

    /// Validate monitoring configuration
    fn validate_monitoring_config(
        &mut self,
        config: &crate::runtime::MonitoringConfig,
    ) -> BearDogResult<()> {
        // Validate log level
        match config.log_level.as_str() {
            "trace" | "debug" | "info" | "warn" | "error" => {
                self.report
                    .add_info(format!("Valid log level: {}", config.log_level));
            }
            _ => {
                self.report
                    .add_error(format!("Invalid log level: {}", config.log_level));
            }
        }

        // Check monitoring settings
        if !config.enable_metrics {
            self.report.add_warning(
                "Metrics collection is disabled - monitoring will be limited".to_string(),
            );
        }

        if config.metrics_interval.as_secs() == 0 {
            self.report
                .add_error("Metrics interval must be greater than 0".to_string());
        } else if config.metrics_interval.as_secs() > 300 {
            self.report
                .add_warning("Very high metrics interval - may miss important events".to_string());
        }

        Ok(())
    }

    /// Validate a port number and check for conflicts
    fn validate_port(&mut self, service_name: &str, port: u16) {
        if port == 0 {
            self.report
                .add_error(format!("{service_name} port cannot be 0"));
            return;
        }

        if port < 1024 && std::env::var("USER").unwrap_or_default() != "root" {
            self.report.add_warning(format!(
                "{service_name} port {port} is privileged and may require root access"
            ));
        }

        if self.used_ports.contains(&port) {
            self.report.add_error(format!(
                "{service_name} port {port} conflicts with another service"
            ));
        } else {
            self.used_ports.insert(port);
        }
    }

    /// Validate a URL
    fn validate_url(&mut self, service_name: &str, url: &str) {
        if url.is_empty() {
            self.report
                .add_error(format!("{service_name} endpoint URL is empty"));
            return;
        }

        match Url::parse(url) {
            Ok(parsed_url) => match parsed_url.scheme() {
                "http" | "https" => {
                    self.report
                        .add_info(format!("Valid {service_name} endpoint: {url}"));

                    if parsed_url.scheme() == "http" {
                        self.report.add_warning(format!(
                            "{service_name} endpoint uses HTTP instead of HTTPS: {url}"
                        ));
                    }
                }
                scheme => {
                    self.report
                        .add_warning(format!("Unusual URL scheme for {service_name}: {scheme}"));
                }
            },
            Err(e) => {
                self.report
                    .add_error(format!("Invalid {service_name} endpoint URL '{url}': {e}"));
            }
        }
    }
}

/// Validate individual configuration values
pub mod validators {
    use super::*;

    /// Validate a bind address
    pub fn validate_bind_address(address: &str) -> BearDogResult<()> {
        if address.parse::<SocketAddr>().is_err() {
            return Err(BearDogError::Configuration {
                message: format!("Invalid bind address: {address}"),
            });
        }
        Ok(())
    }

    /// Validate a port number
    pub fn validate_port(port: u16) -> BearDogResult<()> {
        if port == 0 {
            return Err(BearDogError::Configuration {
                message: "Port cannot be 0".to_string(),
            });
        }
        Ok(())
    }

    /// Validate a URL
    pub fn validate_url(url: &str) -> BearDogResult<()> {
        Url::parse(url).map_err(|e| BearDogError::Configuration {
            message: format!("Invalid URL '{url}': {e}"),
        })?;
        Ok(())
    }

    /// Validate a database URL
    pub fn validate_database_url(url: &str) -> BearDogResult<()> {
        let parsed = Url::parse(url).map_err(|e| BearDogError::Configuration {
            message: format!("Invalid database URL: {e}"),
        })?;

        match parsed.scheme() {
            "postgresql" | "postgres" | "sqlite" | "mysql" => Ok(()),
            scheme => Err(BearDogError::Configuration {
                message: format!("Unsupported database scheme: {scheme}"),
            }),
        }
    }

    /// Validate a log level
    pub fn validate_log_level(level: &str) -> BearDogResult<()> {
        match level {
            "trace" | "debug" | "info" | "warn" | "error" => Ok(()),
            _ => Err(BearDogError::Configuration {
                message: format!("Invalid log level: {level}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::*;

    #[test]
    fn test_validate_port() {
        assert!(validators::validate_port(8080).is_ok());
        assert!(validators::validate_port(443).is_ok());
        assert!(validators::validate_port(0).is_err());
    }

    #[test]
    fn test_validate_url() {
        assert!(validators::validate_url("https://example.com").is_ok());
        assert!(validators::validate_url("http://localhost:8080").is_ok());
        assert!(validators::validate_url("invalid-url").is_err());
        assert!(validators::validate_url("").is_err());
    }

    #[test]
    fn test_validate_database_url() {
        assert!(validators::validate_database_url("postgresql://localhost/test").is_ok());
        assert!(validators::validate_database_url("sqlite:///tmp/test.db").is_ok());
        assert!(validators::validate_database_url("redis://localhost").is_err());
    }

    #[test]
    fn test_validate_log_level() {
        assert!(validators::validate_log_level("info").is_ok());
        assert!(validators::validate_log_level("debug").is_ok());
        assert!(validators::validate_log_level("invalid").is_err());
    }

    #[test]
    fn test_port_conflict_detection() {
        let mut validator = ConfigurationValidator::new();
        validator.validate_port("API", 8080);
        validator.validate_port("Metrics", 8080); // Should conflict

        assert!(!validator.report.is_valid);
        assert!(validator
            .report
            .errors
            .iter()
            .any(|e| e.contains("conflicts")));
    }

    #[test]
    fn test_complete_configuration_validation() {
        let config = RuntimeConfig::default();
        let report = validate_configuration(&config).expect("Validation failed");

        // Should have some warnings about default configuration
        assert!(!report.warnings.is_empty());
    }
}
