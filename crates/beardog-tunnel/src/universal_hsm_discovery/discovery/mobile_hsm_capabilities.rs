// SPDX-License-Identifier: AGPL-3.0-or-later

//! Static capability profiles for mobile HSM platforms.
//!
//! Each function returns a [`UniversalHsmCapabilities`] describing the algorithms, key management,
//! security, performance, and compliance properties of one mobile HSM type. These profiles are
//! static data — no runtime detection. Discovery logic lives in [`super::mobile_discoverer`].

use crate::tunnel::hsm::types::capability::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, ComplianceCapabilities,
    CryptoOperationCapabilities, HsmCapabilities as UniversalHsmCapabilities,
    HumanEntropyCapabilities, KeyGenerationCapabilities, KeyManagementCapabilities,
    PerformanceCapabilities, SecurityCapabilities, TamperResistanceLevel as TamperResistance,
};

fn no_api_support() -> ApiSupportCapabilities {
    ApiSupportCapabilities {
        pkcs11: false,
        tpm2: false,
        kmip: false,
        pkcs7: false,
    }
}

fn no_advanced_features() -> AdvancedFeatureCapabilities {
    AdvancedFeatureCapabilities {
        quantum_resistant: false,
        multi_party_computation: false,
        threshold_cryptography: false,
        homomorphic_encryption: false,
    }
}

fn no_human_entropy() -> HumanEntropyCapabilities {
    HumanEntropyCapabilities {
        supported: false,
        methods: vec![],
        quality_score: 0.0,
    }
}

/// iOS Secure Enclave: hardware-backed, FIPS 140-2 Level 2, no key export.
#[must_use]
pub fn ios_secure_enclave() -> UniversalHsmCapabilities {
    UniversalHsmCapabilities {
        crypto_operations: CryptoOperationCapabilities {
            symmetric_encryption: vec!["AES-256".to_string()],
            asymmetric_encryption: vec!["ECC-P256".to_string()],
            signing: vec!["ECDSA".to_string()],
            hashing: vec!["SHA-256".to_string()],
            key_agreement: vec!["ECDH".to_string()],
        },
        key_management: KeyManagementCapabilities {
            key_generation: true,
            key_storage: true,
            key_rotation: false,
            key_backup: false,
            key_recovery: false,
        },
        key_generation: KeyGenerationCapabilities {
            rsa: vec![],
            ecc: vec![256],
            aes: vec![256],
            supports_secure_random: true,
        },
        security: SecurityCapabilities {
            tamper_resistance: TamperResistance::Tier1,
            fips_140_2_level: Some(2),
            common_criteria_eal: None,
            secure_boot: true,
            attestation: true,
        },
        performance: PerformanceCapabilities {
            max_operations_per_second: 1000,
            typical_latency_ms: 10.0,
            supports_parallel_operations: false,
            hardware_acceleration: true,
        },
        compliance: ComplianceCapabilities {
            fips_140_2: true,
            common_criteria: false,
            pci_dss: false,
            hipaa: true,
            gdpr: true,
        },
        api_support: no_api_support(),
        advanced_features: no_advanced_features(),
        human_entropy: no_human_entropy(),
    }
}

/// iOS Keychain: software-backed, broader algorithm support, iCloud backup capable.
#[must_use]
pub fn ios_keychain() -> UniversalHsmCapabilities {
    UniversalHsmCapabilities {
        crypto_operations: CryptoOperationCapabilities {
            symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
            asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
            signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
            hashing: vec!["SHA-256".to_string()],
            key_agreement: vec!["ECDH".to_string()],
        },
        key_management: KeyManagementCapabilities {
            key_generation: true,
            key_storage: true,
            key_rotation: false,
            key_backup: true,
            key_recovery: true,
        },
        key_generation: KeyGenerationCapabilities {
            rsa: vec![2048],
            ecc: vec![256],
            aes: vec![128, 256],
            supports_secure_random: true,
        },
        security: SecurityCapabilities {
            tamper_resistance: TamperResistance::Tier3,
            fips_140_2_level: None,
            common_criteria_eal: None,
            secure_boot: false,
            attestation: false,
        },
        performance: PerformanceCapabilities {
            max_operations_per_second: 5000,
            typical_latency_ms: 1.0,
            supports_parallel_operations: true,
            hardware_acceleration: false,
        },
        compliance: ComplianceCapabilities {
            fips_140_2: false,
            common_criteria: false,
            pci_dss: false,
            hipaa: false,
            gdpr: true,
        },
        api_support: no_api_support(),
        advanced_features: no_advanced_features(),
        human_entropy: no_human_entropy(),
    }
}

/// Android `StrongBox`: hardware SE, FIPS certified, no key export.
#[must_use]
pub fn android_strongbox() -> UniversalHsmCapabilities {
    UniversalHsmCapabilities {
        crypto_operations: CryptoOperationCapabilities {
            symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
            asymmetric_encryption: vec![
                "RSA-2048".to_string(),
                "RSA-4096".to_string(),
                "ECC-P256".to_string(),
            ],
            signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
            hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
            key_agreement: vec!["ECDH".to_string()],
        },
        key_management: KeyManagementCapabilities {
            key_generation: true,
            key_storage: true,
            key_rotation: false,
            key_backup: false,
            key_recovery: false,
        },
        key_generation: KeyGenerationCapabilities {
            rsa: vec![2048, 4096],
            ecc: vec![256],
            aes: vec![128, 256],
            supports_secure_random: true,
        },
        security: SecurityCapabilities {
            tamper_resistance: TamperResistance::Tier1,
            fips_140_2_level: Some(1),
            common_criteria_eal: None,
            secure_boot: true,
            attestation: true,
        },
        performance: PerformanceCapabilities {
            max_operations_per_second: 500,
            typical_latency_ms: 20.0,
            supports_parallel_operations: false,
            hardware_acceleration: true,
        },
        compliance: ComplianceCapabilities {
            fips_140_2: true,
            common_criteria: false,
            pci_dss: false,
            hipaa: false,
            gdpr: true,
        },
        api_support: no_api_support(),
        advanced_features: no_advanced_features(),
        human_entropy: no_human_entropy(),
    }
}

/// Android Keystore: software or hardware-backed, Tier 2 tamper resistance.
#[must_use]
pub fn android_keystore() -> UniversalHsmCapabilities {
    UniversalHsmCapabilities {
        crypto_operations: CryptoOperationCapabilities {
            symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
            asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
            signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
            hashing: vec!["SHA-256".to_string()],
            key_agreement: vec!["ECDH".to_string()],
        },
        key_management: KeyManagementCapabilities {
            key_generation: true,
            key_storage: true,
            key_rotation: false,
            key_backup: false,
            key_recovery: false,
        },
        key_generation: KeyGenerationCapabilities {
            rsa: vec![2048],
            ecc: vec![256],
            aes: vec![128, 256],
            supports_secure_random: true,
        },
        security: SecurityCapabilities {
            tamper_resistance: TamperResistance::Tier2,
            fips_140_2_level: None,
            common_criteria_eal: None,
            secure_boot: true,
            attestation: true,
        },
        performance: PerformanceCapabilities {
            max_operations_per_second: 2000,
            typical_latency_ms: 5.0,
            supports_parallel_operations: true,
            hardware_acceleration: true,
        },
        compliance: ComplianceCapabilities {
            fips_140_2: false,
            common_criteria: false,
            pci_dss: false,
            hipaa: false,
            gdpr: true,
        },
        api_support: no_api_support(),
        advanced_features: no_advanced_features(),
        human_entropy: no_human_entropy(),
    }
}

/// Samsung Knox: CC EAL5+, FIPS certified, backup-capable.
#[must_use]
pub fn samsung_knox() -> UniversalHsmCapabilities {
    UniversalHsmCapabilities {
        crypto_operations: CryptoOperationCapabilities {
            symmetric_encryption: vec!["AES-256".to_string()],
            asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
            signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
            hashing: vec!["SHA-256".to_string(), "SHA-512".to_string()],
            key_agreement: vec!["ECDH".to_string()],
        },
        key_management: KeyManagementCapabilities {
            key_generation: true,
            key_storage: true,
            key_rotation: false,
            key_backup: true,
            key_recovery: true,
        },
        key_generation: KeyGenerationCapabilities {
            rsa: vec![2048],
            ecc: vec![256],
            aes: vec![256],
            supports_secure_random: true,
        },
        security: SecurityCapabilities {
            tamper_resistance: TamperResistance::Tier1,
            fips_140_2_level: Some(1),
            common_criteria_eal: Some(5),
            secure_boot: true,
            attestation: true,
        },
        performance: PerformanceCapabilities {
            max_operations_per_second: 1000,
            typical_latency_ms: 15.0,
            supports_parallel_operations: false,
            hardware_acceleration: true,
        },
        compliance: ComplianceCapabilities {
            fips_140_2: true,
            common_criteria: true,
            pci_dss: false,
            hipaa: true,
            gdpr: true,
        },
        api_support: no_api_support(),
        advanced_features: no_advanced_features(),
        human_entropy: no_human_entropy(),
    }
}
