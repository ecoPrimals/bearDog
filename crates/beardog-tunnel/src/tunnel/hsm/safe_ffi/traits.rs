// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub use beardog_errors::BearDogError;
// Use local types instead of canonical types for now
pub use crate::tunnel::hsm::types::{HsmKey, KeyType};

/// Platform-specific security provider trait
///
/// NOTE: This is a local definition until `beardog_traits::unified::PlatformProvider` exists
pub trait PlatformSecurityProvider: Send + Sync {
    /// Generate a new key
    ///
    /// # Errors
    ///
    /// Returns an error if key generation fails in the backend provider.
    fn generate_key(&self, key_id: &str, key_type: &KeyType) -> Result<HsmKey, BearDogError>;

    /// Sign data with a key
    ///
    /// # Errors
    ///
    /// Returns an error if signing fails in the backend provider.
    fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature
    ///
    /// # Errors
    ///
    /// Returns an error if signature verification fails in the backend provider.
    fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;
}

/// Biometric authentication provider trait
pub trait BiometricAuthProvider: Send + Sync {
    /// Authenticate using biometrics
    ///
    /// # Errors
    ///
    /// Returns an error if biometric authentication cannot be completed.
    fn authenticate(&self) -> Result<bool, BearDogError>;
}
