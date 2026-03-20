// SPDX-License-Identifier: AGPL-3.0-only

//! Connection, I/O, and DNS timeout constants.

use std::time::Duration;

/// Connection timeouts
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: handshake timeout
pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
/// Configuration constant: tls handshake timeout
pub const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: keep alive timeout
pub const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(60);
/// Configuration constant: idle connection timeout
pub const IDLE_CONNECTION_TIMEOUT: Duration = Duration::from_secs(300);

/// I/O timeouts
pub const READ_TIMEOUT: Duration = Duration::from_secs(60);
/// Configuration constant: write timeout
pub const WRITE_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: send timeout
pub const SEND_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: receive timeout
pub const RECEIVE_TIMEOUT: Duration = Duration::from_secs(60);

/// Request/Response timeouts
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: response timeout
pub const RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: http request timeout
pub const HTTP_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: http response timeout
pub const HTTP_RESPONSE_TIMEOUT: Duration = Duration::from_secs(30);

/// DNS and resolution timeouts
pub const DNS_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(5);
/// Configuration constant: hostname resolution timeout
pub const HOSTNAME_RESOLUTION_TIMEOUT: Duration = Duration::from_secs(10);

/// Retry and backoff timeouts
pub const RETRY_TIMEOUT: Duration = Duration::from_millis(100);
/// Configuration constant: max retry timeout
pub const MAX_RETRY_TIMEOUT: Duration = Duration::from_secs(30);
/// Configuration constant: backoff timeout
pub const BACKOFF_TIMEOUT: Duration = Duration::from_millis(500);

/// Health check timeouts
pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
/// Configuration constant: ping timeout
pub const PING_TIMEOUT: Duration = Duration::from_secs(1);
/// Configuration constant: heartbeat timeout
pub const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
