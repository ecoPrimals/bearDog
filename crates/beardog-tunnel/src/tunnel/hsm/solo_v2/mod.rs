// SPDX-License-Identifier: AGPL-3.0-or-later

//! Solo V2 USB Security Key HSM Provider
//!
//! Provides hardware-backed cryptographic operations using `SoloKeys` Solo V2
//! USB security keys via the FIDO2/CTAP2 protocol.
//!
//! # Overview
//!
//! This module integrates Solo V2 hardware security keys as an HSM provider
//! for `BearDog`. It uses the CTAP2 (Client to Authenticator Protocol 2) to
//! communicate with the device over USB HID.
//!
//! # Features
//!
//! - Hardware-backed key generation
//! - FIDO2/WebAuthn compatible operations
//! - PIN protection
//! - Biometric support (if available on device)
//! - Resident key storage on device
//! - USB HID communication
//!
//! # Architecture
//!
//! ```text
//! BearDog
//!    ↓
//! Solo V2 Provider (this module)
//!    ↓
//! CTAP2 Protocol (beardog-hid - pure Rust, ecoBin compliant)
//!    ↓
//! USB HID Layer (beardog-hid - no C dependencies)
//!    ↓
//! Solo V2 Hardware
//! ```
//!
//! # Security Model
//!
//! - Private keys never leave the hardware device
//! - PIN required for operations (configured per-key)
//! - Hardware-backed attestation available
//! - Tamper-resistant key storage
//!
//! # Usage
//!
//! ```ignore
//! use beardog_tunnel::hsm::{SoloV2Provider, UniversalHsmProvider};
//!
//! // Discover Solo V2 devices
//! let devices = SoloV2Provider::discover_devices()?;
//!
//! // Create provider for first device
//! let provider = SoloV2Provider::new(devices[0].clone())?;
//!
//! // Generate a key on the device
//! let key = provider.generate_key(KeyType::Ed25519)?;
//!
//! // Sign data (will prompt for PIN if needed)
//! let signature = provider.sign(&key_id, data)?;
//! ```
//!
//! # Platform Support
//!
//! This module requires the `solo-v2` feature flag:
//! ```toml
//! [dependencies]
//! beardog-tunnel = { version = "0.9", features = ["solo-v2"] }
//! ```
//!
//! USB permissions may need to be configured on Linux:
//! - Add udev rule for Solo V2 VID:PID
//! - Add user to `plugdev` group
//!
//! # Limitations
//!
//! - Requires physical USB connection
//! - May require user interaction (PIN entry)
//! - Limited key storage (device-dependent)
//! - Platform-specific USB permissions

pub mod ctap2_protocol;
pub mod provider;
pub mod transport;
pub mod types;

#[cfg(feature = "ctap2")]
pub mod hid_transport;

#[cfg(feature = "ctap2")]
pub use hid_transport::{Ctap2TransportBackend, HidCtap2Transport};

pub use ctap2_protocol::{
    GetAssertionResponse, MakeCredentialResponse, build_get_assertion, build_make_credential,
    parse_get_assertion_response, parse_make_credential_response,
};
pub use provider::SoloV2Provider;
pub use transport::Ctap2Transport;
pub use types::*;
