/*
 * BearDog HSM Unit Tests
 * 
 * Unit tests for real HSM integration components
 */

use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    HsmAdapter, Pkcs11Adapter, AndroidStrongBoxAdapter, BearDogNativeAdapter,
    HsmConnection, UniversalOperation, OperationType, AuthenticationStatus,
    HumanEntropyRequirements
};
use beardog_tunnel::universal_hsm_discovery::{DiscoveredHsm, HsmInterfaceType, HsmTier};
use beardog_errors::{BearDogError, BearDogResult};
use tokio_test;
use std::collections::HashMap;

#[tokio::test]
async fn test_pkcs11_adapter_creation() {
    let adapter = Pkcs11Adapter;
    // Test that adapter exists and has proper Debug trait
    println!("PKCS#11 Adapter: {:?}", adapter);
}

#[tokio::test]
async fn test_android_strongbox_adapter_creation() {
    let adapter_result = AndroidStrongBoxAdapter::new();
    assert!(adapter_result.is_ok());
    
    let adapter = adapter_result.unwrap();
    println!("Android StrongBox Adapter: {:?}", adapter);
}

#[tokio::test]
async fn test_beardog_native_adapter_creation() {
    let adapter_result = BearDogNativeAdapter::new();
    assert!(adapter_result.is_ok());
    
    let adapter = adapter_result.unwrap();
    println!("BearDog Native Adapter: {:?}", adapter);
}

#[tokio::test]
async fn test_operation_type_variants() {
    let operations = vec![
        OperationType::GenerateKey,
        OperationType::Sign,
        OperationType::Verify,
        OperationType::Encrypt,
        OperationType::Decrypt,
        OperationType::HumanEntropyGeneration,
    ];
    
    // Test that all variants exist and can be used in HashMap
    let mut op_map = HashMap::new();
    for (i, op) in operations.iter().enumerate() {
        op_map.insert(op.clone(), i);
    }
    
    assert_eq!(op_map.len(), 6);
    assert!(op_map.contains_key(&OperationType::HumanEntropyGeneration));
}

#[tokio::test]
async fn test_human_entropy_requirements() {
    let requirements = HumanEntropyRequirements {
        minimum_entropy_bits: 256,
        collection_timeout_seconds: 30,
    };
    
    assert_eq!(requirements.minimum_entropy_bits, 256);
    assert_eq!(requirements.collection_timeout_seconds, 30);
    
    // Test serialization works
    let json = serde_json::to_string(&requirements);
    assert!(json.is_ok());
}

#[tokio::test]
async fn test_authentication_status_enum() {
    let statuses = vec![
        AuthenticationStatus::Unauthenticated,
        AuthenticationStatus::BiometricRequired,
        AuthenticationStatus::Authenticated,
    ];
    
    for status in statuses {
        println!("Authentication Status: {:?}", status);
    }
}

#[tokio::test]
async fn test_hsm_interface_types() {
    let interfaces = vec![
        HsmInterfaceType::Pkcs11 {
            library_path: "/usr/lib/libpkcs11.so".to_string(),
        },
        HsmInterfaceType::AndroidStrongBox {
            security_level: "STRONGBOX".to_string(),
        },
        HsmInterfaceType::BearDogNative {
            instance_id: "native-001".to_string(),
        },
    ];
    
    for interface in interfaces {
        println!("HSM Interface: {:?}", interface);
    }
}

#[tokio::test]
async fn test_hsm_tiers() {
    let tiers = vec![
        HsmTier::Software,
        HsmTier::BasicHardware,
        HsmTier::CertifiedHardware,
        HsmTier::HighSecurity,
        HsmTier::HumanEntropyPremium,
    ];
    
    // Test ordering
    assert!(HsmTier::HumanEntropyPremium > HsmTier::HighSecurity);
    assert!(HsmTier::HighSecurity > HsmTier::CertifiedHardware);
    assert!(HsmTier::CertifiedHardware > HsmTier::BasicHardware);
    assert!(HsmTier::BasicHardware > HsmTier::Software);
}

#[tokio::test]  
async fn test_universal_operation() {
    let operation = UniversalOperation {
        operation_type: OperationType::GenerateKey,
        parameters: HashMap::from([
            ("key_type".to_string(), "rsa_2048".to_string()),
            ("key_id".to_string(), "test-key".to_string()),
        ]),
    };
    
    assert_eq!(operation.operation_type, OperationType::GenerateKey);
    assert_eq!(operation.parameters.len(), 2);
    assert_eq!(operation.parameters.get("key_type").unwrap(), "rsa_2048");
}

#[tokio::test]
async fn test_pkcs11_human_entropy_not_supported() {
    let adapter = Pkcs11Adapter;
    
    let supports_result = adapter.supports_human_entropy().await;
    assert!(supports_result.is_ok());
    assert!(!supports_result.unwrap());
}

#[tokio::test]
async fn test_beardog_native_human_entropy_supported() {
    let adapter = BearDogNativeAdapter::new().unwrap();
    
    let supports_result = adapter.supports_human_entropy().await;
    assert!(supports_result.is_ok());
    assert!(supports_result.unwrap());
} 