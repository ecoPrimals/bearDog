// SPDX-License-Identifier: AGPL-3.0-or-later

//! High-level `BirdSong` manager integrating all components

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;
use tracing::{info, warn};

use beardog_errors::BearDogError;

use super::encryption::BirdSongEncryption;
use super::key_derivation::LineageKeyDerivation;
use super::lineage_chain::LineageChainManager;
use super::lineage_proof::LineageProofManager;
use super::types::{
    BirdSongBroadcast, BirdSongConfig, BirdSongDecryptRequest, BirdSongEncryptRequest, BirdSongKey,
    LineageChain, LineageHint, LineageMetadata, LineageNode, LineageProof,
    LineageVerificationResult,
};

/// High-level `BirdSong` manager
///
/// Provides a unified API for:
/// - Lineage management (creating parent-child relationships)
/// - Lineage proofs (verifying ancestry)
/// - Key derivation (deriving keys from lineage)
/// - Broadcast encryption (encrypting for specific lineages)
/// - Key distribution (sharing keys with descendants)
pub struct BirdSongManager {
    #[expect(
        dead_code,
        reason = "BirdSongConfig retained for future manager options"
    )]
    config: BirdSongConfig,
    chain_manager: Arc<LineageChainManager>,
    proof_manager: Arc<LineageProofManager>,
    kdf: Arc<LineageKeyDerivation>,
    encryption: Arc<BirdSongEncryption>,
    /// Distributed keys (`node_id` -> `Vec` of `BirdSongKey`)
    distributed_keys: Arc<RwLock<HashMap<String, Vec<BirdSongKey>>>>,
}

impl BirdSongManager {
    /// Create a new `BirdSong` manager
    ///
    /// # Arguments
    ///
    /// * `master_secret` - Master secret for key derivation (from HSM in production)
    /// * `config` - Optional configuration (uses defaults if None)
    ///
    /// # Errors
    ///
    /// Returns error if initialization fails
    pub async fn new(
        master_secret: Vec<u8>,
        config: Option<BirdSongConfig>,
    ) -> Result<Self, BearDogError> {
        info!("🐻🎵 Initializing BirdSongManager");

        let config = config.unwrap_or_default();

        // Initialize components
        let chain_manager = Arc::new(LineageChainManager::new());
        let proof_manager = Arc::new(LineageProofManager::new(chain_manager.clone()));
        let kdf = Arc::new(LineageKeyDerivation::new(master_secret)?);
        let encryption = Arc::new(BirdSongEncryption::new(kdf.clone()));

        info!("✅ BirdSongManager initialized");

        Ok(Self {
            config,
            chain_manager,
            proof_manager,
            kdf,
            encryption,
            distributed_keys: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    // =============================================================================
    // Lineage Management
    // =============================================================================

    /// Generate a new root lineage
    ///
    /// # Arguments
    ///
    /// * `root_node_id` - ID for the root node
    /// * `metadata` - Optional metadata
    ///
    /// # Errors
    ///
    /// Returns error if generation fails
    pub async fn generate_root_lineage(
        &self,
        root_node_id: String,
        metadata: Option<LineageMetadata>,
    ) -> Result<LineageChain, BearDogError> {
        self.chain_manager
            .generate_root_chain(root_node_id, metadata)
            .await
    }

    /// Add a child to an existing lineage
    ///
    /// # Arguments
    ///
    /// * `chain_id` - Lineage chain ID
    /// * `parent_id` - Parent node ID
    /// * `child_id` - New child node ID
    /// * `metadata` - Optional metadata
    ///
    /// # Errors
    ///
    /// Returns error if adding child fails
    pub async fn add_child(
        &self,
        chain_id: &str,
        parent_id: &str,
        child_id: String,
        metadata: Option<LineageMetadata>,
    ) -> Result<LineageNode, BearDogError> {
        self.chain_manager
            .add_child(chain_id, parent_id, child_id, metadata)
            .await
    }

    /// Get a lineage chain
    pub fn get_lineage_chain(&self, chain_id: &str) -> Option<LineageChain> {
        self.chain_manager.get_chain(chain_id)
    }

    /// Get all descendants of a node
    pub fn get_descendants(&self, chain_id: &str, node_id: &str) -> Vec<LineageNode> {
        self.chain_manager.get_descendants(chain_id, node_id)
    }

    // =============================================================================
    // Lineage Proofs
    // =============================================================================

    /// Generate a lineage proof for a node
    ///
    /// # Arguments
    ///
    /// * `chain_id` - Lineage chain ID
    /// * `node_id` - Node ID to generate proof for
    ///
    /// # Errors
    ///
    /// Returns error if proof generation fails
    pub fn generate_lineage_proof(
        &self,
        chain_id: &str,
        node_id: &str,
    ) -> Result<LineageProof, BearDogError> {
        self.proof_manager.generate_proof(chain_id, node_id)
    }

    /// Verify a lineage proof
    ///
    /// # Arguments
    ///
    /// * `proof` - The proof to verify
    /// * `chain_id` - Lineage chain ID to verify against
    ///
    /// # Errors
    ///
    /// Returns error if verification fails
    pub fn verify_lineage_proof(
        &self,
        proof: &LineageProof,
        chain_id: &str,
    ) -> Result<LineageVerificationResult, BearDogError> {
        self.proof_manager.verify_proof(proof, chain_id)
    }

    // =============================================================================
    // BirdSong Encryption
    // =============================================================================

    /// Encrypt a broadcast for a specific lineage
    ///
    /// # Arguments
    ///
    /// * `request` - Encryption request
    ///
    /// # Errors
    ///
    /// Returns error if encryption fails
    pub fn encrypt_broadcast(
        &self,
        request: &BirdSongEncryptRequest,
    ) -> Result<BirdSongBroadcast, BearDogError> {
        self.encryption.encrypt(request)
    }

    /// Decrypt a broadcast (requires valid lineage proof)
    ///
    /// # Arguments
    ///
    /// * `request` - Decryption request with broadcast and proof
    ///
    /// # Errors
    ///
    /// Returns error if decryption fails or proof is invalid
    pub fn decrypt_broadcast(
        &self,
        request: &BirdSongDecryptRequest,
    ) -> Result<Vec<u8>, BearDogError> {
        self.encryption.decrypt(request)
    }

    /// Check if a node can decrypt a broadcast
    pub fn can_decrypt(&self, broadcast: &BirdSongBroadcast, node_depth: u32) -> bool {
        self.encryption.can_decrypt(broadcast, node_depth)
    }

    // =============================================================================
    // Discovery Encryption (Family-Based, Simpler than Lineage)
    // =============================================================================

    /// Encrypt discovery packet for a specific family
    ///
    /// Uses family-specific keys derived from `family_id`. Only towers with the
    /// same family ID can decrypt. This is simpler than lineage-based encryption
    /// and perfect for UDP discovery broadcasts.
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Discovery message to encrypt
    /// * `family_id` - Family ID for key derivation
    ///
    /// # Returns
    ///
    /// Encrypted bytes with nonce prepended (nonce + ciphertext)
    ///
    /// # Errors
    ///
    /// Returns error if encryption fails
    pub fn encrypt_discovery_for_family(
        &self,
        plaintext: &[u8],
        family_id: &str,
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{
            ChaCha20Poly1305, Nonce,
            aead::{Aead, KeyInit},
        };
        use rand::RngCore;
        use sha3::{Digest, Sha3_256};

        // Derive 256-bit key from family_id using SHA3
        let mut hasher = Sha3_256::new();
        hasher.update(b"beardog:birdsong:discovery:v1:");
        hasher.update(family_id.as_bytes());
        let key_bytes = hasher.finalize();

        // Create cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to create cipher: {e}")))?;

        // Generate random nonce (ChaCha20-Poly1305 uses 12 bytes)
        let mut nonce_bytes = [0u8; 12];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {e}")))?;

        // Prepend nonce to ciphertext (nonce is public, safe to transmit)
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);

        Ok(result)
    }

    /// Decrypt discovery packet from a specific family
    ///
    /// Attempts to decrypt using family-specific keys. Will fail if:
    /// - Different family (different keys)
    /// - Corrupted ciphertext
    /// - Invalid authentication tag
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted bytes (nonce + ciphertext)
    /// * `family_id` - Family ID for key derivation
    ///
    /// # Returns
    ///
    /// Decrypted plaintext bytes
    ///
    /// # Errors
    ///
    /// Returns error if decryption fails (likely different family or corrupted data)
    pub fn decrypt_discovery_from_family(
        &self,
        encrypted: &[u8],
        family_id: &str,
    ) -> Result<Vec<u8>, BearDogError> {
        use chacha20poly1305::{
            ChaCha20Poly1305, Nonce,
            aead::{Aead, KeyInit},
        };
        use sha3::{Digest, Sha3_256};

        // Check minimum length (nonce is 12 bytes + at least some ciphertext)
        if encrypted.len() < 13 {
            return Err(BearDogError::validation(
                "Encrypted data too short (need nonce + ciphertext)",
            ));
        }

        // Derive same 256-bit key from family_id
        let mut hasher = Sha3_256::new();
        hasher.update(b"beardog:birdsong:discovery:v1:");
        hasher.update(family_id.as_bytes());
        let key_bytes = hasher.finalize();

        // Create cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key_bytes)
            .map_err(|e| BearDogError::crypto_error(format!("Failed to create cipher: {e}")))?;

        // Extract nonce (first 12 bytes) and ciphertext (rest)
        let (nonce_bytes, ciphertext) = encrypted.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt
        let plaintext = cipher.decrypt(nonce, ciphertext).map_err(|e| {
            BearDogError::crypto_error(format!("Decryption failed (likely different family): {e}"))
        })?;

        Ok(plaintext)
    }

    // =============================================================================
    // Key Distribution (Phase 1-2 Requirement)
    // =============================================================================

    /// Distribute keys to all descendants of a node
    ///
    /// This generates and distributes keys to all nodes in the subtree,
    /// allowing them to decrypt broadcasts for their lineage level.
    ///
    /// # Arguments
    ///
    /// * `chain_id` - Lineage chain ID
    /// * `root_id` - Root node to start distribution from
    /// * `generation` - Key generation number
    ///
    /// # Errors
    ///
    /// Returns error if key distribution fails
    pub fn distribute_keys_to_descendants(
        &self,
        chain_id: &str,
        root_id: &str,
        generation: u32,
    ) -> Result<usize, BearDogError> {
        info!(
            "🔑 Distributing keys to descendants of {} (gen: {})",
            root_id, generation
        );

        let descendants = self.chain_manager.get_descendants(chain_id, root_id);

        let mut distributed_count = 0;
        let mut keys_map = self.distributed_keys.write();

        for descendant in descendants {
            // Create lineage hint for this descendant
            let hint = LineageHint {
                root_id: root_id.to_string(),
                min_depth: descendant.depth,
                max_depth: descendant.depth,
                biome_filter: None,
                version: 1,
            };

            // Derive key for this descendant
            let key = self.kdf.derive_key(&hint, generation)?;

            // Store distributed key
            keys_map
                .entry(descendant.node_id.clone())
                .or_default()
                .push(key);

            distributed_count += 1;
        }

        info!("✅ Distributed {} keys to descendants", distributed_count);
        Ok(distributed_count)
    }

    /// Get distributed keys for a node
    ///
    /// # Arguments
    ///
    /// * `node_id` - Node ID
    ///
    /// # Returns
    ///
    /// List of keys distributed to this node
    pub fn get_distributed_keys(&self, node_id: &str) -> Vec<BirdSongKey> {
        self.distributed_keys
            .read()
            .get(node_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Request a key for a specific lineage (with proof)
    ///
    /// This is the key request flow mentioned in the `BirdSong` spec.
    ///
    /// # Arguments
    ///
    /// * `lineage_hint` - Hint for which lineage key is needed
    /// * `proof` - Lineage proof from requester
    /// * `chain_id` - Chain ID to verify proof against
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Proof is invalid
    /// - Requester not authorized for this key
    pub fn request_key(
        &self,
        lineage_hint: &LineageHint,
        proof: &LineageProof,
        chain_id: &str,
    ) -> Result<BirdSongKey, BearDogError> {
        info!(
            "🔑 Key request from {} for lineage {}",
            proof.node_id, lineage_hint.root_id
        );

        // Verify proof
        let verification = self.proof_manager.verify_proof(proof, chain_id)?;
        if !verification.valid {
            warn!("Key request denied: invalid proof");
            return Err(BearDogError::system(format!(
                "Invalid lineage proof: {:?}",
                verification.failure_reason
            )));
        }

        // Check if requester is authorized for this depth
        if verification.depth < lineage_hint.min_depth
            || verification.depth > lineage_hint.max_depth
        {
            warn!(
                "Key request denied: depth {} not in range [{}, {}]",
                verification.depth, lineage_hint.min_depth, lineage_hint.max_depth
            );
            return Err(BearDogError::system(format!(
                "Requester depth {} not authorized for key (range: [{}, {}])",
                verification.depth, lineage_hint.min_depth, lineage_hint.max_depth
            )));
        }

        // Derive and return key
        let key = self.kdf.derive_key(lineage_hint, 0)?; // Generation 0 for now

        info!("✅ Key granted to {}", proof.node_id);
        Ok(key)
    }

    /// Revoke keys for a node and its descendants
    ///
    /// # Arguments
    ///
    /// * `chain_id` - Lineage chain ID
    /// * `node_id` - Node to revoke keys from (including descendants)
    ///
    /// # Returns
    ///
    /// Number of nodes affected
    pub fn revoke_keys(&self, chain_id: &str, node_id: &str) -> usize {
        info!("🚫 Revoking keys for {} and descendants", node_id);

        let descendants = self.chain_manager.get_descendants(chain_id, node_id);
        let mut keys_map = self.distributed_keys.write();

        let mut revoked_count = 0;

        // Revoke keys for the node itself
        if keys_map.remove(node_id).is_some() {
            revoked_count += 1;
        }

        // Revoke keys for all descendants
        for descendant in descendants {
            if keys_map.remove(&descendant.node_id).is_some() {
                revoked_count += 1;
            }
        }

        info!("✅ Revoked keys from {} nodes", revoked_count);
        revoked_count
    }

    /// Rotate all keys (increment generation)
    ///
    /// This should be called periodically based on `config.key_rotation_interval_secs`
    ///
    /// # Arguments
    ///
    /// * `chain_id` - Lineage chain ID
    /// * `root_id` - Root node to start rotation from
    /// * `new_generation` - New generation number
    ///
    /// # Errors
    ///
    /// Returns error if rotation fails
    pub fn rotate_all_keys(
        &self,
        chain_id: &str,
        root_id: &str,
        new_generation: u32,
    ) -> Result<usize, BearDogError> {
        info!(
            "🔄 Rotating all keys for lineage {} (gen: {})",
            root_id, new_generation
        );

        // Distribute new keys with new generation
        self.distribute_keys_to_descendants(chain_id, root_id, new_generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_manager() -> BirdSongManager {
        let master_secret = vec![0xEF; 32];
        BirdSongManager::new(master_secret, None).await.unwrap()
    }

    #[tokio::test]
    async fn test_full_workflow() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // 1. Generate root lineage
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        // 2. Add children
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-2".to_string(), None)
            .await?;

        // 3. Distribute keys
        let count = manager.distribute_keys_to_descendants(&chain.chain_id, "root", 0)?;
        assert_eq!(count, 2); // 2 children

        // 4. Generate proof for child-1
        let proof = manager.generate_lineage_proof(&chain.chain_id, "child-1")?;

        // 5. Verify proof
        let verification = manager.verify_lineage_proof(&proof, &chain.chain_id)?;
        assert!(verification.valid);
        assert_eq!(verification.depth, 1);

        // 6. Encrypt broadcast for depth 1
        let hint = LineageHint {
            root_id: "root".to_string(),
            min_depth: 1,
            max_depth: 1,
            biome_filter: None,
            version: 1,
        };

        let encrypt_req = BirdSongEncryptRequest {
            plaintext: b"Message for children".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };

        let broadcast = manager.encrypt_broadcast(&encrypt_req)?;

        // 7. Decrypt broadcast with child-1's proof
        let decrypt_req = BirdSongDecryptRequest { broadcast, proof };

        let decrypted = manager.decrypt_broadcast(&decrypt_req)?;
        assert_eq!(decrypted, b"Message for children");

        Ok(())
    }

    #[tokio::test]
    async fn test_key_request_with_proof() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // Setup lineage
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child".to_string(), None)
            .await?;

        // Generate proof
        let proof = manager.generate_lineage_proof(&chain.chain_id, "child")?;

        // Request key
        let hint = LineageHint {
            root_id: "root".to_string(),
            min_depth: 1,
            max_depth: 1,
            biome_filter: None,
            version: 1,
        };

        let key = manager.request_key(&hint, &proof, &chain.chain_id)?;
        assert_eq!(key.hint.root_id, "root");

        Ok(())
    }

    #[tokio::test]
    async fn test_key_revocation() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // Setup lineage
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "child-1", "grandchild-1".to_string(), None)
            .await?;

        // Distribute keys
        manager.distribute_keys_to_descendants(&chain.chain_id, "root", 0)?;

        // Verify keys exist
        assert!(!manager.get_distributed_keys("child-1").is_empty());
        assert!(!manager.get_distributed_keys("grandchild-1").is_empty());

        // Revoke child-1 and its descendants
        let revoked_count = manager.revoke_keys(&chain.chain_id, "child-1");
        assert_eq!(revoked_count, 2);

        // Verify keys are gone
        assert!(manager.get_distributed_keys("child-1").is_empty());
        assert!(manager.get_distributed_keys("grandchild-1").is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_key_rotation() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // Setup lineage
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child".to_string(), None)
            .await?;

        // Distribute gen 0 keys
        manager.distribute_keys_to_descendants(&chain.chain_id, "root", 0)?;
        let keys_gen0 = manager.get_distributed_keys("child");
        assert_eq!(keys_gen0.len(), 1);
        assert_eq!(keys_gen0[0].generation, 0);

        // Rotate to gen 1
        manager.rotate_all_keys(&chain.chain_id, "root", 1)?;
        let keys_gen1 = manager.get_distributed_keys("child");
        assert_eq!(keys_gen1.len(), 2); // Both gen 0 and gen 1
        assert_eq!(keys_gen1[1].generation, 1);

        Ok(())
    }
}
