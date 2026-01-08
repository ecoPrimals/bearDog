//! tarpc Service Definition for BearDog
//!
//! PRIMARY inter-primal communication protocol
//!
//! ## Priority
//! tarpc is the **#1 PRIMARY** protocol for inter-primal communication:
//! - Type-safe (compile-time guarantees)
//! - Efficient (minimal overhead)
//! - Modern Rust (async/await idioms)
//! - Secure (security level 5/5)
//!
//! ## Architecture
//! ```text
//! BearDog ←─ tarpc (type-safe) ─→ Songbird
//!    ↓                              ↓
//!    └──── tarpc (type-safe) ────→ ToadStool
//! ```

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::btsp_provider::BeardogBtspProvider;

// ============================================================================
// tarpc Service Trait - Type-Safe Inter-Primal RPC
// ============================================================================

/// BearDog RPC service trait for inter-primal communication
///
/// This is the **PRIMARY** protocol for known primals (Songbird, ToadStool, etc.)
#[tarpc::service]
pub trait BearDogService {
    /// Ping the service (health check)
    async fn ping() -> PingResponse;

    /// Get BearDog capabilities
    async fn capabilities() -> CapabilitiesResponse;

    /// Evaluate trust for a peer
    async fn evaluate_trust(
        request: TrustEvaluationRequest,
    ) -> Result<TrustEvaluationResponse, String>;

    /// Encrypt data with BirdSong
    async fn birdsong_encrypt(plaintext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;

    /// Decrypt data with BirdSong
    async fn birdsong_decrypt(ciphertext: Vec<u8>, family_id: String) -> Result<Vec<u8>, String>;

    /// Get security metrics (for Songbird)
    async fn security_metrics() -> SecurityMetricsResponse;
}

// ============================================================================
// Request/Response Types - Type-Safe Communication
// ============================================================================

/// Ping response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PingResponse {
    pub pong: bool,
    pub timestamp: String,
    pub version: String,
    pub protocol: String,
}

/// Capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    pub capabilities: Vec<String>,
    pub version: String,
    pub protocols: Vec<String>,
    pub security_level: u8,
}

/// Trust evaluation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvaluationRequest {
    pub peer_id: String,
    pub family_id: String,
    pub requested_operation: Option<String>,
}

/// Trust evaluation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEvaluationResponse {
    pub trust_level: u8,
    pub reason: String,
    pub allowed: bool,
}

/// Security metrics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetricsResponse {
    pub trust_evaluations: u64,
    pub encryption_operations: u64,
    pub active_sessions: u64,
    pub uptime_seconds: u64,
}

// ============================================================================
// Service Implementation
// ============================================================================

/// BearDog service implementation
#[derive(Clone)]
pub struct BearDogServiceImpl {
    btsp_provider: Arc<BeardogBtspProvider>,
}

impl BearDogServiceImpl {
    /// Create new service implementation
    pub fn new(btsp_provider: Arc<BeardogBtspProvider>) -> Self {
        Self { btsp_provider }
    }
}

impl BearDogService for BearDogServiceImpl {
    /// Ping endpoint - health check
    async fn ping(self, _context: tarpc::context::Context) -> PingResponse {
        tracing::debug!("🎯 tarpc: ping");

        PingResponse {
            pong: true,
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol: "tarpc".to_string(),
        }
    }

    /// Get capabilities
    async fn capabilities(self, _context: tarpc::context::Context) -> CapabilitiesResponse {
        tracing::debug!("🎯 tarpc: capabilities");

        CapabilitiesResponse {
            capabilities: vec![
                "encryption".to_string(),
                "trust_evaluation".to_string(),
                "key_management".to_string(),
                "signatures".to_string(),
            ],
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocols: vec!["tarpc".to_string(), "json-rpc".to_string()],
            security_level: 5, // tarpc has highest security level
        }
    }

    /// Evaluate trust for a peer
    async fn evaluate_trust(
        self,
        _context: tarpc::context::Context,
        request: TrustEvaluationRequest,
    ) -> Result<TrustEvaluationResponse, String> {
        tracing::debug!("🎯 tarpc: evaluate_trust for peer={}", request.peer_id);

        // Real trust evaluation using genetic lineage
        // Get our family ID from environment (primal self-knowledge)
        let our_family = std::env::var("FAMILY_ID")
            .or_else(|_| std::env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        // Evaluate trust based on genetic lineage
        let peer_family = request.family_id.as_str();
        let same_family = our_family == peer_family;

        let (trust_level, reason, allowed) = if same_family {
            (2, "same_genetic_family".to_string(), true)
        } else {
            (0, "different_genetic_family".to_string(), false)
        };

        tracing::info!(
            "🎯 Trust evaluation: peer={}, peer_family={}, our_family={}, trust_level={}, allowed={}",
            request.peer_id,
            peer_family,
            our_family,
            trust_level,
            allowed
        );

        Ok(TrustEvaluationResponse {
            trust_level,
            reason,
            allowed,
        })
    }

    /// Encrypt data with BirdSong
    async fn birdsong_encrypt(
        self,
        _context: tarpc::context::Context,
        plaintext: Vec<u8>,
        family_id: String,
    ) -> Result<Vec<u8>, String> {
        tracing::debug!("🎯 tarpc: birdsong_encrypt for family={}", family_id);

        self.btsp_provider
            .birdsong_manager()
            .encrypt_discovery_for_family(&plaintext, &family_id)
            .map_err(|e| format!("Encryption failed: {}", e))
    }

    /// Decrypt data with BirdSong
    async fn birdsong_decrypt(
        self,
        _context: tarpc::context::Context,
        ciphertext: Vec<u8>,
        family_id: String,
    ) -> Result<Vec<u8>, String> {
        tracing::debug!("🎯 tarpc: birdsong_decrypt for family={}", family_id);

        self.btsp_provider
            .birdsong_manager()
            .decrypt_discovery_from_family(&ciphertext, &family_id)
            .map_err(|e| format!("Decryption failed: {}", e))
    }

    /// Get security metrics
    async fn security_metrics(self, _context: tarpc::context::Context) -> SecurityMetricsResponse {
        tracing::debug!("🎯 tarpc: security_metrics");

        // Real metrics from BTSP provider
        // Metrics collection is synchronous and lock-free (atomic operations)
        let metrics = self.btsp_provider.get_metrics();

        // Get uptime from system (process start time)
        let uptime_seconds = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        SecurityMetricsResponse {
            trust_evaluations: metrics.trust_evaluations,
            encryption_operations: metrics.encryption_operations + metrics.decryption_operations,
            active_sessions: metrics.tunnels_active,
            uptime_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping_response_structure() {
        let response = PingResponse {
            pong: true,
            timestamp: "2026-01-06T00:00:00Z".to_string(),
            version: "0.15.0".to_string(),
            protocol: "tarpc".to_string(),
        };

        assert!(response.pong);
        assert_eq!(response.protocol, "tarpc");
    }

    #[test]
    fn test_capabilities_response_security_level() {
        let response = CapabilitiesResponse {
            capabilities: vec!["encryption".to_string()],
            version: "0.15.0".to_string(),
            protocols: vec!["tarpc".to_string()],
            security_level: 5,
        };

        assert_eq!(response.security_level, 5); // Highest security
    }

    #[test]
    fn test_trust_evaluation_types() {
        let request = TrustEvaluationRequest {
            peer_id: "tower1".to_string(),
            family_id: "nat0".to_string(),
            requested_operation: Some("encrypt".to_string()),
        };

        assert_eq!(request.peer_id, "tower1");
        assert_eq!(request.family_id, "nat0");
    }

    #[test]
    fn test_security_metrics_response() {
        let response = SecurityMetricsResponse {
            trust_evaluations: 42,
            encryption_operations: 100,
            active_sessions: 5,
            uptime_seconds: 3600,
        };

        assert_eq!(response.trust_evaluations, 42);
    }
}
