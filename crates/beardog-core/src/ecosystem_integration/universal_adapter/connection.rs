// Connection Management for Universal Adapter

use super::types::ConnectionInfo;
use beardog_types::canonical::config::domains::network::ConnectionPoolConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Connection pool
#[derive(Debug)]
pub struct ConnectionPool {
    /// Pool configuration
    config: ConnectionPoolConfig,
    /// Active connections
    connections: Arc<RwLock<HashMap<String, ConnectionInfo>>>,
}

impl ConnectionPool {
    /// Create a new connection pool
    /// Creates a new instance
    #[must_use]
    pub fn new(config: ConnectionPoolConfig) -> Self {
        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get pool statistics
    pub async fn stats(&self) -> PoolStats {
        let connections = self.connections.read().await;
        PoolStats {
            total_connections: u32::try_from(connections.len()).unwrap_or(u32::MAX),
            active_connections: u32::try_from(connections.len()).unwrap_or(u32::MAX),
            idle_connections: 0,
            max_connections: u32::try_from(self.config.max_size).unwrap_or(u32::MAX),
        }
    }
}

/// Pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolStats {
    /// Number of `total_connections`
    pub total_connections: u32,
    /// Number of `active_connections`
    pub active_connections: u32,
    /// Number of idle connections in the pool
    pub idle_connections: u32,
    /// Number of `max_connections`
    pub max_connections: u32,
}
