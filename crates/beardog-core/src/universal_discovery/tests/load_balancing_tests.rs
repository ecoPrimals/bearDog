// Comprehensive tests for load balancing module

use crate::universal_discovery::load_balancing::*;

#[test]
fn test_load_balancing_config_default() {
    let config = LoadBalancingConfig::default();

    assert!(!config.enable_sticky_sessions);
    assert_eq!(config.session_timeout_secs, 1800);
    assert!((config.health_weight_factor - 0.7).abs() < f64::EPSILON);
    assert!(config.enable_adaptive);
}

#[test]
fn test_load_balancing_algorithm_variants() {
    let algorithms = [
        LoadBalancingAlgorithm::RoundRobin,
        LoadBalancingAlgorithm::LeastConnections,
        LoadBalancingAlgorithm::WeightedRoundRobin,
        LoadBalancingAlgorithm::Random,
        LoadBalancingAlgorithm::IpHash,
        LoadBalancingAlgorithm::LeastResponseTime,
        LoadBalancingAlgorithm::ResourceBased,
    ];

    assert_eq!(algorithms.len(), 7);
}

#[test]
fn test_load_balancing_config_creation() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::LeastConnections,
        enable_sticky_sessions: true,
        session_timeout_secs: 3600,
        health_weight_factor: 0.8,
        enable_adaptive: false,
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert!(config.enable_sticky_sessions);
    assert_eq!(config.session_timeout_secs, 3600);
    assert!(!config.enable_adaptive);
}

#[test]
fn test_circuit_breaker_config_default() {
    let config = CircuitBreakerConfig::default();

    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.recovery_timeout_secs, 60);
    assert_eq!(config.half_open_max_calls, 3);
}

#[test]
fn test_circuit_breaker_config_creation() {
    let config = CircuitBreakerConfig {
        failure_threshold: 10,
        recovery_timeout_secs: 120,
        half_open_max_calls: 5,
    };

    assert_eq!(config.failure_threshold, 10);
    assert_eq!(config.recovery_timeout_secs, 120);
}

#[test]
fn test_load_balancer_creation() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config);

    assert!(balancer.is_ok());
}

#[test]
fn test_load_balancer_with_round_robin() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::RoundRobin,
        enable_sticky_sessions: false,
        session_timeout_secs: 1800,
        health_weight_factor: 0.7,
        enable_adaptive: true,
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    let balancer = LoadBalancer::new(&config);
    assert!(balancer.is_ok());
}

#[test]
fn test_load_balancer_with_weighted_round_robin() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
        enable_sticky_sessions: false,
        session_timeout_secs: 1800,
        health_weight_factor: 0.5,
        enable_adaptive: true,
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    let balancer = LoadBalancer::new(&config);
    assert!(balancer.is_ok());
}

#[test]
fn test_load_balancer_with_sticky_sessions() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::IpHash,
        enable_sticky_sessions: true,
        session_timeout_secs: 7200,
        health_weight_factor: 0.9,
        enable_adaptive: false,
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    let balancer = LoadBalancer::new(&config);
    assert!(balancer.is_ok());
}

#[test]
fn test_load_balancing_config_serialization() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::LeastConnections,
        enable_sticky_sessions: true,
        session_timeout_secs: 2400,
        health_weight_factor: 0.75,
        enable_adaptive: true,
        circuit_breaker: CircuitBreakerConfig {
            failure_threshold: 8,
            recovery_timeout_secs: 90,
            half_open_max_calls: 4,
        },
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: LoadBalancingConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(
        config.enable_sticky_sessions,
        deserialized.enable_sticky_sessions
    );
    assert_eq!(
        config.session_timeout_secs,
        deserialized.session_timeout_secs
    );
}

#[test]
fn test_circuit_breaker_config_serialization() {
    let config = CircuitBreakerConfig {
        failure_threshold: 7,
        recovery_timeout_secs: 180,
        half_open_max_calls: 6,
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: CircuitBreakerConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.failure_threshold, deserialized.failure_threshold);
    assert_eq!(
        config.recovery_timeout_secs,
        deserialized.recovery_timeout_secs
    );
}

#[test]
fn test_load_balancing_algorithm_serialization() {
    let algorithm = LoadBalancingAlgorithm::LeastResponseTime;
    let serialized = serde_json::to_string(&algorithm).unwrap();
    let _deserialized: LoadBalancingAlgorithm = serde_json::from_str(&serialized).unwrap();
    // Both serialize/deserialize successfully
    assert!(serialized.contains("LeastResponseTime"));
}

#[test]
fn test_multiple_load_balancing_algorithms() {
    let configs = [
        LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            ..Default::default()
        },
        LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::Random,
            ..Default::default()
        },
        LoadBalancingConfig {
            algorithm: LoadBalancingAlgorithm::ResourceBased,
            ..Default::default()
        },
    ];

    assert_eq!(configs.len(), 3);
}

#[test]
fn test_adaptive_load_balancing() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::ResourceBased,
        enable_sticky_sessions: false,
        session_timeout_secs: 1800,
        health_weight_factor: 0.85,
        enable_adaptive: true,
        circuit_breaker: CircuitBreakerConfig::default(),
    };

    assert!(config.enable_adaptive);
    assert!((config.health_weight_factor - 0.85).abs() < f64::EPSILON);
}

#[test]
fn test_circuit_breaker_thresholds() {
    let low_threshold = CircuitBreakerConfig {
        failure_threshold: 3,
        recovery_timeout_secs: 30,
        half_open_max_calls: 2,
    };

    let high_threshold = CircuitBreakerConfig {
        failure_threshold: 15,
        recovery_timeout_secs: 300,
        half_open_max_calls: 10,
    };

    assert!(low_threshold.failure_threshold < high_threshold.failure_threshold);
    assert!(low_threshold.recovery_timeout_secs < high_threshold.recovery_timeout_secs);
}
