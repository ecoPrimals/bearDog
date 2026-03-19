// SPDX-License-Identifier: AGPL-3.0-only

//! # Consolidated Monitoring Configuration Domain
//!
//! This module consolidates ALL monitoring-related configuration structs across the BearDog
//! ecosystem into a single, unified monitoring configuration system.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CONSOLIDATED MONITORING CONFIGURATION** - Single source of truth for all monitoring settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedMonitoringConfiguration {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval seconds
    pub metrics_interval_seconds: u64,
    /// Alert configuration
    pub alerts: HashMap<String, AlertConfiguration>,
}

/// Alert configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfiguration {
    /// Enable alerts
    pub enabled: bool,
    /// Alert threshold
    pub threshold: f64,
}

impl Default for ConsolidatedMonitoringConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval_seconds: 60,
            alerts: HashMap::new(),
        }
    }
}

impl ConsolidatedMonitoringConfiguration {
    /// Validate monitoring configuration
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.metrics_interval_seconds == 0 {
            return Err(BearDogError::configuration("Metrics interval cannot be zero"));
        }
        Ok(())
    }
    
    /// Create development configuration
    pub fn development() -> Self {
        Self::default()
    }
    
    /// Create production configuration
    pub fn production() -> Self {
        Self::default()
    }
}

// =============================================================================
// BACKWARD COMPATIBILITY TYPE ALIASES - Phase 2 Consolidation (Nov 11, 2025)
// =============================================================================
//
// These aliases consolidate 41 MonitoringConfig variants found across:
// - beardog-monitoring (8 variants)
// - beardog-core (3 variants)
// - beardog-types/canonical/* (18 variants)
// - beardog-adapters (2 variants)
// - beardog-tunnel (1 variant)
// - beardog-production (2 variants)
// - beardog-security (1 variant)
// - beardog-config (2 variants)
// - beardog-genetics (1 variant)
//
// All monitoring configs now point to ConsolidatedMonitoringConfiguration
// with domain-specific extensions where needed.

/// Primary MonitoringConfig alias
///
/// **CONSOLIDATED (Nov 11, 2025)**: Use `ConsolidatedMonitoringConfiguration` for new code.
pub type MonitoringConfig = ConsolidatedMonitoringConfiguration;

/// System monitoring configuration alias
pub type SystemMonitorConfig = ConsolidatedMonitoringConfiguration;

/// AI monitoring configuration alias  
pub type AIMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// AI management monitoring configuration alias
pub type AIManagementMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Adapter monitoring configuration alias
pub type AdapterMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Monitoring handler configuration alias
pub type MonitoringHandlerConfig = ConsolidatedMonitoringConfiguration;

/// Tunnel monitoring configuration alias
pub type TunnelMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Provider monitoring configuration alias
pub type ProviderMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Performance monitoring configuration alias
pub type PerformanceMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// HSM monitoring configuration alias
pub type HsmMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Security monitoring configuration alias
pub type SecurityMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Health monitoring configuration alias
pub type HealthMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Monitoring core configuration alias
pub type MonitoringCoreConfig = ConsolidatedMonitoringConfiguration;

/// Canonical monitoring configuration alias (from deprecated module)
pub type CanonicalMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Improved monitoring configuration alias
pub type ImprovedMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Monitoring configuration builder
pub type MonitoringConfigBuilder = ConsolidatedMonitoringConfiguration;

/// Unified security monitoring config alias
pub type UnifiedSecurityMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Unified performance monitoring config alias
pub type UnifiedPerformanceMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Core monitoring config alias
pub type CoreMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Unified integration monitoring config alias
pub type UnifiedIntegrationMonitoringConfig = ConsolidatedMonitoringConfiguration;

/// Monitoring exporters configuration alias
pub type MonitoringExportersConfig = ConsolidatedMonitoringConfiguration;

/// Entropy monitoring configuration (domain-specific, keeps separate structure)
pub use crate::canonical::config::domains::genetics::EntropyMonitoringConfig;

/// Sovereignty monitoring configuration (domain-specific, keeps separate structure)
///
/// Note: This is specific to sovereignty compliance monitoring and maintains
/// its own structure in beardog-monitoring crate.
// Re-exported from beardog-monitoring when needed

/// Health monitor configuration alias  
pub type HealthMonitorConfig = ConsolidatedMonitoringConfiguration;

// =============================================================================
// MIGRATION HELPERS
// =============================================================================

impl ConsolidatedMonitoringConfiguration {
    /// Create from monitoring interval (common pattern)
    pub fn with_interval(interval_seconds: u64) -> Self {
        Self {
            enabled: true,
            metrics_interval_seconds: interval_seconds,
            alerts: HashMap::new(),
        }
    }

    /// Enable with default settings
    pub fn enabled() -> Self {
        Self {
            enabled: true,
            ..Default::default()
        }
    }

    /// Disabled configuration
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
} 