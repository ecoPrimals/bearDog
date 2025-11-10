//! FIDO2 HSM Provider Implementation

use super::types::{Fido2DeviceInfo, Fido2KeyHandle, Fido2PresenceProof};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

#[cfg(feature = "fido2")]
use hidapi::{HidApi, HidDevice};

/// FIDO2/CTAP2 HSM Provider
/// 
/// Provides hardware security operations using FIDO2-compliant security keys.
pub struct Fido2HsmProvider {
    /// Device information
    device_info: Fido2DeviceInfo,
    
    /// HID API handle (shared)
    #[cfg(feature = "fido2")]
    hid_api: Arc<RwLock<HidApi>>,
    
    /// HID device handle
    #[cfg(feature = "fido2")]
    device: Arc<RwLock<Option<HidDevice>>>,
}

impl Fido2HsmProvider {
    /// Create a new FIDO2 HSM provider for a specific device
    pub async fn new(device_info: Fido2DeviceInfo) -> Result<Self, BearDogError> {
        info!(
            "🔐 Initializing FIDO2 HSM Provider for: {} ({})",
            device_info.product, device_info.manufacturer
        );
        
        #[cfg(not(feature = "fido2"))]
        {
            return Err(BearDogError::system(
                "FIDO2 support not enabled (compile with --features fido2)".to_string()
            ));
        }
        
        #[cfg(feature = "fido2")]
        {
            let hid_api = Arc::new(RwLock::new(HidApi::new().map_err(|e| {
                BearDogError::system(format!("Failed to initialize HID API: {}", e))
            })?));
            
            Ok(Self {
                device_info,
                hid_api,
                device: Arc::new(RwLock::new(None)),
            })
        }
    }
    
    /// Get device information
    pub fn device_info(&self) -> &Fido2DeviceInfo {
        &self.device_info
    }
    
    /// Get device name
    pub fn name(&self) -> &str {
        &self.device_info.product
    }
    
    /// Get device path
    pub fn device_path(&self) -> &std::path::Path {
        &self.device_info.device_path
    }
    
    /// Check if device supports hmac-secret extension
    pub fn supports_hmac_secret(&self) -> bool {
        self.device_info.capabilities.hmac_secret
    }
    
    /// Check if device supports resident keys
    pub fn supports_resident_keys(&self) -> bool {
        self.device_info.capabilities.resident_keys
    }
    
    /// Generate cryptographically secure entropy using hmac-secret extension
    ///
    /// This is the most universal operation that works on almost all FIDO2 devices.
    pub async fn generate_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        if !self.supports_hmac_secret() {
            return Err(BearDogError::system(
                "Device does not support hmac-secret extension".to_string()
            ));
        }
        
        debug!("Generating {} bytes of entropy from FIDO2 device", size);
        
        // TODO: Implement actual CTAP2 hmac-secret entropy generation
        // For now, return a placeholder error
        Err(BearDogError::system(
            "FIDO2 entropy generation not yet implemented (Phase 1 in progress)".to_string()
        ))
    }
    
    /// Create a new resident key on the device
    pub async fn generate_key(
        &self,
        algorithm: KeyAlgorithm,
        params: KeyGenerationParams,
    ) -> Result<Fido2KeyHandle, BearDogError> {
        if !self.supports_resident_keys() {
            return Err(BearDogError::system(
                "Device does not support resident keys".to_string()
            ));
        }
        
        debug!("Generating resident key with algorithm: {:?}", algorithm);
        
        // TODO: Implement actual CTAP2 makeCredential command
        Err(BearDogError::system(
            "FIDO2 key generation not yet implemented (Phase 1 in progress)".to_string()
        ))
    }
    
    /// Sign data with a resident key
    pub async fn sign(
        &self,
        data: &[u8],
        key_handle: &Fido2KeyHandle,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("Signing {} bytes with FIDO2 device", data.len());
        
        // TODO: Implement actual CTAP2 getAssertion command
        Err(BearDogError::system(
            "FIDO2 signing not yet implemented (Phase 1 in progress)".to_string()
        ))
    }
    
    /// Request human presence proof (button press)
    pub async fn require_human_presence(&self) -> Result<Fido2PresenceProof, BearDogError> {
        debug!("Requesting human presence proof from FIDO2 device");
        
        // TODO: Implement actual CTAP2 user presence check
        Err(BearDogError::system(
            "FIDO2 presence verification not yet implemented (Phase 1 in progress)".to_string()
        ))
    }
}

// Placeholder types (will be moved to proper location)
#[derive(Debug, Clone)]
pub enum KeyAlgorithm {
    Ed25519,
    Es256,  // ECDSA P-256
}

#[derive(Debug, Clone, Default)]
pub struct KeyGenerationParams {
    pub require_user_verification: bool,
    pub require_resident_key: bool,
}

impl std::fmt::Debug for Fido2HsmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fido2HsmProvider")
            .field("device", &self.device_info.product)
            .field("manufacturer", &self.device_info.manufacturer)
            .field("path", &self.device_info.device_path)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hsm::fido2::discovery::discover_fido2_devices;
    
    #[tokio::test]
    async fn test_fido2_provider_creation() {
        let devices = discover_fido2_devices().await.unwrap();
        
        if devices.is_empty() {
            println!("⚠️  No FIDO2 devices found for testing");
            return;
        }
        
        for device_info in devices {
            let provider = Fido2HsmProvider::new(device_info).await;
            assert!(provider.is_ok(), "Should create provider successfully");
            
            let provider = provider.unwrap();
            println!("Created provider for: {}", provider.name());
            println!("  Supports hmac-secret: {}", provider.supports_hmac_secret());
            println!("  Supports resident keys: {}", provider.supports_resident_keys());
        }
    }
}

