

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum KeyType {

    #[default]
    Aes128,

    Aes192,

    Aes256,

    ChaCha20,

    Rsa { key_size: u32 },

    Hmac { key_size: u32 },

    KeyDerivation { key_size: u32 },

    EccP256,

    EccP384,

    EccP521,

    Ed25519,

    X25519,

    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKey {

    pub id: String,

    pub hsm_type: String,

    pub key_type: KeyType,

    pub metadata: KeyMetadata,

    pub key_material: KeyMaterial,

    pub hsm_tier: String,

    pub health_status: KeyHealthStatus,

    pub attestation: Option<KeyAttestation>,

    pub created_at: DateTime<Utc>,

pub enum KeyMaterial {

    Encrypted {

        encrypted_data: Vec<u8>,

        encryption_algorithm: String,

        kdf_params: Option<HashMap<String, String>>,
    },

    Reference {

        key_reference: String,

        hsm_instance: String,

    HardwareReference {

        reference: String,

        hsm_location: String,

    Handle {

        key_handle: String,

        handle_type: String,

pub struct HsmKeyInfo {

    pub performance_metrics: KeyPerformanceMetrics,

    pub last_accessed: Option<DateTime<Utc>>,

    pub access_count: u64,

    pub key_id: String,

    pub usage_policy: KeyUsagePolicy,

pub struct KeyPerformanceMetrics {

    pub avg_latency_ms: f64,

    pub ops_per_second: f64,

    pub error_rate: f64,

    pub total_operations: u64,

pub struct KeyAttestation {

    pub certificate_chain: Vec<Vec<u8>>,

    pub attestation_statement: Vec<u8>,

    pub format: String,

    pub generated_at: DateTime<Utc>,

    pub valid_until: DateTime<Utc>,

    pub attestation_type: String,

    pub attestation_data: Vec<u8>,

    pub attestation_signature: Vec<u8>,

    pub verified: bool,

pub enum KeyHealthStatus {

    Healthy,

    Warning {

        message: String,

        severity: WarningSeverity,

    Unhealthy {

        error: String,

        error_time: DateTime<Utc>,

    Unknown,

pub enum WarningSeverity {

    Low,

    Medium,

    High,

pub struct GenerateKeyRequest {

    pub target_hsm_tier: String,

    pub generate_attestation: bool,

    pub attestation_challenge: Option<Vec<u8>>,

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

pub enum HsmOperation {

    GenerateKey {

        key_type: KeyType,

        usage_policy: KeyUsagePolicy,

    ImportKey {

        key_material: Vec<u8>,

    ExportKey {

        key_id: String,

        format: String,

    DeleteKey {

    Encrypt {

        data: Vec<u8>,

        algorithm: String,

    Decrypt {

    Sign {

    Verify {

        signature: Vec<u8>,

    WrapKey {

        wrapping_key_id: String,

    UnwrapKey {

        wrapped_key: Vec<u8>,

        unwrapping_key_id: String,

    DeriveKey {

        base_key_id: String,

        derivation_params: std::collections::HashMap<String, String>,

        derived_key_type: KeyType,

    GenerateRandom {

        byte_count: u32,

    Hash {

    AttestKey {

        challenge: Vec<u8>,

    GeneticEvolution {

        parameters: std::collections::HashMap<String, String>,

    LineageProof {

    GeneticWitness {

    Custom {

        operation: String,

