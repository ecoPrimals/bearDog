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


/// SongBird Client Integration
///
/// Provides integration with the SongBird service mesh for distributed operations.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::{NetworkConfig, ServiceEndpoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// SongBird service client configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongBirdClientConfig {
    pub service_endpoint: ServiceEndpoint,
    pub timeout: Duration,
    pub retry_attempts: u32,
    pub enable_discovery: bool,
}

impl Default for SongBirdClientConfig {
    fn default() -> Self {
        Self {
            service_endpoint: ServiceEndpoint {
                host: "localhost".to_string(),
                port: 8080,
                scheme: "http".to_string(),
            },
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            enable_discovery: true,
        }
    }
}

/// SongBird client for service mesh operations
#[derive(Debug)]
pub struct SongBirdClient {
    config: SongBirdClientConfig,
    client: reqwest::Client,
}

impl SongBirdClient {
    /// Create new SongBird client
    pub fn new(config: SongBirdClientConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Failed to create HTTP client", e))
})?;

        Self { config, client }
    }

    /// Connect to SongBird service mesh
    pub async fn connect(&self) -> BearDogResult<()> {
        let url = format!(
            "{}://{}:{}/health",
            self.config.service_endpoint.scheme,
            self.config.service_endpoint.host,
            self.config.service_endpoint.port
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to connect to SongBird: {}", e)))?;

        if response.status().is_success() {
            tracing::info!("Successfully connected to SongBird service mesh");
            Ok(())
        } else {
            Err(BearDogError::system(format!(
                "SongBird health check failed with status: {}",
                response.status()
            )))
        }
    }

    /// Register service with SongBird discovery
    pub async fn register_service(
        &self,
        service_name: &str,
        metadata: HashMap<String, String>,
    ) -> BearDogResult<String> {
        if !self.config.enable_discovery {
            return Err(BearDogError::system(
                "Service discovery is disabled".to_string(),
            ));
        }

        let registration_data = ServiceRegistration {
            service_name: service_name.to_string(),
            metadata,
            health_check_url: format!(
                "{}://{}:{}/health",
                self.config.service_endpoint.scheme,
                self.config.service_endpoint.host,
                self.config.service_endpoint.port
            ),
        };

        let url = format!(
            "{}://{}:{}/register",
            self.config.service_endpoint.scheme,
            self.config.service_endpoint.host,
            self.config.service_endpoint.port
        );

        let response = self
            .client
            .post(&url)
            .json(&registration_data)
            .send()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to register service: {}", e)))?;

        if response.status().is_success() {
            let registration_id: String = response
                .json()
                .await
                .map_err(|e| BearDogError::system(format!("Failed to parse response: {}", e)))?;

            tracing::info!("Service '{}' registered with ID: {}", service_name, registration_id);
            Ok(registration_id)
        } else {
            Err(BearDogError::system(format!(
                "Service registration failed with status: {}",
                response.status()
            )))
        }
    }

    /// Discover services by name
    pub async fn discover_service(&self, service_name: &str) -> BearDogResult<Vec<ServiceEndpoint>> {
        let url = format!(
            "{}://{}:{}/discover/{}",
            self.config.service_endpoint.scheme,
            self.config.service_endpoint.host,
            self.config.service_endpoint.port,
            service_name
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to discover service: {}", e)))?;

        if response.status().is_success() {
            let endpoints: Vec<ServiceEndpoint> = response
                .json()
                .await
                .map_err(|e| BearDogError::system(format!("Failed to parse response: {}", e)))?;

            tracing::info!("Discovered {} endpoints for service '{}'", endpoints.len(), service_name);
            Ok(endpoints)
        } else {
            Err(BearDogError::system(format!(
                "Service discovery failed with status: {}",
                response.status()
            )))
        }
    }
}

/// Service registration data
#[derive(Debug, Serialize, Deserialize)]
struct ServiceRegistration {
    service_name: String,
    metadata: HashMap<String, String>,
    health_check_url: String,
}

/// SongBird service trait - modernized with native async fn
#[allow(async_fn_in_trait)]
pub trait SongBirdService: Send + Sync {
    /// Initialize the service
    async fn initialize(&mut self) -> BearDogResult<()>;

    /// Handle incoming request
    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse>;

    /// Get service health status
    async fn health_check(&self) -> BearDogResult<ServiceHealth>;

    /// Shutdown the service gracefully
    async fn shutdown(&mut self) -> BearDogResult<()>;
}

/// Service request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Service response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Service health information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub healthy: bool,
    pub message: Option<String>,
    pub uptime_seconds: u64,
    pub version: String,
}
