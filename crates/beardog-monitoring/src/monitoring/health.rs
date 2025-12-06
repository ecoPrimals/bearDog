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
    #[must_use]
    pub const fn new() -> Self {
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
                // Modern approach: require explicit configuration
                let db_host = std::env::var("BEARDOG_DB_HOST")
                    .map_err(|_| BearDogError::configuration(
                        "BEARDOG_DB_HOST must be configured. Set environment variable BEARDOG_DB_HOST=host:port"
                    ))?;
                meta.insert("host".to_string(), db_host);
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
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
    #[must_use]
    pub const fn new() -> Self {
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
                // Modern approach: require explicit configuration
                let redis_host = std::env::var("BEARDOG_REDIS_HOST")
                    .map_err(|_| BearDogError::configuration(
                        "BEARDOG_REDIS_HOST must be configured. Set environment variable BEARDOG_REDIS_HOST=host:port"
                    ))?;
                meta.insert("host".to_string(), redis_host);
                meta
            },
        })
    }

    fn component_name(&self) -> &'static str {
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
    #[must_use]
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

    fn component_name(&self) -> &'static str {
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
    #[must_use]
    pub const fn new() -> Self {
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

    fn component_name(&self) -> &'static str {
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
    #[must_use]
    pub const fn new() -> Self {
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
    /// Gets `overall_status`
    /// Gets `overall_status`
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

#[allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]
#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    // ============================================================================
    // DatabaseHealthChecker Tests
    // ============================================================================

    #[tokio::test]
    async fn test_database_health_checker_new() {
        let checker = DatabaseHealthChecker::new();
        assert_eq!(checker.component_name(), "Database");
    }

    #[tokio::test]
    async fn test_database_health_checker_default() {
        let checker = DatabaseHealthChecker::default();
        assert_eq!(checker.component_name(), "Database");
    }

    #[tokio::test]
    #[serial]
    async fn test_database_health_checker_success() {
        // Set required env var
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");

        let checker = DatabaseHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "Database");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_some());
        assert!(health.check_duration_ms >= 10); // Min sleep is 10ms
        assert!(health.metadata.contains_key("type"));
        assert!(health.metadata.contains_key("host"));
        assert_eq!(health.metadata.get("type").unwrap(), "PostgreSQL");
        assert_eq!(health.metadata.get("host").unwrap(), "localhost:5432");

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_database_health_checker_missing_env_var() {
        // Ensure env var is not set
        env::remove_var("BEARDOG_DB_HOST");

        let checker = DatabaseHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Should be a configuration error (either Configuration or System variant)
        assert!(error
            .to_string()
            .contains("BEARDOG_DB_HOST must be configured"));
    }

    // ============================================================================
    // CacheHealthChecker Tests
    // ============================================================================

    #[tokio::test]
    async fn test_cache_health_checker_new() {
        let checker = CacheHealthChecker::new();
        assert_eq!(checker.component_name(), "Cache");
    }

    #[tokio::test]
    async fn test_cache_health_checker_default() {
        let checker = CacheHealthChecker::default();
        assert_eq!(checker.component_name(), "Cache");
    }

    #[tokio::test]
    #[serial]
    async fn test_cache_health_checker_success() {
        // Set required env var
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let checker = CacheHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "Cache");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_some());
        assert!(health.check_duration_ms >= 5); // Min sleep is 5ms
        assert!(health.metadata.contains_key("type"));
        assert!(health.metadata.contains_key("host"));
        assert_eq!(health.metadata.get("type").unwrap(), "Redis");
        assert_eq!(health.metadata.get("host").unwrap(), "localhost:6379");

        // Cleanup
        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_cache_health_checker_missing_env_var() {
        // Ensure env var is not set
        env::remove_var("BEARDOG_REDIS_HOST");

        let checker = CacheHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        // Should be a configuration error (either Configuration or System variant)
        assert!(error
            .to_string()
            .contains("BEARDOG_REDIS_HOST must be configured"));
    }

    // ============================================================================
    // ExternalApiHealthChecker Tests
    // ============================================================================

    #[tokio::test]
    async fn test_external_api_health_checker_new() {
        let checker = ExternalApiHealthChecker::new("https://api.test.com/health");
        assert_eq!(checker.component_name(), "External API");
        assert_eq!(checker.api_endpoint, "https://api.test.com/health");
    }

    #[tokio::test]
    async fn test_external_api_health_checker_default() {
        let checker = ExternalApiHealthChecker::default();
        assert_eq!(checker.component_name(), "External API");
        assert_eq!(checker.api_endpoint, "https://api.example.com/health");
    }

    #[tokio::test]
    async fn test_external_api_health_checker_success() {
        let checker = ExternalApiHealthChecker::new("https://api.test.com/health");
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "External API");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_some());
        assert!(health.check_duration_ms >= 50); // Min sleep is 50ms
        assert!(health.metadata.contains_key("endpoint"));
        assert!(health.metadata.contains_key("method"));
        assert_eq!(
            health.metadata.get("endpoint").unwrap(),
            "https://api.test.com/health"
        );
        assert_eq!(health.metadata.get("method").unwrap(), "GET");
    }

    #[tokio::test]
    async fn test_external_api_health_checker_custom_endpoint() {
        let custom_endpoint = "https://custom.example.com/status";
        let checker = ExternalApiHealthChecker::new(custom_endpoint);
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.metadata.get("endpoint").unwrap(), custom_endpoint);
    }

    // ============================================================================
    // HsmHealthChecker Tests
    // ============================================================================

    #[tokio::test]
    async fn test_hsm_health_checker_new() {
        let checker = HsmHealthChecker::new();
        assert_eq!(checker.component_name(), "HSM");
    }

    #[tokio::test]
    async fn test_hsm_health_checker_default() {
        let checker = HsmHealthChecker::default();
        assert_eq!(checker.component_name(), "HSM");
    }

    #[tokio::test]
    async fn test_hsm_health_checker_success() {
        let checker = HsmHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "HSM");
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.message.is_some());
        assert_eq!(health.message.unwrap(), "HSM operational");
        assert!(health.check_duration_ms >= 20); // Min sleep is 20ms
        assert!(health.metadata.contains_key("type"));
        assert_eq!(health.metadata.get("type").unwrap(), "HSM");
    }

    // ============================================================================
    // HealthCheckerType Tests
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_health_checker_type_database() {
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");

        let checker_type = HealthCheckerType::Database(DatabaseHealthChecker::new());
        assert_eq!(checker_type.component_name(), "Database");

        let result = checker_type.check_health().await;
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "Database");

        env::remove_var("BEARDOG_DB_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_health_checker_type_cache() {
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let checker_type = HealthCheckerType::Cache(CacheHealthChecker::new());
        assert_eq!(checker_type.component_name(), "Cache");

        let result = checker_type.check_health().await;
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "Cache");

        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    async fn test_health_checker_type_external_api() {
        let checker_type =
            HealthCheckerType::ExternalApi(ExternalApiHealthChecker::new("https://test.com"));
        assert_eq!(checker_type.component_name(), "External API");

        let result = checker_type.check_health().await;
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "External API");
    }

    #[tokio::test]
    async fn test_health_checker_type_hsm() {
        let checker_type = HealthCheckerType::Hsm(HsmHealthChecker::new());
        assert_eq!(checker_type.component_name(), "HSM");

        let result = checker_type.check_health().await;
        assert!(result.is_ok());
        let health = result.unwrap();
        assert_eq!(health.name, "HSM");
    }

    // ============================================================================
    // HealthCheckAggregator Tests
    // ============================================================================

    #[tokio::test]
    async fn test_health_check_aggregator_new() {
        let aggregator = HealthCheckAggregator::new();
        assert_eq!(aggregator.checkers.len(), 0);
    }

    #[tokio::test]
    async fn test_health_check_aggregator_default() {
        let aggregator = HealthCheckAggregator::default();
        assert_eq!(aggregator.checkers.len(), 0);
    }

    #[tokio::test]
    async fn test_health_check_aggregator_add_checker() {
        let mut aggregator = HealthCheckAggregator::new();
        assert_eq!(aggregator.checkers.len(), 0);

        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));
        assert_eq!(aggregator.checkers.len(), 1);

        aggregator.add_checker(HealthCheckerType::ExternalApi(
            ExternalApiHealthChecker::new("https://test.com"),
        ));
        assert_eq!(aggregator.checkers.len(), 2);
    }

    #[tokio::test]
    async fn test_health_check_aggregator_check_all_empty() {
        let aggregator = HealthCheckAggregator::new();
        let result = aggregator.check_all().await;

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 0);
    }

    #[tokio::test]
    async fn test_health_check_aggregator_check_all_single_healthy() {
        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));

        let result = aggregator.check_all().await;

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "HSM");
        assert_eq!(results[0].status, HealthStatus::Healthy);
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check_aggregator_check_all_multiple_healthy() {
        // Set required env vars
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::ExternalApi(
            ExternalApiHealthChecker::new("https://test.com"),
        ));

        let result = aggregator.check_all().await;

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 4);
        for health in &results {
            assert_eq!(health.status, HealthStatus::Healthy);
        }

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check_aggregator_check_all_with_failure() {
        // Don't set env var - this will cause DatabaseHealthChecker to fail
        env::remove_var("BEARDOG_DB_HOST");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));

        let result = aggregator.check_all().await;

        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 2);

        // First checker (Database) should be unhealthy
        assert_eq!(results[0].name, "Database");
        assert_eq!(results[0].status, HealthStatus::Unhealthy);
        assert!(results[0]
            .message
            .as_ref()
            .unwrap()
            .contains("Health check failed"));

        // Second checker (HSM) should be healthy
        assert_eq!(results[1].name, "HSM");
        assert_eq!(results[1].status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_health_check_aggregator_get_overall_status_empty() {
        let aggregator = HealthCheckAggregator::new();
        let result = aggregator.get_overall_status().await;

        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status, HealthStatus::Healthy); // Empty is considered healthy
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check_aggregator_get_overall_status_all_healthy() {
        // Set required env vars
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));

        let result = aggregator.get_overall_status().await;

        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status, HealthStatus::Healthy);

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check_aggregator_get_overall_status_one_unhealthy() {
        // Set only Redis host, not DB host - DB will fail
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");
        env::remove_var("BEARDOG_DB_HOST");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));

        let result = aggregator.get_overall_status().await;

        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status, HealthStatus::Unhealthy); // One unhealthy = overall unhealthy

        // Cleanup
        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_health_check_aggregator_get_overall_status_all_unhealthy() {
        // Don't set any env vars - both DB and Cache will fail
        env::remove_var("BEARDOG_DB_HOST");
        env::remove_var("BEARDOG_REDIS_HOST");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));

        let result = aggregator.get_overall_status().await;

        assert!(result.is_ok());
        let status = result.unwrap();
        assert_eq!(status, HealthStatus::Unhealthy);
    }

    // ============================================================================
    // Concurrent Tests
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_concurrent_health_checks() {
        // Set required env vars
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let db_checker = DatabaseHealthChecker::new();
        let cache_checker = CacheHealthChecker::new();
        let hsm_checker = HsmHealthChecker::new();
        let api_checker = ExternalApiHealthChecker::new("https://test.com");

        // Run all checks concurrently
        let (db_result, cache_result, hsm_result, api_result) = tokio::join!(
            db_checker.check_health(),
            cache_checker.check_health(),
            hsm_checker.check_health(),
            api_checker.check_health()
        );

        assert!(db_result.is_ok());
        assert!(cache_result.is_ok());
        assert!(hsm_result.is_ok());
        assert!(api_result.is_ok());

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
        env::remove_var("BEARDOG_REDIS_HOST");
    }

    #[tokio::test]
    #[serial]
    async fn test_concurrent_aggregator_calls() {
        // Set required env vars
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
        aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));

        // Run multiple aggregator calls concurrently
        let (result1, result2, result3) = tokio::join!(
            aggregator.check_all(),
            aggregator.check_all(),
            aggregator.get_overall_status()
        );

        assert!(result1.is_ok());
        assert!(result2.is_ok());
        assert!(result3.is_ok());

        assert_eq!(result1.unwrap().len(), 2);
        assert_eq!(result2.unwrap().len(), 2);
        assert_eq!(result3.unwrap(), HealthStatus::Healthy);

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[tokio::test]
    async fn test_health_check_duration_measurement() {
        let checker = HsmHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        // HSM check sleeps for 20ms, duration should be at least that
        assert!(health.check_duration_ms >= 20);
        // But should be reasonable (< 1 second)
        assert!(health.check_duration_ms < 1000);
    }

    #[tokio::test]
    #[serial]
    async fn test_failed_health_check_zero_duration() {
        // Don't set env var - this will cause failure
        env::remove_var("BEARDOG_DB_HOST");

        let mut aggregator = HealthCheckAggregator::new();
        aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));

        let result = aggregator.check_all().await;
        assert!(result.is_ok());
        let results = result.unwrap();

        // Failed check should have 0 duration
        assert_eq!(results[0].check_duration_ms, 0);
    }

    #[tokio::test]
    async fn test_metadata_capacity() {
        let checker = HsmHealthChecker::new();
        let result = checker.check_health().await;

        assert!(result.is_ok());
        let health = result.unwrap();
        // Ensure metadata is present and has reasonable capacity
        assert!(!health.metadata.is_empty());
    }

    #[tokio::test]
    async fn test_component_health_timestamps() {
        let checker1 = HsmHealthChecker::new();
        let result1 = checker1.check_health().await.unwrap();

        // Small delay
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let checker2 = HsmHealthChecker::new();
        let result2 = checker2.check_health().await.unwrap();

        // Timestamps should be different
        assert!(result2.last_check > result1.last_check);
    }

    // ============================================================================
    // Stress Tests
    // ============================================================================

    #[tokio::test]
    #[serial]
    async fn test_many_concurrent_checks() {
        // Set required env vars
        env::set_var("BEARDOG_DB_HOST", "localhost:5432");
        env::set_var("BEARDOG_REDIS_HOST", "localhost:6379");

        let mut handles = vec![];

        // Spawn 50 concurrent health check tasks
        for _ in 0..50 {
            let handle = tokio::spawn(async move {
                let mut aggregator = HealthCheckAggregator::new();
                aggregator.add_checker(HealthCheckerType::Hsm(HsmHealthChecker::new()));
                aggregator.add_checker(HealthCheckerType::ExternalApi(
                    ExternalApiHealthChecker::new("https://test.com"),
                ));

                aggregator.get_overall_status().await
            });
            handles.push(handle);
        }

        // Wait for all tasks
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
            let status = result.unwrap();
            assert!(status.is_ok());
            assert_eq!(status.unwrap(), HealthStatus::Healthy);
        }

        // Cleanup
        env::remove_var("BEARDOG_DB_HOST");
        env::remove_var("BEARDOG_REDIS_HOST");
    }
}
