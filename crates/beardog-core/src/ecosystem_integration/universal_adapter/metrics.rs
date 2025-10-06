// Metrics for Universal Adapter System

use serde::{Deserialize, Serialize};

/// Adapter metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterMetrics {
    /// Total number of requests processed
    /// Number of `total_requests`
    pub total_requests: u64,
    /// Total number of successful requests
    /// Number of `successful_requests`
    pub successful_requests: u64,
    /// Total number of failed requests
    /// Number of `failed_requests`
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Connection metrics
    /// The connection metrics value
    pub connection_metrics: ConnectionMetrics,
}

impl Default for AdapterMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_response_time_ms: 0.0,
            connection_metrics: ConnectionMetrics::default(),
        }
    }
}

/// Connection metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionMetrics {
    /// Total number of connections established
    /// Number of `total_connections`
    pub total_connections: u64,
    /// Number of active connections
    /// Number of `active_connections`
    pub active_connections: u32,
    /// Number of failed connections
    /// Number of `failed_connections`
    pub failed_connections: u64,
    /// Average connection establishment time in milliseconds
    pub avg_connection_time_ms: f64,
}

impl Default for ConnectionMetrics {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            failed_connections: 0,
            avg_connection_time_ms: 0.0,
        }
    }
}
