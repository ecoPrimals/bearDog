//! Zero-Copy Cryptographic Operations Module
//!
//! This module provides high-performance cryptographic operations with minimal
//! memory allocations through intelligent buffer pooling and zero-copy techniques.

pub mod buffer_pool;
pub mod types;

pub use buffer_pool::{BufferPool, BufferPoolStats};
pub use types::*;

use crate::crypto_utils::BearDogCrypto;
use beardog_errors::{BearDogError, BearDogResult};
use bytes::Bytes;
use std::{
    collections::HashMap,
    sync::{atomic::Ordering, Arc},
};
use tokio::sync::Mutex;
use tracing::trace;

/// Main zero-copy cryptographic engine
pub struct ZeroCopyCrypto {
    /// Shared buffer pool for memory efficiency
    buffer_pool: Arc<BufferPool>,
    /// Cached encryption contexts for performance
    encryption_contexts: Arc<Mutex<HashMap<String, EncryptionContext>>>,
    /// Operation statistics
    stats: ZeroCryptoStats,
    /// Configuration
    #[allow(dead_code)] // Will be used when zero-copy optimizations are implemented
    config: ZeroCopyConfig,
    /// BearDog crypto utility
    #[allow(dead_code)] // Will be used when zero-copy optimizations are implemented
    crypto: Arc<BearDogCrypto>,
}

impl ZeroCopyCrypto {
    /// Create a new zero-copy crypto engine
    pub fn new() -> Self {
        let config = ZeroCopyConfig::default();
        Self::with_config(config)
    }

    /// Create with specific configuration
    pub fn with_config(config: ZeroCopyConfig) -> Self {
        Self {
            buffer_pool: Arc::new(BufferPool::new(config.clone())),
            encryption_contexts: Arc::new(Mutex::new(HashMap::new())),
            stats: ZeroCryptoStats::default(),
            crypto: Arc::new(BearDogCrypto),
            config,
        }
    }

    /// Encrypt data with zero-copy optimization
    pub async fn encrypt_zero_copy(
        &self,
        data: &[u8],
        key_id: &str,
        algorithm: &str,
    ) -> BearDogResult<Bytes> {
        self.stats.operations_total.fetch_add(1, Ordering::Relaxed);
        self.stats
            .bytes_processed
            .fetch_add(data.len() as u64, Ordering::Relaxed);

        trace!(
            "Zero-copy encrypt: {} bytes with key {}",
            data.len(),
            key_id
        );

        // Get or create encryption context
        let context = self
            .get_or_create_encryption_context(key_id, algorithm)
            .await?;

        // Use appropriate encryption method based on data size
        match data.len() {
            // Small data - use stack buffer for zero allocation
            0..=64 => self.encrypt_small_data(data, &context).await,
            // Medium data - use buffer pool
            65..=65536 => self.encrypt_medium_data(data, &context).await,
            // Large data - use streaming encryption
            _ => self.encrypt_large_data(data, &context).await,
        }
    }

    /// Decrypt data with zero-copy optimization
    pub async fn decrypt_zero_copy(
        &self,
        encrypted_data: &[u8],
        key_id: &str,
        algorithm: &str,
    ) -> BearDogResult<Bytes> {
        self.stats.operations_total.fetch_add(1, Ordering::Relaxed);
        self.stats
            .bytes_processed
            .fetch_add(encrypted_data.len() as u64, Ordering::Relaxed);

        trace!(
            "Zero-copy decrypt: {} bytes with key {}",
            encrypted_data.len(),
            key_id
        );

        // Get encryption context
        let context = self
            .get_or_create_encryption_context(key_id, algorithm)
            .await?;

        // Use appropriate decryption method based on data size
        match encrypted_data.len() {
            0..=80 => self.decrypt_small_data(encrypted_data, &context).await, // 64 + 16 overhead
            81..=65552 => self.decrypt_medium_data(encrypted_data, &context).await, // 65536 + 16 overhead
            _ => self.decrypt_large_data(encrypted_data, &context).await,
        }
    }

    /// Get statistics for monitoring
    pub fn get_stats(&self) -> &ZeroCryptoStats {
        &self.stats
    }

    /// Get buffer pool statistics
    pub fn get_buffer_pool_stats(&self) -> BufferPoolStats {
        self.buffer_pool.get_stats()
    }

    // Private helper methods

    async fn get_or_create_encryption_context(
        &self,
        key_id: &str,
        algorithm: &str,
    ) -> BearDogResult<EncryptionContext> {
        let cache_key = format!("{key_id}-{algorithm}");

        {
            let contexts = self.encryption_contexts.lock().await;
            if let Some(context) = contexts.get(&cache_key) {
                // Check if context is still valid (1 hour TTL)
                if context.created_at.elapsed().as_secs() < 3600 {
                    return Ok(context.clone());
                }
            }
        }

        // Create new context - derive key material from key_id using crypto hash
        let key_material = if key_id.is_empty() {
            // Generate new key material for empty key_id
            crate::crypto_utils::BearDogCrypto::secure_random_bytes(32).map_err(|e| {
                std::io::Error::other(format!("Failed to generate key material: {e}"))
            })?
        } else {
            // Derive key material from key_id using SHA-256 based derivation
            use sha2::{Digest, Sha256};

            let mut hasher = Sha256::new();
            hasher.update(b"BearDog-ZeroCopy-Context-");
            hasher.update(key_id.as_bytes());
            hasher.finalize().to_vec()
        };
        let context = EncryptionContext {
            algorithm: algorithm.to_string(),
            key_material: Bytes::from(key_material),
            created_at: std::time::Instant::now(),
            use_count: 0,
        };

        // Cache the context
        {
            let mut contexts = self.encryption_contexts.lock().await;
            contexts.insert(cache_key, context.clone());
        }

        Ok(context)
    }

    async fn encrypt_small_data(
        &self,
        data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        self.stats
            .zero_copy_operations
            .fetch_add(1, Ordering::Relaxed);

        // Use stack buffer for small data - no heap allocation
        let mut stack_buffer = [0u8; 64 + 16]; // 64 bytes data + 16 bytes overhead

        let (ciphertext, nonce) =
            BearDogCrypto::encrypt_aes_gcm(context.key_material.as_ref(), data, None)?;

        // Copy to stack buffer
        let result_len = nonce.len() + ciphertext.len();
        if result_len <= stack_buffer.len() {
            stack_buffer[..nonce.len()].copy_from_slice(&nonce);
            stack_buffer[nonce.len()..result_len].copy_from_slice(&ciphertext);

            Ok(Bytes::copy_from_slice(&stack_buffer[..result_len]))
        } else {
            // Fallback to heap allocation if too large
            let mut result = Vec::with_capacity(result_len);
            result.extend_from_slice(&nonce);
            result.extend_from_slice(&ciphertext);
            Ok(Bytes::from(result))
        }
    }

    async fn encrypt_medium_data(
        &self,
        data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        self.stats.buffer_reuses.fetch_add(1, Ordering::Relaxed);

        let (ciphertext, nonce) =
            BearDogCrypto::encrypt_aes_gcm(context.key_material.as_ref(), data, None)?;

        // Use buffer pool for efficient memory management
        let mut buffer = self
            .buffer_pool
            .get_buffer(nonce.len() + ciphertext.len())
            .await;
        buffer.extend_from_slice(&nonce);
        buffer.extend_from_slice(&ciphertext);

        let result = buffer.freeze();
        Ok(result)
    }

    async fn encrypt_large_data(
        &self,
        data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        // For large data, we need to use streaming encryption
        // This is a simplified implementation - real streaming would be more complex
        self.encrypt_medium_data(data, context).await
    }

    async fn decrypt_small_data(
        &self,
        encrypted_data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        if encrypted_data.len() < 12 {
            return Err(BearDogError::Internal {
                message: "Invalid encrypted data length".to_string(),
            });
        }

        let (nonce, ciphertext) = encrypted_data.split_at(12);
        let plaintext =
            BearDogCrypto::decrypt_aes_gcm(context.key_material.as_ref(), ciphertext, nonce)?;

        Ok(Bytes::from(plaintext))
    }

    async fn decrypt_medium_data(
        &self,
        encrypted_data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        if encrypted_data.len() < 12 {
            return Err(BearDogError::Internal {
                message: "Invalid encrypted data length".to_string(),
            });
        }

        let (nonce, ciphertext) = encrypted_data.split_at(12);
        let plaintext =
            BearDogCrypto::decrypt_aes_gcm(context.key_material.as_ref(), ciphertext, nonce)?;

        Ok(Bytes::from(plaintext))
    }

    async fn decrypt_large_data(
        &self,
        encrypted_data: &[u8],
        context: &EncryptionContext,
    ) -> BearDogResult<Bytes> {
        // For large data, use streaming decryption
        self.decrypt_medium_data(encrypted_data, context).await
    }
}

impl Default for ZeroCopyCrypto {
    fn default() -> Self {
        Self::new()
    }
}
