// SPDX-License-Identifier: AGPL-3.0-or-later
//! `providers_unified` performance configuration tests.

#![cfg(test)]

use crate::canonical::providers_unified::performance::{
    BufferConfig, CacheType, CachingConfig, CompressionAlgorithm, CompressionConfig,
    PerformanceConfig, PerformanceThresholds,
};
use crate::canonical::traits::cache::CacheStrategy;

use super::common::assert_serde_json_roundtrip;

#[test]
fn performance_config_and_subcomponents_roundtrip() {
    let p = PerformanceConfig::default();
    assert_serde_json_roundtrip(&p);
    let json = serde_json::to_string(&p).expect("serialize performance");
    let rt: PerformanceConfig = serde_json::from_str(&json).expect("deserialize performance");
    assert_eq!(p.max_concurrent_requests, rt.max_concurrent_requests);

    let ct = CacheType::Custom("redis-cluster".to_string());
    assert_serde_json_roundtrip(&ct);

    let alg = CompressionAlgorithm::Zstd;
    assert_serde_json_roundtrip(&alg);

    let comp = CompressionConfig::default();
    assert_serde_json_roundtrip(&comp);

    let buf = BufferConfig::default();
    assert_serde_json_roundtrip(&buf);

    let t = PerformanceThresholds::default();
    assert_serde_json_roundtrip(&t);
}

#[test]
fn performance_from_env_loads_without_panic() {
    let _ = PerformanceConfig::from_env();
    let _ = CachingConfig::from_env();
    let _ = CompressionConfig::from_env();
    let _ = BufferConfig::from_env();
    let _ =
        crate::canonical::providers_unified::performance::PerformanceMonitoringConfig::from_env();
    let _ = PerformanceThresholds::from_env();
    let _ = crate::canonical::providers_unified::performance::PerformanceAlertingConfig::from_env();
}

#[test]
fn caching_config_trait_validate_errors() {
    let mut c = CachingConfig::default();
    c.enabled = true;
    c.max_entries = 0;
    assert!(CacheStrategy::validate(&c).is_err());
}

#[test]
fn performance_caching_is_production_ready_by_default() {
    let c = CachingConfig::default();
    assert!(c.is_production_ready());
}

#[test]
fn performance_monitoring_thresholds_from_env_runs() {
    let _ = crate::canonical::providers_unified::performance::PerformanceThresholds::from_env();
}
