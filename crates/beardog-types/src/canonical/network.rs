// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Network and Communication Types
///
/// **CANONICAL NETWORK TYPES** - Single source of truth for all network-related types
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL** Network Protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NetworkProtocol {
    Http,
    Https,
    Tcp,
    Udp,
    Grpc,
    WebSocket,
    Quic,
}

impl Default for NetworkProtocol {
    fn default() -> Self {
        Self::Https
    }
}

/// **CANONICAL** Connection Pool Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionPoolConfig {
    pub min_connections: u32,
    pub max_connections: u32,
    pub connection_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
    pub test_on_borrow: bool,
    pub test_on_return: bool,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            min_connections: 5,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(3600),
            test_on_borrow: true,
            test_on_return: false,
        }
    }
}

/// **CANONICAL** Health Check Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub interval: Duration,
    pub timeout: Duration,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
    pub expected_status_codes: Vec<u16>,
    pub headers: HashMap<String, String>,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: "/health".to_string(),
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            healthy_threshold: 2,
            unhealthy_threshold: 3,
            expected_status_codes: vec![200],
            headers: HashMap::new(),
        }
    }
}

/// **CANONICAL** Load Balancing Strategy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
    Random,
    LeastResponseTime,
}

impl Default for LoadBalancingStrategy {
    fn default() -> Self {
        Self::RoundRobin
    }
}

/// **CANONICAL** Circuit Breaker Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 10,
        }
    }
}

/// **CANONICAL** Failover Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub backoff_multiplier: f64,
    pub max_delay: Duration,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
        }
    }
}

/// **CANONICAL** Load Balancing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct LoadBalancingConfig {
    pub strategy: LoadBalancingStrategy,
    pub health_check: HealthCheckConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    pub failover: FailoverConfig,
}


/// **CANONICAL** Service Discovery Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceDiscoveryType {
    Static,
    Consul,
    Etcd,
    Kubernetes,
    Zookeeper,
}

impl Default for ServiceDiscoveryType {
    fn default() -> Self {
        Self::Static
    }
}

/// **CANONICAL** Service Discovery Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    pub enabled: bool,
    pub discovery_type: ServiceDiscoveryType,
    pub endpoints: Vec<String>,
    pub refresh_interval: Duration,
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            discovery_type: ServiceDiscoveryType::Static,
            endpoints: Vec::new(),
            refresh_interval: Duration::from_secs(30),
        }
    }
}
