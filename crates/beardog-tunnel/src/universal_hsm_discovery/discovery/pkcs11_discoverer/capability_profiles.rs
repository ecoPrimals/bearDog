// SPDX-License-Identifier: AGPL-3.0-or-later

//! Static [`UniversalHsmCapabilities`] matrices for PKCS#11 vendor profiles.

use super::discoverer::Pkcs11Discoverer;
use crate::tunnel::hsm::types::capability::*;
use crate::universal_hsm_discovery::*;

impl Pkcs11Discoverer {
    /// Create capabilities for enterprise HSMs
    pub(crate) fn create_enterprise_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec![
                    "AES-128".to_string(),
                    "AES-256".to_string(),
                    "3DES".to_string(),
                ],
                asymmetric_encryption: vec![
                    "RSA-2048".to_string(),
                    "RSA-4096".to_string(),
                    "ECC-P256".to_string(),
                    "ECC-P384".to_string(),
                    "ECC-P521".to_string(),
                ],
                signing: vec![
                    "RSA-PSS".to_string(),
                    "ECDSA".to_string(),
                    "Ed25519".to_string(),
                ],
                hashing: vec![
                    "SHA-256".to_string(),
                    "SHA-384".to_string(),
                    "SHA-512".to_string(),
                ],
                key_agreement: vec!["ECDH".to_string(), "X25519".to_string()],
            },
            key_management: KeyManagementCapabilities {
                key_generation: true,
                key_storage: true,
                key_rotation: true,
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 3072, 4096],
                ecc: vec![256, 384, 521],
                aes: vec![128, 192, 256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier1,
                fips_140_2_level: Some(3),
                common_criteria_eal: Some(4),
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 10000,
                typical_latency_ms: 5.0,
                supports_parallel_operations: true,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: true,
                common_criteria: true,
                pci_dss: true,
                hipaa: true,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: true,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: true,
                multi_party_computation: true,
                threshold_cryptography: true,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for cloud HSMs
    fn create_cloud_hsm_capabilities(&self) -> UniversalHsmCapabilities {
        let mut caps = self.create_enterprise_hsm_capabilities();
        caps.performance.max_operations_per_second = 25000;
        caps.performance.typical_latency_ms = 10.0;
        caps.security.fips_140_2_level = Some(3);
        caps
    }

    /// Create capabilities for YubiKey via PKCS#11
    fn create_yubikey_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
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
                fips_140_2_level: Some(2),
                common_criteria_eal: None,
                secure_boot: true,
                attestation: true,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100,
                typical_latency_ms: 50.0,
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
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for TPM via PKCS#11
    fn create_tpm_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string(), "AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string(), "ECC-P256".to_string()],
                signing: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                hashing: vec!["SHA-256".to_string(), "SHA-384".to_string()],
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
                fips_140_2_level: Some(2),
                common_criteria_eal: Some(4),
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
                common_criteria: true,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: true,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for SoftHSM
    fn create_softhsm_capabilities(&self) -> UniversalHsmCapabilities {
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
                key_backup: true,
                key_recovery: true,
            },
            key_generation: KeyGenerationCapabilities {
                rsa: vec![2048, 4096],
                ecc: vec![256, 384],
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
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: true,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for SmartCard tokens
    fn create_smartcard_capabilities(&self) -> UniversalHsmCapabilities {
        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-128".to_string()],
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
                aes: vec![128],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier3,
                fips_140_2_level: None,
                common_criteria_eal: Some(4),
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 50,
                typical_latency_ms: 100.0,
                supports_parallel_operations: false,
                hardware_acceleration: true,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: true,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }

    /// Create capabilities for generic PKCS#11 tokens
    fn create_generic_pkcs11_capabilities(&self) -> UniversalHsmCapabilities {
        UniversalHsmCapabilities {
            crypto_operations: CryptoOperationCapabilities {
                symmetric_encryption: vec!["AES-256".to_string()],
                asymmetric_encryption: vec!["RSA-2048".to_string()],
                signing: vec!["RSA-PSS".to_string()],
                hashing: vec!["SHA-256".to_string()],
                key_agreement: vec![],
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
                ecc: vec![],
                aes: vec![256],
                supports_secure_random: true,
            },
            security: SecurityCapabilities {
                tamper_resistance: TamperResistance::Tier4,
                fips_140_2_level: None,
                common_criteria_eal: None,
                secure_boot: false,
                attestation: false,
            },
            performance: PerformanceCapabilities {
                max_operations_per_second: 100,
                typical_latency_ms: 50.0,
                supports_parallel_operations: false,
                hardware_acceleration: false,
            },
            compliance: ComplianceCapabilities {
                fips_140_2: false,
                common_criteria: false,
                pci_dss: false,
                hipaa: false,
                gdpr: true,
            },
            api_support: ApiSupportCapabilities {
                pkcs11: true,
                tpm2: false,
                kmip: false,
                pkcs7: false,
            },
            advanced_features: AdvancedFeatureCapabilities {
                quantum_resistant: false,
                multi_party_computation: false,
                threshold_cryptography: false,
                homomorphic_encryption: false,
            },
            human_entropy: HumanEntropyCapabilities {
                supported: false,
                methods: vec![],
                quality_score: 0.0,
            },
        }
    }
}
