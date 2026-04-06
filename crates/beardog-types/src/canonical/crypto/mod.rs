// SPDX-License-Identifier: AGPL-3.0-or-later

// Cryptographic types and configurations for BearDog
// Provides structured definitions for cryptographic algorithms, keys, and security settings

mod algorithms;
mod constants;
mod crypto_config;
mod key_management;
mod keys;
mod signature;

pub use algorithms::{
    CryptoAlgorithm, EncryptionMode, HashAlgorithm, KeyDerivationFunction, PaddingScheme,
};
pub use crypto_config::{CryptoConfig, EncryptionConfig};
pub use key_management::{EscrowConfig, HsmConfig, KeyManagementConfig, RngConfig, RotationPolicy};
pub use keys::{CryptoKeyPair, KeyConfig, KeyPairAlgorithm, KeyUsage};
pub use signature::SignatureConfig;

#[cfg(test)]
#[path = "crypto_tests.rs"]
mod tests;
