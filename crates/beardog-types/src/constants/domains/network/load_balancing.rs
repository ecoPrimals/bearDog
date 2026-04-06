// SPDX-License-Identifier: AGPL-3.0-or-later

//! Load balancer algorithms, health paths, and session affinity.

use std::time::Duration;
/// Load balancing algorithms
pub const ROUND_ROBIN: &str = "round_robin";
/// Configuration constant: least connections
pub const LEAST_CONNECTIONS: &str = "least_connections";
/// Configuration constant: weighted round robin
pub const WEIGHTED_ROUND_ROBIN: &str = "weighted_round_robin";
/// Configuration constant: ip hash
pub const IP_HASH: &str = "ip_hash";
/// Configuration constant: random
pub const RANDOM: &str = "random";
/// Configuration constant: least response time
pub const LEAST_RESPONSE_TIME: &str = "least_response_time";

/// Health check settings
pub const DEFAULT_HEALTH_CHECK_PATH: &str = "/health";
/// Configuration constant: default health check method
pub const DEFAULT_HEALTH_CHECK_METHOD: &str = "GET";
/// Configuration constant: default health check interval
pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
/// Configuration constant: default health check timeout
pub const DEFAULT_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);

/// Failover settings
pub const DEFAULT_MAX_FAILS: u32 = 3;
/// Configuration constant: default fail timeout
pub const DEFAULT_FAIL_TIMEOUT: Duration = Duration::from_secs(60);
/// Configuration constant: default recovery time
pub const DEFAULT_RECOVERY_TIME: Duration = Duration::from_secs(30);

/// Session affinity
pub const SESSION_COOKIE_NAME: &str = "BEARDOG_SESSION";
/// Configuration constant: session header name
pub const SESSION_HEADER_NAME: &str = "X-BearDog-Session";
/// Configuration constant: default session timeout
pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600);
