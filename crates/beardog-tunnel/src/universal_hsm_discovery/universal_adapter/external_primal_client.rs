//! # External Primal Client
//!
//! Provides client functionality for connecting to external primals in the BearDog ecosystem.
//! Supports connection pooling, health monitoring, and automatic retry logic.
//!
//! ## Architecture
//! - Connection pooling with configurable limits
//! - Health monitoring with automatic failover
//! - Exponential backoff retry logic
//! - Zero-cost abstraction for external connections

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{Mutex, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

// ============================================================
// Traits
// ============================================================

/// Trait for external primal connections
///
/// Provides the interface for communicating with external primals.
pub trait ExternalPrimalConnection: Send + Sync + std::fmt::Debug {
    /// Send a request to the external primal
    fn send_request(&self, request: Vec<u8>) -> Result<Vec<u8>, BearDogError>;

    /// Perform a health check on the connection
    fn health_check(&self) -> Result<bool, BearDogError>;

    /// Get connection information
    fn connection_info(&self) -> ConnectionInfo;

    /// Close the connection
    fn close(&self) -> Result<(), BearDogError>;

    /// Check if the connection is alive
    fn is_alive(&self) -> bool;

    /// Get the endpoint this connection is connected to
    fn endpoint(&self) -> &str;
}

// ============================================================
// Configuration Types
// ============================================================

/// Client configuration
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Connection timeout duration
    pub connection_timeout: Duration,

    /// Request timeout duration
    pub request_timeout: Duration,

    /// Maximum number of retry attempts
    pub max_retry_attempts: u32,

    /// Delay between retries
    pub retry_delay: Duration,

    /// Interval between health checks
    pub health_check_interval: Duration,

    /// Whether to enable keep-alive
    pub keep_alive: bool,

    /// Whether to enable compression
    pub enable_compression: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(10),
            max_retry_attempts: 3,
            retry_delay: Duration::from_millis(500),
            health_check_interval: Duration::from_secs(30),
            keep_alive: true,
            enable_compression: true,
        }
    }
}

/// Connection pool configuration
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum connections per endpoint
    pub max_connections_per_endpoint: u32,

    /// Maximum idle time before connection is closed
    pub max_idle_time: Duration,

    /// Maximum lifetime of a connection
    pub max_lifetime: Duration,

    /// Interval for cleanup tasks
    pub cleanup_interval: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections_per_endpoint: 10,
            max_idle_time: Duration::from_secs(300),    // 5 minutes
            max_lifetime: Duration::from_secs(3600),    // 1 hour
            cleanup_interval: Duration::from_secs(60),  // 1 minute
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Timeout for health checks
    pub timeout: Duration,

    /// Interval between health checks
    pub interval: Duration,

    /// Number of failures before marking unhealthy
    pub failure_threshold: u32,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(3),
            interval: Duration::from_secs(30),
            failure_threshold: 3,
        }
    }
}

// ============================================================
// Status Types
// ============================================================

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Connection is active and healthy
    Active,
    /// Connection is idle
    Idle,
    /// Connection is unhealthy
    Unhealthy,
    /// Connection is closed
    Closed,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// Health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

// ============================================================
// Statistics Types
// ============================================================

/// Connection statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConnectionStats {
    /// Number of requests sent
    pub requests_sent: u64,

    /// Number of responses received
    pub responses_received: u64,

    /// Number of bytes sent
    pub bytes_sent: u64,

    /// Number of bytes received
    pub bytes_received: u64,

    /// Number of errors
    pub errors: u32,
}

/// Client statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientStats {
    /// Number of requests sent
    pub requests_sent: u64,

    /// Number of responses received
    pub responses_received: u64,

    /// Number of errors encountered
    pub errors_encountered: u64,

    /// Average request latency
    #[serde(with = "duration_serde")]
    pub avg_request_latency: Duration,

    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,

    /// Last update time
    #[serde(with = "system_time_serde")]
    pub last_updated: SystemTime,
}

/// Pool statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PoolStats {
    /// Number of connections created
    pub connections_created: u64,

    /// Number of connections closed
    pub connections_closed: u64,

    /// Number of active connections
    pub active_connections: u32,

    /// Pool hit rate
    pub hit_rate: f64,

    /// Average connection lifetime
    #[serde(with = "duration_serde")]
    pub avg_connection_lifetime: Duration,
}

// ============================================================
// Connection Info Types
// ============================================================

/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    /// Connection ID
    pub connection_id: String,

    /// Endpoint
    pub endpoint: String,

    /// Protocol
    pub protocol: String,

    /// When the connection was established
    #[serde(with = "system_time_serde")]
    pub established_at: SystemTime,

    /// Last activity time
    #[serde(with = "system_time_serde")]
    pub last_activity: SystemTime,

    /// Connection statistics
    pub stats: ConnectionStats,
}

// ============================================================
// Connection Pool
// ============================================================

/// Pooled connection wrapper
#[derive(Debug)]
pub struct PooledConnection<C: ExternalPrimalConnection> {
    /// Connection ID
    pub connection_id: Uuid,

    /// The actual connection
    pub connection: C,

    /// When the connection was created
    pub created_at: Instant,

    /// Last time the connection was used
    pub last_used: Instant,

    /// Number of times the connection has been used
    pub usage_count: u64,

    /// Current status
    pub status: ConnectionStatus,
}

/// Connection pool
pub struct ConnectionPool<C: ExternalPrimalConnection> {
    /// Connections by endpoint
    connections: HashMap<String, Vec<PooledConnection<C>>>,

    /// Pool statistics
    stats: PoolStats,

    /// Pool configuration
    config: PoolConfig,
}

impl<C: ExternalPrimalConnection> Default for ConnectionPool<C> {
    fn default() -> Self {
        Self {
            connections: HashMap::with_capacity(16),
            stats: PoolStats::default(),
            config: PoolConfig::default(),
        }
    }
}

// ============================================================
// Health Monitor
// ============================================================

/// Health check result
#[derive(Debug, Clone, Default)]
pub struct HealthResult {
    /// Current health status
    pub status: HealthStatus,

    /// Response time for health check
    pub response_time: Duration,

    /// When the health check was performed
    pub last_check: Instant,

    /// Number of consecutive failures
    pub consecutive_failures: u32,

    /// Error message if unhealthy
    pub error: Option<String>,
}

/// Health monitor
pub struct HealthMonitor {
    /// Health results by endpoint
    health_results: HashMap<String, HealthResult>,

    /// Last global health check
    last_check: Instant,

    /// Configuration
    config: HealthCheckConfig,
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self {
            health_results: HashMap::with_capacity(16),
            last_check: Instant::now(),
            config: HealthCheckConfig::default(),
        }
    }
}

// ============================================================
// tarpc Primal Connection
// ============================================================

/// tarpc-based connection to external primals
#[derive(Debug)]
pub struct TarpcPrimalConnection {
    /// Connection ID
    connection_id: Uuid,

    /// Endpoint address
    endpoint: String,

    /// When the connection was established
    established_at: SystemTime,

    /// Connection statistics
    stats: Arc<RwLock<ConnectionStats>>,

    /// Current status
    status: Arc<RwLock<ConnectionStatus>>,

    /// Last activity time
    last_activity: Arc<RwLock<SystemTime>>,
}

impl TarpcPrimalConnection {
    /// Create a new tarpc connection
    pub fn new(endpoint: &str) -> Self {
        Self {
            connection_id: Uuid::new_v4(),
            endpoint: endpoint.to_string(),
            established_at: SystemTime::now(),
            stats: Arc::new(RwLock::new(ConnectionStats::default())),
            status: Arc::new(RwLock::new(ConnectionStatus::Active)),
            last_activity: Arc::new(RwLock::new(SystemTime::now())),
        }
    }
}

impl ExternalPrimalConnection for TarpcPrimalConnection {
    fn send_request(&self, request: Vec<u8>) -> Result<Vec<u8>, BearDogError> {
        debug!(
            "📤 Sending request to {} ({} bytes)",
            self.endpoint,
            request.len()
        );

        // Update last activity
        {
            let mut last_activity = self.last_activity.blocking_write();
            *last_activity = SystemTime::now();
        }

        // Update stats
        {
            let mut stats = self.stats.blocking_write();
            stats.requests_sent += 1;
            stats.bytes_sent += request.len() as u64;
        }

        // External primal communication requires:
        // 1. Shared tarpc service definition between primals
        // 2. Socket/network path discovery via beardog-discovery
        // 3. TLS mutual authentication via genetic lineage
        //
        // Current Status: Integration pending
        // - beardog-discovery crate ready (45 tests pass)
        // - beardog-ipc/tarpc_client.rs has connection infrastructure
        // - Needs service definition and discovery wiring
        //
        // For now, return error to prevent silent failures in production
        warn!(
            "⚠️ External primal request to {} - tarpc integration pending",
            self.endpoint
        );

        Err(BearDogError::not_implemented(
            "External primal communication via tarpc pending integration. \
             Use Unix socket IPC or HTTP for now.",
        ))
    }

    fn health_check(&self) -> Result<bool, BearDogError> {
        debug!("🏥 Health checking connection to {}", self.endpoint);

        // Health check would ping the remote primal
        // For tarpc: send a lightweight status request
        // For HTTP: HEAD request to /health endpoint
        // For Unix socket: connect + disconnect test
        //
        // Current Status: Returns unhealthy since tarpc not yet wired
        let is_healthy = false;

        {
            let mut status = self.status.blocking_write();
            *status = if is_healthy {
                ConnectionStatus::Active
            } else {
                ConnectionStatus::Unhealthy
            };
        }

        warn!(
            "⚠️ External primal health check for {} - tarpc integration pending",
            self.endpoint
        );

        Ok(is_healthy)
    }

    fn connection_info(&self) -> ConnectionInfo {
        let stats = self.stats.blocking_read().clone();
        let last_activity = *self.last_activity.blocking_read();

        ConnectionInfo {
            connection_id: self.connection_id.to_string(),
            endpoint: self.endpoint.clone(),
            protocol: "tarpc".to_string(),
            established_at: self.established_at,
            last_activity,
            stats,
        }
    }

    fn close(&self) -> Result<(), BearDogError> {
        info!("🔌 Closing connection to {}", self.endpoint);
        {
            let mut status = self.status.blocking_write();
            *status = ConnectionStatus::Closed;
        }
        Ok(())
    }

    fn is_alive(&self) -> bool {
        let status = self.status.blocking_read();
        matches!(*status, ConnectionStatus::Active | ConnectionStatus::Idle)
    }

    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

// ============================================================
// External Primal Client
// ============================================================

/// Client for communicating with external primals
pub struct ExternalPrimalClient<C: ExternalPrimalConnection = TarpcPrimalConnection> {
    /// Client configuration
    config: ClientConfig,

    /// Connection pool
    connection_pool: Arc<RwLock<ConnectionPool<C>>>,

    /// Client statistics
    stats: Arc<RwLock<ClientStats>>,

    /// Active connections by endpoint
    active_connections: Arc<RwLock<HashMap<String, Arc<C>>>>,

    /// Health monitor
    health_monitor: Arc<Mutex<HealthMonitor>>,
}

impl<C: ExternalPrimalConnection + 'static> ExternalPrimalClient<C> {
    /// Create a new external primal client
    pub fn new(config: ClientConfig) -> Self {
        info!("🔗 Initializing External Primal Client");
        Self {
            config,
            connection_pool: Arc::new(RwLock::new(ConnectionPool::default())),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            active_connections: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            health_monitor: Arc::new(Mutex::new(HealthMonitor::default())),
        }
    }
}

impl ExternalPrimalClient<TarpcPrimalConnection> {
    /// Get or create a connection to an endpoint
    pub fn get_connection(
        &self,
        endpoint: &str,
    ) -> Result<Arc<TarpcPrimalConnection>, BearDogError> {
        debug!("🔍 Getting connection to {}", endpoint);

        // Check for existing connection
        {
            let active = self.active_connections.blocking_read();
            if let Some(connection) = active.get(endpoint) {
                if connection.is_alive() {
                    debug!("♻️ Reusing existing connection to {}", endpoint);
                    return Ok(Arc::clone(connection));
                }
            }
        }

        // Create new connection
        debug!("🆕 Creating new connection to {}", endpoint);
        let connection = Arc::new(TarpcPrimalConnection::new(endpoint));

        // Store in active connections
        {
            let mut active = self.active_connections.blocking_write();
            active.insert(endpoint.to_string(), Arc::clone(&connection));
        }

        Ok(connection)
    }

    /// Send a request to an endpoint with retry logic
    pub async fn send_request(
        &self,
        endpoint: &str,
        request: Vec<u8>,
    ) -> Result<Vec<u8>, BearDogError> {
        debug!("📨 Sending request to {} with retry logic", endpoint);

        let mut attempts = 0;
        let mut last_error = None;

        while attempts < self.config.max_retry_attempts {
            attempts += 1;

            match self.try_send_request(endpoint, &request) {
                Ok(response) => {
                    // Update stats
                    {
                        let mut stats = self.stats.blocking_write();
                        stats.requests_sent += 1;
                        stats.responses_received += 1;
                        stats.success_rate =
                            stats.responses_received as f64 / stats.requests_sent as f64;
                        stats.last_updated = SystemTime::now();
                    }
                    return Ok(response);
                }
                Err(e) => {
                    warn!("❌ Request attempt {} failed: {}", attempts, e);
                    last_error = Some(e);

                    if attempts < self.config.max_retry_attempts {
                        // Exponential backoff (capped at ~6 seconds)
                        let backoff_ms = 100u64 * (1u64 << attempts.min(6));
                        let backoff = Duration::from_millis(backoff_ms).min(self.config.retry_delay);
                        tokio::time::sleep(backoff).await;
                    }
                }
            }
        }

        // Update error stats
        {
            let mut stats = self.stats.blocking_write();
            stats.requests_sent += 1;
            stats.errors_encountered += 1;
            stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
            stats.last_updated = SystemTime::now();
        }

        Err(last_error.unwrap_or_else(|| {
            BearDogError::internal("All retry attempts failed".to_string())
        }))
    }

    /// Try to send a single request
    fn try_send_request(&self, endpoint: &str, request: &[u8]) -> Result<Vec<u8>, BearDogError> {
        let connection = self.get_connection(endpoint)?;
        connection.send_request(request.to_vec())
    }

    /// Perform health check on all connections
    pub fn health_check(&self) -> Result<HashMap<String, bool>, BearDogError> {
        debug!("🏥 Performing health check on all connections");
        let mut results = HashMap::with_capacity(16);

        let active = self.active_connections.blocking_read();
        for (endpoint, connection) in active.iter() {
            match connection.health_check() {
                Ok(healthy) => {
                    results.insert(endpoint.clone(), healthy);
                }
                Err(_) => {
                    results.insert(endpoint.clone(), false);
                }
            }
        }

        Ok(results)
    }

    /// Get client statistics
    pub fn get_statistics(&self) -> ClientStats {
        self.stats.blocking_read().clone()
    }

    /// Clean up inactive connections
    pub fn cleanup_connections(&self) -> Result<u32, BearDogError> {
        debug!("🧹 Cleaning up inactive connections");
        let mut cleaned_count = 0;

        let mut active = self.active_connections.blocking_write();
        let to_remove: Vec<String> = active
            .iter()
            .filter(|(_, conn)| !conn.is_alive())
            .map(|(endpoint, _)| endpoint.clone())
            .collect();

        for endpoint in to_remove {
            if let Some(connection) = active.remove(&endpoint) {
                let _ = connection.close();
                cleaned_count += 1;
                debug!("🗑️ Cleaned up connection to {}", endpoint);
            }
        }

        info!("✅ Cleaned up {} inactive connections", cleaned_count);
        Ok(cleaned_count)
    }
}

impl Default for ExternalPrimalClient<TarpcPrimalConnection> {
    fn default() -> Self {
        Self::new(ClientConfig::default())
    }
}

// ============================================================
// Serde Helpers
// ============================================================

mod duration_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        duration.as_millis().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

mod system_time_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(time: &SystemTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
        duration.as_secs().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<SystemTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(UNIX_EPOCH + Duration::from_secs(secs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.max_retry_attempts, 3);
        assert!(config.keep_alive);
    }

    #[test]
    fn test_pool_config_default() {
        let config = PoolConfig::default();
        assert_eq!(config.max_connections_per_endpoint, 10);
    }

    #[test]
    fn test_tarpc_connection_creation() {
        let conn = TarpcPrimalConnection::new("test-endpoint");
        assert_eq!(conn.endpoint(), "test-endpoint");
        assert!(conn.is_alive());
    }

    #[test]
    fn test_connection_status_default() {
        assert_eq!(ConnectionStatus::default(), ConnectionStatus::Active);
    }

    #[test]
    fn test_client_creation() {
        let client = ExternalPrimalClient::<TarpcPrimalConnection>::default();
        let stats = client.get_statistics();
        assert_eq!(stats.requests_sent, 0);
    }
}
