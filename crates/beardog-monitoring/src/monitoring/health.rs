use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::{ComponentHealth, HealthStatus};
use beardog_errors::BearDogResult;

/// Health check trait for components to implement
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    /// Perform a health check on this component
    async fn check_health(&self) -> BearDogResult<ComponentHealth>;
    /// Get the name of this component
    fn component_name(&self) -> &str;
}

/// Database health checker
pub struct DatabaseHealthChecker {
    // Add database connection pool or client here
}

impl Default for DatabaseHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseHealthChecker {
    /// Create a new database health checker
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for DatabaseHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();

        // In a real implementation, you'd ping the database
        // For now, simulate a health check
        tokio::time::sleep(Duration::from_millis(10)).await;

        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "PostgreSQL".to_string());
                meta.insert("version".to_string(), "14.5".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Database"
    }
}

/// Redis health checker
pub struct RedisHealthChecker {
    // Add Redis connection pool or client here
}

impl Default for RedisHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl RedisHealthChecker {
    /// Create a new Redis health checker
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for RedisHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();

        // In a real implementation, you'd ping Redis
        tokio::time::sleep(Duration::from_millis(5)).await;

        Ok(ComponentHealth {
            name: "Redis".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Redis connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "Redis".to_string());
                meta.insert("version".to_string(), "7.0".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Redis"
    }
}

/// External service health checker for monitoring external dependencies
pub struct ExternalServiceHealthChecker {
    /// Name of the external service being monitored
    pub name: String,
    /// Endpoint URL for health check requests
    pub endpoint: String,
    /// HTTP client for making health check requests
    pub client: reqwest::Client,
}

impl ExternalServiceHealthChecker {
    /// Create a new external service health checker
    pub fn new(name: String, endpoint: String) -> Self {
        Self {
            name,
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HealthChecker for ExternalServiceHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();

        match self
            .client
            .get(&self.endpoint)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                };

                Ok(ComponentHealth {
                    name: self.name.clone(),
                    status,
                    message: Some(format!("HTTP {}", response.status())),
                    last_check: Utc::now(),
                    check_duration_ms: start.elapsed().as_millis() as u64,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("endpoint".to_string(), self.endpoint.clone());
                        meta.insert(
                            "status_code".to_string(),
                            response.status().as_u16().to_string(),
                        );
                        meta
                    },
                })
            }
            Err(e) => Ok(ComponentHealth {
                name: self.name.clone(),
                status: HealthStatus::Unhealthy,
                message: Some(format!("Request failed: {e}")),
                last_check: Utc::now(),
                check_duration_ms: start.elapsed().as_millis() as u64,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("endpoint".to_string(), self.endpoint.clone());
                    meta.insert("error".to_string(), e.to_string());
                    meta
                },
            }),
        }
    }

    fn component_name(&self) -> &str {
        &self.name
    }
}
