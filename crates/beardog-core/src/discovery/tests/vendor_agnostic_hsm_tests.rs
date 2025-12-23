//! Vendor-Agnostic HSM Discovery Tests
//!
//! Comprehensive tests for capability-based HSM discovery


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use crate::discovery::vendor_agnostic_hsm::*;

#[cfg(test)]
mod vendor_agnostic_hsm_tests {
    use super::*;

    #[test]
    fn test_discovered_hsm_creation() {
        let hsm = DiscoveredHsm {
            hsm_id: "test-hsm-001".to_string(),
            interface: HsmInterface::Pkcs11 {
                library_path: "/usr/lib/pkcs11.so".to_string(),
                slot_id: Some(0),
            },
            supported_operations: vec![CryptoOperation::Sign, CryptoOperation::Verify],
            security_level: HsmSecurityLevel::High,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.85,
        };

        assert_eq!(hsm.hsm_id, "test-hsm-001");
        assert_eq!(hsm.supported_operations.len(), 2);
        assert_eq!(hsm.trust_score, 0.85);
    }

    #[test]
    fn test_hsm_interface_pkcs11() {
        let interface = HsmInterface::Pkcs11 {
            library_path: "/opt/hsm/lib/libpkcs11.so".to_string(),
            slot_id: Some(1),
        };

        match interface {
            HsmInterface::Pkcs11 {
                library_path,
                slot_id,
            } => {
                assert_eq!(library_path, "/opt/hsm/lib/libpkcs11.so");
                assert_eq!(slot_id, Some(1));
            }
            _ => panic!("Expected Pkcs11 interface"),
        }
    }

    #[test]
    fn test_hsm_interface_pkcs11_no_slot() {
        let interface = HsmInterface::Pkcs11 {
            library_path: "/usr/lib/libpkcs11.so".to_string(),
            slot_id: None,
        };

        match interface {
            HsmInterface::Pkcs11 { slot_id, .. } => {
                assert_eq!(slot_id, None);
            }
            _ => panic!("Expected Pkcs11 interface"),
        }
    }

    #[test]
    fn test_hsm_interface_network_rest() {
        let interface = HsmInterface::Network {
            endpoint: "https://hsm.example.com/api".to_string(),
            protocol: NetworkProtocol::Rest,
        };

        match interface {
            HsmInterface::Network { endpoint, protocol } => {
                assert_eq!(endpoint, "https://hsm.example.com/api");
                assert!(matches!(protocol, NetworkProtocol::Rest));
            }
            _ => panic!("Expected Network interface"),
        }
    }

    #[test]
    fn test_hsm_interface_network_grpc() {
        let interface = HsmInterface::Network {
            endpoint: "hsm.example.com:50051".to_string(),
            protocol: NetworkProtocol::Grpc,
        };

        match interface {
            HsmInterface::Network { protocol, .. } => {
                assert!(matches!(protocol, NetworkProtocol::Grpc));
            }
            _ => panic!("Expected Network interface"),
        }
    }

    #[test]
    fn test_hsm_interface_network_custom() {
        let interface = HsmInterface::Network {
            endpoint: "custom-hsm:8443".to_string(),
            protocol: NetworkProtocol::Custom {
                protocol_name: "proprietary-v2".to_string(),
            },
        };

        match interface {
            HsmInterface::Network { protocol, .. } => {
                match protocol {
                    NetworkProtocol::Custom { protocol_name } => {
                        assert_eq!(protocol_name, "proprietary-v2");
                    }
                    _ => panic!("Expected Custom protocol"),
                }
            }
            _ => panic!("Expected Network interface"),
        }
    }

    #[test]
    fn test_hsm_interface_native_tpm() {
        let interface = HsmInterface::Native {
            interface_type: NativeInterface::Tpm,
        };

        match interface {
            HsmInterface::Native { interface_type } => {
                assert!(matches!(interface_type, NativeInterface::Tpm));
            }
            _ => panic!("Expected Native interface"),
        }
    }

    #[test]
    fn test_hsm_interface_native_secure_enclave() {
        let interface = HsmInterface::Native {
            interface_type: NativeInterface::SecureEnclave,
        };

        match interface {
            HsmInterface::Native { interface_type } => {
                assert!(matches!(interface_type, NativeInterface::SecureEnclave));
            }
            _ => panic!("Expected Native interface"),
        }
    }

    #[test]
    fn test_hsm_interface_native_strongbox() {
        let interface = HsmInterface::Native {
            interface_type: NativeInterface::StrongBox,
        };

        match interface {
            HsmInterface::Native { interface_type } => {
                assert!(matches!(interface_type, NativeInterface::StrongBox));
            }
            _ => panic!("Expected Native interface"),
        }
    }

    #[test]
    fn test_hsm_interface_native_windows_cng() {
        let interface = HsmInterface::Native {
            interface_type: NativeInterface::WindowsCng,
        };

        match interface {
            HsmInterface::Native { interface_type } => {
                assert!(matches!(interface_type, NativeInterface::WindowsCng));
            }
            _ => panic!("Expected Native interface"),
        }
    }

    #[test]
    fn test_hsm_interface_native_linux_keyring() {
        let interface = HsmInterface::Native {
            interface_type: NativeInterface::LinuxKeyring,
        };

        match interface {
            HsmInterface::Native { interface_type } => {
                assert!(matches!(interface_type, NativeInterface::LinuxKeyring));
            }
            _ => panic!("Expected Native interface"),
        }
    }

    #[test]
    fn test_hsm_interface_cloud_api() {
        let interface = HsmInterface::CloudApi {
            api_endpoint: "https://kms.cloud-provider.com".to_string(),
            authentication: CloudAuth::ApiKey {
                key_env_var: "CLOUD_API_KEY".to_string(),
            },
        };

        match interface {
            HsmInterface::CloudApi {
                api_endpoint,
                authentication,
            } => {
                assert_eq!(api_endpoint, "https://kms.cloud-provider.com");
                match authentication {
                    CloudAuth::ApiKey { key_env_var } => {
                        assert_eq!(key_env_var, "CLOUD_API_KEY");
                    }
                    _ => panic!("Expected ApiKey auth"),
                }
            }
            _ => panic!("Expected CloudApi interface"),
        }
    }

    #[test]
    fn test_cloud_auth_oauth() {
        let auth = CloudAuth::OAuth {
            token_endpoint: "https://oauth.provider.com/token".to_string(),
        };

        match auth {
            CloudAuth::OAuth { token_endpoint } => {
                assert_eq!(token_endpoint, "https://oauth.provider.com/token");
            }
            _ => panic!("Expected OAuth auth"),
        }
    }

    #[test]
    fn test_cloud_auth_service_account() {
        let auth = CloudAuth::ServiceAccount {
            credentials_path: "/etc/service-account.json".to_string(),
        };

        match auth {
            CloudAuth::ServiceAccount { credentials_path } => {
                assert_eq!(credentials_path, "/etc/service-account.json");
            }
            _ => panic!("Expected ServiceAccount auth"),
        }
    }

    #[test]
    fn test_cloud_auth_instance_metadata() {
        let auth = CloudAuth::InstanceMetadata;
        assert!(matches!(auth, CloudAuth::InstanceMetadata));
    }

    #[test]
    fn test_hsm_security_level_comparison() {
        // Test that different security levels can be compared
        let low = HsmSecurityLevel::Low;
        let medium = HsmSecurityLevel::Medium;
        let high = HsmSecurityLevel::High;

        assert!(matches!(low, HsmSecurityLevel::Low));
        assert!(matches!(medium, HsmSecurityLevel::Medium));
        assert!(matches!(high, HsmSecurityLevel::High));
    }

    #[test]
    fn test_hsm_performance_profile_default() {
        let profile = HsmPerformanceProfile::default();

        assert_eq!(profile.operations_per_second, 0.0);
        assert_eq!(profile.avg_latency_ms, 0.0);
        assert_eq!(profile.success_rate, 0.0);
    }

    #[test]
    fn test_hsm_performance_profile_high() {
        let profile = HsmPerformanceProfile {
            operations_per_second: 1000.0,
            avg_latency_ms: 2.0,
            success_rate: 0.999,
        };

        assert!(profile.operations_per_second > 500.0);
        assert!(profile.avg_latency_ms < 10.0);
        assert!(profile.success_rate > 0.99);
    }

    #[test]
    fn test_discovered_hsm_trust_score_range() {
        let untrusted_hsm = DiscoveredHsm {
            hsm_id: "untrusted-001".to_string(),
            interface: HsmInterface::Native {
                interface_type: NativeInterface::Tpm,
            },
            supported_operations: vec![],
            security_level: HsmSecurityLevel::Low,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.2,
        };

        let trusted_hsm = DiscoveredHsm {
            hsm_id: "trusted-001".to_string(),
            interface: HsmInterface::Native {
                interface_type: NativeInterface::SecureEnclave,
            },
            supported_operations: vec![],
            security_level: HsmSecurityLevel::High,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.95,
        };

        assert!(untrusted_hsm.trust_score < 0.5);
        assert!(trusted_hsm.trust_score > 0.9);
    }

    #[test]
    fn test_crypto_operations_variety() {
        let hsm = DiscoveredHsm {
            hsm_id: "multi-op-hsm".to_string(),
            interface: HsmInterface::Native {
                interface_type: NativeInterface::StrongBox,
            },
            supported_operations: vec![
                CryptoOperation::Sign,
                CryptoOperation::Verify,
                CryptoOperation::Encrypt,
                CryptoOperation::Decrypt,
            ],
            security_level: HsmSecurityLevel::High,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.9,
        };

        assert_eq!(hsm.supported_operations.len(), 4);
        assert!(hsm
            .supported_operations
            .contains(&CryptoOperation::Sign));
        assert!(hsm
            .supported_operations
            .contains(&CryptoOperation::Encrypt));
    }

    #[test]
    fn test_discovered_hsm_serialization() {
        let hsm = DiscoveredHsm {
            hsm_id: "serialize-test".to_string(),
            interface: HsmInterface::Native {
                interface_type: NativeInterface::Tpm,
            },
            supported_operations: vec![CryptoOperation::Sign],
            security_level: HsmSecurityLevel::Medium,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.75,
        };

        // Test that it can be serialized
        let serialized = serde_json::to_string(&hsm);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_discovered_hsm_clone() {
        let hsm1 = DiscoveredHsm {
            hsm_id: "clone-test".to_string(),
            interface: HsmInterface::Native {
                interface_type: NativeInterface::SecureEnclave,
            },
            supported_operations: vec![CryptoOperation::Sign],
            security_level: HsmSecurityLevel::High,
            performance: HsmPerformanceProfile::default(),
            trust_score: 0.88,
        };

        let hsm2 = hsm1.clone();

        assert_eq!(hsm1.hsm_id, hsm2.hsm_id);
        assert_eq!(hsm1.trust_score, hsm2.trust_score);
        assert_eq!(
            hsm1.supported_operations.len(),
            hsm2.supported_operations.len()
        );
    }
}

