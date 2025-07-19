//! Security operations for AI interface

use serde::{Deserialize, Serialize};

/// AI Encryption Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEncryptRequest {
    pub data: String,
}

/// AI Encryption Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIEncryptResponse {
    pub encrypted_data: String,
    pub success: bool,
}

/// AI Decryption Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecryptRequest {
    pub encrypted_data: String,
}

/// AI Decryption Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIDecryptResponse {
    pub decrypted_data: String,
    pub success: bool,
}

/// AI Signature Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISignRequest {
    pub data: String,
}

/// AI Signature Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISignResponse {
    pub signature: String,
    pub success: bool,
}

/// AI Verification Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVerifyRequest {
    pub data: String,
    pub signature: String,
}

/// AI Verification Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIVerifyResponse {
    pub valid: bool,
}

/// AI Key Generation Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateKeyRequest {
    pub key_type: String,
}

/// AI Key Generation Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIGenerateKeyResponse {
    pub public_key: String,
    pub success: bool,
}

/// Security operation types for AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AISecurityOperation {
    /// Encrypt data
    Encrypt { data: String },
    /// Decrypt data
    Decrypt { encrypted_data: String },
    /// Sign data
    Sign { data: String },
    /// Verify signature
    Verify { data: String, signature: String },
    /// Generate key pair
    GenerateKey { key_type: String },
    /// Derive key from password
    DeriveKey { password: String, salt: String },
    /// Hash data
    Hash { data: String, algorithm: String },
    /// Generate random bytes
    Random { length: usize },
    /// Validate certificate
    ValidateCertificate { certificate: String },
    /// Create certificate
    CreateCertificate { subject: String },
}
