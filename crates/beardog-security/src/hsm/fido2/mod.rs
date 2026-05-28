// SPDX-License-Identifier: AGPL-3.0-or-later

//! FIDO2/CTAP2 HSM Provider
//!
//! Support for modern security keys using the FIDO2/CTAP2 protocol.
//!
//! ## Supported Devices
//!
//! - **`SoloKeys` Solo 2**: Full support (FIDO 2.1, hmac-secret extension)
//! - **`YubiKey` 5 Series** (FIDO2 mode): Full support
//! - **Titan Security Key**: Full support
//! - **`OnlyKey`**: Compatible devices
//! - **Any FIDO2-compliant security key**
//!
//! ## Features
//!
//! - **Hardware Entropy**: Via `hmac-secret` extension
//! - **Resident Keys**: Hardware-backed credential storage
//! - **Signatures**: Ed25519, ES256 (ECDSA P-256)
//! - **Human Presence**: Button press verification
//! - **Device Attestation**: Hardware authenticity verification
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_security::hsm::fido2::Fido2HsmProvider;
//!
//! // Discover all FIDO2 devices
//! let devices = Fido2HsmProvider::discover().await?;
//!
//! for device in devices {
//!     println!("Found: {} ({})", device.name(), device.device_path());
//!     
//!     // Generate entropy
//!     let entropy = device.generate_entropy(32).await?;
//!     
//!     // Create resident key
//!     let key_handle = device.generate_key(KeyAlgorithm::Ed25519, params).await?;
//!     
//!     // Sign with button press
//!     let signature = device.sign(data, &key_handle).await?;
//! }
//! ```

pub mod constants; // Protocol constants
pub mod ctap2; // CTAP2 protocol implementation
pub mod discovery;
pub mod multi_credential_provider;
mod provider;
pub mod types;

pub use constants::*;
pub use ctap2::types::Ctap2Command;
pub use discovery::discover_fido2_devices;
pub use multi_credential_provider::*;
pub use provider::Fido2HsmProvider;
pub use types::*;
