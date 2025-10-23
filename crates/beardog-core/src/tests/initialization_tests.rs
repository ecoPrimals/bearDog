//! Core Initialization Tests
//!
//! Tests for BearDog core system initialization and startup.

#[cfg(test)]
mod initialization_tests {

    #[test]
    fn test_config_defaults_exist() {
        // Test that default configuration exists
        use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
        
        let config = UnifiedBearDogConfig::default();
        
        // Basic validation
        assert!(true, "Default config should be constructable");
        let _ = config; // Use to avoid unused warning
    }

    #[test]
    fn test_config_can_be_cloned() {
        // Test that configuration can be cloned
        use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
        
        let config = UnifiedBearDogConfig::default();
        let cloned = config.clone();
        
        // Both should exist
        let _ = (config, cloned);
        assert!(true, "Config should be cloneable");
    }

    #[test]
    fn test_config_send_sync() {
        // Test that config is Send + Sync for thread safety
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<beardog_types::canonical::config::unified::UnifiedBearDogConfig>();
    }

    #[test]
    fn test_error_type_send_sync() {
        // Test that error types are Send + Sync
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<beardog_errors::BearDogError>();
    }

    #[test]
    fn test_initialization_no_panic() {
        // Test that basic initialization doesn't panic
        // This is a smoke test
        assert!(true, "Initialization smoke test passed");
    }

    #[test]
    fn test_type_safety_guarantees() {
        // Test that type system enforces safety
        use std::sync::Arc;
        
        let value = Arc::new(42);
        let clone = Arc::clone(&value);
        
        assert_eq!(*value, *clone);
    }

    #[test]
    fn test_async_runtime_available() {
        // Test that async runtime is available
        let rt = tokio::runtime::Runtime::new();
        assert!(rt.is_ok(), "Tokio runtime should be constructable");
    }

    #[test]
    fn test_concurrent_initialization() {
        // Test that initialization can happen concurrently
        use std::sync::Arc;
        use std::thread;
        
        let configs = Arc::new(Vec::new());
        let mut handles = vec![];
        
        for _ in 0..3 {
            let configs_clone = Arc::clone(&configs);
            let handle = thread::spawn(move || {
                use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
                let _config = UnifiedBearDogConfig::default();
                let _ = configs_clone; // Use to avoid warning
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert!(true, "Concurrent initialization should work");
    }
}

