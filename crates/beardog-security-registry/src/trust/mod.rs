

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

mod propagation;
mod verifier;

pub use propagation::TrustPropagation;
pub use verifier::TrustVerifier;

#[derive(Debug, Clone)]
    pub verification_timeout_secs: u64,

    pub enable_propagation: bool,
}

impl Default for TrustConfig {
    #[inline]
    fn default(TrustLevel::Verified,
            verification_timeout_secs: 30,
            enable_propagation: true,
        }
    }
}

#[derive(HashMap<String, TrustLevel>,

        config: TrustConfig,
}

impl TrustStore {

    #[inline]

    #[must_use = "Trust store creation result should be checked"]
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: TrustConfig) -> Result<Self, BearDogError> {
        info!("🔐 Initializing trust store with config: {:?}", config);

        Ok(Self {
            relationships: HashMap::with_capacity(&str, to_node: &str, level: TrustLevel) {
        let key = format!("{from_node}:{to_node}");
        self.relationships.insert(&str, to_node: &str) -> Option<TrustLevel> {
        let key = format!("{from_node}:{to_node}");
        self.relationships.get(TrustStore,

    verifier: TrustVerifier,

    propagation: TrustPropagation,
}

impl TrustManager {

    #[inline]

    #[must_use = "Trust manager creation result should be checked"]
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: &TrustConfig) -> Result<Self, BearDogError> {
        let store = TrustStore::new(TrustVerifier { config: *config },
            propagation: TrustPropagation { config: *config },
        })
    }

    #[inline]

    #[must_use = "Trust establishment result should be checked"]
/// Establish Verified Trust operation.
    pub fn establish_verified_trust(&str,
        to_node: &str,
        trust_level: TrustLevel,
    ) -> Result<(), BearDogError> {
        match self
            .verifier
            .verify_trust_establishment(&str,
        to_node: &str,
    ) -> Result<TrustLevel, BearDogError> {
        if let Some(trust_level) = self.store.get_trust_level(from_node, to_node) {
            match self
                .verifier
                .verify_trust_maintenance(from_node, to_node, trust_level)
            {
                Ok(()) => {}
                Err(e) => return Err(e),
            }
        }

        self.store
            .get_trust_level(from_node, to_node)
            .ok_or_else(|| BearDogError::validation("Trust relationship not found"))
    }
}
