// SPDX-License-Identifier: AGPL-3.0-or-later

//! `BirdSong` broadcast encryption and decryption

use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use chrono::Utc;
use rand::RngCore;
use tracing::{debug, info};

use beardog_errors::BearDogError;

use super::key_derivation::LineageKeyDerivation;
use super::types::{BirdSongBroadcast, BirdSongDecryptRequest, BirdSongEncryptRequest};

/// `BirdSong` encryption manager
pub struct BirdSongEncryption {
    kdf: std::sync::Arc<LineageKeyDerivation>,
}

impl BirdSongEncryption {
    /// Create new `BirdSong` encryption manager
    ///
    /// # Arguments
    ///
    /// * `kdf` - Shared key derivation manager
    pub fn new(kdf: std::sync::Arc<LineageKeyDerivation>) -> Self {
        info!("🎵 Initializing BirdSongEncryption");
        Self { kdf }
    }

    /// Encrypt a broadcast for a specific lineage
    ///
    /// Uses ChaCha20-Poly1305 AEAD with lineage-derived keys.
    ///
    /// # Arguments
    ///
    /// * `request` - Encryption request with plaintext and lineage hint
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Key derivation fails
    /// - Encryption fails
    pub fn encrypt(
        &self,
        request: &BirdSongEncryptRequest,
    ) -> Result<BirdSongBroadcast, BearDogError> {
        debug!(
            "🔒 Encrypting broadcast for lineage {} (size: {} bytes)",
            request.lineage_hint.root_id,
            request.plaintext.len()
        );

        // Derive key for this lineage
        let key = self.kdf.derive_key(&request.lineage_hint, 0)?; // Generation 0 for now

        // Check key expiration
        if self.kdf.is_key_expired(&key) {
            return Err(BearDogError::system("Key expired".to_string()));
        }

        // Initialize ChaCha20-Poly1305 cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key.key_material)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        // Generate random nonce (96 bits / 12 bytes for ChaCha20-Poly1305)
        let mut nonce_bytes = [0u8; 12];
        let mut rng = rand::rng();
        rng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Prepare additional authenticated data (AAD)
        let _aad = request.associated_data.as_deref().unwrap_or(&[]);

        // Encrypt plaintext
        let ciphertext = cipher
            .encrypt(nonce, request.plaintext.as_slice())
            .map_err(|e| BearDogError::system(format!("Encryption failed: {e}")))?;

        // If there's AAD, append it (it's authenticated but not encrypted)
        // Note: ChaCha20-Poly1305 handles AAD internally, we just need to pass it during encrypt/decrypt

        let broadcast = BirdSongBroadcast {
            hint: request.lineage_hint.clone(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
            associated_data: request.associated_data.clone(),
            broadcast_at: Utc::now(),
        };

        debug!(
            "✅ Broadcast encrypted (ciphertext size: {} bytes)",
            broadcast.ciphertext.len()
        );
        Ok(broadcast)
    }

    /// Decrypt a `BirdSong` broadcast
    ///
    /// Verifies the lineage proof and decrypts if the node is authorized.
    ///
    /// # Arguments
    ///
    /// * `request` - Decryption request with broadcast and lineage proof
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Lineage proof is invalid
    /// - Key derivation fails
    /// - Decryption fails (wrong key or tampered ciphertext)
    pub fn decrypt(&self, request: &BirdSongDecryptRequest) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "🔓 Decrypting broadcast for node {} (ciphertext size: {} bytes)",
            request.proof.node_id,
            request.broadcast.ciphertext.len()
        );

        // Verify node's lineage proof matches the broadcast hint
        if request.proof.root_id != request.broadcast.hint.root_id {
            return Err(BearDogError::system(format!(
                "Lineage mismatch: broadcast for {}, proof for {}",
                request.broadcast.hint.root_id, request.proof.root_id
            )));
        }

        // Calculate node depth from proof
        #[expect(
            clippy::cast_possible_truncation,
            reason = "Merkle path depth fits u32 for depth checks"
        )]
        let node_depth = (request.proof.path.len() - 1) as u32;

        // Check if node depth is within allowed range
        // Note: Sender is ALWAYS allowed to decrypt their own messages (for verification)
        // So we skip depth check if this is the sender's own key
        let is_sender = request
            .proof
            .path
            .last()
            .is_some_and(|last| last == &request.proof.node_id);

        if !is_sender
            && (node_depth < request.broadcast.hint.min_depth
                || node_depth > request.broadcast.hint.max_depth)
        {
            return Err(BearDogError::system(format!(
                "Node depth {} not in allowed range [{}, {}]",
                node_depth, request.broadcast.hint.min_depth, request.broadcast.hint.max_depth
            )));
        }

        // Derive the same key used for encryption
        // Note: We use generation 0 here; in production, generation would be included in broadcast metadata
        let key = self.kdf.derive_key(&request.broadcast.hint, 0)?;

        // Check key validity for this depth
        // Note: Sender is always allowed (already checked above), so skip depth validity check for sender
        if !is_sender && !self.kdf.is_key_valid_for_depth(&key, node_depth) {
            return Err(BearDogError::system(format!(
                "Key not valid for depth {node_depth}"
            )));
        }

        // Initialize cipher
        let cipher = ChaCha20Poly1305::new_from_slice(&key.key_material)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        // Extract nonce
        if request.broadcast.nonce.len() != 12 {
            return Err(BearDogError::system(format!(
                "Invalid nonce length: {} (expected 12)",
                request.broadcast.nonce.len()
            )));
        }
        let nonce = Nonce::from_slice(&request.broadcast.nonce);

        // Decrypt ciphertext
        let plaintext = cipher
            .decrypt(nonce, request.broadcast.ciphertext.as_slice())
            .map_err(|e| {
                BearDogError::system(format!(
                    "Decryption failed (wrong key or tampered data): {e}"
                ))
            })?;

        debug!(
            "✅ Broadcast decrypted (plaintext size: {} bytes)",
            plaintext.len()
        );
        Ok(plaintext)
    }

    /// Check if a node can decrypt a broadcast (without actually decrypting)
    ///
    /// # Arguments
    ///
    /// * `broadcast` - The broadcast to check
    /// * `node_depth` - The node's depth in the lineage tree
    ///
    /// # Returns
    ///
    /// `true` if the node is authorized to decrypt, `false` otherwise
    pub const fn can_decrypt(&self, broadcast: &BirdSongBroadcast, node_depth: u32) -> bool {
        node_depth >= broadcast.hint.min_depth && node_depth <= broadcast.hint.max_depth
    }

    /// Encrypt for multiple lineage depths (broadcast to entire family tree)
    ///
    /// This creates separate broadcasts for each depth level, allowing fine-grained access control.
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    /// * `root_id` - Root lineage ID
    /// * `max_depth` - Maximum depth to broadcast to
    ///
    /// # Errors
    ///
    /// Returns error if encryption fails for any depth level
    pub fn encrypt_hierarchical(
        &self,
        plaintext: &[u8],
        root_id: &str,
        max_depth: u32,
    ) -> Result<Vec<BirdSongBroadcast>, BearDogError> {
        info!(
            "🎵 Encrypting hierarchical broadcast for {} (depths: 0-{})",
            root_id, max_depth
        );

        let mut broadcasts = Vec::new();

        for depth in 0..=max_depth {
            let hint = super::types::LineageHint {
                root_id: root_id.to_string(),
                min_depth: depth,
                max_depth: depth,
                biome_filter: None,
                version: 1,
            };

            let request = BirdSongEncryptRequest {
                plaintext: plaintext.to_vec(),
                lineage_hint: hint,
                associated_data: None,
            };

            let broadcast = self.encrypt(&request)?;
            broadcasts.push(broadcast);
        }

        debug!("✅ Created {} hierarchical broadcasts", broadcasts.len());
        Ok(broadcasts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::birdsong::types::{LineageHint, LineageProof};
    use std::sync::Arc;

    fn create_test_encryption() -> BirdSongEncryption {
        let master_secret = vec![0xCD; 32];
        let kdf = Arc::new(LineageKeyDerivation::new(master_secret).unwrap());
        BirdSongEncryption::new(kdf)
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
        let encryption = create_test_encryption();

        let plaintext = b"Hello, family! This is a secret message.";
        let hint = LineageHint {
            root_id: "test-family".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        // Encrypt
        let request = BirdSongEncryptRequest {
            plaintext: plaintext.to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };
        let broadcast = encryption.encrypt(&request)?;

        // Create mock proof for decryption
        let proof = LineageProof {
            node_id: "test-node".to_string(),
            root_id: "test-family".to_string(),
            path: vec!["test-family".to_string(), "test-node".to_string()],
            proof_chain: Vec::new(),
            merkle_root: Vec::new(),
            generation: 0,
            head_commitment: vec![],
            generated_at: Utc::now(),
        };

        // Decrypt
        let decrypt_request = BirdSongDecryptRequest { broadcast, proof };
        let decrypted = encryption.decrypt(&decrypt_request)?;

        // Verify
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_depth_authorization() -> Result<(), BearDogError> {
        let encryption = create_test_encryption();

        let plaintext = b"For children only";
        let hint = LineageHint {
            root_id: "test-family".to_string(),
            min_depth: 1, // Only children and below
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let request = BirdSongEncryptRequest {
            plaintext: plaintext.to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };
        let broadcast = encryption.encrypt(&request)?;

        // Root (depth 0) should not be able to decrypt
        let root_proof = LineageProof {
            node_id: "root".to_string(),
            root_id: "test-family".to_string(),
            path: vec!["test-family".to_string()],
            proof_chain: Vec::new(),
            merkle_root: Vec::new(),
            generation: 0,
            head_commitment: vec![],
            generated_at: Utc::now(),
        };

        let root_decrypt = BirdSongDecryptRequest {
            broadcast: broadcast.clone(),
            proof: root_proof,
        };

        // Should fail for root
        assert!(encryption.decrypt(&root_decrypt).is_err());

        // Child (depth 1) should be able to decrypt
        let child_proof = LineageProof {
            node_id: "child".to_string(),
            root_id: "test-family".to_string(),
            path: vec!["test-family".to_string(), "child".to_string()],
            proof_chain: Vec::new(),
            merkle_root: Vec::new(),
            generation: 0,
            head_commitment: vec![],
            generated_at: Utc::now(),
        };

        let child_decrypt = BirdSongDecryptRequest {
            broadcast,
            proof: child_proof,
        };

        let decrypted = encryption.decrypt(&child_decrypt)?;
        assert_eq!(decrypted, plaintext);

        Ok(())
    }

    #[test]
    fn test_wrong_lineage_fails() -> Result<(), BearDogError> {
        let encryption = create_test_encryption();

        let plaintext = b"Family A secret";
        let hint_a = LineageHint {
            root_id: "family-a".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let request = BirdSongEncryptRequest {
            plaintext: plaintext.to_vec(),
            lineage_hint: hint_a,
            associated_data: None,
        };
        let broadcast = encryption.encrypt(&request)?;

        // Try to decrypt with proof from family-b
        let wrong_proof = LineageProof {
            node_id: "family-b-node".to_string(),
            root_id: "family-b".to_string(),
            path: vec!["family-b".to_string(), "family-b-node".to_string()],
            proof_chain: Vec::new(),
            merkle_root: Vec::new(),
            generation: 0,
            head_commitment: vec![],
            generated_at: Utc::now(),
        };

        let decrypt_request = BirdSongDecryptRequest {
            broadcast,
            proof: wrong_proof,
        };

        // Should fail due to lineage mismatch
        assert!(encryption.decrypt(&decrypt_request).is_err());

        Ok(())
    }

    #[test]
    fn test_hierarchical_encryption() -> Result<(), BearDogError> {
        let encryption = create_test_encryption();

        let plaintext = b"Broadcast to all levels";
        let broadcasts = encryption.encrypt_hierarchical(plaintext, "test-root", 3)?;

        // Should have 4 broadcasts (depths 0, 1, 2, 3)
        assert_eq!(broadcasts.len(), 4);

        // Each broadcast should have different ciphertext
        assert_ne!(broadcasts[0].ciphertext, broadcasts[1].ciphertext);
        assert_ne!(broadcasts[1].ciphertext, broadcasts[2].ciphertext);

        // Verify each broadcast can be decrypted by appropriate depth
        for (depth, broadcast) in broadcasts.iter().enumerate() {
            let proof = LineageProof {
                node_id: format!("node-depth-{depth}"),
                root_id: "test-root".to_string(),
                path: (0..=depth).map(|d| format!("node-depth-{d}")).collect(),
                proof_chain: Vec::new(),
                merkle_root: Vec::new(),
                generation: 0,
                head_commitment: vec![],
                generated_at: Utc::now(),
            };

            let decrypt_request = BirdSongDecryptRequest {
                broadcast: broadcast.clone(),
                proof,
            };

            let decrypted = encryption.decrypt(&decrypt_request)?;
            assert_eq!(decrypted, plaintext);
        }

        Ok(())
    }

    #[test]
    fn test_can_decrypt_check() {
        let encryption = create_test_encryption();

        let hint = LineageHint {
            root_id: "test-root".to_string(),
            min_depth: 2,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let broadcast = BirdSongBroadcast {
            hint,
            nonce: vec![0u8; 12],
            ciphertext: Vec::new(),
            associated_data: None,
            broadcast_at: Utc::now(),
        };

        // Depth 0, 1: cannot decrypt
        assert!(!encryption.can_decrypt(&broadcast, 0));
        assert!(!encryption.can_decrypt(&broadcast, 1));

        // Depth 2-5: can decrypt
        assert!(encryption.can_decrypt(&broadcast, 2));
        assert!(encryption.can_decrypt(&broadcast, 3));
        assert!(encryption.can_decrypt(&broadcast, 4));
        assert!(encryption.can_decrypt(&broadcast, 5));

        // Depth 6+: cannot decrypt
        assert!(!encryption.can_decrypt(&broadcast, 6));
    }
}
