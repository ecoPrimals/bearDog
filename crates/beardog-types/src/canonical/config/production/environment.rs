// Production Environment Configuration

use crate::canonical::capabilities::CapabilityType;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **ENVIRONMENT CONFIGURATION** - Environment-specific settings
///
/// Contains configuration that varies between different deployment environments,
/// including environment validation, secrets management, and environment overrides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    /// Environment type
    /// The environment type value
    pub environment_type: EnvironmentType,

    /// Environment-specific overrides
    pub overrides: HashMap<String, String>,

    /// Environment validation
    pub validation: EnvironmentValidation,

    /// Environment secrets management
    /// The secrets value
    pub secrets: ModernSecretsConfig,
}

/// Environment type classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
/// Types of environment
pub enum EnvironmentType {
    /// Local development environment
    Local,
    /// Shared development environment
    Development,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
    /// Disaster recovery environment
    Disaster,
}

/// Environment validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentValidation {
    /// Validate environment on startup
    pub validate_on_startup: bool,
    /// Validate environment periodically during runtime
    pub validate_periodically: bool,
    /// Interval between periodic validations
    pub validation_interval: Duration,
    /// Required environment variables that must be present
    /// Collection of required environment variables
    pub required_environment_variables: Vec<String>,
    /// Required files that must be accessible
    /// Collection of required files
    pub required_files: Vec<String>,
    /// Required network endpoints that must be reachable
    /// Collection of required endpoints
    pub required_endpoints: Vec<String>,
    /// Minimum required disk space in MB
    /// Optional min disk space mb
    pub min_disk_space_mb: Option<u64>,
    /// Minimum required memory in MB
    /// Optional min memory mb
    pub min_memory_mb: Option<u64>,
}

/// Modern capability-based secrets configuration
///
/// Replaces the deprecated `SecretsProviderType` enum with dynamic capability discovery.
/// This allows `BearDog` to work with any secrets provider that implements the
/// `SecretsManagement` capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernSecretsConfig {
    /// Collection of required capabilities
    pub required_capabilities: Vec<CapabilityType>,

    /// Optional preferred capabilities
    pub preferred_capabilities: Option<Vec<CapabilityType>>,

    pub fallback_config: Option<serde_json::Value>,
}

impl Default for ModernSecretsConfig {
    fn default() -> Self {
        Self {
            required_capabilities: vec![CapabilityType::SecretsManagement],
            preferred_capabilities: None,
            fallback_config: None,
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self::new(EnvironmentType::Development)
    }
}

impl EnvironmentConfig {
    /// Create a new environment configuration
    #[must_use]
    /// Creates a new instance
    pub fn new(environment_type: EnvironmentType) -> Self {
        Self {
            environment_type,
            overrides: HashMap::new(),
            validation: EnvironmentValidation::default(),
            secrets: ModernSecretsConfig::default(),
        }
    }

    /// Add an environment override
    #[must_use]
    /// Creates instance with override
    pub fn with_override(mut self, key: &str, value: &str) -> Self {
        self.overrides.insert(key.to_string(), value.to_string());
        self
    }

    /// Set secrets provider
    #[must_use]
    /// Creates instance with secrets provider
    pub fn with_secrets_provider(mut self, provider: ModernSecretsConfig) -> Self {
        self.secrets = provider;
        self
    }

    /// Set secrets rotation settings
    #[must_use]
    /// Creates instance with secrets rotation
    pub fn with_secrets_rotation(self, _enabled: bool, _interval: Duration) -> Self {
        // This functionality is now handled by the ModernSecretsConfig struct
        // and its required_capabilities.
        // For now, we'll keep it as a placeholder or remove if not directly applicable.
        // The ModernSecretsConfig struct manages rotation via required_capabilities.
        self
    }

    /// Validate the environment configuration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Validation interval is less than 60 seconds for periodic validation
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Note: Production-specific secrets validation is now handled via
        // ModernSecretsConfig.required_capabilities
        if self.environment_type == EnvironmentType::Production {
            // Validation moved to capability-based system
        }

        // Validate validation settings
        if self.validation.validate_periodically
            && self.validation.validation_interval < Duration::from_secs(60)
        {
            return Err(BearDogError::Business {
                message: "Validation interval must be at least 60 seconds".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Note: Secrets rotation validation now handled via required_capabilities
        Ok(())
    }

    /// Get an environment override value
    #[must_use]
    pub fn get_override(&self, key: &str) -> Option<&String> {
        self.overrides.get(key)
    }

    /// Check if this is a production-like environment
    #[must_use]
    pub fn is_production_like(&self) -> bool {
        matches!(
            self.environment_type,
            EnvironmentType::Production | EnvironmentType::Staging | EnvironmentType::Disaster
        )
    }

    #[must_use]
    pub fn recommended_for_type(env_type: EnvironmentType) -> Self {
        let mut config = Self::new(env_type);

        match env_type {
            EnvironmentType::Local => {
                config.validation.validate_on_startup = false;
            }
            EnvironmentType::Development => {
                config.validation.validate_on_startup = true;
            }
            EnvironmentType::Testing => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
            }
            EnvironmentType::Staging => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
            }
            EnvironmentType::Production => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
                // Note: Secrets settings now configured via ModernSecretsConfig.required_capabilities
            }
            EnvironmentType::Disaster => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
            }
        }

        config
    }
}

impl Default for EnvironmentValidation {
    fn default() -> Self {
        Self {
            validate_on_startup: true,
            validate_periodically: false,
            validation_interval: Duration::from_secs(
                std::env::var("BEARDOG_ENV_VALIDATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
            required_environment_variables: vec!["PATH".to_string(), "HOME".to_string()],
            required_files: vec![],
            required_endpoints: vec![],
            min_disk_space_mb: Some(1024), // 1 GB
            min_memory_mb: Some(512),      // 512 MB
        }
    }
}

impl EnvironmentType {
    /// Check if this environment type requires high availability
    #[must_use]
    pub fn requires_high_availability(&self) -> bool {
        matches!(self, Self::Production | Self::Staging | Self::Disaster)
    }

    #[must_use]
    pub fn recommended_log_level(&self) -> &'static str {
        match self {
            Self::Development | Self::Local => "debug",
            Self::Staging | Self::Testing => "info",
            Self::Production => "warn",
            Self::Disaster => "error",
        }
    }

    /// Get the recommended metrics collection interval
    #[must_use]
    pub fn metrics_collection_interval(&self) -> Duration {
        match self {
            Self::Local => Duration::from_secs(
                std::env::var("BEARDOG_LOCAL_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60),
            ),
            Self::Development => Duration::from_secs(
                std::env::var("BEARDOG_DEV_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            Self::Testing => Duration::from_secs(
                std::env::var("BEARDOG_TEST_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(15),
            ),
            Self::Staging => Duration::from_secs(
                std::env::var("BEARDOG_STAGING_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(10),
            ),
            Self::Production => Duration::from_secs(
                std::env::var("BEARDOG_PRODUCTION_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5),
            ),
            Self::Disaster => Duration::from_secs(1), // 1 second
        }
    }
}

impl ModernSecretsConfig {
    /// Check if this provider supports automatic rotation
    /// DEPRECATED: Use capability-based feature detection instead
    #[deprecated(note = "Use universal adapter capability discovery to check features")]
    #[allow(deprecated)]
    #[must_use]
    pub fn supports_rotation(&self) -> bool {
        // This check is now handled by required_capabilities
        self.required_capabilities
            .contains(&CapabilityType::SecretsManagement)
    }

    /// Check if this provider supports encryption at rest
    /// DEPRECATED: Use capability-based feature detection instead
    #[deprecated(note = "Use universal adapter capability discovery to check features")]
    #[must_use]
    pub fn supports_encryption_at_rest(&self) -> bool {
        // This check is now handled by required_capabilities
        self.required_capabilities
            .contains(&CapabilityType::SecretsManagement)
    }

    #[deprecated(note = "Use universal adapter to discover backup capabilities")]
    #[must_use]
    pub fn recommended_backup(&self) -> Option<Self> {
        // This logic is now handled by required_capabilities and preferred_capabilities
        // For now, we'll return a placeholder or remove if not directly applicable.
        // The ModernSecretsConfig struct manages fallback_config.
        None
    }

    /// Check if provider is deprecated
    #[allow(deprecated)]
    #[must_use]
    pub fn is_deprecated(&self) -> bool {
        // This check is now handled by required_capabilities
        self.required_capabilities
            .contains(&CapabilityType::SecretsManagement)
    }

    /// Get the modern replacement configuration
    #[must_use]
    pub fn get_modern_replacement(&self) -> Option<Self> {
        // This logic is now handled by required_capabilities
        if self
            .required_capabilities
            .contains(&CapabilityType::SecretsManagement)
        {
            Some(Self {
                required_capabilities: vec![CapabilityType::SecretsManagement],
                preferred_capabilities: None,
                fallback_config: None,
            })
        } else {
            None
        }
    }

    /// Get migration guidance for transitioning to modern configuration
    #[must_use]
    pub fn get_migration_guidance(&self) -> Option<&'static str> {
        // This logic is now handled by required_capabilities
        if self
            .required_capabilities
            .contains(&CapabilityType::SecretsManagement)
        {
            Some(
                "🚨 MIGRATION: Secrets management is now handled by required_capabilities. This struct is deprecated."
            )
        } else {
            None
        }
    }

    #[must_use]
    /// Converts to capability type
    pub fn to_capability_type(&self) -> crate::canonical::capabilities::CapabilityType {
        use crate::canonical::capabilities::CapabilityType;
        // This logic is now handled by required_capabilities
        if self
            .required_capabilities
            .contains(&CapabilityType::SecretsManagement)
        {
            CapabilityType::SecretsManagement
        } else {
            CapabilityType::Custom("local_secrets".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_environment_type_display() {
        assert_eq!(EnvironmentType::Local as i32, 0);
        assert!(matches!(
            EnvironmentType::Production,
            EnvironmentType::Production
        ));
    }

    #[test]
    fn test_environment_config_default() {
        let config = EnvironmentConfig::default();
        assert_eq!(config.environment_type, EnvironmentType::Development);
        assert!(config.overrides.is_empty());
        assert!(config.validation.validate_on_startup);
    }

    #[test]
    fn test_environment_config_new() {
        let config = EnvironmentConfig::new(EnvironmentType::Production);
        assert_eq!(config.environment_type, EnvironmentType::Production);
    }

    #[test]
    fn test_environment_config_with_override() {
        let config = EnvironmentConfig::default()
            .with_override("key1", "value1")
            .with_override("key2", "value2");

        assert_eq!(config.get_override("key1"), Some(&"value1".to_string()));
        assert_eq!(config.get_override("key2"), Some(&"value2".to_string()));
        assert_eq!(config.get_override("key3"), None);
    }

    #[test]
    fn test_environment_config_validate_success() {
        let config = EnvironmentConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_environment_config_validate_interval_too_short() {
        let mut config = EnvironmentConfig::default();
        config.validation.validate_periodically = true;
        config.validation.validation_interval = Duration::from_secs(30);

        let result = config.validate();
        assert!(result.is_err());
        if let Err(BearDogError::Business { message, .. }) = result {
            assert!(message.contains("at least 60 seconds"));
        }
    }

    #[test]
    fn test_environment_config_is_production_like() {
        assert!(EnvironmentConfig::new(EnvironmentType::Production).is_production_like());
        assert!(EnvironmentConfig::new(EnvironmentType::Staging).is_production_like());
        assert!(EnvironmentConfig::new(EnvironmentType::Disaster).is_production_like());
        assert!(!EnvironmentConfig::new(EnvironmentType::Development).is_production_like());
        assert!(!EnvironmentConfig::new(EnvironmentType::Local).is_production_like());
        assert!(!EnvironmentConfig::new(EnvironmentType::Testing).is_production_like());
    }

    #[test]
    fn test_environment_config_recommended_for_type_production() {
        let config = EnvironmentConfig::recommended_for_type(EnvironmentType::Production);
        assert!(config.validation.validate_on_startup);
        assert!(config.validation.validate_periodically);
    }

    #[test]
    fn test_environment_type_requires_high_availability() {
        assert!(EnvironmentType::Production.requires_high_availability());
        assert!(EnvironmentType::Staging.requires_high_availability());
        assert!(EnvironmentType::Disaster.requires_high_availability());
        assert!(!EnvironmentType::Development.requires_high_availability());
    }

    #[test]
    fn test_environment_type_recommended_log_level() {
        assert_eq!(EnvironmentType::Local.recommended_log_level(), "debug");
        assert_eq!(
            EnvironmentType::Development.recommended_log_level(),
            "debug"
        );
        assert_eq!(EnvironmentType::Testing.recommended_log_level(), "info");
        assert_eq!(EnvironmentType::Staging.recommended_log_level(), "info");
        assert_eq!(EnvironmentType::Production.recommended_log_level(), "warn");
        assert_eq!(EnvironmentType::Disaster.recommended_log_level(), "error");
    }

    #[test]
    fn test_environment_type_metrics_collection_interval() {
        let disaster = EnvironmentType::Disaster.metrics_collection_interval();
        assert_eq!(disaster, Duration::from_secs(1));
    }

    #[test]
    fn test_modern_secrets_config_default() {
        let secrets = ModernSecretsConfig::default();
        assert_eq!(secrets.required_capabilities.len(), 1);
        assert_eq!(
            secrets.required_capabilities[0],
            CapabilityType::SecretsManagement
        );
    }

    #[test]
    fn test_all_environment_types() {
        let types = vec![
            EnvironmentType::Local,
            EnvironmentType::Development,
            EnvironmentType::Testing,
            EnvironmentType::Staging,
            EnvironmentType::Production,
            EnvironmentType::Disaster,
        ];

        for env_type in types {
            let config = EnvironmentConfig::recommended_for_type(env_type);
            assert_eq!(config.environment_type, env_type);
            assert!(config.validate().is_ok());
        }
    }
}
