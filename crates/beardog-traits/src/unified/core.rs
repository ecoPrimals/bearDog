// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cross-cutting building blocks: identity, configuration, serialization, validation, lifecycle, and probes.

use beardog_errors::BearDogError;
use beardog_types::canonical::config::r#trait::BearDogConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Anything that can be named, typed, and tagged with loose metadata.
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

/// Component whose runtime settings are loaded from a [`BearDogConfig`] implementor.
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

/// Exposes semantic and schema versions for migrations and compatibility gates.
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

/// Serializes entities to multiple wire formats for storage and IPC.
pub trait Serializable: Identifiable {
    /// Serialize to JSON
    /// Converts to json
    ///
    /// # Errors
    ///
    /// Implementations return [`BearDogError`] when serialization fails.
    fn to_json(&self) -> Result<String, BearDogError>;

    /// Serialize to TOML
    /// Converts to toml
    ///
    /// # Errors
    ///
    /// Implementations return [`BearDogError`] when serialization fails.
    fn to_toml(&self) -> Result<String, BearDogError>;

    /// Converts to binary
    ///
    /// # Errors
    ///
    /// Implementations return [`BearDogError`] when serialization fails.
    fn to_binary(&self) -> Result<Vec<u8>, BearDogError>;
}

/// Aggregated output of deep or batch validation passes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// True when `errors` is empty.
    pub valid: bool,
    /// Collection of errors
    pub errors: Vec<String>,
    /// Collection of warnings
    pub warnings: Vec<String>,
}

/// Runs structural validation beyond type checking (requires [`Configurable`] for batch helpers).
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

            for (i, config) in configs.iter().enumerate() {
                if let Err(e) = config.validate() {
                    result.valid = false;
                    result.errors.push(format!("[config {i}] {e}"));
                }
            }

            Ok(result)
        }
    }
}

/// Minimal start/stop/restart contract for daemons and workers.
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

/// Surfaces a typed health snapshot for orchestrators and load balancers.
pub trait HealthMonitored: Identifiable {
    /// Structured health payload (often serialized to JSON for probes).
    type Health: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Produces the latest health view; should be cheap enough for periodic polling.
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Health, BearDogError>> + Send;

    /// Check if the component is ready
    /// Checks if ready
    fn is_ready(&self) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send {
        async move { Ok(true) }
    }
}

/// Periodically samples operational metrics for autoscaling and SLO tracking.
pub trait MetricsCollector: Identifiable {
    /// Metrics bundle returned to monitoring pipelines.
    type Metrics: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Collect current metrics
    fn collect_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<Self::Metrics, BearDogError>> + Send;

    /// Single scalar summarizing “how well” the component is performing (implementation-defined).
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
        if entity_id.chars().any(char::is_control) {
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
