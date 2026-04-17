// SPDX-License-Identifier: AGPL-3.0-or-later
//! Canonical HSM provider configuration and builder tests.

#![cfg(test)]

use std::time::Duration;

use crate::canonical::hsm::config::{
    AuthMethod, CloudCredentials, CloudHsmConfig, HsmConfigBuilder, HsmProviderConfig,
    LoadBalancingStrategy, NetworkHsmConfig, UniversalHsmProvider,
};

use super::common::assert_serde_json_roundtrip;

#[test]
fn hsm_provider_config_validate_and_description() {
    let mut sw = HsmProviderConfig::default();
    match &mut sw {
        HsmProviderConfig::Software(s) => s.storage_path = String::new(),
        _ => panic!("expected software default"),
    }
    sw.validate().expect_err("empty storage");

    let hw = HsmProviderConfig::Hardware(crate::canonical::hsm::config::HardwareHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        device_path: String::new(),
        hardware_settings: std::collections::HashMap::new(),
        min_firmware_version: None,
        hardware_attestation: false,
    });
    hw.validate().expect_err("empty device");

    let net = HsmProviderConfig::Network(NetworkHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        server_address: String::new(),
        port: 1,
        use_tls: false,
        tls_cert_path: None,
        load_balancing: None,
    });
    net.validate().expect_err("empty server");

    let net2 = HsmProviderConfig::Network(NetworkHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        server_address: "h".to_string(),
        port: 0,
        use_tls: false,
        tls_cert_path: None,
        load_balancing: None,
    });
    net2.validate().expect_err("zero port");

    let cloud = HsmProviderConfig::Cloud(CloudHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        provider: "p".to_string(),
        region: String::new(),
        credentials: CloudCredentials::default(),
        endpoints: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    });
    cloud.validate().expect_err("empty region");

    let cloud2 = HsmProviderConfig::Cloud(CloudHsmConfig {
        base: crate::canonical::hsm::config::HsmConfig::default(),
        provider: "p".to_string(),
        region: "r".to_string(),
        credentials: CloudCredentials::default(),
        endpoints: std::collections::HashMap::new(),
        options: std::collections::HashMap::new(),
    });
    cloud2.validate().expect_err("empty access key");

    let ok = HsmProviderConfig::default();
    ok.validate().expect("default software config");
    let _ = ok.description();
    let _ = ok.provider_type();
    let _ = ok.base_config();
}

#[test]
fn hsm_config_builder_and_auth_methods() {
    let built = HsmConfigBuilder::software()
        .operation_timeout(Duration::from_secs(2))
        .cache_size(50)
        .auth_method(AuthMethod::Password {
            password: "x".to_string(),
        })
        .build()
        .expect("build software");
    assert!(matches!(built, HsmProviderConfig::Software(_)));

    let u = UniversalHsmProvider::Discovered {
        provider_id: "a".to_string(),
        capability_type: "b".to_string(),
        endpoint: "c".to_string(),
    };
    assert_serde_json_roundtrip(&u);

    let lb = LoadBalancingStrategy::RoundRobin;
    assert_serde_json_roundtrip(&lb);
}

#[test]
#[expect(
    deprecated,
    reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
)]
fn legacy_hsm_provider_type_display_and_default() {
    use crate::canonical::hsm::config::LegacyHsmProviderType;
    let c = LegacyHsmProviderType::Custom {
        name: "x".to_string(),
    };
    assert_eq!(format!("{c}"), "Custom(x)");
    let d: LegacyHsmProviderType = LegacyHsmProviderType::default();
    let _ = format!("{d}");
}

#[test]
fn auth_method_variants_roundtrip() {
    assert_serde_json_roundtrip(&AuthMethod::Token {
        token_id: 9,
        pin: Some("p".to_string()),
    });
    assert_serde_json_roundtrip(&AuthMethod::Certificate {
        cert_path: "/c".to_string(),
        key_path: "/k".to_string(),
    });
}

#[test]
fn hsm_provider_config_base_config_mut_updates_timeout() {
    let mut p = HsmProviderConfig::default();
    p.base_config_mut().operation_timeout = Duration::from_secs(99);
    assert_eq!(p.base_config().operation_timeout, Duration::from_secs(99));
}

#[test]
fn load_balancing_and_server_config_serde() {
    use crate::canonical::hsm::config::{LoadBalancingConfig, ServerConfig};
    let lb = LoadBalancingConfig {
        strategy: LoadBalancingStrategy::LeastConnections,
        servers: vec![ServerConfig {
            address: "10.0.0.1".to_string(),
            port: 443,
            weight: 1,
            enabled: true,
        }],
        health_check_interval: 30,
    };
    assert_serde_json_roundtrip(&lb);
}
