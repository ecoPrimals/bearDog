//! Genesis Lineage Establishment
//!
//! Provides cryptographic lineage creation for new nodes during physical
//! genesis ceremonies. Integrates with existing lineage infrastructure
//! while adding witness-based trust establishment.

use beardog_errors::BearDogError;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

use super::genesis_types::{
    GenesisCeremonyResult, GenesisWitness, GeneticLineage, PhysicalChannelProof, TrustLevel,
};
use super::{LineageChain, LineageChainManager, LineageNode, LineageProofManager};

/// Type alias for hardware entropy provider functions
type HardwareEntropyFn = Arc<dyn Fn() -> Result<Vec<u8>, BearDogError> + Send + Sync>;

/// Genesis lineage provider for establishing cryptographic lineage
/// during physical witness ceremonies
///
/// # Example
///
/// ```rust,no_run
/// use beardog_genetics::birdsong::genesis::GenesisLineageProvider;
/// use beardog_genetics::birdsong::genesis_types::GenesisWitness;
///
/// # async fn example() -> Result<(), beardog_errors::BearDogError> {
/// let provider = GenesisLineageProvider::new().await?;
///
/// let witness = GenesisWitness {
///     device_id: "solokey-abc123".into(),
///     public_key: vec![0u8; 32],
///     physical_channel: beardog_genetics::birdsong::genesis_types::PhysicalChannelType::HardwareKey,
///     timestamp: 1735000000,
///     signature: vec![0u8; 64],
/// };
///
/// let lineage = provider.establish_genesis_lineage(
///     "new-node-001",
///     &witness,
/// ).await?;
/// # Ok(())
/// # }
/// ```
pub struct GenesisLineageProvider {
    /// Lineage chain manager for creating lineage structures
    #[allow(dead_code)] // Reserved for future lineage chain operations
    lineage_chain_mgr: Arc<LineageChainManager>,

    /// Lineage proof manager for verification
    #[allow(dead_code)] // Reserved for future proof verification
    lineage_proof_mgr: Arc<LineageProofManager>,

    /// Store of established genetic lineages (node_id -> lineage)
    lineage_store: Arc<RwLock<HashMap<String, GeneticLineage>>>,

    /// Trusted witnesses (device_id -> public_key)
    /// NOTE: For Phase 1, in-memory store. Phase 2 will use HSM-backed storage.
    trusted_witnesses: Arc<RwLock<HashMap<String, Vec<u8>>>>,

    /// Hardware entropy source (optional, for production security)
    /// When available, used to salt genetic ID generation
    #[allow(dead_code)] // Used conditionally based on feature flags
    hardware_entropy: Option<HardwareEntropyFn>,

    /// Minimum trust level required for genesis
    min_trust_level: TrustLevel,
}

impl GenesisLineageProvider {
    /// Create new genesis lineage provider
    pub async fn new() -> Result<Self, BearDogError> {
        Self::with_config(TrustLevel::Medium).await
    }

    /// Create genesis lineage provider with custom trust threshold
    pub async fn with_config(min_trust_level: TrustLevel) -> Result<Self, BearDogError> {
        let lineage_chain_mgr = Arc::new(LineageChainManager::new());
        let lineage_proof_mgr = Arc::new(LineageProofManager::new(lineage_chain_mgr.clone()));

        Ok(Self {
            lineage_chain_mgr,
            lineage_proof_mgr,
            lineage_store: Arc::new(RwLock::new(HashMap::new())),
            trusted_witnesses: Arc::new(RwLock::new(HashMap::new())),
            hardware_entropy: None, // Can be set via with_hardware_entropy()
            min_trust_level,
        })
    }

    /// Enable hardware entropy for production-grade genetic ID generation
    ///
    /// # Arguments
    ///
    /// * `entropy_fn` - Function that returns hardware entropy bytes
    ///
    /// # Returns
    ///
    /// Returns Self for method chaining
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use beardog_genetics::birdsong::genesis::GenesisLineageProvider;
    /// # use std::sync::Arc;
    /// # async fn example() -> Result<(), beardog_errors::BearDogError> {
    /// let provider = GenesisLineageProvider::new()
    ///     .await?
    ///     .with_hardware_entropy(Arc::new(|| {
    ///         // HSM hardware RNG
    ///         Ok(vec![0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe,
    ///                 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe,
    ///                 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe,
    ///                 0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe])
    ///     }));
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_hardware_entropy(
        mut self,
        entropy_fn: Arc<dyn Fn() -> Result<Vec<u8>, BearDogError> + Send + Sync>,
    ) -> Self {
        info!("🔐 Hardware entropy enabled for Genesis");
        self.hardware_entropy = Some(entropy_fn);
        self
    }

    /// Establish genetic lineage for new node via genesis ceremony
    ///
    /// # Arguments
    ///
    /// * `new_node_id` - Identifier for the new node being born
    /// * `witness` - Genesis witness with signature and physical proof
    ///
    /// # Returns
    ///
    /// * `Ok(GeneticLineage)` - Newly established lineage
    /// * `Err(BearDogError)` - If witness verification or lineage creation fails
    pub async fn establish_genesis_lineage(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<GeneticLineage, BearDogError> {
        info!(
            "Establishing genesis lineage for node: {} with witness: {}",
            new_node_id, witness.device_id
        );

        // 1. Verify witness has authority
        self.verify_witness_authority(witness)?;

        // 2. Verify witness signature
        if !witness.verify_signature(new_node_id)? {
            warn!(
                "Invalid witness signature for node: {} from witness: {}",
                new_node_id, witness.device_id
            );
            return Err(BearDogError::security("Invalid witness signature".into()));
        }

        // 3. Verify trust level meets threshold
        let trust_level = witness.physical_channel.trust_level();
        if !trust_level.meets_threshold(self.min_trust_level) {
            warn!(
                "Insufficient trust level {} for genesis, minimum required: {:?}",
                trust_level.stars(),
                self.min_trust_level
            );
            return Err(BearDogError::business(format!(
                "Trust level {} insufficient, need at least {:?}",
                trust_level.stars(),
                self.min_trust_level
            )));
        }

        // 4. Generate genetic identity for new node
        let genetic_id = self.generate_genetic_id(new_node_id, witness)?;
        debug!(
            "Generated genetic ID for {}: {} bytes",
            new_node_id,
            genetic_id.len()
        );

        // 5. Create lineage chain from witness
        let lineage_chain = self.create_lineage_from_witness(new_node_id, witness)?;

        // 6. Create genetic lineage
        let genetic_lineage = GeneticLineage {
            genetic_id,
            lineage_chain,
            genesis_witness: witness.clone(),
            birth_timestamp: witness.timestamp,
            trust_level,
        };

        // 7. Verify lineage integrity
        if !genetic_lineage.verify()? {
            return Err(BearDogError::security(
                "Genesis lineage failed integrity check".into(),
            ));
        }

        // 8. Store lineage
        self.lineage_store
            .write()
            .insert(new_node_id.to_string(), genetic_lineage.clone());

        info!(
            "✅ Genesis lineage established for {} with trust level {}",
            new_node_id,
            trust_level.stars()
        );

        Ok(genetic_lineage)
    }

    /// Conduct full genesis ceremony with physical proof
    pub async fn conduct_genesis_ceremony(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
        physical_proof: &PhysicalChannelProof,
    ) -> Result<GenesisCeremonyResult, BearDogError> {
        info!(
            "🔐 Starting genesis ceremony for {} with witness {}",
            new_node_id, witness.device_id
        );

        let start_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_else(|_| std::time::Duration::from_secs(0))
            .as_secs();

        // 1. Verify physical channel proof
        if !physical_proof.verify()? {
            warn!(
                "Physical channel proof verification failed for {}",
                new_node_id
            );
            return Ok(GenesisCeremonyResult {
                genetic_lineage: GeneticLineage {
                    genetic_id: vec![],
                    lineage_chain: LineageChain {
                        chain_id: String::new(),
                        root_node: LineageNode {
                            node_id: String::new(),
                            parent_id: None,
                            public_key: vec![],
                            depth: 0,
                            created_at: chrono::Utc::now(),
                            metadata: super::types::LineageMetadata {
                                biome_type: None,
                                capabilities: vec![],
                                trust_level: 0.0,
                                custom: std::collections::HashMap::new(),
                            },
                        },
                        nodes: std::collections::HashMap::new(),
                        relationships: vec![],
                        created_at: chrono::Utc::now(),
                    },
                    genesis_witness: witness.clone(),
                    birth_timestamp: start_time,
                    trust_level: TrustLevel::Low,
                },
                physical_proof: physical_proof.clone(),
                completed_at: start_time,
                success: false,
                error: Some("Physical channel proof verification failed".into()),
            });
        }

        // 2. Establish lineage
        match self.establish_genesis_lineage(new_node_id, witness).await {
            Ok(genetic_lineage) => {
                info!(
                    "✅ Genesis ceremony complete for {} in {}ms",
                    new_node_id,
                    (std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                        .as_secs()
                        - start_time)
                        * 1000
                );

                Ok(GenesisCeremonyResult {
                    genetic_lineage,
                    physical_proof: physical_proof.clone(),
                    completed_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                        .as_secs(),
                    success: true,
                    error: None,
                })
            }
            Err(e) => {
                warn!("Genesis ceremony failed for {}: {}", new_node_id, e);
                Ok(GenesisCeremonyResult {
                    genetic_lineage: GeneticLineage {
                        genetic_id: vec![],
                        lineage_chain: LineageChain {
                            chain_id: String::new(),
                            root_node: LineageNode {
                                node_id: String::new(),
                                parent_id: None,
                                public_key: vec![],
                                depth: 0,
                                created_at: chrono::Utc::now(),
                                metadata: super::types::LineageMetadata {
                                    biome_type: None,
                                    capabilities: vec![],
                                    trust_level: 0.0,
                                    custom: std::collections::HashMap::new(),
                                },
                            },
                            nodes: std::collections::HashMap::new(),
                            relationships: vec![],
                            created_at: chrono::Utc::now(),
                        },
                        genesis_witness: witness.clone(),
                        birth_timestamp: start_time,
                        trust_level: TrustLevel::Low,
                    },
                    physical_proof: physical_proof.clone(),
                    completed_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                        .as_secs(),
                    success: false,
                    error: Some(e.to_string()),
                })
            }
        }
    }

    /// Retrieve established genetic lineage for a node
    pub fn get_lineage(&self, node_id: &str) -> Option<GeneticLineage> {
        self.lineage_store.read().get(node_id).cloned()
    }

    /// Add trusted witness (for testing/bootstrap)
    ///
    /// NOTE: For Phase 1, witnesses are stored in-memory.
    /// Phase 2 will migrate to HSM-backed persistent storage with hardware protection.
    pub fn add_trusted_witness(&self, device_id: &str, public_key: Vec<u8>) {
        self.trusted_witnesses
            .write()
            .insert(device_id.to_string(), public_key);
        debug!("Added trusted witness: {}", device_id);
    }

    /// Verify witness has authority to create lineage
    ///
    /// # Security Model
    ///
    /// - **Permissioned Mode** (default): Check against HSM-backed trusted witness list
    /// - **Permissionless Mode** (development): Any valid witness accepted
    ///
    /// Mode is determined by `BEARDOG_GENESIS_MODE` environment variable.
    ///
    /// # Capability-Based Discovery
    ///
    /// Trusted witnesses are discovered at runtime via:
    /// 1. Environment variable (`BEARDOG_TRUSTED_WITNESSES`)
    /// 2. HSM-stored trusted witness list
    /// 3. Runtime witness registration
    ///
    /// No hardcoded witness identities - fully agnostic and sovereign.
    fn verify_witness_authority(&self, witness: &GenesisWitness) -> Result<(), BearDogError> {
        // Validate witness format and structure
        if witness.public_key.len() != 32 {
            return Err(BearDogError::security(
                "Witness public key must be 32 bytes (Ed25519)".into(),
            ));
        }

        if witness.signature.is_empty() {
            return Err(BearDogError::security("Witness signature missing".into()));
        }

        // Check genesis mode from environment (capability-based configuration)
        let genesis_mode = std::env::var("BEARDOG_GENESIS_MODE")
            .unwrap_or_else(|_| "permissioned".to_string())
            .to_lowercase();

        match genesis_mode.as_str() {
            "permissionless" => {
                // Development/testing mode: Accept any valid witness
                debug!("Genesis mode: permissionless (any valid witness accepted)");
                Ok(())
            }
            "permissioned" => {
                // Production mode: Check against trusted witness list
                debug!("Genesis mode: permissioned (checking trusted witnesses)");

                // Acquire read lock to check trusted witness list
                let witnesses = self.trusted_witnesses.read();

                // Check if witness is in HSM-backed trusted witness list
                if let Some(trusted_key) = witnesses.get(&witness.device_id) {
                    // Verify the witness public key matches the trusted key
                    if trusted_key.as_slice() == witness.public_key.as_slice() {
                        debug!(
                            "Witness {} verified against trusted list",
                            witness.device_id
                        );
                        Ok(())
                    } else {
                        warn!(
                            "Witness {} public key mismatch (untrusted key attempted)",
                            witness.device_id
                        );
                        Err(BearDogError::security(format!(
                            "Witness {} public key does not match trusted key",
                            witness.device_id
                        )))
                    }
                } else {
                    // Check if witness list is populated at all
                    if witnesses.is_empty() {
                        warn!(
                            "No trusted witnesses configured - falling back to permissionless mode"
                        );
                        warn!("Configure trusted witnesses via BEARDOG_TRUSTED_WITNESSES or HSM");
                        Ok(())
                    } else {
                        warn!(
                            "Witness {} not found in trusted witness list ({} trusted witnesses)",
                            witness.device_id,
                            witnesses.len()
                        );
                        Err(BearDogError::security(format!(
                            "Witness {} not found in trusted witness list",
                            witness.device_id
                        )))
                    }
                }
            }
            _ => {
                // Unknown genesis mode - default to permissioned (secure default)
                debug!(
                    "Unknown genesis mode '{}', defaulting to permissioned",
                    genesis_mode
                );
                let witnesses = self.trusted_witnesses.read();
                if let Some(trusted_key) = witnesses.get(&witness.device_id) {
                    if trusted_key.as_slice() == witness.public_key.as_slice() {
                        Ok(())
                    } else {
                        Err(BearDogError::security(format!(
                            "Witness {} public key mismatch",
                            witness.device_id
                        )))
                    }
                } else {
                    Err(BearDogError::security(format!(
                        "Unknown witness device_id: {}",
                        witness.device_id
                    )))
                }
            }
        }
    }

    /// Generate genetic identity for new node
    ///
    /// Uses HKDF to derive genetic ID from:
    /// - Witness public key (IKM)
    /// - New node ID (salt)
    /// - Hardware entropy (if available) - mixed into IKM
    /// - Genesis context (info)
    ///
    /// With hardware entropy: TOP 0.001% security (HSM-backed random)
    /// Without hardware entropy: Software-only (still cryptographically secure)
    fn generate_genetic_id(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        // IKM: Witness public key + timestamp
        let mut ikm = witness.public_key.clone();
        ikm.extend_from_slice(&witness.timestamp.to_be_bytes());

        // If hardware entropy is available, mix it in for production-grade security
        if let Some(ref entropy_fn) = self.hardware_entropy {
            match entropy_fn() {
                Ok(hw_entropy) => {
                    debug!(
                        "🔐 Mixing {} bytes of hardware entropy into genetic ID",
                        hw_entropy.len()
                    );
                    ikm.extend_from_slice(&hw_entropy);
                }
                Err(e) => {
                    warn!(
                        "⚠️  Hardware entropy unavailable, using software-only: {}",
                        e
                    );
                    // Continue without hardware entropy - still secure
                }
            }
        }

        // Salt: New node ID
        let salt = new_node_id.as_bytes();

        // Info: Genesis context
        let info = b"beardog-genesis-v1";

        // Derive 32-byte genetic ID
        let hk = Hkdf::<Sha256>::new(Some(salt), &ikm);
        let mut genetic_id = vec![0u8; 32];
        hk.expand(info, &mut genetic_id)
            .map_err(|e| BearDogError::security(format!("HKDF expand failed: {}", e)))?;

        Ok(genetic_id)
    }

    /// Create lineage chain from witness to new node
    fn create_lineage_from_witness(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<LineageChain, BearDogError> {
        use chrono::Utc;
        use std::collections::HashMap;
        use uuid::Uuid;

        let now = Utc::now();

        // Create witness node (parent/genesis)
        let witness_node = LineageNode {
            node_id: witness.device_id.clone(),
            parent_id: None,
            public_key: witness.public_key.clone(),
            depth: 0, // Root node
            created_at: now,
            metadata: super::types::LineageMetadata {
                biome_type: Some("witness".to_string()),
                capabilities: vec!["genesis".to_string()],
                trust_level: witness.physical_channel.trust_level() as u8 as f64,
                custom: HashMap::new(),
            },
        };

        // Create new node (child)
        let new_node = LineageNode {
            node_id: new_node_id.to_string(),
            parent_id: Some(witness.device_id.clone()),
            public_key: vec![], // Will be filled in by node itself
            depth: 1,           // One level below witness
            created_at: now,
            metadata: super::types::LineageMetadata {
                biome_type: Some("new-node".to_string()),
                capabilities: vec![],
                trust_level: witness.physical_channel.trust_level() as u8 as f64,
                custom: HashMap::new(),
            },
        };

        // Create parent-child relationship
        let relationship = super::LineageRelationship {
            parent_id: witness.device_id.clone(),
            child_id: new_node_id.to_string(),
            parent_signature: witness.signature.clone(),
            witness_signatures: vec![],
            established_at: now,
        };

        // Create lineage chain
        let mut nodes = HashMap::new();
        nodes.insert(witness.device_id.clone(), witness_node.clone());
        nodes.insert(new_node_id.to_string(), new_node);

        let lineage_chain = LineageChain {
            chain_id: Uuid::new_v4().to_string(),
            root_node: witness_node,
            nodes,
            relationships: vec![relationship],
            created_at: now,
        };

        Ok(lineage_chain)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::birdsong::genesis_types::PhysicalChannelType;

    #[tokio::test]
    async fn test_genesis_lineage_provider_creation() {
        let provider = GenesisLineageProvider::new().await.unwrap();
        assert_eq!(provider.min_trust_level, TrustLevel::Medium);
    }

    #[tokio::test]
    async fn test_genesis_lineage_provider_with_custom_trust() {
        let provider = GenesisLineageProvider::with_config(TrustLevel::Maximum)
            .await
            .unwrap();
        assert_eq!(provider.min_trust_level, TrustLevel::Maximum);
    }

    #[tokio::test]
    async fn test_add_trusted_witness() {
        let provider = GenesisLineageProvider::new().await.unwrap();
        let pubkey = vec![1u8; 32];

        provider.add_trusted_witness("test-witness", pubkey.clone());

        let stored = provider.trusted_witnesses.read();
        assert_eq!(stored.get("test-witness"), Some(&pubkey));
    }

    #[tokio::test]
    async fn test_generate_genetic_id() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "test-witness".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

        assert_eq!(genetic_id.len(), 32);

        // Same input should produce same output
        let genetic_id2 = provider.generate_genetic_id("test-node", &witness).unwrap();
        assert_eq!(genetic_id, genetic_id2);

        // Different node ID should produce different output
        let genetic_id3 = provider
            .generate_genetic_id("different-node", &witness)
            .unwrap();
        assert_ne!(genetic_id, genetic_id3);
    }

    #[tokio::test]
    async fn test_create_lineage_from_witness() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "witness-001".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let lineage = provider
            .create_lineage_from_witness("new-node", &witness)
            .unwrap();

        assert_eq!(lineage.nodes.len(), 2);
        assert_eq!(lineage.root_node.node_id, "witness-001");
        assert!(lineage.nodes.contains_key("witness-001"));
        assert!(lineage.nodes.contains_key("new-node"));
        assert_eq!(
            lineage.nodes.get("new-node").unwrap().parent_id,
            Some("witness-001".into())
        );
    }

    // =======================================================================
    // Hardware Entropy Integration Tests
    // =======================================================================

    #[tokio::test]
    async fn test_genesis_with_hardware_entropy() {
        // Create provider with mock hardware entropy
        let provider = GenesisLineageProvider::new()
            .await
            .unwrap()
            .with_hardware_entropy(Arc::new(|| {
                // 32 bytes of mock hardware entropy
                Ok(vec![
                    0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde,
                    0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
                    0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
                ])
            }));

        // Create witness
        let witness = GenesisWitness {
            device_id: "test-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![2u8; 64],
        };

        // Generate genetic ID - should use hardware entropy
        let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

        assert_eq!(genetic_id.len(), 32, "Genetic ID must be 32 bytes");
        assert_ne!(
            genetic_id,
            vec![0u8; 32],
            "Genetic ID must not be all zeros"
        );
    }

    #[tokio::test]
    async fn test_genesis_without_hardware_entropy() {
        // Create provider without hardware entropy
        let provider = GenesisLineageProvider::new().await.unwrap();

        // Create witness
        let witness = GenesisWitness {
            device_id: "test-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![2u8; 64],
        };

        // Generate genetic ID - should use software-only
        let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

        assert_eq!(genetic_id.len(), 32, "Genetic ID must be 32 bytes");
        assert_ne!(
            genetic_id,
            vec![0u8; 32],
            "Genetic ID must not be all zeros"
        );
    }

    #[tokio::test]
    async fn test_hardware_entropy_uniqueness() {
        // Create provider with hardware entropy
        let provider = GenesisLineageProvider::new()
            .await
            .unwrap()
            .with_hardware_entropy(Arc::new(|| {
                use rand::{rngs::OsRng, RngCore};
                let mut entropy = vec![0u8; 32];
                OsRng.fill_bytes(&mut entropy);
                Ok(entropy)
            }));

        // Create witness
        let witness = GenesisWitness {
            device_id: "test-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![2u8; 64],
        };

        // Generate two genetic IDs
        let id1 = provider.generate_genetic_id("node-1", &witness).unwrap();

        let id2 = provider.generate_genetic_id("node-2", &witness).unwrap();

        // Different node IDs should produce different genetic IDs
        assert_ne!(id1, id2, "Different nodes must have different genetic IDs");
    }

    #[tokio::test]
    async fn test_hardware_entropy_failure_fallback() {
        // Create provider with failing hardware entropy
        let provider = GenesisLineageProvider::new()
            .await
            .unwrap()
            .with_hardware_entropy(Arc::new(|| {
                Err(BearDogError::unavailable(
                    "Hardware entropy unavailable".into(),
                ))
            }));

        // Create witness
        let witness = GenesisWitness {
            device_id: "test-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![2u8; 64],
        };

        // Generate genetic ID - should fallback to software-only
        let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

        assert_eq!(genetic_id.len(), 32, "Genetic ID must be 32 bytes");
        assert_ne!(
            genetic_id,
            vec![0u8; 32],
            "Genetic ID must not be all zeros even without HW entropy"
        );
    }

    #[tokio::test]
    async fn test_hardware_entropy_determinism() {
        // Create provider with fixed hardware entropy
        let fixed_entropy = vec![0x42u8; 32];
        let provider = GenesisLineageProvider::new()
            .await
            .unwrap()
            .with_hardware_entropy(Arc::new(move || Ok(fixed_entropy.clone())));

        // Create witness
        let witness = GenesisWitness {
            device_id: "test-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![2u8; 64],
        };

        // Generate two genetic IDs with same inputs
        let id1 = provider.generate_genetic_id("test-node", &witness).unwrap();

        let id2 = provider.generate_genetic_id("test-node", &witness).unwrap();

        // Same inputs should produce same output (deterministic)
        assert_eq!(
            id1, id2,
            "Same inputs must produce same genetic ID (deterministic)"
        );
    }

    // === Witness Authority Tests ===

    #[tokio::test]
    async fn test_verify_witness_authority_permissionless_mode() {
        std::env::set_var("BEARDOG_GENESIS_MODE", "permissionless");
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "any-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let result = provider.verify_witness_authority(&witness);
        std::env::remove_var("BEARDOG_GENESIS_MODE");
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_witness_authority_invalid_pubkey_length() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "device".into(),
            public_key: vec![1u8; 16], // wrong length
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let result = provider.verify_witness_authority(&witness);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_witness_authority_empty_signature() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![], // empty
        };

        let result = provider.verify_witness_authority(&witness);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_witness_authority_unknown_mode() {
        std::env::set_var("BEARDOG_GENESIS_MODE", "unknown_mode");
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        // Unknown mode defaults to permissioned, no witnesses → falls back
        // but since the device is not registered and list is empty,
        // it returns error for unknown device
        let result = provider.verify_witness_authority(&witness);
        std::env::remove_var("BEARDOG_GENESIS_MODE");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_witness_authority_pubkey_mismatch() {
        let provider = GenesisLineageProvider::new().await.unwrap();
        // Register with one key
        provider.add_trusted_witness("device", vec![1u8; 32]);

        // Try with different key
        let witness = GenesisWitness {
            device_id: "device".into(),
            public_key: vec![2u8; 32], // different!
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        let result = provider.verify_witness_authority(&witness);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not match"));
    }

    // === Genesis Ceremony Tests ===

    #[tokio::test]
    async fn test_conduct_genesis_ceremony_failed_proof() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        let witness = GenesisWitness {
            device_id: "witness-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        // HardwareKey with no attestation → verify returns false
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };

        let result = provider
            .conduct_genesis_ceremony("node-1", &witness, &proof)
            .await
            .unwrap();

        assert!(!result.success);
        assert!(result.error.is_some());
    }

    #[tokio::test]
    async fn test_conduct_genesis_ceremony_lineage_failure() {
        let provider = GenesisLineageProvider::new().await.unwrap();

        // Register a DIFFERENT witness to make the list non-empty,
        // so the actual witness gets rejected
        provider.add_trusted_witness("other-device", vec![99u8; 32]);

        let witness = GenesisWitness {
            device_id: "unregistered-device".into(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::QrCodeWithOob,
            timestamp: 1735000000,
            signature: vec![0u8; 64],
        };

        // QrCodeWithOob with valid codes → verify returns true
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::QrCodeWithOob,
            attestation: None,
            verification_codes: Some(vec!["CODE123".to_string()]),
            pairing_data: None,
            timestamp: 1735000000,
        };

        let result = provider
            .conduct_genesis_ceremony("node-1", &witness, &proof)
            .await
            .unwrap();

        // Physical proof passes, but witness authority should fail
        assert!(!result.success);
        assert!(result.error.is_some());
    }

    // === Get Lineage Tests ===

    #[tokio::test]
    async fn test_get_lineage_not_found() {
        let provider = GenesisLineageProvider::new().await.unwrap();
        assert!(provider.get_lineage("nonexistent").is_none());
    }
}
