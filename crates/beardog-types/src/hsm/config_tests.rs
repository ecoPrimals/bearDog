// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for HSM configuration types

#[cfg(test)]
mod tests {
    use crate::hsm::config::*;

    #[test]
    fn test_database_config_new() {
        let config = DatabaseConfig::new(
            "postgresql://localhost:5432/testdb".to_string(),
            "testdb".to_string(),
        );
        
        assert_eq!(config.connection_string, "postgresql://localhost:5432/testdb");
        assert_eq!(config.database_name, "testdb");
    }

    #[test]
    fn test_database_config_default() {
        let config = DatabaseConfig::default();
        
        assert!(!config.connection_string.is_empty());
        assert!(!config.database_name.is_empty());
        assert_eq!(config.database_name, "beardog");
    }

    #[test]
    fn test_database_config_validation() {
        let valid_config = DatabaseConfig::new(
            "postgresql://localhost:5432/db".to_string(),
            "db".to_string(),
        );
        
        assert!(valid_config.validate().is_ok());
    }

    #[test]
    fn test_database_config_empty_connection_string() {
        let config = DatabaseConfig::new(
            String::new(),
            "testdb".to_string(),
        );
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_database_config_empty_database_name() {
        let config = DatabaseConfig::new(
            "postgresql://localhost:5432/db".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            String::new(),
        );
        
        assert!(config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_keystore_config_new() {
        let config = KeyStoreConfig::new(
            "/var/lib/beardog/keys".to_string(),
            "secure123".to_string(),
        );
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(config.path, "/var/lib/beardog/keys");
        assert_eq!(config.encryption_key, "secure123");
    }

    #[test]
    fn test_keystore_config_default() {
        let config = KeyStoreConfig::default();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        assert!(!config.path.is_empty());
        assert!(!config.encryption_key.is_empty());
    }

    #[test]
    fn test_keystore_config_validation() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let valid_config = KeyStoreConfig::new(
            "/tmp/keys".to_string(),
            "strongkey".to_string(),
        );
        
        assert!(valid_config.validate().is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_keystore_config_empty_path() {
        let config = KeyStoreConfig::new(
            String::new(),
            "key123".to_string(),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        );
        
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_keystore_config_weak_encryption_key() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = KeyStoreConfig::new(
            "/tmp/keys".to_string(),
            "123".to_string(), // Too short
        );
        
        assert!(config.validate().is_err());
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_keystore_config_minimum_key_length() {
        let config = KeyStoreConfig::new(
            "/tmp/keys".to_string(),
            "12345678".to_string(), // Minimum 8 chars
        );
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_database_config_clone() {
        let config1 = DatabaseConfig::new(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            "postgresql://localhost:5432/db1".to_string(),
            "db1".to_string(),
        );
        let config2 = config1.clone();
        
        assert_eq!(config1.connection_string, config2.connection_string);
        assert_eq!(config1.database_name, config2.database_name);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_keystore_config_clone() {
        let config1 = KeyStoreConfig::new(
            "/path/to/keys".to_string(),
            "secretkey".to_string(),
        );
        let config2 = config1.clone();
        
        assert_eq!(config1.path, config2.path);
        assert_eq!(config1.encryption_key, config2.encryption_key);
    }
}

