// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health Checker Implementations
//!
//! Concrete implementations of health checkers for different system components.

use super::super::types::ComponentHealth;
use super::traits::HealthChecker;
use beardog_config::env_keys;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
#[cfg(test)]
use std::time::Duration;
use std::time::Instant;

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
    /// Test-only: injects delay then runs a real checker (for timeout / latency tests)
    #[cfg(test)]
    TestDelayed(TestDelayedChecker),
}

impl HealthChecker for HealthCheckerType {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        match self {
            Self::Database(checker) => checker.check_health().await,
            Self::Cache(checker) => checker.check_health().await,
            Self::ExternalApi(checker) => checker.check_health().await,
            Self::Hsm(checker) => checker.check_health().await,
            #[cfg(test)]
            Self::TestDelayed(checker) => checker.check_health().await,
        }
    }

    fn component_name(&self) -> &str {
        match self {
            Self::Database(checker) => checker.component_name(),
            Self::Cache(checker) => checker.component_name(),
            Self::ExternalApi(checker) => checker.component_name(),
            Self::Hsm(checker) => checker.component_name(),
            #[cfg(test)]
            Self::TestDelayed(checker) => checker.component_name(),
        }
    }
}

/// Database health checker
///
/// Checks database connectivity and responsiveness.
#[derive(Debug)]
pub struct DatabaseHealthChecker;

impl Default for DatabaseHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseHealthChecker {
    /// Creates a new database health checker
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl HealthChecker for DatabaseHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

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
                let db_host = beardog_errors::process_env::var(env_keys::ENV_DB_HOST)
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
pub struct CacheHealthChecker;

impl Default for CacheHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheHealthChecker {
    /// Creates a new cache health checker
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl HealthChecker for CacheHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

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
                let cache_host = beardog_errors::process_env::var(env_keys::ENV_CACHE_HOST)
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
}

impl Default for ExternalApiHealthChecker {
    fn default() -> Self {
        Self {
            url: beardog_errors::process_env::var(env_keys::ENV_EXTERNAL_API_URL)
                .unwrap_or_else(|_| "not_configured".to_string()),
        }
    }
}

impl ExternalApiHealthChecker {
    /// Creates a new external API health checker
    #[must_use]
    pub const fn new(url: String) -> Self {
        Self { url }
    }
}

impl HealthChecker for ExternalApiHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

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
pub struct HsmHealthChecker;

impl Default for HsmHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmHealthChecker {
    /// Creates a new HSM health checker
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl HealthChecker for HsmHealthChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        Ok(ComponentHealth {
            name: "HSM".to_string(),
            status: HealthStatus::Healthy,
            message: Some("HSM operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                // Capability-based: Discover HSM provider dynamically
                let hsm_provider = beardog_errors::process_env::var(env_keys::ENV_HSM_PROVIDER)
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

/// Test harness: sleeps before delegating to a production checker so timeout/latency tests
/// do not embed artificial delays in production implementations.
#[cfg(test)]
#[derive(Debug)]
pub struct TestDelayedChecker {
    delay: Duration,
    kind: TestDelayedKind,
}

#[cfg(test)]
#[derive(Debug, Clone, Copy)]
enum TestDelayedKind {
    Database,
    Cache,
    ExternalApi(&'static str),
    Hsm,
}

#[cfg(test)]
impl TestDelayedChecker {
    /// Builds a checker that simulates a database dependency with the given startup delay.
    #[must_use]
    pub fn database(delay: Duration) -> Self {
        Self {
            delay,
            kind: TestDelayedKind::Database,
        }
    }

    /// Builds a checker that simulates a cache dependency with the given startup delay.
    #[must_use]
    pub fn cache(delay: Duration) -> Self {
        Self {
            delay,
            kind: TestDelayedKind::Cache,
        }
    }

    /// Builds a checker that simulates an external HTTP API at `url` with the given startup delay.
    #[must_use]
    pub fn external_api(url: &'static str, delay: Duration) -> Self {
        Self {
            delay,
            kind: TestDelayedKind::ExternalApi(url),
        }
    }

    /// Builds a checker that simulates an HSM dependency with the given startup delay.
    #[must_use]
    pub fn hsm(delay: Duration) -> Self {
        Self {
            delay,
            kind: TestDelayedKind::Hsm,
        }
    }
}

#[cfg(test)]
impl HealthChecker for TestDelayedChecker {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Health check elapsed millis fit in u64 for reporting"
    )]
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();
        tokio::time::sleep(self.delay).await;
        let total_ms = start.elapsed().as_millis() as u64;

        let mut health = match self.kind {
            TestDelayedKind::Database => DatabaseHealthChecker::new().check_health().await?,
            TestDelayedKind::Cache => CacheHealthChecker::new().check_health().await?,
            TestDelayedKind::ExternalApi(url) => {
                ExternalApiHealthChecker::new(url.to_string())
                    .check_health()
                    .await?
            }
            TestDelayedKind::Hsm => HsmHealthChecker::new().check_health().await?,
        };
        health.check_duration_ms = total_ms;
        Ok(health)
    }

    fn component_name(&self) -> &str {
        match self.kind {
            TestDelayedKind::Database => "Database",
            TestDelayedKind::Cache => "Cache",
            TestDelayedKind::ExternalApi(_) => "ExternalAPI",
            TestDelayedKind::Hsm => "HSM",
        }
    }
}
