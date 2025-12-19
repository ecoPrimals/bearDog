//! Protocol Discovery Endpoints
//!
//! Allows clients to discover available protocols and how to connect to them.
//! Essential for protocol escalation (HTTP → JSON-RPC → tarpc).
//!
//! ## Endpoints
//!
//! - `GET /api/v1/protocols` - List all available protocols
//! - `GET /api/v1/protocols/{protocol}` - Get details for specific protocol
//!
//! ## Use Cases
//!
//! 1. **Client Discovery**: "What protocols does BearDog support?"
//! 2. **Protocol Selection**: "Which protocol is best for my use case?"
//! 3. **Connection Info**: "How do I connect to tarpc?"
//! 4. **Capability Matching**: "Does BearDog support the algorithm I need?"

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ApiResponse, ApiState};

/// Information about an available protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolInfo {
    /// Protocol name
    pub name: String,
    /// Protocol type (http, jsonrpc, tarpc)
    pub protocol_type: String,
    /// Whether this protocol is currently available
    pub available: bool,
    /// Connection endpoint (URL, address:port, etc.)
    pub endpoint: String,
    /// Protocol version
    pub version: String,
    /// Supported features
    pub features: Vec<String>,
    /// Supported crypto algorithms
    pub supported_algorithms: Vec<String>,
    /// Performance characteristics
    pub characteristics: ProtocolCharacteristics,
    /// How to use this protocol
    pub usage: ProtocolUsage,
}

/// Performance characteristics of a protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolCharacteristics {
    /// Transport efficiency (1-10, 10 = most efficient)
    pub efficiency: u8,
    /// Type safety level
    pub type_safety: String,
    /// Latency category
    pub latency: String,
    /// Binary data support
    pub binary_support: String,
}

/// Usage information for a protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolUsage {
    /// Example client code or connection string
    pub example: String,
    /// Language support
    pub languages: Vec<String>,
    /// Use case recommendations
    pub recommended_for: Vec<String>,
}

/// Get all available protocols
///
/// Returns information about HTTP, JSON-RPC, and tarpc protocols.
pub async fn get_protocols(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<HashMap<String, ProtocolInfo>>>, StatusCode> {
    let mut protocols = HashMap::new();

    // HTTP REST Protocol
    protocols.insert(
        "http".to_string(),
        ProtocolInfo {
            name: "HTTP REST API".to_string(),
            protocol_type: "http".to_string(),
            available: true,
            endpoint: "/api/v1/crypto/*".to_string(),
            version: "1.0".to_string(),
            features: vec![
                "REST endpoints".to_string(),
                "JSON request/response".to_string(),
                "Base64 binary encoding".to_string(),
                "CORS support".to_string(),
            ],
            supported_algorithms: vec!["aes-256-gcm".to_string(), "ed25519".to_string()],
            characteristics: ProtocolCharacteristics {
                efficiency: 6,
                type_safety: "Medium".to_string(),
                latency: "Medium".to_string(),
                binary_support: "Base64 encoded".to_string(),
            },
            usage: ProtocolUsage {
                example: "POST /api/v1/crypto/aes-gcm/encrypt".to_string(),
                languages: vec!["Any (curl, Python, JavaScript, Go, Rust, etc.)".to_string()],
                recommended_for: vec![
                    "Web applications".to_string(),
                    "Mobile apps".to_string(),
                    "Quick integration".to_string(),
                ],
            },
        },
    );

    // JSON-RPC 2.0 Protocol
    protocols.insert(
        "jsonrpc".to_string(),
        ProtocolInfo {
            name: "JSON-RPC 2.0".to_string(),
            protocol_type: "jsonrpc".to_string(),
            available: true,
            endpoint: "/rpc".to_string(),
            version: "2.0".to_string(),
            features: vec![
                "JSON-RPC 2.0 compliant".to_string(),
                "Batch requests".to_string(),
                "Standard error codes".to_string(),
                "Language-agnostic".to_string(),
            ],
            supported_algorithms: vec![
                "aes-256-gcm".to_string(),
                "aes-128-gcm".to_string(),
                "chacha20-poly1305".to_string(),
                "ed25519".to_string(),
                "ecdsa-p256".to_string(),
                "rsa-pss".to_string(),
            ],
            characteristics: ProtocolCharacteristics {
                efficiency: 6,
                type_safety: "Medium".to_string(),
                latency: "Medium".to_string(),
                binary_support: "Base64 encoded".to_string(),
            },
            usage: ProtocolUsage {
                example: r#"{"jsonrpc":"2.0","method":"beardog.encrypt","params":{...},"id":1}"#
                    .to_string(),
                languages: vec![
                    "Python (jsonrpcclient)".to_string(),
                    "JavaScript (jayson)".to_string(),
                    "Go (gorilla/rpc)".to_string(),
                    "Rust (jsonrpc-core)".to_string(),
                    "Java (JSON-RPC-Java)".to_string(),
                ],
                recommended_for: vec![
                    "Multi-language systems".to_string(),
                    "Structured RPC".to_string(),
                    "Batch operations".to_string(),
                ],
            },
        },
    );

    // tarpc Binary RPC Protocol
    protocols.insert(
        "tarpc".to_string(),
        ProtocolInfo {
            name: "tarpc Binary RPC".to_string(),
            protocol_type: "tarpc".to_string(),
            available: true,
            endpoint: "tcp://127.0.0.1:9000".to_string(), // Default, can be configured
            version: "0.34".to_string(),
            features: vec![
                "Binary protocol".to_string(),
                "Type-safe (Rust)".to_string(),
                "High performance".to_string(),
                "Zero-copy serialization".to_string(),
                "Native async".to_string(),
            ],
            supported_algorithms: vec![
                "aes-256-gcm".to_string(),
                "aes-128-gcm".to_string(),
                "chacha20-poly1305".to_string(),
                "ed25519".to_string(),
                "ecdsa-p256".to_string(),
                "rsa-pss".to_string(),
            ],
            characteristics: ProtocolCharacteristics {
                efficiency: 10,
                type_safety: "Strong (Rust types)".to_string(),
                latency: "Low".to_string(),
                binary_support: "Native binary".to_string(),
            },
            usage: ProtocolUsage {
                example: "BearDogCryptoRpcClient::new(...).encrypt(...)".to_string(),
                languages: vec!["Rust".to_string()],
                recommended_for: vec![
                    "High-performance services".to_string(),
                    "Rust-to-Rust communication".to_string(),
                    "Distributed compute".to_string(),
                    "Low-latency operations".to_string(),
                ],
            },
        },
    );

    Ok(Json(ApiResponse::success(protocols)))
}

/// Get information about a specific protocol
pub async fn get_protocol(
    State(_state): State<ApiState>,
    Path(protocol): Path<String>,
) -> Result<Json<ApiResponse<ProtocolInfo>>, StatusCode> {
    // Get all protocols
    let protocols_response = get_protocols(State(_state)).await?;
    let protocols = protocols_response
        .0
        .data
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    // Find the requested protocol
    match protocols.get(&protocol) {
        Some(info) => Ok(Json(ApiResponse::success(info.clone()))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// Protocol escalation guide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationGuide {
    /// Current protocol
    pub from: String,
    /// Target protocol
    pub to: String,
    /// Steps to escalate
    pub steps: Vec<String>,
    /// Expected performance gain
    pub performance_gain: String,
    /// Recommended use cases
    pub when_to_escalate: Vec<String>,
}

/// Get protocol escalation information
///
/// Helps clients understand how to escalate from one protocol to another.
pub async fn get_escalation_guide(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<Vec<EscalationGuide>>>, StatusCode> {
    let guides = vec![
        EscalationGuide {
            from: "http".to_string(),
            to: "jsonrpc".to_string(),
            steps: vec![
                "1. Query /api/v1/protocols to get JSON-RPC endpoint".to_string(),
                "2. Switch client to JSON-RPC 2.0 format".to_string(),
                "3. POST to /rpc with JSON-RPC requests".to_string(),
            ],
            performance_gain: "Minimal (~5%)".to_string(),
            when_to_escalate: vec![
                "Need batch operations".to_string(),
                "Want standardized RPC".to_string(),
                "Using multiple languages".to_string(),
            ],
        },
        EscalationGuide {
            from: "jsonrpc".to_string(),
            to: "tarpc".to_string(),
            steps: vec![
                "1. Query /api/v1/protocols to get tarpc endpoint".to_string(),
                "2. Switch to Rust client".to_string(),
                "3. Use tarpc client library to connect".to_string(),
                "4. Make type-safe RPC calls".to_string(),
            ],
            performance_gain: "Significant (~10x for binary data)".to_string(),
            when_to_escalate: vec![
                "High-volume operations".to_string(),
                "Large binary data".to_string(),
                "Low-latency requirements".to_string(),
                "Rust-to-Rust communication".to_string(),
            ],
        },
        EscalationGuide {
            from: "http".to_string(),
            to: "tarpc".to_string(),
            steps: vec![
                "1. Query /api/v1/protocols to get tarpc endpoint".to_string(),
                "2. Implement Rust client using tarpc".to_string(),
                "3. Connect directly to binary RPC endpoint".to_string(),
            ],
            performance_gain: "Maximum (~10x for binary data)".to_string(),
            when_to_escalate: vec![
                "Maximum performance needed".to_string(),
                "Rust ecosystem".to_string(),
                "Distributed compute".to_string(),
            ],
        },
    ];

    Ok(Json(ApiResponse::success(guides)))
}

/// Protocol comparison matrix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolComparison {
    /// Feature being compared
    pub feature: String,
    /// HTTP score/value
    pub http: String,
    /// JSON-RPC score/value
    pub jsonrpc: String,
    /// tarpc score/value
    pub tarpc: String,
    /// Winner
    pub best: String,
}

/// Get protocol comparison matrix
///
/// Helps clients choose the best protocol for their use case.
pub async fn get_protocol_comparison(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<Vec<ProtocolComparison>>>, StatusCode> {
    let comparison = vec![
        ProtocolComparison {
            feature: "Encoding Efficiency".to_string(),
            http: "Base64 (6/10)".to_string(),
            jsonrpc: "Base64 (6/10)".to_string(),
            tarpc: "Binary (10/10)".to_string(),
            best: "tarpc".to_string(),
        },
        ProtocolComparison {
            feature: "Type Safety".to_string(),
            http: "Medium".to_string(),
            jsonrpc: "Medium".to_string(),
            tarpc: "Strong (Rust)".to_string(),
            best: "tarpc".to_string(),
        },
        ProtocolComparison {
            feature: "Language Support".to_string(),
            http: "Any".to_string(),
            jsonrpc: "Any".to_string(),
            tarpc: "Rust only".to_string(),
            best: "http/jsonrpc".to_string(),
        },
        ProtocolComparison {
            feature: "Ease of Integration".to_string(),
            http: "Easy".to_string(),
            jsonrpc: "Medium".to_string(),
            tarpc: "Medium (Rust)".to_string(),
            best: "http".to_string(),
        },
        ProtocolComparison {
            feature: "Performance".to_string(),
            http: "Medium".to_string(),
            jsonrpc: "Medium".to_string(),
            tarpc: "High".to_string(),
            best: "tarpc".to_string(),
        },
        ProtocolComparison {
            feature: "Batch Operations".to_string(),
            http: "No".to_string(),
            jsonrpc: "Yes".to_string(),
            tarpc: "Yes".to_string(),
            best: "jsonrpc/tarpc".to_string(),
        },
    ];

    Ok(Json(ApiResponse::success(comparison)))
}
