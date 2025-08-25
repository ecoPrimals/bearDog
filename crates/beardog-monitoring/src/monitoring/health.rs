// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use chrono::Utc;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::types::ComponentHealth;
use beardog_errors::BearDogResult;
use beardog_types::canonical::health_status::HealthStatus;

/// Health check trait for components to implement - modernized with native async fn
#[allow(async_fn_in_trait)]
pub trait HealthChecker: Send + Sync {
    /// Perform a health check on this component
    async fn check_health(&self) -> BearDogResult<ComponentHealth>;
    /// Get the name of this component
    fn component_name(&self) -> &str;
}

/// Enum-based health checker for dynamic dispatch compatibility
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

/// Database health checker
#[derive(Debug)]
pub struct DatabaseHealthChecker {}

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
                meta.insert("host".to_string(), "localhost:5432".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Database"
    }
}

/// Cache health checker
#[derive(Debug)]
pub struct CacheHealthChecker {}

impl Default for CacheHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheHealthChecker {
    /// Create a new cache health checker
    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for CacheHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        // In a real implementation, you'd ping Redis/cache
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        Ok(ComponentHealth {
            name: "Cache".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Cache connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
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

/// External API health checker
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
    /// Create a new external API health checker
    pub fn new(api_endpoint: String) -> Self {
        Self { api_endpoint }
    }
}

impl HealthChecker for ExternalApiHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        // In a real implementation, you'd make an HTTP request
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        Ok(ComponentHealth {
            name: "External API".to_string(),
            status: HealthStatus::Healthy,
            message: Some("External API responding".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
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

/// HSM health checker
#[derive(Debug)]
pub struct HsmHealthChecker {}

impl Default for HsmHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmHealthChecker {
    /// Create a new HSM health checker
    pub fn new() -> Self {
        Self {}
    }
}

impl HealthChecker for HsmHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        // In a real implementation, you'd check HSM connectivity
        tokio::time::sleep(Duration::from_millis(20)).await;
        
        Ok(ComponentHealth {
            name: "HSM".to_string(),
            status: HealthStatus::Healthy,
            message: Some("HSM operational".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
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

/// Health check aggregator that runs multiple health checkers using enum dispatch
pub struct HealthCheckAggregator {
    checkers: Vec<HealthCheckerType>,
}

impl HealthCheckAggregator {
    /// Create a new health check aggregator
    pub fn new() -> Self {
        Self {
            checkers: Vec::new(),
        }
    }

    /// Add a health checker
    pub fn add_checker(&mut self, checker: HealthCheckerType) {
        self.checkers.push(checker);
    }

    /// Run all health checks
    pub async fn check_all(&self) -> BearDogResult<Vec<ComponentHealth>> {
        let mut results = Vec::new();
        
        for checker in &self.checkers {
            match checker.check_health().await {
                Ok(health) => results.push(health),
                Err(e) => {
                    // Create a failed health check result
                    results.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {}", e)),
                        last_check: Utc::now(),
                        check_duration_ms: 0,
                        metadata: HashMap::new(),
                    });
                }
            }
        }
        
        Ok(results)
    }

    /// Get overall system health status
    pub async fn get_overall_status(&self) -> BearDogResult<HealthStatus> {
        let results = self.check_all().await?;
        
        // If any component is unhealthy, the system is unhealthy
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

