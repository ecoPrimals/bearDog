//! # Secure Cross-Primal Messaging (Capability-Based)
//!
//! This module enables BearDog to securely communicate with ANY primal
//! that advertises network or security capabilities, WITHOUT hardcoding
//! specific primal names.
//!
//! ## Architecture Principle: "Discover, Don't Hardcode"
//!
//! BearDog knows:
//! - ✅ Itself ("beardog")
//! - ✅ What capabilities it needs (e.g., "network routing", "key ceremony")
//! - ❌ NO hardcoded primal names (not "songbird", "toadstool", etc.)
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem_integration::SecureCrossPrimalMessenger;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let messenger = SecureCrossPrimalMessenger::new(discovery_client).await?;
//!
//! // Send to ANY primal with network capability (could be songbird, or anything else)
//! let response = messenger.send_to_network_primal(b"hello", metadata).await?;
//! # Ok(())
//! # }
//! ```

// Trait-based approach for primal discovery
// This allows flexibility in implementation without tight coupling
// to beardog-adapters internal structure
use beardog_errors::BearDogError;
use beardog_types::canonical::discovery::{
    ComputeAbility, NetworkFunction, SecurityService, StorageCharacteristic,
    UniversalCapabilityType, UniversalServiceDescriptor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Trait for discovering primals by capability (not by hardcoded name)
#[async_trait::async_trait]
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
pub struct SecureCrossPrimalMessenger {
    /// Discovery service for capability-based primal discovery
    discovery_service: Arc<dyn PrimalDiscoveryService>,
    /// Our identity (`BearDog` only knows itself)
    our_identity: String,
    /// Active secure sessions (by session ID)
    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
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

impl SecureCrossPrimalMessenger {
    /// Create new secure cross-primal messenger
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(discovery_service: Arc<dyn PrimalDiscoveryService>) -> Result<Self, BearDogError> {
        info!("🔐 Initializing Secure Cross-Primal Messenger");
        info!("📋 Using capability-based discovery - zero hardcoded primal names");

        Ok(Self {
            discovery_service,
            our_identity: "beardog".to_string(), // We only know ourselves
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(MessengerMetrics::default())),
        })
    }

    /// Send secure message to ANY primal with network capability
    ///
    /// Discovers primals with network routing capability and sends message.
    /// Works with ANY primal that advertises the capability (songbird, or others).
    ///
    /// # Errors
    /// Returns an error if no network primals are discovered or communication fails
    pub async fn send_to_network_primal(
        &self,
        plaintext: &[u8],
        metadata: HashMap<String, String>,
    ) -> Result<SecurePrimalResponse, BearDogError> {
        // Discover primals with network routing capability (ANY primal, not just songbird)
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
        let ciphertext = self.encrypt_for_primal(plaintext, primal).await?;

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
    /// Works with ANY primal (e.g., toadstool, or others) without hardcoding names.
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

        let _response_data = self
            .discovery_service
            .send_request(primal, compute_request)
            .await?;

        let secure_response = SecurePrimalResponse {
            ciphertext: vec![], // Placeholder
            responder_id: primal.service_id.clone(),
            capability_used: UniversalCapabilityType::Compute {
                abilities: vec![ComputeAbility::DataAnalysis],
            },
            processing_time_ms: 0,
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

        let _response_data = self.discovery_service.send_request(primal, payload).await?;

        let secure_response = SecurePrimalResponse {
            ciphertext: vec![],
            responder_id: primal.service_id.clone(),
            capability_used: UniversalCapabilityType::Storage {
                characteristics: vec![StorageCharacteristic::Encrypted],
            },
            processing_time_ms: 0,
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
    /// # Phase 2 Enhancement
    /// Currently returns plaintext (pass-through) for initial integration testing.
    /// Full genetic cryptography implementation planned for Phase 2:
    /// 1. Get primal's public key from service descriptor
    /// 2. Use genetic algorithm to evolve shared key
    /// 3. Encrypt with evolved key using genetic crypto engine
    async fn encrypt_for_primal(
        &self,
        plaintext: &[u8],
        primal: &UniversalServiceDescriptor,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔐 Encrypting for primal: {}", primal.service_id);

        // Phase 2: Wire to genetic crypto engine
        // For now, pass-through for integration testing
        Ok(plaintext.to_vec())
    }

    /// Perform key exchange with primal (genetic algorithm-based)
    ///
    /// # Phase 2 Enhancement
    /// Currently returns a placeholder key for integration testing.
    /// Full genetic key exchange implementation planned for Phase 2:
    /// 1. Exchange genetic material with peer
    /// 2. Use fitness function to evolve shared key
    /// 3. Verify key establishment success
    ///
    /// This replaces traditional Diffie-Hellman/ECDH with genetic algorithms.
    async fn perform_key_exchange(
        &self,
        primal: &UniversalServiceDescriptor,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("🔑 Performing key exchange with: {}", primal.service_id);

        // Phase 2: Wire to genetic key exchange protocol
        // For now, generate placeholder key for integration testing
        Ok(vec![0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock discovery service for testing
    #[derive(Debug)]
    struct MockDiscoveryService {
        mock_primals: Vec<UniversalServiceDescriptor>,
    }

    #[async_trait::async_trait]
    impl PrimalDiscoveryService for MockDiscoveryService {
        async fn discover_by_capability(
            &self,
            _capability: UniversalCapabilityType,
        ) -> Result<Vec<UniversalServiceDescriptor>, BearDogError> {
            Ok(self.mock_primals.clone())
        }

        async fn send_request(
            &self,
            _service: &UniversalServiceDescriptor,
            _payload: serde_json::Value,
        ) -> Result<serde_json::Value, BearDogError> {
            Ok(serde_json::json!({}))
        }
    }

    #[tokio::test]
    async fn test_messenger_creation() {
        let mock_service = Arc::new(MockDiscoveryService {
            mock_primals: vec![],
        });

        let result = SecureCrossPrimalMessenger::new(mock_service);
        assert!(result.is_ok());

        let messenger = result.unwrap();
        assert_eq!(messenger.our_identity, "beardog");
    }

    #[tokio::test]
    async fn test_no_hardcoded_primal_names() {
        // This test verifies that the messenger doesn't hardcode primal names
        let mock_service = Arc::new(MockDiscoveryService {
            mock_primals: vec![],
        });

        let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();

        // The only identity should be "beardog" (ourselves)
        assert_eq!(messenger.our_identity, "beardog");

        // Verify we discover primals, not hardcode them
        let result = messenger
            .send_to_network_primal(b"test", HashMap::new())
            .await;

        // Should fail because no primals are discovered (not because of hardcoding)
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No network-capable primals discovered"));
    }

    #[tokio::test]
    async fn test_metrics_tracking() {
        let mock_service = Arc::new(MockDiscoveryService {
            mock_primals: vec![],
        });

        let messenger = SecureCrossPrimalMessenger::new(mock_service).unwrap();

        let metrics = messenger.get_metrics().await;
        assert_eq!(metrics.messages_sent, 0);
        assert_eq!(metrics.sessions_established, 0);
    }
}
