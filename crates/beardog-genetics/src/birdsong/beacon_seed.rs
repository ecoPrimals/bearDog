// SPDX-License-Identifier: AGPL-3.0-only

//! Beacon Seed for Dark Forest Discovery
//!
//! Separate from lineage seed - controls WHO CAN SEE your beacons,
//! not what they can do (that's lineage permissions).
//!
//! ## Two-Seed Architecture
//!
//! ```text
//! ┌──────────────────────────────────────────────────────────────┐
//! │  BEACON SEED (Discovery)                                     │
//! │  • Who can see my beacons?                                   │
//! │  • Social graph of meetings                                  │
//! │  • Exchanged on meeting, not inherited                       │
//! │  • Enables TRUE Dark Forest (no metadata leakage)            │
//! └──────────────────────────────────────────────────────────────┘
//!                         │
//!                         │ After beacon discovery
//!                         ▼
//! ┌──────────────────────────────────────────────────────────────┐
//! │  LINEAGE SEED (Permissions)                                  │
//! │  • What can they do after meeting?                           │
//! │  • Cryptographic family trust                                │
//! │  • Inherited through lineage                                 │
//! │  • Unchanged from existing implementation                    │
//! └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Security Properties
//!
//! - **ChaCha20-Poly1305 AEAD**: Authenticated encryption
//! - **HKDF-SHA256**: Key derivation from master secret
//! - **BLAKE3**: Beacon ID derivation (fast, cryptographically secure)
//! - **Zeroize**: Automatic secret cleanup on drop
//! - **`OsRng`**: Cryptographically secure randomness

use beardog_errors::BearDogError;
use blake3::Hasher;
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use hkdf::Hkdf;
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeroize::Zeroizing;

/// Beacon seed for discovery encryption
///
/// Controls visibility in Dark Forest - who can decrypt and see your beacons.
///
/// **Deep Debt Principle #2**: Separate concerns (beacon discovery ≠ lineage permissions)
#[derive(Clone)]
pub struct BeaconSeed {
    /// Core seed material (32 bytes, zeroized on drop)
    seed: Zeroizing<[u8; 32]>,

    /// Public beacon ID (derived from seed, safe to share)
    beacon_id: BeaconId,
}

/// Public beacon identifier (safe to share, deterministically derived)
///
/// 16 bytes (128 bits) provides:
/// - ~10^38 possible IDs (collision resistance)
/// - Compact representation
/// - Derived from seed via BLAKE3
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BeaconId(pub [u8; 16]);

/// Encrypted beacon data (opaque to non-family members)
///
/// Observers see only encrypted blob + nonce + timestamp.
/// No metadata leakage - TRUE Dark Forest!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeaconCiphertext {
    /// Encrypted payload (opaque to outsiders)
    pub ciphertext: Vec<u8>,

    /// Nonce for ChaCha20-Poly1305 (12 bytes)
    pub nonce: [u8; 12],

    /// Unix timestamp for replay protection
    pub timestamp: u64,
}

impl BeaconSeed {
    /// Generate new random beacon seed
    ///
    /// Uses `OsRng` for cryptographically secure randomness.
    ///
    /// **Deep Debt Principle #1**: Pure Rust crypto (`OsRng`, BLAKE3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_genetics::birdsong::BeaconSeed;
    ///
    /// let beacon = BeaconSeed::generate();
    /// let id = beacon.id(); // Public beacon ID
    /// ```
    #[must_use]
    pub fn generate() -> Self {
        let mut seed = [0u8; 32];
        OsRng.fill_bytes(&mut seed);

        let beacon_id = Self::derive_beacon_id(&seed);

        Self {
            seed: Zeroizing::new(seed),
            beacon_id,
        }
    }

    /// Derive beacon seed from master secret
    ///
    /// Enables deriving beacon seed from existing family seed
    /// for backward compatibility.
    ///
    /// **Deep Debt Principle #4**: Agnostic - works with existing or new seeds
    ///
    /// # Arguments
    ///
    /// * `master` - Master secret (64 bytes from genetic derivation)
    ///
    /// # Errors
    ///
    /// Returns error if HKDF derivation fails (should not happen with valid input)
    pub fn derive_from_master(master: &[u8; 64]) -> Result<Self, BearDogError> {
        // Use HKDF to derive beacon seed from master
        let hk = Hkdf::<Sha256>::new(None, &master[..32]);

        let mut seed = [0u8; 32];
        hk.expand(b"ecoPrimals-beacon-v1", &mut seed)
            .map_err(|e| BearDogError::system(format!("Beacon seed derivation failed: {e}")))?;

        let beacon_id = Self::derive_beacon_id(&seed);

        Ok(Self {
            seed: Zeroizing::new(seed),
            beacon_id,
        })
    }

    /// Get public beacon ID
    ///
    /// Safe to share - derived from seed but doesn't reveal seed.
    #[must_use]
    pub const fn id(&self) -> &BeaconId {
        &self.beacon_id
    }

    /// Encrypt data for Dark Forest broadcast
    ///
    /// Creates fully encrypted beacon with no metadata leakage.
    /// Only those with matching beacon genetics can decrypt.
    ///
    /// **Deep Debt Principle #6**: Production crypto (ChaCha20-Poly1305 AEAD)
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    ///
    /// # Errors
    ///
    /// Returns error if encryption fails (should not happen with valid key)
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<BeaconCiphertext, BearDogError> {
        let key = self.derive_encryption_key()?;

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        // Generate random nonce (12 bytes for ChaCha20-Poly1305)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt with AEAD (authenticated encryption)
        let ciphertext = cipher
            .encrypt(nonce, plaintext)
            .map_err(|e| BearDogError::system(format!("Encryption failed: {e}")))?;

        // Get current timestamp for replay protection
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| BearDogError::system(format!("Time error: {e}")))?
            .as_secs();

        Ok(BeaconCiphertext {
            ciphertext,
            nonce: nonce_bytes,
            timestamp,
        })
    }

    /// Try to decrypt beacon data
    ///
    /// Returns None if this beacon seed cannot decrypt (different beacon family).
    /// This is the core of Dark Forest - silent failure for outsiders.
    ///
    /// **Deep Debt Principle #5**: Runtime discovery - try and see what works
    ///
    /// # Arguments
    ///
    /// * `encrypted` - Encrypted beacon data to try decrypting
    ///
    /// # Errors
    ///
    /// Returns error only on system failures (key derivation, etc).
    /// Decryption failure returns Ok(None) - that's expected behavior.
    pub fn try_decrypt(
        &self,
        encrypted: &BeaconCiphertext,
    ) -> Result<Option<Vec<u8>>, BearDogError> {
        let key = self.derive_encryption_key()?;

        let cipher = ChaCha20Poly1305::new_from_slice(&key)
            .map_err(|e| BearDogError::system(format!("Cipher init failed: {e}")))?;

        let nonce = Nonce::from_slice(&encrypted.nonce);

        // Try decrypt - will fail silently if wrong beacon family
        match cipher.decrypt(nonce, encrypted.ciphertext.as_slice()) {
            Ok(plaintext) => Ok(Some(plaintext)),
            Err(_) => Ok(None), // Different beacon family - not an error!
        }
    }

    /// Derive beacon ID from seed
    ///
    /// Uses BLAKE3 for fast, cryptographically secure hashing.
    /// Deterministic - same seed always produces same ID.
    ///
    /// **Deep Debt Principle #1**: Pure Rust (BLAKE3)
    fn derive_beacon_id(seed: &[u8; 32]) -> BeaconId {
        let mut hasher = Hasher::new();
        hasher.update(seed);
        hasher.update(b"beacon-id-v1");

        let hash = hasher.finalize();

        // Use first 16 bytes of BLAKE3 hash
        let mut id = [0u8; 16];
        id.copy_from_slice(&hash.as_bytes()[..16]);

        BeaconId(id)
    }

    /// Derive encryption key from beacon seed
    ///
    /// Uses HKDF-SHA256 with domain separation.
    ///
    /// **Deep Debt Principle #1**: Pure Rust (HKDF)
    fn derive_encryption_key(&self) -> Result<[u8; 32], BearDogError> {
        let hk = Hkdf::<Sha256>::new(None, self.seed.as_ref());

        let mut key = [0u8; 32];
        hk.expand(b"beacon-encrypt-v1", &mut key)
            .map_err(|e| BearDogError::system(format!("Key derivation failed: {e}")))?;

        Ok(key)
    }

    /// Create `BeaconSeed` from raw seed material (for meeting exchange)
    ///
    /// Used when receiving beacon seed during meeting exchange.
    ///
    /// **Security**: Caller must ensure seed is from trusted source (meeting protocol)
    #[must_use]
    pub fn from_raw_seed(seed: [u8; 32], beacon_id: BeaconId) -> Self {
        Self {
            seed: Zeroizing::new(seed),
            beacon_id,
        }
    }

    /// Static beacon ID derivation (for external callers)
    ///
    /// Allows deriving beacon ID without creating full `BeaconSeed`.
    /// Useful for meeting verification.
    #[must_use]
    pub fn derive_beacon_id_static(seed: &[u8; 32]) -> BeaconId {
        Self::derive_beacon_id(seed)
    }
}

impl BeaconId {
    /// Convert beacon ID to hex string for display/logging
    ///
    /// Safe to log - doesn't reveal seed material.
    #[must_use]
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }

    /// Parse beacon ID from hex string
    ///
    /// # Errors
    ///
    /// Returns error if hex string is invalid or wrong length
    pub fn from_hex(hex_str: &str) -> Result<Self, BearDogError> {
        let bytes =
            hex::decode(hex_str).map_err(|e| BearDogError::system(format!("Invalid hex: {e}")))?;

        if bytes.len() != 16 {
            return Err(BearDogError::system(format!(
                "Beacon ID must be 16 bytes, got {}",
                bytes.len()
            )));
        }

        let mut id = [0u8; 16];
        id.copy_from_slice(&bytes);

        Ok(Self(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beacon_seed_generation() {
        let beacon1 = BeaconSeed::generate();
        let beacon2 = BeaconSeed::generate();

        // Different seeds produce different IDs
        assert_ne!(beacon1.id(), beacon2.id());
    }

    #[test]
    fn test_beacon_encrypt_decrypt_roundtrip() {
        let beacon = BeaconSeed::generate();
        let plaintext = b"Hello Dark Forest!";

        let encrypted = beacon.encrypt(plaintext).expect("encrypt failed");
        let decrypted = beacon
            .try_decrypt(&encrypted)
            .expect("decrypt failed")
            .expect("should decrypt with same beacon");

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn test_different_beacon_cannot_decrypt() {
        let beacon1 = BeaconSeed::generate();
        let beacon2 = BeaconSeed::generate();

        let plaintext = b"Secret message";
        let encrypted = beacon1.encrypt(plaintext).expect("encrypt failed");

        // Different beacon cannot decrypt (TRUE Dark Forest!)
        let result = beacon2.try_decrypt(&encrypted).expect("should not error");
        assert!(
            result.is_none(),
            "Different beacon should not be able to decrypt"
        );
    }

    #[test]
    fn test_beacon_id_derivation_deterministic() {
        let master = [42u8; 64];

        let beacon1 = BeaconSeed::derive_from_master(&master).expect("derive failed");
        let beacon2 = BeaconSeed::derive_from_master(&master).expect("derive failed");

        // Same master produces same beacon ID
        assert_eq!(beacon1.id(), beacon2.id());
    }

    #[test]
    fn test_beacon_id_hex_roundtrip() {
        let beacon = BeaconSeed::generate();
        let id = beacon.id();

        let hex = id.to_hex();
        let parsed = BeaconId::from_hex(&hex).expect("parse failed");

        assert_eq!(id, &parsed);
    }

    #[test]
    fn test_backward_compat_family_seed_derives_beacon() {
        // Simulate family seed (existing)
        let family_seed = [123u8; 64];

        let beacon = BeaconSeed::derive_from_master(&family_seed).expect("derive failed");

        // Should produce valid beacon seed
        assert_eq!(beacon.id().0.len(), 16);

        // Should be able to encrypt/decrypt
        let encrypted = beacon.encrypt(b"test").expect("encrypt failed");
        let decrypted = beacon.try_decrypt(&encrypted).expect("decrypt failed");
        assert!(decrypted.is_some());
    }

    #[test]
    fn test_beacon_ciphertext_has_timestamp() {
        let beacon = BeaconSeed::generate();
        let encrypted = beacon.encrypt(b"timestamped data").expect("encrypt failed");

        // Timestamp should be recent (within last minute)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        assert!(encrypted.timestamp <= now);
        assert!(encrypted.timestamp > now - 60, "Timestamp too old");
    }
}
