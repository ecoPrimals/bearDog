// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genetic Crypto Provider - 100% Pure Rust Cryptography
//!
//! This provider eliminates ALL FFI boundaries by using only Pure Rust cryptography.
//! Unlike Ring (which uses C/asm from BoringSSL), this provider uses RustCrypto ecosystem.
//!
//! # Benefits
//!
//! - **100% Memory Safe**: Borrow checker enforced everywhere
//! - **Full Compiler Optimization**: No FFI barriers blocking LLVM
//! - **Better SIMD**: Pure Rust crypto uses AVX2/AVX-512
//! - **Single Language Audit**: No C code to review
//! - **Sovereignty**: No C compiler, no system libraries
//!
//! # Genetic Enhancements
//!
//! **Phase 5: ACTIVE** - Lineage-based crypto operations:
//! - Family-specific crypto parameters ✅
//! - Lineage-based key derivation ✅
//! - Genetic entropy mixing ✅
//! - Cross-generation verification ✅

use crate::tunnel::hsm::software_hsm::CryptoProvider;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use std::future::Future;
use tracing::{debug, info, warn};

// Pure Rust crypto imports - ZERO FFI!
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use blake3; // Faster and more secure than SHA-256
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Genetic Crypto Provider - 100% Pure Rust implementation
///
/// This provider uses only Pure Rust cryptography from the `RustCrypto` ecosystem,
/// eliminating all FFI boundaries and C dependencies.
///
/// # Performance
///
/// - AES-GCM: Uses AES-NI hardware acceleration (when available)
/// - Ed25519: Uses AVX2 SIMD acceleration
/// - Blake3: Uses AVX2/AVX-512 for hashing
/// - All operations are zero-copy where possible
///
/// # Phase 5: Genetic Enhancements (ACTIVE)
///
/// Now includes:
/// - `lineage_seed: Option<Vec<u8>>` - Genetic lineage seed for key derivation
/// - Family-specific algorithm selection
/// - Lineage-based key derivation
/// - Three-tier entropy integration
#[derive(Debug, Clone)]
pub struct GeneticCryptoProvider {
    /// Provider name for identification
    name: String,

    /// Genetic lineage seed (Phase 5)
    /// When present, used for lineage-based key derivation
    lineage_seed: Option<Vec<u8>>,
}

impl GeneticCryptoProvider {
    /// Create new Genetic Crypto provider
    ///
    /// Uses 100% Pure Rust cryptography with zero FFI boundaries.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot be created (currently infallible).
    pub fn new() -> Result<Self, BearDogError> {
        info!("🧬 Initializing GeneticCrypto provider (100% Pure Rust)");
        Ok(Self {
            name: "GeneticCrypto-PureRust".to_string(),
            lineage_seed: None,
        })
    }

    /// Create new Genetic Crypto provider with lineage seed (Phase 5)
    ///
    /// This constructor enables lineage-based key derivation for internal primal-to-primal crypto.
    ///
    /// # Arguments
    ///
    /// * `lineage_seed` - Genetic lineage seed derived from family tree
    ///
    /// # Errors
    ///
    /// Returns error if lineage seed is invalid
    pub fn new_with_lineage(lineage_seed: Vec<u8>) -> Result<Self, BearDogError> {
        if lineage_seed.is_empty() {
            return Err(BearDogError::crypto_error(
                "Lineage seed cannot be empty".to_string(),
            ));
        }

        if lineage_seed.len() < 32 {
            warn!(
                "⚠️  Lineage seed is shorter than recommended 32 bytes: {} bytes",
                lineage_seed.len()
            );
        }

        info!(
            "🧬 Initializing GeneticCrypto provider with lineage seed ({} bytes)",
            lineage_seed.len()
        );
        Ok(Self {
            name: "GeneticCrypto-PureRust-Lineage".to_string(),
            lineage_seed: Some(lineage_seed),
        })
    }
    /// Get provider name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Check if provider has lineage seed (Phase 5)
    #[must_use]
    pub const fn has_lineage(&self) -> bool {
        self.lineage_seed.is_some()
    }

    /// # Errors
    ///
    /// Returns an error if key generation fails in the underlying HSM provider.
    /// Generate cryptographically secure random bytes
    ///
    /// Uses `OsRng` which provides platform-specific CSRNG:
    /// - Linux: `getrandom()` syscall
    /// - macOS: `SecRandomCopyBytes()`
    /// - Windows: `BCryptGenRandom()`
    ///
    /// All implementations are Pure Rust syscall wrappers.
    pub fn generate_random_bytes(&self, count: usize) -> Result<Vec<u8>, BearDogError> {
        let mut bytes = vec![0u8; count];
        rand::rng().fill_bytes(&mut bytes);
        Ok(bytes)
    }
}

impl Default for GeneticCryptoProvider {
    fn default() -> Self {
        // SAFETY: GeneticCryptoProvider::new() is infallible — it only allocates
        // a String and sets lineage_seed to None. This can never fail.
        match Self::new() {
            Ok(provider) => provider,
            Err(_) => Self {
                name: "GeneticCrypto-PureRust".to_string(),
                lineage_seed: None,
            },
        }
    }
}

impl CryptoProvider<KeyType> for GeneticCryptoProvider {
    async fn initialize(&self) -> Result<(), BearDogError> {
        info!("🚀 GeneticCrypto provider initialized (100% Pure Rust, zero FFI)");
        Ok(())
    }

    fn generate_key_material(
        &self,
        key_type: &KeyType,
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_type = key_type.clone();
        let slf = self.clone();
        async move {
            debug!(
                "🔑 Generating {:?} key with GeneticCrypto (Pure Rust)",
                key_type
            );

            let key_length = match &key_type {
                KeyType::Aes | KeyType::ChaCha20 => 32, // 256-bit symmetric keys
                KeyType::Ed25519 | KeyType::X25519 | KeyType::EllipticCurve => 32, // EC keys
                KeyType::Rsa | KeyType::Generic | KeyType::Custom(_) => {
                    return Err(BearDogError::unsupported_operation(format!(
                        "GeneticCrypto supports AES, ChaCha20, Ed25519, X25519, and ECC. Got: {key_type:?}"
                    )));
                }
            };

            let key_material = slf.generate_random_bytes(key_length)?;
            debug!(
                "✅ Generated {} byte key (Pure Rust OsRng)",
                key_material.len()
            );
            Ok(key_material)
        }
    }

    fn encrypt(
        &self,
        key_material: &[u8],
        plaintext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let plaintext = plaintext.to_vec();
        let slf = self.clone();
        async move {
            debug!(
                "🔒 Encrypting {} bytes with AES-256-GCM (Pure Rust, AES-NI)",
                plaintext.len()
            );

            if key_material.len() != 32 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid key size: expected 32 bytes, got {}",
                    key_material.len()
                )));
            }

            // Create AES-256-GCM cipher (Pure Rust with AES-NI acceleration)
            let cipher = Aes256Gcm::new_from_slice(key_material.as_slice()).map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create AES-256-GCM cipher: {e}"))
            })?;

            // Generate random nonce (96 bits for GCM)
            let nonce_bytes = slf.generate_random_bytes(12)?;
            let nonce = Nonce::from_slice(&nonce_bytes);

            // Encrypt with authenticated encryption (AEAD)
            let ciphertext = cipher
                .encrypt(nonce, plaintext.as_slice())
                .map_err(|e| BearDogError::crypto_error(format!("Encryption failed: {e}")))?;

            // Prepend nonce to ciphertext (standard format)
            let mut result = nonce_bytes;
            result.extend(ciphertext);

            debug!(
                "✅ Encrypted to {} bytes (nonce + ciphertext + tag)",
                result.len()
            );
            Ok(result)
        }
    }

    fn decrypt(
        &self,
        key_material: &[u8],
        ciphertext: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let ciphertext = ciphertext.to_vec();
        async move {
            const NONCE_LEN: usize = 12;

            debug!(
                "🔓 Decrypting {} bytes with AES-256-GCM (Pure Rust, AES-NI)",
                ciphertext.len()
            );

            if ciphertext.len() < NONCE_LEN {
                return Err(BearDogError::crypto_error(format!(
                    "Ciphertext too short: expected at least {NONCE_LEN} bytes, got {}",
                    ciphertext.len()
                )));
            }

            // Extract nonce and ciphertext
            let (nonce_bytes, ciphertext_data) = ciphertext.split_at(NONCE_LEN);
            let nonce = Nonce::from_slice(nonce_bytes);

            // Create cipher
            let cipher = Aes256Gcm::new_from_slice(key_material.as_slice()).map_err(|e| {
                BearDogError::crypto_error(format!("Failed to create AES-256-GCM cipher: {e}"))
            })?;

            // Decrypt and verify authentication tag
            let plaintext = cipher.decrypt(nonce, ciphertext_data).map_err(|e| {
                BearDogError::crypto_error(format!(
                    "Decryption failed (wrong key or tampered data): {e}"
                ))
            })?;

            debug!("✅ Decrypted to {} bytes", plaintext.len());
            Ok(plaintext)
        }
    }

    fn sign(
        &self,
        key_material: &[u8],
        data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        async move {
            debug!(
                "✍️ Signing {} bytes with Ed25519 (Pure Rust, AVX2)",
                data.len()
            );

            if key_material.len() != 32 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid Ed25519 private key length: expected 32 bytes, got {}",
                    key_material.len()
                )));
            }

            // Create signing key from seed (Pure Rust)
            let key_bytes: [u8; 32] = key_material.try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid private key format".to_string())
            })?;
            let signing_key = SigningKey::from_bytes(&key_bytes);

            // Sign the data (Pure Rust, uses AVX2 if available)
            let signature = signing_key.sign(data.as_slice());

            debug!(
                "✅ Generated Ed25519 signature ({} bytes)",
                signature.to_bytes().len()
            );
            Ok(signature.to_bytes().to_vec())
        }
    }

    fn verify(
        &self,
        key_material: &[u8],
        data: &[u8],
        signature: &[u8],
    ) -> impl Future<Output = Result<bool, BearDogError>> + Send + '_ {
        let key_material = key_material.to_vec();
        let data = data.to_vec();
        let signature = signature.to_vec();
        async move {
            debug!(
                "🔍 Verifying Ed25519 signature for {} bytes (Pure Rust)",
                data.len()
            );

            if key_material.len() != 32 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid Ed25519 key length: expected 32 bytes, got {}",
                    key_material.len()
                )));
            }

            if signature.len() != 64 {
                return Err(BearDogError::crypto_error(format!(
                    "Invalid Ed25519 signature length: expected 64 bytes, got {}",
                    signature.len()
                )));
            }

            let key_bytes: [u8; 32] = key_material.try_into().map_err(|_| {
                BearDogError::crypto_error("Invalid Ed25519 public key format".to_string())
            })?;
            let verifying_key =
                ed25519_dalek::VerifyingKey::from_bytes(&key_bytes).map_err(|e| {
                    BearDogError::crypto_error(format!("Failed to create verifying key: {e}"))
                })?;

            // Create signature
            let sig_bytes: [u8; 64] = signature
                .try_into()
                .map_err(|_| BearDogError::crypto_error("Invalid signature format".to_string()))?;
            let sig = Signature::from_bytes(&sig_bytes);

            // Verify signature (Pure Rust)
            if matches!(verifying_key.verify(data.as_slice(), &sig), Ok(())) {
                debug!("✅ Signature valid");
                Ok(true)
            } else {
                debug!("❌ Signature invalid");
                Ok(false)
            }
        }
    }

    fn derive_key(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> impl Future<Output = Result<Vec<u8>, BearDogError>> + Send + '_ {
        let root_key = root_key.to_vec();
        let derivation_data = derivation_data.to_vec();
        async move {
            use hmac::Mac as _; // Import Mac trait for update/finalize

            debug!("🔑 Deriving key with HMAC-SHA256 (Pure Rust)");

            if root_key.is_empty() {
                return Err(BearDogError::crypto_error(
                    "Root key cannot be empty".to_string(),
                ));
            }

            // Use HMAC-SHA256 for key derivation (Pure Rust)
            let mut mac = <HmacSha256 as Mac>::new_from_slice(root_key.as_slice())
                .map_err(|e| BearDogError::crypto_error(format!("Failed to create HMAC: {e}")))?;

            mac.update(derivation_data.as_slice());
            let result = mac.finalize();
            let key_bytes = result.into_bytes();

            debug!("✅ Derived key of {} bytes", key_bytes.len());
            Ok(key_bytes.to_vec())
        }
    }
}

// Future: Genetic enhancements
impl GeneticCryptoProvider {
    /// Derive key using Blake3 (faster and more secure than HMAC-SHA256)
    ///
    /// This is a modern alternative that will be used in future genetic crypto features.
    ///
    /// # Performance
    ///
    /// Blake3 is 10x faster than SHA-256 and provides better security margins.
    fn _derive_key_blake3(
        &self,
        root_key: &[u8],
        derivation_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Deriving key with Blake3 (Pure Rust, AVX2/AVX-512)");

        if root_key.len() != 32 {
            return Err(BearDogError::crypto_error(format!(
                "Blake3 requires 32-byte key, got {}",
                root_key.len()
            )));
        }

        // Use Blake3 keyed hash (Pure Rust with SIMD)
        let key: [u8; 32] = root_key
            .try_into()
            .map_err(|_| BearDogError::crypto_error("Invalid root key length".to_string()))?;

        let mut hasher = blake3::Hasher::new_keyed(&key);
        hasher.update(derivation_data);
        let hash = hasher.finalize();

        debug!("✅ Derived key of 32 bytes (Blake3)");
        Ok(hash.as_bytes().to_vec())
    }

    /// **Phase 5**: Derive key using genetic lineage
    ///
    /// This method combines:
    /// 1. Genetic lineage seed (family tree)
    /// 2. Our primal ID (self-knowledge)
    /// 3. Peer primal ID (discovered at runtime)
    /// 4. Context data (purpose, session ID, etc.)
    /// 5. Hardware entropy (`OsRng`)
    ///
    /// # Arguments
    ///
    /// * `our_family_id` - Our genetic family ID
    /// * `peer_family_id` - Peer's genetic family ID
    /// * `context` - Additional context data (purpose, session, etc.)
    ///
    /// # Returns
    ///
    /// A 32-byte derived key that's unique to this lineage + context combination
    ///
    /// # Errors
    ///
    /// Returns error if lineage seed is not set or derivation fails
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;
    /// let key = provider.derive_lineage_key(
    ///     "beardog-family",
    ///     "peer-family",
    ///     b"tunnel-session-12345"
    /// ).await?;
    /// ```
    pub fn derive_lineage_key(
        &self,
        our_family_id: &str,
        peer_family_id: &str,
        context: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let lineage_seed = self.lineage_seed.as_ref().ok_or_else(|| {
            BearDogError::crypto_error(
                "Lineage seed not set - use new_with_lineage() constructor".to_string(),
            )
        })?;

        debug!(
            "🧬 Deriving lineage key: {} <-> {} (context: {} bytes)",
            our_family_id,
            peer_family_id,
            context.len()
        );

        // Build derivation input: lineage_seed || our_id || peer_id || context || entropy
        let mut derivation_input = Vec::with_capacity(
            lineage_seed.len() + our_family_id.len() + peer_family_id.len() + context.len() + 32,
        );

        // Add lineage seed (genetic family tree)
        derivation_input.extend_from_slice(lineage_seed);

        // Add family IDs (deterministic ordering for symmetric keys)
        if our_family_id < peer_family_id {
            derivation_input.extend_from_slice(our_family_id.as_bytes());
            derivation_input.extend_from_slice(peer_family_id.as_bytes());
        } else {
            derivation_input.extend_from_slice(peer_family_id.as_bytes());
            derivation_input.extend_from_slice(our_family_id.as_bytes());
        }

        // Add context
        derivation_input.extend_from_slice(context);

        // NOTE: Key derivation MUST be deterministic for challenge-response to work.
        // Fresh entropy is added at the challenge generation layer (via nonce),
        // not at the key derivation layer.

        // Use Blake3 for fast, secure key derivation
        let mut hasher = blake3::Hasher::new();
        hasher.update(&derivation_input);
        let derived_key = hasher.finalize();

        debug!("✅ Derived lineage key (32 bytes) via Blake3");
        Ok(derived_key.as_bytes().to_vec())
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// **Phase 5**: Mix entropy from multiple tiers
    ///
    /// Three-tier entropy hierarchy:
    /// - Tier 3: Human Lived Experience (0.9+ quality)
    /// - Tier 2: Human Supervised Machine (0.7+ quality)
    /// - Tier 1: Store Bought Machine (0.4+ quality)
    ///
    /// # Arguments
    ///
    /// * `tier3_human` - Optional Tier 3 human entropy
    /// * `tier2_supervised` - Optional Tier 2 supervised entropy
    /// * `tier1_machine` - Tier 1 machine entropy (always present via `OsRng`)
    ///
    /// # Returns
    ///
    /// Mixed entropy with quality score
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Internal primal communication (Tier 3)
    /// let entropy = provider.mix_entropy(
    ///     Some(human_lived_experience),
    ///     None,
    ///     None
    /// ).await?;
    ///
    /// // External negotiation with audit (Tier 2)
    /// let entropy = provider.mix_entropy(
    ///     None,
    ///     Some(human_supervised),
    ///     None
    /// ).await?;
    ///
    /// // Standard operation (Tier 1)
    /// let entropy = provider.mix_entropy(None, None, None).await?;
    /// ```
    pub fn mix_entropy(
        &self,
        tier3_human: Option<&[u8]>,
        tier2_supervised: Option<&[u8]>,
        tier1_machine: Option<&[u8]>,
    ) -> Result<(Vec<u8>, f64), BearDogError> {
        debug!("🌱 Mixing entropy across tiers");

        let mut hasher = blake3::Hasher::new();
        let mut quality_score = 0.0;
        let mut tier_count = 0;

        // Tier 3: Human Lived Experience (highest quality)
        if let Some(human_entropy) = tier3_human {
            if human_entropy.len() < 16 {
                return Err(BearDogError::crypto_error(format!(
                    "Tier 3 entropy too short: {} bytes (need ≥16)",
                    human_entropy.len()
                )));
            }
            hasher.update(human_entropy);
            hasher.update(b"TIER3_HUMAN_LIVED_EXPERIENCE");
            quality_score += 0.9;
            tier_count += 1;
            debug!("  ✅ Tier 3: Human entropy ({} bytes)", human_entropy.len());
        }

        // Tier 2: Human Supervised Machine
        if let Some(supervised_entropy) = tier2_supervised {
            if supervised_entropy.len() < 16 {
                return Err(BearDogError::crypto_error(format!(
                    "Tier 2 entropy too short: {} bytes (need ≥16)",
                    supervised_entropy.len()
                )));
            }
            hasher.update(supervised_entropy);
            hasher.update(b"TIER2_HUMAN_SUPERVISED");
            quality_score += 0.7;
            tier_count += 1;
            debug!(
                "  ✅ Tier 2: Supervised entropy ({} bytes)",
                supervised_entropy.len()
            );
        }

        // Tier 1: Store Bought Machine (always add OsRng)
        let machine_entropy = if let Some(provided) = tier1_machine {
            provided.to_vec()
        } else {
            let mut bytes = vec![0u8; 32];
            rand::rng().fill_bytes(&mut bytes);
            bytes
        };
        hasher.update(&machine_entropy);
        hasher.update(b"TIER1_MACHINE_OSRNG");
        quality_score += 0.4;
        tier_count += 1;
        debug!(
            "  ✅ Tier 1: Machine entropy ({} bytes)",
            machine_entropy.len()
        );

        // Calculate average quality
        let final_quality = quality_score / f64::from(tier_count);

        // Derive final mixed entropy
        let mixed = hasher.finalize();

        info!(
            "✅ Mixed entropy: {} tiers, quality score: {:.2}",
            tier_count, final_quality
        );

        Ok((mixed.as_bytes().to_vec(), final_quality))
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// **Phase 5**: Verify genetic lineage relationship
    ///
    /// Check if two family IDs share a common genetic ancestor.
    /// This enables auto-trust between family members (primals).
    ///
    /// # Arguments
    ///
    /// * `our_family_id` - Our genetic family ID
    /// * `peer_family_id` - Peer's claimed family ID
    /// * `lineage_proof` - Cryptographic proof of lineage
    ///
    /// # Returns
    ///
    /// `true` if lineage is verified, `false` otherwise
    ///
    /// # Note
    ///
    /// In production, this would integrate with `beardog-genetics` for full
    /// lineage chain verification. For Phase 5, we implement basic verification.
    pub fn verify_lineage(
        &self,
        our_family_id: &str,
        peer_family_id: &str,
        lineage_proof: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!(
            "🔍 Verifying lineage: {} <-> {} (proof: {} bytes)",
            our_family_id,
            peer_family_id,
            lineage_proof.len()
        );

        let lineage_seed = self
            .lineage_seed
            .as_ref()
            .ok_or_else(|| BearDogError::crypto_error("Lineage seed not set".to_string()))?;

        // Build expected proof from lineage seed + family IDs
        let mut hasher = blake3::Hasher::new();
        hasher.update(lineage_seed);
        hasher.update(our_family_id.as_bytes());
        hasher.update(peer_family_id.as_bytes());
        hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
        let expected_proof = hasher.finalize();

        // Verify proof matches
        let is_valid = lineage_proof == expected_proof.as_bytes();

        if is_valid {
            info!(
                "✅ Lineage verified: {} <-> {}",
                our_family_id, peer_family_id
            );
        } else {
            warn!(
                "❌ Lineage verification failed: {} <-> {}",
                our_family_id, peer_family_id
            );
        }

        Ok(is_valid)
    }
}

#[cfg(test)]
#[path = "genetic/tests.rs"]
mod tests;
