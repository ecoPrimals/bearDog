// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genesis lineage provider - establishes cryptographic lineage during physical ceremonies

use beardog_errors::BearDogError;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info, warn};

use crate::birdsong::genesis_types::{
    GenesisCeremonyResult, GenesisWitness, GeneticLineage, PhysicalChannelProof, TrustLevel,
};
use crate::birdsong::types::{LineageChain, LineageMetadata, LineageNode, LineageRelationship};
use crate::birdsong::{LineageChainManager, LineageProofManager};

/// Type alias for hardware entropy provider functions
type HardwareEntropyFn = Arc<dyn Fn() -> Result<Vec<u8>, BearDogError> + Send + Sync>;

/// Genesis lineage provider for establishing cryptographic lineage
/// during physical witness ceremonies
pub struct GenesisLineageProvider {
    _lineage_chain_mgr: Arc<LineageChainManager>,
    _lineage_proof_mgr: Arc<LineageProofManager>,
    lineage_store: Arc<RwLock<HashMap<String, GeneticLineage>>>,
    pub(crate) trusted_witnesses: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    _hardware_entropy: Option<HardwareEntropyFn>,
    /// Minimum [`TrustLevel`] a [`GenesisWitness`] must meet for lineage to be accepted.
    pub min_trust_level: TrustLevel,
    /// Witness policy: `permissioned` | `permissionless` (see `verify_witness_authority`).
    pub genesis_mode: String,
}

impl GenesisLineageProvider {
    /// Create new genesis lineage provider
    ///
    /// # Errors
    ///
    /// Returns [`beardog_errors::BearDogError`] if [`Self::with_config`] fails.
    pub async fn new() -> Result<Self, BearDogError> {
        Self::with_config(TrustLevel::Medium).await
    }

    /// Create genesis lineage provider with custom trust threshold
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future initialization failures.
    pub async fn with_config(min_trust_level: TrustLevel) -> Result<Self, BearDogError> {
        let lineage_chain_mgr = Arc::new(LineageChainManager::new());
        let lineage_proof_mgr = Arc::new(LineageProofManager::new(lineage_chain_mgr.clone()));

        Ok(Self {
            _lineage_chain_mgr: lineage_chain_mgr,
            _lineage_proof_mgr: lineage_proof_mgr,
            lineage_store: Arc::new(RwLock::new(HashMap::new())),
            trusted_witnesses: Arc::new(RwLock::new(HashMap::new())),
            _hardware_entropy: None,
            min_trust_level,
            genesis_mode: "permissioned".to_string(),
        })
    }

    /// Override genesis witness policy (default `permissioned`).
    pub fn with_genesis_mode(mut self, mode: impl Into<String>) -> Self {
        self.genesis_mode = mode.into();
        self
    }

    /// Set [`Self::genesis_mode`] from `BEARDOG_GENESIS_MODE` (default `permissioned` when unset).
    pub fn with_genesis_mode_from_env(mut self) -> Self {
        self.genesis_mode = std::env::var("BEARDOG_GENESIS_MODE")
            .unwrap_or_else(|_| "permissioned".to_string())
            .to_lowercase();
        self
    }

    /// Enable hardware entropy for production-grade genetic ID generation
    pub fn with_hardware_entropy(
        mut self,
        entropy_fn: Arc<dyn Fn() -> Result<Vec<u8>, BearDogError> + Send + Sync>,
    ) -> Self {
        info!("🔐 Hardware entropy enabled for Genesis");
        self._hardware_entropy = Some(entropy_fn);
        self
    }

    /// Establish genetic lineage for new node via genesis ceremony
    ///
    /// # Errors
    ///
    /// Returns [`beardog_errors::BearDogError`] when witness authority checks fail, the witness signature is invalid,
    /// trust is below [`Self::min_trust_level`], genetic ID or lineage construction fails, or
    /// [`GeneticLineage::verify`] reports an integrity failure.
    pub async fn establish_genesis_lineage(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<GeneticLineage, BearDogError> {
        info!(
            "Establishing genesis lineage for node: {} with witness: {}",
            new_node_id, witness.device_id
        );

        self.verify_witness_authority(witness, &self.genesis_mode)?;

        if !witness.verify_signature(new_node_id)? {
            warn!(
                "Invalid witness signature for node: {} from witness: {}",
                new_node_id, witness.device_id
            );
            return Err(BearDogError::security("Invalid witness signature".into()));
        }

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

        let genetic_id = self.generate_genetic_id(new_node_id, witness)?;
        debug!(
            "Generated genetic ID for {}: {} bytes",
            new_node_id,
            genetic_id.len()
        );

        let lineage_chain = self.create_lineage_from_witness(new_node_id, witness)?;

        let genetic_lineage = GeneticLineage {
            genetic_id,
            lineage_chain,
            genesis_witness: witness.clone(),
            birth_timestamp: witness.timestamp,
            trust_level,
        };

        if !genetic_lineage.verify()? {
            return Err(BearDogError::security(
                "Genesis lineage failed integrity check".into(),
            ));
        }

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
    ///
    /// # Errors
    ///
    /// Returns [`beardog_errors::BearDogError`] when [`PhysicalChannelProof::verify_from_env`] fails (propagates the
    /// underlying verification error). Ceremony failures from [`Self::establish_genesis_lineage`] are
    /// captured in the result’s `error` field, not returned as `Err`.
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

        if !physical_proof.verify_from_env()? {
            warn!(
                "Physical channel proof verification failed for {}",
                new_node_id
            );
            return Ok(GenesisCeremonyResult {
                genetic_lineage: Self::empty_genetic_lineage(witness, start_time),
                physical_proof: physical_proof.clone(),
                completed_at: start_time,
                success: false,
                error: Some("Physical channel proof verification failed".into()),
            });
        }

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
                    genetic_lineage: Self::empty_genetic_lineage(witness, start_time),
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
    pub fn add_trusted_witness(&self, device_id: &str, public_key: Vec<u8>) {
        self.trusted_witnesses
            .write()
            .insert(device_id.to_string(), public_key);
        debug!("Added trusted witness: {}", device_id);
    }

    /// Verify witness has authority to create lineage.
    pub(crate) fn verify_witness_authority(
        &self,
        witness: &GenesisWitness,
        genesis_mode: &str,
    ) -> Result<(), BearDogError> {
        if witness.public_key.len() != 32 {
            return Err(BearDogError::security(
                "Witness public key must be 32 bytes (Ed25519)".into(),
            ));
        }

        if witness.signature.is_empty() {
            return Err(BearDogError::security("Witness signature missing".into()));
        }

        match genesis_mode.to_lowercase().as_str() {
            "permissionless" => {
                debug!("Genesis mode: permissionless (any valid witness accepted)");
                Ok(())
            }
            "permissioned" => {
                debug!("Genesis mode: permissioned (checking trusted witnesses)");
                let witnesses = self.trusted_witnesses.read();

                if let Some(trusted_key) = witnesses.get(&witness.device_id) {
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
                } else if witnesses.is_empty() {
                    warn!("No trusted witnesses configured - falling back to permissionless mode");
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
            _ => {
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

    /// Wrapper for `verify_witness_authority` using `BEARDOG_GENESIS_MODE`
    /// (default `permissioned` when unset).
    ///
    /// # Errors
    ///
    /// Same as `verify_witness_authority`: invalid witness keys, missing signature, or
    /// permissioned-mode witness not trusted.
    pub fn verify_witness_authority_from_env(
        &self,
        witness: &GenesisWitness,
    ) -> Result<(), BearDogError> {
        let mode = std::env::var("BEARDOG_GENESIS_MODE")
            .unwrap_or_else(|_| "permissioned".to_string())
            .to_lowercase();
        self.verify_witness_authority(witness, &mode)
    }

    /// Generate genetic identity for new node
    pub(crate) fn generate_genetic_id(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<Vec<u8>, BearDogError> {
        use hkdf::Hkdf;
        use sha2::Sha256;

        let mut ikm = witness.public_key.clone();
        ikm.extend_from_slice(&witness.timestamp.to_be_bytes());

        if let Some(ref entropy_fn) = self._hardware_entropy {
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
                }
            }
        }

        let salt = new_node_id.as_bytes();
        let info = b"beardog-genesis-v1";

        let hk = Hkdf::<Sha256>::new(Some(salt), &ikm);
        let mut genetic_id = vec![0u8; 32];
        hk.expand(info, &mut genetic_id)
            .map_err(|e| BearDogError::security(format!("HKDF expand failed: {e}")))?;

        Ok(genetic_id)
    }

    /// Create lineage chain from witness to new node
    pub(crate) fn create_lineage_from_witness(
        &self,
        new_node_id: &str,
        witness: &GenesisWitness,
    ) -> Result<LineageChain, BearDogError> {
        use chrono::Utc;
        use std::collections::HashMap;
        use uuid::Uuid;

        let now = Utc::now();

        let witness_node = LineageNode {
            node_id: witness.device_id.clone(),
            parent_id: None,
            public_key: witness.public_key.clone(),
            depth: 0,
            created_at: now,
            metadata: LineageMetadata {
                biome_type: Some("witness".to_string()),
                capabilities: vec!["genesis".to_string()],
                trust_level: f64::from(witness.physical_channel.trust_level() as u8),
                custom: HashMap::new(),
            },
        };

        let new_node = LineageNode {
            node_id: new_node_id.to_string(),
            parent_id: Some(witness.device_id.clone()),
            public_key: vec![],
            depth: 1,
            created_at: now,
            metadata: LineageMetadata {
                biome_type: Some("new-node".to_string()),
                capabilities: vec![],
                trust_level: f64::from(witness.physical_channel.trust_level() as u8),
                custom: HashMap::new(),
            },
        };

        let relationship = LineageRelationship {
            parent_id: witness.device_id.clone(),
            child_id: new_node_id.to_string(),
            parent_signature: witness.signature.clone(),
            witness_signatures: vec![],
            established_at: now,
        };

        let mut nodes = HashMap::new();
        nodes.insert(witness.device_id.clone(), witness_node.clone());
        nodes.insert(new_node_id.to_string(), new_node);

        let chain_id = Uuid::new_v4().to_string();
        let genesis_commitment = blake3::hash(chain_id.as_bytes()).as_bytes().to_vec();
        let lineage_chain = LineageChain {
            chain_id,
            root_node: witness_node,
            nodes,
            relationships: vec![relationship],
            generation: 1,
            head_commitment: genesis_commitment,
            created_at: now,
        };

        Ok(lineage_chain)
    }

    fn empty_genetic_lineage(witness: &GenesisWitness, birth_timestamp: u64) -> GeneticLineage {
        GeneticLineage {
            genetic_id: vec![],
            lineage_chain: LineageChain {
                chain_id: String::new(),
                root_node: LineageNode {
                    node_id: String::new(),
                    parent_id: None,
                    public_key: vec![],
                    depth: 0,
                    created_at: chrono::Utc::now(),
                    metadata: LineageMetadata {
                        biome_type: None,
                        capabilities: vec![],
                        trust_level: 0.0,
                        custom: std::collections::HashMap::new(),
                    },
                },
                nodes: std::collections::HashMap::new(),
                relationships: vec![],
                generation: 0,
                head_commitment: vec![],
                created_at: chrono::Utc::now(),
            },
            genesis_witness: witness.clone(),
            birth_timestamp,
            trust_level: TrustLevel::Low,
        }
    }
}
