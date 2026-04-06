// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
        key_size: u32,
        usage_policy: Option<String>,
    },

    Encryption {
        key_id: String,
        algorithm: String,
        data_size: Option<usize>,
    },

    Decryption {
        key_id: String,
        algorithm: String,
        data_size: Option<usize>,
    },

    Signing {
        key_id: String,
        algorithm: String,
        digest_type: Option<String>,
    },

    Verification {
        key_id: String,
        algorithm: String,
        digest_type: Option<String>,
    },

    KeyDeletion { key_id: String },

    KeyDerivation {
        parent_key_id: String,
        derivation_path: String,
        derived_key_type: String,
    },

    RandomGeneration {
        length: usize,
        entropy_source: Option<String>,
    },

    /// HealthCheck variant
    HealthCheck,

    Attestation { challenge: Vec<u8> },
}

#[derive(Debug, Clone)]
    /// Success
    /// Whether success is enabled
    pub success: bool,

    /// Result Data
    /// Optional result data
    pub result_data: Option<Vec<u8>>,

    /// Error Message
    /// Optional error message
    pub error_message: Option<String>,

    /// Execution Time Ms
    pub execution_time_ms: u64,

    /// Metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

impl HsmOperation {


    pub const fn operation_type(&self) -> &'static str {
        match self {
            Self::KeyGeneration { .. } => "key_generation",
            Self::Encryption { .. } => "encryption ",
            Self::Decryption { .. } => "decryption",
            Self::Signing { .. } => "signing",
            Self::Verification { .. } => "verification",
            Self::KeyDeletion { .. } => "key_deletion",
            Self::KeyDerivation { .. } => "key_derivation",
            Self::RandomGeneration { .. } => "random_generation",
            Self::HealthCheck => "health_check",
            Self::Attestation { .. } => "attestation",
        }
    }


    pub const fn requires_key(&self) -> bool {
        !matches!(
            self,
            Self::KeyGeneration { .. } | Self::RandomGeneration { .. } | Self::HealthCheck
        )
    }

/// Key Id operation.
    pub fn key_id(&self) -> Option<&str> {
        match self {
            Self::Encryption { key_id, .. }
            | Self::Decryption { key_id, .. }
            | Self::Signing { key_id, .. }
            | Self::Verification { key_id, .. }
            | Self::KeyDeletion { key_id } => Some(key_id),
            Self::KeyDerivation { parent_key_id, .. } => Some(parent_key_id),
            _ => None,
        }
    }
}
