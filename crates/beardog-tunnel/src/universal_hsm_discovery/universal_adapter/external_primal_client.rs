

use super::core_types::*;
use super::external_primal_service::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct DefaultExternalConnection {
    endpoint: String,
    connection_id: String,
}

impl ExternalPrimalConnection for DefaultExternalConnection {
    fn endpoint(&self) -> &str { &self.endpoint }
    fn connection_id(&self) -> &str { &self.connection_id }
}

#[derive(Debug)]
pub struct ExternalPrimalClient<C: ExternalPrimalConnection = DefaultExternalConnection> {

    config: ClientConfig,

    connection_pool: Arc<RwLock<ConnectionPool>>,

    stats: Arc<RwLock<ClientStats>>,

    active_connections: Arc<RwLock<HashMap<String, Arc<C>>>>,

    health_monitor: Arc<Mutex<HealthMonitor>>,
}

#[derive(Debug, Clone)]
pub struct ClientConfig {

    pub max_connections_per_primal: usize,

    pub connection_timeout: Duration,

    pub request_timeout: Duration,

    pub max_retry_attempts: u32,

    pub retry_delay: Duration,

    pub health_check_interval: Duration,

    pub keep_alive: bool,

    pub enable_compression: bool,}

impl Default for ClientConfig {}

    fn default() -> Self {
        Self {
            max_connections_per_primal: 10,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_retry_attempts: 3,
            retry_delay: Duration::from_millis(500),
            health_check_interval: Duration::from_secs(30),
            keep_alive: true,
            enable_compression: true,
        }
    }

pub struct ConnectionPool {

    connections: HashMap<String, Vec<PooledConnection>>,

    stats: PoolStats,

    config: PoolConfig,

pub struct PooledConnection {

    pub connection_id: Uuid,

    pub connection: Arc<dyn ExternalPrimalConnection>,

    pub created_at: Instant,

    pub last_used: Instant,

    pub usage_count: u64,

    pub status: ConnectionStatus,

#[derive(Debug, Clone, Default)]
pub struct PoolStats {

    pub connections_created: u64,

    pub connections_closed: u64,

    pub active_connections: u32,

    pub hit_rate: f64,

    pub avg_connection_lifetime: Duration,

pub struct PoolConfig {

    pub max_idle_time: Duration,

    pub max_lifetime: Duration,

    pub cleanup_interval: Duration,}

impl Default for PoolConfig {
            max_idle_time: Duration::from_secs(300), // 5 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(60), // 1 minute

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {

    Active,

    Idle,

    Unhealthy,

    Closing,

    Closed,

pub struct ClientStats {

    pub requests_sent: u64,

    pub responses_received: u64,

    pub errors_encountered: u64,

    pub avg_request_latency: Duration,

    pub success_rate: f64,

    pub last_updated: SystemTime,

pub struct HealthMonitor {

    health_results: HashMap<String, HealthResult>,

    last_check: Instant,

    config: HealthCheckConfig,

pub struct HealthResult {

    pub status: HealthStatus,

    pub response_time: Duration,

    pub last_check: Instant,

    pub consecutive_failures: u32,

    pub error: Option<String>,

impl Default for HealthCheckConfig {
            timeout: Duration::from_secs(10),
            max_failures: 3,
            interval: Duration::from_secs(30),

pub trait ExternalPrimalConnection: Send + Sync + std::fmt::Debug {

    async fn send_request(&self, request: Vec<u8>) -> BearDogResult<Vec<u8>>;

    async fn health_check(&self) -> BearDogResult<bool>;

    fn get_connection_info(&self) -> ConnectionInfo;

    async fn close(&self) -> BearDogResult<()>;

    fn is_alive(&self) -> bool;

pub struct ConnectionInfo {

    pub endpoint: String,

    pub protocol: String,

    pub established_at: SystemTime,

    pub last_activity: SystemTime,

    pub stats: ConnectionStats,

pub struct ConnectionStats {

    pub bytes_sent: u64,

    pub bytes_received: u64,

    pub errors: u32,

pub struct TarpcPrimalConnection {
    connection_id: Uuid,
    endpoint: String,
    established_at: SystemTime,
    stats: Arc<RwLock<ConnectionStats>>,
    status: Arc<RwLock<ConnectionStatus>>,
    last_activity: Arc<RwLock<SystemTime>>,}

impl TarpcPrimalConnection {

    pub fn new(endpoint: &str) -> Self {
            connection_id: Uuid::new_v4(),
            endpoint,
            established_at: SystemTime::now(),
            stats: Arc::new(RwLock::new(ConnectionStats::default())),
            status: Arc::new(RwLock::new(ConnectionStatus::Active)),
            last_activity: Arc::new(RwLock::new(SystemTime::now())),
impl ExternalPrimalConnection for TarpcPrimalConnection {}

    async fn send_request(&self, request: Vec<u8>) -> BearDogResult<Vec<u8>> {
        debug!("📤 Sending request to {} ({} bytes)", self.endpoint, request.len());

        {
            let mut last_activity = self.last_activity.write().await;
            *last_activity = SystemTime::now();

            let mut stats = self.stats.write().await;
            stats.requests_sent += 1;
            stats.bytes_sent += request.len() as u64;

        let response = vec![0u8; 32]; // Placeholder response

            stats.responses_received += 1;
            stats.bytes_received += response.len() as u64;
        debug!("📥 Received response from {} ({} bytes)", self.endpoint, response.len());
        Ok(response)
    async fn health_check(&self) -> BearDogResult<bool> {
        debug!("🏥 Health checking connection to {}", self.endpoint);

        let is_healthy = true; // Placeholder
        if is_healthy {
            let mut status = self.status.write().await;
            *status = ConnectionStatus::Active;
        } else {
            *status = ConnectionStatus::Unhealthy;
        Ok(is_healthy)
    fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            connection_id: self.connection_id,
            endpoint: self.endpoint.clone(),
            protocol: "tarpc".to_string(),
            established_at: self.established_at,
            last_activity: SystemTime::now(), // Would get actual last activity
            stats: ConnectionStats::default(), // Would get actual stats}

    async fn close(&self) -> BearDogResult<()> {
        info!("🔌 Closing connection to {}", self.endpoint);
            *status = ConnectionStatus::Closed;

        Ok(())
    fn is_alive(&self) -> bool {

        true // Placeholder
impl ExternalPrimalClient {

    pub fn new(config: ClientConfig) -> Self {
        info!("🔗 Initializing External Primal Client");
            config,
            connection_pool: Arc::new(RwLock::new(ConnectionPool {
                connections: HashMap::with_capacity(16),
                stats: PoolStats::default(),
                config: PoolConfig::default(),
            })),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            active_connections: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            health_monitor: Arc::new(Mutex::new(HealthMonitor {
                health_results: HashMap::with_capacity(16),
                last_check: Instant::now(),
                config: HealthCheckConfig::default(),

    pub async fn get_connection(&self, endpoint: &str) -> BearDogResult<Arc<dyn ExternalPrimalConnection>> {
        debug!("🔍 Getting connection to {}", endpoint);

            let active = self.active_connections.read().await;
            if let Some(connection) = active.get(endpoint) {
                if connection.is_alive() {
                    debug!("♻️ Reusing existing connection to {}", endpoint);
                    return Ok(connection.clone());
                }
            }

        let connection = Arc::new(TarpcPrimalConnection::new(endpoint.to_string()));

            let mut active = self.active_connections.write().await;
            active.insert(endpoint.to_string(), connection.clone());
        info!("🆕 Created new connection to {}", endpoint);
        Ok(connection)

    pub async fn send_request(&self, endpoint: &str, request: Vec<u8>) -> BearDogResult<Vec<u8>> {
        debug!("📨 Sending request to {} with retry logic", endpoint);
        let mut attempts = 0;
        let mut last_error = None;
        while attempts < self.config.max_retry_attempts {
            attempts += 1;
            
            match self.try_send_request(endpoint, &request).await {
                Ok(response) => {

                    {
                        let mut stats = self.stats.write().await;
                        stats.requests_sent += 1;
                        stats.responses_received += 1;
                        stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
                        stats.last_updated = SystemTime::now();
                    }
                    
                    return Ok(response);
                Err(e) => {
                    warn!("⚠️ Request attempt {} failed: {}", attempts, e);
                    last_error = Some(e);
                    if attempts < self.config.max_retry_attempts {
                        tokio::time::sleep(self.config.retry_delay).await;

            stats.errors_encountered += 1;
            stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
            stats.last_updated = SystemTime::now();
        Err(last_error.unwrap_or_else(|| BearDogError::internal("All retry attempts failed")))

    async fn try_send_request(&self, endpoint: &str, request: &[u8]) -> BearDogResult<Vec<u8>> {
        let connection = self.get_connection(endpoint).await?;
        connection.send_request(request.to_vec()).await

    pub async fn health_check(&self) -> BearDogResult<HashMap<String, bool>> {
        debug!("🏥 Performing health check on all connections");
        let mut results = HashMap::with_capacity(16);
        let active = self.active_connections.read().await;
        for (endpoint, connection) in active.iter() {
            match connection.health_check().await {
                Ok(healthy) => {
                    results.insert(endpoint.clone(), healthy);
                Err(_) => {
                    results.insert(endpoint.clone(), false);
        Ok(results)

    pub async fn get_statistics(&self) -> ClientStats {
        self.stats.read().await.clone()

    pub async fn cleanup_connections(&self) -> BearDogResult<u32> {
        debug!("🧹 Cleaning up inactive connections");
        let mut cleaned_count = 0;
        let mut active = self.active_connections.write().await;
        let mut to_remove = Vec::new();
            if !connection.is_alive() {
                to_remove.push(endpoint.clone());
        for endpoint in to_remove {
            if let Some(connection) = active.remove(&endpoint) {
                let _ = connection.close().await;
                cleaned_count += 1;
                debug!("🗑️ Cleaned up connection to {}", endpoint);
        info!("✅ Cleaned up {} inactive connections", cleaned_count);
        Ok(cleaned_count)
