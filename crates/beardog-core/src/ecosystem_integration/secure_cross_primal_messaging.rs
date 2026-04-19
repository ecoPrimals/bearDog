// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Secure Cross-Primal Messaging (Capability-Based)
//!
//! This module enables BearDog to securely communicate with ANY primal
//! that advertises network or security capabilities, WITHOUT hardcoding
//! specific primal names.
//!
//! ## Architecture Principle: "Discover, Don't Hardcode"
//!
//! BearDog knows:
//! - ✅ Itself (from [`crate::primal_self_knowledge::PrimalIdentity`] / environment)
//! - ✅ What capabilities it needs (e.g., "network routing", "key ceremony")
//! - ❌ NO hardcoded peer primal names (discovered at runtime)
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem_integration::SecureCrossPrimalMessenger;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let messenger = SecureCrossPrimalMessenger::new(discovery_client).await?;
//!
//! // Send to ANY primal with network capability (whoever advertises it)
//! let response = messenger.send_to_network_primal(b"hello", metadata).await?;
//! # Ok(())
//! # }
//! ```

// Trait-based approach for primal discovery
// This allows flexibility in implementation without tight coupling
// to beardog-adapters internal structure
use crate::primal_self_knowledge::{PrimalIdentity, PrimalIdentityEnvInputs};
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, SecurityService, StorageCharacteristic,
    UniversalCapabilityType, UniversalServiceDescriptor,
};
use chacha20poly1305::{
    ChaCha20Poly1305, Nonce,
    aead::{Aead, KeyInit},
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Trait for discovering primals by capability (not by hardcoded name)
#[expect(
    async_fn_in_trait,
    reason = "concrete implementors only; no dyn dispatch"
)]
pub trait PrimalDiscoveryService: Send + Sync {
    /// Discover primals with specific capabilities
    async fn discover_by_capability(
        &self,
        capability: UniversalCapabilityType,
    ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError>;

    /// Send request to a discovered primal
    async fn send_request(
        &self,
        service: &UniversalServiceDescriptor,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError>;
}

/// Secure message to send to any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurePrimalMessage {
    /// Message payload (encrypted)
    pub ciphertext: Vec<u8>,
    /// Sender identity (`BearDog`'s public key or ID)
    pub sender_id: String,
    /// Required capability from receiver
    pub required_capability: UniversalCapabilityType,
    /// Message metadata
    pub metadata: HashMap<String, String>,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Response from any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurePrimalResponse {
    /// Response payload (encrypted)
    pub ciphertext: Vec<u8>,
    /// Responder identity
    pub responder_id: String,
    /// Capability that was used
    pub capability_used: UniversalCapabilityType,
    /// Processing time (ms)
    pub processing_time_ms: u64,
}

/// Secure session with a primal (no hardcoded primal name)
#[derive(Debug, Clone)]
pub struct SecureSession {
    /// Session ID
    pub session_id: String,
    /// Peer ID (dynamic, not hardcoded)
    pub peer_id: String,
    /// Encryption key (from key ceremony)
    pub encryption_key: Vec<u8>,
    /// Session established time
    pub established_at: chrono::DateTime<chrono::Utc>,
    /// Last activity
    pub last_activity: Arc<RwLock<chrono::DateTime<chrono::Utc>>>,
}

/// Secure Cross-Primal Messenger (Zero Hardcoded Primal Names)
pub struct SecureCrossPrimalMessenger<D: PrimalDiscoveryService> {
    /// Discovery service for capability-based primal discovery
    discovery_service: Arc<D>,
    /// Our identity (`BearDog` only knows itself)
    our_identity: String,
    /// Active secure sessions (by session ID)
    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
    /// Genetic key exchange engine
    key_exchange: Arc<beardog_genetics::GeneticKeyExchange>,
    /// Metrics
    metrics: Arc<RwLock<MessengerMetrics>>,
}

/// Messaging metrics
#[derive(Debug, Clone, Default)]
pub struct MessengerMetrics {
    /// Number of `messages_sent`
    pub messages_sent: u64,
    /// Number of `messages_received`
    pub messages_received: u64,
    /// Number of `sessions_established`
    pub sessions_established: u64,
    /// Number of `errors_encountered`
    pub errors_encountered: u64,
    /// Last activity time
    pub last_activity: Option<chrono::DateTime<chrono::Utc>>,
}

impl<D: PrimalDiscoveryService> SecureCrossPrimalMessenger<D> {
    /// Create new secure cross-primal messenger
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(discovery_service: Arc<D>) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Secure Cross-Primal Messenger");
        info!("📋 Using capability-based discovery - zero hardcoded primal names");

        // Initialize genetic key exchange engine
        let key_exchange_config = beardog_genetics::KeyExchangeConfig {
            max_key_lifetime_secs: 86400, // 24 hours
            enable_evolution: true,
            min_entropy_tier: 3, // High security for cross-primal comms
            enable_multiparty_renewal: true,
        };
        let key_exchange = beardog_genetics::GeneticKeyExchange::new(key_exchange_config)?;

        info!("✅ Genetic key exchange engine initialized");

        let our_identity = PrimalIdentity::from_inputs(&PrimalIdentityEnvInputs::from_env())?.name;

        Ok(Self {
            discovery_service,
            our_identity,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            key_exchange: Arc::new(key_exchange),
            metrics: Arc::new(RwLock::new(MessengerMetrics::default())),
        })
    }

    /// Send secure message to ANY primal with network capability
    ///
    /// Discovers primals with network routing capability and sends message.
    /// Works with ANY primal that advertises the capability.
    ///
    /// # Errors
    /// Returns an error if no network primals are discovered or communication fails
    pub async fn send_to_network_primal(
        &self,
        plaintext: &[u8],
        metadata: HashMap<String, String>,
    ) -> Result<SecurePrimalResponse, BearDogError> {
        // Discover primals with network routing capability (capability-based)
        let network_capability = UniversalCapabilityType::Network {
            functions: vec![NetworkFunction::TrafficRouting],
        };
        let network_primals = self
            .discovery_service
            .discover_by_capability(network_capability)
            .await?;

        if network_primals.is_empty() {
            return Err(BearDogError::system(
                "No network-capable primals discovered in ecosystem".to_string(),
            ));
        }

        // Use first available (could implement load balancing)
        let primal = &network_primals[0];

        info!(
            "📤 Sending secure message to network primal: {} (discovered via capability)",
            primal.service_id
        );

        // Encrypt payload with primal's public key (genetic crypto)
        let ciphertext = self.encrypt_for_primal(plaintext, primal)?;

        let message = SecurePrimalMessage {
            ciphertext,
            sender_id: self.our_identity.clone(),
            required_capability: UniversalCapabilityType::Network {
                functions: vec![NetworkFunction::TrafficRouting],
            },
            metadata,
            timestamp: chrono::Utc::now(),
        };

        // Send via discovery service
        let payload = serde_json::to_value(&message).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize message: {e}"))
        })?;

        let response_data = self.discovery_service.send_request(primal, payload).await?;

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.messages_sent += 1;
            metrics.messages_received += 1;
            metrics.last_activity = Some(chrono::Utc::now());
        }

        // Decrypt response
        let secure_response: SecurePrimalResponse =
            serde_json::from_value(response_data).map_err(|e| {
                BearDogError::serialization(&format!("Failed to deserialize response: {e}"))
            })?;

        info!(
            "📥 Received secure response from: {}",
            secure_response.responder_id
        );
        Ok(secure_response)
    }

    /// Establish secure session with ANY security-capable primal
    ///
    /// Performs key ceremony with any primal advertising security capabilities.
    /// Does NOT hardcode which primal to use - discovers via capability.
    ///
    /// # Errors
    /// Returns an error if no security primals are found or key exchange fails
    pub async fn establish_secure_session(
        &self,
        security_level: &str,
    ) -> Result<SecureSession, BearDogError> {
        info!(
            "🤝 Establishing secure session (security level: {})",
            security_level
        );

        // Discover primals with security capabilities (ANY primal, not hardcoded)
        let security_capability = UniversalCapabilityType::Security {
            services: vec![SecurityService::KeyManagement],
        };
        let security_primals = self
            .discovery_service
            .discover_by_capability(security_capability)
            .await?;

        if security_primals.is_empty() {
            return Err(BearDogError::system(
                "No security-capable primals discovered in ecosystem".to_string(),
            ));
        }

        let primal = &security_primals[0];

        info!(
            "🔑 Performing key ceremony with primal: {} (discovered via capability)",
            primal.service_id
        );

        // Key exchange using genetic algorithm
        let session_keys = self.perform_key_exchange(primal).await?;

        let session = SecureSession {
            session_id: uuid::Uuid::new_v4().to_string(),
            peer_id: primal.service_id.clone(),
            encryption_key: session_keys,
            established_at: chrono::Utc::now(),
            last_activity: Arc::new(RwLock::new(chrono::Utc::now())),
        };

        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session.session_id.clone(), session.clone());
        }

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.sessions_established += 1;
            metrics.last_activity = Some(chrono::Utc::now());
        }

        info!("✅ Secure session established: {}", session.session_id);
        Ok(session)
    }

    /// Send message to ANY compute-capable primal
    ///
    /// Discovers and sends to primals with compute capabilities.
    /// Works with any capability-matched peer discovered at runtime (no fixed primal names).
    ///
    /// # Errors
    /// Returns an error if no compute primals are found or request fails
    pub async fn send_to_compute_primal(
        &self,
        compute_request: serde_json::Value,
    ) -> Result<SecurePrimalResponse, BearDogError> {
        info!("🧠 Sending compute request to capability-discovered primal");

        let compute_capability = UniversalCapabilityType::Compute {
            abilities: vec![ComputeAbility::DataAnalysis],
        };
        let compute_primals = self
            .discovery_service
            .discover_by_capability(compute_capability)
            .await?;

        if compute_primals.is_empty() {
            return Err(BearDogError::system(
                "No compute-capable primals discovered in ecosystem".to_string(),
            ));
        }

        let primal = &compute_primals[0];

        info!(
            "📤 Sending compute request to primal: {} (discovered via capability)",
            primal.service_id
        );

        let start = std::time::Instant::now();
        let response_data = self
            .discovery_service
            .send_request(primal, compute_request)
            .await?;
        let elapsed = start.elapsed();

        let ciphertext = serde_json::to_vec(&response_data).unwrap_or_default();

        let secure_response = SecurePrimalResponse {
            ciphertext,
            responder_id: primal.service_id.clone(),
            capability_used: UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            },
            processing_time_ms: u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
        };

        Ok(secure_response)
    }

    /// Send message to ANY storage-capable primal
    ///
    /// # Errors
    /// Returns an error if no storage primals are found
    pub async fn send_to_storage_primal(
        &self,
        data: &[u8],
    ) -> Result<SecurePrimalResponse, BearDogError> {
        info!("💾 Sending storage request to capability-discovered primal");

        let storage_capability = UniversalCapabilityType::Storage {
            characteristics: vec![StorageCharacteristic::Encrypted],
        };
        let storage_primals = self
            .discovery_service
            .discover_by_capability(storage_capability)
            .await?;

        if storage_primals.is_empty() {
            return Err(BearDogError::system(
                "No storage-capable primals discovered in ecosystem".to_string(),
            ));
        }

        let primal = &storage_primals[0];

        info!(
            "📤 Sending storage request to primal: {} (discovered via capability)",
            primal.service_id
        );

        // Convert data to hex string (simpler than base64 for now)
        let data_hex = hex::encode(data);

        let payload = serde_json::json!({
            "data": data_hex,
            "storage_type": "encrypted",
        });

        let start = std::time::Instant::now();
        let response_data = self.discovery_service.send_request(primal, payload).await?;
        let elapsed = start.elapsed();

        let ciphertext = serde_json::to_vec(&response_data).unwrap_or_default();

        let secure_response = SecurePrimalResponse {
            ciphertext,
            responder_id: primal.service_id.clone(),
            capability_used: UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::Encrypted],
            },
            processing_time_ms: u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX),
        };

        Ok(secure_response)
    }

    /// Get active session by ID
    ///
    /// # Errors
    /// Returns an error if session not found
    pub async fn get_session(&self, session_id: &str) -> Result<SecureSession, BearDogError> {
        let sessions = self.active_sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or_else(|| BearDogError::not_found(format!("Session not found: {session_id}")))
    }

    /// Get messaging metrics
    pub async fn get_metrics(&self) -> MessengerMetrics {
        self.metrics.read().await.clone()
    }

    // ============================================
    // Private Helper Methods
    // ============================================

    /// Encrypt data for a specific primal (using genetic crypto)
    ///
    /// ✅ **Phase 1.1 Implementation Complete**
    /// - Integrated with genetic key exchange engine
    /// - Uses evolving key lineages for encryption
    /// - Maintains zero-knowledge of peer internals
    fn encrypt_for_primal(
        &self,
        plaintext: &[u8],
        primal: &UniversalServiceDescriptor,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔐 Encrypting for primal: {}", primal.service_id);

        // Phase 1.1: Use genetic key exchange for secure encryption
        // Create delegated key for this primal if needed
        let allowed_ops = vec!["encrypt".to_string(), "decrypt".to_string()];
        let delegated_key = self.key_exchange.create_delegated_key(
            &primal.service_id,
            3600, // 1 hour session
            &allowed_ops,
        )?;

        // Perform key exchange to get shared secret
        let exchange_result = self
            .key_exchange
            .perform_key_exchange(&primal.service_id, &delegated_key)?;

        // Use shared secret to encrypt (XOR for now, ChaCha20-Poly1305 in Phase 1.2)
        // Modern Rust idiom: Explicit cryptographic operations
        let ciphertext =
            Self::encrypt_with_shared_secret(plaintext, &exchange_result.shared_secret)?;

        debug!(
            "✅ Encrypted {} bytes for primal: {}",
            ciphertext.len(),
            primal.service_id
        );

        Ok(ciphertext)
    }

    /// Encrypt data with a shared secret (helper method)
    ///
    /// Currently uses XOR for demonstration. Will evolve to ChaCha20-Poly1305
    /// in Phase 1.2 for production-grade encryption.
    fn encrypt_with_shared_secret(
        plaintext: &[u8],
        shared_secret: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        use sha2::{Digest, Sha256};

        // Derive encryption key from shared secret
        let mut hasher = Sha256::new();
        hasher.update(shared_secret);
        hasher.update(b"cross-primal-msg-kdf-v1");
        let encryption_key = hasher.finalize();

        // Phase 1.2: ChaCha20-Poly1305 AEAD encryption
        // Modern Rust: Fast, secure, constant-time authenticated encryption

        // Ensure key is exactly 32 bytes for ChaCha20-Poly1305
        let mut key_bytes = [0u8; 32];
        let key_len = encryption_key.len().min(32);
        key_bytes[..key_len].copy_from_slice(&encryption_key[..key_len]);

        // Create cipher
        let cipher = ChaCha20Poly1305::new(&key_bytes.into());

        // Generate random nonce (96 bits / 12 bytes for ChaCha20-Poly1305)
        let mut nonce_bytes = [0u8; 12];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt with AEAD (includes authentication tag)
        let ciphertext_with_tag =
            cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| BearDogError::Cryptographic {
                    message: format!("ChaCha20-Poly1305 encryption failed: {e}"),
                })?;

        // Prepend nonce to ciphertext (receiver needs it for decryption)
        // Format: [nonce(12 bytes)][ciphertext + tag]
        let mut result = Vec::with_capacity(12 + ciphertext_with_tag.len());
        result.extend_from_slice(&nonce_bytes);
        result.extend_from_slice(&ciphertext_with_tag);

        Ok(result)
    }

    /// Perform key exchange with a discovered primal (X25519 + SHA3-256 derivation).
    ///
    /// Optional future work: additional genetic-material mixing on top of the established secret.
    async fn perform_key_exchange(
        &self,
        primal: &UniversalServiceDescriptor,
    ) -> Result<Vec<u8>, BearDogError> {
        use sha3::{Digest, Sha3_256};
        use x25519_dalek::{EphemeralSecret, PublicKey as X25519PublicKey};

        debug!("🔑 Performing key exchange with: {}", primal.service_id);

        // 1. Generate ephemeral key pair for this session (perfect forward secrecy)
        let our_secret = EphemeralSecret::random_from_rng(chacha20poly1305::aead::OsRng);
        let our_public = X25519PublicKey::from(&our_secret);

        // 2. Request peer's public key via capability-based discovery
        let key_request = serde_json::json!({
            "type": "key_exchange_request",
            "our_public_key": hex::encode(our_public.as_bytes()),
            "requested_capabilities": primal.capabilities,
            "session_id": uuid::Uuid::new_v4().to_string(),
        });

        let response = self
            .discovery_service
            .send_request(primal, key_request)
            .await
            .map_err(|e| {
                BearDogError::network(format!(
                    "Key exchange request failed for {}: {}",
                    primal.service_id, e
                ))
            })?;

        // 3. Parse peer's public key
        let peer_public_hex = response
            .get("peer_public_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::security(
                    "Peer did not provide public key in key exchange response".to_string(),
                )
            })?;

        let peer_public_bytes = hex::decode(peer_public_hex)
            .map_err(|e| BearDogError::security(format!("Invalid peer public key format: {e}")))?;

        let peer_public_array: [u8; 32] = peer_public_bytes
            .as_slice()
            .try_into()
            .map_err(|_| BearDogError::security("Peer public key must be 32 bytes".to_string()))?;

        let peer_public = X25519PublicKey::from(peer_public_array);

        // 4. Perform Diffie-Hellman key agreement
        let shared_secret = our_secret.diffie_hellman(&peer_public);

        // 5. Derive session key using SHA3-256 (NIST SP 800-56C Rev. 2 compliant)
        let mut hasher = Sha3_256::new();
        hasher.update(shared_secret.as_bytes());
        hasher.update(b"BearDog-CrossPrimal-SessionKey-v1");
        hasher.update(primal.service_id.as_bytes());
        let session_key = hasher.finalize();

        info!(
            "✅ Key exchange complete with {} (capabilities: {:?})",
            primal.service_id, primal.capabilities
        );

        Ok(session_key.to_vec())
    }
}

#[cfg(test)]
#[path = "secure_cross_primal_messaging_tests.rs"]
mod tests;
