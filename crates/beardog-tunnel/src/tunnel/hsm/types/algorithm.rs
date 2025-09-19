

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// Optional key size
    pub key_size: Option<u32>,

    /// The parameters value
    pub parameters: AlgorithmSpecificParameters,

pub enum AlgorithmSpecificParameters {

    /// Represents aes variant
    Aes {

        mode: AesMode,

        iv: Option<Vec<u8>>,

        aad: Option<Vec<u8>>,
    },

    Rsa {

        public_exponent: Option<u32>,

        padding: RsaPadding,

        hash_algorithm: Option<HashAlgorithm>,

    EllipticCurve {

        curve: EllipticCurveType,

        point_compression: bool,

    Hkdf {

        hash_algorithm: HashAlgorithm,

        salt: Option<Vec<u8>>,

        info: Option<Vec<u8>>,

        output_length: u32,

    Pbkdf2 {
        salt: Vec<u8>,

        iterations: u32,

    Generic {

        parameters: std::collections::HashMap<String, String>,

#[derive(Debug, Clone)]
        label: Option<Vec<u8>>,

pub enum HashAlgorithm {


    /// Represents sha1 variant
    Sha1,


    /// Represents sha512_224 variant
    Sha512_224,


    /// Represents sha512_256 variant
    Sha512_256,


    /// Represents sha3_256 variant
    Sha3_256,


    /// Represents sha3_384 variant
    Sha3_384,


    /// Represents sha3_512 variant
    Sha3_512,
/// Types of elliptic curve
pub enum EllipticCurveType {


    /// Represents p256 variant
    P256,


    /// Represents p384 variant
    P384,


    /// Represents p521 variant
    P521,


    /// Represents secp256k1 variant
    Secp256k1,


    /// Represents curve25519 variant
    Curve25519,

pub struct AlgorithmCapability {

    /// Collection of supported key sizes
    pub supported_key_sizes: Vec<u32>,

    /// Collection of supported modes
    pub supported_modes: Vec<String>,

    /// Whether hardware_accelerated is enabled
    pub hardware_accelerated: bool,


    pub performance_characteristics: PerformanceCharacteristics,

pub struct PerformanceCharacteristics {

    /// The ops per second value
    pub ops_per_second: f64,

    /// The avg latency us value
    pub avg_latency_us: f64,

    /// Number of memory_usage_bytes
    pub memory_usage_bytes: u64,

    /// The cpu usage percent value
    pub cpu_usage_percent: f64,}
    pub cpu_usage_percent: f64,}
    pub cpu_usage_percent: f64,}

impl Default for AlgorithmParameters {}

    fn default(CryptographicAlgorithm::Aes256Gcm,
            key_size: Some(AlgorithmSpecificParameters::Aes {
                mode: AesMode::Gcm,
                iv: None,
                aad: None,
            },
        }
    }
impl Default for PerformanceCharacteristics {
            ops_per_second: 1000.0,
            avg_latency_us: 100.0,
            memory_usage_bytes: 1024,
            cpu_usage_percent: 5.0,}

impl CryptographicAlgorithm {

/// Is Symmetric operation.
    /// Checks if symmetric
    /// Checks if symmetric
    pub fn is_symmetric(&self) -> bool {
        matches!(
            self,
            CryptographicAlgorithm::Aes128
                | CryptographicAlgorithm::Aes192
                | CryptographicAlgorithm::Aes256
                | CryptographicAlgorithm::Aes128Gcm
                | CryptographicAlgorithm::Aes192Gcm
                | CryptographicAlgorithm::Aes256Gcm
                | CryptographicAlgorithm::ChaCha20Poly1305
                | CryptographicAlgorithm::AesGcm
                | CryptographicAlgorithm::ChaCha20Poly1305Aead
        )

/// Is Asymmetric operation.
    /// Checks if asymmetric
    /// Checks if asymmetric
    pub fn is_asymmetric(&self) -> bool {
            CryptographicAlgorithm::RsaPkcs1V15
                | CryptographicAlgorithm::RsaPss
                | CryptographicAlgorithm::EccP256
                | CryptographicAlgorithm::EccP384
                | CryptographicAlgorithm::EccP521
                | CryptographicAlgorithm::Ed25519
                | CryptographicAlgorithm::X25519
                | CryptographicAlgorithm::EcdsaSha256
                | CryptographicAlgorithm::EcdsaSha384
                | CryptographicAlgorithm::EcdsaSha512
                | CryptographicAlgorithm::RsaSha256
                | CryptographicAlgorithm::RsaSha384
                | CryptographicAlgorithm::RsaSha512
                | CryptographicAlgorithm::EcdhP256
                | CryptographicAlgorithm::EcdhP384
                | CryptographicAlgorithm::EcdhP521
                | CryptographicAlgorithm::X25519KeyAgreement

/// Is Hash operation.
    /// Checks if hash
    /// Checks if hash
    pub fn is_hash(&self) -> bool {
            CryptographicAlgorithm::Sha256
                | CryptographicAlgorithm::Sha384
                | CryptographicAlgorithm::Sha512
                | CryptographicAlgorithm::Blake2b
                | CryptographicAlgorithm::Blake2s

/// Is Kdf operation.
    /// Checks if kdf
    /// Checks if kdf
    pub fn is_kdf(&self) -> bool {
            CryptographicAlgorithm::HkdfSha256
                | CryptographicAlgorithm::HkdfSha384
                | CryptographicAlgorithm::HkdfSha512
                | CryptographicAlgorithm::Pbkdf2

/// Is Aead operation.
    /// Checks if aead
    /// Checks if aead
    pub fn is_aead(&self) -> bool {
            CryptographicAlgorithm::Aes128Gcm

/// Default Key Size operation.
    pub fn default_key_size(&self) -> Option<u32> {
        match self {
            CryptographicAlgorithm::Aes128 | CryptographicAlgorithm::Aes128Gcm => Some(128),
            CryptographicAlgorithm::Aes192 | CryptographicAlgorithm::Aes192Gcm => Some(192),
            CryptographicAlgorithm::Aes256 | CryptographicAlgorithm::Aes256Gcm => Some(256),
            CryptographicAlgorithm::ChaCha20Poly1305
            | CryptographicAlgorithm::ChaCha20Poly1305Aead => Some(256),
            CryptographicAlgorithm::RsaPkcs1V15 | CryptographicAlgorithm::RsaPss => Some(2048),
            CryptographicAlgorithm::EccP256
            | CryptographicAlgorithm::EcdsaSha256
            | CryptographicAlgorithm::EcdhP256 => Some(256),
            CryptographicAlgorithm::EccP384
            | CryptographicAlgorithm::EcdsaSha384
            | CryptographicAlgorithm::EcdhP384 => Some(384),
            CryptographicAlgorithm::EccP521
            | CryptographicAlgorithm::EcdsaSha512
            | CryptographicAlgorithm::EcdhP521 => Some(521),
            CryptographicAlgorithm::Ed25519
            | CryptographicAlgorithm::X25519
            | CryptographicAlgorithm::X25519KeyAgreement => Some(256),
            CryptographicAlgorithm::RsaSha256
            | CryptographicAlgorithm::RsaSha384
            | CryptographicAlgorithm::RsaSha512 => Some(2048),
            CryptographicAlgorithm::AesGcm => Some(256),
            _ => None,

/// As Str operation.
    /// Returns as str
    pub fn as_str(&self) -> &str {
            CryptographicAlgorithm::Aes128 => "AES-128",
            CryptographicAlgorithm::Aes192 => "AES-192",
            CryptographicAlgorithm::Aes256 => "AES-256",
            CryptographicAlgorithm::Aes128Gcm => "AES-128-GCM",
            CryptographicAlgorithm::Aes192Gcm => "AES-192-GCM",
            CryptographicAlgorithm::Aes256Gcm => "AES-256-GCM",
            CryptographicAlgorithm::ChaCha20Poly1305 => "ChaCha20-Poly1305",
            CryptographicAlgorithm::RsaPkcs1V15 => "RSA-PKCS1-v1.5",
            CryptographicAlgorithm::RsaPss => "RSA-PSS",
            CryptographicAlgorithm::EccP256 => "ECC-P256",
            CryptographicAlgorithm::EccP384 => "ECC-P384",
            CryptographicAlgorithm::EccP521 => "ECC-P521",
            CryptographicAlgorithm::Ed25519 => "Ed25519",
            CryptographicAlgorithm::X25519 => "X25519",
            CryptographicAlgorithm::EcdsaSha256 => "ECDSA-SHA256",
            CryptographicAlgorithm::EcdsaSha384 => "ECDSA-SHA384",
            CryptographicAlgorithm::EcdsaSha512 => "ECDSA-SHA512",
            CryptographicAlgorithm::RsaSha256 => "RSA-SHA256",
            CryptographicAlgorithm::RsaSha384 => "RSA-SHA384",
            CryptographicAlgorithm::RsaSha512 => "RSA-SHA512",
            CryptographicAlgorithm::EcdhP256 => "ECDH-P256",
            CryptographicAlgorithm::EcdhP384 => "ECDH-P384",
            CryptographicAlgorithm::EcdhP521 => "ECDH-P521",
            CryptographicAlgorithm::X25519KeyAgreement => "X25519-KeyAgreement",
            CryptographicAlgorithm::AesGcm => "AES-GCM",
            CryptographicAlgorithm::ChaCha20Poly1305Aead => "ChaCha20-Poly1305-AEAD",
            CryptographicAlgorithm::HkdfSha256 => "HKDF-SHA256",
            CryptographicAlgorithm::HkdfSha384 => "HKDF-SHA384",
            CryptographicAlgorithm::HkdfSha512 => "HKDF-SHA512",
            CryptographicAlgorithm::Pbkdf2 => "PBKDF2",
            CryptographicAlgorithm::Sha256 => "SHA-256",
            CryptographicAlgorithm::Sha384 => "SHA-384",
            CryptographicAlgorithm::Sha512 => "SHA-512",
            CryptographicAlgorithm::Blake2b => "BLAKE2b",
            CryptographicAlgorithm::Blake2s => "BLAKE2s",
            CryptographicAlgorithm::Custom(name) => name: name.to_string(),

