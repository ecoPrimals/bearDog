// SPDX-License-Identifier: AGPL-3.0-or-later

use super::{
    AIMonitoringConfig, AIRegistryConfig, AuthType, CachingConfig, DeploymentStrategy,
    EvictionPolicy, InferenceConfig, LoadBalancingStrategy, MetricType, ModelManagementConfig,
    NeuralNetworkConfig, NormalizationStrategy, OptimizerType, PreprocessingConfig, RegistryType,
    ResourceRequirements, ServingConfig, TrainingConfig, VersioningStrategy,
};
use std::time::Duration;

#[test]
fn ai_monitoring_config_helpers_reflect_enabled_set() {
    let mut cfg = AIMonitoringConfig::default();
    assert!(cfg.collects_training_metrics());
    cfg.enabled_metrics.clear();
    assert!(!cfg.collects_training_metrics());
    assert!(!cfg.collects_inference_metrics());
    assert!(!cfg.tracks_model_performance());
    assert!(!cfg.monitors_resource_usage());
}

#[test]
fn versioning_and_registry_type_variants() {
    let _ = VersioningStrategy::Hash;
    let _ = RegistryType::CloudStorage;
    let _ = DeploymentStrategy::Canary;
    let _ = AuthType::OAuth2;
    let _ = MetricType::PredictionConfidence;
    let _ = LoadBalancingStrategy::WeightedRoundRobin;
    let _ = EvictionPolicy::Lfu;
    let _ = NormalizationStrategy::Robust;
}

#[test]
fn optimizer_type_roundtrip_debug() {
    let o = OptimizerType::Sgd { momentum: 0.9 };
    assert!(format!("{o:?}").contains("Sgd"));
}

#[test]
fn model_management_and_preprocessing_defaults_are_serializable() {
    let mm: ModelManagementConfig = ModelManagementConfig::default();
    let json = serde_json::to_string(&mm).expect("serialize");
    let _: ModelManagementConfig = serde_json::from_str(&json).expect("deserialize");

    let p: PreprocessingConfig = PreprocessingConfig::default();
    let json = serde_json::to_string(&p).expect("serialize");
    let _: PreprocessingConfig = serde_json::from_str(&json).expect("deserialize");
}

#[test]
fn neural_network_config_default_serializes() {
    let n: NeuralNetworkConfig = NeuralNetworkConfig::default();
    let json = serde_json::to_string(&n).expect("serialize");
    assert!(json.contains("Feedforward") || json.contains("architecture"));
}

#[test]
fn training_config_overrides_are_explicit() {
    let t = TrainingConfig {
        batch_size: 64,
        epochs: 42,
        learning_rate: 0.5,
        ..TrainingConfig::default()
    };
    assert_eq!(t.batch_size, 64);
    assert_eq!(t.epochs, 42);
    assert!((t.learning_rate - 0.5).abs() < f64::EPSILON);
}

#[test]
fn inference_and_serving_config_overrides_are_explicit() {
    let s = ServingConfig {
        max_concurrent_requests: 50,
        request_timeout: Duration::from_secs(120),
        load_balancing: LoadBalancingStrategy::RoundRobin,
    };
    let i = InferenceConfig {
        batch_size: 8,
        max_inference_time_ms: 2500,
        serving_config: s,
        caching: None,
    };
    assert_eq!(i.batch_size, 8);
    assert_eq!(i.max_inference_time_ms, 2500);
    assert_eq!(i.serving_config.max_concurrent_requests, 50);
    assert_eq!(i.serving_config.request_timeout.as_secs(), 120);
}

#[test]
fn ai_registry_config_custom_endpoint() {
    let r = AIRegistryConfig {
        registry_type: RegistryType::Local,
        endpoint: "http://registry.test:9000".to_string(),
        auth: None,
    };
    assert!(r.endpoint.contains("registry.test"));
}

#[test]
fn resource_requirements_custom_values() {
    let r = ResourceRequirements {
        cpu: 4.5,
        memory: 8192,
        gpu: Some(2),
        storage: 100,
    };
    assert!((r.cpu - 4.5).abs() < f64::EPSILON);
    assert_eq!(r.memory, 8192);
    assert_eq!(r.gpu, Some(2));
    assert_eq!(r.storage, 100);
}

#[test]
fn caching_config_struct_fields() {
    let c = CachingConfig {
        max_cache_size: 100,
        ttl: std::time::Duration::from_secs(60),
        eviction_policy: EvictionPolicy::Lru,
    };
    assert_eq!(c.max_cache_size, 100);
}
