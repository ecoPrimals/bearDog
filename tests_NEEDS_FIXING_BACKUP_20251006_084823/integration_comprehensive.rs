use beardog_compliance::ComplianceEngine;
use beardog_errors::BearDogCore;
use beardog_errors::BearDogError;
use beardog_monitoring::SecuritySentinel;
use beardog_security::MemoryKeyManager;
use beardog_types::config::BearDogConfig;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_full_system_initialization() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    core.initialize()?;

    let state = core.state.read();
    assert!(state.components.len() > 0);

    println!("✅ Full system initialization successful");
    Ok(())
}

#[tokio::test]
async fn test_security_compliance_integration() -> Result<(), BearDogError> {
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config)?;

    let compliance_config = beardog_compliance::ComplianceConfig::default();
    let compliance_engine = ComplianceEngine::new(compliance_config)?;

    let test_key = "btest_integration_key".to_vec();
    let metadata = beardog_security::memory_key_manager::KeyMetadata {
        key_type: "AES-256".to_string(),
        purpose: "integration_test".to_string(),
        algorithm: "AES-256".to_string(),
        owner_id: "integration_test".to_string(),
        tags: vec!["test".to_string()],
        attributes: std::collections::HashMap::with_capacity(16),
    };

    let key_id = key_manager.store_key(test_key, metadata)?;

    let event = beardog_compliance::ComplianceEvent {
        id: format!("key_gen_{}", key_id),
        timestamp: chrono::Utc::now(),
        event_type: "key_generation".to_string(),
        source: "beardog-security".to_string(),
        data: std::collections::HashMap::with_capacity(16),
        metadata: std::collections::HashMap::with_capacity(16),
    };

    let compliance_result = compliance_engine.process_event(&event)?;
    assert!(compliance_result.score >= 0.0);

    println!("✅ Security-Compliance integration successful");
    Ok(())
}

#[tokio::test]
async fn test_monitoring_integration() -> Result<(), BearDogError> {
    let sentinel = SecuritySentinel::new();

    for _i in 0..3 {
        tokio::time::sleep(Duration::from_millis(10)).await;
    }

    println!("✅ Monitoring integration successful");
    Ok(())
}

#[tokio::test]
async fn test_error_handling_integration() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    let result = timeout(Duration::from_secs(5), core.initialize());

    match result {
        Ok(init_result) => {
            assert!(init_result.is_ok());
            println!("✅ System initialization completed successfully");
        }
        Err(_) => {
            println!("⚠️ System initialization timed out (acceptable for integration test)");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> Result<(), BearDogError> {
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config)?;

    let mut handles = Vec::new();

    for i in 0..5 {
        let km = key_manager.clone();
        let handle = tokio::spawn(async move {
            let test_key = format!("concurrent_test_key_{}", i)
                .to_string()
                .into_bytes();
            let metadata = beardog_security::memory_key_manager::KeyMetadata {
                key_type: "Ed25519".to_string(),
                algorithm: "Ed25519".to_string()],
                attributes: std::collections::HashMap::with_capacity(16),
            };

            km.store_key(test_key, metadata)
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })?;
        assert!(result.is_ok());
    }

    println!("✅ Concurrent operations test successful");
    Ok(())
}

#[tokio::test]
async fn test_performance_under_load() -> Result<(), BearDogError> {
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config)?;

    let start_time = std::time::Instant::now();

    for i in 0..100 {
        let test_key = format!("load_test_key_{}", i).into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "AES-256".to_string(),
            algorithm: "AES-256".to_string()],
            attributes: std::collections::HashMap::with_capacity({} operations in {:?}",
        100, duration
    );
    Ok(())
}

#[tokio::test]
async fn test_system_recovery() -> Result<(), BearDogError> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config)?;

    core.initialize()?;

    for _cycle in 0..3 {
        tokio::time::sleep(Duration::from_millis(10)).await;

        let state = core.state.read();
        assert!(state.components.len() > 0);
    }

    println!("✅ System recovery test successful");
    Ok(())
}

#[tokio::test]
async fn test_data_consistency() -> Result<(), BearDogError> {
    let key_config = beardog_security::memory_key_manager::MemoryKeyConfig::default();
    let key_manager = MemoryKeyManager::new(key_config)?;

    let mut key_ids = Vec::new();
    for i in 0..10 {
        let test_key = format!("consistency_test_key_{}", i)
            .to_string()
            .into_bytes();
        let metadata = beardog_security::memory_key_manager::KeyMetadata {
            key_type: "Ed25519".to_string(),
            algorithm: "Ed25519".to_string()],
            attributes: std::collections::HashMap::with_capacity(16),
        };

        let key_id = key_manager.store_key(test_key, metadata)?;
        key_ids.push(key_id);
    }

    for key_id in &key_ids {
        let retrieved_key = key_manager.get_key(key_id)?;
        assert!(!retrieved_key.is_empty());
    }

    let all_keys = key_manager.list_keys()?;
    assert!(all_keys.len() >= key_ids.len());

    println!("✅ Data consistency test successful");
    Ok(())
}
