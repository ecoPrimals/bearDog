// SPDX-License-Identifier: AGPL-3.0-only

// Canonical Production Configuration - Modular Architecture
//
// This module provides the **single source of truth** for all production configuration
// across the BearDog ecosystem. The configuration is organized into logical domain modules
// for better maintainability and adherence to the 2000-line limit per file.
//
// ## Modular Architecture
//
// Production configuration is split into focused domain modules:
// - **core**: Essential production settings and service identification
// - **environment**: Environment-specific configuration and validation
// - **resources**: System resource management (memory, CPU, network, storage)
// - **operations**: Operational procedures (health checks, maintenance, backup)
// - **deployment**: Deployment and release management strategies
// - **observability**: Production monitoring, logging, and alerting
//
// ## Migration from Monolithic File
//
// This modular structure replaces the previous 1,436-line monolithic production.rs file,
// improving maintainability while preserving all functionality and API compatibility.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

// Domain-specific production configuration modules
/// Core module
/// Core functionality
/// Core functionality
pub mod core;
/// Deployment module
pub mod deployment;
/// Environment module
pub mod environment;
/// Observability module
pub mod observability;
/// Operations module
pub mod operations;
/// Resources module
pub mod resources;

// Re-export all configuration types for API compatibility
pub use core::*;
pub use deployment::*;
pub use environment::*;
pub use observability::*;
pub use operations::*;
pub use resources::*;

// Explicit re-exports for backward compatibility
pub use core::{EnvironmentLevel, ProductionFeatureFlags};
#[allow(deprecated)]
pub use environment::{EnvironmentType, EnvironmentValidation, ModernSecretsConfig};

///
/// across the `BearDog` ecosystem while maintaining backward compatibility.
///
/// Note: CanonicalProductionConfig type alias removed. Use UnifiedProductionConfig directly.
/// **UNIFIED PRODUCTION CONFIGURATION** - Primary production config
///
/// This consolidates all production configurations into a single, comprehensive system
/// that provides enterprise-grade operational capabilities across all environments.
///
/// - Core production settings and service identification
/// - Environment-specific configuration and secrets management
/// - System resource management and capacity planning
/// - Operational procedures and disaster recovery
/// - Deployment strategies and release management
/// - Production observability and monitoring
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnifiedProductionConfig {
    /// **CORE PRODUCTION SETTINGS**
    /// The core value
    pub core: ProductionCoreConfig,

    /// **ENVIRONMENT CONFIGURATION**
    /// The environment value
    pub environment: EnvironmentConfig,

    /// **RESOURCE MANAGEMENT**
    /// The resources value
    pub resources: ResourceManagementConfig,

    /// **OPERATIONAL SETTINGS**
    /// The operations value
    pub operations: OperationalConfig,

    /// **DEPLOYMENT CONFIGURATION**
    /// The deployment value
    pub deployment: DeploymentConfig,

    /// **OBSERVABILITY CONFIGURATION**
    /// The observability value
    pub observability: ObservabilityConfig,
}

impl UnifiedProductionConfig {
    /// Create a new production configuration with sensible defaults
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate the entire production configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        self.core.validate()?;
        self.environment.validate()?;
        self.resources.validate()?;
        self.operations.validate()?;
        self.deployment.validate()?;
        self.observability.validate()?;
        Ok(())
    }

    /// Get the production environment level
    #[must_use]
    pub fn environment_level(&self) -> &EnvironmentLevel {
        &self.core.environment_level
    }

    /// Check if this is a production environment
    #[must_use]
    /// Checks if production
    /// Checks if production
    pub fn is_production(&self) -> bool {
        matches!(self.core.environment_level, EnvironmentLevel::Production)
    }

    #[must_use]
    pub fn service_info(&self) -> (&str, &str, &str) {
        (
            &self.core.service_name,
            &self.core.service_version,
            &self.core.deployment_id,
        )
    }
}
