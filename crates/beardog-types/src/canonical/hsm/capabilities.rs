use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilities {
    pub vendor: String,

    pub model: String,

    pub firmware_version: String,

    pub supported_algorithms: Vec<String>,

    pub supported_key_types: Vec<String>,

    pub max_keys: Option<u32>,

    pub supported_operations: Vec<String>,

    pub security_features: Vec<String>,

    pub performance_metrics: HashMap<String, String>,

    pub certifications: Vec<String>,

    pub key_generation: KeyGenerationCapabilities,

    pub key_management: KeyManagementCapabilities,

    pub advanced_features: AdvancedFeatureCapabilities,

    pub api_support: ApiSupportCapabilities,

    pub security: SecurityCapabilities,

    pub human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities,

    pub performance: crate::canonical::capabilities::PerformanceCapabilities,

    pub compliance: crate::canonical::capabilities::ComplianceCapabilities,

    pub vendor_capabilities: HashMap<String, serde_json::Value>,

    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationCapabilities {
    pub hardware_generation: bool,

    pub supported_key_sizes: Vec<u32>,

    pub generation_speed: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementCapabilities {
    pub max_keys: Option<u32>,

    pub backup_recovery: bool,

    pub key_migration: bool,

    pub key_versioning: bool,

    pub lifecycle_management: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeatureCapabilities {
    pub physical_security_level: String,

    pub tamper_resistance: bool,

    pub secure_boot: bool,

    pub attestation: bool,

    pub hardware_rng: bool,

    pub side_channel_resistance: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSupportCapabilities {
    pub pkcs11: bool,

    pub crypto_api: bool,

    pub jca: bool,

    pub openssl: bool,

    pub rest_api: bool,

    pub grpc_api: bool,

    pub websocket: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    pub authentication_methods: Vec<String>,

    pub rbac: bool,

    pub audit_logging: bool,

    pub secure_protocols: Vec<String>,
    pub compliance_certifications: Vec<String>,

    pub security_level: String,
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            vendor: "BearDog".to_string(),
            model: "Software HSM".to_string(),
            firmware_version: "1.0.0".to_string(),
            supported_algorithms: vec![
                "AES-256".to_string(),
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
                "Ed25519".to_string(),
                "ECDSA-P256".to_string(),
            ],
            supported_key_types: vec!["AES".to_string(), "RSA".to_string(), "ECDSA".to_string()],
            max_keys: Some(1000),
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "sign".to_string(),
                "verify".to_string(),
                "key_generation".to_string(),
                "key_derivation".to_string(),
            ],
            security_features: vec![
                "tamper_resistance".to_string(),
                "secure_key_storage".to_string(),
                "hardware_random_generator".to_string(),
            ],
            performance_metrics: HashMap::with_capacity(16),
            certifications: vec!["FIPS-140-2".to_string()],
            key_generation: KeyGenerationCapabilities {
                hardware_generation: false,
                supported_key_sizes: vec![256, 512, 1024, 2048, 4096],
                generation_speed: Some(1000),
            },
            key_management: KeyManagementCapabilities {
                max_keys: Some(10000),
                backup_recovery: true,
                key_migration: true,
                key_versioning: true,
                lifecycle_management: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                physical_security_level: "Software".to_string(),
                tamper_resistance: false,
                secure_boot: false,
                attestation: false,
                hardware_rng: false,
                side_channel_resistance: false,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                crypto_api: false,
                jca: false,
                openssl: true,
                rest_api: true,
                grpc_api: true,
                websocket: false,
            },
            security: SecurityCapabilities {
                authentication_methods: vec!["none".to_string(), "password".to_string()],
                rbac: true,
                audit_logging: true,
                secure_protocols: vec!["TLS".to_string()],
                compliance_certifications: vec!["FIPS-140-2".to_string()],
                security_level: "Software".to_string(),
            },

            human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities::default(),
            performance: crate::canonical::capabilities::PerformanceCapabilities::default(),
            compliance: crate::canonical::capabilities::ComplianceCapabilities::default(),
            vendor_capabilities: HashMap::with_capacity(16),
            custom_capabilities: HashMap::with_capacity(16),
        }
    }
}
