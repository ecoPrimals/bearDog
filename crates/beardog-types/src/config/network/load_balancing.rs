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


/// # Load Balancing Configuration
///
/// **LOAD BALANCING ALGORITHMS AND UPSTREAM MANAGEMENT**
/// Contains load balancing configuration structs including algorithms,
/// upstream server management, and failover strategies.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Re-export canonical configurations from production module
pub use crate::config::monitoring::CircuitBreakerConfig;
pub use crate::config::production::FailoverConfig;
/// **COMPREHENSIVE LOAD BALANCING CONFIGURATION**
/// Unifies all LoadBalancer/LoadBalancing config duplicates across the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    // === Core Enable/Disable ===
    /// Whether load balancing is enabled
    pub enabled: Option<bool>,
    /// Health checking enabled (load testing compatibility)
    pub health_checking: Option<bool>,
    // === Algorithm Configuration ===
    /// Load balancing strategy (string format for flexibility)
    pub strategy: String,
    /// Structured algorithm enum (preferred format)
    pub algorithm: Option<LoadBalancingAlgorithm>,
    /// Load balancer type (nginx, haproxy, envoy, etc.)
    pub lb_type: Option<String>,
    // === Weight and Capacity Management ===
    /// Service weight for weighted algorithms
    pub weight: Option<u32>,
    /// Maximum concurrent requests per service
    pub max_requests: Option<u32>,
    // === Health Check Configuration ===
    /// Health check interval in seconds
    pub health_check_interval_secs: u64,
    /// Health check configuration object
    pub health_check: Option<HealthCheckConfig>,
    // === Retry and Failover ===
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Failover configuration
    pub failover: Option<FailoverConfig>,
    // === Server Management ===
    /// Upstream servers configuration
    pub upstream_servers: Option<Vec<UpstreamServer>>,
    // === Circuit Breaker Integration ===
    /// Circuit breaker settings (for resilience)
    pub circuit_breaker: Option<CircuitBreakerConfig>,
}
/// Load balancing algorithm enumeration
pub enum LoadBalancingAlgorithm {
    /// Round-robin distribution
    RoundRobin,
    /// Least connections
    LeastConnections,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// IP hash
    IpHash,
    /// Random selection
    Random,
    /// Least response time
    LeastResponseTime,
    /// Resource based
    ResourceBased,
/// Upstream server configuration}


pub struct UpstreamServer {
    /// Server address
    pub address: String,
    /// Server port
    pub port: u16,
    /// Server weight for load balancing
    /// Maximum number of connections
    pub max_connections: Option<u32>,
    /// Server enabled status
    /// Health check URL for this server
    pub health_check_url: Option<String>,
    /// Connection timeout for this server
    pub connection_timeout: Option<Duration>,
    /// Additional server metadata
    pub metadata: Option<std::collections::HashMap<String, String>>,
/// Health check configuration for load balancing
pub struct HealthCheckConfig {
    /// Health check URL path
    pub path: String,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of consecutive failures to mark unhealthy
    pub unhealthy_threshold: u32,
    /// Number of consecutive successes to mark healthy
    pub healthy_threshold: u32,
    /// HTTP method for health check
    pub method: String,
    /// Expected HTTP status codes
    pub expected_status_codes: Vec<u16>,
    /// Custom headers for health check
    pub headers: Option<std::collections::HashMap<String, String>>,
/// Load balancer session affinity configuration
pub struct SessionAffinityConfig {
    /// Session affinity enabled
    pub enabled: bool,
    /// Affinity method (cookie, ip, header)
    pub method: SessionAffinityMethod,
    /// Cookie name for cookie-based affinity
    pub cookie_name: Option<String>,
    /// Header name for header-based affinity
    pub header_name: Option<String>,
    /// Session timeout
/// Session affinity methods
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
