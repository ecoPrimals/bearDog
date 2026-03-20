// SPDX-License-Identifier: AGPL-3.0-only

// Unified Canonical HSM Configuration
//
// This module provides a comprehensive HSM (Hardware Security Module) configuration system that
// consolidates all HSM patterns across BearDog into a single, modern, production-ready structure.

use serde::{Deserialize, Serialize};

// HSM module organization
/// Cloud providers module
pub mod cloud;
/// Connection module
pub mod connection;
/// Core module
/// Core functionality
/// Core functionality
pub mod core;
/// Key management module
pub mod key_management;
/// Migration module
pub mod migration;
/// Mobile module
pub mod mobile;
/// Monitoring module
pub mod monitoring;
/// Throughput ceilings, batching, and latency SLOs for the unified HSM stack.
pub mod performance;
/// Platform module
pub mod platform;
/// Providers module
pub mod providers;
/// Security module
pub mod security;

// Re-export all types
pub use cloud::*;
pub use connection::*;
pub use core::*;
pub use key_management::*;
pub use migration::*;
pub use mobile::*;
pub use monitoring::*;
pub use performance::*;
pub use platform::*;
pub use providers::*;
pub use security::*;

/// Unified canonical HSM configuration - Single source of truth
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalHsmConfig {
    /// Core HSM settings
    /// The core value
    pub core: HsmCoreConfig,

    /// HSM provider configuration
    pub provider: HsmProviderConfig,

    /// Connection management
    /// The connection value
    pub connection: HsmConnectionConfig,

    /// Security configuration
    /// The security value
    pub security: HsmSecurityConfig,

    /// Performance envelope (QPS, concurrency, caching) for crypto operations.
    pub performance: HsmPerformanceConfig,

    /// Monitoring configuration
    /// The monitoring value
    pub monitoring: HsmMonitoringConfig,

    /// Mobile HSM configuration
    /// The mobile value
    pub mobile: MobileHsmConfig,
}

///
/// Provides backward compatibility while maintaining the canonical
pub type HsmConfig = CanonicalHsmConfig;
