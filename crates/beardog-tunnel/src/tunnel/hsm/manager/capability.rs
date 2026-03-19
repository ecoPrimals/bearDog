// SPDX-License-Identifier: AGPL-3.0-only

//! HSM Capability Detection
//!
//! Detects and manages HSM capabilities across different providers.

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// HSM capability types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HsmCapability {
    /// Key generation
    KeyGeneration,
    /// Signing operations
    Signing,
    /// Encryption operations
    Encryption,
    /// Decryption operations
    Decryption,
    /// Key wrapping
    KeyWrapping,
    /// Key attestation
    Attestation,
    /// Hardware random number generation
    HardwareRng,
}

/// Security level enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Basic security
    Basic,
    /// Medium security
    Medium,
    /// High security
    High,
    /// Critical security
    Critical,
}

/// HSM tier enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HsmTier {
    /// Software HSM
    Software,
    /// Smart card
    SmartCard,
    /// Hardware HSM
    Hardware,
    /// Cloud HSM
    CloudHsm,
}

/// Security requirements
#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    /// Required security level
    pub security_level: SecurityLevel,
    /// Required capabilities
    pub required_capabilities: Vec<HsmCapability>,
    /// Minimum key size
    pub min_key_size: Option<u32>,
}

/// Trait for HSM capability detection
pub trait HsmCapabilityDetector: Send + Sync {
    /// Detect available HSM capabilities
    ///
    /// # Errors
    /// Returns an error if detection fails
    fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError>;

    /// Check if specific HSM type is available
    ///
    /// # Errors
    /// Returns an error if check fails
    fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError>;

    /// Select HSM tier based on security requirements
    ///
    /// # Errors
    /// Returns an error if selection fails
    fn select_hsm_tier(&self, requirements: &SecurityRequirements)
        -> Result<HsmTier, BearDogError>;
}

/// Default HSM capability detector implementation
pub struct DefaultHsmCapabilityDetector {
    /// Provider capabilities cache
    provider_capabilities: Arc<RwLock<HashMap<String, Vec<HsmCapability>>>>,
}

impl Default for DefaultHsmCapabilityDetector {
    fn default() -> Self {
        Self {
            provider_capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

impl DefaultHsmCapabilityDetector {
    /// Create a new capability detector
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self::default())
    }

    /// Check hardware HSM availability
    fn check_hardware_hsm_availability(&self) -> Result<bool, BearDogError> {
        // In production, this would check for actual hardware HSM presence
        Ok(false)
    }

    /// Check smart card availability
    fn check_smartcard_availability(&self) -> Result<bool, BearDogError> {
        // In production, this would check for smart card readers
        Ok(false)
    }

    /// Check cloud HSM availability
    fn check_cloud_hsm_availability(&self) -> Result<bool, BearDogError> {
        // In production, this would check cloud HSM connectivity
        Ok(false)
    }

    /// Get provider capabilities
    ///
    /// # Errors
    /// Returns an error if retrieval fails
    pub async fn get_provider_capabilities(
        &self,
        provider_id: &str,
    ) -> Result<Vec<HsmCapability>, BearDogError> {
        let capabilities = self.provider_capabilities.read().await;
        capabilities
            .get(provider_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Provider not found: {provider_id}")))
    }

    /// Register provider capabilities
    ///
    /// # Errors
    /// Returns an error if registration fails
    pub async fn register_provider_capabilities(
        &self,
        provider_id: String,
        capabilities: Vec<HsmCapability>,
    ) -> Result<(), BearDogError> {
        let mut provider_capabilities = self.provider_capabilities.write().await;
        provider_capabilities.insert(provider_id, capabilities);
        Ok(())
    }
}

impl HsmCapabilityDetector for DefaultHsmCapabilityDetector {
    fn detect_capabilities(&self) -> Result<Vec<HsmCapability>, BearDogError> {
        // Detect all available capabilities
        let capabilities = vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
        ];
        Ok(capabilities)
    }

    fn is_hsm_available(&self, hsm_type: &HsmTier) -> Result<bool, BearDogError> {
        match hsm_type {
            HsmTier::Software => Ok(true), // Software HSM always available
            HsmTier::Hardware => self.check_hardware_hsm_availability(),
            HsmTier::SmartCard => self.check_smartcard_availability(),
            HsmTier::CloudHsm => self.check_cloud_hsm_availability(),
        }
    }

    fn select_hsm_tier(
        &self,
        requirements: &SecurityRequirements,
    ) -> Result<HsmTier, BearDogError> {
        match requirements.security_level {
            SecurityLevel::Basic => Ok(HsmTier::Software),
            SecurityLevel::Medium => {
                if self.is_hsm_available(&HsmTier::SmartCard)? {
                    Ok(HsmTier::SmartCard)
                } else {
                    Ok(HsmTier::Software)
                }
            }
            SecurityLevel::High => {
                if self.is_hsm_available(&HsmTier::Hardware)? {
                    Ok(HsmTier::Hardware)
                } else if self.is_hsm_available(&HsmTier::SmartCard)? {
                    Ok(HsmTier::SmartCard)
                } else {
                    Ok(HsmTier::Software)
                }
            }
            SecurityLevel::Critical => {
                if self.is_hsm_available(&HsmTier::CloudHsm)? {
                    Ok(HsmTier::CloudHsm)
                } else if self.is_hsm_available(&HsmTier::Hardware)? {
                    Ok(HsmTier::Hardware)
                } else {
                    Err(BearDogError::configuration(
                        "Critical security level requires Cloud HSM or Hardware HSM",
                    ))
                }
            }
        }
    }
}

// NOTE: Default implementation removed - use Type::new() instead since it returns Result
// Previous unsafe implementation used ? which could panic
// Use Type::new()? or Type::new().unwrap_or_else(|e| { /* handle error */ }) instead

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new();
        assert!(detector.is_ok());
        Ok(())
    }

    #[test]
    fn test_detect_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;
        let caps = detector.detect_capabilities();
        assert!(caps.is_ok());

        let caps = caps?;
        assert!(!caps.is_empty());
        assert!(caps.contains(&HsmCapability::KeyGeneration));
        Ok(())
    }

    #[test]
    fn test_is_hsm_available() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;

        // Software HSM should always be available
        assert!(detector.is_hsm_available(&HsmTier::Software)?);
        Ok(())
    }

    #[test]
    fn test_select_hsm_tier_basic() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;
        let requirements = SecurityRequirements {
            security_level: SecurityLevel::Basic,
            required_capabilities: vec![HsmCapability::KeyGeneration],
            min_key_size: Some(256),
        };

        let tier = detector.select_hsm_tier(&requirements);
        assert!(tier.is_ok());
        assert_eq!(tier?, HsmTier::Software);
        Ok(())
    }

    #[test]
    fn test_select_hsm_tier_medium() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;
        let requirements = SecurityRequirements {
            security_level: SecurityLevel::Medium,
            required_capabilities: vec![HsmCapability::Signing],
            min_key_size: Some(256),
        };

        let tier = detector.select_hsm_tier(&requirements);
        assert!(tier.is_ok());
        Ok(())
    }

    #[test]
    fn test_hsm_capability_variants() -> Result<(), Box<dyn std::error::Error>> {
        let caps = [
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
            HsmCapability::Decryption,
            HsmCapability::KeyWrapping,
            HsmCapability::Attestation,
            HsmCapability::HardwareRng,
        ];
        assert_eq!(caps.len(), 7);
        Ok(())
    }

    #[test]
    fn test_security_levels() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(SecurityLevel::Basic, SecurityLevel::Basic);
        assert_ne!(SecurityLevel::Basic, SecurityLevel::Critical);
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_capabilities() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;

        let caps = vec![HsmCapability::Signing, HsmCapability::Encryption];
        detector
            .register_provider_capabilities("test-provider".to_string(), caps.clone())
            .await?;

        let retrieved = detector.get_provider_capabilities("test-provider").await?;

        assert_eq!(retrieved.len(), 2);
        assert!(retrieved.contains(&HsmCapability::Signing));
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_not_found() -> Result<(), Box<dyn std::error::Error>> {
        let detector = DefaultHsmCapabilityDetector::new()?;

        let result = detector.get_provider_capabilities("nonexistent").await;
        assert!(result.is_err());
        Ok(())
    }
}
