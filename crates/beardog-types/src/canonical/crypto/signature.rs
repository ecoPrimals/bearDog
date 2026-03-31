// SPDX-License-Identifier: AGPL-3.0-only

use super::algorithms::{CryptoAlgorithm, HashAlgorithm};
use super::keys::KeyConfig;
use serde::{Deserialize, Serialize};

/// Digital signature configuration
/// `SignatureConfig`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureConfig {
    /// Signature algorithm
    /// The algorithm value
    pub algorithm: CryptoAlgorithm,
    /// The hash algorithm value
    pub hash_algorithm: HashAlgorithm,
    /// Key configuration
    pub key_config: KeyConfig,
    /// Serialized signature container format (e.g. `DER`, `P1363`) understood by verifiers.
    pub format: String,
}

impl Default for SignatureConfig {
    fn default() -> Self {
        Self {
            algorithm: CryptoAlgorithm::Ed25519,
            hash_algorithm: HashAlgorithm::default(),
            key_config: KeyConfig::default(),
            format: "DER".to_string(),
        }
    }
}
