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


/// # Compliance Configuration - Canonical
///
/// **UNIFIED COMPLIANCE CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};

/// **CANONICAL** Unified Compliance Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedComplianceConfig {
    pub audit: AuditConfig,
    pub data_retention: DataRetentionConfig,
    pub privacy: PrivacyConfig,
    pub security_standards: SecurityStandardsConfig,
    pub reporting: ComplianceReportingConfig,
}


/// **CANONICAL** Audit Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub log_level: String,
    pub retention_days: u32,
    pub include_sensitive_data: bool,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: "info".to_string(),
            retention_days: 365,
            include_sensitive_data: false,
        }
    }
}

/// **CANONICAL** Data Retention Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionConfig {
    pub default_retention_days: u32,
    pub personal_data_retention_days: u32,
    pub audit_log_retention_days: u32,
    pub automatic_deletion_enabled: bool,
}

impl Default for DataRetentionConfig {
    fn default() -> Self {
        Self {
            default_retention_days: 2555, // 7 years
            personal_data_retention_days: 1095, // 3 years
            audit_log_retention_days: 365, // 1 year
            automatic_deletion_enabled: true,
        }
    }
}

/// **CANONICAL** Privacy Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub gdpr_compliance: bool,
    pub ccpa_compliance: bool,
    pub data_anonymization: bool,
    pub consent_management: bool,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            gdpr_compliance: true,
            ccpa_compliance: true,
            data_anonymization: true,
            consent_management: true,
        }
    }
}

/// **CANONICAL** Security Standards Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityStandardsConfig {
    pub iso27001_compliance: bool,
    pub soc2_compliance: bool,
    pub pci_dss_compliance: bool,
    pub hipaa_compliance: bool,
}


/// **CANONICAL** Compliance Reporting Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReportingConfig {
    pub enabled: bool,
    pub report_frequency: String,
    pub output_format: String,
    pub recipients: Vec<String>,
}

impl Default for ComplianceReportingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            report_frequency: "monthly".to_string(),
            output_format: "json".to_string(),
            recipients: Vec::new(),
        }
    }
}
