// SPDX-License-Identifier: AGPL-3.0-or-later

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool_new() {
        let config = ConnectionPoolConfig::default();
        let pool = ConnectionPool::new(config);
        assert!(std::mem::size_of_val(&pool) > 0);
    }

    #[tokio::test]
    async fn test_connection_pool_stats() {
        let config = ConnectionPoolConfig::default();
        let pool = ConnectionPool::new(config);
        let stats = pool.stats().await;
        assert_eq!(stats.total_connections, 0);
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.idle_connections, 0);
    }

    #[test]
    fn test_pool_stats_serialization() {
        let stats = PoolStats {
            total_connections: 10,
            active_connections: 5,
            idle_connections: 5,
            max_connections: 20,
        };
        let json = serde_json::to_string(&stats).unwrap();
        let decoded: PoolStats = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.total_connections, 10);
    }
}
