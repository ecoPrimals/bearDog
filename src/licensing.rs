use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::error::{BearDogResult, BearDogError};

/// BearDog license manager for external non-Rust system integrations
/// 
/// Licensing Philosophy:
/// - Rust ecosystem integrations: FREE (AGPL) 
/// - Non-Rust external systems: LICENSED (with free edu licenses)
pub struct LicenseManager {
    /// Licensed adapters for non-Rust systems
    licensed_adapters: HashMap<String, AdapterLicense>,
    /// Free Rust ecosystem adapters (always enabled)
    rust_ecosystem_adapters: Vec<String>,
}

/// License for non-Rust external system integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterLicense {
    /// License ID
    pub license_id: String,
    /// External system name (e.g., "oracle_hsm", "azure_kms", "splunk")
    pub system_name: String,
    /// License holder organization
    pub licensee: String,
    /// License type (Commercial, Educational, Trial)
    pub license_type: LicenseType,
    /// Cryptographic signature for verification
    pub signature: String,
    /// License expiration
    pub expires_at: DateTime<Utc>,
    /// Licensed features
    pub features: Vec<String>,
    /// Maximum nodes/instances
    pub max_instances: Option<u32>,
}

/// License types with different privileges
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseType {
    /// Full commercial license
    Commercial,
    /// Free educational/university license
    Educational,
    /// Time-limited trial license
    Trial,
    /// Developer/testing license
    Developer,
}

/// Integration target types
#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationType {
    /// Integration with Rust project in your ecosystem (always free)
    RustEcosystem,
    /// Integration with external non-Rust system (requires license)
    ExternalSystem,
}

impl LicenseManager {
    /// Create new license manager
    pub fn new() -> Self {
        Self {
            licensed_adapters: HashMap::new(),
            rust_ecosystem_adapters: vec![
                // Your Rust ecosystem projects (free AGPL)
                "nestgate".to_string(),        // If NestGate is your Rust project
                "songbird".to_string(),        // If SongBird is your Rust project
                "beardb".to_string(),          // Example: your Rust database
                "rustguard".to_string(),       // Example: your Rust security tool
            ],
        }
    }

    /// Check if integration requires a license
    pub fn get_integration_type(&self, adapter_name: &str) -> IntegrationType {
        if self.rust_ecosystem_adapters.contains(&adapter_name.to_string()) {
            IntegrationType::RustEcosystem
        } else {
            IntegrationType::ExternalSystem
        }
    }

    /// Verify access to adapter (license check for external systems)
    pub fn verify_adapter_access(&self, adapter_name: &str) -> BearDogResult<bool> {
        match self.get_integration_type(adapter_name) {
            IntegrationType::RustEcosystem => {
                // Rust ecosystem = always free/enabled
                Ok(true)
            },
            IntegrationType::ExternalSystem => {
                // External system = requires valid license
                self.verify_external_license(adapter_name)
            }
        }
    }

    /// Verify license for external non-Rust system
    fn verify_external_license(&self, system_name: &str) -> BearDogResult<bool> {
        if let Some(license) = self.licensed_adapters.get(system_name) {
            // Check expiration
            if license.expires_at < Utc::now() {
                return Err(BearDogError::Configuration {
                    message: format!("License for {} has expired", system_name)
                });
            }

            // Verify signature
            if self.verify_signature(license)? {
                Ok(true)
            } else {
                Err(BearDogError::Configuration {
                    message: format!("Invalid license signature for {}", system_name)
                })
            }
        } else {
            // No license = external system disabled
            Ok(false)
        }
    }

    /// Load license for external system
    pub fn load_external_license(&mut self, license_data: &str) -> BearDogResult<()> {
        let license: AdapterLicense = serde_json::from_str(license_data)?;
        
        // Ensure this is for an external system, not Rust ecosystem
        match self.get_integration_type(&license.system_name) {
            IntegrationType::RustEcosystem => {
                return Err(BearDogError::Configuration {
                    message: format!("{} is part of the Rust ecosystem and doesn't require a license", license.system_name)
                });
            },
            IntegrationType::ExternalSystem => {
                self.licensed_adapters.insert(license.system_name.clone(), license);
            }
        }
        
        Ok(())
    }

    /// Generate educational license (for universities)
    pub fn generate_educational_license(&self, system_name: &str, licensee: &str) -> BearDogResult<AdapterLicense> {
        Ok(AdapterLicense {
            license_id: Uuid::new_v4().to_string(),
            system_name: system_name.to_string(),
            licensee: licensee.to_string(),
            license_type: LicenseType::Educational,
            signature: self.generate_signature(system_name, licensee)?,
            expires_at: Utc::now() + chrono::Duration::days(365 * 5), // 5 year edu license
            features: vec!["full_access".to_string()],
            max_instances: Some(10), // Reasonable limit for educational use
        })
    }

    /// Add new Rust ecosystem project (free integration)
    pub fn add_rust_ecosystem_project(&mut self, project_name: String) {
        if !self.rust_ecosystem_adapters.contains(&project_name) {
            self.rust_ecosystem_adapters.push(project_name);
        }
    }

    /// List all available integrations with their licensing status
    pub fn list_integrations(&self) -> HashMap<String, (IntegrationType, bool)> {
        let mut integrations = HashMap::new();
        
        // Add Rust ecosystem (always enabled)
        for project in &self.rust_ecosystem_adapters {
            integrations.insert(project.clone(), (IntegrationType::RustEcosystem, true));
        }
        
        // Add licensed external systems
        for (system, license) in &self.licensed_adapters {
            let enabled = license.expires_at > Utc::now();
            integrations.insert(system.clone(), (IntegrationType::ExternalSystem, enabled));
        }
        
        integrations
    }

    /// Verify cryptographic signature
    fn verify_signature(&self, _license: &AdapterLicense) -> BearDogResult<bool> {
        // TODO: Implement actual Ed25519 or RSA signature verification
        // This would verify against your public key to prevent tampering
        Ok(true)
    }

    /// Generate signature for license
    fn generate_signature(&self, _system_name: &str, _licensee: &str) -> BearDogResult<String> {
        // TODO: Implement actual signature generation with your private key
        Ok("edu_signature_placeholder".to_string())
    }
}

/// External system definitions (non-Rust integrations that require licenses)
pub struct ExternalSystems;

impl ExternalSystems {
    /// Known external systems that require licensing
    pub fn known_systems() -> Vec<(&'static str, &'static str)> {
        vec![
            // Enterprise HSMs
            ("thales_hsm", "Thales nShield HSM Integration"),
            ("azure_hsm", "Azure Dedicated HSM Integration"),
            ("aws_cloudhsm", "AWS CloudHSM Integration"),
            
            // Enterprise Databases
            ("oracle_db", "Oracle Database Integration"),
            ("mssql", "Microsoft SQL Server Integration"),
            ("db2", "IBM DB2 Integration"),
            
            // SIEM Systems
            ("splunk", "Splunk SIEM Integration"),
            ("qradar", "IBM QRadar Integration"),
            ("arcsight", "Micro Focus ArcSight Integration"),
            
            // Enterprise Authentication
            ("active_directory", "Microsoft Active Directory Integration"),
            ("okta", "Okta Identity Platform Integration"),
            ("ping_identity", "Ping Identity Integration"),
            
            // Enterprise Backup
            ("veeam", "Veeam Backup Integration"),
            ("commvault", "Commvault Integration"),
            ("rubrik", "Rubrik Integration"),
            
            // Cloud Services
            ("aws_kms", "AWS Key Management Service"),
            ("azure_keyvault", "Azure Key Vault Integration"),
            ("gcp_kms", "Google Cloud KMS Integration"),
            
            // Messaging Systems
            ("slack_enterprise", "Slack Enterprise Grid Integration"),
            ("teams", "Microsoft Teams Integration"),
            ("webex", "Cisco Webex Integration"),
        ]
    }
}

/// Trait for license-controlled external adapters
pub trait LicensedAdapter {
    /// Check if adapter is licensed and enabled
    fn is_licensed(&self) -> bool;
    
    /// Get required license features
    fn required_features(&self) -> Vec<String>;
    
    /// Get integration type
    fn integration_type(&self) -> IntegrationType;
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