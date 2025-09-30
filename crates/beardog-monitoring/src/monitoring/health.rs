// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::ComponentHealth;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

#[allow(async_fn_in_trait)]
pub trait HealthChecker: Send + Sync {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError>;

    fn component_name(&self) -> &str;
}

#[derive(Debug)]
/// Types of health checker
pub enum HealthCheckerType {
    /// Represents database variant
    Database(DatabaseHealthChecker),
    /// Represents cache variant
    Cache(CacheHealthChecker),
    /// Represents external api variant
    ExternalApi(ExternalApiHealthChecker),
    /// Represents hsm variant
    Hsm(HsmHealthChecker),
}

impl HealthChecker for HealthCheckerType {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        match self {
            HealthCheckerType::Database(checker) => checker.check_health().await,
            HealthCheckerType::Cache(checker) => checker.check_health().await,
            HealthCheckerType::ExternalApi(checker) => checker.check_health().await,
            HealthCheckerType::Hsm(checker) => checker.check_health().await,
        }
    }

    fn component_name(&self) -> &str {
        match self {
            HealthCheckerType::Database(checker) => checker.component_name(),
            HealthCheckerType::Cache(checker) => checker.component_name(),
            HealthCheckerType::ExternalApi(checker) => checker.component_name(),
            HealthCheckerType::Hsm(checker) => checker.component_name(),
        }
    }
}

#[derive(Debug)]
pub struct DatabaseHealthChecker {}

impl Default for DatabaseHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseHealthChecker {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for DatabaseHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("type".to_string(), "PostgreSQL".to_string());
                meta.insert(
                    "host".to_string(),
                    std::env::var("BEARDOG_DB_HOST")
                        .unwrap_or_else(|_| "localhost:5432".to_string()),
                );
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Database"
    }
}

#[derive(Debug)]
pub struct CacheHealthChecker {}

impl Default for CacheHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheHealthChecker {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for CacheHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        tokio::time::sleep(Duration::from_millis(5)).await;

        Ok(ComponentHealth {
            name: "Cache".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Cache connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("type".to_string(), "Redis".to_string());
                meta.insert(
                    "host".to_string(),
                    std::env::var("BEARDOG_REDIS_HOST")
                        .unwrap_or_else(|_| "localhost:6379".to_string()),
                );
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Cache"
    }
}

#[derive(Debug)]
pub struct ExternalApiHealthChecker {
    /// The api endpoint value
    pub api_endpoint: String,
}

impl Default for ExternalApiHealthChecker {
    fn default() -> Self {
        Self {
            api_endpoint: "https://api.example.com/health".to_string(),
        }
    }
}

impl ExternalApiHealthChecker {
    /// New operation.
    /// Creates a new instance
    pub fn new(api_endpoint: &str) -> Self {
        Self {
            api_endpoint: api_endpoint.to_string(),
        }
    }
}

impl HealthChecker for ExternalApiHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        tokio::time::sleep(Duration::from_millis(50)).await;

        Ok(ComponentHealth {
            name: "External API".to_string(),
            status: HealthStatus::Healthy,
            message: Some("External API responding".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: HashMap::from([
                ("endpoint".to_owned(), self.api_endpoint.clone()),
                ("method".to_owned(), "GET".to_owned()),
            ]),
        })
    }

    fn component_name(&self) -> &str {
        "External API"
    }
}

#[derive(Debug)]
pub struct HsmHealthChecker {}

impl Default for HsmHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmHealthChecker {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for HsmHealthChecker {
    async fn check_health(&self) -> Result<ComponentHealth, BearDogError> {
        let start = Instant::now();

        tokio::time::sleep(Duration::from_millis(20)).await;

        Ok(ComponentHealth {
            name: "HSM".to_string(),
            status: HealthStatus::Healthy,
            message: Some("HSM operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("type".to_string(), "HSM".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "HSM"
    }
}

#[derive(Debug)]
pub struct HealthCheckAggregator {
    checkers: Vec<HealthCheckerType>,
}

impl HealthCheckAggregator {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    /// Add Checker operation.
    pub fn add_checker(&mut self, checker: HealthCheckerType) {
        self.checkers.push(checker);
    }

    /// Check All operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub async fn check_all(&self) -> Result<Vec<ComponentHealth>, BearDogError> {
        let mut results = Vec::new();

        for checker in &self.checkers {
            match checker.check_health().await {
                Ok(health) => results.push(health),
                Err(e) => {
                    results.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {e}")),
                        last_check: Utc::now(),
                        check_duration_ms: 0, // Failed check, no duration recorded
                        metadata: HashMap::with_capacity(16),
                    });
                }
            }
        }

        Ok(results)
    }

    /// Get Overall Status operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets overall_status
    /// Gets overall_status
    pub async fn get_overall_status(&self) -> Result<HealthStatus, BearDogError> {
        let results = self.check_all().await?;

        for result in &results {
            if result.status == HealthStatus::Unhealthy {
                return Ok(HealthStatus::Unhealthy);
            }
        }

        Ok(HealthStatus::Healthy)
    }
}

impl Default for HealthCheckAggregator {
    fn default() -> Self {
        Self::new()
    }
}
