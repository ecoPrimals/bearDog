

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::core_types::*;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone)]
    routing_rules: Arc<RwLock<Vec<RoutingRule>>>,

    connection_pools: Arc<RwLock<HashMap<String, ConnectionPool>>>,

    stats: Arc<RwLock<RouterStats>>,

    load_balancer: Arc<RwLock<LoadBalancer>>,

    failover_manager: Arc<RwLock<FailoverManager>>,
}

pub use beardog_types::canonical::configuration::RouterConfig;

impl Default for RouterConfig {}

    fn default(RoutingStrategy::RoundRobin,
            pool_size_per_endpoint: 5,
            connection_timeout: Duration::from_secs(30),
            operation_timeout: Duration::from_secs(300),
            health_check_interval: Duration::from_secs(true,
            max_retry_attempts: 3,
            load_balancing: LoadBalancingAlgorithm::WeightedRoundRobin,
        }
    }

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,

    /// Number of priority
    pub priority: u32,

    /// The criteria value
    pub criteria: RoutingCriteria,

    /// Collection of targets
    pub targets: Vec<RoutingTarget>,

    /// The strategy value
    pub strategy: RoutingStrategy,

    /// Whether feature is enabled
    pub enabled: bool,

    /// The created at value
    pub created_at: SystemTime,

    /// The modified at value
    pub modified_at: SystemTime,

pub struct RoutingCriteria {

    /// Optional operation types
    pub operation_types: Option<Vec<String>>,

    /// Optional sources
    pub sources: Option<Vec<String>>,

    /// Optional priorities
    pub priorities: Option<Vec<OperationPriority>>,

    /// Optional security levels
    pub security_levels: Option<Vec<SecurityLevel>>,

    /// Optional custom attributes
    pub custom_attributes: Option<HashMap<String, String>>,


    pub time_criteria: Option<TimeCriteria>,

pub struct RoutingTarget {


    pub target_id: Uuid,

    /// The endpoint value
    pub endpoint: String,

    /// Number of weight
    pub weight: u32,

    /// Current status of the health
    pub health_status: TargetHealth,

    /// Collection of capabilities
    pub capabilities: Vec<String>,


    pub performance_metrics: PerformanceMetrics,


    pub config: TargetConfig,

pub struct ConnectionPool {


    pub pool_id: Uuid,

    /// Collection of available
    pub available: VecDeque<PooledConnection>,

    /// Mapping of active
    pub active: HashMap<Uuid, PooledConnection>,


    pub config: PoolConfig,

    /// The stats value
    pub stats: PoolStats,

    /// The created at value
    pub created_at: Instant,

    /// The last activity value
    pub last_activity: Instant,

pub struct PooledConnection {


    pub connection_id: Uuid,

    /// The connection handle value
    pub connection_handle: String, // Would be actual connection type

    /// The last used value
    pub last_used: Instant,

    /// Number of usage
    pub usage_count: u64,

    /// The health value
    pub health: ConnectionHealth,

    /// The stats value
    pub stats: ConnectionStats,

#[derive(Debug, Clone)]
    /// Mapping of operations by strategy
    pub operations_by_strategy: HashMap<String, u64>,

    /// Mapping of operations by target
    pub operations_by_target: HashMap<String, u64>,


    pub avg_routing_time: Duration,

    /// Number of failover_events
    pub failover_events: u64,

    /// Number of pool_hits
    pub pool_hits: u64,

    /// Number of pool_misses
    pub pool_misses: u64,

    /// The last updated value
    pub last_updated: SystemTime,

pub struct LoadBalancer {

    algorithm: LoadBalancingAlgorithm,

    round_robin_state: HashMap<String, usize>,

    target_weights: HashMap<String, u32>,

    load_metrics: HashMap<String, LoadMetrics>,

pub struct FailoverManager {

    config: FailoverConfig,

    failed_targets: HashMap<String, FailureInfo>,

    failover_history: VecDeque<FailoverEvent>,

    recovery_attempts: HashMap<String, RecoveryAttempt>,

#[derive(Debug, Clone)]
    /// Optional days of week
    pub days_of_week: Option<Vec<u8>>, // 0-6, Sunday=0


    pub timezone: Option<String>,

pub struct TimeRange {


    pub start_time: String,


    pub end_time: String,

#[derive(Debug, Clone)]
    /// The success rate value
    pub success_rate: f64,

    /// The throughput value
    pub throughput: f64,

    /// The error rate value
    pub error_rate: f64,

    /// The current load value
    pub current_load: f64,

pub struct TargetConfig {

    /// Number of max_concurrent_ops
    pub max_concurrent_ops: u32,


    pub retry_config: RetryConfig,


    pub health_check_config: HealthCheckConfig,

pub struct PoolConfig {

    /// Number of min_size
    pub min_size: usize,

    /// Number of max_size
    pub max_size: usize,


    pub idle_timeout: Duration,


    pub max_lifetime: Duration,

    /// The cleanup interval value
    pub cleanup_interval: Duration,

pub struct PoolStats {

    /// Number of connections_created
    pub connections_created: u64,

    /// Number of connections_destroyed
    pub connections_destroyed: u64,

    /// Number of hits
    pub hits: u64,

    /// Number of misses
    pub misses: u64,


    pub avg_wait_time: Duration,

pub struct ConnectionStats {


    pub operations_performed: u64,

    /// Number of bytes_transferred
    pub bytes_transferred: u64,


    pub avg_operation_time: Duration,

    /// Number of errors
    pub errors: u32,

pub struct LoadMetrics {

    /// Number of active_connections
    pub active_connections: u32,

    /// The cpu utilization value
    pub cpu_utilization: f64,

    /// The memory utilization value
    pub memory_utilization: f64,

    /// Number of queue_depth
    pub queue_depth: u32,
    /// The last updated value
    pub last_updated: Instant,

pub struct FailoverConfig {

    /// Number of failure_threshold
    pub failure_threshold: u32,

    /// The detection window value
    pub detection_window: Duration,

    /// The recovery interval value
    pub recovery_interval: Duration,

    /// Number of max_failover_attempts
    pub max_failover_attempts: u32,

pub struct FailureInfo {

    /// Number of failure
    pub failure_count: u32,

    /// The first failure value
    pub first_failure: Instant,

    /// The last failure value
    pub last_failure: Instant,

    /// Collection of failure reasons
    pub failure_reasons: Vec<String>,

pub struct FailoverEvent {


    pub event_id: Uuid,

    /// The failed target value
    pub failed_target: String,

    /// The failover target value
    pub failover_target: String,


    pub timestamp: Instant,

    /// The reason value
    pub reason: String,

pub struct RecoveryAttempt {

    /// Number of attempt
    pub attempt_count: u32,

    /// The last attempt value
    pub last_attempt: Instant,

    /// The next attempt value
    pub next_attempt: Instant,

    /// Current status of the component
    pub status: RecoveryStatus,

pub enum RecoveryStatus {


    /// Operation in progress
    InProgress,


    /// State indicating succeeded
    Succeeded,


    /// Error or failure state
    Failed,


    /// State indicating paused
    Paused,
}

// MIGRATED: Now using canonical RetryConfig from beardog-types
// See: crates/beardog-types/src/canonical/config/domains/retry.rs
//
// Old definition (replaced Nov 8, 2025):
// pub struct RetryConfig {
//     pub max_attempts: u32,
//     pub initial_delay: Duration,
//     pub max_delay: Duration,
//     pub backoff_multiplier: f64,
//     pub jitter: bool,
// }
pub use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig as RetryConfig;

impl OperationRouter {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: RouterConfig) -> Result<Self, BearDogError> {
        info!("🚦 Initializing Operation Router");
        
        /// Successful completion state
        Ok(Self {
            config,
            routing_rules: Arc::new(RwLock::new(Vec::new())),
            connection_pools: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: Arc::new(RwLock::new(RouterStats::default(Arc::new(RwLock::new(LoadBalancer {
                algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
                round_robin_state: HashMap::with_capacity(16),
                target_weights: HashMap::with_capacity(16),
                load_metrics: HashMap::with_capacity(Arc::new(RwLock::new(FailoverManager {
                config: FailoverConfig {
                    enabled: true,
                    failure_threshold: 3,
                    detection_window: Duration::from_secs(300),
                    recovery_interval: Duration::from_secs(5,
                },
                },
                },
                failed_targets: HashMap::with_capacity(16),
                failover_history: VecDeque::new(),
                recovery_attempts: HashMap::with_capacity(16),
        })

/// Add Routing Rule operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn add_routing_rule(&self, rule: RoutingRule) -> Result<(), BearDogError> {
        info!("📋 Adding routing rule: {}", rule.name);
        let mut rules = self.routing_rules.write();
        rules.push(rule);

        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())

/// Route Operation operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn route_operation(&self, operation: &RoutingOperation) -> Result<RoutingTarget, BearDogError> {
        debug!("🎯 Routing operation: {}", operation.operation_id);
        let start_time = Instant::now({}", rule.name);
            self.select_target_from_rule(rule, operation)?
        } else {
            debug!("🔄 Using default routing strategy");
            self.select_default_target(operation)?
        };

        {
            let mut stats = self.stats.write();
            stats.operations_routed += 1;
            stats.avg_routing_time = Duration::from_nanos(
                (stats.avg_routing_time.as_nanos() as u64 * (stats.operations_routed - 1) + 
                 start_time.elapsed().as_nanos() as u64) / stats.operations_routed
            );
            stats.last_updated = SystemTime::now();
        info!("✅ Routed operation {} to target {}", operation.operation_id, target.endpoint);
        Ok(target)

/// Get Connection operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets connection
    /// Gets connection
    pub fn get_connection(&self, endpoint: &str) -> Result<PooledConnection, BearDogError> {
        debug!("🔗 Getting connection to {}", endpoint);
        let mut pools = self.connection_pools.write();
        let pool = pools.entry(endpoint.to_string()).or_insert_with(|| {
            ConnectionPool {
                pool_id: Uuid::new_v4(),
                endpoint: endpoint.to_string(),
                available: VecDeque::new(),
                active: HashMap::with_capacity(PoolConfig {
                    min_size: 2,
                    max_size: self.config.pool_size_per_endpoint,
                    idle_timeout: Duration::from_secs(300),
                    max_lifetime: Duration::from_secs(3600),
                    cleanup_interval: Duration::from_secs(60),
                stats: PoolStats::default(),
                created_at: Instant::now(),
                last_activity: Instant::now(),
            }

        if let Some(mut connection) = pool.available.pop_front() {
            connection.last_used = Instant::now();
            connection.usage_count += 1;
            pool.active.insert(connection.connection_id, connection);
            pool.stats.hits += 1;
            
            debug!("♻️ Reusing pooled connection to {}", endpoint);
            Ok(connection)
        } else if pool.active.len() < pool.config.max_size {

            let connection = PooledConnection {
                connection_id: Uuid::new_v4(format!("conn_{}", Uuid::new_v4()),
                last_used: Instant::now(1,
                health: ConnectionHealth::Healthy,
                stats: ConnectionStats::default(),
            };
            pool.stats.connections_created += 1;
            pool.stats.misses += 1;
            info!("🆕 Created new connection to {}", endpoint);
            Err(BearDogError::internal(&str, connection: PooledConnection) -> Result<(), BearDogError> {
        debug!("🔙 Returning connection {} to pool", connection.connection_id);
        if let Some(pool) = pools.get_mut(endpoint) {
            pool.active.remove(&connection.connection_id);
            if connection.health == ConnectionHealth::Healthy {
                pool.available.push_back(connection);
            } else {
                pool.stats.connections_destroyed += 1;
                debug!("🗑️ Destroyed unhealthy connection");
            pool.last_activity = Instant::now(&RoutingCriteria, operation: &RoutingOperation) -> bool {

        true // Placeholder


    fn select_target_from_rule(&RoutingRule, operation: &RoutingOperation) -> Result<RoutingTarget, BearDogError> {

        rule.targets.iter()
            .find(|t| t.health_status == TargetHealth::Healthy)
            .cloned()
            .ok_or_else(|| BearDogError::internal("No healthy targets available"))


    fn select_default_target(&self, operation: &RoutingOperation) -> Result<RoutingTarget, BearDogError> {

        Ok(RoutingTarget {
            target_id: Uuid::new_v4(),
            endpoint: "default-endpoint".to_string(),
            health_status: TargetHealth::Healthy,
            capabilities: vec!["default".to_string()],
            performance_metrics: PerformanceMetrics::default(TargetConfig {
                max_concurrent_ops: 100,
                connection_timeout: Duration::from_secs(30),
                operation_timeout: Duration::from_secs(RetryConfig {
                    max_attempts: 3,
                    initial_delay: Duration::from_millis(100),
                    max_delay: Duration::from_secs(2.0,
                    jitter: true,
                health_check_config: HealthCheckConfig {
                    interval: Duration::from_secs(30),
                    timeout: Duration::from_secs(2,
                    unhealthy_threshold: 3,
            },

pub struct RoutingOperation {


    pub operation_id: Uuid,

    /// The operation type value
    pub operation_type: String,

    /// The source value
    pub source: String,

    /// The priority value
    pub priority: OperationPriority,

    /// The security level value
    pub security_level: SecurityLevel,
    /// Mapping of attributes
    pub attributes: HashMap<String, String>,

    /// Number of data_size
    pub data_size: u64,
