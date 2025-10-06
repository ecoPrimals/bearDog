// Canonical Compliance Configuration
//
// This module provides both simple and consolidated compliance configurations:
// - CanonicalComplianceConfig: Simple compliance config
// - ConsolidatedComplianceConfiguration: Rich compliance config (from domains/)

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// Re-export consolidated compliance types from domains module
pub use super::domains::compliance::{
    ComplianceStandard, ConsolidatedComplianceConfiguration, DataSovereigntyConfiguration,
    PrivacyAuditConfiguration, ReportFormat, ReportFrequency, ReportingConfiguration,
};

/// Compliance framework types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceFramework {
    /// General Data Protection Regulation
    Gdpr,
    /// Health Insurance Portability and Accountability Act
    Hipaa,
    /// Payment Card Industry Data Security Standard
    PciDss,
}

/// Compliance enablement status
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Compliance is disabled
    #[default]
    /// Inactive or disabled state
    Disabled,
    /// Compliance is enabled
    Enabled,
}

/// Canonical compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalComplianceConfig {
    /// Overall compliance status
    /// Current status of the component
    pub status: ComplianceStatus,
    /// Enabled compliance frameworks
    /// The frameworks value
    pub frameworks: HashSet<ComplianceFramework>,
    /// Data retention period in days
    /// Number of `data_retention_days`
    pub data_retention_days: u32,
}

impl CanonicalComplianceConfig {
    /// Check if a specific framework is enabled
    /// Checks if framework enabled
    /// Checks if framework enabled
    pub fn is_framework_enabled(&self, framework: &ComplianceFramework) -> bool {
        self.status == ComplianceStatus::Enabled && self.frameworks.contains(framework)
    }

    /// Enable a compliance framework
    pub fn enable_framework(&mut self, framework: ComplianceFramework) {
        self.status = ComplianceStatus::Enabled;
        self.frameworks.insert(framework);
    }

    /// Disable a compliance framework
    pub fn disable_framework(&mut self, framework: &ComplianceFramework) {
        self.frameworks.remove(framework);
        if self.frameworks.is_empty() {
            self.status = ComplianceStatus::Disabled;
        }
    }
}

pub type ComplianceConfig = CanonicalComplianceConfig;
