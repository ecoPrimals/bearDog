// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for [`OptimizationConfig`] and [`PerformanceOptimizer`] (config holder).

use super::optimization::*;

#[test]
fn config_default_is_all_disabled() {
    let config = OptimizationConfig::default();
    assert!(!config.enable_auto_scaling);
    assert!(!config.enable_profiling);
    assert_eq!(config.optimization_interval_seconds, 0);
}

#[test]
fn config_custom_values() {
    let config = OptimizationConfig {
        enable_auto_scaling: true,
        enable_profiling: true,
        optimization_interval_seconds: 300,
    };
    assert!(config.enable_auto_scaling);
    assert!(config.enable_profiling);
    assert_eq!(config.optimization_interval_seconds, 300);
}

#[test]
fn config_roundtrip_serialization() {
    let config = OptimizationConfig {
        enable_auto_scaling: true,
        enable_profiling: false,
        optimization_interval_seconds: 600,
    };
    let json = serde_json::to_string(&config).expect("serialize");
    let back: OptimizationConfig = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.enable_auto_scaling, config.enable_auto_scaling);
    assert_eq!(back.enable_profiling, config.enable_profiling);
    assert_eq!(
        back.optimization_interval_seconds,
        config.optimization_interval_seconds
    );
}

#[test]
fn config_extreme_interval() {
    let config = OptimizationConfig {
        enable_auto_scaling: true,
        enable_profiling: true,
        optimization_interval_seconds: u64::MAX,
    };
    let optimizer = PerformanceOptimizer::new(&config);
    assert!(optimizer.is_ok());
}

#[test]
fn optimizer_wraps_config() {
    let config = OptimizationConfig {
        enable_auto_scaling: true,
        enable_profiling: false,
        optimization_interval_seconds: 300,
    };
    let optimizer = PerformanceOptimizer::new(&config).expect("create");
    assert!(optimizer.config().enable_auto_scaling);
    assert!(!optimizer.config().enable_profiling);
    assert_eq!(optimizer.config().optimization_interval_seconds, 300);
}

#[test]
fn optimizer_debug_format() {
    let config = OptimizationConfig::default();
    let optimizer = PerformanceOptimizer::new(&config).expect("create");
    let debug = format!("{optimizer:?}");
    assert!(debug.contains("PerformanceOptimizer"));
}

#[test]
fn multiple_optimizers_independent() {
    let o1 = PerformanceOptimizer::new(&OptimizationConfig {
        enable_auto_scaling: true,
        enable_profiling: false,
        optimization_interval_seconds: 300,
    })
    .expect("create");

    let o2 = PerformanceOptimizer::new(&OptimizationConfig {
        enable_auto_scaling: false,
        enable_profiling: true,
        optimization_interval_seconds: 600,
    })
    .expect("create");

    assert!(o1.config().enable_auto_scaling);
    assert!(!o2.config().enable_auto_scaling);
}
