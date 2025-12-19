//! Cryptographic Operation Endpoints
//!
//! Real crypto operations using the CryptoService trait (Phase 2 complete!)
//! Mocks have been evolved to production implementations.

use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use base64::{engine::general_purpose::STANDARD, Engine};
use beardog_types::crypto_service::{
    CryptoAlgorithm, DecryptOptions, EncryptOptions, SignOptions, SignatureAlgorithm, VerifyOptions,
};
use serde::{Deserialize, Serialize};

use crate::{ApiResponse, ApiState};

/// Encryption request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptRequest {
    /// Data to encrypt (base64 encoded)
    pub data: String,
    /// Key identifier (not the key itself)
    pub key_id: String,
    /// Optional additional authenticated data (base64 encoded)
    pub aad: Option<String>,
}

/// Encryption response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptResponse {
    /// Encrypted data (base64 encoded)
    pub ciphertext: String,
    /// Nonce used (base64 encoded)
    pub nonce: String,
    /// Authentication tag (base64 encoded)
    pub tag: String,
}

/// Decryption request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    /// Ciphertext to decrypt (base64 encoded)
    pub ciphertext: String,
    /// Key identifier
    pub key_id: String,
    /// Nonce (base64 encoded)
    pub nonce: String,
    /// Authentication tag (base64 encoded)
    pub tag: String,
    /// Optional additional authenticated data (base64 encoded)
    pub aad: Option<String>,
}

/// Decryption response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    /// Decrypted plaintext (base64 encoded)
    pub plaintext: String,
}

/// Signing request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignRequest {
    /// Message to sign (base64 encoded)
    pub message: String,
    /// Key identifier for signing key
    pub key_id: String,
}

/// Signing response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignResponse {
    /// Digital signature (base64 encoded)
    pub signature: String,
    /// Algorithm used
    pub algorithm: String,
}

/// Verification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyRequest {
    /// Message that was signed (base64 encoded)
    pub message: String,
    /// Signature to verify (base64 encoded)
    pub signature: String,
    /// Public key (base64 encoded)
    pub public_key: String,
}

/// Verification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResponse {
    /// Whether signature is valid
    pub valid: bool,
    /// Algorithm detected
    pub algorithm: String,
}

/// AES-GCM encryption endpoint - NOW WITH REAL CRYPTO!
///
/// # Capability
/// Requires: `aes-256-gcm-encrypt`
pub async fn aes_gcm_encrypt(
    State(state): State<ApiState>,
    Json(request): Json<EncryptRequest>,
) -> Result<Json<ApiResponse<EncryptResponse>>, StatusCode> {
    // Decode base64 input
    let data = STANDARD
        .decode(&request.data)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Decode AAD if provided
    let aad = if let Some(aad_b64) = request.aad {
        Some(
            STANDARD
                .decode(&aad_b64)
                .map_err(|_| StatusCode::BAD_REQUEST)?,
        )
    } else {
        None
    };

    // Call the REAL crypto service (Phase 1 trait!)
    let encrypted = state
        .crypto_service
        .encrypt(
            &data,
            CryptoAlgorithm::Aes256Gcm,
            EncryptOptions {
                key_id: request.key_id,
                associated_data: aad,
            },
        )
        .await
        .map_err(|e| {
            tracing::error!("Encryption failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Build response
    let response = EncryptResponse {
        ciphertext: STANDARD.encode(&encrypted.ciphertext),
        nonce: STANDARD.encode(&encrypted.metadata.nonce),
        tag: STANDARD.encode(
            encrypted
                .metadata
                .tag
                .as_ref()
                .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?,
        ),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// AES-GCM decryption endpoint - NOW WITH REAL CRYPTO!
///
/// # Capability
/// Requires: `aes-256-gcm-decrypt`
pub async fn aes_gcm_decrypt(
    State(state): State<ApiState>,
    Json(request): Json<DecryptRequest>,
) -> Result<Json<ApiResponse<DecryptResponse>>, StatusCode> {
    // Decode base64 inputs
    let ciphertext = STANDARD
        .decode(&request.ciphertext)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let nonce = STANDARD
        .decode(&request.nonce)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let tag = STANDARD
        .decode(&request.tag)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Decode AAD if provided
    let aad = if let Some(aad_b64) = request.aad {
        Some(
            STANDARD
                .decode(&aad_b64)
                .map_err(|_| StatusCode::BAD_REQUEST)?,
        )
    } else {
        None
    };

    // Build EncryptedData from request
    let encrypted = beardog_types::crypto_service::EncryptedData {
        ciphertext,
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: beardog_types::crypto_service::EncryptionMetadata {
            timestamp: std::time::SystemTime::now(),
            key_id: Some(request.key_id.clone()),
            nonce,
            tag: Some(tag),
        },
    };

    // Call the REAL crypto service (Phase 1 trait!)
    let plaintext = state
        .crypto_service
        .decrypt(
            &encrypted,
            DecryptOptions {
                key_id: request.key_id,
                associated_data: aad,
            },
        )
        .await
        .map_err(|e| {
            tracing::error!("Decryption failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Build response
    let response = DecryptResponse {
        plaintext: STANDARD.encode(&plaintext),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Ed25519 signing endpoint - NOW WITH REAL CRYPTO!
///
/// # Capability
/// Requires: `ed25519-sign`
pub async fn ed25519_sign(
    State(state): State<ApiState>,
    Json(request): Json<SignRequest>,
) -> Result<Json<ApiResponse<SignResponse>>, StatusCode> {
    // Decode base64 message
    let message = STANDARD
        .decode(&request.message)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Call the REAL crypto service (Phase 1 trait!)
    let signature = state
        .crypto_service
        .sign(
            &message,
            SignatureAlgorithm::Ed25519,
            SignOptions {
                key_id: request.key_id,
                context: None,
            },
        )
        .await
        .map_err(|e| {
            tracing::error!("Signing failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Build response
    let response = SignResponse {
        signature: STANDARD.encode(&signature.signature),
        algorithm: signature.algorithm.as_str().to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Ed25519 verification endpoint - NOW WITH REAL CRYPTO!
///
/// # Capability
/// Requires: `ed25519-verify`
pub async fn ed25519_verify(
    State(state): State<ApiState>,
    Json(request): Json<VerifyRequest>,
) -> Result<Json<ApiResponse<VerifyResponse>>, StatusCode> {
    // Decode base64 inputs
    let message = STANDARD
        .decode(&request.message)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let signature_bytes = STANDARD
        .decode(&request.signature)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let public_key = STANDARD
        .decode(&request.public_key)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Build Signature from request
    let signature = beardog_types::crypto_service::Signature {
        signature: signature_bytes,
        algorithm: SignatureAlgorithm::Ed25519,
        metadata: beardog_types::crypto_service::SignatureMetadata {
            timestamp: std::time::SystemTime::now(),
            key_id: None,
            context: None,
        },
    };

    // Call the REAL crypto service (Phase 1 trait!)
    let valid = state
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
        .map_err(|e| {
            tracing::error!("Verification failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Build response
    let response = VerifyResponse {
        valid,
        algorithm: "ed25519".to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Create crypto operation routes
pub fn crypto_routes() -> Router<ApiState> {
    Router::new()
        .route("/api/v1/crypto/aes-gcm/encrypt", post(aes_gcm_encrypt))
        .route("/api/v1/crypto/aes-gcm/decrypt", post(aes_gcm_decrypt))
        .route("/api/v1/crypto/ed25519/sign", post(ed25519_sign))
        .route("/api/v1/crypto/ed25519/verify", post(ed25519_verify))
}
