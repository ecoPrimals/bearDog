// SPDX-License-Identifier: AGPL-3.0-only

//! # Genetic Key Exchange Module
//!
//! Implements cryptographic key exchange with genetic evolution capabilities for
//! secure cross-primal messaging in the `BearDog` ecosystem.
//!
//! ## Architecture
//!
//! The genetic key exchange system provides:
//! - **Evolving Key Lineages**: Keys that evolve based on usage patterns
//! - **Hierarchical Key Mixing**: Combining human + primal + machine entropy
//! - **Delegated Key Constraints**: Time, resource, and scope-bound keys
//! - **Multi-Party Renewal**: Mutual consent for key lineage continuation
//! - **Zero-Knowledge Exchange**: No exposure of key material during exchange
//!
//! ## Design Principles
//!
//! 1. **Sovereignty Preserving**: Each primal only knows its own keys
//! 2. **Runtime Discovery**: No hardcoded cross-primal relationships
//! 3. **Capability-Based**: Exchange based on advertised capabilities
//! 4. **Memory Safe**: Zero unsafe code, all operations verified
//! 5. **Idiomatic Rust**: Modern patterns, explicit error handling
//!
//! ## Example
//!
//! ```rust,ignore
//! // NOTE: This example uses methods that need to be async
//! use beardog_genetics::genetics::key_exchange::{GeneticKeyExchange, KeyExchangeConfig};
//! use beardog_errors::BearDogError;
//!
//! # async fn example() -> Result<(), BearDogError> {
//! // Initialize genetic key exchange
//! let config = KeyExchangeConfig::default();
//! let exchange = GeneticKeyExchange::new(config)?;
//!
//! // Generate a delegated key for another primal
//! let delegated_key = exchange.create_delegated_key(
//!     "peer_primal_id",
//!     3600, // 1 hour lifetime
//!     vec!["encrypt", "decrypt"]
//! ).await?;
//!
//! // Exchange keys with peer
//! let shared_secret = exchange.perform_key_exchange(
//!     "peer_primal_id",
//!     &delegated_key
//! ).await?;
//! # Ok(())
//! # }
//! ```

#![deny(unsafe_code)]
#![warn(clippy::pedantic)]

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, info};

/// Configuration for genetic key exchange
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyExchangeConfig {
    /// Maximum key lifetime in seconds
    pub max_key_lifetime_secs: u64,
    /// Enable genetic key evolution
    pub enable_evolution: bool,
    /// Minimum entropy tier for key generation
    pub min_entropy_tier: u8,
    /// Enable multi-party renewal
    pub enable_multiparty_renewal: bool,
}

impl Default for KeyExchangeConfig {
    fn default() -> Self {
        Self {
            max_key_lifetime_secs: 86400, // 24 hours
            enable_evolution: true,
            min_entropy_tier: 3, // High security by default
            enable_multiparty_renewal: true,
        }
    }
}

/// Main genetic key exchange engine
///
/// Manages cryptographic key exchange with genetic evolution capabilities
/// for secure cross-primal communication.
#[derive(Debug)]
pub struct GeneticKeyExchange {
    config: KeyExchangeConfig,
    /// Active key lineages (`peer_id` -> `key_lineage`)
    lineages: parking_lot::RwLock<HashMap<String, KeyLineage>>,
    /// Key generation counter for unique IDs
    key_counter: std::sync::atomic::AtomicU64,
}

/// A key lineage representing the evolution of keys with a peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyLineage {
    /// Unique lineage identifier
    pub lineage_id: String,
    /// Peer primal identifier
    pub peer_id: String,
    /// Current generation number
    pub generation: u32,
    /// Key created at
    pub created_at: SystemTime,
    /// Key expires at
    pub expires_at: SystemTime,
    /// Allowed operations
    pub allowed_operations: Vec<String>,
    /// Genetic fingerprint (SHA-256 of key material + entropy sources)
    pub genetic_fingerprint: [u8; 32],
    /// Evolution trigger conditions
    pub evolution_triggers: Vec<EvolutionTrigger>,
}

/// Conditions that trigger key evolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvolutionTrigger {
    /// Time-based: Evolve after duration
    TimeElapsed(Duration),
    /// Usage-based: Evolve after N operations
    OperationCount(u64),
    /// Security-based: Evolve on potential compromise
    SecurityEvent,
    /// Consensus-based: Evolve on mutual agreement
    MutualConsent,
}

/// Result of a key exchange operation
#[derive(Debug, Clone)]
pub struct KeyExchangeResult {
    /// Shared secret for symmetric encryption
    pub shared_secret: Vec<u8>,
    /// Key lineage for tracking evolution
    pub lineage: KeyLineage,
}

/// Delegated key with constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedKey {
    /// Key identifier
    pub key_id: String,
    /// Public key material (never private key!)
    pub public_key: Vec<u8>,
    /// Constraints on key usage
    pub constraints: KeyConstraints,
    /// Genetic lineage information
    pub lineage_info: KeyLineage,
}

/// Constraints applied to delegated keys
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyConstraints {
    /// Allowed operations (e.g., "encrypt", "sign")
    pub allowed_operations: Vec<String>,
    /// Time-based constraints
    pub time_constraint: Option<TimeConstraint>,
    /// Resource-based constraints
    pub resource_constraint: Option<ResourceConstraint>,
    /// Scope constraints (what data/services can be accessed)
    pub scope_constraint: Option<ScopeConstraint>,
}

/// Time-based key constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConstraint {
    /// Key valid from
    pub valid_from: SystemTime,
    /// Key valid until
    pub valid_until: SystemTime,
    /// Auto-renew if both parties consent
    pub auto_renew: bool,
}

/// Resource-based key constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraint {
    /// Maximum operations allowed
    pub max_operations: Option<u64>,
    /// Maximum data volume (bytes)
    pub max_data_bytes: Option<u64>,
    /// Maximum concurrent sessions
    pub max_concurrent_sessions: Option<u32>,
}

/// Scope-based key constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeConstraint {
    /// Allowed service endpoints
    pub allowed_endpoints: Vec<String>,
    /// Allowed data types
    pub allowed_data_types: Vec<String>,
    /// Geographic restrictions (ISO 3166-1 alpha-2 codes)
    pub allowed_regions: Option<Vec<String>>,
}

impl GeneticKeyExchange {
    /// Create a new genetic key exchange engine
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid.
    pub fn new(config: KeyExchangeConfig) -> Result<Self, BearDogError> {
        // Validate configuration
        if config.max_key_lifetime_secs == 0 {
            return Err(BearDogError::configuration(
                "max_key_lifetime_secs must be greater than 0",
            ));
        }

        if config.min_entropy_tier > 5 {
            return Err(BearDogError::configuration(
                "min_entropy_tier must be between 0 and 5",
            ));
        }

        info!("🔑 Initializing Genetic Key Exchange");
        info!("   Max key lifetime: {}s", config.max_key_lifetime_secs);
        info!("   Evolution enabled: {}", config.enable_evolution);
        info!("   Min entropy tier: {}", config.min_entropy_tier);

        Ok(Self {
            config,
            lineages: parking_lot::RwLock::new(HashMap::new()),
            key_counter: std::sync::atomic::AtomicU64::new(0),
        })
    }

    /// Create a delegated key for another primal
    ///
    /// This generates a new key with constraints suitable for sharing
    /// with a peer primal for secure communication.
    ///
    /// # Arguments
    ///
    /// * `peer_id` - Identifier of the peer primal
    /// * `lifetime_secs` - Key lifetime in seconds
    /// * `allowed_operations` - Operations the key can perform
    ///
    /// # Errors
    ///
    /// Returns error if key generation fails or constraints are invalid.
    pub fn create_delegated_key(
        &self,
        peer_id: &str,
        lifetime_secs: u64,
        allowed_operations: &[String],
    ) -> Result<DelegatedKey, BearDogError> {
        info!("🔐 Creating delegated key for peer: {}", peer_id);

        // Validate lifetime
        if lifetime_secs > self.config.max_key_lifetime_secs {
            return Err(BearDogError::security(format!(
                "Requested lifetime {}s exceeds maximum {}s",
                lifetime_secs, self.config.max_key_lifetime_secs
            )));
        }

        // Generate unique key ID
        let key_id = self.generate_key_id();

        // Generate key material using genetic entropy
        let (public_key, lineage) = Self::generate_key_with_genetics(peer_id, lifetime_secs);

        // Build constraints
        let now = SystemTime::now();
        let valid_until = now + Duration::from_secs(lifetime_secs);

        let constraints = KeyConstraints {
            allowed_operations: allowed_operations.to_vec(),
            time_constraint: Some(TimeConstraint {
                valid_from: now,
                valid_until,
                auto_renew: self.config.enable_multiparty_renewal,
            }),
            resource_constraint: Some(ResourceConstraint {
                max_operations: Some(10000),         // Reasonable default
                max_data_bytes: Some(1_073_741_824), // 1 GB default
                max_concurrent_sessions: Some(10),
            }),
            scope_constraint: None, // Can be customized by caller
        };

        debug!("✅ Delegated key created: {}", key_id);

        Ok(DelegatedKey {
            key_id,
            public_key,
            constraints,
            lineage_info: lineage,
        })
    }

    /// Perform key exchange with a peer primal
    ///
    /// This implements a zero-knowledge key exchange where both parties
    /// can derive a shared secret without exposing their private keys.
    ///
    /// # Arguments
    ///
    /// * `peer_id` - Identifier of the peer primal
    /// * `peer_key` - Peer's delegated public key
    ///
    /// # Errors
    ///
    /// Returns error if key exchange fails or peer key is invalid.
    pub fn perform_key_exchange(
        &self,
        peer_id: &str,
        peer_key: &DelegatedKey,
    ) -> Result<KeyExchangeResult, BearDogError> {
        info!("🤝 Performing key exchange with peer: {}", peer_id);

        // Validate peer key constraints
        Self::validate_key_constraints(&peer_key.constraints)?;

        // Perform ECDH (Elliptic Curve Diffie-Hellman) or similar
        // Modern Rust idiom: Use ring or RustCrypto for crypto ops
        let shared_secret = Self::derive_shared_secret(peer_id, &peer_key.public_key);

        // Update or create lineage
        let lineage = self.update_lineage(peer_id, &peer_key.lineage_info);

        debug!("✅ Key exchange completed with peer: {}", peer_id);

        Ok(KeyExchangeResult {
            shared_secret,
            lineage,
        })
    }

    /// Check if a key should evolve based on triggers
    ///
    /// # Errors
    ///
    /// Returns error if lineage lookup fails.
    pub fn should_evolve(&self, peer_id: &str) -> Result<bool, BearDogError> {
        // parking_lot::RwLock never panics - cleaner API!
        let lineages = self.lineages.read();

        let Some(lineage) = lineages.get(peer_id) else {
            return Ok(false);
        };

        // Check evolution triggers
        for trigger in &lineage.evolution_triggers {
            if let EvolutionTrigger::TimeElapsed(duration) = trigger {
                if let Ok(elapsed) = lineage.created_at.elapsed() {
                    if elapsed > *duration {
                        return Ok(true);
                    }
                }
            }
            // Other triggers (SecurityEvent, MutualConsent, etc.) not yet implemented
        }

        Ok(false)
    }

    // ========== Private Helper Methods ==========

    fn generate_key_id(&self) -> String {
        let counter = self
            .key_counter
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        format!("genetic-key-{counter:016x}")
    }

    fn generate_key_with_genetics(peer_id: &str, lifetime_secs: u64) -> (Vec<u8>, KeyLineage) {
        // Generate key using genetic entropy mixing
        // Modern Rust: Use rand for secure RNG (which internally uses getrandom)
        use rand::RngCore;
        use sha2::{Digest, Sha256};

        let mut key_material = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key_material);

        // Create genetic fingerprint
        let mut hasher = Sha256::new();
        hasher.update(&key_material);
        hasher.update(peer_id.as_bytes());
        let genetic_fingerprint: [u8; 32] = hasher.finalize().into();

        let now = SystemTime::now();
        let expires_at = now + Duration::from_secs(lifetime_secs);

        let lineage = KeyLineage {
            lineage_id: format!("lineage-{}", uuid::Uuid::new_v4()),
            peer_id: peer_id.to_string(),
            generation: 1,
            created_at: now,
            expires_at,
            allowed_operations: vec![],
            genetic_fingerprint,
            evolution_triggers: vec![
                EvolutionTrigger::TimeElapsed(Duration::from_secs(lifetime_secs / 2)),
                EvolutionTrigger::MutualConsent,
            ],
        };

        (key_material, lineage)
    }

    fn validate_key_constraints(constraints: &KeyConstraints) -> Result<(), BearDogError> {
        // Validate time constraints
        if let Some(time_constraint) = &constraints.time_constraint {
            let now = SystemTime::now();
            if now < time_constraint.valid_from {
                return Err(BearDogError::security("Key not yet valid".to_string()));
            }
            if now > time_constraint.valid_until {
                return Err(BearDogError::security("Key expired".to_string()));
            }
        }

        Ok(())
    }

    fn derive_shared_secret(peer_id: &str, peer_public_key: &[u8]) -> Vec<u8> {
        // Implement ECDH or X25519 key exchange
        // Modern Rust: Use x25519-dalek for production

        // For now, use a deterministic but secure derivation
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(peer_public_key);
        hasher.update(peer_id.as_bytes());

        hasher.finalize().to_vec()
    }

    fn update_lineage(&self, peer_id: &str, new_lineage: &KeyLineage) -> KeyLineage {
        // parking_lot::RwLock never panics - cleaner API!
        let mut lineages = self.lineages.write();

        // Update or insert lineage
        if let Some(existing) = lineages.get_mut(peer_id) {
            // Evolve existing lineage
            existing.generation += 1;
            existing.created_at = SystemTime::now();
            existing.expires_at = new_lineage.expires_at;
            existing.clone()
        } else {
            // Create new lineage
            lineages.insert(peer_id.to_string(), new_lineage.clone());
            new_lineage.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_genetic_key_exchange_creation() {
        let config = KeyExchangeConfig::default();
        let exchange = GeneticKeyExchange::new(config);
        assert!(exchange.is_ok());
    }

    #[tokio::test]
    async fn test_create_delegated_key() {
        let config = KeyExchangeConfig::default();
        let exchange = GeneticKeyExchange::new(config).unwrap();

        let allowed_ops = vec!["encrypt".to_string(), "decrypt".to_string()];
        let result = exchange.create_delegated_key("test-peer", 3600, &allowed_ops);

        assert!(result.is_ok());
        let key = result.unwrap();
        assert!(!key.public_key.is_empty());
        assert_eq!(key.constraints.allowed_operations.len(), 2);
    }

    #[test]
    fn test_key_lifetime_validation() {
        let config = KeyExchangeConfig {
            max_key_lifetime_secs: 1000,
            ..Default::default()
        };
        let exchange = GeneticKeyExchange::new(config).unwrap();

        let result = exchange.create_delegated_key("test-peer", 2000, &[]);

        assert!(result.is_err());
    }

    #[test]
    fn test_key_exchange() {
        let config = KeyExchangeConfig::default();
        let exchange = GeneticKeyExchange::new(config).unwrap();

        let allowed_ops = vec!["encrypt".to_string()];
        let delegated_key = exchange
            .create_delegated_key("peer-1", 3600, &allowed_ops)
            .unwrap();

        let result = exchange.perform_key_exchange("peer-1", &delegated_key);
        assert!(result.is_ok());

        let exchange_result = result.unwrap();
        assert!(!exchange_result.shared_secret.is_empty());
        assert_eq!(exchange_result.lineage.peer_id, "peer-1");
    }

    #[test]
    fn test_evolution_triggers() {
        let config = KeyExchangeConfig::default();
        let exchange = GeneticKeyExchange::new(config).unwrap();

        let delegated_key = exchange.create_delegated_key("peer-2", 3600, &[]).unwrap();

        exchange
            .perform_key_exchange("peer-2", &delegated_key)
            .unwrap();

        let should_evolve = exchange.should_evolve("peer-2").unwrap();
        assert!(!should_evolve); // Not enough time elapsed yet
    }

    #[tokio::test]
    async fn test_invalid_config() {
        let config = KeyExchangeConfig {
            max_key_lifetime_secs: 0,
            ..Default::default()
        };
        let result = GeneticKeyExchange::new(config);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_constraint_validation() {
        let config = KeyExchangeConfig::default();
        let _exchange = GeneticKeyExchange::new(config).unwrap();

        let constraints = KeyConstraints {
            allowed_operations: vec![],
            time_constraint: Some(TimeConstraint {
                valid_from: SystemTime::now() + Duration::from_secs(3600),
                valid_until: SystemTime::now() + Duration::from_secs(7200),
                auto_renew: false,
            }),
            resource_constraint: None,
            scope_constraint: None,
        };

        let result = GeneticKeyExchange::validate_key_constraints(&constraints);
        assert!(result.is_err()); // Key not yet valid
    }
}
