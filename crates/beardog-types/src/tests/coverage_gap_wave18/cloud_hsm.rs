// SPDX-License-Identifier: AGPL-3.0-or-later
//! Unified cloud HSM configuration tests.

#![cfg(test)]

use crate::canonical::capabilities::CapabilityType;
use crate::canonical::config::hsm::{HsmConfigValidation, UnifiedCloudHsmConfig};

use super::common::assert_serde_json_roundtrip;

#[test]
fn unified_cloud_hsm_default_serde_roundtrip() {
    let d = UnifiedCloudHsmConfig::default();
    assert_serde_json_roundtrip(&d);
    let json = serde_json::to_string(&d).expect("serialize cloud hsm");
    let rt: UnifiedCloudHsmConfig = serde_json::from_str(&json).expect("deserialize cloud hsm");
    assert_eq!(d.enabled, rt.enabled);
}

#[test]
fn unified_cloud_hsm_validate_rejects_enabled_without_capabilities() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: true,
        required_capabilities: None,
    };
    let err = cfg
        .validate()
        .expect_err("validation should fail when enabled without capabilities");
    let _ = format!("{err:?}");
}

#[test]
fn unified_cloud_hsm_validate_accepts_disabled_without_capabilities() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: false,
        required_capabilities: None,
    };
    cfg.validate().expect("disabled cloud HSM without caps");
}

#[test]
fn unified_cloud_hsm_validate_accepts_enabled_with_capabilities() {
    let cfg =
        UnifiedCloudHsmConfig::with_required_capabilities(vec![CapabilityType::KeyManagement]);
    cfg.validate().expect("enabled with caps");
}

#[test]
fn unified_cloud_hsm_get_required_capabilities_falls_back_when_none() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: false,
        required_capabilities: None,
    };
    let caps = cfg.get_required_capabilities();
    assert!(caps.contains(&CapabilityType::KeyManagement));
}

#[test]
fn unified_cloud_hsm_with_required_capabilities_sets_enabled() {
    let cfg = UnifiedCloudHsmConfig::with_required_capabilities(vec![
        CapabilityType::HardwareSecurityModule,
    ]);
    assert!(cfg.is_enabled());
    assert_eq!(cfg.get_required_capabilities().len(), 1);
}

#[test]
fn unified_cloud_hsm_hsm_config_validation_trait_is_compatible() {
    let cfg = UnifiedCloudHsmConfig::default();
    assert!(HsmConfigValidation::is_compatible_with(&cfg, 1));
}

#[test]
fn unified_cloud_hsm_clone_debug() {
    let a = UnifiedCloudHsmConfig::default();
    let b = a.clone();
    assert_eq!(format!("{a:?}"), format!("{b:?}"));
}

#[test]
fn unified_cloud_hsm_validate_error_displayed() {
    let cfg = UnifiedCloudHsmConfig {
        enabled: true,
        required_capabilities: None,
    };
    let e = cfg.validate().expect_err("expected validation error");
    let s = format!("{e}");
    assert!(!s.is_empty());
}
