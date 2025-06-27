use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::{BearDogResult, BearDogError};

/// BearDog license manager for crypto-locked external functions
/// 
/// **REFINED LICENSING PHILOSOPHY**:
/// - ALL CODE IS OPEN SOURCE (AGPL) - complete transparency
/// - External system functions are crypto-locked with signed licenses
/// - Free signed licenses for: basic users, universities, research that gives back
/// - Paid licenses for: enterprises (they can afford it)
/// - Same codebase for everyone - just license validation gates
pub struct LicenseManager {
    /// Signed licenses for external functions (crypto-locked)
    signed_licenses: HashMap<String, SignedLicense>,
    /// License verification public key
    verification_key: Vec<u8>,
    /// Grace period for license validation (development/testing)
    grace_period_hours: u64,
}

/// Cryptographically signed license for external system functions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedLicense {
    /// License metadata
    pub license: LicenseData,
    /// Ed25519 signature from BearDog (prevents tampering)
    pub signature: String,
    /// License format version
    pub version: u32,
}

/// License data that gets cryptographically signed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseData {
    /// Unique license ID
    pub license_id: String,
    /// External system/function name
    pub function_name: String,
    /// Licensee information
    pub licensee: LicenseeInfo,
    /// License tier and pricing
    pub tier: LicenseTier,
    /// License validity period
    pub valid_from: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    /// Enabled external functions
    pub enabled_functions: Vec<String>,
    /// Usage limits (if any)
    pub limits: Option<UsageLimits>,
    /// License conditions
    pub conditions: Vec<String>,
}

/// Information about the license holder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseeInfo {
    /// Organization name
    pub organization: String,
    /// Contact email
    pub email: String,
    /// License type classification
    pub classification: LicenseeClassification,
    /// Optional: research contribution details
    pub research_contribution: Option<String>,
}

/// Classification determines pricing and terms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseeClassification {
    /// Individual developer or small team
    Individual,
    /// Small business (< 50 employees)
    SmallBusiness,
    /// Educational institution
    Educational,
    /// Research institution that contributes back
    Research,
    /// Non-profit organization
    NonProfit,
    /// Open source project
    OpenSource,
    /// Enterprise (>= 50 employees)
    Enterprise,
}

/// License tiers with different access levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseTier {
    /// Free tier for individuals, education, research
    Community {
        /// Reason for free license
        justification: String,
    },
    /// Paid tier for enterprises
    Enterprise {
        /// Annual license fee (for records)
        annual_fee_usd: u32,
        /// Support level included
        support_level: SupportLevel,
    },
    /// Trial license (30 days)
    Trial {
        /// Trial expiration
        trial_ends: DateTime<Utc>,
    },
}

/// Support levels for different license tiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SupportLevel {
    /// Community support (forums, GitHub issues)
    Community,
    /// Business hours email support
    Business,
    /// 24/7 premium support with SLA
    Premium,
}

/// Usage limits for specific license types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageLimits {
    /// Maximum API calls per month
    pub max_api_calls: Option<u64>,
    /// Maximum data processed per month (GB)
    pub max_data_gb: Option<u64>,
    /// Maximum concurrent instances
    pub max_instances: Option<u32>,
}

impl LicenseManager {
    /// Create new license manager
    pub fn new() -> Self {
        Self {
            signed_licenses: HashMap::new(),
            verification_key: Self::get_verification_key(),
            grace_period_hours: 72, // 3 days grace period for development
        }
    }

    /// Load a BearDog-signed license
    pub fn load_signed_license(&mut self, license_json: &str) -> BearDogResult<()> {
        let signed_license: SignedLicense = serde_json::from_str(license_json)
            .map_err(|e| BearDogError::Configuration {
                message: format!("Invalid license format: {}", e)
            })?;

        // Verify the cryptographic signature
        if !self.verify_license_signature(&signed_license)? {
            return Err(BearDogError::Configuration {
                message: "Invalid license signature - license may be tampered with".to_string()
            });
        }

        // Check license validity period
        let now = Utc::now();
        if now < signed_license.license.valid_from || now > signed_license.license.valid_until {
            return Err(BearDogError::Configuration {
                message: format!("License is not valid at current time. Valid from {} to {}", 
                    signed_license.license.valid_from, signed_license.license.valid_until)
            });
        }

        // Store the valid license
        for function_name in &signed_license.license.enabled_functions {
            self.signed_licenses.insert(function_name.clone(), signed_license.clone());
        }

        tracing::info!("✅ Loaded signed license for {} (tier: {:?})", 
            signed_license.license.licensee.organization,
            signed_license.license.tier
        );

        Ok(())
    }

    /// Check if external function is licensed and available
    pub fn verify_external_function_access(&self, function_name: &str) -> BearDogResult<bool> {
        // Check if we have a valid signed license for this function
        if let Some(license) = self.signed_licenses.get(function_name) {
            // Double-check license is still valid
            let now = Utc::now();
            if now >= license.license.valid_from && now <= license.license.valid_until {
                return Ok(true);
            } else {
                return Err(BearDogError::Configuration {
                    message: format!("License for {} has expired. Please renew your BearDog license.", function_name)
                });
            }
        }

        // No license found - check if we're in grace period (for development)
        if self.is_in_grace_period() {
            tracing::warn!("⚠️  No license found for {}, but grace period is active. External function will work for {} more hours.", 
                function_name, self.grace_period_hours);
            return Ok(true);
        }

        // No license and no grace period
        Err(BearDogError::Configuration {
            message: format!(
                "🔒 External function '{}' requires a BearDog-signed license.\n\n\
                📚 For FREE licenses (individuals, universities, research):\n\
                   Contact: free-licenses@beardog-security.com\n\n\
                🏢 For Enterprise licenses:\n\
                   Contact: enterprise@beardog-security.com\n\n\
                💡 All code is open source - you only pay to unlock external integrations!\n\
                   Basic users, universities, and research get FREE signed licenses.",
                function_name
            )
        })
    }

    /// Generate a free community license (for signing by BearDog)
    pub fn generate_community_license_request(
        &self,
        organization: &str,
        email: &str,
        classification: LicenseeClassification,
        functions: Vec<String>,
        justification: &str,
    ) -> BearDogResult<LicenseData> {
        // Validate that this qualifies for a free license
        match classification {
            LicenseeClassification::Enterprise => {
                return Err(BearDogError::Configuration {
                    message: "Enterprise organizations require paid licenses. Contact enterprise@beardog-security.com".to_string()
                });
            },
            _ => {} // All other classifications can get free licenses
        }

        let license_data = LicenseData {
            license_id: Uuid::new_v4().to_string(),
            function_name: functions.join(","),
            licensee: LicenseeInfo {
                organization: organization.to_string(),
                email: email.to_string(),
                classification,
                research_contribution: if justification.len() > 10 { 
                    Some(justification.to_string()) 
                } else { 
                    None 
                },
            },
            tier: LicenseTier::Community {
                justification: justification.to_string(),
            },
            valid_from: Utc::now(),
            valid_until: Utc::now() + chrono::Duration::days(365 * 5), // 5 year free license
            enabled_functions: functions,
            limits: self.get_community_limits(&classification),
            conditions: vec![
                "License is non-transferable".to_string(),
                "Must comply with AGPL-3.0 for derivative works".to_string(),
                "Commercial use by enterprises requires paid license".to_string(),
            ],
        };

        Ok(license_data)
    }

    /// Get appropriate limits for community licenses
    fn get_community_limits(&self, classification: &LicenseeClassification) -> Option<UsageLimits> {
        match classification {
            LicenseeClassification::Individual => Some(UsageLimits {
                max_api_calls: Some(10_000), // 10K calls/month
                max_data_gb: Some(10),       // 10GB/month
                max_instances: Some(3),      // 3 instances
            }),
            LicenseeClassification::SmallBusiness => Some(UsageLimits {
                max_api_calls: Some(100_000), // 100K calls/month
                max_data_gb: Some(100),       // 100GB/month
                max_instances: Some(10),      // 10 instances
            }),
            LicenseeClassification::Educational | 
            LicenseeClassification::Research |
            LicenseeClassification::NonProfit |
            LicenseeClassification::OpenSource => None, // No limits for education/research
            LicenseeClassification::Enterprise => unreachable!(), // Should not reach here
        }
    }

    /// Verify cryptographic signature of license
    fn verify_license_signature(&self, signed_license: &SignedLicense) -> BearDogResult<bool> {
        // TODO: Implement Ed25519 signature verification
        // This would verify the signature against BearDog's public key
        // For now, return true for development
        
        // In production, this would:
        // 1. Serialize the license data to canonical JSON
        // 2. Verify Ed25519 signature using BearDog's public key
        // 3. Return true only if signature is valid
        
        tracing::debug!("Verifying license signature for {}", signed_license.license.licensee.organization);
        Ok(true) // TODO: Replace with actual signature verification
    }

    /// Get BearDog's public key for license verification
    fn get_verification_key() -> Vec<u8> {
        // TODO: Embed BearDog's actual public key
        // This would be the Ed25519 public key used to verify licenses
        vec![0u8; 32] // Placeholder
    }

    /// Check if we're in grace period (for development/testing)
    fn is_in_grace_period(&self) -> bool {
        // TODO: Check if this is a development build or if grace period is active
        // For now, return false to enforce licensing
        false
    }

    /// List all loaded licenses and their status
    pub fn list_licenses(&self) -> Vec<LicenseStatus> {
        let mut statuses = Vec::new();
        let now = Utc::now();

        for (function_name, signed_license) in &self.signed_licenses {
            let is_valid = now >= signed_license.license.valid_from && 
                          now <= signed_license.license.valid_until;
            
            statuses.push(LicenseStatus {
                function_name: function_name.clone(),
                organization: signed_license.license.licensee.organization.clone(),
                tier: signed_license.license.tier.clone(),
                valid_until: signed_license.license.valid_until,
                is_valid,
                days_remaining: if is_valid {
                    (signed_license.license.valid_until - now).num_days()
                } else {
                    0
                },
            });
        }

        statuses
    }
}

/// Status information for a loaded license
#[derive(Debug, Clone)]
pub struct LicenseStatus {
    pub function_name: String,
    pub organization: String,
    pub tier: LicenseTier,
    pub valid_until: DateTime<Utc>,
    pub is_valid: bool,
    pub days_remaining: i64,
}

/// Known external functions that require signed licenses
pub struct ExternalFunctions;

impl ExternalFunctions {
    /// List of all external functions that require BearDog-signed licenses
    pub fn all_functions() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            // Format: (function_name, description, category)
            
            // Monitoring & Observability
            ("prometheus_export", "Export metrics to Prometheus", "monitoring"),
            ("grafana_dashboards", "Create Grafana dashboards", "monitoring"),
            ("splunk_integration", "Send data to Splunk SIEM", "monitoring"),
            ("datadog_metrics", "Send metrics to DataDog", "monitoring"),
            ("newrelic_apm", "New Relic APM integration", "monitoring"),
            ("elasticsearch_logs", "Send logs to Elasticsearch", "monitoring"),
            
            // Cloud Provider Services
            ("aws_kms_integration", "AWS Key Management Service", "cloud"),
            ("azure_keyvault", "Azure Key Vault integration", "cloud"),
            ("gcp_kms", "Google Cloud KMS", "cloud"),
            ("aws_secrets_manager", "AWS Secrets Manager", "cloud"),
            ("azure_secrets", "Azure Key Vault Secrets", "cloud"),
            
            // Enterprise Identity
            ("active_directory", "Microsoft Active Directory", "identity"),
            ("ldap_integration", "LDAP directory services", "identity"),
            ("okta_sso", "Okta single sign-on", "identity"),
            ("auth0_integration", "Auth0 identity platform", "identity"),
            ("ping_identity", "PingIdentity services", "identity"),
            
            // Hardware Security Modules
            ("thales_hsm", "Thales nShield HSM", "hsm"),
            ("safenet_hsm", "SafeNet HSM integration", "hsm"),
            ("aws_cloudhsm", "AWS CloudHSM", "hsm"),
            ("azure_dedicated_hsm", "Azure Dedicated HSM", "hsm"),
            
            // Enterprise Databases
            ("oracle_database", "Oracle Database integration", "database"),
            ("mssql_integration", "Microsoft SQL Server", "database"),
            ("db2_integration", "IBM Db2 integration", "database"),
            ("mongodb_atlas", "MongoDB Atlas cloud", "database"),
            
            // Compliance & Audit
            ("archer_grc", "RSA Archer GRC platform", "compliance"),
            ("servicenow_itsm", "ServiceNow ITSM", "compliance"),
            ("jira_integration", "Atlassian Jira", "compliance"),
            ("sharepoint_docs", "Microsoft SharePoint", "compliance"),
        ]
    }

    /// Check if a function name is a known external function
    pub fn is_external_function(name: &str) -> bool {
        Self::all_functions().iter().any(|(func_name, _, _)| *func_name == name)
    }

    /// Get category for external function
    pub fn get_category(name: &str) -> Option<&'static str> {
        Self::all_functions().iter()
            .find(|(func_name, _, _)| *func_name == name)
            .map(|(_, _, category)| *category)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_ecosystem_always_free() {
        let manager = LicenseManager::new();
        
        // Rust ecosystem projects should always be enabled
        assert_eq!(manager.verify_adapter_access("nestgate").unwrap(), true);
        assert_eq!(manager.verify_adapter_access("songbird").unwrap(), true);
        
        // External systems without license should be disabled
        assert_eq!(manager.verify_adapter_access("oracle_hsm").unwrap(), false);
    }

    #[test]
    fn test_educational_license_generation() {
        let manager = LicenseManager::new();
        
        let edu_license = manager.generate_educational_license(
            "aws_kms", 
            "Stanford University"
        ).unwrap();
        
        assert_eq!(edu_license.license_type, LicenseType::Educational);
        assert_eq!(edu_license.system_name, "aws_kms");
        assert_eq!(edu_license.licensee, "Stanford University");
        assert!(edu_license.expires_at > Utc::now() + chrono::Duration::days(365 * 4));
    }

    #[test]
    fn test_integration_type_detection() {
        let manager = LicenseManager::new();
        
        // Rust ecosystem
        assert_eq!(
            manager.get_integration_type("nestgate"), 
            IntegrationType::RustEcosystem
        );
        
        // External system
        assert_eq!(
            manager.get_integration_type("oracle_hsm"), 
            IntegrationType::ExternalSystem
        );
    }
} 