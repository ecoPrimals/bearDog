// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Compliance Configuration
///
/// **UNIFIED COMPLIANCE CONFIGURATION** - Single source of truth for all compliance settings
/// This module consolidates compliance configuration from:
/// - beardog-compliance/src/compliance/types.rs::ComplianceConfig
/// - beardog-api/src/api/sovereignty/privacy/audit.rs::ComplianceConfig
/// - Various scattered compliance settings across the ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL COMPLIANCE CONFIGURATION** - Main compliance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    /// List of enabled compliance standards
    pub enabled_standards: Vec<ComplianceStandard>,
    /// Monitoring interval for compliance checks
    pub monitoring_interval: Duration,
    /// Audit log retention period
    pub audit_retention: Duration,
    /// Dashboard refresh interval
    pub dashboard_refresh_interval: Duration,
    /// Reporting configuration
    pub reporting: ReportingConfig,
    /// Privacy audit settings
    pub privacy_audit: PrivacyAuditConfig,
    /// Data sovereignty settings
    pub data_sovereignty: DataSovereigntyConfig,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enabled_standards: vec![
                ComplianceStandard::Gdpr,
                ComplianceStandard::Sox,
                ComplianceStandard::PciDss,
            ],
            monitoring_interval: Duration::from_secs(300), // 5 minutes
            audit_retention: Duration::from_secs(31_536_000), // 365 days
            dashboard_refresh_interval: Duration::from_secs(60), // 1 minute
            reporting: ReportingConfig::default(),
            privacy_audit: PrivacyAuditConfig::default(),
            data_sovereignty: DataSovereigntyConfig::default(),
        }
    }
}

/// **CANONICAL COMPLIANCE STANDARDS** - Supported compliance frameworks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    /// General Data Protection Regulation (EU)
    Gdpr,
    /// Sarbanes-Oxley Act (US)
    Sox,
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// Health Insurance Portability and Accountability Act (US)
    Hipaa,
    /// ISO 27001 Information Security Management
    Iso27001,
    /// SOC 2 Type II
    Soc2,
    /// California Consumer Privacy Act
    Ccpa,
}

/// **CANONICAL REPORTING CONFIGURATION** - Compliance reporting settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    /// Enable automated reporting
    pub enabled: bool,
    /// Report generation frequency
    pub frequency: ReportFrequency,
    /// Report formats to generate
    pub formats: Vec<ReportFormat>,
    /// Email recipients for reports
    pub email_recipients: Vec<String>,
    /// Report storage location
    pub storage_path: String,
    /// Report retention period
    pub retention_period: Duration,
}

impl Default for ReportingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency: ReportFrequency::Daily,
            formats: vec![ReportFormat::Json, ReportFormat::Pdf],
            email_recipients: Vec::new(),
            storage_path: "/var/log/beardog/compliance".to_string(),
            retention_period: Duration::from_secs(7_776_000), // 90 days
        }
    }
}

/// **CANONICAL PRIVACY AUDIT CONFIGURATION** - Privacy-specific audit settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditConfig {
    /// Enable privacy audit logging
    pub enabled: bool,
    /// Data access tracking
    pub track_data_access: bool,
    /// Data modification tracking
    pub track_data_modification: bool,
    /// User consent tracking
    pub track_consent: bool,
    /// Audit log encryption
    pub encrypt_logs: bool,
}

impl Default for PrivacyAuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            track_data_access: true,
            track_data_modification: true,
            track_consent: true,
            encrypt_logs: true,
        }
    }
}

/// **CANONICAL DATA SOVEREIGNTY CONFIGURATION** - Data location and control settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfig {
    /// Enforce data residency requirements
    pub enforce_residency: bool,
    /// Allowed data regions
    pub allowed_regions: Vec<String>,
    /// Data classification requirements
    pub classification_required: bool,
    /// Cross-border data transfer restrictions
    pub restrict_cross_border: bool,
}

impl Default for DataSovereigntyConfig {
    fn default() -> Self {
        Self {
            enforce_residency: false,
            allowed_regions: vec!["US".to_string(), "EU".to_string()],
            classification_required: true,
            restrict_cross_border: false,
        }
    }
}

/// **CANONICAL REPORT FREQUENCY** - How often reports are generated
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFrequency {
    /// Generate reports hourly
    Hourly,
    /// Generate reports daily
    Daily,
    /// Generate reports weekly
    Weekly,
    /// Generate reports monthly
    Monthly,
    /// Generate reports quarterly
    Quarterly,
    /// Generate reports annually
    Annually,
}

/// **CANONICAL REPORT FORMAT** - Available report output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    /// JSON format for programmatic access
    Json,
    /// PDF format for human reading
    Pdf,
    /// CSV format for data analysis
    Csv,
    /// HTML format for web viewing
    Html,
    /// XML format for system integration
    Xml,
} 