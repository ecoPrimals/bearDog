// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::canonical::capabilities::CapabilityType;
use crate::canonical::hsm_unified::providers::HsmSecurityCapability;
use serde_json::json;

fn json_roundtrip<T>(v: &T) -> serde_json::Value
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let val = serde_json::to_value(v).expect("serialize to JSON value");
    let back: T = serde_json::from_value(val.clone()).expect("deserialize from JSON value");
    let val2 = serde_json::to_value(&back).expect("re-serialize after roundtrip");
    assert_eq!(
        val, val2,
        "serde roundtrip must preserve JSON representation"
    );
    val2
}

#[test]
#[allow(deprecated)]
fn legacy_hsm_provider_type_default_display_serde() {
    let d = LegacyHsmProviderType::default();
    assert_eq!(format!("{d}"), "Software");
    let custom = LegacyHsmProviderType::Custom {
        name: "x".to_string(),
    };
    assert!(format!("{custom}").contains('x'));
    json_roundtrip(&LegacyHsmProviderType::Network);
}

#[test]
fn connection_config_default_clone_debug() {
    let c = ConnectionConfig::default();
    let _ = format!("{c:?}");
    let d = c.clone();
    assert_eq!(d.max_retries, c.max_retries, "clone preserves retries");
}

#[test]
fn security_config_default_serde_roundtrip() {
    let s = SecurityConfig::default();
    json_roundtrip(&s);
}

#[test]
fn auth_method_variants_json_roundtrip() {
    json_roundtrip(&AuthMethod::None);
    json_roundtrip(&AuthMethod::Password {
        password: "p".to_string(),
    });
    json_roundtrip(&AuthMethod::Token {
        token_id: 9,
        pin: Some("1234".to_string()),
    });
    json_roundtrip(&AuthMethod::Biometric {
        method: "face".to_string(),
    });
    let m = AuthMethod::Certificate {
        cert_path: "/c.pem".to_string(),
        key_path: "/k.pem".to_string(),
    };
    json_roundtrip(&m);
}

#[test]
fn hsm_provider_type_variants_roundtrip() {
    json_roundtrip(&HsmProviderType::Software);
    json_roundtrip(&HsmProviderType::Hardware {
        capabilities: vec![HsmSecurityCapability::HardwareCrypto],
    });
    json_roundtrip(&HsmProviderType::Universal {
        provider_id: "pid".to_string(),
        capability_type: CapabilityType::HardwareSecurityModule,
        capabilities: vec![HsmSecurityCapability::CloudBased],
    });
}

#[test]
fn hsm_config_default_serde_roundtrip() {
    json_roundtrip(&HsmConfig::default());
}

#[test]
fn software_hardware_network_cloud_defaults_descriptions() {
    let sw = SoftwareHsmConfig::default();
    assert!(!sw.storage_path.is_empty(), "software default storage path");
    let hw = HardwareHsmConfig::default();
    assert!(
        HsmProviderConfig::Hardware(hw.clone())
            .description()
            .contains("/dev/hsm"),
        "hardware description mentions device"
    );
    let net = NetworkHsmConfig::default();
    let desc = HsmProviderConfig::Network(net.clone()).description();
    assert!(desc.contains(':'), "network description includes port");
    let cloud = CloudHsmConfig::default();
    assert!(
        HsmProviderConfig::Cloud(cloud.clone())
            .description()
            .contains("universal"),
        "cloud description mentions provider"
    );
    json_roundtrip(&sw);
    json_roundtrip(&hw);
    json_roundtrip(&net);
    json_roundtrip(&cloud);
}

#[test]
fn load_balancing_and_server_config_roundtrip() {
    let lb = LoadBalancingConfig {
        strategy: LoadBalancingStrategy::LeastConnections,
        servers: vec![ServerConfig {
            address: "10.0.0.1".to_string(),
            port: 443,
            weight: 2,
            enabled: true,
        }],
        health_check_interval: 10,
    };
    json_roundtrip(&lb);
}

#[test]
fn universal_hsm_provider_roundtrip() {
    let u = UniversalHsmProvider::Discovered {
        provider_id: "a".to_string(),
        capability_type: "hsm".to_string(),
        endpoint: "e".to_string(),
    };
    json_roundtrip(&u);
}

#[test]
fn cloud_credentials_roundtrip() {
    let mut c = CloudCredentials::default();
    c.expires_at = Some(chrono::Utc::now());
    json_roundtrip(&c);
}

#[test]
fn hsm_provider_config_default_base_and_type() {
    let d = HsmProviderConfig::default();
    assert!(matches!(d, HsmProviderConfig::Software(_)));
    assert_eq!(
        d.provider_type(),
        HsmProviderType::default(),
        "default provider type matches base"
    );
    d.validate().expect("default software config validates");
}

#[test]
fn validate_operation_timeout_zero_errors() {
    let mut cfg = HsmProviderConfig::Software(SoftwareHsmConfig::default());
    cfg.base_config_mut().operation_timeout = Duration::from_secs(0);
    assert!(
        cfg.validate().is_err(),
        "zero operation timeout must fail validation"
    );
}

#[test]
fn validate_connection_timeout_zero_errors() {
    let mut cfg = HsmProviderConfig::Software(SoftwareHsmConfig::default());
    cfg.base_config_mut().connection.timeout_ms = 0;
    assert!(
        cfg.validate().is_err(),
        "zero connection timeout must fail validation"
    );
}

#[test]
fn validate_software_empty_storage_errors() {
    let mut sw = SoftwareHsmConfig::default();
    sw.storage_path.clear();
    let cfg = HsmProviderConfig::Software(sw);
    assert!(
        cfg.validate().is_err(),
        "empty software storage path must fail"
    );
}

#[test]
fn validate_hardware_empty_device_errors() {
    let mut hw = HardwareHsmConfig::default();
    hw.device_path.clear();
    assert!(
        HsmProviderConfig::Hardware(hw).validate().is_err(),
        "empty hardware device path must fail"
    );
}

#[test]
fn validate_network_address_and_port_errors() {
    let mut n = NetworkHsmConfig::default();
    n.server_address.clear();
    assert!(
        HsmProviderConfig::Network(n.clone()).validate().is_err(),
        "empty server address must fail"
    );
    let mut n2 = NetworkHsmConfig::default();
    n2.port = 0;
    assert!(
        HsmProviderConfig::Network(n2).validate().is_err(),
        "zero port must fail"
    );
}

#[test]
fn validate_cloud_region_and_key_errors() {
    let mut c = CloudHsmConfig::default();
    c.region.clear();
    assert!(
        HsmProviderConfig::Cloud(c.clone()).validate().is_err(),
        "empty cloud region must fail"
    );
    let mut c2 = CloudHsmConfig::default();
    c2.credentials.access_key_id.clear();
    assert!(
        HsmProviderConfig::Cloud(c2).validate().is_err(),
        "empty access key must fail"
    );
}

#[test]
fn base_config_mut_updates_provider() {
    let mut cfg = HsmProviderConfig::Hardware(HardwareHsmConfig::default());
    let p = HsmProviderType::Network {
        capabilities: vec![],
    };
    cfg.base_config_mut().provider = p.clone();
    assert_eq!(cfg.provider_type(), p);
}

#[test]
fn hsm_config_builder_builds_valid_software() {
    let built = HsmConfigBuilder::software()
        .operation_timeout(Duration::from_secs(5))
        .cache_size(42)
        .auth_method(AuthMethod::None)
        .build()
        .expect("builder should produce valid software config");
    assert!(built.validate().is_ok());
    assert_eq!(built.base_config().cache_size, Some(42));
}

#[test]
fn hsm_config_builder_propagates_validation_failure() {
    let err = HsmConfigBuilder::software()
        .operation_timeout(Duration::from_secs(0))
        .build()
        .expect_err("zero timeout must fail build");
    assert!(!err.is_empty(), "error message must be non-empty");
}

#[test]
fn hsm_provider_config_serde_variants() {
    json_roundtrip(&HsmProviderConfig::Software(SoftwareHsmConfig::default()));
    json_roundtrip(&HsmProviderConfig::Hardware(HardwareHsmConfig::default()));
    json_roundtrip(&HsmProviderConfig::Network(NetworkHsmConfig::default()));
    json_roundtrip(&HsmProviderConfig::Cloud(CloudHsmConfig::default()));
}

#[test]
fn load_balancing_strategy_all_variants_roundtrip() {
    for s in [
        LoadBalancingStrategy::RoundRobin,
        LoadBalancingStrategy::Random,
        LoadBalancingStrategy::LeastConnections,
        LoadBalancingStrategy::WeightedRoundRobin,
    ] {
        let v = json!(s);
        let back: LoadBalancingStrategy = serde_json::from_value(v).expect("deserialize strategy");
        assert_eq!(back, s, "strategy roundtrip");
    }
}
