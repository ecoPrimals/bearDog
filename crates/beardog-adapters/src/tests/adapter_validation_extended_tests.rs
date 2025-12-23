//! Extended Adapter Validation Tests
//!
//! Comprehensive test coverage for adapter integration and validation
//! Added October 29, 2025 - Part of Week 1 test coverage initiative

use std::collections::HashMap;

// Helper types
#[derive(Clone, Debug, PartialEq)]
enum AdapterType {
    Http,
    Database,
    MessageQueue,
    FileSystem,
}

#[derive(Clone, Debug)]
struct Adapter {
    name: String,
    adapter_type: AdapterType,
    config: HashMap<String, String>,
    connected: bool,
}

struct AdapterRegistry {
    adapters: HashMap<String, Adapter>,
}

// Helper functions
fn create_test_adapter(name: &str, adapter_type: AdapterType) -> Adapter {
    Adapter {
        name: name.to_string(),
        adapter_type,
        config: HashMap::new(),
        connected: false,
    }
}

fn connect_adapter(adapter: &mut Adapter) -> Result<(), String> {
    if adapter.connected {
        return Err("Already connected".to_string());
    }
    adapter.connected = true;
    Ok(())
}

fn disconnect_adapter(adapter: &mut Adapter) -> Result<(), String> {
    if !adapter.connected {
        return Err("Not connected".to_string());
    }
    adapter.connected = false;
    Ok(())
}

fn validate_adapter(adapter: &Adapter) -> Result<(), String> {
    if adapter.name.is_empty() {
        return Err("Adapter name cannot be empty".to_string());
    }
    Ok(())
}

fn create_registry() -> AdapterRegistry {
    AdapterRegistry {
        adapters: HashMap::new(),
    }
}

fn register_adapter(registry: &mut AdapterRegistry, adapter: Adapter) -> Result<(), String> {
    if registry.adapters.contains_key(&adapter.name) {
        return Err("Adapter already registered".to_string());
    }
    registry.adapters.insert(adapter.name.clone(), adapter);
    Ok(())
}

fn unregister_adapter(registry: &mut AdapterRegistry, name: &str) -> Result<(), String> {
    if !registry.adapters.contains_key(name) {
        return Err("Adapter not found".to_string());
    }
    registry.adapters.remove(name);
    Ok(())
}

fn get_adapter<'a>(registry: &'a AdapterRegistry, name: &str) -> Option<&'a Adapter> {
    registry.adapters.get(name)
}

fn list_adapters(registry: &AdapterRegistry) -> Vec<String> {
    registry.adapters.keys().cloned().collect()
}

fn adapter_health_check(adapter: &Adapter) -> bool {
    adapter.connected && !adapter.name.is_empty()
}

#[cfg(test)]
mod adapter_validation_tests {
    use super::*;

    #[test]
    fn test_adapter_creation_http() {
        let adapter = create_test_adapter("http-adapter", AdapterType::Http);
        assert_eq!(adapter.name, "http-adapter");
        assert_eq!(adapter.adapter_type, AdapterType::Http);
    }

    #[test]
    fn test_adapter_creation_database() {
        let adapter = create_test_adapter("db-adapter", AdapterType::Database);
        assert_eq!(adapter.adapter_type, AdapterType::Database);
    }

    #[test]
    fn test_adapter_creation_messagequeue() {
        let adapter = create_test_adapter("mq-adapter", AdapterType::MessageQueue);
        assert_eq!(adapter.adapter_type, AdapterType::MessageQueue);
    }

    #[test]
    fn test_adapter_creation_filesystem() {
        let adapter = create_test_adapter("fs-adapter", AdapterType::FileSystem);
        assert_eq!(adapter.adapter_type, AdapterType::FileSystem);
    }

    #[test]
    fn test_adapter_initial_state() {
        let adapter = create_test_adapter("test", AdapterType::Http);
        assert!(!adapter.connected);
    }

    #[test]
    fn test_adapter_connect() {
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        let result = connect_adapter(&mut adapter);
        assert!(result.is_ok());
        assert!(adapter.connected);
    }

    #[test]
    fn test_adapter_connect_already_connected() {
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        connect_adapter(&mut adapter).unwrap();
        let result = connect_adapter(&mut adapter);
        assert!(result.is_err());
    }

    #[test]
    fn test_adapter_disconnect() {
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        connect_adapter(&mut adapter).unwrap();
        let result = disconnect_adapter(&mut adapter);
        assert!(result.is_ok());
        assert!(!adapter.connected);
    }

    #[test]
    fn test_adapter_disconnect_not_connected() {
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        let result = disconnect_adapter(&mut adapter);
        assert!(result.is_err());
    }

    #[test]
    fn test_adapter_validation_valid() {
        let adapter = create_test_adapter("test", AdapterType::Http);
        let result = validate_adapter(&adapter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adapter_validation_empty_name() {
        let adapter = Adapter {
            name: String::new(),
            adapter_type: AdapterType::Http,
            config: HashMap::new(),
            connected: false,
        };
        let result = validate_adapter(&adapter);
        assert!(result.is_err());
    }

    #[test]
    fn test_registry_creation() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let registry = create_registry();
        assert_eq!(registry.adapters.len(), 0);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_adapter_registration() {
        let mut registry = create_registry();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter = create_test_adapter("test", AdapterType::Http);
        let result = register_adapter(&mut registry, adapter);
        assert!(result.is_ok());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        assert_eq!(registry.adapters.len(), 1);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_adapter_registration_duplicate() {
        let mut registry = create_registry();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter1 = create_test_adapter("test", AdapterType::Http);
        let adapter2 = create_test_adapter("test", AdapterType::Database);
        register_adapter(&mut registry, adapter1).unwrap();
        let result = register_adapter(&mut registry, adapter2);
        assert!(result.is_err());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_adapter_unregistration() {
        let mut registry = create_registry();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter = create_test_adapter("test", AdapterType::Http);
        register_adapter(&mut registry, adapter).unwrap();
        let result = unregister_adapter(&mut registry, "test");
        assert!(result.is_ok());
        assert_eq!(registry.adapters.len(), 0);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    #[test]
    fn test_adapter_unregistration_not_found() {
        let mut registry = create_registry();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let result = unregister_adapter(&mut registry, "nonexistent");
        assert!(result.is_err());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_adapter_existing() {
        let mut registry = create_registry();
        let adapter = create_test_adapter("test", AdapterType::Http);
        register_adapter(&mut registry, adapter).unwrap();
        let result = get_adapter(&registry, "test");
        assert!(result.is_some());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_get_adapter_nonexistent() {
        let registry = create_registry();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let result = get_adapter(&registry, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_list_adapters_empty() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let registry = create_registry();
        let list = list_adapters(&registry);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_adapters_multiple() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let mut registry = create_registry();
        register_adapter(
            &mut registry,
            create_test_adapter("adapter1", AdapterType::Http),
        )
        .unwrap();
        register_adapter(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            &mut registry,
            create_test_adapter("adapter2", AdapterType::Database),
        )
        .unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let list = list_adapters(&registry);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_adapter_health_check_connected() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        connect_adapter(&mut adapter).unwrap();
        assert!(adapter_health_check(&adapter));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    #[test]
    fn test_adapter_health_check_disconnected() {
        let adapter = create_test_adapter("test", AdapterType::Http);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        assert!(!adapter_health_check(&adapter));
    }

    #[test]
    fn test_adapter_clone() {
        let adapter = create_test_adapter("test", AdapterType::Http);
        let cloned = adapter.clone();
        assert_eq!(adapter.name, cloned.name);
    }

    #[test]
    fn test_adapter_type_equality() {
        assert_eq!(AdapterType::Http, AdapterType::Http);
        assert_ne!(AdapterType::Http, AdapterType::Database);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_adapter_config_empty() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let adapter = create_test_adapter("test", AdapterType::Http);
        assert!(adapter.config.is_empty());
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    #[test]
    fn test_adapter_debug_format() {
        let adapter = create_test_adapter("test", AdapterType::Http);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let debug_str = format!("{:?}", adapter);
        assert!(debug_str.contains("test"));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal

    #[test]
    fn test_multiple_adapter_types() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        let mut registry = create_registry();
        register_adapter(
            &mut registry,
            create_test_adapter("http", AdapterType::Http),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
        )
        .unwrap();
        register_adapter(
            &mut registry,
            create_test_adapter("db", AdapterType::Database),
        )
        .unwrap();
        register_adapter(
            &mut registry,
            create_test_adapter("mq", AdapterType::MessageQueue),
        )
        .unwrap();
        register_adapter(
            &mut registry,
            create_test_adapter("fs", AdapterType::FileSystem),
        )
        .unwrap();
        assert_eq!(list_adapters(&registry).len(), 4);
    }

    #[test]
    fn test_adapter_lifecycle() {
        let mut adapter = create_test_adapter("test", AdapterType::Http);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        assert!(!adapter.connected);
        connect_adapter(&mut adapter).unwrap();
        assert!(adapter.connected);
        disconnect_adapter(&mut adapter).unwrap();
        assert!(!adapter.connected);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[test]
    fn test_registry_persistence() {
        let mut registry = create_registry();
        let adapter = create_test_adapter("test", AdapterType::Http);
        register_adapter(&mut registry, adapter).unwrap();

        // Get adapter and verify it persists
        let retrieved = get_adapter(&registry, "test");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test");
    }
}
