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


use serde::{Deserialize, Serialize};

/// Cryptographic algorithms supported by HSM
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CryptographicAlgorithm {
    // Symmetric algorithms
    /// AES-128 encryption
    Aes128,
    /// AES-192 encryption
    Aes192,
    /// AES-256 encryption
    Aes256,
    /// AES-128 in GCM mode
    Aes128Gcm,
    /// AES-192 in GCM mode
    Aes192Gcm,
    /// AES-256 in GCM mode
    Aes256Gcm,
    /// ChaCha20-Poly1305 authenticated encryption
    ChaCha20Poly1305,
    // Asymmetric algorithms
    /// RSA PKCS#1 v1.5 padding
    RsaPkcs1V15,
    /// RSA PSS (Probabilistic Signature Scheme)
    RsaPss,
    /// Elliptic Curve P-256
    EccP256,
    /// Elliptic Curve P-384
    EccP384,
    /// Elliptic Curve P-521
    EccP521,
    /// Ed25519 signature algorithm
    Ed25519,
    /// X25519 key agreement algorithm
    X25519,
    // Digital signature algorithms
    /// ECDSA with SHA-256
    EcdsaSha256,
    /// ECDSA with SHA-384
    EcdsaSha384,
    /// ECDSA with SHA-512
    EcdsaSha512,
    /// RSA with SHA-256
    RsaSha256,
    /// RSA with SHA-384
    RsaSha384,
    /// RSA with SHA-512
    RsaSha512,
    // Key agreement algorithms
    /// ECDH with P-256 curve
    EcdhP256,
    /// ECDH with P-384 curve
    EcdhP384,
    /// ECDH with P-521 curve
    EcdhP521,
    /// X25519 key agreement
    X25519KeyAgreement,
    // AEAD algorithms
    /// AES-GCM authenticated encryption
    AesGcm,
    /// ChaCha20-Poly1305 AEAD
    ChaCha20Poly1305Aead,
    // Key derivation algorithms
    /// HKDF with SHA-256
    HkdfSha256,
    /// HKDF with SHA-384
    HkdfSha384,
    /// HKDF with SHA-512
    HkdfSha512,
    /// PBKDF2 key derivation
    Pbkdf2,
    // Hash algorithms
    /// SHA-256 hash function
    Sha256,
    /// SHA-384 hash function
    Sha384,
    /// SHA-512 hash function
    Sha512,
    /// BLAKE2b hash function
    Blake2b,
    /// BLAKE2s hash function
    Blake2s,
    /// Custom algorithm implementation
    Custom(String),
}
/// Algorithm parameters for cryptographic operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlgorithmParameters {
    /// Algorithm identifier
    pub algorithm: CryptographicAlgorithm,
    /// Key size in bits
    pub key_size: Option<u32>,
    /// Algorithm-specific parameters
    pub parameters: AlgorithmSpecificParameters,
/// Algorithm-specific parameters}


pub enum AlgorithmSpecificParameters {
    /// AES parameters
    Aes {
        /// Block cipher mode}


        mode: AesMode,
        /// Initialization vector
        iv: Option<Vec<u8>>,
        /// Additional authenticated data (for AEAD modes)
        aad: Option<Vec<u8>>,
    },
    /// RSA parameters
    Rsa {
        /// Public exponent
        public_exponent: Option<u32>,
        /// Padding scheme
        padding: RsaPadding,
        /// Hash algorithm for PSS padding
        hash_algorithm: Option<HashAlgorithm>,
    /// Elliptic curve parameters
    EllipticCurve {
        /// Curve identifier
        curve: EllipticCurveType,
        /// Point compression
        point_compression: bool,
    /// HKDF parameters
    Hkdf {
        /// Hash algorithm
        hash_algorithm: HashAlgorithm,
        /// Salt value
        salt: Option<Vec<u8>>,
        /// Info parameter
        info: Option<Vec<u8>>,
        /// Output length
        output_length: u32,
    /// PBKDF2 parameters
    Pbkdf2 {
        salt: Vec<u8>,
        /// Iteration count
        iterations: u32,
    /// Generic parameters
    Generic {
        /// Parameter map
        parameters: std::collections::HashMap<String, String>,
/// AES block cipher modes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AesMode {
    /// Electronic Codebook mode
    Ecb,
    /// Cipher Block Chaining mode
    Cbc,
    /// Cipher Feedback mode
    Cfb,
    /// Output Feedback mode
    Ofb,
    /// Counter mode
    Ctr,
    /// Galois/Counter Mode (authenticated encryption)
    Gcm,
    /// Counter with CBC-MAC (authenticated encryption)
    Ccm,
/// RSA padding schemes}


pub enum RsaPadding {
    /// PKCS#1 v1.5 padding
    Pkcs1V15,
    /// PSS padding
    Pss,
    /// OAEP padding
    Oaep {
        /// MGF1 hash algorithm
        mgf1_hash: HashAlgorithm,
        /// Label
        label: Option<Vec<u8>>,
/// Hash algorithms
pub enum HashAlgorithm {
    /// SHA-1 (legacy support only - not recommended for new applications)
    Sha1,
    /// SHA-256
    /// SHA-384
    /// SHA-512
    /// SHA-512/224
    Sha512_224,
    /// SHA-512/256
    Sha512_256,
    /// SHA-3 256-bit
    Sha3_256,
    /// SHA-3 384-bit
    Sha3_384,
    /// SHA-3 512-bit
    Sha3_512,
    /// BLAKE2b
    /// BLAKE2s
/// Elliptic curve types}


pub enum EllipticCurveType {
    /// NIST P-256 (secp256r1)
    P256,
    /// NIST P-384 (secp384r1)
    P384,
    /// NIST P-521 (secp521r1)
    P521,
    /// secp256k1 (Bitcoin curve)
    Secp256k1,
    /// Curve25519
    Curve25519,
    /// Ed25519
    /// Custom curve
/// Algorithm capability information
pub struct AlgorithmCapability {
    /// Supported key sizes
    pub supported_key_sizes: Vec<u32>,
    /// Supported modes (for block ciphers)
    pub supported_modes: Vec<String>,
    /// Hardware acceleration available
    pub hardware_accelerated: bool,
    /// Performance characteristics
    pub performance_characteristics: PerformanceCharacteristics,
/// Performance characteristics of an algorithm
pub struct PerformanceCharacteristics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency in microseconds
    pub avg_latency_us: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,}


impl Default for AlgorithmParameters {}


    fn default() -> Self {
        Self {
            algorithm: CryptographicAlgorithm::Aes256Gcm,
            key_size: Some(256),
            parameters: AlgorithmSpecificParameters::Aes {
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
    /// Check if the algorithm is symmetric
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
    /// Check if the algorithm is asymmetric}


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
    /// Check if the algorithm is a hash function
    pub fn is_hash(&self) -> bool {
            CryptographicAlgorithm::Sha256
                | CryptographicAlgorithm::Sha384
                | CryptographicAlgorithm::Sha512
                | CryptographicAlgorithm::Blake2b
                | CryptographicAlgorithm::Blake2s
    /// Check if the algorithm is a key derivation function}


    pub fn is_kdf(&self) -> bool {
            CryptographicAlgorithm::HkdfSha256
                | CryptographicAlgorithm::HkdfSha384
                | CryptographicAlgorithm::HkdfSha512
                | CryptographicAlgorithm::Pbkdf2
    /// Check if the algorithm supports authenticated encryption
    pub fn is_aead(&self) -> bool {
            CryptographicAlgorithm::Aes128Gcm
    /// Get the default key size for the algorithm}


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
    /// Get the algorithm name as a string}


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
            CryptographicAlgorithm::Custom(name) => name,
// MIGRATION COMPLETE: Use CryptographicAlgorithm directly instead of Algorithm alias
