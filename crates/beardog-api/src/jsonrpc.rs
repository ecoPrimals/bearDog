//! JSON-RPC 2.0 API for BearDog Crypto Operations
//!
//! Provides language-agnostic RPC access to crypto operations using the same
//! `CryptoService` trait as HTTP endpoints (Phase 1).
//!
//! ## JSON-RPC 2.0 Specification
//!
//! All requests must include:
//! - `jsonrpc`: "2.0"
//! - `method`: The RPC method name (e.g., "beardog.encrypt")
//! - `params`: Method parameters (object or array)
//! - `id`: Request identifier (for matching responses)
//!
//! ## Supported Methods
//!
//! - `beardog.encrypt` - Encrypt data with AES-256-GCM
//! - `beardog.decrypt` - Decrypt data
//! - `beardog.sign` - Sign data with Ed25519
//! - `beardog.verify` - Verify signature
//! - `beardog.capabilities` - Query available algorithms
//! - `beardog.health` - Service health status

use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use base64::{engine::general_purpose::STANDARD, Engine};
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, SignOptions, SignatureAlgorithm, VerifyOptions,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ApiState;

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (must be "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Method parameters (object or array)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    /// Request identifier
    pub id: Value,
}

/// JSON-RPC 2.0 Success Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version
    pub jsonrpc: String,
    /// Result (on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (on failure)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request identifier (matches request)
    pub id: Value,
}

/// JSON-RPC 2.0 Error Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

// JSON-RPC error codes
#[allow(dead_code)]
const PARSE_ERROR: i32 = -32700;
const INVALID_REQUEST: i32 = -32600;
const METHOD_NOT_FOUND: i32 = -32601;
const INVALID_PARAMS: i32 = -32602;
const INTERNAL_ERROR: i32 = -32603;

impl JsonRpcResponse {
    /// Create success response
    fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Create error response
    fn error(id: Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code,
                message,
                data: None,
            }),
            id,
        }
    }
}

/// Encrypt parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
struct EncryptParams {
    /// Data to encrypt (base64 encoded)
    data: String,
    /// Algorithm (default: aes-256-gcm)
    #[serde(default = "default_encrypt_algorithm")]
    algorithm: String,
    /// Key identifier
    key_id: String,
    /// Optional AAD (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    aad: Option<String>,
}

fn default_encrypt_algorithm() -> String {
    "aes-256-gcm".to_string()
}

/// Decrypt parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DecryptParams {
    /// Ciphertext (base64 encoded)
    ciphertext: String,
    /// Nonce (base64 encoded)
    nonce: String,
    /// Tag (base64 encoded)
    tag: String,
    /// Algorithm (default: aes-256-gcm)
    #[serde(default = "default_encrypt_algorithm")]
    algorithm: String,
    /// Key identifier
    key_id: String,
    /// Optional AAD (base64 encoded)
    #[serde(skip_serializing_if = "Option::is_none")]
    aad: Option<String>,
}

/// Sign parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SignParams {
    /// Message to sign (base64 encoded)
    message: String,
    /// Algorithm (default: ed25519)
    #[serde(default = "default_sign_algorithm")]
    algorithm: String,
    /// Key identifier
    key_id: String,
}

fn default_sign_algorithm() -> String {
    "ed25519".to_string()
}

/// Verify parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerifyParams {
    /// Message that was signed (base64 encoded)
    message: String,
    /// Signature (base64 encoded)
    signature: String,
    /// Public key (base64 encoded)
    public_key: String,
    /// Algorithm (default: ed25519)
    #[serde(default = "default_sign_algorithm")]
    algorithm: String,
}

/// JSON-RPC handler endpoint
pub async fn jsonrpc_handler(
    State(state): State<ApiState>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcResponse>, StatusCode> {
    // Validate JSON-RPC version
    if request.jsonrpc != "2.0" {
        return Ok(Json(JsonRpcResponse::error(
            request.id,
            INVALID_REQUEST,
            "JSON-RPC version must be 2.0".to_string(),
        )));
    }

    // Route to handler based on method
    let response = match request.method.as_str() {
        "beardog.encrypt" => handle_encrypt(&state, request).await,
        "beardog.decrypt" => handle_decrypt(&state, request).await,
        "beardog.sign" => handle_sign(&state, request).await,
        "beardog.verify" => handle_verify(&state, request).await,
        "beardog.capabilities" => handle_capabilities(&state, request).await,
        "beardog.health" => handle_health(&state, request).await,
        _ => JsonRpcResponse::error(
            request.id,
            METHOD_NOT_FOUND,
            format!("Method '{}' not found", request.method),
        ),
    };

    Ok(Json(response))
}

/// Handle encrypt method
async fn handle_encrypt(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Parse parameters
    let params: EncryptParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                )
            }
        },
        None => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                "Missing parameters".to_string(),
            )
        }
    };

    // Decode base64 input
    let data = match STANDARD.decode(&params.data) {
        Ok(d) => d,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 data: {}", e),
            )
        }
    };

    // Decode AAD if provided
    let aad = if let Some(aad_b64) = params.aad {
        match STANDARD.decode(&aad_b64) {
            Ok(a) => Some(a),
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid base64 AAD: {}", e),
                )
            }
        }
    } else {
        None
    };

    // Determine algorithm
    let algorithm = match params.algorithm.as_str() {
        "aes-256-gcm" => CryptoAlgorithm::Aes256Gcm,
        "aes-128-gcm" => CryptoAlgorithm::Aes128Gcm,
        "chacha20-poly1305" => CryptoAlgorithm::ChaCha20Poly1305,
        _ => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Unsupported algorithm: {}", params.algorithm),
            )
        }
    };

    // Call crypto service
    let encrypted = match state
        .crypto_service
        .encrypt(
            &data,
            algorithm,
            EncryptOptions {
                key_id: params.key_id,
                associated_data: aad,
            },
        )
        .await
    {
        Ok(e) => e,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Encryption failed: {}", e),
            )
        }
    };

    // Build result
    let result = serde_json::json!({
        "ciphertext": STANDARD.encode(&encrypted.ciphertext),
        "nonce": STANDARD.encode(&encrypted.metadata.nonce),
        "tag": STANDARD.encode(encrypted.metadata.tag.as_ref().unwrap_or(&vec![])),
        "algorithm": match encrypted.algorithm {
            CryptoAlgorithm::Aes256Gcm => "aes-256-gcm",
            CryptoAlgorithm::Aes128Gcm => "aes-128-gcm",
            CryptoAlgorithm::ChaCha20Poly1305 => "chacha20-poly1305",
        },
    });

    JsonRpcResponse::success(request.id, result)
}

/// Handle decrypt method
async fn handle_decrypt(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Parse parameters
    let params: DecryptParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                )
            }
        },
        None => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                "Missing parameters".to_string(),
            )
        }
    };

    // Decode base64 inputs
    let ciphertext = match STANDARD.decode(&params.ciphertext) {
        Ok(c) => c,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 ciphertext: {}", e),
            )
        }
    };

    let nonce = match STANDARD.decode(&params.nonce) {
        Ok(n) => n,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 nonce: {}", e),
            )
        }
    };

    let tag = match STANDARD.decode(&params.tag) {
        Ok(t) => t,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 tag: {}", e),
            )
        }
    };

    // Decode AAD if provided
    let aad = if let Some(aad_b64) = params.aad {
        match STANDARD.decode(&aad_b64) {
            Ok(a) => Some(a),
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid base64 AAD: {}", e),
                )
            }
        }
    } else {
        None
    };

    // Determine algorithm
    let algorithm = match params.algorithm.as_str() {
        "aes-256-gcm" => CryptoAlgorithm::Aes256Gcm,
        "aes-128-gcm" => CryptoAlgorithm::Aes128Gcm,
        "chacha20-poly1305" => CryptoAlgorithm::ChaCha20Poly1305,
        _ => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Unsupported algorithm: {}", params.algorithm),
            )
        }
    };

    // Build EncryptedData
    let encrypted = beardog_types::crypto_service::EncryptedData {
        ciphertext,
        algorithm,
        metadata: beardog_types::crypto_service::EncryptionMetadata {
            timestamp: std::time::SystemTime::now(),
            key_id: Some(params.key_id.clone()),
            nonce,
            tag: Some(tag),
        },
    };

    // Call crypto service
    let plaintext = match state
        .crypto_service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: params.key_id,
                associated_data: aad,
            },
        )
        .await
    {
        Ok(p) => p,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Decryption failed: {}", e),
            )
        }
    };

    // Build result
    let result = serde_json::json!({
        "plaintext": STANDARD.encode(&plaintext),
    });

    JsonRpcResponse::success(request.id, result)
}

/// Handle sign method
async fn handle_sign(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Parse parameters
    let params: SignParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                )
            }
        },
        None => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                "Missing parameters".to_string(),
            )
        }
    };

    // Decode base64 message
    let message = match STANDARD.decode(&params.message) {
        Ok(m) => m,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 message: {}", e),
            )
        }
    };

    // Determine algorithm
    let algorithm = match params.algorithm.as_str() {
        "ed25519" => SignatureAlgorithm::Ed25519,
        "ecdsa-p256" => SignatureAlgorithm::EcdsaP256,
        "rsa-pss" => SignatureAlgorithm::RsaPss,
        _ => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Unsupported algorithm: {}", params.algorithm),
            )
        }
    };

    // Call crypto service
    let signature = match state
        .crypto_service
        .sign(
            &message,
            algorithm,
            SignOptions {
                key_id: params.key_id,
                context: None,
            },
        )
        .await
    {
        Ok(s) => s,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Signing failed: {}", e),
            )
        }
    };

    // Build result
    let result = serde_json::json!({
        "signature": STANDARD.encode(&signature.signature),
        "algorithm": signature.algorithm.as_str(),
    });

    JsonRpcResponse::success(request.id, result)
}

/// Handle verify method
async fn handle_verify(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Parse parameters
    let params: VerifyParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return JsonRpcResponse::error(
                    request.id,
                    INVALID_PARAMS,
                    format!("Invalid parameters: {}", e),
                )
            }
        },
        None => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                "Missing parameters".to_string(),
            )
        }
    };

    // Decode base64 inputs
    let message = match STANDARD.decode(&params.message) {
        Ok(m) => m,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 message: {}", e),
            )
        }
    };

    let signature_bytes = match STANDARD.decode(&params.signature) {
        Ok(s) => s,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 signature: {}", e),
            )
        }
    };

    let public_key = match STANDARD.decode(&params.public_key) {
        Ok(pk) => pk,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Invalid base64 public_key: {}", e),
            )
        }
    };

    // Determine algorithm
    let algorithm = match params.algorithm.as_str() {
        "ed25519" => SignatureAlgorithm::Ed25519,
        "ecdsa-p256" => SignatureAlgorithm::EcdsaP256,
        "rsa-pss" => SignatureAlgorithm::RsaPss,
        _ => {
            return JsonRpcResponse::error(
                request.id,
                INVALID_PARAMS,
                format!("Unsupported algorithm: {}", params.algorithm),
            )
        }
    };

    // Build Signature
    let signature = beardog_types::crypto_service::Signature {
        signature: signature_bytes,
        algorithm,
        metadata: beardog_types::crypto_service::SignatureMetadata {
            timestamp: std::time::SystemTime::now(),
            key_id: None,
            context: None,
        },
    };

    // Call crypto service
    let valid = match state
        .crypto_service
        .verify(
            &message,
            &signature,
            VerifyOptions {
                public_key,
                context: None,
            },
        )
        .await
    {
        Ok(v) => v,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Verification failed: {}", e),
            )
        }
    };

    // Build result
    let result = serde_json::json!({
        "valid": valid,
        "algorithm": algorithm.as_str(),
    });

    JsonRpcResponse::success(request.id, result)
}

/// Handle capabilities method
async fn handle_capabilities(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Call crypto service
    let capabilities = match state.crypto_service.get_capabilities().await {
        Ok(c) => c,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Failed to get capabilities: {}", e),
            )
        }
    };

    // Convert to JSON
    let result = match serde_json::to_value(&capabilities) {
        Ok(r) => r,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Failed to serialize capabilities: {}", e),
            )
        }
    };

    JsonRpcResponse::success(request.id, result)
}

/// Handle health method
async fn handle_health(state: &ApiState, request: JsonRpcRequest) -> JsonRpcResponse {
    // Call crypto service
    let health = match state.crypto_service.get_health().await {
        Ok(h) => h,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Failed to get health: {}", e),
            )
        }
    };

    // Convert to JSON
    let result = match serde_json::to_value(&health) {
        Ok(r) => r,
        Err(e) => {
            return JsonRpcResponse::error(
                request.id,
                INTERNAL_ERROR,
                format!("Failed to serialize health: {}", e),
            )
        }
    };

    JsonRpcResponse::success(request.id, result)
}

/// Create JSON-RPC routes
pub fn jsonrpc_routes() -> Router<ApiState> {
    Router::new().route("/rpc", post(jsonrpc_handler))
}
