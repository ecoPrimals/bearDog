// SPDX-License-Identifier: AGPL-3.0-only

//! # Network Performance Configuration Module
//!
//! This module contains network performance and optimization configurations.

use serde::{Deserialize, Serialize};

/// Network performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceConfiguration {
    /// Enable performance optimizations
    pub enable_optimizations: bool,
    /// Buffer size optimizations
    pub optimize_buffer_sizes: bool,
    /// Connection reuse
    pub enable_connection_reuse: bool,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfiguration {
    /// Enable caching
    pub enabled: bool,
    /// Cache size in MB
    pub size_mb: usize,
    /// Cache TTL seconds
    pub ttl_seconds: u64,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRateLimitConfiguration {
    /// Enable rate limiting
    pub enabled: bool,
    /// Global requests per second
    pub global_rps: Option<u64>,
    /// Per-IP requests per minute
    pub per_ip_rpm: Option<u64>,
    /// Burst size
    pub burst_size: u64,
}

impl Default for NetworkPerformanceConfiguration {
    fn default() -> Self {
        Self {
            enable_optimizations: true,
            optimize_buffer_sizes: true,
            enable_connection_reuse: true,
        }
    }
}

impl Default for CacheConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            size_mb: 128,
            ttl_seconds: 3600,
        }
    }
}

impl Default for NetworkRateLimitConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            global_rps: Some(1000),
            per_ip_rpm: Some(60),
            burst_size: 100,
        }
    }
}
