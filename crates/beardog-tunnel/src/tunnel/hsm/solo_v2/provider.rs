// SPDX-License-Identifier: AGPL-3.0-or-later

//! Solo V2 HSM Provider Implementation

use super::types::{KeyType, PinConfig, SoloV2Config, SoloV2DeviceInfo, SoloV2KeyHandle};
use crate::tunnel::hsm::types::HsmCapability;
use crate::universal_hsm::traits::{ProviderInfo, ProviderType, UniversalHsmProvider};
use beardog_errors::BearDogError;
use std::sync::Arc;
#[cfg(feature = "ctap2")]
use tokio::sync::Mutex;
use tokio::sync::RwLock;
#[cfg(any(feature = "ctap2", feature = "usb-discovery"))]
use tracing::info;

#[cfg(feature = "ctap2")]
use super::ctap2_protocol::{
    build_get_assertion, build_make_credential, parse_get_assertion_response,
    parse_make_credential_response,
};
#[cfg(feature = "ctap2")]
use super::hid_transport::Ctap2TransportBackend;
#[cfg(feature = "ctap2")]
use super::transport::Ctap2Transport;

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
    /// Configuration (`relying_party_id` and options for CTAP2 when feature `ctap2` is enabled).
    #[cfg_attr(not(feature = "ctap2"), allow(dead_code))]
    config: SoloV2Config,
    /// PIN configuration (protected)
    pin_config: Arc<RwLock<PinConfig>>,
    /// Stored key handles
    key_handles: Arc<RwLock<std::collections::HashMap<String, SoloV2KeyHandle>>>,
    /// CTAP2 HID transport (set via [`Self::with_ctap2_transport`] or [`Self::with_hid_device_path`])
    #[cfg(feature = "ctap2")]
    ctap_transport: Option<Arc<Mutex<Ctap2TransportBackend>>>,
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
            #[cfg(feature = "ctap2")]
            ctap_transport: None,
        })
    }

    /// Use a [`Ctap2Transport`] implementation (e.g. [`super::HidCtap2Transport`] or a mock for tests).
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`] if the device is not connected.
    #[cfg(feature = "ctap2")]
    pub fn with_ctap2_transport(
        device_info: SoloV2DeviceInfo,
        config: SoloV2Config,
        transport: Arc<Mutex<Ctap2TransportBackend>>,
    ) -> Result<Self, BearDogError> {
        let mut s = Self::new(device_info, config)?;
        s.ctap_transport = Some(transport);
        Ok(s)
    }

    /// Open the HID device at `hid_path` and use it as the CTAP2 transport.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`], or if opening the HID device fails.
    #[cfg(feature = "ctap2")]
    pub async fn with_hid_device_path(
        device_info: SoloV2DeviceInfo,
        config: SoloV2Config,
        hid_path: &str,
    ) -> Result<Self, BearDogError> {
        let hid = super::hid_transport::HidCtap2Transport::open(hid_path).await?;
        Self::with_ctap2_transport(
            device_info,
            config,
            Arc::new(Mutex::new(Ctap2TransportBackend::Hid(hid))),
        )
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
    /// Real implementation strategy (Pure Rust - ecoBin compliant):
    /// 1. USB HID enumeration via `beardog-hid` crate (Pure Rust, vendor-agnostic)
    /// 2. CTAP2/FIDO2 capability detection (ANY compliant device)
    /// 3. Device feature querying via authenticatorGetInfo
    ///
    /// Current: Returns empty vec if no devices found (graceful degradation)
    #[cfg(feature = "solo-v2")]
    /// # Errors
    ///
    /// Returns an error if the Tor-related operation fails.
    pub fn discover_devices() -> Result<Vec<SoloV2DeviceInfo>, BearDogError> {
        // Real implementation: USB HID device enumeration using Pure Rust beardog-hid

        #[cfg(feature = "usb-discovery")]
        {
            // Use beardog-hid to discover FIDO2-compliant devices (Pure Rust - ecoBin compliant)
            // This is vendor-agnostic and works with ANY CTAP2/FIDO2 token

            // Use tokio runtime handle to call async function from sync context
            let hid_devices = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(beardog_hid::discover())
            })
            .map_err(|e| BearDogError::system(format!("Failed to discover HID devices: {e}")))?;

            let mut devices = Vec::new();

            for device in &hid_devices {
                // Check if device is a known FIDO2-compliant device by VID/PID
                if beardog_hid::types::is_fido2_device(device.vendor_id, device.product_id) {
                    let device_info = SoloV2DeviceInfo {
                        device_id: if device.serial.is_empty() {
                            format!("{}:{}", device.vendor_id, device.product_id)
                        } else {
                            device.serial.clone()
                        },
                        product_name: if device.product.is_empty() {
                            "Unknown FIDO2 Device".to_string()
                        } else {
                            device.product.clone()
                        },
                        firmware_version: if device.product.is_empty() {
                            "unknown".to_string()
                        } else {
                            device.product.clone()
                        },
                        is_connected: true,
                        vendor_id: device.vendor_id.0,
                        product_id: device.product_id.0,
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
            tracing::debug!("USB discovery feature not enabled, no hardware devices available");
            Ok(Vec::new())
        }
    }

    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
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
    /// With the `ctap2` feature enabled, this sends a real CTAP2 `MakeCredential`
    /// command via `HidCtap2Transport`. Without the feature, returns a capability error.
    /// Full PIN/UV verification requires Phase 2 `ClientPIN` protocol.
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
                .ctap2_make_credential(
                    rp_id,
                    user_id,
                    _key_id.as_str(),
                    &client_data_hash,
                    _key_type,
                    pin_auth.as_deref(),
                )
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
            Err(BearDogError::requires_capability(
                "ctap2",
                "Solo V2 key generation requires the ctap2 Cargo feature, a CTAP2 transport, and a connected device; enable with --features ctap2 or use a software HSM for development",
            ))
        }
    }

    /// Sign data using a key on the Solo V2 device (requires prior registration
    /// in the same provider session — use [`Self::authenticate_with_credential`]
    /// for stateless assertion with a known `credential_id`).
    pub async fn sign_with_device(
        &self,
        key_id: &str,
        _data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let handles = self.key_handles.read().await;
        let handle = handles
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key not found: {key_id}")))?
            .clone();
        drop(handles);

        self.authenticate_with_credential(&handle.credential_id, _data)
            .await
    }

    /// Stateless CTAP2 `GetAssertion` — takes raw `credential_id` bytes directly.
    ///
    /// Unlike [`Self::sign_with_device`], this does not require the credential
    /// to have been registered in the same provider session. The CTAP2 protocol
    /// uses the `credential_id` as an allow-list entry, so the authenticator
    /// looks up the credential internally.
    pub async fn authenticate_with_credential(
        &self,
        credential_id: &[u8],
        data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.device_info.is_connected {
            return Err(BearDogError::system(
                "Solo V2 device not connected".to_string(),
            ));
        }

        #[cfg(feature = "ctap2")]
        {
            use sha2::{Digest, Sha256};

            let pin_auth = {
                let pin_config = self.pin_config.read().await;
                pin_config
                    .cached_pin
                    .as_ref()
                    .map(|pin| pin.as_bytes().to_vec())
            };

            let rp_id = &self.config.relying_party_id;
            let client_data_hash = {
                let mut hasher = Sha256::new();
                hasher.update(data);
                hasher.finalize().to_vec()
            };

            let result = self
                .ctap2_get_assertion(
                    rp_id,
                    &client_data_hash,
                    credential_id,
                    pin_auth.as_deref(),
                )
                .await?;

            info!(
                cred_len = credential_id.len(),
                sig_len = result.signature.len(),
                "CTAP2 GetAssertion complete (hardware-attested)"
            );
            Ok(result.signature)
        }

        #[cfg(not(feature = "ctap2"))]
        {
            let _ = (credential_id, data);
            Err(BearDogError::requires_capability(
                "ctap2",
                "Solo V2 signing requires the ctap2 Cargo feature, a CTAP2 transport, and a connected device; enable with --features ctap2 or use a software HSM for development",
            ))
        }
    }

    /// CTAP2 `MakeCredential` helper (feature-gated)
    #[cfg(feature = "ctap2")]
    async fn ctap2_make_credential(
        &self,
        rp_id: &str,
        user_id: &[u8],
        user_name: &str,
        client_data_hash: &[u8],
        key_type: KeyType,
        pin_auth: Option<&[u8]>,
    ) -> Result<Ctap2MakeCredentialResult, BearDogError> {
        let transport = self.ctap_transport.as_ref().ok_or_else(|| {
            BearDogError::system(
                "CTAP2 transport not configured; use SoloV2Provider::with_ctap2_transport or with_hid_device_path"
                    .to_string(),
            )
        })?;

        let alg = match key_type {
            KeyType::Ed25519 => -8_i64,
            KeyType::EcdsaP256 => -7_i64,
        };
        let pin_uv = pin_auth.map(|p| (p, 1_u64));
        let cmd = build_make_credential(rp_id, user_id, user_name, client_data_hash, alg, pin_uv)?;

        let mut guard = transport.lock().await;
        let resp = guard.send_receive(&cmd).await?;
        drop(guard);

        let parsed = parse_make_credential_response(&resp)?;
        Ok(Ctap2MakeCredentialResult {
            credential_id: parsed.credential_id,
            public_key: parsed.raw_cose_public_key,
            attestation_statement: parsed.attestation_statement,
        })
    }

    /// CTAP2 `GetAssertion` helper (feature-gated)
    #[cfg(feature = "ctap2")]
    async fn ctap2_get_assertion(
        &self,
        rp_id: &str,
        client_data_hash: &[u8],
        credential_id: &[u8],
        pin_auth: Option<&[u8]>,
    ) -> Result<Ctap2GetAssertionResult, BearDogError> {
        let transport = self.ctap_transport.as_ref().ok_or_else(|| {
            BearDogError::system(
                "CTAP2 transport not configured; use SoloV2Provider::with_ctap2_transport or with_hid_device_path"
                    .to_string(),
            )
        })?;

        let pin_uv = pin_auth.map(|p| (p, 1_u64));
        let cmd = build_get_assertion(rp_id, client_data_hash, &[credential_id], pin_uv)?;

        let mut guard = transport.lock().await;
        let resp = guard.send_receive(&cmd).await?;
        drop(guard);

        let parsed = parse_get_assertion_response(&resp)?;
        Ok(Ctap2GetAssertionResult {
            signature: parsed.signature,
        })
    }
}

/// CTAP2 `MakeCredential` result
#[cfg(feature = "ctap2")]
struct Ctap2MakeCredentialResult {
    credential_id: Vec<u8>,
    public_key: Vec<u8>,
    /// CBOR-encoded `attStmt`; reserved for attestation chain verification.
    #[expect(
        dead_code,
        reason = "reserved for Phase 2 attestation chain verification"
    )]
    attestation_statement: Vec<u8>,
}

/// CTAP2 `GetAssertion` result
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

        Err(BearDogError::unsupported_operation(
            "Solo V2 requires async API: use generate_key_on_device() instead",
        ))
    }

    fn sign(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "Solo V2 requires async API: use sign_with_device() instead",
        ))
    }

    fn verify(&self, _key_id: &str, _data: &[u8], _signature: &[u8]) -> Result<bool, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "Solo V2 requires async API: use device-backed signing and verify with the returned public key",
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
#[path = "provider_tests.rs"]
mod tests;
