// SPDX-License-Identifier: AGPL-3.0-or-later

//! Per-IP connection rate limiter for `BearDog` TCP server (H2-11 sovereignty).
//!
//! Protects direct-exposed endpoints from connection floods and request abuse
//! when operating without `Cloudflare` or any external `DDoS` mitigation.
//!
//! Uses a sliding-window token bucket per IP address. Stale entries are pruned
//! periodically to bound memory usage.

use dashmap::DashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Configuration for the connection rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum new connections per IP within the window.
    pub max_connections_per_window: u32,
    /// Sliding window duration.
    pub window: Duration,
    /// Maximum concurrent connections across all IPs.
    pub max_total_connections: u32,
    /// IPs that are never rate-limited (e.g. loopback, known primals).
    pub allowlist: Vec<IpAddr>,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_connections_per_window: 100,
            window: Duration::from_secs(60),
            max_total_connections: 1000,
            allowlist: vec![
                IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            ],
        }
    }
}

impl RateLimitConfig {
    /// Load config from environment variables with sane defaults.
    #[must_use]
    pub fn from_env() -> Self {
        let max_conn = std::env::var("BEARDOG_RATE_LIMIT_MAX_CONN")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);

        let window_secs = std::env::var("BEARDOG_RATE_LIMIT_WINDOW_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        let max_total = std::env::var("BEARDOG_RATE_LIMIT_MAX_TOTAL")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000);

        Self {
            max_connections_per_window: max_conn,
            window: Duration::from_secs(window_secs),
            max_total_connections: max_total,
            allowlist: vec![
                IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
                IpAddr::V6(std::net::Ipv6Addr::LOCALHOST),
            ],
        }
    }
}

/// Tracks connection timestamps for a single IP.
struct IpBucket {
    timestamps: Vec<Instant>,
}

/// Per-IP sliding-window rate limiter.
///
/// Thread-safe via `DashMap`. Each IP gets a bucket of connection timestamps;
/// expired timestamps are pruned on each check.
pub struct ConnectionRateLimiter {
    buckets: DashMap<IpAddr, IpBucket>,
    config: RateLimitConfig,
    total_active: Arc<std::sync::atomic::AtomicU32>,
}

impl ConnectionRateLimiter {
    /// Create a new rate limiter with the given config.
    #[must_use]
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: DashMap::new(),
            config,
            total_active: Arc::new(std::sync::atomic::AtomicU32::new(0)),
        }
    }

    /// Check whether a new connection from `ip` should be allowed.
    ///
    /// Returns `Ok(())` if allowed, `Err(reason)` if rejected.
    ///
    /// # Errors
    ///
    /// Returns a string describing why the connection was rejected.
    pub fn check_connection(&self, ip: &IpAddr) -> Result<(), String> {
        if self.config.allowlist.contains(ip) {
            return Ok(());
        }

        let total = self.total_active.load(std::sync::atomic::Ordering::Relaxed);
        if total >= self.config.max_total_connections {
            warn!(
                ip = %ip,
                total,
                max = self.config.max_total_connections,
                "rate limit: total connection limit reached"
            );
            return Err(format!(
                "server at connection capacity ({}/{})",
                total, self.config.max_total_connections
            ));
        }

        let now = Instant::now();
        let window_start = now.checked_sub(self.config.window).unwrap_or(now);

        let mut entry = self.buckets.entry(*ip).or_insert_with(|| IpBucket {
            timestamps: Vec::new(),
        });

        entry.timestamps.retain(|t| *t > window_start);

        #[expect(
            clippy::cast_possible_truncation,
            reason = "timestamps.len() bounded by max_connections_per_window (u32)"
        )]
        let count = entry.timestamps.len() as u32;

        if count >= self.config.max_connections_per_window {
            warn!(
                ip = %ip,
                count,
                max = self.config.max_connections_per_window,
                window_secs = self.config.window.as_secs(),
                "rate limit: per-IP connection limit exceeded"
            );
            return Err(format!(
                "too many connections from {} ({}/{})",
                ip, count, self.config.max_connections_per_window
            ));
        }

        entry.timestamps.push(now);
        Ok(())
    }

    /// Record that a connection was accepted (increment total counter).
    pub fn on_connect(&self) {
        self.total_active
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Record that a connection was closed (decrement total counter).
    pub fn on_disconnect(&self) {
        self.total_active
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    /// Get the current total active connection count.
    #[must_use]
    pub fn active_connections(&self) -> u32 {
        self.total_active.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Prune stale IP buckets that have no recent timestamps.
    ///
    /// Call periodically (e.g. every 5 minutes) to bound memory.
    pub fn prune_stale(&self) {
        let now = Instant::now();
        let window_start = now.checked_sub(self.config.window).unwrap_or(now);
        let before = self.buckets.len();

        self.buckets
            .retain(|_, bucket| bucket.timestamps.iter().any(|t| *t > window_start));

        let pruned = before - self.buckets.len();
        if pruned > 0 {
            debug!(
                pruned,
                remaining = self.buckets.len(),
                "rate limiter: pruned stale IP buckets"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RateLimitConfig {
        RateLimitConfig {
            max_connections_per_window: 3,
            window: Duration::from_secs(60),
            max_total_connections: 10,
            allowlist: vec![IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)],
        }
    }

    #[test]
    fn loopback_always_allowed() {
        let limiter = ConnectionRateLimiter::new(test_config());
        let ip = IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
        for _ in 0..100 {
            assert!(limiter.check_connection(&ip).is_ok());
        }
    }

    #[test]
    fn remote_ip_limited_after_threshold() {
        let limiter = ConnectionRateLimiter::new(test_config());
        let ip: IpAddr = "192.168.1.100".parse().expect("valid IP");

        assert!(limiter.check_connection(&ip).is_ok());
        assert!(limiter.check_connection(&ip).is_ok());
        assert!(limiter.check_connection(&ip).is_ok());
        assert!(limiter.check_connection(&ip).is_err());
    }

    #[test]
    fn different_ips_independent() {
        let limiter = ConnectionRateLimiter::new(test_config());
        let ip1: IpAddr = "10.0.0.1".parse().expect("valid IP");
        let ip2: IpAddr = "10.0.0.2".parse().expect("valid IP");

        for _ in 0..3 {
            assert!(limiter.check_connection(&ip1).is_ok());
            assert!(limiter.check_connection(&ip2).is_ok());
        }

        assert!(limiter.check_connection(&ip1).is_err());
        assert!(limiter.check_connection(&ip2).is_err());
    }

    #[test]
    fn total_connection_limit() {
        let config = RateLimitConfig {
            max_connections_per_window: 100,
            window: Duration::from_secs(60),
            max_total_connections: 2,
            allowlist: vec![],
        };
        let limiter = ConnectionRateLimiter::new(config);
        let ip: IpAddr = "10.0.0.1".parse().expect("valid IP");

        limiter.on_connect();
        limiter.on_connect();
        assert!(limiter.check_connection(&ip).is_err());

        limiter.on_disconnect();
        assert!(limiter.check_connection(&ip).is_ok());
    }

    #[test]
    fn connect_disconnect_tracking() {
        let limiter = ConnectionRateLimiter::new(test_config());
        assert_eq!(limiter.active_connections(), 0);
        limiter.on_connect();
        assert_eq!(limiter.active_connections(), 1);
        limiter.on_connect();
        assert_eq!(limiter.active_connections(), 2);
        limiter.on_disconnect();
        assert_eq!(limiter.active_connections(), 1);
    }

    #[test]
    fn prune_stale_removes_expired_entries() {
        let config = RateLimitConfig {
            max_connections_per_window: 100,
            window: Duration::from_millis(1),
            max_total_connections: 1000,
            allowlist: vec![],
        };
        let limiter = ConnectionRateLimiter::new(config);
        let ip: IpAddr = "10.0.0.1".parse().expect("valid IP");

        assert!(limiter.check_connection(&ip).is_ok());
        assert_eq!(limiter.buckets.len(), 1);

        std::thread::sleep(Duration::from_millis(10));
        limiter.prune_stale();
        assert_eq!(limiter.buckets.len(), 0);
    }

    #[test]
    fn default_config_has_loopback_allowlisted() {
        let config = RateLimitConfig::default();
        assert!(
            config
                .allowlist
                .contains(&IpAddr::V4(std::net::Ipv4Addr::LOCALHOST))
        );
        assert!(
            config
                .allowlist
                .contains(&IpAddr::V6(std::net::Ipv6Addr::LOCALHOST))
        );
    }
}
