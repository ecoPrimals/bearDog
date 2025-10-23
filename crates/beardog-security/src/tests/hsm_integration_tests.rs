//! HSM Integration Tests
//!
//! Comprehensive testing of Hardware Security Module (HSM) integration including:
//! - HSM provider initialization
//! - HSM configuration validation
//! - Provider type handling
//! - Connection and security configuration
//! - Authentication methods
//! - Custom parameters

use beardog_types::canonical::hsm::{
    AuthMethod, ConnectionConfig, HsmConfig, HsmProviderType, SecurityConfig,
};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// HSM Configuration Tests
// ============================================================================

#[test]
fn test_hsm_config_creation() {
    let config = HsmConfig {
        provider: HsmProviderType::Hardware,
        connection: ConnectionConfig::default(),
        security: SecurityConfig::default(),
        auth_method: AuthMethod::None,
        operation_timeout: Duration::from_secs(30),
        cache_size: Some(100),
        custom_params: HashMap::new(),
    };

    assert_eq!(config.provider, HsmProviderType::Hardware);
    assert_eq!(config.operation_timeout, Duration::from_secs(30));
    assert_eq!(config.cache_size, Some(100));
}

#[test]
fn test_hsm_config_default() {
    let config = HsmConfig::default();

    assert_eq!(config.provider, HsmProviderType::Software);
    assert_eq!(config.operation_timeout, Duration::from_secs(30));
    assert!(
        config.security.strict_mode,
        "Strict mode should be enabled by default"
    );
    assert!(
        config.security.audit_logging,
        "Audit logging should be enabled by default"
    );
}

#[test]
fn test_hsm_provider_types() {
    let providers = vec![
        HsmProviderType::Software,
        HsmProviderType::Hardware,
        HsmProviderType::Network,
        HsmProviderType::Cloud,
        HsmProviderType::Custom {
            name: "CustomHSM".to_string(),
        },
    ];

    for provider in providers {
        let config = HsmConfig {
            provider: provider.clone(),
            connection: ConnectionConfig::default(),
            security: SecurityConfig::default(),
            auth_method: AuthMethod::None,
            operation_timeout: Duration::from_secs(30),
            cache_size: Some(100),
            custom_params: HashMap::new(),
        };

        assert_eq!(config.provider, provider, "Provider type should match");
    }
}

#[test]
fn test_hsm_config_with_custom_params() {
    let mut custom_params = HashMap::new();
    custom_params.insert(
        "library_path".to_string(),
        "/usr/lib/softhsm/libsofthsm2.so".to_string(),
    );
    custom_params.insert("slot_id".to_string(), "0".to_string());
    custom_params.insert("pin".to_string(), "1234".to_string());

    let config = HsmConfig {
        provider: HsmProviderType::Software,
        connection: ConnectionConfig::default(),
        security: SecurityConfig::default(),
        auth_method: AuthMethod::Password {
            password: "secure_password".to_string(),
        },
        operation_timeout: Duration::from_secs(60),
        cache_size: Some(200),
        custom_params: custom_params.clone(),
    };

    assert_eq!(config.custom_params.len(), 3, "Should have 3 custom params");
    assert!(config.custom_params.contains_key("library_path"));
    assert_eq!(config.operation_timeout, Duration::from_secs(60));
}

// ============================================================================
// Connection Configuration Tests
// ============================================================================

#[test]
fn test_connection_config_default() {
    let conn = ConnectionConfig::default();

    assert_eq!(conn.timeout_ms, 30_000);
    assert_eq!(conn.max_retries, 3);
    assert_eq!(conn.retry_delay_ms, 1000);
    assert_eq!(conn.keep_alive_seconds, Some(300));
}

#[test]
fn test_connection_config_custom() {
    let conn = ConnectionConfig {
        timeout_ms: 60_000,
        max_retries: 5,
        retry_delay_ms: 2000,
        keep_alive_seconds: Some(600),
    };

    assert_eq!(conn.timeout_ms, 60_000);
    assert_eq!(conn.max_retries, 5);
    assert_eq!(conn.retry_delay_ms, 2000);
    assert_eq!(conn.keep_alive_seconds, Some(600));
}

#[test]
fn test_connection_config_no_keepalive() {
    let conn = ConnectionConfig {
        timeout_ms: 30_000,
        max_retries: 3,
        retry_delay_ms: 1000,
        keep_alive_seconds: None,
    };

    assert!(
        conn.keep_alive_seconds.is_none(),
        "Keep-alive should be disabled"
    );
}

// ============================================================================
// Security Configuration Tests
// ============================================================================

#[test]
fn test_security_config_default() {
    let sec = SecurityConfig::default();

    assert!(sec.strict_mode, "Strict mode should be enabled");
    assert_eq!(sec.key_validation, "strict");
    assert_eq!(sec.session_timeout, Duration::from_secs(3600));
    assert!(sec.audit_logging, "Audit logging should be enabled");
    assert!(
        !sec.access_policies.is_empty(),
        "Should have default access policies"
    );
}

#[test]
fn test_security_config_custom() {
    let sec = SecurityConfig {
        strict_mode: false,
        key_validation: "relaxed".to_string(),
        session_timeout: Duration::from_secs(7200),
        audit_logging: false,
        access_policies: vec!["policy1".to_string(), "policy2".to_string()],
    };

    assert!(!sec.strict_mode, "Strict mode should be disabled");
    assert_eq!(sec.key_validation, "relaxed");
    assert_eq!(sec.session_timeout, Duration::from_secs(7200));
    assert!(!sec.audit_logging, "Audit logging should be disabled");
    assert_eq!(sec.access_policies.len(), 2);
}

#[test]
fn test_security_config_multiple_policies() {
    let policies = vec![
        "authenticated_access".to_string(),
        "role_based_control".to_string(),
        "time_based_access".to_string(),
        "location_based_access".to_string(),
    ];

    let sec = SecurityConfig {
        strict_mode: true,
        key_validation: "strict".to_string(),
        session_timeout: Duration::from_secs(3600),
        audit_logging: true,
        access_policies: policies.clone(),
    };

    assert_eq!(sec.access_policies.len(), 4);
    assert!(sec
        .access_policies
        .contains(&"role_based_control".to_string()));
}

// ============================================================================
// Authentication Method Tests
// ============================================================================

#[test]
fn test_auth_method_none() {
    let auth = AuthMethod::None;

    match auth {
        AuthMethod::None => {
            // Successfully matched None variant
        }
        _ => panic!("Expected None variant"),
    }
}

#[test]
fn test_auth_method_password() {
    let auth = AuthMethod::Password {
        password: "secure_password_123".to_string(),
    };

    match auth {
        AuthMethod::Password { password } => {
            assert_eq!(password, "secure_password_123");
        }
        _ => panic!("Expected Password variant"),
    }
}

#[test]
fn test_auth_method_certificate() {
    let auth = AuthMethod::Certificate {
        cert_path: "/path/to/cert.pem".to_string(),
        key_path: "/path/to/key.pem".to_string(),
    };

    match auth {
        AuthMethod::Certificate {
            cert_path,
            key_path,
        } => {
            assert_eq!(cert_path, "/path/to/cert.pem");
            assert_eq!(key_path, "/path/to/key.pem");
        }
        _ => panic!("Expected Certificate variant"),
    }
}

#[test]
fn test_auth_method_token() {
    let auth = AuthMethod::Token {
        token_id: 12345,
        pin: Some("1234".to_string()),
    };

    match auth {
        AuthMethod::Token { token_id, pin } => {
            assert_eq!(token_id, 12345);
            assert_eq!(pin, Some("1234".to_string()));
        }
        _ => panic!("Expected Token variant"),
    }
}

#[test]
fn test_auth_method_token_no_pin() {
    let auth = AuthMethod::Token {
        token_id: 67890,
        pin: None,
    };

    match auth {
        AuthMethod::Token { token_id, pin } => {
            assert_eq!(token_id, 67890);
            assert!(pin.is_none());
        }
        _ => panic!("Expected Token variant"),
    }
}

#[test]
fn test_auth_method_biometric() {
    let auth = AuthMethod::Biometric {
        method: "fingerprint".to_string(),
    };

    match auth {
        AuthMethod::Biometric { method } => {
            assert_eq!(method, "fingerprint");
        }
        _ => panic!("Expected Biometric variant"),
    }
}

// ============================================================================
// Provider Type Tests
// ============================================================================

#[test]
fn test_provider_type_equality() {
    let software1 = HsmProviderType::Software;
    let software2 = HsmProviderType::Software;
    let hardware = HsmProviderType::Hardware;

    assert_eq!(software1, software2, "Same provider types should be equal");
    assert_ne!(
        software1, hardware,
        "Different provider types should not be equal"
    );
}

#[test]
fn test_provider_type_custom() {
    let custom1 = HsmProviderType::Custom {
        name: "MyHSM".to_string(),
    };
    let custom2 = HsmProviderType::Custom {
        name: "MyHSM".to_string(),
    };
    let custom3 = HsmProviderType::Custom {
        name: "OtherHSM".to_string(),
    };

    assert_eq!(
        custom1, custom2,
        "Custom types with same name should be equal"
    );
    assert_ne!(
        custom1, custom3,
        "Custom types with different names should not be equal"
    );
}

#[test]
fn test_provider_type_display() {
    let providers = vec![
        (HsmProviderType::Software, "Software"),
        (HsmProviderType::Hardware, "Hardware"),
        (HsmProviderType::Network, "Network"),
        (HsmProviderType::Cloud, "Cloud"),
    ];

    for (provider, expected) in providers {
        assert_eq!(provider.to_string(), expected);
    }
}

#[test]
fn test_provider_type_custom_display() {
    let custom = HsmProviderType::Custom {
        name: "MyCustomHSM".to_string(),
    };
    let display = custom.to_string();

    assert!(
        display.contains("Custom"),
        "Display should contain 'Custom'"
    );
    assert!(
        display.contains("MyCustomHSM"),
        "Display should contain custom name"
    );
}

// ============================================================================
// HSM Config Integration Tests
// ============================================================================

#[test]
fn test_hsm_config_full_setup() {
    let mut custom_params = HashMap::new();
    custom_params.insert("param1".to_string(), "value1".to_string());

    let config = HsmConfig {
        provider: HsmProviderType::Hardware,
        connection: ConnectionConfig {
            timeout_ms: 45_000,
            max_retries: 4,
            retry_delay_ms: 1500,
            keep_alive_seconds: Some(450),
        },
        security: SecurityConfig {
            strict_mode: true,
            key_validation: "strict".to_string(),
            session_timeout: Duration::from_secs(1800),
            audit_logging: true,
            access_policies: vec!["policy1".to_string()],
        },
        auth_method: AuthMethod::Certificate {
            cert_path: "/certs/hsm.pem".to_string(),
            key_path: "/keys/hsm.key".to_string(),
        },
        operation_timeout: Duration::from_secs(45),
        cache_size: Some(150),
        custom_params,
    };

    assert_eq!(config.provider, HsmProviderType::Hardware);
    assert_eq!(config.connection.timeout_ms, 45_000);
    assert_eq!(config.operation_timeout, Duration::from_secs(45));
    assert_eq!(config.cache_size, Some(150));
    assert!(!config.custom_params.is_empty());
}

#[test]
fn test_hsm_config_cloning() {
    let original = HsmConfig::default();
    let cloned = original.clone();

    assert_eq!(cloned.provider, original.provider);
    assert_eq!(cloned.operation_timeout, original.operation_timeout);
    assert_eq!(cloned.cache_size, original.cache_size);
}

#[test]
fn test_hsm_config_debug_format() {
    let config = HsmConfig::default();
    let debug_str = format!("{:?}", config);

    assert!(!debug_str.is_empty(), "Debug format should produce output");
    assert!(
        debug_str.contains("HsmConfig"),
        "Debug should contain type name"
    );
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_zero_cache_size() {
    let config = HsmConfig {
        provider: HsmProviderType::Software,
        connection: ConnectionConfig::default(),
        security: SecurityConfig::default(),
        auth_method: AuthMethod::None,
        operation_timeout: Duration::from_secs(30),
        cache_size: None,
        custom_params: HashMap::new(),
    };

    assert!(config.cache_size.is_none(), "Cache should be disabled");
}

#[test]
fn test_large_timeout_values() {
    let config = HsmConfig {
        provider: HsmProviderType::Cloud,
        connection: ConnectionConfig {
            timeout_ms: 300_000, // 5 minutes
            max_retries: 10,
            retry_delay_ms: 5000,
            keep_alive_seconds: Some(3600), // 1 hour
        },
        security: SecurityConfig::default(),
        auth_method: AuthMethod::None,
        operation_timeout: Duration::from_secs(600), // 10 minutes
        cache_size: Some(1000),
        custom_params: HashMap::new(),
    };

    assert_eq!(config.connection.timeout_ms, 300_000);
    assert_eq!(config.operation_timeout, Duration::from_secs(600));
}

#[test]
fn test_minimal_retry_config() {
    let conn = ConnectionConfig {
        timeout_ms: 1000,
        max_retries: 1,
        retry_delay_ms: 100,
        keep_alive_seconds: None,
    };

    assert_eq!(conn.max_retries, 1);
    assert_eq!(conn.retry_delay_ms, 100);
}

#[test]
fn test_empty_custom_params() {
    let config = HsmConfig {
        provider: HsmProviderType::Software,
        connection: ConnectionConfig::default(),
        security: SecurityConfig::default(),
        auth_method: AuthMethod::None,
        operation_timeout: Duration::from_secs(30),
        cache_size: Some(100),
        custom_params: HashMap::new(),
    };

    assert!(
        config.custom_params.is_empty(),
        "Custom params should be empty"
    );
}

#[test]
fn test_many_access_policies() {
    let mut policies = Vec::new();
    for i in 0..100 {
        policies.push(format!("policy_{}", i));
    }

    let sec = SecurityConfig {
        strict_mode: true,
        key_validation: "strict".to_string(),
        session_timeout: Duration::from_secs(3600),
        audit_logging: true,
        access_policies: policies.clone(),
    };

    assert_eq!(sec.access_policies.len(), 100);
}
