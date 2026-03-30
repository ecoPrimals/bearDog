// SPDX-License-Identifier: AGPL-3.0-only

//! Structured capability matrices advertised by HSM devices (crypto, performance, APIs).

use crate::constants::defaults;
use serde::{Deserialize, Serialize};

/// HSM hardware capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilities {
    /// HSM provider/manufacturer
    pub provider: String,
    /// HSM model
    /// The model value
    pub model: String,
    /// Firmware version
    /// The firmware version value
    pub firmware_version: String,
    /// Hardware version
    /// The hardware version value
    pub hardware_version: String,
    /// Security certifications
    /// Collection of certifications
    pub certifications: Vec<String>,
    /// Key generation capabilities
    /// The key generation value
    pub key_generation: KeyGenerationCapabilities,
    /// Key management capabilities
    /// The key management value
    pub key_management: KeyManagementCapabilities,
    /// Cryptographic capabilities
    /// The cryptographic value
    pub cryptographic: CryptographicCapabilities,
    /// Security capabilities
    /// The security value
    pub security: SecurityCapabilities,
    /// Throughput, latency, and parallelism advertised by the device.
    pub performance: PerformanceCapabilities,
    /// Advanced features
    /// The advanced features value
    pub advanced_features: AdvancedFeatureCapabilities,
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            provider: "Generic HSM".to_string(),
            model: "Standard".to_string(),
            firmware_version: "1.0.0".to_string(),
            hardware_version: "1.0".to_string(),
            certifications: vec!["FIPS-140-2".to_string(), "Common Criteria".to_string()],
            key_generation: KeyGenerationCapabilities::default(),
            key_management: KeyManagementCapabilities::default(),
            cryptographic: CryptographicCapabilities::default(),
            security: SecurityCapabilities::default(),
            performance: PerformanceCapabilities::default(),
            advanced_features: AdvancedFeatureCapabilities::default(),
        }
    }
}

/// Key generation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationCapabilities {
    /// Supported key algorithms
    /// Collection of supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Supported key sizes
    /// Collection of supported key sizes
    pub supported_key_sizes: Vec<u32>,
    /// True random number generation
    /// Whether `true_random_generation` is enabled
    pub true_random_generation: bool,
    /// Hardware entropy source
    /// Whether `hardware_entropy` is enabled
    pub hardware_entropy: bool,
    /// Key derivation support
    /// Whether `key_derivation` is enabled
    pub key_derivation: bool,
    /// Deterministic key generation
    /// Whether `deterministic_generation` is enabled
    pub deterministic_generation: bool,
}

impl Default for KeyGenerationCapabilities {
    fn default() -> Self {
        Self {
            supported_algorithms: vec![
                "AES".to_string(),
                "RSA".to_string(),
                "ECC".to_string(),
                "HMAC".to_string(),
            ],
            supported_key_sizes: vec![128, 192, 256, 1024, 2048, 3072, 4096],
            true_random_generation: true,
            hardware_entropy: true,
            key_derivation: true,
            deterministic_generation: false,
        }
    }
}

/// Key management capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementCapabilities {
    /// Maximum number of keys
    /// Optional max keys
    pub max_keys: Option<u32>,
    /// Key backup support
    /// Whether `backup_support` is enabled
    pub backup_support: bool,
    /// Key recovery support
    /// Whether `recovery_support` is enabled
    pub recovery_support: bool,
    /// Key escrow support
    /// Whether `escrow_support` is enabled
    pub escrow_support: bool,
    /// Key rotation support
    /// Whether `rotation_support` is enabled
    pub rotation_support: bool,
    /// Key lifecycle management
    /// Whether `lifecycle_management` is enabled
    pub lifecycle_management: bool,
    /// Key versioning
    /// Whether versioning is enabled
    pub versioning: bool,
}

impl Default for KeyManagementCapabilities {
    fn default() -> Self {
        Self {
            max_keys: Some(
                std::env::var("BEARDOG_HSM_MAX_KEYS")
                    .ok()
                    .and_then(|k| k.parse().ok())
                    .unwrap_or_else(|| {
                        u32::try_from(defaults::DEFAULT_MAX_ENTRIES).unwrap_or(u32::MAX)
                    }), // 10K keys default
            ),
            backup_support: true,
            recovery_support: true,
            escrow_support: false,
            rotation_support: true,
            lifecycle_management: true,
            versioning: true,
        }
    }
}

/// Cryptographic operation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptographicCapabilities {
    /// Supported encryption algorithms
    /// Collection of encryption algorithms
    pub encryption_algorithms: Vec<String>,
    /// Supported signature algorithms
    /// Collection of signature algorithms
    pub signature_algorithms: Vec<String>,
    /// Supported hash algorithms
    /// Collection of hash algorithms
    pub hash_algorithms: Vec<String>,
    /// Supported key agreement algorithms
    /// Collection of key agreement algorithms
    pub key_agreement_algorithms: Vec<String>,
    /// Hardware acceleration
    /// Whether `hardware_acceleration` is enabled
    pub hardware_acceleration: bool,
    /// Batch operations support
    /// Whether `batch_operations` is enabled
    pub batch_operations: bool,
}

impl Default for CryptographicCapabilities {
    fn default() -> Self {
        Self {
            encryption_algorithms: vec![
                "AES-GCM".to_string(),
                "AES-CBC".to_string(),
                "RSA-OAEP".to_string(),
                "ChaCha20-Poly1305".to_string(),
            ],
            signature_algorithms: vec![
                "RSA-PSS".to_string(),
                "ECDSA".to_string(),
                "Ed25519".to_string(),
                "HMAC".to_string(),
            ],
            hash_algorithms: vec![
                "SHA-256".to_string(),
                "SHA-384".to_string(),
                "SHA-512".to_string(),
                "SHA3-256".to_string(),
            ],
            key_agreement_algorithms: vec!["ECDH".to_string(), "X25519".to_string()],
            hardware_acceleration: true,
            batch_operations: true,
        }
    }
}

/// Security feature capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Authentication methods supported
    /// Collection of authentication methods
    pub authentication_methods: Vec<String>,
    /// Access control support
    /// Whether `access_control` is enabled
    pub access_control: bool,
    /// Audit logging
    /// Whether `audit_logging` is enabled
    pub audit_logging: bool,
    /// Tamper detection
    /// Whether `tamper_detection` is enabled
    pub tamper_detection: bool,
    /// Tamper response
    /// Whether `tamper_response` is enabled
    pub tamper_response: bool,
    /// Secure boot
    /// Whether `secure_boot` is enabled
    pub secure_boot: bool,
    /// Physical security level
    /// The physical security level value
    pub physical_security_level: String,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            authentication_methods: vec![
                "password".to_string(),
                "smart_card".to_string(),
                "biometric".to_string(),
            ],
            access_control: true,
            audit_logging: true,
            tamper_detection: true,
            tamper_response: true,
            secure_boot: true,
            physical_security_level: "Level 3".to_string(),
        }
    }
}

/// Advertised throughput and latency envelope for an HSM device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    /// Maximum operations per second
    /// Number of `max_operations_per_second`
    pub max_operations_per_second: u32,
    /// Concurrent operation support
    /// Number of `concurrent_operations`
    pub concurrent_operations: u32,
    /// Average latency in milliseconds
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// Throughput in MB/s
    /// The throughput mbps value
    pub throughput_mbps: f64,
    /// Load balancing support
    /// Whether `load_balancing` is enabled
    pub load_balancing: bool,
    /// High availability support
    /// Whether `high_availability` is enabled
    pub high_availability: bool,
}

impl Default for PerformanceCapabilities {
    fn default() -> Self {
        Self {
            max_operations_per_second: std::env::var("BEARDOG_HSM_MAX_OPS_PER_SEC")
                .ok()
                .and_then(|o| o.parse().ok())
                .unwrap_or_else(|| {
                    u32::try_from(defaults::DEFAULT_MAX_ENTRIES).unwrap_or(u32::MAX)
                }), // 10K ops/sec default
            concurrent_operations: 100,
            average_latency_ms: 5.0,
            throughput_mbps: 100.0,
            load_balancing: true,
            high_availability: true,
        }
    }
}

/// Advanced HSM features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeatureCapabilities {
    /// Secure enclaves support
    /// Whether `secure_enclaves` is enabled
    pub secure_enclaves: bool,
    /// Remote attestation
    /// Whether `remote_attestation` is enabled
    pub remote_attestation: bool,
    /// Secure multi-party computation
    /// Whether `secure_multiparty_computation` is enabled
    pub secure_multiparty_computation: bool,
    /// Homomorphic encryption support
    /// Whether `homomorphic_encryption` is enabled
    pub homomorphic_encryption: bool,
    /// Zero-knowledge proof support
    /// Whether `zero_knowledge_proofs` is enabled
    pub zero_knowledge_proofs: bool,
    /// Post-quantum cryptography
    /// Whether `post_quantum_crypto` is enabled
    pub post_quantum_crypto: bool,
    /// Custom firmware support
    /// Whether `custom_firmware` is enabled
    pub custom_firmware: bool,
    /// API extensions
    /// Collection of api extensions
    pub api_extensions: Vec<String>,
}

impl Default for AdvancedFeatureCapabilities {
    fn default() -> Self {
        Self {
            secure_enclaves: false,
            remote_attestation: true,
            secure_multiparty_computation: false,
            homomorphic_encryption: false,
            zero_knowledge_proofs: false,
            post_quantum_crypto: false,
            custom_firmware: false,
            api_extensions: Vec::new(),
        }
    }
}

/// HSM capability query and validation
impl HsmCapabilities {
    /// Materializes [`Default`] capability advertisement (used in tests and synthetic HSMs).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns true when `algorithm` appears in [`KeyGenerationCapabilities::supported_algorithms`].
    #[must_use]
    pub fn supports_key_algorithm(&self, algorithm: &str) -> bool {
        self.key_generation
            .supported_algorithms
            .contains(&algorithm.to_string())
    }

    /// Check if a specific key size is supported
    #[must_use]
    pub fn supports_key_size(&self, size: u32) -> bool {
        self.key_generation.supported_key_sizes.contains(&size)
    }

    /// Check if a specific encryption algorithm is supported
    #[must_use]
    pub fn supports_encryption(&self, algorithm: &str) -> bool {
        self.cryptographic
            .encryption_algorithms
            .contains(&algorithm.to_string())
    }

    /// Check if a specific signature algorithm is supported
    #[must_use]
    pub fn supports_signature(&self, algorithm: &str) -> bool {
        self.cryptographic
            .signature_algorithms
            .contains(&algorithm.to_string())
    }

    /// Get the maximum number of keys that can be stored
    #[must_use]
    pub const fn max_key_capacity(&self) -> Option<u32> {
        self.key_management.max_keys
    }

    /// Check if the HSM meets minimum security requirements
    #[must_use]
    pub fn meets_security_requirements(&self, min_level: &str) -> bool {
        // Simple level comparison - in practice this would be more sophisticated
        matches!(
            (self.security.physical_security_level.as_str(), min_level),
            ("Level 4", _)
                | ("Level 3", "Level 1" | "Level 2" | "Level 3")
                | ("Level 2", "Level 1" | "Level 2")
                | ("Level 1", "Level 1")
        )
    }

    /// Heuristic score in `[0.0, 1.0]` combining ops/sec, latency, and throughput.
    #[must_use]
    pub fn performance_rating(&self) -> f64 {
        let ops_score =
            (f64::from(self.performance.max_operations_per_second) / 100_000.0).min(1.0);
        let latency_score = (10.0 / self.performance.average_latency_ms).min(1.0);
        let throughput_score = (self.performance.throughput_mbps / 1000.0).min(1.0);

        (ops_score + latency_score + throughput_score) / 3.0
    }

    /// Returns true when any “advanced” cryptographic or isolation feature is advertised.
    #[must_use]
    pub const fn has_advanced_features(&self) -> bool {
        self.advanced_features.secure_enclaves
            || self.advanced_features.remote_attestation
            || self.advanced_features.post_quantum_crypto
    }

    /// Get a summary of key capabilities
    #[must_use]
    pub fn capability_summary(&self) -> CapabilitySummary {
        CapabilitySummary {
            total_algorithms: self.key_generation.supported_algorithms.len(),
            total_key_sizes: self.key_generation.supported_key_sizes.len(),
            security_level: self.security.physical_security_level.clone(),
            performance_rating: self.performance_rating(),
            certifications: self.certifications.len(),
            advanced_features: self.has_advanced_features(),
        }
    }
}

/// Summary of HSM capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitySummary {
    /// Total number of supported algorithms
    /// Number of `total_algorithms`
    pub total_algorithms: usize,
    /// Total number of supported key sizes
    /// Number of `total_key_sizes`
    pub total_key_sizes: usize,
    /// Security level
    /// The security level value
    pub security_level: String,
    /// Normalized score from [`HsmCapabilities::performance_rating`].
    pub performance_rating: f64,
    /// Number of certifications
    pub certifications: usize,
    /// Advanced features available
    /// Whether `advanced_features` is enabled
    pub advanced_features: bool,
}

/// Minimum capability set required before selecting an HSM for a workload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Required algorithms
    /// Collection of required algorithms
    pub required_algorithms: Vec<String>,
    /// Required key sizes
    /// Collection of required key sizes
    pub required_key_sizes: Vec<u32>,
    /// Minimum security level
    /// The min security level value
    pub min_security_level: String,
    /// Floor for ops/sec, latency, and throughput checks in [`HsmCapabilities::meets_requirements`].
    pub min_performance: PerformanceRequirements,
    /// Required certifications
    /// Collection of required certifications
    pub required_certifications: Vec<String>,
    /// Required advanced features
    /// Collection of required advanced features
    pub required_advanced_features: Vec<String>,
}

/// Performance floor used with [`CapabilityRequirements`] (distinct from config-tier types).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRequirements {
    /// Minimum operations per second
    /// Number of `min_ops_per_second`
    pub min_ops_per_second: u32,
    /// Maximum acceptable latency in milliseconds
    /// The max latency ms value
    pub max_latency_ms: f64,
    /// Minimum throughput in MB/s
    /// The min throughput mbps value
    pub min_throughput_mbps: f64,
    /// Concurrent operations required
    /// Number of `min_concurrent_operations`
    pub min_concurrent_operations: u32,
}

impl Default for PerformanceRequirements {
    fn default() -> Self {
        Self {
            min_ops_per_second: 1000,
            max_latency_ms: 50.0,
            min_throughput_mbps: 10.0,
            min_concurrent_operations: 10,
        }
    }
}

impl HsmCapabilities {
    /// Check if this HSM meets the specified requirements
    #[must_use]
    pub fn meets_requirements(&self, requirements: &CapabilityRequirements) -> bool {
        // Check algorithms
        for algorithm in &requirements.required_algorithms {
            if !self.supports_key_algorithm(algorithm) {
                return false;
            }
        }

        // Check key sizes
        for &size in &requirements.required_key_sizes {
            if !self.supports_key_size(size) {
                return false;
            }
        }

        // Check security level
        if !self.meets_security_requirements(&requirements.min_security_level) {
            return false;
        }

        // Check performance
        let perf = &requirements.min_performance;
        if self.performance.max_operations_per_second < perf.min_ops_per_second
            || self.performance.average_latency_ms > perf.max_latency_ms
            || self.performance.throughput_mbps < perf.min_throughput_mbps
            || self.performance.concurrent_operations < perf.min_concurrent_operations
        {
            return false;
        }

        // Check certifications
        for cert in &requirements.required_certifications {
            if !self.certifications.contains(cert) {
                return false;
            }
        }

        true
    }
}

/// Control-plane protocols exposed by the HSM vendor (REST, gRPC, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSupportCapabilities {
    /// Supported API versions
    /// Collection of api versions
    pub api_versions: Vec<String>,
    /// REST API support
    /// Whether `rest_api_support` is enabled
    pub rest_api_support: bool,
    /// GraphQL API support
    /// Whether `graphql_support` is enabled
    pub graphql_support: bool,
    /// WebSocket support
    /// Whether `websocket_support` is enabled
    pub websocket_support: bool,
    /// gRPC support
    /// Whether `grpc_support` is enabled
    pub grpc_support: bool,
    /// Collection of auth methods
    pub auth_methods: Vec<String>,
    /// Rate limiting capabilities
    /// Whether `rate_limiting` is enabled
    pub rate_limiting: bool,
    /// API documentation availability
    /// Whether `documentation_available` is enabled
    pub documentation_available: bool,
    /// SDK availability
    /// Collection of sdk languages
    pub sdk_languages: Vec<String>,
}

impl Default for ApiSupportCapabilities {
    fn default() -> Self {
        Self {
            api_versions: vec!["v1".to_string(), "v2".to_string()],
            rest_api_support: true,
            graphql_support: false,
            websocket_support: true,
            grpc_support: true,
            auth_methods: vec!["Bearer".to_string(), "API-Key".to_string()],
            rate_limiting: true,
            documentation_available: true,
            sdk_languages: vec!["rust".to_string(), "python".to_string()],
        }
    }
}
