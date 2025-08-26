

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub use crate::config::monitoring::CircuitBreakerConfig;
pub use crate::config::production::FailoverConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {

    pub enabled: Option<bool>,

    pub health_checking: Option<bool>,

    pub strategy: String,

    pub algorithm: Option<LoadBalancingAlgorithm>,

    pub lb_type: Option<String>,

    pub weight: Option<u32>,

    pub max_requests: Option<u32>,

    pub health_check_interval_secs: u64,

    pub health_check: Option<HealthCheckConfig>,

    pub max_retries: u32,

    pub failover: Option<FailoverConfig>,

    pub upstream_servers: Option<Vec<UpstreamServer>>,

    pub circuit_breaker: Option<CircuitBreakerConfig>,
}

pub enum LoadBalancingAlgorithm {

    RoundRobin,

    LeastConnections,

    WeightedRoundRobin,

    IpHash,

    Random,

    LeastResponseTime,

    ResourceBased,

pub struct UpstreamServer {

    pub address: String,

    pub port: u16,

    pub max_connections: Option<u32>,

    pub health_check_url: Option<String>,

    pub connection_timeout: Option<Duration>,

    pub metadata: Option<std::collections::HashMap<String, String>>,

pub struct HealthCheckConfig {

    pub path: String,

    pub interval: Duration,

    pub timeout: Duration,

    pub unhealthy_threshold: u32,

    pub healthy_threshold: u32,

    pub method: String,

    pub expected_status_codes: Vec<u16>,

    pub headers: Option<std::collections::HashMap<String, String>>,

pub struct SessionAffinityConfig {

    pub enabled: bool,

    pub method: SessionAffinityMethod,

    pub cookie_name: Option<String>,

    pub header_name: Option<String>,

pub enum SessionAffinityMethod {
    Cookie,
    Header,
    None,}

impl Default for LoadBalancingConfig {}

    fn default() -> Self {
        Self {
            enabled: Some(true),
            health_checking: Some(true),
            strategy: "round_robin".to_string(),
            algorithm: Some(LoadBalancingAlgorithm::RoundRobin),
            lb_type: Some("nginx".to_string()),
            weight: Some(1),
            max_requests: Some(1000),
            health_check_interval_secs: 30,
            health_check: Some(HealthCheckConfig::default()),
            max_retries: 3,
            failover: None,
            upstream_servers: Some(vec![]),
            circuit_breaker: None,
        }
    }
impl Default for HealthCheckConfig {
            path: "/health".to_string(),
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            unhealthy_threshold: 3,
            healthy_threshold: 2,
            method: "GET".to_string(),
            expected_status_codes: vec![200],
            headers: None,}

impl Default for UpstreamServer {
            address: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: Some(100),
            health_check_url: None,
            connection_timeout: Some(Duration::from_secs(5)),
            metadata: None,
