//! Canonical Compliance Configuration
//!
//! Regulatory compliance configuration for GDPR, HIPAA, PCI-DSS and other frameworks.
//!
//! # Overview
//!
//! This module provides comprehensive compliance configuration including:
//! - Multiple compliance framework support (GDPR, HIPAA, PCI-DSS)
//! - Data retention and sovereignty management
//! - Privacy auditing and reporting
//! - Framework enable/disable control
//!
//! Two configuration types are available:
//! - `CanonicalComplianceConfig`: Simple compliance config with framework toggles
//! - `ConsolidatedComplianceConfiguration`: Rich compliance config with full auditing
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::compliance::{
//!     CanonicalComplianceConfig, ComplianceFramework, ComplianceStatus,
//! };
//! use std::collections::HashSet;
//!
//! // Create GDPR compliance configuration
//! let mut config = CanonicalComplianceConfig {
//!     status: ComplianceStatus::Enabled,
//!     frameworks: HashSet::new(),
//!     data_retention_days: 90,
//! };
//!
//! config.enable_framework(ComplianceFramework::Gdpr);
//! assert!(config.is_framework_enabled(&ComplianceFramework::Gdpr));
//! ```
//!
//! # Multi-Framework Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::compliance::{
//!     CanonicalComplianceConfig, ComplianceFramework, ComplianceStatus,
//! };
//! use std::collections::HashSet;
//!
//! // Enable multiple compliance frameworks
//! let mut config = CanonicalComplianceConfig {
//!     status: ComplianceStatus::Enabled,
//!     frameworks: [
//!         ComplianceFramework::Gdpr,
//!         ComplianceFramework::Hipaa,
//!         ComplianceFramework::PciDss,
//!     ].iter().cloned().collect(),
//!     data_retention_days: 90,  // 90 days for GDPR
//! };
//!
//! assert!(config.is_framework_enabled(&ComplianceFramework::Gdpr));
//! assert!(config.is_framework_enabled(&ComplianceFramework::Hipaa));
//! ```
//!
//! # Data Retention Guidelines
//!
//! ```rust
//! use beardog_types::canonical::config::compliance::{
//!     CanonicalComplianceConfig, ComplianceFramework, ComplianceStatus,
//! };
//! use std::collections::HashSet;
//!
//! // GDPR compliance: Right to be forgotten (90 days typical)
//! let gdpr_config = CanonicalComplianceConfig {
//!     status: ComplianceStatus::Enabled,
//!     frameworks: [ComplianceFramework::Gdpr].iter().cloned().collect(),
//!     data_retention_days: 90,
//! };
//!
//! // HIPAA compliance: 6 years minimum
//! let hipaa_config = CanonicalComplianceConfig {
//!     status: ComplianceStatus::Enabled,
//!     frameworks: [ComplianceFramework::Hipaa].iter().cloned().collect(),
//!     data_retention_days: 2190,  // 6 years
//! };
//!
//! // PCI-DSS: 1 year for transaction logs
//! let pci_config = CanonicalComplianceConfig {
//!     status: ComplianceStatus::Enabled,
//!     frameworks: [ComplianceFramework::PciDss].iter().cloned().collect(),
//!     data_retention_days: 365,
//! };
//! ```
//!
//! # Framework Management
//!
//! ```rust
//! use beardog_types::canonical::config::compliance::{
//!     CanonicalComplianceConfig, ComplianceFramework, ComplianceStatus,
//! };
//!
//! // Start with default (disabled)
//! let mut config = CanonicalComplianceConfig::default();
//! assert_eq!(config.status, ComplianceStatus::Disabled);
//!
//! // Enable GDPR compliance
//! config.enable_framework(ComplianceFramework::Gdpr);
//! assert_eq!(config.status, ComplianceStatus::Enabled);
//!
//! // Disable framework
//! config.disable_framework(&ComplianceFramework::Gdpr);
//! assert_eq!(config.status, ComplianceStatus::Disabled);
//! ```
//!
//! # Compliance Framework Requirements
//!
//! | Framework | Data Retention | Key Requirements |
//! |-----------|----------------|------------------|
//! | GDPR | 90 days typical | Right to be forgotten, data portability, consent |
//! | HIPAA | 6 years minimum | PHI encryption, access logs, breach notification |
//! | PCI-DSS | 1 year (logs) | Cardholder data encryption, access control, monitoring |
//!
//! # Security Considerations
//!
//! - **GDPR**: Implement right to erasure, data portability, and consent management
//! - **HIPAA**: Encrypt PHI at rest and in transit, maintain access audit logs
//! - **PCI-DSS**: Never store full card numbers, CVV, or PINs; use tokenization
//! - **Data Retention**: Automatically purge data after retention period expires
//!
//! # Design Principles
//!
//! - **Flexibility**: Support multiple compliance frameworks simultaneously
//! - **Safety**: Default to disabled state; explicit opt-in required
//! - **Auditability**: Track which frameworks are active
//! - **Configurability**: Data retention periods fully configurable

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// Re-export consolidated compliance types from domains module
pub use super::domains::compliance::{
    ComplianceStandard, ConsolidatedComplianceConfiguration, DataSovereigntyConfiguration,
    PrivacyAuditConfiguration, ReportFormat, ReportFrequency, ReportingConfiguration,
};

/// Compliance framework types
///
/// Major regulatory frameworks supported by BearDog.
///
/// # Frameworks
///
/// * `Gdpr` - General Data Protection Regulation (EU)
/// * `Hipaa` - Health Insurance Portability and Accountability Act (US healthcare)
/// * `PciDss` - Payment Card Industry Data Security Standard
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::compliance::ComplianceFramework;
///
/// let gdpr = ComplianceFramework::Gdpr;
/// let hipaa = ComplianceFramework::Hipaa;
/// let pci = ComplianceFramework::PciDss;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceFramework {
    /// General Data Protection Regulation (GDPR)
    ///
    /// EU regulation for data protection and privacy.
    /// **Key requirements**: Right to be forgotten, data portability, consent management.
    /// **Typical retention**: 90 days (varies by use case).
    Gdpr,

    /// Health Insurance Portability and Accountability Act (HIPAA)
    ///
    /// US regulation for protected health information (PHI).
    /// **Key requirements**: PHI encryption, access logs, breach notification.
    /// **Minimum retention**: 6 years.
    Hipaa,

    /// Payment Card Industry Data Security Standard (PCI-DSS)
    ///
    /// Standard for organizations handling credit card data.
    /// **Key requirements**: Cardholder data encryption, access control, monitoring.
    /// **Log retention**: 1 year minimum.
    PciDss,
}

/// Compliance enablement status
///
/// Controls whether compliance checks and enforcement are active.
///
/// # States
///
/// * `Disabled` - Compliance checks are not performed (default)
/// * `Enabled` - Compliance checks and enforcement are active
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::compliance::ComplianceStatus;
///
/// let disabled = ComplianceStatus::Disabled;  // Default
/// let enabled = ComplianceStatus::Enabled;
///
/// assert_eq!(ComplianceStatus::default(), ComplianceStatus::Disabled);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub enum ComplianceStatus {
    /// Compliance is disabled (default)
    ///
    /// Compliance checks are not performed. Use only for testing or
    /// non-production environments.
    #[default]
    Disabled,

    /// Compliance is enabled
    ///
    /// All configured compliance frameworks are actively enforced.
    Enabled,
}

/// Canonical compliance configuration
///
/// Simple compliance configuration with framework toggles and data retention.
/// For more comprehensive compliance features, see `ConsolidatedComplianceConfiguration`.
///
/// # Fields
///
/// * `status` - Whether compliance enforcement is active
/// * `frameworks` - Set of enabled compliance frameworks
/// * `data_retention_days` - How long to retain data before automatic deletion
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::compliance::{
///     CanonicalComplianceConfig, ComplianceFramework, ComplianceStatus,
/// };
/// use std::collections::HashSet;
///
/// let mut config = CanonicalComplianceConfig {
///     status: ComplianceStatus::Enabled,
///     frameworks: HashSet::new(),
///     data_retention_days: 90,
/// };
///
/// config.enable_framework(ComplianceFramework::Gdpr);
/// ```
///
/// # Data Retention Guidelines
///
/// - **GDPR**: 90 days typical (varies by purpose)
/// - **HIPAA**: 2190 days minimum (6 years)
/// - **PCI-DSS**: 365 days for transaction logs
///
/// # Thread Safety
///
/// This type is `Send + Sync` and can be safely shared across threads.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalComplianceConfig {
    /// Overall compliance status
    ///
    /// Controls whether compliance checks are performed.
    /// Defaults to `Disabled` for safety.
    pub status: ComplianceStatus,

    /// Enabled compliance frameworks
    ///
    /// Set of frameworks that are actively enforced.
    /// Empty set means no frameworks are enforced.
    pub frameworks: HashSet<ComplianceFramework>,

    /// Data retention period in days
    ///
    /// Number of days to retain data before automatic deletion.
    /// Set according to the strictest framework requirement:
    /// - GDPR: 90 days typical
    /// - HIPAA: 2190 days (6 years)
    /// - PCI-DSS: 365 days (1 year for logs)
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
