//! Trust management module for BearDog security registry
//!
//! This module provides functionality for managing trust relationships,
//! verification, and propagation within the BearDog ecosystem.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::info;

mod propagation;
mod verifier;

pub use propagation::TrustPropagation;
pub use verifier::TrustVerifier;

/// Trust configuration settings
#[derive(Debug, Clone)]
pub struct TrustConfig {
    /// Timeout for trust verification in seconds
    pub verification_timeout_secs: u64,
    /// Enable trust propagation
    pub enable_propagation: bool,
}

impl Default for TrustConfig {
    #[inline]
    fn default() -> Self {
        Self {
            verification_timeout_secs: 30,
            enable_propagation: true,
        }
    }
}

/// Trust level enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Untrusted entity
    Untrusted,
    /// Basic trust level
    Basic,
    /// Verified trust level
    Verified,
    /// Full trust level
    Full,
}

/// Trust store for managing trust relationships
#[derive(Debug, Clone)]
pub struct TrustStore {
    /// Trust relationships mapping
    relationships: HashMap<String, TrustLevel>,
}

impl TrustStore {
    /// Create a new trust store
    pub fn new() -> Self {
        Self {
            relationships: HashMap::new(),
        }
    }

    /// Get trust level for an entity
    pub fn get_trust_level(&self, entity: &str) -> Option<&TrustLevel> {
        self.relationships.get(entity)
    }

    /// Set trust level for an entity
    pub fn set_trust_level(&mut self, entity: String, level: TrustLevel) {
        self.relationships.insert(entity, level);
    }

    /// Get the number of trust relationships
    pub fn count(&self) -> usize {
        self.relationships.len()
    }
}

impl Default for TrustStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Trust manager for coordinating trust operations
#[derive(Debug)]
pub struct TrustManager {
    /// Trust store
    store: TrustStore,
    /// Trust verifier
    verifier: TrustVerifier,
    /// Trust propagation handler
    propagation: TrustPropagation,
}

impl TrustManager {
    /// Create a new trust manager
    pub fn new(config: TrustConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            store: TrustStore::new(),
            verifier: TrustVerifier::new(config.clone()),
            propagation: TrustPropagation::new(config),
        })
    }

    /// Verify trust for an entity
    pub async fn verify_trust(&mut self, entity: &str) -> Result<TrustLevel, BearDogError> {
        info!("Verifying trust for entity: {}", entity);
        
        // Check existing trust level
        if let Some(level) = self.store.get_trust_level(entity) {
            return Ok(level.clone());
        }

        // Perform verification
        let level = self.verifier.verify(entity).await?;
        self.store.set_trust_level(entity.to_string(), level.clone());
        
        Ok(level)
    }

    /// Propagate trust to related entities
    pub async fn propagate_trust(
        &mut self,
        entity: &str,
        level: TrustLevel,
    ) -> Result<(), BearDogError> {
        info!("Propagating trust level {:?} for entity: {}", level, entity);
        
        match self.propagation.propagate(entity, &level).await {
            Ok(_) => {
                info!("Trust propagation completed successfully");
                Ok(())
            }
            Err(e) => {
                info!("Trust propagation failed: {}", e);
                Err(e)
            }
        }
    }
}
