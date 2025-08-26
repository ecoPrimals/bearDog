

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug)]
pub struct OperationRouter {

    config: RouterConfig,

    routing_rules: Arc<RwLock<Vec<RoutingRule>>>,

    connection_pools: Arc<RwLock<HashMap<String, ConnectionPool>>>,

    stats: Arc<RwLock<RouterStats>>,

    load_balancer: Arc<RwLock<LoadBalancer>>,

    failover_manager: Arc<RwLock<FailoverManager>>,
}

#[derive(Debug, Clone)]
pub struct RouterConfig {

    pub default_strategy: RoutingStrategy,

    pub pool_size_per_endpoint: usize,

    pub connection_timeout: Duration,

    pub operation_timeout: Duration,

    pub health_check_interval: Duration,

    pub enable_failover: bool,

    pub max_retry_attempts: u32,

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingRule {

    pub rule_id: Uuid,

    pub name: String,

    pub priority: u32,

    pub criteria: RoutingCriteria,

    pub targets: Vec<RoutingTarget>,

    pub strategy: RoutingStrategy,

    pub enabled: bool,

    pub created_at: SystemTime,

    pub modified_at: SystemTime,

pub struct RoutingCriteria {

    pub operation_types: Option<Vec<String>>,

    pub sources: Option<Vec<String>>,

    pub priorities: Option<Vec<OperationPriority>>,

    pub security_levels: Option<Vec<SecurityLevel>>,

    pub custom_attributes: Option<HashMap<String, String>>,

    pub time_criteria: Option<TimeCriteria>,

pub struct RoutingTarget {

    pub target_id: Uuid,

    pub endpoint: String,

    pub weight: u32,

    pub health_status: TargetHealth,

    pub capabilities: Vec<String>,

    pub performance_metrics: PerformanceMetrics,

    pub config: TargetConfig,

pub struct ConnectionPool {

    pub pool_id: Uuid,

    pub available: VecDeque<PooledConnection>,

    pub active: HashMap<Uuid, PooledConnection>,

    pub config: PoolConfig,

    pub stats: PoolStats,

    pub created_at: Instant,

    pub last_activity: Instant,

pub struct PooledConnection {

    pub connection_id: Uuid,

    pub connection_handle: String, // Would be actual connection type

    pub last_used: Instant,

    pub usage_count: u64,

    pub health: ConnectionHealth,

    pub stats: ConnectionStats,

#[derive(Debug, Clone, Default)]
pub struct RouterStats {

    pub operations_routed: u64,

    pub operations_by_strategy: HashMap<String, u64>,

    pub operations_by_target: HashMap<String, u64>,

    pub avg_routing_time: Duration,

    pub failover_events: u64,

    pub pool_hits: u64,

    pub pool_misses: u64,

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RoutingStrategy {

    RoundRobin,

    WeightedRoundRobin,

    LeastLoaded,

    FastestResponse,

    HighestPriority,

    Random,

    Sticky,

    Custom(String),

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]}

pub enum OperationPriority {

    Low,

    Normal,

    High,

    Critical,

pub enum SecurityLevel {

    Standard,

    Maximum,

pub enum TargetHealth {

    Healthy,

    Degraded,

    Unhealthy,

    Offline,

    Unknown,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionHealth {

pub enum LoadBalancingAlgorithm {

    LeastConnections,

    LeastResponseTime,

    ResourceBased,

pub struct TimeCriteria {

    pub time_ranges: Vec<TimeRange>,

    pub days_of_week: Option<Vec<u8>>, // 0-6, Sunday=0

    pub timezone: Option<String>,

pub struct TimeRange {

    pub start_time: String,

    pub end_time: String,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {

    pub avg_response_time: Duration,

    pub success_rate: f64,

    pub throughput: f64,

    pub error_rate: f64,

    pub current_load: f64,

pub struct TargetConfig {

    pub max_concurrent_ops: u32,

    pub retry_config: RetryConfig,

    pub health_check_config: HealthCheckConfig,

pub struct PoolConfig {

    pub min_size: usize,

    pub max_size: usize,

    pub idle_timeout: Duration,

    pub max_lifetime: Duration,

    pub cleanup_interval: Duration,

pub struct PoolStats {

    pub connections_created: u64,

    pub connections_destroyed: u64,

    pub hits: u64,

    pub misses: u64,

    pub avg_wait_time: Duration,

pub struct ConnectionStats {

    pub operations_performed: u64,

    pub bytes_transferred: u64,

    pub avg_operation_time: Duration,

    pub errors: u32,

pub struct LoadMetrics {

    pub active_connections: u32,

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub queue_depth: u32,
    pub last_updated: Instant,

pub struct FailoverConfig {

    pub failure_threshold: u32,

    pub detection_window: Duration,

    pub recovery_interval: Duration,

    pub max_failover_attempts: u32,

pub struct FailureInfo {

    pub failure_count: u32,

    pub first_failure: Instant,

    pub last_failure: Instant,

    pub failure_reasons: Vec<String>,

pub struct FailoverEvent {

    pub event_id: Uuid,

    pub failed_target: String,

    pub failover_target: String,

    pub timestamp: Instant,

    pub reason: String,

pub struct RecoveryAttempt {

    pub attempt_count: u32,

    pub last_attempt: Instant,

    pub next_attempt: Instant,

    pub status: RecoveryStatus,

pub enum RecoveryStatus {

    InProgress,

    Succeeded,

    Failed,

    Paused,

pub struct RetryConfig {
    pub max_attempts: u32,

    pub initial_delay: Duration,

    pub max_delay: Duration,

    pub backoff_multiplier: f64,

    pub jitter: bool,

impl OperationRouter {

    pub fn new(config: RouterConfig) -> BearDogResult<Self> {
        info!("🚦 Initializing Operation Router");
        
        Ok(Self {
            config,
            routing_rules: Arc::new(RwLock::new(Vec::new())),
            connection_pools: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: Arc::new(RwLock::new(RouterStats::default())),
            load_balancer: Arc::new(RwLock::new(LoadBalancer {
                algorithm: LoadBalancingAlgorithm::WeightedRoundRobin,
                round_robin_state: HashMap::with_capacity(16),
                target_weights: HashMap::with_capacity(16),
                load_metrics: HashMap::with_capacity(16),
            })),
            failover_manager: Arc::new(RwLock::new(FailoverManager {
                config: FailoverConfig {
                    enabled: true,
                    failure_threshold: 3,
                    detection_window: Duration::from_secs(300),
                    recovery_interval: Duration::from_secs(60),
                    max_failover_attempts: 5,
                },
                failed_targets: HashMap::with_capacity(16),
                failover_history: VecDeque::new(),
                recovery_attempts: HashMap::with_capacity(16),
        })

    pub async fn add_routing_rule(&self, rule: RoutingRule) -> BearDogResult<()> {
        info!("📋 Adding routing rule: {}", rule.name);
        let mut rules = self.routing_rules.write().await;
        rules.push(rule);

        rules.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())

    pub async fn route_operation(&self, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {
        debug!("🎯 Routing operation: {}", operation.operation_id);
        let start_time = Instant::now();

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

    pub async fn get_connection(&self, endpoint: &str) -> BearDogResult<PooledConnection> {
        debug!("🔗 Getting connection to {}", endpoint);
        let mut pools = self.connection_pools.write().await;
        let pool = pools.entry(endpoint.to_string()).or_insert_with(|| {
            ConnectionPool {
                pool_id: Uuid::new_v4(),
                endpoint: endpoint.to_string(),
                available: VecDeque::new(),
                active: HashMap::with_capacity(16),
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

        if let Some(mut connection) = pool.available.pop_front() {
            connection.last_used = Instant::now();
            connection.usage_count += 1;
            pool.active.insert(connection.connection_id, connection.clone());
            pool.stats.hits += 1;
            
            debug!("♻️ Reusing pooled connection to {}", endpoint);
            Ok(connection)
        } else if pool.active.len() < pool.config.max_size {

            let connection = PooledConnection {
                connection_id: Uuid::new_v4(),
                connection_handle: format_args!("conn_{}", Uuid::new_v4().to_string()),
                last_used: Instant::now(),
                usage_count: 1,
                health: ConnectionHealth::Healthy,
                stats: ConnectionStats::default(),
            };
            pool.stats.connections_created += 1;
            pool.stats.misses += 1;
            info!("🆕 Created new connection to {}", endpoint);
            Err(BearDogError::internal(format_args!("Connection pool exhausted for {}", endpoint).to_string()))

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

    pub async fn get_statistics(&self) -> RouterStats {
        self.stats.read().await.clone()

    fn matches_criteria(&self, criteria: &RoutingCriteria, operation: &RoutingOperation) -> bool {

        true // Placeholder

    async fn select_target_from_rule(&self, rule: &RoutingRule, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {

        rule.targets.iter()
            .find(|t| t.health_status == TargetHealth::Healthy)
            .cloned()
            .ok_or_else(|| BearDogError::internal("No healthy targets available"))

    async fn select_default_target(&self, operation: &RoutingOperation) -> BearDogResult<RoutingTarget> {

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

pub struct RoutingOperation {

    pub operation_id: Uuid,

    pub operation_type: String,

    pub source: String,

    pub priority: OperationPriority,

    pub security_level: SecurityLevel,
    pub attributes: HashMap<String, String>,

    pub data_size: u64,
