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
    lineage_chain_mgr: Arc<LineageChainManager>,

    /// Lineage proof manager for verification
    lineage_proof_mgr: Arc<LineageProofManager>,

    /// Store of established genetic lineages (node_id -> lineage)
    lineage_store: Arc<RwLock<HashMap<String, GeneticLineage>>>,

    /// Trusted witnesses (device_id -> public_key)
    /// TODO: Integrate with HSM for production
    trusted_witnesses: Arc<RwLock<HashMap<String, Vec<u8>>>>,

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
            min_trust_level,
        })
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
            .unwrap()
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
                        .unwrap()
                        .as_secs()
                        - start_time)
                        * 1000
                );

                Ok(GenesisCeremonyResult {
                    genetic_lineage,
                    physical_proof: physical_proof.clone(),
                    completed_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
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
                        .unwrap()
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
    /// TODO: In production, this should be managed via HSM
    pub fn add_trusted_witness(&self, device_id: &str, public_key: Vec<u8>) {
        self.trusted_witnesses
            .write()
            .insert(device_id.to_string(), public_key);
        debug!("Added trusted witness: {}", device_id);
    }

    /// Verify witness has authority to create lineage
    fn verify_witness_authority(&self, witness: &GenesisWitness) -> Result<(), BearDogError> {
        // For now, allow any witness (permissionless genesis)
        // TODO: In production, check against HSM-backed trusted witness list

        // Check witness has valid public key
        if witness.public_key.len() != 32 {
            return Err(BearDogError::security(
                "Witness public key must be 32 bytes (Ed25519)".into(),
            ));
        }

        // Check signature is present
        if witness.signature.is_empty() {
            return Err(BearDogError::security("Witness signature missing".into()));
        }

        Ok(())
    }

    /// Generate genetic identity for new node
    ///
    /// Uses HKDF to derive genetic ID from:
    /// - Witness public key (IKM)
    /// - New node ID (salt)
    /// - Genesis context (info)
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
}
