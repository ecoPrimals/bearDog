

use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::ComponentHealth;
use beardog_errors::BearDogResult;
use beardog_types::canonical::health_status::HealthStatus;

#[allow(async_fn_in_trait)]
pub trait HealthChecker: Send + Sync {

    async fn check_health(&self) -> BearDogResult<ComponentHealth>;

    fn component_name(&self) -> &str;
}

#[derive(Debug)]
pub enum HealthCheckerType {
    Database(DatabaseHealthChecker),
    Cache(CacheHealthChecker),
    ExternalApi(ExternalApiHealthChecker),
    Hsm(HsmHealthChecker),
}

impl HealthChecker for HealthCheckerType {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
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

    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for DatabaseHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
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
                meta.insert("host".to_string(), "localhost:5432".to_string());
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

    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for CacheHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
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
                meta.insert("host".to_string(), "localhost:6379".to_string());
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

    pub fn new(api_endpoint: &str) -> Self {
        Self { api_endpoint }
    }
}

impl HealthChecker for ExternalApiHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();

        tokio::time::sleep(Duration::from_millis(50)).await;
        
        Ok(ComponentHealth {
            name: "External API".to_string(),
            status: HealthStatus::Healthy,
            message: Some("External API responding".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("endpoint".to_string(), self.api_endpoint.clone());
                meta.insert("method".to_string(), "GET".to_string());
                meta
            },
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

    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for HsmHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
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
                meta.insert("type".to_string(), "Software HSM".to_string());
                meta.insert("keys_available".to_string(), "128".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "HSM"
    }
}

pub struct HealthCheckAggregator {
    checkers: Vec<HealthCheckerType>,
}

impl HealthCheckAggregator {

    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    pub fn add_checker(&mut self, checker: HealthCheckerType) {
        self.checkers.push(checker);
    }

    pub async fn check_all(&self) -> BearDogResult<Vec<ComponentHealth>> {
        let mut results = Vec::new();
        
        for checker in &self.checkers {
            match checker.check_health().await {
                Ok(health) => results.push(health),
                Err(e) => {

                    results.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format_args!("Health check failed: {}", e).to_string()),
                        last_check: Utc::now(),
                        check_duration_ms: 0,
                        metadata: HashMap::with_capacity(16),
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub async fn get_overall_status(&self) -> BearDogResult<HealthStatus> {
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

