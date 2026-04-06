// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key Management Capability Trait
//!
//! Vendor-agnostic HSM/KMS abstraction that eliminates hardcoded cloud provider
//! dependencies (AWS KMS, Azure Key Vault, GCP KMS, etc.).
//!
//! # Philosophy: Infant Discovery
//!
//! BearDog starts without knowledge of which KMS provider is available.
//! It detects AWS, Azure, GCP, or falls back to software HSM, using the
//! same interface regardless of provider.
//!
//! # Architecture
//!
//! ```text
//! KeyManagementCapability (trait)
//! ├── AwsKmsProvider         (if AWS credentials available)
//! ├── AzureKeyVaultProvider  (if Azure credentials available)
//! ├── GcpKmsProvider         (if GCP credentials available)
//! ├── Pkcs11HsmProvider      (if hardware HSM available)
//! └── SoftwareHsmProvider    (fallback - always available)
//! ```
//!
//! # Examples
//!
//! ## Auto-Detection
//!
//! ```rust,no_run
//! use beardog_types::canonical::discovery::key_management_capability::*;
//! use beardog_types::canonical::types::ids::KeyId;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), KmsError> {
//! # let key_id = KeyId::new("example-key".to_string());
//! // Automatically detects and uses best available KMS
//! let kms = create_key_management().await?;
//!
//! // Use it - no need to know if it's AWS, Azure, or local!
//! let ciphertext = kms.encrypt(b"sensitive data", &key_id).await?;
//! # Ok(())
//! # }
//! ```

mod discovery;
mod software_hsm_provider;
mod types;

#[cfg(test)]
mod tests;

pub use discovery::create_key_management;
pub use software_hsm_provider::SoftwareHsmProvider;
pub use types::{
    KeyAlgorithm, KeyManagementCapability, KeyMetadata, KeySpec, KeyState, KeyUsage,
    KmsCapabilities, KmsError, KmsHealthStatus,
};
