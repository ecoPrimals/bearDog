//! Core BearDog PrimalProvider implementation
//!
//! This module contains the main BearDogPrimalProvider struct and its constructor.

use std::collections::HashMap;
use std::sync::Arc;

use super::super::traits::*;
use crate::BearDogCore;

/// BearDog's universal PrimalProvider implementation
///
/// This implementation shows how BearDog integrates with the universal ecosystem
/// as a security provider, offering security capabilities to all ecosystem components.
pub struct BearDogPrimalProvider {
    /// Core BearDog instance
    pub(crate) core: Arc<BearDogCore>,

    /// Instance identifier
    pub(crate) instance_id: String,

    /// Service endpoints
    pub(crate) endpoints: ServiceEndpoints,

    /// Provider metadata
    pub(crate) metadata: ProviderMetadata,
}

impl BearDogPrimalProvider {
    /// Create a new BearDog PrimalProvider
    pub fn new(core: Arc<BearDogCore>, instance_id: String) -> Self {
        let endpoints = ServiceEndpoints {
            primary: "http://localhost:8443".to_string(),
            health: "http://localhost:8443/health".to_string(),
            metrics: Some("http://localhost:8443/metrics".to_string()),
            admin: Some("http://localhost:8443/admin".to_string()),
            events: Some("http://localhost:8443/events".to_string()),
            custom: HashMap::new(),
        };

        let metadata = ProviderMetadata {
            name: "BearDog Security Provider".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: "Universal security provider for the ecoPrimals ecosystem".to_string(),
            author: "ecoPrimals Security Team".to_string(),
            website: Some("https://github.com/ecoprimal/beardog".to_string()),
            license: "AGPL-3.0".to_string(),
            tags: vec![
                "security".to_string(),
                "encryption".to_string(),
                "authentication".to_string(),
                "authorization".to_string(),
                "audit".to_string(),
                "compliance".to_string(),
            ],
            custom: HashMap::new(),
        };

        Self {
            core,
            instance_id,
            endpoints,
            metadata,
        }
    }
} 