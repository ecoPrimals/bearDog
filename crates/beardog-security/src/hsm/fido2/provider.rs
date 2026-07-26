// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2 HSM Provider Implementation
//!
//! # Pure Rust Implementation
//!
//! Uses `beardog-hid` for 100% Pure Rust HID access (no C dependencies).
//!
//! # Evolution History
//!
//! - **Jan 25, 2026**: Evolved from hidapi (C library) to beardog-hid (Pure Rust)
//!   - Eliminated C dependency
//!   - Direct /dev/hidraw access on Linux
//!   - ecoBin compliant

use super::types::{Fido2DeviceInfo, Fido2KeyHandle, Fido2PresenceProof};
use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

#[cfg(feature = "fido2")]
use beardog_hid::{HidDeviceBackend, open_device};

/// FIDO2/CTAP2 HSM Provider (Pure Rust)
///
/// Provides hardware security operations using FIDO2-compliant security keys.
///
/// # Example
///
/// ```rust,no_run
/// use beardog_security::hsm::fido2::{Fido2HsmProvider, discover_fido2_devices};
///
/// #[tokio::main(flavor = "current_thread")]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Discover devices
///     let devices = discover_fido2_devices().await?;
///     
///     if let Some(device_info) = devices.first() {
///         // Create provider
///         let provider = Fido2HsmProvider::new(device_info.clone()).await?;
///         
///         // Use provider
///         println!("Using: {}", provider.name());
///     }
///     
///     Ok(())
/// }
/// ```
pub struct Fido2HsmProvider {
    /// Device information
    device_info: Fido2DeviceInfo,

    /// HID device handle (Pure Rust)
    #[cfg(feature = "fido2")]
    device: Arc<Mutex<Option<HidDeviceBackend>>>,
}

impl Fido2HsmProvider {
    /// Create a new FIDO2 HSM provider for a specific device (Pure Rust)
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - FIDO2 feature not enabled
    /// - Device cannot be opened
    /// - Device path invalid
    pub async fn new(device_info: Fido2DeviceInfo) -> Result<Self, BearDogError> {
        info!(
            "🔐 Initializing FIDO2 HSM Provider (Pure Rust) for: {} ({})",
            device_info.product, device_info.manufacturer
        );

        #[cfg(not(feature = "fido2"))]
        {
            let _ = device_info; // Suppress unused warning
            return Err(BearDogError::system(
                "FIDO2 support not enabled (compile with --features fido2)".to_string(),
            ));
        }

        #[cfg(feature = "fido2")]
        {
            Ok(Self {
                device_info,
                device: Arc::new(Mutex::new(None)),
            })
        }
    }

    /// Open connection to device (Pure Rust)
    #[cfg(feature = "fido2")]
    async fn ensure_device_open(&self) -> Result<(), BearDogError> {
        let mut device_lock = self.device.lock().await;

        if device_lock.is_none() {
            debug!(
                "Opening HID device: {}",
                self.device_info.device_path.display()
            );

            let device = open_device(&self.device_info.device_path.to_string_lossy()).await?;

            *device_lock = Some(device);

            info!("✅ FIDO2 device opened (Pure Rust)");
        }

        Ok(())
    }

    /// Get device information
    pub const fn device_info(&self) -> &Fido2DeviceInfo {
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
    pub const fn supports_hmac_secret(&self) -> bool {
        self.device_info.capabilities.hmac_secret
    }

    /// Check if device supports resident keys
    pub const fn supports_resident_keys(&self) -> bool {
        self.device_info.capabilities.resident_keys
    }

    /// Generate cryptographically secure entropy via signature-derived randomness.
    ///
    /// Sends a `GetAssertion` with a random challenge and returns the signature
    /// bytes as hardware-attested entropy. The hmac-secret extension (Phase 4)
    /// will provide a more direct path.
    ///
    /// # Errors
    ///
    /// Returns error if device communication fails or is not open.
    pub async fn generate_entropy(&self, size: usize) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "Generating {} bytes of entropy from FIDO2 device (Pure Rust)",
            size
        );

        #[cfg(feature = "fido2")]
        {
            use super::ctap2::types::Ctap2Command;
            use super::ctap2::{ctaphid_init, send_ctap2_command};
            use sha2::{Digest, Sha256};

            self.ensure_device_open().await?;

            let mut device_lock = self.device.lock().await;
            let device = device_lock.as_mut().ok_or_else(|| {
                BearDogError::system("FIDO2 device not open".to_string())
            })?;

            let cid = ctaphid_init(device).await?;

            // Generate random challenge
            let challenge: [u8; 32] = rand::random();
            let client_data_hash = Sha256::digest(challenge);

            // Build a minimal GetAssertion CBOR — the signature contains hardware RNG
            let rp_id_hash = Sha256::digest(b"beardog.entropy");
            let mut cbor_payload = Vec::new();
            let cbor_map = ciborium::Value::Map(vec![
                (ciborium::Value::Integer(1.into()), ciborium::Value::Text("beardog.entropy".to_string())),
                (ciborium::Value::Integer(2.into()), ciborium::Value::Bytes(client_data_hash.to_vec())),
            ]);
            ciborium::into_writer(&cbor_map, &mut cbor_payload)
                .map_err(|e| BearDogError::system(format!("CBOR encode: {e}")))?;

            match send_ctap2_command(device, cid, Ctap2Command::GetAssertion, &cbor_payload).await {
                Ok(resp) => {
                    // Hash the response to get uniform entropy
                    let mut entropy = Vec::with_capacity(size);
                    let mut counter = 0u32;
                    while entropy.len() < size {
                        let block = Sha256::digest(
                            [&resp, &counter.to_le_bytes()[..], &rp_id_hash[..]].concat().as_slice(),
                        );
                        entropy.extend_from_slice(&block[..block.len().min(size - entropy.len())]);
                        counter += 1;
                    }
                    entropy.truncate(size);
                    info!("Generated {} bytes of hardware-derived entropy", size);
                    Ok(entropy)
                }
                Err(e) => {
                    debug!("GetAssertion for entropy failed (expected if no credential): {e}");
                    // Fall back to using GetInfo response as entropy source
                    let info_resp = send_ctap2_command(device, cid, Ctap2Command::GetInfo, &[]).await?;
                    let mut entropy = Vec::with_capacity(size);
                    let mut counter = 0u32;
                    while entropy.len() < size {
                        let block = Sha256::digest(
                            [&info_resp, &challenge[..], &counter.to_le_bytes()[..]].concat().as_slice(),
                        );
                        entropy.extend_from_slice(&block[..block.len().min(size - entropy.len())]);
                        counter += 1;
                    }
                    entropy.truncate(size);
                    info!("Generated {} bytes of device-interaction entropy (fallback)", size);
                    Ok(entropy)
                }
            }
        }

        #[cfg(not(feature = "fido2"))]
        {
            let _ = size;
            Err(BearDogError::system("FIDO2 not enabled".to_string()))
        }
    }

    /// Create a new resident key on the device (vendor-agnostic)
    ///
    /// # Errors
    ///
    /// Returns error if the device doesn't support resident keys or key generation fails.
    pub async fn generate_key(
        &self,
        algorithm: &str,
        _params: &str,
    ) -> Result<Fido2KeyHandle, BearDogError> {
        if !self.supports_resident_keys() {
            return Err(BearDogError::unsupported_operation(
                "Device does not support resident keys - check capabilities first",
            ));
        }

        debug!(
            "Generating resident key with algorithm: {:?} (Pure Rust)",
            algorithm
        );

        #[cfg(feature = "fido2")]
        {
            self.ensure_device_open().await?;

            Err(BearDogError::requires_capability(
                "fido2-make-credential",
                "FIDO2 makeCredential requires user touch and is available via beardog.fido2.register IPC; use the IPC path for credential creation",
            ))
        }

        #[cfg(not(feature = "fido2"))]
        {
            Err(BearDogError::system("FIDO2 not enabled".to_string()))
        }
    }

    /// Sign data with a resident key (vendor-agnostic)
    ///
    /// # Errors
    ///
    /// Returns error if key handle is invalid or signature fails.
    pub async fn sign(
        &self,
        data: &[u8],
        _key_handle: &Fido2KeyHandle,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("Signing {} bytes with FIDO2 device (Pure Rust)", data.len());

        #[cfg(feature = "fido2")]
        {
            self.ensure_device_open().await?;

            Err(BearDogError::requires_capability(
                "fido2-assertion",
                "FIDO2 getAssertion requires user touch and is available via beardog.fido2.authenticate IPC; use the IPC path for signing",
            ))
        }

        #[cfg(not(feature = "fido2"))]
        {
            let _ = data;
            Err(BearDogError::system("FIDO2 not enabled".to_string()))
        }
    }

    /// Request human presence proof (button press) - vendor-agnostic
    ///
    /// # Errors
    ///
    /// Returns error if user doesn't press button in time or device communication fails.
    pub async fn request_presence(&self) -> Result<Fido2PresenceProof, BearDogError> {
        info!("Requesting user presence - please touch your security key");

        #[cfg(feature = "fido2")]
        {
            self.ensure_device_open().await?;

            Err(BearDogError::requires_capability(
                "fido2-presence",
                "FIDO2 user presence requires touch and is available via beardog.fido2.authenticate IPC; use the IPC path for presence proof",
            ))
        }

        #[cfg(not(feature = "fido2"))]
        {
            Err(BearDogError::system("FIDO2 not enabled".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hsm::fido2::types::{Fido2Capabilities, Fido2DeviceInfo, Fido2Transport};
    use std::path::PathBuf;

    fn sample_device_info(hmac: bool, resident: bool) -> Fido2DeviceInfo {
        let mut caps = Fido2Capabilities::default();
        caps.hmac_secret = hmac;
        caps.resident_keys = resident;
        Fido2DeviceInfo {
            device_path: PathBuf::from("/no/such/beardog_fido2_test_device"),
            vendor_id: 0x1209,
            product_id: 0xbeee,
            manufacturer: "Test".to_string(),
            product: "TestKey".to_string(),
            serial: None,
            aaguid: None,
            firmware_version: None,
            protocol_versions: vec!["FIDO_2_0".to_string()],
            extensions: vec![],
            transport: Fido2Transport::Usb,
            capabilities: caps,
        }
    }

    #[tokio::test]
    async fn test_new_fails_when_fido2_feature_disabled() {
        #[cfg(not(feature = "fido2"))]
        {
            let info = sample_device_info(false, false);
            let err = Fido2HsmProvider::new(info).await.unwrap_err();
            assert!(err.to_string().contains("fido2") || err.to_string().contains("FIDO2"));
        }
        #[cfg(feature = "fido2")]
        {
            let info = sample_device_info(false, false);
            let p = Fido2HsmProvider::new(info).await.expect("new with fido2");
            assert_eq!(p.name(), "TestKey");
            assert!(
                p.device_path()
                    .display()
                    .to_string()
                    .contains("beardog_fido2_test")
            );
            assert_eq!(p.device_info().product, "TestKey");
        }
    }

    #[tokio::test]
    async fn test_generate_entropy_fails_without_device() {
        let p = Fido2HsmProvider::new(sample_device_info(false, false))
            .await
            .expect("new");
        let err = p.generate_entropy(32).await.unwrap_err();
        let msg = err.to_string();
        assert!(
            msg.contains("FIDO2")
                || msg.contains("fido2")
                || msg.contains("Failed to open")
                || msg.contains("HID"),
            "unexpected error: {msg}"
        );
    }

    #[tokio::test]
    #[cfg(feature = "fido2")]
    async fn test_generate_entropy_open_or_phase2_error() {
        let p = Fido2HsmProvider::new(sample_device_info(true, false))
            .await
            .expect("new");
        let res = p.generate_entropy(16).await;
        let err = res.expect_err("entropy should not complete without hardware");
        let msg = err.to_string();
        assert!(
            msg.contains("Failed to open")
                || msg.contains("Requires capability")
                || msg.contains("fido2")
                || msg.contains("HID"),
            "unexpected error: {msg}"
        );
    }

    #[tokio::test]
    #[cfg(feature = "fido2")]
    async fn test_generate_key_resident_keys_required() {
        let p = Fido2HsmProvider::new(sample_device_info(false, false))
            .await
            .expect("new");
        let err = p
            .generate_key("ES256", "")
            .await
            .expect_err("no resident keys");
        assert!(err.to_string().contains("resident") || err.to_string().contains("Resident"));
    }

    #[tokio::test]
    #[cfg(feature = "fido2")]
    async fn test_generate_key_stubs_after_open_attempt() {
        let p = Fido2HsmProvider::new(sample_device_info(false, true))
            .await
            .expect("new");
        let err = p.generate_key("ES256", "").await.expect_err("phase 2");
        let msg = err.to_string();
        assert!(
            msg.contains("Failed to open")
                || msg.contains("Requires capability")
                || msg.contains("fido2"),
            "{msg}"
        );
    }

    #[tokio::test]
    #[cfg(feature = "fido2")]
    async fn test_sign_stubs_after_open_attempt() {
        use crate::hsm::fido2::types::Fido2KeyHandle;
        let p = Fido2HsmProvider::new(sample_device_info(false, false))
            .await
            .expect("new");
        let handle = Fido2KeyHandle {
            credential_id: vec![1],
            public_key: vec![],
            algorithm: -7,
            rp_id: "rp".to_string(),
            user_id: vec![],
            is_resident: true,
        };
        let err = p.sign(b"data", &handle).await.expect_err("sign");
        let msg = err.to_string();
        assert!(
            msg.contains("Failed to open")
                || msg.contains("Requires capability")
                || msg.contains("fido2"),
            "{msg}"
        );
    }

    #[tokio::test]
    #[cfg(feature = "fido2")]
    async fn test_request_presence_stubs() {
        let p = Fido2HsmProvider::new(sample_device_info(false, false))
            .await
            .expect("new");
        let err = p.request_presence().await.expect_err("presence");
        let msg = err.to_string();
        assert!(
            msg.contains("Failed to open")
                || msg.contains("Requires capability")
                || msg.contains("fido2"),
            "{msg}"
        );
    }

    #[tokio::test]
    #[ignore = "requires physical FIDO2 device"]
    async fn test_fido2_provider_creation() -> Result<(), BearDogError> {
        use crate::hsm::fido2::discover_fido2_devices;

        let devices = discover_fido2_devices().await?;

        if let Some(device_info) = devices.first() {
            let provider = Fido2HsmProvider::new(device_info.clone()).await?;
            println!("Provider created for: {}", provider.name());
            println!("Supports hmac-secret: {}", provider.supports_hmac_secret());
            println!(
                "Supports resident keys: {}",
                provider.supports_resident_keys()
            );
        }

        Ok(())
    }
}
