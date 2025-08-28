use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub enabled_standards: Vec<ComplianceStandard>,

    pub monitoring_interval: Duration,

    pub audit_retention: Duration,

    pub dashboard_refresh_interval: Duration,

    pub reporting: ReportingConfig,

    pub privacy_audit: PrivacyAuditConfig,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ComplianceStandard {
    Gdpr,

    Sox,

    PciDss,

    Hipaa,

    Iso27001,

    Soc2,

    Ccpa,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportingConfig {
    pub enabled: bool,

    pub frequency: ReportFrequency,

    pub formats: Vec<ReportFormat>,

    pub email_recipients: Vec<String>,

    pub storage_path: String,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyAuditConfig {
    pub enabled: bool,

    pub track_data_access: bool,

    pub track_data_modification: bool,

    pub track_consent: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSovereigntyConfig {
    pub enforce_residency: bool,

    pub allowed_regions: Vec<String>,

    pub classification_required: bool,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFrequency {
    Hourly,

    Daily,

    Weekly,

    Monthly,

    Quarterly,

    Annually,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReportFormat {
    Json,

    Pdf,

    Csv,

    Html,

    Xml,
}
