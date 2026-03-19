// SPDX-License-Identifier: AGPL-3.0-only

//! Production Ecosystem Builder
//!
//! Fluent builder API for constructing `ProductionEcosystem` instances with
//! validated configuration and sensible defaults.
//!
//! # Design Philosophy
//!
//! - **Fluent API**: Chain methods for readable configuration
//! - **Validation**: Catch configuration errors at build time
//! - **Defaults**: Sensible defaults for all optional settings
//! - **Type Safety**: Compile-time guarantees for required fields
//!
//! # Examples
//!
//! ## Basic Usage
//! ```rust,ignore
//! use beardog_types::production::builder::ProductionEcosystemBuilder;
//!
//! let ecosystem = ProductionEcosystemBuilder::new()
//!     .with_service_name("beardog-api")
//!     .with_region("us-west-2")
//!     .build()?;
//! ```
//!
//! ## Advanced Configuration
//! ```rust,ignore
//! use beardog_types::production::{
//!     builder::ProductionEcosystemBuilder,
//!     types::EnvironmentLevel,
//! };
//!
//! let ecosystem = ProductionEcosystemBuilder::new()
//!     .with_environment(EnvironmentLevel::Production)
//!     .with_service_name("beardog-security")
//!     .with_region("us-east-1")
//!     .with_monitoring_enabled(true)
//!     .with_auto_scaling(true)
//!     .build()?;
//! ```

use super::{
    config::{ProductionConfig, ProductionFlags},
    ecosystem::ProductionEcosystem,
    types::EnvironmentLevel,
};
use beardog_errors::BearDogError;

// ============================================================================
// BUILDER
// ============================================================================

/// Builder for `ProductionEcosystem` with fluent configuration API
///
/// Provides a convenient way to construct production ecosystems with validated
/// configuration. The builder pattern ensures all required fields are set and
/// applies sensible defaults for optional settings.
///
/// # Builder Methods
///
/// - `with_environment()` - Set environment level
/// - `with_service_name()` - Set service identifier
/// - `with_region()` - Set cloud region
/// - `with_monitoring_enabled()` - Enable/disable monitoring
/// - `with_auto_scaling()` - Enable/disable auto-scaling
/// - `build()` - Construct the ecosystem
///
/// # Examples
///
/// ## Minimal Configuration
/// ```rust,ignore
/// use beardog_types::production::builder::ProductionEcosystemBuilder;
///
/// let ecosystem = ProductionEcosystemBuilder::new()
///     .build()?; // Uses all defaults
/// ```
///
/// ## Production Configuration
/// ```rust,ignore
/// use beardog_types::production::{
///     builder::ProductionEcosystemBuilder,
///     types::EnvironmentLevel,
/// };
///
/// let ecosystem = ProductionEcosystemBuilder::new()
///     .with_environment(EnvironmentLevel::Production)
///     .with_service_name("beardog-api")
///     .with_region("us-west-2")
///     .with_monitoring_enabled(true)
///     .with_auto_scaling(true)
///     .build()?;
/// ```
///
/// ## Environment-Specific
/// ```rust,ignore
/// use beardog_types::production::{
///     builder::ProductionEcosystemBuilder,
///     types::EnvironmentLevel,
/// };
///
/// let env = EnvironmentLevel::from_env().unwrap_or(EnvironmentLevel::Development);
///
/// let ecosystem = ProductionEcosystemBuilder::new()
///     .with_environment(env)
///     .build()?;
/// ```
#[derive(Debug, Clone, Default)]
pub struct ProductionEcosystemBuilder {
    /// Production configuration (public for testing)
    pub config: ProductionConfig,
}

impl ProductionEcosystemBuilder {
    /// Create a new builder with default configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ProductionConfig::default(),
        }
    }

    /// Set the environment level
    ///
    /// # Arguments
    ///
    /// * `environment` - Environment level (Development, Staging, Production, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::{
    ///     builder::ProductionEcosystemBuilder,
    ///     types::EnvironmentLevel,
    /// };
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_environment(EnvironmentLevel::Production);
    /// ```
    #[must_use]
    pub fn with_environment(mut self, environment: EnvironmentLevel) -> Self {
        self.config.core.environment_level = environment;
        self
    }

    /// Alias for `with_environment` (backwards compatibility)
    #[must_use]
    pub fn environment_level(self, level: EnvironmentLevel) -> Self {
        self.with_environment(level)
    }

    /// Set the service name
    ///
    /// # Arguments
    ///
    /// * `name` - Service identifier (e.g., "beardog-api", "beardog-security")
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_service_name("beardog-api");
    /// ```
    #[must_use]
    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.config.core.service_name = name.into();
        self
    }

    /// Set service name and version (backwards compatibility)
    #[must_use]
    pub fn service(mut self, name: String, version: String) -> Self {
        self.config.core.service_name = name;
        self.config.core.service_version = version;
        self
    }

    /// Set the cloud region
    ///
    /// # Arguments
    ///
    /// * `region` - Region identifier (e.g., "us-west-2", "eu-central-1")
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_region("us-west-2");
    /// ```
    #[must_use]
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.config.core.region = region.into();
        self
    }

    /// Set the cluster ID
    ///
    /// # Arguments
    ///
    /// * `cluster_id` - Cluster identifier for orchestration platforms
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_cluster_id("prod-cluster-01");
    /// ```
    #[must_use]
    pub fn with_cluster_id(mut self, cluster_id: impl Into<String>) -> Self {
        self.config.core.cluster_id = cluster_id.into();
        self
    }

    /// Set deployment details (backwards compatibility)
    #[must_use]
    pub fn deployment(mut self, id: String, region: String, cluster: String) -> Self {
        self.config.core.deployment_id = id;
        self.config.core.region = region;
        self.config.core.cluster_id = cluster;
        self
    }

    /// Enable advanced features (backwards compatibility)
    #[must_use]
    pub fn enable_advanced_features(mut self) -> Self {
        self.config.core.flags.enable_advanced_monitoring = true;
        self.config.core.flags.enable_distributed_tracing = true;
        self.config.core.flags.enable_performance_profiling = true;
        self.config.core.flags.enable_security_auditing = true;
        self
    }

    /// Enable or disable monitoring
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable monitoring
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_monitoring_enabled(true);
    /// ```
    #[must_use]
    pub fn with_monitoring_enabled(mut self, enabled: bool) -> Self {
        self.config.core.flags.enable_advanced_monitoring = enabled;
        self
    }

    /// Enable or disable auto-scaling
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable auto-scaling
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_auto_scaling(true);
    /// ```
    #[must_use]
    pub fn with_auto_scaling(mut self, enabled: bool) -> Self {
        self.config.core.flags.enable_auto_scaling = enabled;
        self
    }

    /// Enable or disable distributed tracing
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable distributed tracing
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_distributed_tracing(true);
    /// ```
    #[must_use]
    pub fn with_distributed_tracing(mut self, enabled: bool) -> Self {
        self.config.core.flags.enable_distributed_tracing = enabled;
        self
    }

    /// Set production flags
    ///
    /// # Arguments
    ///
    /// * `flags` - Complete production flags configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::{
    ///     builder::ProductionEcosystemBuilder,
    ///     config::ProductionFlags,
    /// };
    ///
    /// let flags = ProductionFlags::production_defaults();
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_flags(flags);
    /// ```
    #[must_use]
    pub fn with_flags(mut self, flags: ProductionFlags) -> Self {
        self.config.core.flags = flags;
        self
    }

    /// Set complete production configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Complete production configuration
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::{
    ///     builder::ProductionEcosystemBuilder,
    ///     config::ProductionConfig,
    /// };
    ///
    /// let config = ProductionConfig::default();
    /// let builder = ProductionEcosystemBuilder::new()
    ///     .with_config(config);
    /// ```
    #[must_use]
    pub fn with_config(mut self, config: ProductionConfig) -> Self {
        self.config = config;
        self
    }

    /// Build the production ecosystem
    ///
    /// Validates the configuration and constructs a `ProductionEcosystem` instance.
    ///
    /// # Returns
    ///
    /// Returns the constructed ecosystem on success
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Configuration is invalid
    /// - Required subsystems fail to initialize
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use beardog_types::production::builder::ProductionEcosystemBuilder;
    ///
    /// let ecosystem = ProductionEcosystemBuilder::new()
    ///     .with_service_name("beardog-api")
    ///     .build()?;
    /// ```
    pub fn build(self) -> Result<ProductionEcosystem, BearDogError> {
        // Validate configuration
        self.validate()?;

        // Construct ecosystem
        ProductionEcosystem::new(self.config)
    }

    /// Validate the configuration
    ///
    /// Checks that all required fields are set and values are valid.
    ///
    /// # Errors
    ///
    /// Returns an error if configuration is invalid
    fn validate(&self) -> Result<(), BearDogError> {
        // Validate service name is not empty
        if self.config.core.service_name.is_empty() {
            return Err(BearDogError::validation("Service name cannot be empty"));
        }

        // Validate region is not empty
        if self.config.core.region.is_empty() {
            return Err(BearDogError::validation("Region cannot be empty"));
        }

        Ok(())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let builder = ProductionEcosystemBuilder::new();
        assert_eq!(
            builder.config.core.environment_level,
            EnvironmentLevel::Development
        );
    }

    #[test]
    fn test_builder_with_environment() {
        let builder =
            ProductionEcosystemBuilder::new().with_environment(EnvironmentLevel::Production);
        assert_eq!(
            builder.config.core.environment_level,
            EnvironmentLevel::Production
        );
    }

    #[test]
    fn test_builder_with_service_name() {
        let builder = ProductionEcosystemBuilder::new().with_service_name("test-service");
        assert_eq!(builder.config.core.service_name, "test-service");
    }

    #[test]
    fn test_builder_with_region() {
        let builder = ProductionEcosystemBuilder::new().with_region("us-west-2");
        assert_eq!(builder.config.core.region, "us-west-2");
    }

    #[test]
    fn test_builder_with_monitoring() {
        let builder = ProductionEcosystemBuilder::new().with_monitoring_enabled(false);
        assert!(!builder.config.core.flags.enable_advanced_monitoring);
    }

    #[test]
    fn test_builder_with_auto_scaling() {
        let builder = ProductionEcosystemBuilder::new().with_auto_scaling(true);
        assert!(builder.config.core.flags.enable_auto_scaling);
    }

    #[test]
    fn test_builder_fluent_api() {
        let builder = ProductionEcosystemBuilder::new()
            .with_environment(EnvironmentLevel::Production)
            .with_service_name("beardog-api")
            .with_region("us-west-2")
            .with_monitoring_enabled(true)
            .with_auto_scaling(true);

        assert_eq!(
            builder.config.core.environment_level,
            EnvironmentLevel::Production
        );
        assert_eq!(builder.config.core.service_name, "beardog-api");
        assert_eq!(builder.config.core.region, "us-west-2");
        assert!(builder.config.core.flags.enable_advanced_monitoring);
        assert!(builder.config.core.flags.enable_auto_scaling);
    }
}
