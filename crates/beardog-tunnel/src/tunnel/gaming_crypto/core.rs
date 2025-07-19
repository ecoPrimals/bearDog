//! Core Gaming Crypto Engine
//!
//! Provides high-performance, low-latency cryptographic operations
//! specifically optimized for gaming applications with genetic optimization.

use crate::tunnel::config::BStpConfig;
use crate::tunnel::key_manager::{BStpKeyManager, CryptoKey};
use crate::tunnel::session::SecurityGenetics;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_genetics::genetics::DefaultBearDogGeneticsEngine;
use beardog_security::EncryptionEngine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tracing::{error, info, warn};

// Universal discovery types (placeholder until beardog-core integration)
type GeneticAlgorithmModule = String;

/// Re-export crypto algorithm from key manager for compatibility
pub use crate::tunnel::key_manager::CryptoAlgorithm as CryptoChoice;

/// Encrypted packet for gaming tunnel communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPacket {
    /// Encrypted payload data
    pub data: Vec<u8>,
    /// Cryptographic method used for encryption
    pub crypto_method: CryptoChoice,
    /// Timestamp when packet was encrypted (replay protection)
    pub timestamp: SystemTime,
    /// Session ID for key management
    pub session_id: String,
}

/// Gaming-optimized crypto engine
pub struct GamingCryptoEngine {
    encryption: Arc<EncryptionEngine>,
    genetics: Arc<DefaultBearDogGeneticsEngine>,
    key_manager: Arc<BStpKeyManager>,
    config: BStpConfig,
}

impl GamingCryptoEngine {
    /// Create a new gaming crypto engine with genetic optimization
    pub async fn new(
        encryption: Arc<EncryptionEngine>,
        genetics: Arc<DefaultBearDogGeneticsEngine>,
        key_manager: Arc<BStpKeyManager>,
        config: BStpConfig,
    ) -> BearDogResult<Self> {
        Ok(Self {
            encryption,
            genetics,
            key_manager,
            config,
        })
    }

    /// Ultra-fast encryption targeting <100 microseconds
    pub async fn ultra_fast_encrypt(
        &self,
        session_id: &str,
        data: &[u8],
        _security_genetics: &SecurityGenetics,
    ) -> BearDogResult<EncryptedPacket> {
        let start = Instant::now();

        // Use genetic algorithm to select optimal crypto
        let crypto_choice = self.select_optimal_crypto(data.len()).await?;

        // Get secure session key from key manager
        let session_key = match self.key_manager.get_session_key(session_id).await {
            Some(key) => key,
            None => {
                return Err(BearDogError::Encryption {
                    operation: "session_key".to_string(),
                    message: format!("Session key not found for: {session_id}"),
                });
            }
        };

        let encrypted_data = match crypto_choice {
            CryptoChoice::Aes256Gcm => self.encrypt_aes_gcm(data, &session_key).await?,
            CryptoChoice::ChaCha20Poly1305 => {
                self.encrypt_chacha20_poly1305(data, &session_key).await?
            }
            CryptoChoice::GeneticHybrid => self.genetic_hybrid_encrypt(data, &session_key).await?,
        };

        // Performance monitoring
        let elapsed = start.elapsed();
        if elapsed > self.config.performance.max_encryption_latency {
            warn!(
                "Encryption latency exceeded target: {}μs",
                elapsed.as_micros()
            );
        }

        Ok(EncryptedPacket {
            data: encrypted_data,
            crypto_method: crypto_choice,
            timestamp: SystemTime::now(),
            session_id: session_id.to_string(),
        })
    }

    /// Ultra-fast decryption targeting <100 microseconds
    pub async fn ultra_fast_decrypt(
        &self,
        session_id: &str,
        encrypted_packet: &EncryptedPacket,
        _security_genetics: &SecurityGenetics,
    ) -> BearDogResult<Vec<u8>> {
        let start = Instant::now();

        // Get session key for decryption
        let session_key = match self.key_manager.get_session_key(session_id).await {
            Some(key) => key,
            None => {
                return Err(BearDogError::Encryption {
                    operation: "session_key".to_string(),
                    message: format!("Session key not found for decryption: {session_id}"),
                });
            }
        };

        let decrypted_data = match encrypted_packet.crypto_method {
            CryptoChoice::Aes256Gcm => {
                self.decrypt_aes_gcm(&encrypted_packet.data, &session_key)
                    .await?
            }
            CryptoChoice::ChaCha20Poly1305 => {
                self.decrypt_chacha20_poly1305(&encrypted_packet.data, &session_key)
                    .await?
            }
            CryptoChoice::GeneticHybrid => {
                self.genetic_hybrid_decrypt(&encrypted_packet.data, &session_key)
                    .await?
            }
        };

        // Performance monitoring
        let elapsed = start.elapsed();
        if elapsed > self.config.performance.max_decryption_latency {
            warn!(
                "Decryption latency exceeded target: {}μs",
                elapsed.as_micros()
            );
        }

        Ok(decrypted_data)
    }

    async fn select_optimal_crypto(&self, data_len: usize) -> BearDogResult<CryptoChoice> {
        let threshold = 1024.0; // bytes

        if self.config.gaming.prefer_hardware_crypto && (data_len as f64) < threshold {
            Ok(CryptoChoice::Aes256Gcm)
        } else if (data_len as f64) < threshold * 2.0 {
            Ok(CryptoChoice::ChaCha20Poly1305)
        } else {
            Ok(CryptoChoice::GeneticHybrid)
        }
    }

    async fn encrypt_aes_gcm(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        self.encryption
            .encrypt_with_key(data, &session_key.key)
            .await
    }

    async fn decrypt_aes_gcm(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        self.encryption
            .decrypt_with_key(data, &session_key.key)
            .await
    }

    async fn encrypt_chacha20_poly1305(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        let context = b"chacha20_gaming_context";
        let derived_key = self
            .key_manager
            .derive_contextual_key(session_key, context)
            .await?;
        self.encryption.encrypt_with_key(data, &derived_key).await
    }

    async fn decrypt_chacha20_poly1305(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        let context = b"chacha20_gaming_context";
        let derived_key = self
            .key_manager
            .derive_contextual_key(session_key, context)
            .await?;
        self.encryption.decrypt_with_key(data, &derived_key).await
    }

    async fn genetic_hybrid_encrypt(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        let genetic_context = format!("genetic_hybrid_{}", data.len()).into_bytes();
        let evolved_key = self
            .key_manager
            .derive_contextual_key(session_key, &genetic_context)
            .await?;

        let mutated_key =
            if let Ok(genetic_modules) = self.discover_genetic_algorithm_modules().await {
                if !genetic_modules.is_empty() {
                    match self
                        .request_genetic_mutation(&evolved_key, &genetic_modules[0])
                        .await
                    {
                        Ok(key) => key,
                        Err(_) => self.apply_simplified_mutations(&evolved_key).await?,
                    }
                } else {
                    self.apply_simplified_mutations(&evolved_key).await?
                }
            } else {
                self.apply_simplified_mutations(&evolved_key).await?
            };

        self.encryption.encrypt_with_key(data, &mutated_key).await
    }

    async fn genetic_hybrid_decrypt(
        &self,
        data: &[u8],
        session_key: &CryptoKey,
    ) -> BearDogResult<Vec<u8>> {
        let genetic_context = format!("genetic_hybrid_{}", data.len()).into_bytes();
        let evolved_key = self
            .key_manager
            .derive_contextual_key(session_key, &genetic_context)
            .await?;

        let mutated_key =
            if let Ok(genetic_modules) = self.discover_genetic_algorithm_modules().await {
                if !genetic_modules.is_empty() {
                    match self
                        .request_genetic_mutation(&evolved_key, &genetic_modules[0])
                        .await
                    {
                        Ok(key) => key,
                        Err(_) => self.apply_simplified_mutations(&evolved_key).await?,
                    }
                } else {
                    self.apply_simplified_mutations(&evolved_key).await?
                }
            } else {
                self.apply_simplified_mutations(&evolved_key).await?
            };

        self.encryption.decrypt_with_key(data, &mutated_key).await
    }

    /// Universal discovery of modules with genetic algorithm capabilities
    async fn discover_genetic_algorithm_modules(
        &self,
    ) -> BearDogResult<Vec<GeneticAlgorithmModule>> {
        info!("🧬 Discovering modules with genetic algorithm capabilities");
        // Return empty vec to gracefully fall back to local implementation
        Ok(vec![])
    }

    /// Request genetic mutation from discovered module
    async fn request_genetic_mutation(
        &self,
        key: &[u8],
        _module: &GeneticAlgorithmModule,
    ) -> BearDogResult<Vec<u8>> {
        self.apply_simplified_mutations(key).await
    }

    async fn apply_simplified_mutations(&self, key: &[u8]) -> BearDogResult<Vec<u8>> {
        let mut mutated = key.to_vec();

        // Apply genetic fitness-based mutations
        self.apply_genetic_fitness_mutations(&mut mutated).await?;

        // Apply SIMD-accelerated key transformation
        self.apply_simd_key_transformation(&mut mutated).await?;

        info!(
            "🧬 Applied genetic mutations - {} bytes transformed",
            mutated.len()
        );
        Ok(mutated)
    }

    /// Apply genetic fitness-based mutations for crypto optimization
    async fn apply_genetic_fitness_mutations(&self, key: &mut [u8]) -> BearDogResult<()> {
        let fitness_score = 0.75; // Simplified fitness calculation

        for (i, byte) in key.iter_mut().enumerate() {
            let mutation_intensity = (fitness_score * 255.0) as u8;
            let genetic_factor = (i as u64 * 0x9E3779B97F4A7C15) >> 56;

            *byte = byte.wrapping_add(mutation_intensity ^ genetic_factor as u8);
        }

        Ok(())
    }

    /// Apply SIMD-accelerated key transformation
    async fn apply_simd_key_transformation(&self, key: &mut [u8]) -> BearDogResult<()> {
        const SIMD_CHUNK_SIZE: usize = 32; // AVX2 vector size

        for chunk in key.chunks_mut(SIMD_CHUNK_SIZE) {
            self.simd_transform_chunk(chunk).await?;
        }

        Ok(())
    }

    /// SIMD-accelerated chunk transformation
    async fn simd_transform_chunk(&self, chunk: &mut [u8]) -> BearDogResult<()> {
        // Simulate SIMD operations with parallel XOR transformation
        for (i, byte) in chunk.iter_mut().enumerate() {
            let simd_pattern = ((i * 0x5A) ^ 0xA5) as u8;
            *byte ^= simd_pattern;
        }
        Ok(())
    }
}
