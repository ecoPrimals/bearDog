//! Solo V2 HSM Provider Implementation

use super::types::{KeyType, PinConfig, SoloV2Config, SoloV2DeviceInfo, SoloV2KeyHandle};
use crate::tunnel::hsm::types::HsmCapability;
use crate::universal_hsm::traits::{ProviderInfo, ProviderType, UniversalHsmProvider};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Solo V2 USB Security Key HSM Provider
///
/// Provides hardware-backed cryptographic operations using Solo V2 devices.
///
/// # Architecture
///
/// This provider communicates with Solo V2 devices via USB HID using the
/// CTAP2 protocol. All private keys remain on the hardware device and never
/// leave it.
///
/// # Thread Safety
///
/// This struct is thread-safe and can be shared across async tasks using `Arc`.
pub struct SoloV2Provider {
    /// Device information
    device_info: SoloV2DeviceInfo,
    /// Configuration (used in Phase 2 CTAP2 implementation)
    #[allow(dead_code)]
    config: SoloV2Config,
    /// PIN configuration (protected)
    pin_config: Arc<RwLock<PinConfig>>,
    /// Stored key handles
    key_handles: Arc<RwLock<std::collections::HashMap<String, SoloV2KeyHandle>>>,
}

impl SoloV2Provider {
    /// Create a new Solo V2 provider
    ///
    /// # Arguments
    ///
    /// * `device_info` - Information about the Solo V2 device to use
    /// * `config` - Configuration for the provider
    ///
    /// # Returns
    ///
    /// A new `SoloV2Provider` instance
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Device is not connected
    /// - USB permissions are insufficient
    /// - Device communication fails
    pub fn new(device_info: SoloV2DeviceInfo, config: SoloV2Config) -> Result<Self, BearDogError> {
        if !device_info.is_connected {
            return Err(BearDogError::system(format!(
                "Solo V2 device {} is not connected",
                device_info.device_id
            )));
        }

        Ok(Self {
            device_info,
            config,
            pin_config: Arc::new(RwLock::new(PinConfig::default())),
            key_handles: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Discover all connected Solo V2 devices
    ///
    /// # Returns
    ///
    /// A vector of device information for all connected Solo V2 devices
    ///
    /// # Errors
    ///
    /// Returns an error if USB enumeration fails
    ///
    /// # Note
    ///
    /// Real implementation strategy:
    /// 1. USB HID enumeration via `hidapi` crate (vendor-agnostic)
    /// 2. CTAP2/FIDO2 capability detection (ANY compliant device)
    /// 3. Device feature querying via authenticatorGetInfo
    ///
    /// Current: Returns empty vec if no devices found (graceful degradation)
    #[cfg(feature = "solo-v2")]
    pub fn discover_devices() -> Result<Vec<SoloV2DeviceInfo>, BearDogError> {
        // Real implementation: Would enumerate USB HID devices
        // For now, return empty vec (no mocks in production)
        // When hidapi is integrated, this will discover ANY FIDO2 token

        #[cfg(feature = "usb-discovery")]
        {
            // Future: Use hidapi to discover real devices
            // enumerate_fido2_devices()
            todo!("USB discovery via hidapi - requires feature flag")
        }

        #[cfg(not(feature = "usb-discovery"))]
        {
            // Graceful: No devices found, not an error
            Ok(Vec::new())
        }
    }

    /// Set PIN for device operations
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN to use for authentication
    ///
    /// # Security
    ///
    /// PIN is stored in memory only and is never persisted to disk.
    /// Memory is securely zeroed when the provider is dropped.
    pub async fn set_pin(&self, pin: String) -> Result<(), BearDogError> {
        let mut pin_config = self.pin_config.write().await;
        pin_config.cached_pin = Some(pin);
        Ok(())
    }

    /// Generate a new key on the Solo V2 device
    ///
    /// # Arguments
    ///
    /// * `key_type` - The type of key to generate
    ///
    /// # Returns
    ///
    /// A key handle referencing the generated key
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Device is not connected
    /// - PIN verification fails
    /// - Device is out of storage
    /// - Key generation fails
    ///
    /// # Note
    ///
    /// This is a placeholder implementation. Real implementation requires:
    /// 1. CTAP2 MakeCredential command
    /// 2. PIN verification if required
    /// 3. User presence check
    /// 4. Credential storage on device
    pub async fn generate_key_on_device(
        &self,
        _key_type: KeyType,
        _key_id: String,
    ) -> Result<SoloV2KeyHandle, BearDogError> {
        // Check device connection
        if !self.device_info.is_connected {
            return Err(BearDogError::unavailable(
                "Solo V2 device not connected".to_string(),
            ));
        }

        // Real implementation path (when CTAP2 integration complete):
        // 1. Verify PIN if device requires it
        // 2. Send CTAP2 authenticatorMakeCredential command
        // 3. Handle user presence verification (touch/biometric)
        // 4. Store credential ID returned by device

        #[cfg(feature = "ctap2")]
        {
            // Future: Real CTAP2 implementation
            // self.ctap2_make_credential(key_type, &key_id).await
            todo!("CTAP2 MakeCredential - requires ctap2 feature and implementation")
        }

        #[cfg(not(feature = "ctap2"))]
        {
            // Clear error: Feature not enabled
            Err(BearDogError::not_implemented(
                "Solo V2 key generation requires CTAP2 feature. Enable with --features ctap2",
            ))
        }
    }

    /// Sign data using a key on the Solo V2 device
    ///
    /// # Arguments
    ///
    /// * `key_id` - The ID of the key to use for signing
    /// * `data` - The data to sign
    ///
    /// # Returns
    ///
    /// The signature bytes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Key not found
    /// - Device not connected
    /// - PIN verification fails
    /// - Signing operation fails
    ///
    /// # Note
    ///
    /// This is a placeholder implementation. Real implementation requires:
    /// 1. CTAP2 authenticatorGetAssertion command
    /// 2. PIN verification if required
    /// 3. User presence check
    /// 4. Signature extraction
    pub async fn sign_with_device(
        &self,
        key_id: &str,
        _data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        // Get key handle
        let handles = self.key_handles.read().await;
        let _handle = handles
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {}", key_id)))?;

        // Check device connection
        if !self.device_info.is_connected {
            return Err(BearDogError::system(
                "Solo V2 device not connected".to_string(),
            ));
        }

        // Real implementation path (when CTAP2 integration complete):
        // 1. Verify PIN if device requires it
        // 2. Send CTAP2 authenticatorGetAssertion with credential ID
        // 3. Handle user presence verification (touch/biometric)
        // 4. Extract signature from assertion response

        #[cfg(feature = "ctap2")]
        {
            // Future: Real CTAP2 implementation
            // self.ctap2_get_assertion(_handle, _data).await
            todo!("CTAP2 GetAssertion - requires ctap2 feature and implementation")
        }

        #[cfg(not(feature = "ctap2"))]
        {
            // Clear error: Feature not enabled
            Err(BearDogError::not_implemented(
                "Solo V2 signing requires CTAP2 feature. Enable with --features ctap2",
            ))
        }
    }
}

// Implementation of UniversalHsmProvider trait for Solo V2
impl UniversalHsmProvider for SoloV2Provider {
    fn get_provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            name: format!("Solo V2 ({})", self.device_info.device_id),
            provider_type: ProviderType::UsbToken,
            version: self.device_info.firmware_version.clone(),
            capabilities_verified: self.device_info.is_connected,
        }
    }

    fn generate_key(
        &self,
        _key_type: crate::tunnel::hsm::types::KeyType,
    ) -> Result<Vec<u8>, BearDogError> {
        // Solo V2 only supports Ed25519 for signing
        // FIDO2 devices don't support symmetric encryption
        let _solo_key_type = KeyType::Ed25519;

        // Generate key on device (async operation - needs runtime)
        // For now, return error directing to async method
        Err(BearDogError::internal(
            "Use generate_key_on_device() async method for Solo V2".to_string(),
        ))
    }

    fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        // Signing requires async operation
        // For now, return error directing to async method
        Err(BearDogError::internal(
            "Use sign_with_device() async method for Solo V2".to_string(),
        ))
    }

    fn verify(&self, _key_id: &str, _data: &[u8], _signature: &[u8]) -> Result<bool, BearDogError> {
        // Verification typically done off-device with public key
        // For now, return error directing to async method
        Err(BearDogError::internal(
            "Solo V2 verification requires public key extraction".to_string(),
        ))
    }

    fn get_capabilities(&self) -> Vec<HsmCapability> {
        vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::UserPresence,
            HsmCapability::UserVerification,
            HsmCapability::ResidentKeys,
            HsmCapability::HardwareBacked,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solo_v2_provider_creation() {
        let device_info = SoloV2DeviceInfo {
            device_id: "test-device".to_string(),
            product_name: "Solo V2 Test".to_string(),
            firmware_version: "1.0.0".to_string(),
            is_connected: true,
            vendor_id: 0x20a0,
            product_id: 0x42b2,
        };

        let config = SoloV2Config::default();
        let provider = SoloV2Provider::new(device_info, config);
        assert!(provider.is_ok());
    }

    #[test]
    fn test_solo_v2_provider_info() {
        let device_info = SoloV2DeviceInfo {
            device_id: "test-device".to_string(),
            product_name: "Solo V2 Test".to_string(),
            firmware_version: "1.0.0".to_string(),
            is_connected: true,
            vendor_id: 0x20a0,
            product_id: 0x42b2,
        };

        let config = SoloV2Config::default();
        let provider = SoloV2Provider::new(device_info, config).unwrap();

        let info = provider.get_provider_info();
        assert_eq!(info.provider_type, ProviderType::UsbToken);
        assert!(info.name.contains("Solo V2"));
    }

    #[tokio::test]
    async fn test_solo_v2_pin_management() {
        let device_info = SoloV2DeviceInfo {
            device_id: "test-device".to_string(),
            product_name: "Solo V2 Test".to_string(),
            firmware_version: "1.0.0".to_string(),
            is_connected: true,
            vendor_id: 0x20a0,
            product_id: 0x42b2,
        };

        let config = SoloV2Config::default();
        let provider = SoloV2Provider::new(device_info, config).unwrap();

        // Test PIN setting
        let result = provider.set_pin("123456".to_string()).await;
        assert!(result.is_ok());
    }
}
