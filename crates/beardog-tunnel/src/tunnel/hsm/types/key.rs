

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    Hmac { key_size: u32 },

    KeyDerivation { key_size: u32 },


    EccP256,


    EccP384,


    EccP521,


    Ed25519,


    X25519,

    Custom(String,

    /// The hsm type value
    pub hsm_type: String,

    /// The key type value
    pub key_type: KeyType,

    /// The metadata value
    pub metadata: KeyMetadata,

    /// The key material value
    pub key_material: KeyMaterial,

    /// The hsm tier value
    pub hsm_tier: String,

    /// Current status of the health
    pub health_status: KeyHealthStatus,

    /// Optional attestation
    pub attestation: Option<KeyAttestation>,

    /// The created at value
    pub created_at: DateTime<Utc>,

pub enum KeyMaterial {

    /// State indicating encrypted
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

    /// Optional last accessed
    pub last_accessed: Option<DateTime<Utc>>,

    /// Number of access
    pub access_count: u64,


    pub key_id: String,

    /// The usage policy value
    pub usage_policy: KeyUsagePolicy,

pub struct KeyPerformanceMetrics {

    /// The avg latency ms value
    pub avg_latency_ms: f64,

    /// The ops per second value
    pub ops_per_second: f64,

    /// The error rate value
    pub error_rate: f64,

    /// Number of total_operations
    pub total_operations: u64,

pub struct KeyAttestation {

    /// Collection of certificate chain
    pub certificate_chain: Vec<Vec<u8>>,

    /// Collection of attestation statement
    pub attestation_statement: Vec<u8>,


    pub format: String,

    /// The generated at value
    pub generated_at: DateTime<Utc>,


    pub valid_until: DateTime<Utc>,

    /// The attestation type value
    pub attestation_type: String,

    /// Collection of attestation data
    pub attestation_data: Vec<u8>,

    /// Collection of attestation signature
    pub attestation_signature: Vec<u8>,

    /// Whether verified is enabled
    pub verified: bool,

pub enum KeyHealthStatus {


    /// Represents healthy variant
    Healthy,

    /// Currently warning
    Warning {

        message: String,

        severity: WarningSeverity,

    /// Represents unhealthy variant
    Unhealthy {

        error: String,

        error_time: DateTime<Utc>,


    /// Unknown or undefined state
    Unknown,

pub enum WarningSeverity {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,

pub struct GenerateKeyRequest {

    /// The target hsm tier value
    pub target_hsm_tier: String,

    /// Whether generate_attestation is enabled
    pub generate_attestation: bool,

    /// Optional attestation challenge
    pub attestation_challenge: Option<Vec<u8>>,

    /// Whether require_user_presence is enabled
    pub require_user_presence: bool,}
    pub require_user_presence: bool,}
    pub require_user_presence: bool,}

impl Default for KeyUsagePolicy {}

    fn default(true,
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
            allowed_applications: Vec::new(0.0,
            ops_per_second: 0.0,
            error_rate: 0.0,
            total_operations: 0,

pub enum HsmOperation {

    /// Represents generate key variant
    GenerateKey {

        key_type: KeyType,

        usage_policy: KeyUsagePolicy,

    /// Represents import key variant
    ImportKey {

        key_material: Vec<u8>,

    /// Represents export key variant
    ExportKey {

        key_id: String,

        format: String,

    /// Represents delete key variant
    DeleteKey {

    /// Represents encrypt variant
    Encrypt {

        data: Vec<u8>,

        algorithm: String,

    /// Represents decrypt variant
    Decrypt {

    /// Represents sign variant
    Sign {

    /// Represents verify variant
    Verify {

        signature: Vec<u8>,

    /// Represents wrap key variant
    WrapKey {

        wrapping_key_id: String,

    /// Represents unwrap key variant
    UnwrapKey {

        wrapped_key: Vec<u8>,

        unwrapping_key_id: String,

    /// Represents derive key variant
    DeriveKey {

        base_key_id: String,

        derivation_params: std::collections::HashMap<String, String>,

        derived_key_type: KeyType,

    /// Represents generate random variant
    GenerateRandom {

        byte_count: u32,

    /// Represents hash variant
    Hash {

    /// Represents attest key variant
    AttestKey {

        challenge: Vec<u8>,

    /// Represents genetic evolution variant
    GeneticEvolution {

        parameters: std::collections::HashMap<String, String>,

    /// Represents lineage proof variant
    LineageProof {

    /// Represents genetic witness variant
    GeneticWitness {

    /// Represents custom variant
    Custom {

        operation: String,

