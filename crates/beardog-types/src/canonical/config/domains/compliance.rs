// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Consolidated Compliance Configuration Domain
//!
//! This module consolidates ALL compliance-related configuration structs across the `BearDog`
//! ecosystem into a single, unified compliance configuration system.
//!
//! **CONSOLIDATED FROM:**
//! - `beardog-compliance/src/compliance/types.rs::ComplianceConfig`
//! - `beardog-security/src/orchestration/compliance_orchestration.rs::ComplianceOrchestrationConfig`
//! - `beardog-security/src/sovereignty/compliance_sovereignty.rs::ComplianceSovereigntyConfig`

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **CONSOLIDATED COMPLIANCE CONFIGURATION** - Single source of truth for all compliance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedComplianceConfiguration {
    /// Enable compliance monitoring system
    pub enabled: bool,

    /// **COMPLIANCE STANDARDS**
    /// Active compliance standards (GDPR, HIPAA, SOX, PCI-DSS, ISO-27001, SOC2, CCPA)
    pub standards: Vec<ComplianceStandard>,
    /// Enabled standards for active monitoring
    pub enabled_standards: Vec<ComplianceStandard>,
    /// Compliance frameworks for orchestration
    pub compliance_frameworks: Vec<String>,

    /// **AUDIT & RETENTION**
    /// Audit retention period in days (default: 2555 days = 7 years)
    pub audit_retention_days: u32,
    /// Audit frequency in hours (for automated audits)
    pub audit_frequency_hours: u64,

    /// **REPORTING CONFIGURATION**
    pub reporting: ReportingConfiguration,

    /// **DATA SOVEREIGNTY**
    pub data_sovereignty: DataSovereigntyConfiguration,

    /// **PRIVACY AUDIT**
    pub privacy_audit: PrivacyAuditConfiguration,
}

/// Compliance standards supported by `BearDog`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ComplianceStandard {
    /// General Data Protection Regulation (EU)
    Gdpr,
    /// Health Insurance Portability and Accountability Act (US)
    Hipaa,
    /// Sarbanes-Oxley Act (US)
    Sox,
    /// Payment Card Industry (generic)
    Pci,
    /// Payment Card Industry Data Security Standard
    PciDss,
    /// ISO/IEC 27001 Information Security Management
    Iso27001,
    /// Service Organization Control 2
    Soc2,
    /// California Consumer Privacy Act (US)
    Ccpa,
    /// Custom compliance framework
    Custom(String),
}

impl std::fmt::Display for ComplianceStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gdpr => write!(f, "GDPR"),
            Self::Hipaa => write!(f, "HIPAA"),
            Self::Sox => write!(f, "SOX"),
            Self::Pci => write!(f, "PCI"),
            Self::PciDss => write!(f, "PCI-DSS"),
            Self::Iso27001 => write!(f, "ISO-27001"),
            Self::Soc2 => write!(f, "SOC-2"),
            Self::Ccpa => write!(f, "CCPA"),
            Self::Custom(name) => write!(f, "{name}"),
        }
    }
}

/// Reporting configuration for compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfiguration {
    /// Report output format
    pub format: ReportFormat,
    /// Report generation frequency
    pub frequency: ReportFrequency,
    /// Report recipients (email addresses or endpoints)
    pub recipients: Vec<String>,
    /// Enable automated report generation
    pub automated: bool,
}

/// Report output formats
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFormat {
    /// JSON format
    Json,
    /// XML format
    Xml,
    /// PDF format
    Pdf,
    /// HTML format
    Html,
    /// CSV format
    Csv,
}

/// Report generation frequency
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReportFrequency {
    /// Daily reports
    Daily,
    /// Weekly reports
    Weekly,
    /// Monthly reports
    Monthly,
    /// Quarterly reports
    Quarterly,
    /// Annual reports
    Annually,
}

/// Data sovereignty configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfiguration {
    /// Legal jurisdiction (e.g., "US", "EU", "UK")
    pub jurisdiction: String,
    /// Require data to remain in specified jurisdiction
    pub data_residency_required: bool,
    /// Cross-border data transfer restrictions
    pub cross_border_restrictions: Vec<String>,
}

/// Privacy audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditConfiguration {
    /// Audit frequency
    pub frequency: ReportFrequency,
    /// Enable automated privacy audits
    pub automated: bool,
    /// External auditor contact/identifier
    pub external_auditor: Option<String>,
}

impl Default for ReportingConfiguration {
    fn default() -> Self {
        Self {
            format: ReportFormat::Json,
            frequency: ReportFrequency::Monthly,
            recipients: vec![],
            automated: true,
        }
    }
}

impl Default for DataSovereigntyConfiguration {
    fn default() -> Self {
        Self {
            jurisdiction: "US".to_string(),
            data_residency_required: true,
            cross_border_restrictions: vec![],
        }
    }
}

impl Default for PrivacyAuditConfiguration {
    fn default() -> Self {
        Self {
            frequency: ReportFrequency::Quarterly,
            automated: true,
            external_auditor: None,
        }
    }
}

impl ConsolidatedComplianceConfiguration {
    /// Create configuration from a config source (modern pattern)
    pub fn from_source(source: &dyn crate::canonical::config::source::ConfigSource) -> Self {
        use crate::canonical::config::source::get_parsed;

        Self {
            enabled: true,
            standards: vec![ComplianceStandard::Gdpr, ComplianceStandard::Sox],
            enabled_standards: vec![ComplianceStandard::Gdpr, ComplianceStandard::Sox],
            compliance_frameworks: vec![
                "GDPR".to_string(),
                "SOX".to_string(),
                "CCPA".to_string(),
                "HIPAA".to_string(),
            ],
            audit_retention_days: get_parsed(
                source,
                env_keys::ENV_COMPLIANCE_AUDIT_RETENTION_DAYS,
                2555,
            ), // 7 years
            audit_frequency_hours: get_parsed(
                source,
                env_keys::ENV_COMPLIANCE_AUDIT_FREQUENCY_HOURS,
                168,
            ), // Weekly
            reporting: ReportingConfiguration::default(),
            data_sovereignty: DataSovereigntyConfiguration::default(),
            privacy_audit: PrivacyAuditConfiguration::default(),
        }
    }
}

impl Default for ConsolidatedComplianceConfiguration {
    fn default() -> Self {
        use crate::canonical::config::source::EnvConfigSource;
        Self::from_source(&EnvConfigSource::new())
    }
}

impl ConsolidatedComplianceConfiguration {
    /// Validate compliance configuration
    ///
    /// # Errors
    ///
    /// Returns an error if retention, audit frequency, or enabled standards are inconsistent.
    pub fn validate(&self) -> Result<(), BearDogError> {
        if self.audit_retention_days == 0 {
            return Err(BearDogError::configuration(
                "Audit retention cannot be zero",
            ));
        }

        if self.audit_frequency_hours == 0 {
            return Err(BearDogError::configuration(
                "Audit frequency cannot be zero",
            ));
        }

        if self.enabled && self.enabled_standards.is_empty() {
            return Err(BearDogError::configuration(
                "At least one compliance standard must be enabled when compliance is enabled",
            ));
        }

        Ok(())
    }

    /// Create development configuration
    #[must_use]
    pub fn development() -> Self {
        let mut config = Self::default();
        config.audit_retention_days =
            std::env::var(env_keys::ENV_COMPLIANCE_DEV_AUDIT_RETENTION_DAYS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30); // Shorter for development
        config.audit_frequency_hours =
            std::env::var(env_keys::ENV_COMPLIANCE_DEV_AUDIT_FREQUENCY_HOURS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24); // Daily audits in dev
        config.reporting.frequency = ReportFrequency::Daily;
        config
    }

    /// Create production configuration
    #[must_use]
    pub fn production() -> Self {
        Self::default()
    }

    /// Check if a specific standard is enabled
    #[must_use]
    pub fn is_standard_enabled(&self, standard: &ComplianceStandard) -> bool {
        self.enabled && self.enabled_standards.contains(standard)
    }

    /// Get all enabled standards as strings
    pub fn enabled_standards_as_strings(&self) -> Vec<String> {
        self.enabled_standards
            .iter()
            .map(std::string::ToString::to_string)
            .collect()
    }
}
