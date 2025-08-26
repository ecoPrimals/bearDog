

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::canonical::hsm::{HsmKey, KeyMetadata, KeyType};
pub use crate::canonical::providers::ProviderConfig;

pub use crate::canonical::providers::{
    ProviderType, ProviderStatus, ProviderHealth,
    ProviderCapability, ProviderRegistryEntry,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealthStatus {
    pub is_healthy: bool,
    pub status_message: String,
    pub last_check: DateTime<Utc>,
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    pub request_id: String,
    pub operation: String,
    pub capability: String,
    pub data: Vec<u8>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PrimalResponse {
    pub request_id: String,
    pub success: bool,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalCapability {
    Authentication,
    Authorization,
    Encryption,
    KeyManagement,
    ThreatDetection,
    Compliance,
    Monitoring,
    Storage,
    Networking,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub certificate: Option<Vec<u8>>,
    pub additional_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub user_id: String,
    pub session_token: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub allowed: bool,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub platform: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureSession {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    pub user_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    pub key_id: String,
    pub key_type: KeyType,
    pub metadata: KeyMetadata,
    pub usage_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHardwareStatus {
    pub available: bool,
    pub temperature: Option<f64>,
    pub free_memory: Option<u64>,
    pub uptime_seconds: Option<u64>,
    pub error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    pub instance_id: String,
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub api_version: String,
    pub supported_algorithms: Vec<String>,
    pub max_key_count: u32,
    pub current_key_count: u32,
    pub capabilities: Vec<String>,
    pub certification: Option<String>,
    pub tamper_resistant: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
    Sha3_256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyPairAlgorithm {
    Rsa { bits: u16 },
    Ec { curve: String },
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub algorithm: KeyPairAlgorithm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub size: u64,
    pub eviction_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub name: String,
    pub url: String,
    pub protocol: String,
    pub authentication_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub is_healthy: bool,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: f64,
    pub error_message: Option<String>,
}

pub use crate::canonical::SecurityContext;
