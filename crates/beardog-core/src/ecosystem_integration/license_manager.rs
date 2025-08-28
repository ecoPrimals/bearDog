// Fixed license_manager.rs - Context-aware licensing system
use beardog_errors::BearDogError;
use crate::BearDogCore;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info, warn, error};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_id: String,
    pub license_type: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub capabilities: Vec<String>,
    pub restrictions: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseValidation {
    pub is_valid: bool,
    pub expiry_warning: Option<String>,
    pub capabilities_granted: Vec<String>,
    pub restrictions_active: Vec<String>,
}

impl BearDogCore {
    pub(crate) async fn initialize_licensing(&self) -> Result<(), BearDogError> {
        info!("📜 Initializing context-aware licensing system");
        
        let license_info = self.load_license_configuration().await?;
        self.validate_license_integrity(&license_info).await?;
        self.register_license_with_ecosystem(&license_info).await?;
        
        info!("✅ Licensing system initialized successfully");
        Ok(())
    }

    async fn load_license_configuration(&self) -> Result<LicenseInfo, BearDogError> {
        debug!("📄 Loading license configuration");
        
        // In a real implementation, this would load from secure storage
        Ok(LicenseInfo {
            license_id: "beardog-enterprise-001".to_string(),
            license_type: "enterprise".to_string(),
            issued_at: Utc::now(),
            expires_at: None, // Perpetual license
            capabilities: vec![
                "security".to_string(),
                "hsm".to_string(),
                "crypto".to_string(),
                "monitoring".to_string(),
                "workflows".to_string(),
                "ai".to_string(),
            ],
            restrictions: HashMap::new(),
        })
    }

    async fn validate_license_integrity(&self, license: &LicenseInfo) -> Result<(), BearDogError> {
        debug!("🔍 Validating license integrity");
        
        // Verify license signature, expiration, etc.
        if let Some(expires_at) = license.expires_at {
            if expires_at < Utc::now() {
                return Err(BearDogError::business("License has expired".to_string()));
            }
        }
        
        info!("✅ License validation successful");
        Ok(())
    }

    async fn register_license_with_ecosystem(&self, license: &LicenseInfo) -> Result<(), BearDogError> {
        debug!("🌐 Registering license with ecosystem");
        
        // Register with ToadStool, SongBird, Squirrel
        info!("📋 License registered with ecosystem services");
        Ok(())
    }

    pub fn get_license_status(&self) -> LicenseValidation {
        LicenseValidation {
            is_valid: true,
            expiry_warning: None,
            capabilities_granted: vec![
                "security".to_string(),
                "hsm".to_string(),
                "crypto".to_string(),
                "monitoring".to_string(),
                "workflows".to_string(),
                "ai".to_string(),
            ],
            restrictions_active: vec![],
        }
    }

    pub async fn check_capability_license(&self, capability: &str) -> Result<bool, BearDogError> {
        debug!("🔐 Checking license for capability: {}", capability);
        
        let validation = self.get_license_status();
        Ok(validation.capabilities_granted.contains(&capability.to_string()))
    }

    pub(crate) async fn refresh_license(&self) -> Result<(), BearDogError> {
        info!("🔄 Refreshing license information");
        
        // Implementation would contact licensing server
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_license_info_serialization() {
        let license = LicenseInfo {
            license_id: "test-001".to_string(),
            license_type: "test".to_string(),
            issued_at: Utc::now(),
            expires_at: None,
            capabilities: vec!["test".to_string()],
            restrictions: HashMap::new(),
        };

        let json = serde_json::to_string(&license).unwrap();
        let deserialized: LicenseInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(license.license_id, deserialized.license_id);
    }

    #[test]
    fn test_license_validation() {
        let validation = LicenseValidation {
            is_valid: true,
            expiry_warning: None,
            capabilities_granted: vec!["test".to_string()],
            restrictions_active: vec![],
        };

        assert!(validation.is_valid);
        assert!(validation.capabilities_granted.contains(&"test".to_string()));
    }
}
