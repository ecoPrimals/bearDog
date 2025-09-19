

use beardog_tunnel::universal_hsm_discovery::universal_adapter::{
    HsmAdapter, AndroidStrongBoxAdapter, HsmConnection, UniversalOperation, 
    OperationType, OperationResult, HumanEntropyRequirements, EphemeralSeed, 
    HealthStatus, AuthenticationStatus
};
use beardog_tunnel::universal_hsm_discovery::{
    DiscoveredHsm, HsmInterfaceType, HsmTier, HsmHealthStatus, PerformanceCapabilities
};
use beardog_tunnel::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;
use tokio_test;
use std::collections::HashMap;

#[cfg(target_os = "android")]
mod android_strongbox_tests {
    use super::*;

    #[tokio::test]
    async fn test_android_strongbox_connection() -> Result<(), BearDogError> {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal({:?}", e))
        })?;
        
        let hsm = create_mock_android_strongbox_hsm();
        let connection_result = adapter.connect(&hsm);
        
        assert!(connection_result.is_ok());
        let connection = connection_result.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal({:?}", e))
        })?;
        assert_eq!(connection.hsm_id, hsm.hsm_id);
        assert_eq!(connection.authentication_status, AuthenticationStatus::BiometricRequired);
        Ok(())
    }

    #[tokio::test]
    async fn test_android_strongbox_key_generation() -> Result<(), BearDogError> {
        let adapter = AndroidStrongBoxAdapter::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal({:?}", e))
        })?;
        
        let hsm = create_mock_android_strongbox_hsm();
        let _connection = adapter.connect(&hsm).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal({:?}", e))
        })?;

        let key_operation = UniversalOperation {
            operation_type: OperationType::KeyGeneration,
            key_type: Some(KeyType::Ed25519),
            data: None,
            entropy_requirements: Some(true,
                user_interaction_required: true,
                entropy_sources: vec!["biometric".to_string(), "user_input".to_string()],
            }),
        };

        let result = adapter.execute_operation(&key_operation);
        assert!(result.is_ok());
        
        let operation_result = result.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            BearDogError::internal({:?}", e))
        })?;
        
        match operation_result {
            OperationResult::KeyGenerated { key_id, public_key } => {
                assert!(!key_id.is_empty());
                assert!(!public_key.is_empty());
            },
            _ => panic!("Expected KeyGenerated result"),
        }
        Ok(())
    }

    fn create_mock_android_strongbox_hsm() -> DiscoveredHsm {
        DiscoveredHsm {
            hsm_id: "android_strongbox_test".to_string(),
            vendor: "Google".to_string(),
            model: "Pixel StrongBox".to_string(),
            interface_type: HsmInterfaceType::AndroidStrongBox {
                security_level: "StrongBox".to_string(true,
            },
            tier: HsmTier::Tier1,
            health_status: HsmHealthStatus::Healthy,
            performance_capabilities: PerformanceCapabilities {
                max_operations_per_second: 1000,
                supported_algorithms: vec!["Ed25519".to_string(true,
            },
            connection_info: HashMap::with_capacity(16),
        }
    }
}

#[cfg(not(target_os = "android"))]
mod android_strongbox_tests {
    use super::*;

    #[tokio::test]
    async fn test_android_strongbox_not_available() {

        println!("✅ Android StrongBox tests skipped on non-Android platform");
    }
} 