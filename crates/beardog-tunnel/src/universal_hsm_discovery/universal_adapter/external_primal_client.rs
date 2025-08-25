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


/// External Primal Client
///
/// **CANONICAL EXTERNAL PRIMAL CLIENT** - Complete client manager for external primal network communication
/// This module provides comprehensive client management for communicating with external primals
/// in the ecosystem, including connection pooling, retry logic, and health monitoring.

use super::core_types::*;
use super::external_primal_service::*;
use async_trait::async_trait;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::{RwLock, Mutex};
use tracing::{debug, info, warn, error};
use uuid::Uuid;
/// **CANONICAL EXTERNAL PRIMAL CLIENT** - Main client for external primal communication
#[derive(Debug)]
pub struct ExternalPrimalClient {
    /// Client configuration
    config: ClientConfig,
    
    /// Connection pool
    connection_pool: Arc<RwLock<ConnectionPool>>,
    /// Client statistics
    stats: Arc<RwLock<ClientStats>>,
    /// Active connections
    active_connections: Arc<RwLock<HashMap<String, Arc<dyn ExternalPrimalConnection>>>>,
    /// Connection health monitor
    health_monitor: Arc<Mutex<HealthMonitor>>,
}
/// **CLIENT CONFIGURATION**
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Maximum connections per primal
    pub max_connections_per_primal: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Retry attempts
    pub max_retry_attempts: u32,
    /// Retry delay
    pub retry_delay: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Connection keep-alive
    pub keep_alive: bool,
    /// Enable compression
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
/// **CONNECTION POOL** - Manages connections to external primals
pub struct ConnectionPool {
    /// Pooled connections by primal endpoint
    connections: HashMap<String, Vec<PooledConnection>>,
    /// Pool statistics
    stats: PoolStats,
    /// Pool configuration
    config: PoolConfig,
/// **POOLED CONNECTION** - Connection wrapper with metadata
pub struct PooledConnection {
    /// Unique connection identifier
    pub connection_id: Uuid,
    /// Connection instance
    pub connection: Arc<dyn ExternalPrimalConnection>,
    /// Connection creation time
    pub created_at: Instant,
    /// Last used time
    pub last_used: Instant,
    /// Connection usage count
    pub usage_count: u64,
    /// Connection status
    pub status: ConnectionStatus,
/// **POOL STATISTICS**
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    /// Total connections created
    pub connections_created: u64,
    /// Total connections closed
    pub connections_closed: u64,
    /// Active connections count
    pub active_connections: u32,
    /// Pool hit rate
    pub hit_rate: f64,
    /// Average connection lifetime
    pub avg_connection_lifetime: Duration,
/// **POOL CONFIGURATION**
pub struct PoolConfig {
    /// Maximum idle time before connection cleanup
    pub max_idle_time: Duration,
    /// Maximum connection lifetime
    pub max_lifetime: Duration,
    /// Pool cleanup interval
    pub cleanup_interval: Duration,}


impl Default for PoolConfig {
            max_idle_time: Duration::from_secs(300), // 5 minutes
            max_lifetime: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(60), // 1 minute
/// **CONNECTION STATUS**}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// Connection is active and healthy
    Active,
    /// Connection is idle
    Idle,
    /// Connection is unhealthy
    Unhealthy,
    /// Connection is being closed
    Closing,
    /// Connection is closed
    Closed,
/// **CLIENT STATISTICS**}


pub struct ClientStats {
    /// Total requests sent
    pub requests_sent: u64,
    /// Total responses received
    pub responses_received: u64,
    /// Total errors encountered
    pub errors_encountered: u64,
    /// Average request latency
    pub avg_request_latency: Duration,
    /// Success rate
    pub success_rate: f64,
    /// Last updated timestamp
    pub last_updated: SystemTime,
/// **HEALTH MONITOR** - Monitors connection health
pub struct HealthMonitor {
    /// Health check results
    health_results: HashMap<String, HealthResult>,
    /// Last health check time
    last_check: Instant,
    /// Health check configuration
    config: HealthCheckConfig,
/// **HEALTH RESULT**
pub struct HealthResult {
    /// Health status
    pub status: HealthStatus,
    /// Response time
    pub response_time: Duration,
    /// Last check timestamp
    pub last_check: Instant,
    /// Consecutive failures
    pub consecutive_failures: u32,
    /// Error message (if any)
    pub error: Option<String>,
/// **HEALTH CHECK CONFIGURATION**
pub struct HealthCheckConfig {
    /// Health check timeout
    pub timeout: Duration,
    /// Maximum consecutive failures before marking unhealthy
    pub max_failures: u32,
    pub interval: Duration,}


impl Default for HealthCheckConfig {
            timeout: Duration::from_secs(10),
            max_failures: 3,
            interval: Duration::from_secs(30),
/// **CANONICAL EXTERNAL PRIMAL CONNECTION TRAIT** - Interface for primal connections}



pub trait ExternalPrimalConnection: Send + Sync + std::fmt::Debug {
    /// Send request to external primal
    async fn send_request(&self, request: Vec<u8>) -> BearDogResult<Vec<u8>>;
    /// Perform health check on connection
    async fn health_check(&self) -> BearDogResult<bool>;
    /// Get connection information
    fn get_connection_info(&self) -> ConnectionInfo;
    /// Close the connection
    async fn close(&self) -> BearDogResult<()>;
    /// Check if connection is alive
    fn is_alive(&self) -> bool;
/// **CONNECTION INFO**
pub struct ConnectionInfo {
    /// Connection identifier
    /// Remote endpoint
    pub endpoint: String,
    /// Connection protocol
    pub protocol: String,
    /// Connection established time
    pub established_at: SystemTime,
    /// Last activity time
    pub last_activity: SystemTime,
    /// Connection statistics
    pub stats: ConnectionStats,
/// **CONNECTION STATISTICS**
pub struct ConnectionStats {
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Requests sent
    /// Responses received
    /// Connection errors
    pub errors: u32,
/// **TARPC-BASED EXTERNAL PRIMAL CONNECTION** - Tarpc implementation
pub struct TarpcPrimalConnection {
    connection_id: Uuid,
    endpoint: String,
    established_at: SystemTime,
    stats: Arc<RwLock<ConnectionStats>>,
    status: Arc<RwLock<ConnectionStatus>>,
    last_activity: Arc<RwLock<SystemTime>>,}


impl TarpcPrimalConnection {
    /// Create new Tarpc connection}


    pub fn new(endpoint: String) -> Self {
            connection_id: Uuid::new_v4(),
            endpoint,
            established_at: SystemTime::now(),
            stats: Arc::new(RwLock::new(ConnectionStats::default())),
            status: Arc::new(RwLock::new(ConnectionStatus::Active)),
            last_activity: Arc::new(RwLock::new(SystemTime::now())),
impl ExternalPrimalConnection for TarpcPrimalConnection {}


    async fn send_request(&self, request: Vec<u8>) -> BearDogResult<Vec<u8>> {
        debug!("📤 Sending request to {} ({} bytes)", self.endpoint, request.len());
        
        // Update last activity
        {
            let mut last_activity = self.last_activity.write().await;
            *last_activity = SystemTime::now();
        // Update statistics
            let mut stats = self.stats.write().await;
            stats.requests_sent += 1;
            stats.bytes_sent += request.len() as u64;
        // Implementation would use actual Tarpc client
        // For now, simulate response
        let response = vec![0u8; 32]; // Placeholder response
        // Update response statistics
            stats.responses_received += 1;
            stats.bytes_received += response.len() as u64;
        debug!("📥 Received response from {} ({} bytes)", self.endpoint, response.len());
        Ok(response)
    async fn health_check(&self) -> BearDogResult<bool> {
        debug!("🏥 Health checking connection to {}", self.endpoint);
        // Implementation would perform actual health check
        // For now, simulate health check
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
        // Implementation would close actual Tarpc connection
        Ok(())
    fn is_alive(&self) -> bool {
        // Implementation would check actual connection status
        true // Placeholder
impl ExternalPrimalClient {
    /// **CREATE NEW CLIENT**}


    pub fn new(config: ClientConfig) -> Self {
        info!("🔗 Initializing External Primal Client");
            config,
            connection_pool: Arc::new(RwLock::new(ConnectionPool {
                connections: HashMap::new(),
                stats: PoolStats::default(),
                config: PoolConfig::default(),
            })),
            stats: Arc::new(RwLock::new(ClientStats::default())),
            active_connections: Arc::new(RwLock::new(HashMap::new())),
            health_monitor: Arc::new(Mutex::new(HealthMonitor {
                health_results: HashMap::new(),
                last_check: Instant::now(),
                config: HealthCheckConfig::default(),
    /// **GET CONNECTION** - Get or create connection to primal
    pub async fn get_connection(&self, endpoint: &str) -> BearDogResult<Arc<dyn ExternalPrimalConnection>> {
        debug!("🔍 Getting connection to {}", endpoint);
        // Check if we have an active connection
            let active = self.active_connections.read().await;
            if let Some(connection) = active.get(endpoint) {
                if connection.is_alive() {
                    debug!("♻️ Reusing existing connection to {}", endpoint);
                    return Ok(connection.clone());
                }
            }
        // Create new connection
        let connection = Arc::new(TarpcPrimalConnection::new(endpoint.to_string()));
        // Store in active connections
            let mut active = self.active_connections.write().await;
            active.insert(endpoint.to_string(), connection.clone());
        info!("🆕 Created new connection to {}", endpoint);
        Ok(connection)
    /// **SEND REQUEST** - Send request with retry logic
    pub async fn send_request(&self, endpoint: &str, request: Vec<u8>) -> BearDogResult<Vec<u8>> {
        debug!("📨 Sending request to {} with retry logic", endpoint);
        let mut attempts = 0;
        let mut last_error = None;
        while attempts < self.config.max_retry_attempts {
            attempts += 1;
            
            match self.try_send_request(endpoint, &request).await {
                Ok(response) => {
                    // Update success statistics
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
        // Update error statistics
            stats.errors_encountered += 1;
            stats.success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
            stats.last_updated = SystemTime::now();
        Err(last_error.unwrap_or_else(|| BearDogError::internal("All retry attempts failed")))
    /// **TRY SEND REQUEST** - Single request attempt
    async fn try_send_request(&self, endpoint: &str, request: &[u8]) -> BearDogResult<Vec<u8>> {
        let connection = self.get_connection(endpoint).await?;
        connection.send_request(request.to_vec()).await
    /// **HEALTH CHECK** - Check health of all connections}


    pub async fn health_check(&self) -> BearDogResult<HashMap<String, bool>> {
        debug!("🏥 Performing health check on all connections");
        let mut results = HashMap::new();
        let active = self.active_connections.read().await;
        for (endpoint, connection) in active.iter() {
            match connection.health_check().await {
                Ok(healthy) => {
                    results.insert(endpoint.clone(), healthy);
                Err(_) => {
                    results.insert(endpoint.clone(), false);
        Ok(results)
    /// **GET STATISTICS** - Get client statistics
    pub async fn get_statistics(&self) -> ClientStats {
        self.stats.read().await.clone()
    /// **CLEANUP CONNECTIONS** - Clean up inactive connections}


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
