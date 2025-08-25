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

/// Key types supported by the HSM system
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum KeyType {
    /// AES 128-bit symmetric encryption key
    #[default]
    Aes128,
    /// AES 192-bit symmetric encryption key
    Aes192,
    /// AES 256-bit symmetric encryption key
    Aes256,
    /// ChaCha20 symmetric encryption key
    ChaCha20,
    /// RSA key with specified key size
    Rsa { key_size: u32 },
    /// HMAC key
    Hmac { key_size: u32 },
    /// Key derivation key
    KeyDerivation { key_size: u32 },
    /// ECC P-256 key
    EccP256,
    /// ECC P-384 key
    EccP384,
    /// ECC P-521 key
    EccP521,
    /// Ed25519 signing key
    Ed25519,
    /// X25519 key exchange key
    X25519,
    /// Custom key type
    Custom(String),
}
/// Metadata associated with HSM keys
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Key usage policy defining how a key can be used
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// HSM key structure containing key material and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {
    /// Key identifier
    pub id: String,
    /// HSM type where the key is stored
    pub hsm_type: String,
    /// Key type information
    pub key_type: KeyType,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// Key material (encrypted or reference)
    pub key_material: KeyMaterial,
    /// HSM tier where the key is stored
    pub hsm_tier: String,
    /// Key health status
    pub health_status: KeyHealthStatus,
    /// Key attestation information
    pub attestation: Option<KeyAttestation>,
    /// Key creation timestamp
    pub created_at: DateTime<Utc>,
/// Key material representation
pub enum KeyMaterial {
    /// Encrypted key material
    Encrypted {
        /// Encrypted key data
        encrypted_data: Vec<u8>,
        /// Encryption algorithm used
        encryption_algorithm: String,
        /// Key derivation parameters
        kdf_params: Option<HashMap<String, String>>,
    },
    /// Reference to key stored in HSM
    Reference {
        /// HSM-specific key reference
        key_reference: String,
        /// HSM instance identifier
        hsm_instance: String,
    /// Hardware-backed key reference
    HardwareReference {
        /// Hardware reference identifier
        reference: String,
        /// Hardware location identifier
        hsm_location: String,
    /// Hardware-backed key handle
    Handle {
        /// HSM key handle
        key_handle: String,
        /// Handle type identifier
        handle_type: String,
/// Information about an HSM key without the actual key material
pub struct HsmKeyInfo {
    /// Key performance metrics
    pub performance_metrics: KeyPerformanceMetrics,
    /// Last access timestamp
    pub last_accessed: Option<DateTime<Utc>>,
    /// Access count
    pub access_count: u64,
    /// Key identifier (legacy field for compatibility)
    pub key_id: String,
    /// Key type (legacy field for compatibility)
    /// HSM type (legacy field for compatibility)
    /// Key usage policy (legacy field for compatibility)
    pub usage_policy: KeyUsagePolicy,
/// Key performance metrics
pub struct KeyPerformanceMetrics {
    /// Average operation latency in milliseconds
    pub avg_latency_ms: f64,
    /// Operations per second
    pub ops_per_second: f64,
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    /// Total operations performed
    pub total_operations: u64,
/// Key attestation information
pub struct KeyAttestation {
    /// Attestation certificate chain
    pub certificate_chain: Vec<Vec<u8>>,
    /// Attestation statement
    pub attestation_statement: Vec<u8>,
    /// Attestation format identifier
    pub format: String,
    /// Timestamp when attestation was generated
    pub generated_at: DateTime<Utc>,
    /// Attestation validity period
    pub valid_until: DateTime<Utc>,
    /// Attestation type (legacy field for compatibility)
    pub attestation_type: String,
    /// Attestation data (legacy field for compatibility)
    pub attestation_data: Vec<u8>,
    /// Attestation signature
    pub attestation_signature: Vec<u8>,
    /// Whether the attestation is verified
    pub verified: bool,
/// Health status of an HSM key
pub enum KeyHealthStatus {
    /// Key is healthy and operational
    Healthy,
    /// Key is operational but with warnings
    Warning {
        /// Warning message
        message: String,
        /// Warning severity level
        severity: WarningSeverity,
    /// Key is not operational
    Unhealthy {
        /// Error message
        error: String,
        /// Timestamp when error occurred
        error_time: DateTime<Utc>,
    /// Key health status is unknown
    Unknown,
/// Warning severity levels}


pub enum WarningSeverity {
    /// Low severity warning
    Low,
    /// Medium severity warning
    Medium,
    /// High severity warning
    High,
/// Request structure for key generation
pub struct GenerateKeyRequest {
    /// Type of key to generate
    /// Key usage policy
    /// Target HSM tier for key storage
    pub target_hsm_tier: String,
    /// Whether to generate attestation
    pub generate_attestation: bool,
    /// Attestation challenge
    pub attestation_challenge: Option<Vec<u8>>,
    /// Require user presence for key operations
    pub require_user_presence: bool,}


impl Default for KeyUsagePolicy {}


    fn default() -> Self {
        Self {
            can_encrypt: true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            can_wrap: false,
            can_unwrap: false,
            can_derive: false,
            exportable: false,
            extractable: false,
            min_security_level: 1,
            max_operations: None,
            allowed_applications: Vec::new(),
        }
    }
impl Default for KeyPerformanceMetrics {
            avg_latency_ms: 0.0,
            ops_per_second: 0.0,
            error_rate: 0.0,
            total_operations: 0,
/// HSM operation enumeration}


pub enum HsmOperation {
    /// Generate a new key
    GenerateKey {
        /// Key type to generate
        key_type: KeyType,
        /// Key usage policy
        usage_policy: KeyUsagePolicy,
    /// Import an existing key
    ImportKey {
        /// Key material to import
        key_material: Vec<u8>,
        /// Key type
    /// Export a key
    ExportKey {
        /// Key ID to export
        key_id: String,
        /// Export format
        format: String,
    /// Delete a key
    DeleteKey {
        /// Key ID to delete
    /// Encrypt data
    Encrypt {
        /// Key ID to use for encryption
        /// Data to encrypt
        data: Vec<u8>,
        /// Algorithm parameters
        algorithm: String,
    /// Decrypt data
    Decrypt {
        /// Key ID to use for decryption
        /// Data to decrypt
    /// Sign data
    Sign {
        /// Key ID to use for signing
        /// Data to sign
        /// Signature algorithm
    /// Verify signature
    Verify {
        /// Key ID to use for verification
        /// Original data
        /// Signature to verify
        signature: Vec<u8>,
    /// Wrap a key
    WrapKey {
        /// Key ID to wrap
        /// Wrapping key ID
        wrapping_key_id: String,
        /// Wrapping algorithm
    /// Unwrap a key
    UnwrapKey {
        /// Wrapped key data
        wrapped_key: Vec<u8>,
        /// Unwrapping key ID
        unwrapping_key_id: String,
        /// Unwrapping algorithm
    /// Derive a key
    DeriveKey {
        /// Base key ID
        base_key_id: String,
        /// Derivation parameters
        derivation_params: std::collections::HashMap<String, String>,
        /// Derived key type
        derived_key_type: KeyType,
    /// Generate random data
    GenerateRandom {
        /// Number of bytes to generate
        byte_count: u32,
    /// Hash data
    Hash {
        /// Data to hash
        /// Hash algorithm
    /// Attest a key
    AttestKey {
        /// Key ID to attest
        /// Attestation challenge
        challenge: Vec<u8>,
    /// Genetic evolution operation
    GeneticEvolution {
        /// Evolution parameters
        parameters: std::collections::HashMap<String, String>,
    /// Lineage proof operation
    LineageProof {
        /// Lineage parameters
    /// Genetic witness operation
    GeneticWitness {
        /// Witness parameters
    /// Custom operation
    Custom {
        /// Operation name
        operation: String,
        /// Operation parameters
