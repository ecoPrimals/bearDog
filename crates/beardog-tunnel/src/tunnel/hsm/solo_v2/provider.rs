//! Solo V2 HSM Provider Implementation

use super::types::{KeyType, PinConfig, SoloV2Config, SoloV2DeviceInfo, SoloV2KeyHandle};
use crate::tunnel::hsm::types::HsmCapability;
use crate::universal_hsm::traits::{ProviderInfo, ProviderType, UniversalHsmProvider};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

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
        // Real implementation: USB HID device enumeration
        // When hidapi is integrated, this will discover ANY FIDO2 token

        #[cfg(feature = "usb-discovery")]
        {
            // Use hidapi to discover FIDO2-compliant devices
            // This is vendor-agnostic and works with ANY CTAP2/FIDO2 token
            use hidapi::HidApi;

            let api = HidApi::new().map_err(|e| {
                BearDogError::system(format!("Failed to initialize USB HID: {}", e))
            })?;

            let mut devices = Vec::new();

            // FIDO2/CTAP2 standard HID usage page and usage
            const FIDO_USAGE_PAGE: u16 = 0xF1D0;
            const FIDO_USAGE: u16 = 0x01;

            for device in api.device_list() {
                // Check if device implements FIDO2/CTAP2
                if device.usage_page() == FIDO_USAGE_PAGE && device.usage() == FIDO_USAGE {
                    let device_info = SoloV2DeviceInfo {
                        device_id: device
                            .serial_number()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| {
                                format!("{:04x}:{:04x}", device.vendor_id(), device.product_id())
                            }),
                        product_name: device
                            .product_string()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "Unknown FIDO2 Device".to_string()),
                        firmware_version: device
                            .product_string()
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| "unknown".to_string()),
                        is_connected: true,
                        vendor_id: device.vendor_id(),
                        product_id: device.product_id(),
                    };
                    devices.push(device_info);
                }
            }

            info!("Discovered {} FIDO2-compliant device(s)", devices.len());
            Ok(devices)
        }

        #[cfg(not(feature = "usb-discovery"))]
        {
            // Graceful degradation: No USB discovery feature enabled
            // Return empty vector (not an error - allows software fallback)
            debug!("USB discovery feature not enabled, no hardware devices available");
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

        // Real CTAP2 implementation path:
        // 1. Verify PIN if device requires it
        // 2. Send CTAP2 authenticatorMakeCredential command
        // 3. Handle user presence verification (touch/biometric)
        // 4. Store credential ID returned by device

        #[cfg(feature = "ctap2")]
        {
            // CTAP2 key generation implementation
            // Step 1: Verify PIN if needed
            let pin_auth = {
                let pin_config = self.pin_config.read().await;
                pin_config
                    .cached_pin
                    .as_ref()
                    .map(|pin| pin.as_bytes().to_vec())
            };

            // Step 2: Prepare credential parameters
            let rp_id = &self.config.relying_party_id;
            let user_id = _key_id.as_bytes();
            let client_data_hash = [0u8; 32]; // In real use, hash of client data

            // Step 3: Send CTAP2 MakeCredential command
            let result = self
                .ctap2_make_credential(rp_id, user_id, &client_data_hash, pin_auth.as_deref())
                .await?;

            // Step 4: Store credential handle
            let handle = SoloV2KeyHandle {
                key_id: _key_id.clone(),
                credential_id: result.credential_id,
                key_type: _key_type,
                is_resident: true, // FIDO2 credentials are resident by default
                user_id: Some(user_id.to_vec()),
                public_key: result.public_key,
            };

            let mut handles = self.key_handles.write().await;
            handles.insert(_key_id.clone(), handle.clone());

            info!("Generated key on Solo V2 device: {:?}", _key_type);
            Ok(handle)
        }

        #[cfg(not(feature = "ctap2"))]
        {
            // Feature not enabled - return clear error with guidance
            Err(BearDogError::not_implemented(
                "Solo V2 key generation requires CTAP2 feature.\n\
                 Enable with: cargo build --features ctap2\n\
                 Or use software HSM provider for development.",
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
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {}", key_id)))?
            .clone();
        drop(handles);

        // Check device connection
        if !self.device_info.is_connected {
            return Err(BearDogError::system(
                "Solo V2 device not connected".to_string(),
            ));
        }

        // Real CTAP2 signing implementation:
        // 1. Verify PIN if device requires it
        // 2. Send CTAP2 authenticatorGetAssertion with credential ID
        // 3. Handle user presence verification (touch/biometric)
        // 4. Extract signature from assertion response

        #[cfg(feature = "ctap2")]
        {
            // CTAP2 signing implementation
            use sha2::{Digest, Sha256};

            // Step 1: Verify PIN if needed
            let pin_auth = {
                let pin_config = self.pin_config.read().await;
                if let Some(ref pin) = pin_config.cached_pin {
                    Some(pin.as_bytes().to_vec())
                } else {
                    None
                }
            };

            // Step 2: Prepare assertion parameters
            let rp_id = &self.config.relying_party_id;
            let client_data_hash = {
                let mut hasher = Sha256::new();
                hasher.update(_data);
                hasher.finalize().to_vec()
            };

            // Step 3: Send CTAP2 GetAssertion command with credential ID
            let result = self
                .ctap2_get_assertion(
                    rp_id,
                    &client_data_hash,
                    &_handle.credential_id,
                    pin_auth.as_deref(),
                )
                .await?;

            // Step 4: Extract and return signature
            info!("Signed data using Solo V2 device, key: {}", key_id);
            Ok(result.signature)
        }

        #[cfg(not(feature = "ctap2"))]
        {
            // Feature not enabled - return clear error with guidance
            Err(BearDogError::not_implemented(
                "Solo V2 signing requires CTAP2 feature.\n\
                 Enable with: cargo build --features ctap2\n\
                 Or use software HSM provider for development.",
            ))
        }
    }

    /// CTAP2 MakeCredential helper (feature-gated)
    #[cfg(feature = "ctap2")]
    async fn ctap2_make_credential(
        &self,
        rp_id: &str,
        user_id: &[u8],
        client_data_hash: &[u8],
        pin_auth: Option<&[u8]>,
    ) -> Result<Ctap2MakeCredentialResult, BearDogError> {
        // This would interface with actual CTAP2/FIDO2 library
        // Placeholder for when ctap2 feature is fully integrated
        let _ = (rp_id, user_id, client_data_hash, pin_auth);
        Err(BearDogError::not_implemented(
            "CTAP2 MakeCredential integration pending. Use software HSM for development.",
        ))
    }

    /// CTAP2 GetAssertion helper (feature-gated)
    #[cfg(feature = "ctap2")]
    async fn ctap2_get_assertion(
        &self,
        rp_id: &str,
        client_data_hash: &[u8],
        credential_id: &[u8],
        pin_auth: Option<&[u8]>,
    ) -> Result<Ctap2GetAssertionResult, BearDogError> {
        // This would interface with actual CTAP2/FIDO2 library
        // Placeholder for when ctap2 feature is fully integrated
        let _ = (rp_id, client_data_hash, credential_id, pin_auth);
        Err(BearDogError::not_implemented(
            "CTAP2 GetAssertion integration pending. Use software HSM for development.",
        ))
    }
}

/// CTAP2 MakeCredential result
#[cfg(feature = "ctap2")]
struct Ctap2MakeCredentialResult {
    credential_id: Vec<u8>,
    public_key: Vec<u8>,
}

/// CTAP2 GetAssertion result
#[cfg(feature = "ctap2")]
struct Ctap2GetAssertionResult {
    signature: Vec<u8>,
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
