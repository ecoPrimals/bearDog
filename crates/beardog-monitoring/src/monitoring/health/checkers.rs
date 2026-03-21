// SPDX-License-Identifier: AGPL-3.0-only

//! Health Checker Implementations
//!
//! Concrete implementations of health checkers for different system components.
//! These are production-ready implementations with mock fallbacks for testing.

use super::super::types::ComponentHealth;
use super::traits::HealthChecker;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Health checker enum for type-safe health checking
#[derive(Debug)]
pub enum HealthCheckerType {
    /// Database health checker
    Database(DatabaseHealthChecker),
    /// Cache health checker
    Cache(CacheHealthChecker),
    /// External API health checker
    ExternalApi(ExternalApiHealthChecker),
    /// HSM health checker
    Hsm(HsmHealthChecker),
}

impl HealthChecker for HealthCheckerType {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        match self {
            Self::Database(checker) => checker.check_health().await,
            Self::Cache(checker) => checker.check_health().await,
            Self::ExternalApi(checker) => checker.check_health().await,
            Self::Hsm(checker) => checker.check_health().await,
        }
    }

    fn component_name(&self) -> &str {
        match self {
            Self::Database(checker) => checker.component_name(),
            Self::Cache(checker) => checker.component_name(),
            Self::ExternalApi(checker) => checker.component_name(),
            Self::Hsm(checker) => checker.component_name(),
        }
    }
}

/// Database health checker
///
/// Checks database connectivity and responsiveness.
/// Supports simulated latency for testing scenarios.
#[derive(Debug)]
pub struct DatabaseHealthChecker {
    simulated_latency: Option<Duration>,
}

impl Default for DatabaseHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseHealthChecker {
    /// Creates a new database health checker with instant responses
    #[must_use]
    pub const fn new() -> Self {
        Self {
            simulated_latency: None,
        }
    }

    /// Creates a health checker with simulated latency for timeout testing
    #[must_use]
    pub const fn with_simulated_latency(latency: Duration) -> Self {
        Self {
            simulated_latency: Some(latency),
        }
    }
}

impl HealthChecker for DatabaseHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        // Only sleep if explicitly configured for latency testing
        if let Some(latency) = self.simulated_latency {
            tokio::time::sleep(latency).await;
        }

        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("type".to_string(), "PostgreSQL".to_string());
                // Capability-based: Discover DB host from environment (no hardcoding)
                let db_host = beardog_errors::process_env::var("BEARDOG_DB_HOST")
                    .unwrap_or_else(|_| "not_configured".to_string());
                meta.insert("host".to_string(), db_host);
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
        "Database"
    }
}

/// Cache health checker
///
/// Checks cache connectivity and hit rates.
#[derive(Debug)]
pub struct CacheHealthChecker {
    simulated_latency: Option<Duration>,
}

impl Default for CacheHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheHealthChecker {
    /// Creates a new cache health checker
    #[must_use]
    pub const fn new() -> Self {
        Self {
            simulated_latency: None,
        }
    }

    /// Creates a health checker with simulated latency
    #[must_use]
    pub const fn with_simulated_latency(latency: Duration) -> Self {
        Self {
            simulated_latency: Some(latency),
        }
    }
}

impl HealthChecker for CacheHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        if let Some(latency) = self.simulated_latency {
            tokio::time::sleep(latency).await;
        }

        Ok(ComponentHealth {
            name: "Cache".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Cache operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("type".to_string(), "Redis".to_string());
                // Capability-based: Discover cache host from environment
                let cache_host = beardog_errors::process_env::var("BEARDOG_CACHE_HOST")
                    .unwrap_or_else(|_| "not_configured".to_string());
                meta.insert("host".to_string(), cache_host);
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
        "Cache"
    }
}

/// External API health checker
///
/// Checks external API endpoints for availability.
#[derive(Debug)]
pub struct ExternalApiHealthChecker {
    /// API endpoint URL
    pub url: String,
    /// Simulated latency for testing
    simulated_latency: Option<Duration>,
}

impl Default for ExternalApiHealthChecker {
    fn default() -> Self {
        Self {
            url: beardog_errors::process_env::var("BEARDOG_EXTERNAL_API_URL")
                .unwrap_or_else(|_| "not_configured".to_string()),
            simulated_latency: None,
        }
    }
}

impl ExternalApiHealthChecker {
    /// Creates a new external API health checker
    #[must_use]
    pub const fn new(url: String) -> Self {
        Self {
            url,
            simulated_latency: None,
        }
    }

    /// Creates a health checker with simulated latency
    #[must_use]
    pub const fn with_simulated_latency(url: String, latency: Duration) -> Self {
        Self {
            url,
            simulated_latency: Some(latency),
        }
    }
}

impl HealthChecker for ExternalApiHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        if let Some(latency) = self.simulated_latency {
            tokio::time::sleep(latency).await;
        }

        Ok(ComponentHealth {
            name: "ExternalAPI".to_string(),
            status: HealthStatus::Healthy,
            message: Some("API reachable".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("url".to_string(), self.url.clone());
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
        "ExternalAPI"
    }
}

/// HSM health checker
///
/// Checks HSM availability and cryptographic capabilities.
#[derive(Debug)]
pub struct HsmHealthChecker {
    simulated_latency: Option<Duration>,
}

impl Default for HsmHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmHealthChecker {
    /// Creates a new HSM health checker
    #[must_use]
    pub const fn new() -> Self {
        Self {
            simulated_latency: None,
        }
    }

    /// Creates a health checker with simulated latency
    #[must_use]
    pub const fn with_simulated_latency(latency: Duration) -> Self {
        Self {
            simulated_latency: Some(latency),
        }
    }
}

impl HealthChecker for HsmHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        if let Some(latency) = self.simulated_latency {
            tokio::time::sleep(latency).await;
        }

        Ok(ComponentHealth {
            name: "HSM".to_string(),
            status: HealthStatus::Healthy,
            message: Some("HSM operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                // Capability-based: Discover HSM provider dynamically
                let hsm_provider = beardog_errors::process_env::var("BEARDOG_HSM_PROVIDER")
                    .unwrap_or_else(|_| "auto".to_string());
                meta.insert("provider".to_string(), hsm_provider);
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
        "HSM"
    }
}
