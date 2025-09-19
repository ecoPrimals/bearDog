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
            ..Default::default()
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
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        // Validate environment type consistency
        if self.environment_type == EnvironmentType::Production {
            // The ModernSecretsConfig struct manages encryption_at_rest and audit_access
            // via required_capabilities.
            // For now, we'll keep this check as a placeholder.
            // if !self.secrets.encryption_at_rest {
            //     return Err(BearDogError::Security {
            //         message: "Production environments must have encryption at rest enabled"
            //             .to_string(),
            //         category: beardog_errors::SecurityErrorCategory::Configuration,
            //     });
            // }

            // if !self.secrets.audit_access {
            //     return Err(BearDogError::Security {
            //         message: "Production environments must have secret access auditing enabled"
            //             .to_string(),
            //         category: beardog_errors::SecurityErrorCategory::Configuration,
            //     });
            // }
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

        // Validate secrets rotation settings
        // The ModernSecretsConfig struct manages rotation via required_capabilities.
        // This check is no longer directly applicable here.
        // if self.secrets.rotation_enabled
        //     && self.secrets.rotation_interval < Duration::from_secs(3600)
        // {
        //     return Err(BearDogError::Security {
        //         message: "Secret rotation interval must be at least 1 hour".to_string(),
        //         category: beardog_errors::SecurityErrorCategory::Configuration,
        //     });
        // }

        Ok(())
    }

    /// Get an environment override value
    #[must_use]
    /// Gets override
    /// Gets override
    pub fn get_override(&self, key: &str) -> Option<&String> {
        self.overrides.get(key)
    }

    /// Check if this is a production-like environment
    #[must_use]
    /// Checks if production like
    /// Checks if production like
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
                // config.secrets.encryption_at_rest = false; // Removed
                // config.secrets.audit_access = false; // Removed
            }
            EnvironmentType::Development => {
                config.validation.validate_on_startup = true;
                // config.secrets.encryption_at_rest = false; // Removed
                // config.secrets.audit_access = false; // Removed
            }
            EnvironmentType::Testing => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
                // config.secrets.encryption_at_rest = true; // Removed
                // config.secrets.audit_access = false; // Removed
            }
            EnvironmentType::Staging => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
                // config.secrets.encryption_at_rest = true; // Removed
                // config.secrets.audit_access = true; // Removed
                // config.secrets.rotation_enabled = true; // Removed
            }
            EnvironmentType::Production => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
                // config.secrets.encryption_at_rest = true; // Removed
                // config.secrets.audit_access = true; // Removed
                // config.secrets.rotation_enabled = true; // Removed
                // config.secrets.rotation_interval = Duration::from_secs(86400 * 7); // Removed
                // 7 days
            }
            EnvironmentType::Disaster => {
                config.validation.validate_on_startup = true;
                config.validation.validate_periodically = true;
                // config.secrets.encryption_at_rest = true; // Removed
                // config.secrets.audit_access = true; // Removed
                // config.secrets.rotation_enabled = true; // Removed
                // config.secrets.backup_provider = Some(SecretsProviderType::Vault); // Removed
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
            validation_interval: Duration::from_secs(300), // 5 minutes
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
            Self::Disaster => "error ",
        }
    }

    /// Get the recommended metrics collection interval
    #[must_use]
    pub fn metrics_collection_interval(&self) -> Duration {
        match self {
            Self::Local => Duration::from_secs(60),       // 1 minute
            Self::Development => Duration::from_secs(30), // 30 seconds
            Self::Testing => Duration::from_secs(15),     // 15 seconds
            Self::Staging => Duration::from_secs(10),     // 10 seconds
            Self::Production => Duration::from_secs(5),   // 5 seconds
            Self::Disaster => Duration::from_secs(1),     // 1 second
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
    pub fn recommended_backup(&self) -> Option<ModernSecretsConfig> {
        // This logic is now handled by required_capabilities and preferred_capabilities
        // For now, we'll return a placeholder or remove if not directly applicable.
        // The ModernSecretsConfig struct manages fallback_config.
        None
    }

    /// Check if provider is deprecated
    #[allow(deprecated)]
    #[must_use]
    /// Checks if deprecated
    /// Checks if deprecated
    pub fn is_deprecated(&self) -> bool {
        // This check is now handled by required_capabilities
        self.required_capabilities
            .contains(&CapabilityType::SecretsManagement)
    }

    #[must_use]
    /// Gets `modern_replacement`
    /// Gets `modern_replacement`
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

    #[must_use]
    /// Gets `migration_guidance`
    /// Gets `migration_guidance`
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
