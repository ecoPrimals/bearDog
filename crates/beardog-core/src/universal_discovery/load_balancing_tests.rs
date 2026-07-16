// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

fn create_test_services(count: usize) -> Vec<ServiceInfo> {
    (0..count)
        .map(|i| ServiceInfo {
            name: format!("service-{i}"),
            service_type: "test".to_string(),
            address: format!("192.168.1.{i}"),
            port: 8080,
            metadata: std::collections::BTreeMap::new(),
        })
        .collect()
}

fn create_test_services_with_metadata(count: usize) -> Vec<ServiceInfo> {
    (0..count)
        .map(|i| {
            let mut metadata = std::collections::BTreeMap::new();
            for j in 0..i {
                metadata.insert(format!("key-{j}"), format!("value-{j}"));
            }
            ServiceInfo {
                name: format!("service-{i}"),
                service_type: "test".to_string(),
                address: format!("10.0.0.{i}"),
                port: 9000,
                metadata,
            }
        })
        .collect()
}

#[test]
fn test_default_config() {
    let config = LoadBalancingConfig::default();
    assert!(matches!(
        config.algorithm,
        LoadBalancingAlgorithm::RoundRobin
    ));
    assert!(!config.enable_sticky_sessions);
    assert_eq!(config.session_timeout_secs, 1800);
    assert!((config.health_weight_factor - 0.7).abs() < f64::EPSILON);
    assert!(config.enable_adaptive);
}

#[test]
fn test_circuit_breaker_default_config() {
    let config = CircuitBreakerConfig::default();
    assert_eq!(config.failure_threshold, 5);
    assert_eq!(config.recovery_timeout_secs, 60); // Actual default is 60
    assert_eq!(config.half_open_max_calls, 3);
}

#[tokio::test]
async fn test_load_balancer_creation() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");
    assert!(balancer.balance_services(vec![]).await.is_ok());
}

#[tokio::test]
async fn test_round_robin_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::RoundRobin,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services(3);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services should succeed");

    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn test_least_connections_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::LeastConnections,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services(3);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services should succeed");

    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn test_weighted_round_robin_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    balancer
        .update_service_weight("service-0", 1.0)
        .await
        .expect("update_service_weight service-0 for weighted round robin");
    balancer
        .update_service_weight("service-1", 2.0)
        .await
        .expect("update_service_weight service-1 for weighted round robin");
    balancer
        .update_service_weight("service-2", 3.0)
        .await
        .expect("update_service_weight service-2 for weighted round robin");

    let services = create_test_services(3);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services weighted round robin");

    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn test_random_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::Random,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services(5);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services should succeed");

    assert_eq!(result.len(), 5);
}

#[tokio::test]
async fn test_ip_hash_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::IpHash,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services(3);
    let result1 = balancer
        .balance_services(services.clone())
        .await
        .expect("balance_services first call");
    let result2 = balancer
        .balance_services(services)
        .await
        .expect("balance_services second call");

    assert_eq!(result1.len(), result2.len());
    for (s1, s2) in result1.iter().zip(result2.iter()) {
        assert_eq!(s1.name, s2.name);
    }
}

#[tokio::test]
async fn test_least_response_time_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::LeastResponseTime,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services(3);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services should succeed");

    assert_eq!(result.len(), 3);
}

#[tokio::test]
async fn test_resource_based_balance() {
    let config = LoadBalancingConfig {
        algorithm: LoadBalancingAlgorithm::ResourceBased,
        ..Default::default()
    };
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    let services = create_test_services_with_metadata(3);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services should succeed");

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name, "service-0");
}

#[tokio::test]
async fn test_connection_counting() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    balancer
        .increment_connections("service-1")
        .await
        .expect("increment_connections service-1");
    balancer
        .increment_connections("service-1")
        .await
        .expect("increment_connections service-1 again");
    balancer
        .increment_connections("service-2")
        .await
        .expect("increment_connections service-2");

    balancer
        .decrement_connections("service-1")
        .await
        .expect("decrement_connections service-1");

    balancer
        .decrement_connections("service-999")
        .await
        .expect("decrement_connections missing service");

    let state = balancer.state.read().await;
    assert_eq!(state.service_connections.get("service-1"), Some(&1));
    assert_eq!(state.service_connections.get("service-2"), Some(&1));
}

#[tokio::test]
async fn test_service_weight_update() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    balancer
        .update_service_weight("service-1", 2.5)
        .await
        .expect("update_service_weight service-1 in weight test");
    balancer
        .update_service_weight("service-2", 1.5)
        .await
        .expect("update_service_weight service-2 in weight test");

    let state = balancer.state.read().await;
    let w1 = state
        .service_weights
        .get("service-1")
        .expect("service-1 weight recorded");
    let w2 = state
        .service_weights
        .get("service-2")
        .expect("service-2 weight recorded");
    assert!((w1 - 2.5).abs() < f64::EPSILON);
    assert!((w2 - 1.5).abs() < f64::EPSILON);
}

#[tokio::test]
async fn test_empty_services() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");
    let result = balancer
        .balance_services(vec![])
        .await
        .expect("balance_services empty vec");
    assert!(result.is_empty());
}

#[tokio::test]
async fn test_single_service() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");
    let services = create_test_services(1);
    let result = balancer
        .balance_services(services)
        .await
        .expect("balance_services single service");

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name, "service-0");
}

#[test]
fn test_all_algorithms_variant() {
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
fn test_load_balancer_start_stop() {
    let config = LoadBalancingConfig::default();
    let balancer = LoadBalancer::new(&config).expect("LoadBalancer::new with test config");

    assert!(balancer.start().is_ok());
    assert!(balancer.stop().is_ok());
}
