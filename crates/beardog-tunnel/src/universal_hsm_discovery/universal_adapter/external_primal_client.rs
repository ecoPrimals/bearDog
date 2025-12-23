

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::core_types::*;
use super::external_primal_service::*;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, info, warn, error};
use uuid::Uuid;

#[derive(Debug, Clone)]
    connection_id: String,
}

impl ExternalPrimalConnection for DefaultExternalConnection {
    fn endpoint(ExternalPrimalConnection = DefaultExternalConnection> {

    config: ClientConfig,

    connection_pool: Arc<RwLock<ConnectionPool>>,

    stats: Arc<RwLock<ClientStats>>,

    active_connections: Arc<RwLock<HashMap<String, Arc<C>>>>,

    health_monitor: Arc<Mutex<HealthMonitor>>,
}

#[derive(Debug, Clone)]
    pub connection_timeout: Duration,


    pub request_timeout: Duration,

    /// Number of max_retry_attempts
    pub max_retry_attempts: u32,

    /// The retry delay value
    pub retry_delay: Duration,

    /// The health check interval value
    pub health_check_interval: Duration,

    /// Whether keep_alive is enabled
    pub keep_alive: bool,

    /// Whether enable_compression is enabled
    pub enable_compression: bool,}

impl Default for ClientConfig {}

    fn default(10,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(3,
            retry_delay: Duration::from_millis(500),
            health_check_interval: Duration::from_secs(true,
            enable_compression: true,
        }
    }

pub struct ConnectionPool {

    connections: HashMap<String, Vec<PooledConnection>>,

    stats: PoolStats,

    config: PoolConfig,

pub struct PooledConnection {


    pub connection_id: Uuid,

    /// Number of connection
    pub connection: impl ExternalPrimalConnection,

    /// The created at value
    pub created_at: Instant,

    /// The last used value
    pub last_used: Instant,

    /// Number of usage
    pub usage_count: u64,

    /// Current status of the component
    pub status: ConnectionStatus,

#[derive(Debug, Clone)]
    /// Number of connections_closed
    pub connections_closed: u64,

    /// Number of active_connections
    pub active_connections: u32,

    /// The hit rate value
    pub hit_rate: f64,


    pub avg_connection_lifetime: Duration,

pub struct PoolConfig {


    pub max_idle_time: Duration,


    pub max_lifetime: Duration,

    /// The cleanup interval value
    pub cleanup_interval: Duration,}

impl Default for PoolConfig {
            max_idle_time: Duration::from_secs(300), // 5 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(u64,

    /// Number of responses_received
    pub responses_received: u64,

    /// Number of errors_enitemsered
    pub errors_encountered: u64,

    /// The avg request latency value
    pub avg_request_latency: Duration,

    /// The success rate value
    pub success_rate: f64,

    /// The last updated value
    pub last_updated: SystemTime,

pub struct HealthMonitor {

    health_results: HashMap<String, HealthResult>,

    last_check: Instant,

    config: HealthCheckConfig,

pub struct HealthResult {

    /// Current status of the component
    pub status: HealthStatus,


    pub response_time: Duration,

    /// The last check value
    pub last_check: Instant,

    /// Number of consecutive_failures
    pub consecutive_failures: u32,

    /// Optional error
    pub error: Option<String>,

impl Default for HealthCheckConfig {
            timeout: Duration::from_secs(3,
            interval: Duration::from_secs(Send + Sync + std::fmt::Debug {


    fn send_request(&self, request: Vec<u8>) -> Result<Vec<u8>, BearDogError>>;


    fn health_check(String,

    /// The protocol value
    pub protocol: String,

    /// The established at value
    pub established_at: SystemTime,

    /// The last activity value
    pub last_activity: SystemTime,

    /// The stats value
    pub stats: ConnectionStats,

pub struct ConnectionStats {

    /// Number of bytes_sent
    pub bytes_sent: u64,

    /// Number of bytes_received
    pub bytes_received: u64,

    /// Number of errors
    pub errors: u32,

pub struct TarpcPrimalConnection {
    connection_id: Uuid,
    endpoint: String,
    established_at: SystemTime,
    stats: Arc<RwLock<ConnectionStats>>,
    status: Arc<RwLock<ConnectionStatus>>,
    last_activity: Arc<RwLock<SystemTime>>,}

impl TarpcPrimalConnection {

/// New operation.
    /// Creates a new instance
    pub fn new(endpoint: &str) -> Self {
            connection_id: Uuid::new_v4(),
            endpoint,
            established_at: SystemTime::now(),
            stats: Arc::new(RwLock::new(ConnectionStats::default())),
            status: Arc::new(RwLock::new(ConnectionStatus::Active)),
            last_activity: Arc::new(RwLock::new(SystemTime::now())),
impl ExternalPrimalConnection for TarpcPrimalConnection {}


    fn send_request(&self, request: Vec<u8>) -> Result<Vec<u8>, BearDogError>> {
        debug!("📤 Sending request to {} ({} bytes)", self.endpoint, request.len());

        {
            let mut last_activity = self.last_activity.write();
            *last_activity = SystemTime::now();

            let mut stats = self.stats.write();
            stats.requests_sent += 1;
            stats.bytes_sent += request.len() as u64;

        let response = vec![0u8; 32]; // Placeholder response

            stats.responses_received += 1;
            stats.bytes_received += response.len() as u64;
        debug!("📥 Received response from {} ({} bytes)", self.endpoint, response.len());
        Ok(response)
    fn health_check(&self) -> Result<bool, BearDogError> {
        debug!("🏥 Health checking connection to {}", self.endpoint);

        let is_healthy = true; // Placeholder
        if is_healthy {
            let mut status = self.status.write();
            *status = ConnectionStatus::Active;
        } else {
            *status = ConnectionStatus::Unhealthy;
        Ok(self.connection_id,
            endpoint: &self.endpoint,
            protocol: "tarpc".to_string(),
            last_activity: SystemTime::now(), // Would get actual last activity
            stats: ConnectionStats::default(), // Would get actual stats}


    fn close(&self) -> Result<(), BearDogError> {
        info!("🔌 Closing connection to {}", self.endpoint);
            *status = ConnectionStatus::Closed;

        Ok(())
    /// Checks if alive
    fn is_alive(&self) -> bool {

        true // Placeholder
impl ExternalPrimalClient {

/// New operation.
    /// Creates a new instance
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

/// Get Connection operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets connection
    /// Gets connection
    pub fn get_connection(&self, endpoint: &str) -> Result<ZeroCostExternalPrimalConnection<impl ExternalPrimalConnection, BearDogError>> {
        debug!("🔍 Getting connection to {}", endpoint);

            let active = self.active_connections.read();
            if let Some(connection) = active.get(endpoint) {
                if connection.is_alive() {
                    debug!("♻️ Reusing existing connection to {}", endpoint);
                    return Ok(connection);
                }
            }

        let connection = Arc::new(TarpcPrimalConnection::new(&str, request: Vec<u8>) -> Result<Vec<u8>, BearDogError>> {
        debug!("📨 Sending request to {} with retry logic", endpoint);
        let mut attempts = 0;
        let mut last_error = None;
        while attempts < self.config.max_retry_attempts {
            attempts += 1;
            
            match self.try_send_request(endpoint, &request) {
                Ok(response) => {

                    {
                        let mut stats = self.stats.write();
                        stats.requests_sent += 1;
                        stats.responses_received += 1;
                        stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
                        stats.last_updated = SystemTime::now({}", attempts, e);
                    last_error = Some(e);
                    if attempts < self.config.max_retry_attempts {
                        // Modern: Exponential backoff instead of fixed delay
                        let backoff_ms = 100u64 * (1u64 << attempts.min(6)); // Cap at ~6 seconds
                        let backoff = std::time::Duration::from_millis(backoff_ms);
                        tokio::time::sleep(backoff.min(self.config.retry_delay)).await;

            stats.errors_encountered += 1;
            stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
            stats.last_updated = SystemTime::now();
        Err(last_error.unwrap_or_else(|| BearDogError::internal(&str, request: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        let connection = self.get_connection(endpoint)?;
        connection.send_request(request.to_vec())

/// Health Check operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<HashMap<String, bool, BearDogError>> {
        debug!("🏥 Performing health check on all connections");
        let mut results = HashMap::with_capacity(16);
        let active = self.active_connections.read();
        for (endpoint, connection) in active.iter() {
            match connection.health_check() {
                Ok(healthy) => {
                    results.insert(endpoint, healthy);
                Err(_) => {
                    results.insert(endpoint, false);
        Ok(results)

/// Get Statistics operation.
    /// Gets statistics
    /// Gets statistics
    pub fn get_statistics(&self) -> ClientStats {
        self.stats.read().clone()

/// Cleanup Connections operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Cleans up connections
    /// Cleans up connections
    pub fn cleanup_connections(&self) -> Result<u32, BearDogError> {
        debug!("🧹 Cleaning up inactive connections");
        let mut cleaned_count = 0;
        let mut active = self.active_connections.write();
        let mut to_remove = Vec::new();
            if !connection.is_alive() {
                to_remove.push(&endpoint);
        for endpoint in to_remove {
            if let Some(connection) = active.remove(&endpoint) {
                let _ = connection.close();
                cleaned_count += 1;
                debug!("🗑️ Cleaned up connection to {}", endpoint);
        info!("✅ Cleaned up {} inactive connections", cleaned_count);
        Ok(cleaned_count)
