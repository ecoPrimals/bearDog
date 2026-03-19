// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub use beardog_errors::BearDogError;
// Use local types instead of canonical types for now
pub use crate::tunnel::hsm::types::{HsmKey, KeyType};

/// Platform-specific security provider trait
///
/// NOTE: This is a local definition until beardog_traits::unified::PlatformProvider exists
pub trait PlatformSecurityProvider: Send + Sync {
    /// Generate a new key
    fn generate_key(&self, key_id: &str, key_type: &KeyType) -> Result<HsmKey, BearDogError>;

    /// Sign data with a key
    fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    /// Verify a signature
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
    fn authenticate(&self) -> Result<bool, BearDogError>;
}
