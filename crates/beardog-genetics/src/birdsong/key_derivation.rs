// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key derivation from lineage using HKDF

use chrono::{Duration as ChronoDuration, Utc};
use hkdf::Hkdf;
use sha2::Sha256;
use tracing::{debug, info};
use zeroize::Zeroizing;

use beardog_errors::BearDogError;

use super::types::{BirdSongKey, LineageDepth, LineageHint};

/// Key derivation manager for lineage-based keys
pub struct LineageKeyDerivation {
    /// Master secret for this instance (in production, from HSM)
    master_secret: Zeroizing<Vec<u8>>,
}

impl LineageKeyDerivation {
    /// Create new key derivation manager
    ///
    /// # Arguments
    ///
    /// * `master_secret` - Master secret for key derivation (32 bytes minimum)
    ///
    /// # Errors
    ///
    /// Returns error if master secret is too short
    pub fn new(master_secret: Vec<u8>) -> Result<Self, BearDogError> {
        if master_secret.len() < 32 {
            return Err(BearDogError::system(
                "Master secret must be at least 32 bytes".to_string(),
            ));
        }

        info!("🔑 Initializing LineageKeyDerivation");
        Ok(Self {
            master_secret: Zeroizing::new(master_secret),
        })
    }

    /// Derive a `BirdSong` key for a specific lineage
    ///
    /// Uses HKDF-SHA256 to derive keys from:
    /// - Master secret (IKM)
    /// - Lineage hint (info/context)
    /// - Generation number (for rotation)
    ///
    /// # Arguments
    ///
    /// * `hint` - Lineage hint specifying who can decrypt
    /// * `generation` - Key generation number (for rotation)
    ///
    /// # Errors
    ///
    /// Returns error if key derivation fails
    pub fn derive_key(
        &self,
        hint: &LineageHint,
        generation: u32,
    ) -> Result<BirdSongKey, BearDogError> {
        debug!(
            "🔐 Deriving key for lineage {} (gen: {})",
            hint.root_id, generation
        );

        // Create HKDF context from lineage hint + generation
        let context = self.create_kdf_context(hint, generation);

        // Derive key material using HKDF-SHA256
        let hk = Hkdf::<Sha256>::new(Some(&[0u8; 32]), &self.master_secret);
        let mut key_material = Zeroizing::new(vec![0u8; 32]); // ChaCha20-Poly1305 key size
        hk.expand(&context, &mut key_material)
            .map_err(|e| BearDogError::system(format!("HKDF expansion failed: {e}")))?;

        // Calculate key validity period
        let now = Utc::now();
        let valid_from = now;
        let expires_at = now + ChronoDuration::hours(24); // 24 hour validity

        let key = BirdSongKey {
            key_material,
            hint: hint.clone(),
            generation,
            valid_from,
            expires_at,
        };

        debug!("✅ Key derived successfully");
        Ok(key)
    }

    /// Derive keys for all descendants at different depths
    ///
    /// This creates a key for each depth level, allowing hierarchical access control.
    ///
    /// # Arguments
    ///
    /// * `root_id` - Root lineage ID
    /// * `max_depth` - Maximum depth to derive keys for
    /// * `generation` - Key generation number
    ///
    /// # Errors
    ///
    /// Returns error if key derivation fails
    pub fn derive_hierarchical_keys(
        &self,
        root_id: &str,
        max_depth: LineageDepth,
        generation: u32,
    ) -> Result<Vec<BirdSongKey>, BearDogError> {
        info!(
            "🔑 Deriving hierarchical keys for {} (depth: 0-{})",
            root_id, max_depth
        );

        let mut keys = Vec::new();

        for depth in 0..=max_depth {
            let hint = LineageHint {
                root_id: root_id.to_string(),
                min_depth: depth,
                max_depth: depth,
                biome_filter: None,
                version: 1,
            };

            let key = self.derive_key(&hint, generation)?;
            keys.push(key);
        }

        debug!("✅ Derived {} hierarchical keys", keys.len());
        Ok(keys)
    }

    /// Rotate keys by incrementing the generation number
    ///
    /// # Arguments
    ///
    /// * `current_key` - Current key to rotate
    ///
    /// # Errors
    ///
    /// Returns error if key derivation fails
    pub fn rotate_key(&self, current_key: &BirdSongKey) -> Result<BirdSongKey, BearDogError> {
        info!(
            "🔄 Rotating key (gen: {} -> {})",
            current_key.generation,
            current_key.generation + 1
        );

        self.derive_key(&current_key.hint, current_key.generation + 1)
    }

    /// Check if a key is expired
    pub fn is_key_expired(&self, key: &BirdSongKey) -> bool {
        Utc::now() > key.expires_at
    }

    /// Check if a key is valid for a given lineage depth
    pub const fn is_key_valid_for_depth(&self, key: &BirdSongKey, depth: LineageDepth) -> bool {
        depth >= key.hint.min_depth && depth <= key.hint.max_depth
    }

    /// Create HKDF context from lineage hint and generation
    fn create_kdf_context(&self, hint: &LineageHint, generation: u32) -> Vec<u8> {
        // Context format: "birdsong-v{version}|{root_id}|{min_depth}|{max_depth}|{biome_filter}|{generation}"
        let mut context = Vec::new();

        // Version prefix
        context.extend_from_slice(b"birdsong-v");
        context.push(hint.version);
        context.push(b'|');

        // Root ID
        context.extend_from_slice(hint.root_id.as_bytes());
        context.push(b'|');

        // Depth range
        context.extend_from_slice(&hint.min_depth.to_le_bytes());
        context.push(b'|');
        context.extend_from_slice(&hint.max_depth.to_le_bytes());
        context.push(b'|');

        // Biome filter (optional)
        if let Some(ref biome) = hint.biome_filter {
            context.extend_from_slice(biome.as_bytes());
        }
        context.push(b'|');

        // Generation number
        context.extend_from_slice(&generation.to_le_bytes());

        context
    }

    /// Derive a master secret from entropy sources (for initialization)
    ///
    /// In production, this would use entropy from the HSM and genetics engine.
    ///
    /// # Arguments
    ///
    /// * `entropy_sources` - Multiple entropy sources to mix
    ///
    /// # Errors
    ///
    /// Returns error if derivation fails
    pub fn derive_master_secret(entropy_sources: &[&[u8]]) -> Result<Vec<u8>, BearDogError> {
        if entropy_sources.is_empty() {
            return Err(BearDogError::system(
                "At least one entropy source required".to_string(),
            ));
        }

        // Mix entropy sources using HKDF
        let mut mixed_entropy = Vec::new();
        for source in entropy_sources {
            mixed_entropy.extend_from_slice(source);
        }

        let hk = Hkdf::<Sha256>::new(None, &mixed_entropy);
        let mut master_secret = vec![0u8; 32];
        hk.expand(b"birdsong-master-secret-v1", &mut master_secret)
            .map_err(|e| BearDogError::system(format!("Master secret derivation failed: {e}")))?;

        Ok(master_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_kdf() -> LineageKeyDerivation {
        let master_secret = vec![0xAB; 32]; // Test secret
        LineageKeyDerivation::new(master_secret).unwrap()
    }

    #[test]
    fn test_derive_key() -> Result<(), BearDogError> {
        let kdf = create_test_kdf();

        let hint = LineageHint {
            root_id: "test-root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let key = kdf.derive_key(&hint, 0)?;

        assert_eq!(key.key_material.len(), 32);
        assert_eq!(key.generation, 0);
        assert_eq!(key.hint.root_id, "test-root");
        assert!(!kdf.is_key_expired(&key));

        Ok(())
    }

    #[test]
    fn test_deterministic_derivation() -> Result<(), BearDogError> {
        let kdf = create_test_kdf();

        let hint = LineageHint {
            root_id: "test-root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        // Derive same key twice
        let key1 = kdf.derive_key(&hint, 0)?;
        let key2 = kdf.derive_key(&hint, 0)?;

        // Should be identical (deterministic)
        assert_eq!(*key1.key_material, *key2.key_material);

        Ok(())
    }

    #[test]
    fn test_different_generations_different_keys() -> Result<(), BearDogError> {
        let kdf = create_test_kdf();

        let hint = LineageHint {
            root_id: "test-root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let key_gen0 = kdf.derive_key(&hint, 0)?;
        let key_gen1 = kdf.derive_key(&hint, 1)?;

        // Different generations should have different keys
        assert_ne!(*key_gen0.key_material, *key_gen1.key_material);

        Ok(())
    }

    #[test]
    fn test_key_rotation() -> Result<(), BearDogError> {
        let kdf = create_test_kdf();

        let hint = LineageHint {
            root_id: "test-root".to_string(),
            min_depth: 0,
            max_depth: 5,
            biome_filter: None,
            version: 1,
        };

        let key = kdf.derive_key(&hint, 0)?;
        let rotated_key = kdf.rotate_key(&key)?;

        assert_eq!(rotated_key.generation, 1);
        assert_ne!(*key.key_material, *rotated_key.key_material);

        Ok(())
    }

    #[test]
    fn test_hierarchical_keys() -> Result<(), BearDogError> {
        let kdf = create_test_kdf();

        let keys = kdf.derive_hierarchical_keys("test-root", 3, 0)?;

        assert_eq!(keys.len(), 4); // Depths 0, 1, 2, 3

        // Each key should have different material
        assert_ne!(*keys[0].key_material, *keys[1].key_material);
        assert_ne!(*keys[1].key_material, *keys[2].key_material);
        assert_ne!(*keys[2].key_material, *keys[3].key_material);

        // Each key should be valid for its depth
        assert!(kdf.is_key_valid_for_depth(&keys[0], 0));
        assert!(kdf.is_key_valid_for_depth(&keys[1], 1));
        assert!(!kdf.is_key_valid_for_depth(&keys[0], 1));

        Ok(())
    }

    #[test]
    fn test_derive_master_secret() -> Result<(), BearDogError> {
        let entropy1 = b"source1-entropy-data";
        let entropy2 = b"source2-entropy-data";
        let entropy3 = b"source3-entropy-data";

        let master_secret = LineageKeyDerivation::derive_master_secret(&[
            entropy1.as_slice(),
            entropy2.as_slice(),
            entropy3.as_slice(),
        ])?;

        assert_eq!(master_secret.len(), 32);

        // Should be deterministic
        let master_secret2 = LineageKeyDerivation::derive_master_secret(&[
            entropy1.as_slice(),
            entropy2.as_slice(),
            entropy3.as_slice(),
        ])?;

        assert_eq!(master_secret, master_secret2);

        Ok(())
    }
}
