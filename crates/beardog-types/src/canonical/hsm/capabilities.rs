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


/// # Canonical HSM Capabilities
///
/// **SINGLE SOURCE OF TRUTH** for all HSM capability-related types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **UNIVERSAL HSM CAPABILITIES** - What operations and features an HSM supports across all vendors
/// This structure now supports all HSM patterns including human entropy collection,
/// making it truly universal and vendor-agnostic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilities {
    /// HSM vendor/manufacturer
    pub vendor: String,
    /// HSM model/product name
    pub model: String,
    /// Firmware version
    pub firmware_version: String,
    /// Supported cryptographic algorithms
    pub supported_algorithms: Vec<String>,
    /// Supported key types (simplified for compatibility)
    pub supported_key_types: Vec<String>,
    /// Maximum number of keys that can be stored
    pub max_keys: Option<u32>,
    /// Supported key operations
    pub supported_operations: Vec<String>,
    /// Hardware security features
    pub security_features: Vec<String>,
    /// Performance characteristics
    pub performance_metrics: HashMap<String, String>,
    /// Compliance certifications
    pub certifications: Vec<String>,
    /// Key generation capabilities
    pub key_generation: KeyGenerationCapabilities,
    /// Key management capabilities
    pub key_management: KeyManagementCapabilities,
    /// Advanced feature capabilities
    pub advanced_features: AdvancedFeatureCapabilities,
    /// API support capabilities
    pub api_support: ApiSupportCapabilities,
    /// Security capabilities
    pub security: SecurityCapabilities,
    // === UNIVERSAL EXTENSIONS ===
    /// Human entropy collection capabilities (NEW - universal support)
    pub human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities,
    /// Performance capabilities (enhanced universal metrics)
    pub performance: crate::canonical::capabilities::PerformanceCapabilities,
    /// Compliance capabilities (enhanced universal compliance)
    pub compliance: crate::canonical::capabilities::ComplianceCapabilities,
    /// Vendor-specific capabilities (extensible for any HSM vendor)
    pub vendor_capabilities: HashMap<String, serde_json::Value>,
    /// Custom capabilities (fully extensible for future needs)
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}
/// **KEY GENERATION CAPABILITIES** - Key generation features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationCapabilities {
    /// Whether keys are generated in hardware
    pub hardware_generation: bool,
    /// Supported key sizes in bits
    pub supported_key_sizes: Vec<u32>,
    /// Key generation speed (keys per second)
    pub generation_speed: Option<u32>,
}

/// **KEY MANAGEMENT CAPABILITIES** - Key management features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementCapabilities {
    /// Maximum number of keys
    pub max_keys: Option<u32>,
    /// Backup and recovery support
    pub backup_recovery: bool,
    /// Key migration support
    pub key_migration: bool,
    /// Key versioning support
    pub key_versioning: bool,
    /// Key lifecycle management
    pub lifecycle_management: bool,
}

/// **ADVANCED FEATURE CAPABILITIES** - Advanced HSM features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvancedFeatureCapabilities {
    /// Physical security level
    pub physical_security_level: String,
    /// Tamper resistance level
    pub tamper_resistance: bool,
    /// Secure boot support
    pub secure_boot: bool,
    /// Attestation support
    pub attestation: bool,
    /// Hardware random number generator
    pub hardware_rng: bool,
    /// Side-channel attack resistance
    pub side_channel_resistance: bool,
}

/// **API SUPPORT CAPABILITIES** - Supported APIs and interfaces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiSupportCapabilities {
    /// PKCS#11 support
    pub pkcs11: bool,
    /// Microsoft CryptoAPI support
    pub crypto_api: bool,
    /// Java Cryptography Architecture support
    pub jca: bool,
    /// OpenSSL integration
    pub openssl: bool,
    /// REST API support
    pub rest_api: bool,
    /// gRPC API support
    pub grpc_api: bool,
    /// WebSocket support
    pub websocket: bool,
}

/// **SECURITY CAPABILITIES** - Security-specific features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    /// Supported authentication methods
    pub authentication_methods: Vec<String>,
    /// Role-based access control
    pub rbac: bool,
    /// Audit logging
    pub audit_logging: bool,
    /// Secure communication protocols
    pub secure_protocols: Vec<String>,
    pub compliance_certifications: Vec<String>,
    /// Security assurance level
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
            supported_key_types: vec![
                "AES".to_string(),
                "RSA".to_string(),
                "ECDSA".to_string(),
            ],
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
            performance_metrics: HashMap::new(),
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
            // Universal extensions with defaults
            human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities::default(),
            performance: crate::canonical::capabilities::PerformanceCapabilities::default(),
            compliance: crate::canonical::capabilities::ComplianceCapabilities::default(),
            vendor_capabilities: HashMap::new(),
            custom_capabilities: HashMap::new(),
        }
    }
}
