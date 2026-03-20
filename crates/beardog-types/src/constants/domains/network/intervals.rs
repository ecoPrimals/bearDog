// SPDX-License-Identifier: AGPL-3.0-only

//! Periodic intervals for health, metrics, and keep-alive.

use std::time::Duration;

/// Connection management intervals
pub const CONNECTION_CHECK_INTERVAL: Duration = Duration::from_secs(30);
/// Configuration constant: idle connection cleanup
pub const IDLE_CONNECTION_CLEANUP: Duration = Duration::from_secs(60);
/// Configuration constant: connection pool cleanup
pub const CONNECTION_POOL_CLEANUP: Duration = Duration::from_secs(300);

/// Health check intervals
pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
/// Configuration constant: ping interval
pub const PING_INTERVAL: Duration = Duration::from_secs(60);
/// Configuration constant: heartbeat interval
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);

/// Retry intervals
pub const RETRY_INTERVAL: Duration = Duration::from_millis(100);
/// Configuration constant: exponential backoff base
pub const EXPONENTIAL_BACKOFF_BASE: Duration = Duration::from_millis(100);
/// Configuration constant: max retry interval
pub const MAX_RETRY_INTERVAL: Duration = Duration::from_secs(30);

/// Metrics collection intervals
pub const NETWORK_METRICS_INTERVAL: Duration = Duration::from_secs(15);
/// Configuration constant: bandwidth metrics interval
pub const BANDWIDTH_METRICS_INTERVAL: Duration = Duration::from_secs(5);
/// Configuration constant: connection metrics interval
pub const CONNECTION_METRICS_INTERVAL: Duration = Duration::from_secs(10);

/// Keep-alive intervals
pub const TCP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(60);
/// Configuration constant: http keepalive interval
pub const HTTP_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(120);
/// Configuration constant: websocket ping interval
pub const WEBSOCKET_PING_INTERVAL: Duration = Duration::from_secs(30);
