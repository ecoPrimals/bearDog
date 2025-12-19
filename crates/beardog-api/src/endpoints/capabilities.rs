//! Capability Advertisement Endpoints
//!
//! BearDog advertises its capabilities to the ecosystem.
//! Other primals discover BearDog by querying these endpoints.

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde::{Deserialize, Serialize};

use crate::ApiResponse;

/// Capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityInfo {
    /// Capability identifier (e.g., "aes-gcm-encrypt", "ed25519-sign")
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Capability category (crypto, hsm, audit, etc.)
    pub category: String,
    /// Endpoint path for this capability
    pub endpoint: String,
    /// Supported parameters
    pub parameters: Vec<String>,
    /// Whether this capability is currently available
    pub available: bool,
}

/// Service capabilities response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesResponse {
    /// Service name (self-knowledge)
    pub service_name: String,
    /// Service version
    pub version: String,
    /// All advertised capabilities
    pub capabilities: Vec<CapabilityInfo>,
    /// Service endpoints
    pub endpoints: ServiceEndpoints,
    /// mDNS service name for discovery
    pub mdns_service: String,
}

/// Service endpoint information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// Base API URL
    pub base_url: String,
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: String,
    /// Capabilities discovery endpoint
    pub capabilities: String,
}

/// Get all advertised capabilities
///
/// This endpoint allows other primals to discover what BearDog can do.
/// Response is capability-based, not operation-based.
pub async fn get_capabilities() -> Result<Json<ApiResponse<CapabilitiesResponse>>, StatusCode> {
    let capabilities = vec![
        CapabilityInfo {
            id: "aes-256-gcm-encrypt".to_string(),
            name: "AES-256-GCM Encryption".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/crypto/aes-gcm/encrypt".to_string(),
            parameters: vec!["data".to_string(), "key_id".to_string()],
            available: true,
        },
        CapabilityInfo {
            id: "aes-256-gcm-decrypt".to_string(),
            name: "AES-256-GCM Decryption".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/crypto/aes-gcm/decrypt".to_string(),
            parameters: vec!["data".to_string(), "key_id".to_string()],
            available: true,
        },
        CapabilityInfo {
            id: "ed25519-sign".to_string(),
            name: "Ed25519 Digital Signature".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/crypto/ed25519/sign".to_string(),
            parameters: vec!["message".to_string(), "key_id".to_string()],
            available: true,
        },
        CapabilityInfo {
            id: "ed25519-verify".to_string(),
            name: "Ed25519 Signature Verification".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/crypto/ed25519/verify".to_string(),
            parameters: vec![
                "message".to_string(),
                "signature".to_string(),
                "public_key".to_string(),
            ],
            available: true,
        },
        CapabilityInfo {
            id: "hsm-discovery".to_string(),
            name: "Hardware Security Module Discovery".to_string(),
            category: "hsm".to_string(),
            endpoint: "/api/v1/hsm/discover".to_string(),
            parameters: vec![],
            available: true,
        },
        CapabilityInfo {
            id: "entropy-generation".to_string(),
            name: "Cryptographic Entropy Generation".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/crypto/entropy".to_string(),
            parameters: vec!["size".to_string(), "source".to_string()],
            available: true,
        },
        CapabilityInfo {
            id: "key-generation".to_string(),
            name: "Cryptographic Key Generation".to_string(),
            category: "crypto".to_string(),
            endpoint: "/api/v1/keys/generate".to_string(),
            parameters: vec!["algorithm".to_string(), "purpose".to_string()],
            available: true,
        },
        CapabilityInfo {
            id: "audit-logging".to_string(),
            name: "Tamper-Evident Audit Logging".to_string(),
            category: "audit".to_string(),
            endpoint: "/api/v1/audit/log".to_string(),
            parameters: vec!["event".to_string(), "metadata".to_string()],
            available: true,
        },
    ];

    let response = CapabilitiesResponse {
        service_name: "beardog".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        capabilities,
        endpoints: ServiceEndpoints {
            base_url: std::env::var("BEARDOG_API_BASE_URL").unwrap_or_else(|_| {
                // Construct from configured port (uses beardog-config system)
                use beardog_config::global::BEARDOG_CONFIG;
                let port = BEARDOG_CONFIG.network.api.port;
                let host = &BEARDOG_CONFIG.network.api.bind_address;
                format!("http://{}:{}", host, port)
            }),
            health: "/health".to_string(),
            metrics: "/metrics".to_string(),
            capabilities: "/api/v1/capabilities".to_string(),
        },
        mdns_service: "_beardog._tcp.local".to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Query specific capability by ID
pub async fn get_capability_by_id(
    axum::extract::Path(capability_id): axum::extract::Path<String>,
) -> Result<Json<ApiResponse<CapabilityInfo>>, StatusCode> {
    // In a real implementation, this would query the capability registry
    // For now, return a basic response
    let capability = CapabilityInfo {
        id: capability_id.clone(),
        name: format!("Capability: {}", capability_id),
        category: "unknown".to_string(),
        endpoint: format!("/api/v1/capability/{}", capability_id),
        parameters: vec![],
        available: false,
    };

    Ok(Json(ApiResponse::success(capability)))
}

/// Create capability router
pub fn capability_routes() -> Router<()> {
    Router::new()
        .route("/api/v1/capabilities", get(get_capabilities))
        .route("/api/v1/capability/:id", get(get_capability_by_id))
}
