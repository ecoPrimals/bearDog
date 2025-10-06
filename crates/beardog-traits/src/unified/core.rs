// Core Trait Definitions
//
// This module contains the fundamental traits that form the foundation
// of the unified BearDog trait system.

use beardog_errors::BearDogError;
use beardog_types::canonical::config::r#trait::BearDogConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Identifiable: Send + Sync {
    /// Get the unique identifier
    fn id(&self) -> &str;

    /// Get the entity type
    fn entity_type(&self) -> &str;

    /// Get a human-readable name
    fn name(&self) -> &str {
        self.id()
    }

    /// Get entity metadata
    fn metadata(&self) -> HashMap<String, String> {
        HashMap::new()
    }
}

pub trait Configurable: Identifiable {
    /// Associated configuration type
    type Config: BearDogConfig;

    /// Apply configuration
    fn configure(
        &mut self,
        config: Self::Config,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Get current configuration
    /// Gets config
    fn get_config(&self) -> &Self::Config;

    /// Validate configuration
    /// Validates config
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Reload configuration
    fn reload_config(
        &mut self,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}

pub trait Versionable: Identifiable {
    /// Get the current version
    fn version(&self) -> &str;

    /// Get the schema version
    fn schema_version(&self) -> u32 {
        1
    }

    /// Check compatibility with another version
    /// Checks if compatible with
    fn is_compatible_with(&self, other_version: u32) -> bool {
        self.schema_version() == other_version
    }
}

pub trait Serializable: Identifiable {
    /// Serialize to JSON
    /// Converts to json
    fn to_json(&self) -> Result<String, BearDogError>;

    /// Serialize to TOML
    /// Converts to toml
    fn to_toml(&self) -> Result<String, BearDogError>;

    /// Converts to binary
    fn to_binary(&self) -> Result<Vec<u8>, BearDogError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    /// Collection of errors
    pub errors: Vec<String>,
    /// Collection of warnings
    pub warnings: Vec<String>,
}

pub trait Validatable: Identifiable {
    /// Validates deep
    fn validate_deep(
        &self,
    ) -> impl std::future::Future<Output = Result<ValidationResult, BearDogError>> + Send;

    /// Batch validate multiple configurations
    #[must_use]
    fn batch_validate(
        configs: Vec<Self::Config>,
    ) -> impl std::future::Future<Output = Result<ValidationResult, BearDogError>> + Send
    where
        Self: Configurable,
    {
        async move {
            let mut result = ValidationResult {
                valid: true,
                errors: Vec::new(),
                warnings: Vec::new(),
            };

            for _config in configs {
                // Placeholder validation logic
                result
                    .warnings
                    .push("Batch validation not fully implemented".to_string());
            }

            Ok(result)
        }
    }
}

pub trait Lifecycle: Identifiable {
    /// Start the service
    /// Starts service
    fn start(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Stop the service gracefully
    /// Stops service
    fn stop(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async move { Ok(()) }
    }

    /// Restart the service
    fn restart(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async move {
            self.stop().await?;
            self.start().await
        }
    }

    /// Pause the service
    fn pause(&mut self) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}

pub trait HealthMonitored: Identifiable {
    /// Associated health status type
    type Health: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Health, BearDogError>> + Send;

    /// Check if the component is ready
    /// Checks if ready
    fn is_ready(&self) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send {
        async move { Ok(true) }
    }
}

pub trait MetricsCollector: Identifiable {
    /// Associated metrics type
    type Metrics: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Collect current metrics
    fn collect_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Metrics, BearDogError>> + Send;

    fn performance_score(
        &self,
    ) -> impl std::future::Future<Output = Result<f64, BearDogError>> + Send;
}

/// Validation utilities
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validates entity ID format and constraints
    ///
    /// # Errors
    ///
    /// Returns `BearDogError::Business` if:
    /// - Entity ID is empty
    /// - Entity ID contains invalid characters
    /// - Entity ID exceeds maximum length
    pub fn validate_id(entity_id: &str) -> Result<(), BearDogError> {
        // Ultra-pedantic security: Check for null bytes and control characters
        if entity_id.contains('\0') {
            return Err(BearDogError::Security {
                message: "Entity ID contains null bytes - potential security risk".to_string(),
                category: beardog_errors::SecurityErrorCategory::General,
            });
        }

        if entity_id.is_empty() {
            return Err(BearDogError::Business {
                message: "Entity ID cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Ultra-pedantic: Minimum length requirement
        if entity_id.len() < 3 {
            return Err(BearDogError::Business {
                message: "Entity ID too short (minimum 3 characters)".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if entity_id.len() > 255 {
            return Err(BearDogError::Business {
                message: "Entity ID too long (max 255 characters)".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Ultra-pedantic: Check for control characters
        if entity_id.chars().any(|c| c.is_control()) {
            return Err(BearDogError::Security {
                message: "Entity ID contains control characters - potential security risk"
                    .to_string(),
                category: beardog_errors::SecurityErrorCategory::General,
            });
        }

        if !entity_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(BearDogError::Business {
                message: "Entity ID contains invalid characters (only alphanumeric, hyphens, and underscores allowed)".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(())
    }
}
