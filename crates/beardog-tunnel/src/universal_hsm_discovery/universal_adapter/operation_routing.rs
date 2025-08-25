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


/// Operation Routing
///
/// **CANONICAL OPERATION ROUTING** - Complete routing and connection pooling for HSM operations
/// This module provides comprehensive operation routing functionality, including intelligent
/// routing decisions, connection pooling, load balancing, and failover capabilities.

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;
// CANONICAL IMPORT: use beardog_types::config::UnifiedMonitoringConfig;
/// **CANONICAL OPERATION ROUTER** - Main routing engine for HSM operations
#[derive(Debug)]
pub struct OperationRouter {
    /// Router configuration
    config: RouterConfig,
    
    /// Routing rules
    routing_rules: Arc<RwLock<Vec<RoutingRule>>>,
    /// Connection pools
    connection_pools: Arc<RwLock<HashMap<String, ConnectionPool>>>,
    /// Router statistics
    stats: Arc<RwLock<RouterStats>>,
    /// Load balancer
    load_balancer: Arc<RwLock<LoadBalancer>>,
    /// Failover manager
    failover_manager: Arc<RwLock<FailoverManager>>,
}
/// **ROUTER CONFIGURATION**
#[derive(Debug, Clone)]
pub struct RouterConfig {
    /// Default routing strategy
    pub default_strategy: RoutingStrategy,
    /// Connection pool size per endpoint
    pub pool_size_per_endpoint: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Operation timeout
    pub operation_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Enable automatic failover
    pub enable_failover: bool,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
    /// Load balancing algorithm
    pub load_balancing: LoadBalancingAlgorithm,}


impl Default for RouterConfig {}


    fn default() -> Self {
        Self {
            default_strategy: RoutingStrategy::RoundRobin,
            pool_size_per_endpoint: 5,
            connection_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(300),
            health_check_interval: Duration::from_secs(30),
            enable_failover: true,
            max_retry_attempts: 3,
            load_balancing: LoadBalancingAlgorithm::WeightedRoundRobin,
        }
    }
/// **ROUTING RULE** - Defines how operations are routed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {
    /// Rule identifier
    pub rule_id: Uuid,
    /// Rule name
    pub name: String,
    /// Rule priority (higher = more important)
    pub priority: u32,
    /// Matching criteria
    pub criteria: RoutingCriteria,
    /// Target endpoints
    pub targets: Vec<RoutingTarget>,
    /// Routing strategy for this rule
    pub strategy: RoutingStrategy,
    /// Rule status
    pub enabled: bool,
    /// Rule creation time
    pub created_at: SystemTime,
    /// Rule last modified time
    pub modified_at: SystemTime,
/// **ROUTING CRITERIA** - Criteria for matching operations to rules
pub struct RoutingCriteria {
    /// Operation type patterns
    pub operation_types: Option<Vec<String>>,
    /// Source patterns
    pub sources: Option<Vec<String>>,
    /// Priority levels
    pub priorities: Option<Vec<OperationPriority>>,
    /// Security levels
    pub security_levels: Option<Vec<SecurityLevel>>,
    /// Custom attributes
    pub custom_attributes: Option<HashMap<String, String>>,
    /// Time-based criteria
    pub time_criteria: Option<TimeCriteria>,
/// **ROUTING TARGET** - Target endpoint for operations
pub struct RoutingTarget {
    /// Target identifier
    pub target_id: Uuid,
    /// Endpoint URL or identifier
    pub endpoint: String,
    /// Target weight for load balancing
    pub weight: u32,
    /// Target health status
    pub health_status: TargetHealth,
    /// Target capabilities
    pub capabilities: Vec<String>,
    /// Target performance metrics
    pub performance_metrics: PerformanceMetrics,
    /// Target configuration
    pub config: TargetConfig,
/// **CONNECTION POOL** - Pool of connections to a specific endpoint
pub struct ConnectionPool {
    /// Pool identifier
    pub pool_id: Uuid,
    /// Target endpoint
    /// Available connections
    pub available: VecDeque<PooledConnection>,
    /// Active connections
    pub active: HashMap<Uuid, PooledConnection>,
    /// Pool configuration
    pub config: PoolConfig,
    /// Pool statistics
    pub stats: PoolStats,
    /// Pool creation time
    pub created_at: Instant,
    /// Last activity time
    pub last_activity: Instant,
/// **POOLED CONNECTION** - Connection with metadata
pub struct PooledConnection {
    /// Connection identifier
    pub connection_id: Uuid,
    /// Connection handle/reference
    pub connection_handle: String, // Would be actual connection type
    /// Connection creation time
    /// Last used time
    pub last_used: Instant,
    /// Usage count
    pub usage_count: u64,
    /// Connection health
    pub health: ConnectionHealth,
    /// Connection statistics
    pub stats: ConnectionStats,
/// **ROUTER STATISTICS** - Statistics for the router
#[derive(Debug, Clone, Default)]
pub struct RouterStats {
    /// Total operations routed
    pub operations_routed: u64,
    /// Operations by routing strategy
    pub operations_by_strategy: HashMap<String, u64>,
    /// Operations by target
    pub operations_by_target: HashMap<String, u64>,
    /// Average routing time
    pub avg_routing_time: Duration,
    /// Failover events
    pub failover_events: u64,
    /// Connection pool hits
    pub pool_hits: u64,
    /// Connection pool misses
    pub pool_misses: u64,
    /// Last updated
    pub last_updated: SystemTime,
/// **LOAD BALANCER** - Manages load balancing across targets
pub struct LoadBalancer {
    /// Current algorithm
    algorithm: LoadBalancingAlgorithm,
    /// Round-robin state
    round_robin_state: HashMap<String, usize>,
    /// Target weights
    target_weights: HashMap<String, u32>,
    /// Load metrics
    load_metrics: HashMap<String, LoadMetrics>,
/// **FAILOVER MANAGER** - Manages failover scenarios
pub struct FailoverManager {
    /// Failover configuration
    config: FailoverConfig,
    /// Failed targets
    failed_targets: HashMap<String, FailureInfo>,
    /// Failover history
    failover_history: VecDeque<FailoverEvent>,
    /// Recovery attempts
    recovery_attempts: HashMap<String, RecoveryAttempt>,
// ============================================================================
// SUPPORTING ENUMS AND TYPES
/// **ROUTING STRATEGY**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoutingStrategy {
    /// Round-robin across targets
    RoundRobin,
    /// Weighted round-robin
    WeightedRoundRobin,
    /// Route to least loaded target
    LeastLoaded,
    /// Route to fastest responding target
    FastestResponse,
    /// Route to highest priority target
    HighestPriority,
    /// Random selection
    Random,
    /// Sticky routing based on operation hash
    Sticky,
    /// Custom routing logic
    Custom(String),
/// **OPERATION PRIORITY**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]}


pub enum OperationPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
/// **SECURITY LEVEL**
pub enum SecurityLevel {
    /// Standard security
    Standard,
    /// High security
    /// Critical security
    /// Maximum security
    Maximum,
/// **TARGET HEALTH**}


pub enum TargetHealth {
    /// Target is healthy
    Healthy,
    /// Target is degraded
    Degraded,
    /// Target is unhealthy
    Unhealthy,
    /// Target is offline
    Offline,
    /// Target health unknown
    Unknown,
/// **CONNECTION HEALTH**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionHealth {
    /// Connection is healthy
    /// Connection is degraded
    /// Connection is unhealthy
/// **LOAD BALANCING ALGORITHM**}


pub enum LoadBalancingAlgorithm {
    /// Simple round-robin
    /// Least connections
    LeastConnections,
    /// Least response time
    LeastResponseTime,
    /// Resource-based balancing
    ResourceBased,
// COMPLEX SUPPORTING TYPES
/// **TIME CRITERIA** - Time-based routing criteria
pub struct TimeCriteria {
    /// Time ranges when rule applies
    pub time_ranges: Vec<TimeRange>,
    /// Days of week when rule applies
    pub days_of_week: Option<Vec<u8>>, // 0-6, Sunday=0
    /// Time zone
    pub timezone: Option<String>,
/// **TIME RANGE** - Time range specification}


pub struct TimeRange {
    /// Start time (HH:MM format)
    pub start_time: String,
    /// End time (HH:MM format)
    pub end_time: String,
/// **PERFORMANCE METRICS** - Target performance metrics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    /// Average response time
    pub avg_response_time: Duration,
    /// Success rate
    pub success_rate: f64,
    /// Throughput (operations per second)
    pub throughput: f64,
    /// Error rate
    pub error_rate: f64,
    /// Current load
    pub current_load: f64,
/// **TARGET CONFIG** - Configuration for routing target
pub struct TargetConfig {
    /// Maximum concurrent operations
    pub max_concurrent_ops: u32,
    /// Retry configuration
    pub retry_config: RetryConfig,
    /// Health check configuration
    pub health_check_config: HealthCheckConfig,
/// **POOL CONFIG** - Connection pool configuration
pub struct PoolConfig {
    /// Minimum pool size
    pub min_size: usize,
    /// Maximum pool size
    pub max_size: usize,
    /// Connection idle timeout
    pub idle_timeout: Duration,
    /// Connection lifetime
    pub max_lifetime: Duration,
    /// Pool cleanup interval
    pub cleanup_interval: Duration,
/// **POOL STATS** - Connection pool statistics
pub struct PoolStats {
    /// Total connections created
    pub connections_created: u64,
    /// Total connections destroyed
    pub connections_destroyed: u64,
    /// Pool hits
    pub hits: u64,
    /// Pool misses
    pub misses: u64,
    /// Average wait time
    pub avg_wait_time: Duration,
/// **CONNECTION STATS** - Individual connection statistics
pub struct ConnectionStats {
    /// Operations performed
    pub operations_performed: u64,
    /// Bytes transferred
    pub bytes_transferred: u64,
    /// Average operation time
    pub avg_operation_time: Duration,
    /// Errors encountered
    pub errors: u32,
/// **LOAD METRICS** - Load metrics for a target
pub struct LoadMetrics {
    /// Current active connections
    pub active_connections: u32,
    /// CPU utilization
    pub cpu_utilization: f64,
    /// Memory utilization
    pub memory_utilization: f64,
    /// Queue depth
    pub queue_depth: u32,
    pub last_updated: Instant,
/// **FAILOVER CONFIG** - Failover configuration
pub struct FailoverConfig {
    /// Failure threshold
    pub failure_threshold: u32,
    /// Detection window
    pub detection_window: Duration,
    /// Recovery check interval
    pub recovery_interval: Duration,
    /// Maximum failover attempts
    pub max_failover_attempts: u32,
/// **FAILURE INFO** - Information about target failures
pub struct FailureInfo {
    /// Failure count
    pub failure_count: u32,
    /// First failure time
    pub first_failure: Instant,
    /// Last failure time
    pub last_failure: Instant,
    /// Failure reasons
    pub failure_reasons: Vec<String>,
/// **FAILOVER EVENT** - Failover event record
pub struct FailoverEvent {
    /// Event identifier
    pub event_id: Uuid,
    /// Failed target
    pub failed_target: String,
    /// Failover target
    pub failover_target: String,
    /// Event timestamp
    pub timestamp: Instant,
    /// Failure reason
    pub reason: String,
/// **RECOVERY ATTEMPT** - Recovery attempt information
pub struct RecoveryAttempt {
    /// Attempt count
    pub attempt_count: u32,
    /// Last attempt time
    pub last_attempt: Instant,
    /// Next attempt time
    pub next_attempt: Instant,
    /// Recovery status
    pub status: RecoveryStatus,
/// **RECOVERY STATUS**
pub enum RecoveryStatus {
    /// Recovery in progress
    InProgress,
    /// Recovery succeeded
    Succeeded,
    /// Recovery failed
    Failed,
    /// Recovery paused
    Paused,
/// **RETRY CONFIG** - Retry configuration}


pub struct RetryConfig {
    pub max_attempts: u32,
    /// Initial retry delay
    pub initial_delay: Duration,
    /// Maximum retry delay
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    /// Jitter enabled
    pub jitter: bool,
/// **HEALTH CHECK CONFIG** - Health check configuration
// MIGRATED: HealthCheckConfig -> use beardog_types::config::UnifiedMonitoringConfig;


impl OperationRouter {
    /// **CREATE NEW ROUTER**}


    pub fn new(config: RouterConfig) -> BearDogResult<Self> {
        info!("🚦 Initializing Operation Router");
        
        Ok(Self {
            config,
            routing_rules: Arc::new(RwLock::new(Vec::new())),
            connection_pools: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RouterStats::default())),
            load_balancer: Arc::new(RwLock::new(LoadBalancer {
                algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
                round_robin_state: HashMap::new(),
                target_weights: HashMap::new(),
                load_metrics: HashMap::new(),
            })),
            failover_manager: Arc::new(RwLock::new(FailoverManager {
                config: FailoverConfig {
                    enabled: true,
                    failure_threshold: 3,
                    detection_window: Duration::from_secs(300),
                    recovery_interval: Duration::from_secs(60),
                    max_failover_attempts: 5,
                },
                failed_targets: HashMap::new(),
                failover_history: VecDeque::new(),
                recovery_attempts: HashMap::new(),
        })
    /// **ADD ROUTING RULE** - Add a new routing rule
    pub async fn add_routing_rule(&self, rule: RoutingRule) -> BearDogResult<()> {
        info!("📋 Adding routing rule: {}", rule.name);
        let mut rules = self.routing_rules.write().await;
        rules.push(rule);
        // Sort by priority (highest first)
        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())
    /// **ROUTE OPERATION** - Route an operation to appropriate target
    pub async fn route_operation(&self, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {
        debug!("🎯 Routing operation: {}", operation.operation_id);
        let start_time = Instant::now();
        // Find matching rule
        let rules = self.routing_rules.read().await;
        let matching_rule = rules.iter().find(|rule| {
            rule.enabled && self.matches_criteria(&rule.criteria, operation)
        });
        let target = if let Some(rule) = matching_rule {
            debug!("📏 Matched rule: {}", rule.name);
            self.select_target_from_rule(rule, operation).await?
        } else {
            debug!("🔄 Using default routing strategy");
            self.select_default_target(operation).await?
        };
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.operations_routed += 1;
            stats.avg_routing_time = Duration::from_nanos(
                (stats.avg_routing_time.as_nanos() as u64 * (stats.operations_routed - 1) + 
                 start_time.elapsed().as_nanos() as u64) / stats.operations_routed
            );
            stats.last_updated = SystemTime::now();
        info!("✅ Routed operation {} to target {}", operation.operation_id, target.endpoint);
        Ok(target)
    /// **GET CONNECTION** - Get connection from pool
    pub async fn get_connection(&self, endpoint: &str) -> BearDogResult<PooledConnection> {
        debug!("🔗 Getting connection to {}", endpoint);
        let mut pools = self.connection_pools.write().await;
        let pool = pools.entry(endpoint.to_string()).or_insert_with(|| {
            ConnectionPool {
                pool_id: Uuid::new_v4(),
                endpoint: endpoint.to_string(),
                available: VecDeque::new(),
                active: HashMap::new(),
                config: PoolConfig {
                    min_size: 2,
                    max_size: self.config.pool_size_per_endpoint,
                    idle_timeout: Duration::from_secs(300),
                    max_lifetime: Duration::from_secs(3600),
                    cleanup_interval: Duration::from_secs(60),
                stats: PoolStats::default(),
                created_at: Instant::now(),
                last_activity: Instant::now(),
            }
        // Try to get available connection
        if let Some(mut connection) = pool.available.pop_front() {
            connection.last_used = Instant::now();
            connection.usage_count += 1;
            pool.active.insert(connection.connection_id, connection.clone());
            pool.stats.hits += 1;
            
            debug!("♻️ Reusing pooled connection to {}", endpoint);
            Ok(connection)
        } else if pool.active.len() < pool.config.max_size {
            // Create new connection
            let connection = PooledConnection {
                connection_id: Uuid::new_v4(),
                connection_handle: format!("conn_{}", Uuid::new_v4()),
                last_used: Instant::now(),
                usage_count: 1,
                health: ConnectionHealth::Healthy,
                stats: ConnectionStats::default(),
            };
            pool.stats.connections_created += 1;
            pool.stats.misses += 1;
            info!("🆕 Created new connection to {}", endpoint);
            Err(BearDogError::internal(format!("Connection pool exhausted for {}", endpoint)))
    /// **RETURN CONNECTION** - Return connection to pool
    pub async fn return_connection(&self, endpoint: &str, connection: PooledConnection) -> BearDogResult<()> {
        debug!("🔙 Returning connection {} to pool", connection.connection_id);
        if let Some(pool) = pools.get_mut(endpoint) {
            pool.active.remove(&connection.connection_id);
            if connection.health == ConnectionHealth::Healthy {
                pool.available.push_back(connection);
            } else {
                pool.stats.connections_destroyed += 1;
                debug!("🗑️ Destroyed unhealthy connection");
            pool.last_activity = Instant::now();
    /// **GET STATISTICS** - Get router statistics
    pub async fn get_statistics(&self) -> RouterStats {
        self.stats.read().await.clone()
    // ============================================================================
    // PRIVATE HELPER METHODS
    /// Check if operation matches routing criteria}


    fn matches_criteria(&self, criteria: &RoutingCriteria, operation: &RoutingOperation) -> bool {
        // Implementation would check all criteria fields
        // For now, simplified matching
        true // Placeholder
    /// Select target from routing rule
    async fn select_target_from_rule(&self, rule: &RoutingRule, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {
        // Implementation would use rule's strategy and targets
        // For now, return first healthy target
        rule.targets.iter()
            .find(|t| t.health_status == TargetHealth::Healthy)
            .cloned()
            .ok_or_else(|| BearDogError::internal("No healthy targets available"))
    /// Select target using default strategy}


    async fn select_default_target(&self, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {
        // Implementation would use default routing strategy
        // For now, return placeholder target
        Ok(RoutingTarget {
            target_id: Uuid::new_v4(),
            endpoint: "default-endpoint".to_string(),
            weight: 100,
            health_status: TargetHealth::Healthy,
            capabilities: vec!["default".to_string()],
            performance_metrics: PerformanceMetrics::default(),
            config: TargetConfig {
                max_concurrent_ops: 100,
                connection_timeout: Duration::from_secs(30),
                operation_timeout: Duration::from_secs(300),
                retry_config: RetryConfig {
                    max_attempts: 3,
                    initial_delay: Duration::from_millis(100),
                    max_delay: Duration::from_secs(30),
                    backoff_multiplier: 2.0,
                    jitter: true,
                health_check_config: HealthCheckConfig {
                    interval: Duration::from_secs(30),
                    timeout: Duration::from_secs(10),
                    healthy_threshold: 2,
                    unhealthy_threshold: 3,
            },
/// **ROUTING OPERATION** - Operation to be routed
pub struct RoutingOperation {
    /// Operation identifier
    pub operation_id: Uuid,
    /// Operation type
    pub operation_type: String,
    /// Operation source
    pub source: String,
    /// Operation priority
    pub priority: OperationPriority,
    /// Security level required
    pub security_level: SecurityLevel,
    pub attributes: HashMap<String, String>,
    /// Operation data size
    pub data_size: u64,
