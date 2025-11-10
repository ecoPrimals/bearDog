//! Hardware Security Module (HSM) Integration
//!
//! Multi-protocol HSM support for BearDog, providing truly hardware-agnostic
//! cryptographic operations.
//!
//! ## Supported Protocols
//!
//! - **PKCS#11**: Traditional smart cards, YubiKey PIV mode
//! - **FIDO2/CTAP2**: Modern security keys (SoloKeys, YubiKey FIDO2 mode) - NEW!
//! - **Android StrongBox**: Pixel Titan M2, mobile HSMs
//! - **iOS Secure Enclave**: iPhone cryptographic operations
//! - **TPM 2.0**: Platform TPM chips (coming soon)
//! - **OpenPGP Card**: PGP smart cards (coming soon)
//!
//! ## Protocol-Agnostic Design
//!
//! All HSM providers implement the `UniversalHsmProvider` trait, allowing
//! application code to work with any hardware without protocol-specific logic.
//!
//! ```rust,ignore
//! use beardog_security::hsm::discover_all_hsm_devices;
//!
//! // Discover ALL security hardware, regardless of protocol
//! let devices = discover_all_hsm_devices().await?;
//!
//! for device in devices {
//!     // Works with PKCS#11, FIDO2, TPM, Android, iOS, etc.
//!     let entropy = device.generate_entropy(32).await?;
//!     println!("Got entropy from: {}", device.device_info().name);
//! }
//! ```

// PKCS#11 provider temporarily disabled due to syntax errors
// pub mod pkcs11_provider;

#[cfg(feature = "fido2")]
pub mod fido2;

#[cfg(feature = "tpm2")]
pub mod tpm2;

#[cfg(feature = "openpgp")]
pub mod openpgp;

#[cfg(target_os = "android")]
pub mod android_strongbox;

// Re-exports
// pub use pkcs11_provider::*;

#[cfg(feature = "fido2")]
pub use fido2::*;

#[cfg(target_os = "android")]
pub use android_strongbox::*;

/// Universal HSM Entropy Orchestrator
///
/// Connects all available HSMs (FIDO2, Android StrongBox, iOS Secure Enclave)
/// to BearDog's entropy hierarchy system for human-owned randomness.
pub mod entropy_orchestrator;

