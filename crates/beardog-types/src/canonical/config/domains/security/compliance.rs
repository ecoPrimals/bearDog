// SPDX-License-Identifier: AGPL-3.0-only

//! Security Compliance and Data Sovereignty Configuration
//!
//! This module provides compliance validation, reporting, and data sovereignty
//! configuration structures for the `BearDog` security system.

use serde::{Deserialize, Serialize};

/// Security compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityComplianceConfiguration {
    /// Compliance standards to adhere to
    pub standards: Vec<String>,
    /// Compliance validation settings
    pub validation: ComplianceValidationConfiguration,
    /// Reporting configuration
    pub reporting: ComplianceReportingConfiguration,
    /// Audit requirements
    pub audit_requirements: Vec<String>,
}

/// Compliance validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceValidationConfiguration {
    /// Enable compliance validation
    pub enabled: bool,
    /// Validation frequency
    pub validation_frequency_hours: u64,
    /// Validation strictness level
    pub strictness_level: String,
    /// Automated remediation
    pub auto_remediation: bool,
}

/// Compliance reporting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportingConfiguration {
    /// Enable compliance reporting
    pub enabled: bool,
    /// Report generation frequency
    pub report_frequency_days: u32,
    /// Report formats
    pub report_formats: Vec<String>,
    /// Report recipients
    pub recipients: Vec<String>,
}

/// Data sovereignty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfiguration {
    /// Enable data sovereignty protection
    pub enabled: bool,
    /// Data residency requirements
    pub residency_requirements: Vec<String>,
    /// Cross-border data transfer rules
    pub transfer_rules: Vec<DataTransferRule>,
    /// Sovereignty validation
    pub validation: SovereigntyValidationConfiguration,
}

/// Data transfer rule for sovereignty compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTransferRule {
    /// Rule name
    pub name: String,
    /// Source jurisdictions
    pub source_jurisdictions: Vec<String>,
    /// Target jurisdictions
    pub target_jurisdictions: Vec<String>,
    /// Transfer conditions
    pub conditions: Vec<String>,
    /// Approval required
    pub approval_required: bool,
}

/// Sovereignty validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyValidationConfiguration {
    /// Enable sovereignty validation
    pub enabled: bool,
    /// Validation frequency
    pub validation_frequency_hours: u64,
    /// Validation criteria
    pub criteria: Vec<String>,
    /// Enforcement level
    pub enforcement_level: String,
}
