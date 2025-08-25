// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ================================================================================
// CANONICAL TYPE RE-EXPORTS
// ================================================================================

// Re-export canonical types for provider implementations
pub use crate::canonical::hsm::{HsmKey, KeyMetadata, KeyType};
pub use crate::canonical::providers::ProviderConfig;

// ================================================================================
// CANONICAL PROVIDER TRAIT ACCESS - No More Compatibility Layers
// ================================================================================

/// **MODERNIZATION COMPLETE** ✅
/// All provider traits are now unified in beardog-traits::canonical.
/// No more backward compatibility re-exports - use canonical imports directly.
/// 
/// ## Canonical Usage:
/// ```rust
/// use beardog_traits::canonical::{BaseProvider, SecurityProvider, HsmProvider};
/// ```

// **CANONICAL PROVIDER TYPES** - Re-export from canonical module  
pub use crate::canonical::providers::{
    ProviderType, ProviderStatus, ProviderHealth,
    ProviderCapability, ProviderRegistryEntry,
};

// ================================================================================
// SUPPORTING TYPES FOR PROVIDER IMPLEMENTATIONS
// ================================================================================

/// Provider health status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealthStatus {
    pub is_healthy: bool,
    pub status_message: String,
    pub last_check: DateTime<Utc>,
    pub metrics: HashMap<String, f64>,
}

/// Primal request structure for ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRequest {
    pub request_id: String,
    pub operation: String,
    pub capability: String,
    pub data: Vec<u8>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// Primal response structure for ecosystem communication
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct PrimalResponse {
    pub request_id: String,
    pub success: bool,
    pub data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Primal capabilities enumeration
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

/// Authentication credentials structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCredentials {
    pub username: Option<String>,
    pub password: Option<String>,
    pub token: Option<String>,
    pub certificate: Option<Vec<u8>>,
    pub additional_data: HashMap<String, String>,
}

/// Authentication result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub user_id: String,
    pub session_token: Option<String>,
    pub expiry: Option<DateTime<Utc>>,
    pub permissions: Vec<String>,
}

/// Authorization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResult {
    pub allowed: bool,
    pub reason: Option<String>,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub client_id: String,
    pub ip_address: String,
    pub user_agent: String,
    pub platform: String,
}

/// Secure session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureSession {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Token validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    pub user_id: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub scopes: Vec<String>,
}

/// HSM key information - uses canonical types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    pub key_id: String,
    pub key_type: KeyType,
    pub metadata: KeyMetadata,
    pub usage_count: u64,
}

/// HSM hardware status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHardwareStatus {
    pub available: bool,
    pub temperature: Option<f64>,
    pub free_memory: Option<u64>,
    pub uptime_seconds: Option<u64>,
    pub error_count: u64,
}

/// General HSM information and capabilities
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

/// Hash algorithms supported by crypto providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
    Sha3_256,
}

/// Key pair algorithms for cryptographic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyPairAlgorithm {
    Rsa { bits: u16 },
    Ec { curve: String },
    Ed25519,
}

/// Cryptographic key pair
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub algorithm: KeyPairAlgorithm,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hit_count: u64,
    pub miss_count: u64,
    pub size: u64,
    pub eviction_count: u64,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub name: String,
    pub url: String,
    pub protocol: String,
    pub authentication_required: bool,
}

/// Service health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub is_healthy: bool,
    pub last_check: DateTime<Utc>,
    pub response_time_ms: f64,
    pub error_message: Option<String>,
}

// Re-export canonical security context
pub use crate::canonical::SecurityContext;
