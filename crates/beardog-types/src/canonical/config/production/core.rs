// Core Production Configuration
//
// This module contains essential production settings, service identification,
// and production feature flags that form the foundation of the production configuration.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **PRODUCTION CORE CONFIGURATION** - Essential production settings
///
/// and production feature flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCoreConfig {
    /// Production environment level
    /// The environment level value
    pub environment_level: EnvironmentLevel,

    /// Service identification
    /// Name of the service
    pub service_name: String,
    /// Service Version
    /// The service version value
    pub service_version: String,
    /// Deployment Id
    pub deployment_id: String,

    /// Geographic deployment
    /// The region value
    pub region: String,
    /// Availability Zone
    /// Optional availability zone
    pub availability_zone: Option<String>,
    /// Cluster Id
    pub cluster_id: String,
    /// Node Id
    pub node_id: String,

    /// Production feature flags
    /// The feature flags value
    pub feature_flags: ProductionFeatureFlags,
}

/// Production environment levels with increasing criticality
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EnvironmentLevel {
    /// Development variant
    Development,
    /// Testing variant
    Testing,
    /// Staging variant
    Staging,
    /// `PreProduction` variant
    PreProduction,
    /// Production variant
    Production,
    /// Critical variant
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionFeatureFlags {
    /// Enable advanced monitoring and observability
    /// Whether `enable_advanced_monitoring` is enabled
    pub enable_advanced_monitoring: bool,
    /// Enable distributed tracing across services
    /// Whether `enable_distributed_tracing` is enabled
    pub enable_distributed_tracing: bool,
    pub enable_performance_profiling: bool,
    /// Enable comprehensive security auditing
    /// Whether `enable_security_auditing` is enabled
    pub enable_security_auditing: bool,
    /// Enable automatic scaling based on load
    /// Whether `enable_auto_scaling` is enabled
    pub enable_auto_scaling: bool,
    /// Whether `enable_circuit_breakers` is enabled
    pub enable_circuit_breakers: bool,
    /// Whether `enable_rate_limiting` is enabled
    pub enable_rate_limiting: bool,
    /// Whether `enable_caching` is enabled
    pub enable_caching: bool,
    /// Whether `enable_compression` is enabled
    pub enable_compression: bool,
    /// Whether `enable_encryption_at_rest` is enabled
    pub enable_encryption_at_rest: bool,
}

impl Default for ProductionCoreConfig {
    fn default() -> Self {
        Self {
            environment_level: EnvironmentLevel::Development,
            service_name: "beardog ".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            deployment_id: uuid::Uuid::new_v4().to_string(),
            region: "default".to_string(),
            availability_zone: None,
            cluster_id: "default-cluster".to_string(),
            node_id: uuid::Uuid::new_v4().to_string(),
            feature_flags: ProductionFeatureFlags::default(),
        }
    }
}

impl Default for ProductionFeatureFlags {
    fn default() -> Self {
        Self {
            enable_advanced_monitoring: true,
            enable_distributed_tracing: true,
            enable_performance_profiling: false, // Disabled by default for performance
            enable_security_auditing: true,
            enable_auto_scaling: false, // Requires careful configuration
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: true,
            enable_compression: true,
            enable_encryption_at_rest: true,
        }
    }
}

impl ProductionCoreConfig {
    /// Create a new production core configuration
    #[must_use]
    /// Creates a new instance
    pub fn new(service_name: &str, service_version: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            service_version: service_version.to_string(),
            ..Default::default()
        }
    }

    /// Set the environment level
    #[must_use]
    /// Creates instance with environment level
    pub fn with_environment_level(mut self, level: EnvironmentLevel) -> Self {
        self.environment_level = level;
        self
    }

    /// Set the deployment region
    #[must_use]
    /// Creates instance with region
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = region.to_string();
        self
    }

    /// Set the availability zone
    #[must_use]
    /// Creates instance with availability zone
    pub fn with_availability_zone(mut self, az: Option<&str>) -> Self {
        self.availability_zone = az.map(String::from);
        self
    }

    /// Set the cluster ID
    #[must_use]
    /// Creates instance with cluster id
    pub fn with_cluster_id(mut self, cluster_id: &str) -> Self {
        self.cluster_id = cluster_id.to_string();
        self
    }

    /// Validate the core configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.service_name.is_empty() {
            return Err(BearDogError::Business {
                message: "Service name cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.service_version.is_empty() {
            return Err(BearDogError::Business {
                message: "Service version cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.deployment_id.is_empty() {
            return Err(BearDogError::Business {
                message: "Deployment ID cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.region.is_empty() {
            return Err(BearDogError::Business {
                message: "Region cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.cluster_id.is_empty() {
            return Err(BearDogError::Business {
                message: "Cluster ID cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        if self.node_id.is_empty() {
            return Err(BearDogError::Business {
                message: "Node ID cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        Ok(())
    }

    /// Check if this is a production environment
    #[must_use]
    /// Checks if production
    /// Checks if production
    pub fn is_production(&self) -> bool {
        matches!(
            self.environment_level,
            EnvironmentLevel::Production | EnvironmentLevel::Critical
        )
    }

    /// Get a unique service identifier
    #[must_use]
    pub fn service_identifier(&self) -> String {
        format!(
            "{}:{}:{}",
            self.service_name, self.service_version, self.deployment_id
        )
    }

    /// Get geographic location string
    #[must_use]
    pub fn location(&self) -> String {
        match &self.availability_zone {
            Some(az) => format!("{}:{}", self.region, az),
            None => self.region.clone(),
        }
    }
}

impl EnvironmentLevel {
    /// Check if this environment level requires production-grade settings
    #[must_use]
    /// Checks if production grade
    /// Checks if production grade
    pub fn is_production_grade(&self) -> bool {
        matches!(
            self,
            Self::PreProduction | Self::Production | Self::Critical
        )
    }

    #[must_use]
    pub fn required_uptime(&self) -> f64 {
        match self {
            Self::Development => 0.95,    // 95%
            Self::Testing => 0.98,        // 98%
            Self::Staging => 0.99,        // 99%
            Self::PreProduction => 0.995, // 99.5%
            Self::Production => 0.999,    // 99.9%
            Self::Critical => 0.9999,     // 99.99%
        }
    }

    /// Get the maximum allowed downtime per month
    #[must_use]
    pub fn max_downtime_minutes_per_month(&self) -> f64 {
        let minutes_per_month = 30.0 * 24.0 * 60.0; // ~43,200 minutes
        minutes_per_month * (1.0 - self.required_uptime())
    }
}

impl ProductionFeatureFlags {
    /// Create production-optimized feature flags
    #[must_use]
    pub fn production() -> Self {
        Self {
            enable_advanced_monitoring: true,
            enable_distributed_tracing: true,
            enable_performance_profiling: true,
            enable_security_auditing: true,
            enable_auto_scaling: true,
            enable_circuit_breakers: true,
            enable_rate_limiting: true,
            enable_caching: true,
            enable_compression: true,
            enable_encryption_at_rest: true,
        }
    }

    /// Create development-optimized feature flags
    #[must_use]
    pub fn development() -> Self {
        Self {
            enable_advanced_monitoring: false,
            enable_distributed_tracing: false,
            enable_performance_profiling: true,
            enable_security_auditing: false,
            enable_auto_scaling: false,
            enable_circuit_breakers: false,
            enable_rate_limiting: false,
            enable_caching: false,
            enable_compression: false,
            enable_encryption_at_rest: false,
        }
    }

    #[must_use]
    pub fn has_performance_impact(&self) -> bool {
        self.enable_distributed_tracing
            || self.enable_performance_profiling
            || self.enable_security_auditing
            || self.enable_compression
    }
}
