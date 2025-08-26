

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::{NetworkConfig, ServiceEndpoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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

#[derive(Debug)]
pub struct SongBirdClient {
    config: SongBirdClientConfig,
    client: reqwest::Client,
}

impl SongBirdClient {

    pub fn new(config: SongBirdClientConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create HTTP client", e).to_string())
})?;

        Self { config, client }
    }

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
            .map_err(|e| BearDogError::system(format_args!("Failed to connect to SongBird: {}", e).to_string()))?;

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

    pub async fn register_service(
        &self,
        service_name: &str,
        metadata: HashMap<&str, &str>,
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
            .map_err(|e| BearDogError::system(format_args!("Failed to register service: {}", e).to_string()))?;

        if response.status().is_success() {
            let registration_id: String = response
                .json()
                .await
                .map_err(|e| BearDogError::system(format_args!("Failed to parse response: {}", e).to_string()))?;

            tracing::info!("Service '{}' registered with ID: {}", service_name, registration_id);
            Ok(registration_id)
        } else {
            Err(BearDogError::system(format!(
                "Service registration failed with status: {}",
                response.status()
            )))
        }
    }

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
            .map_err(|e| BearDogError::system(format_args!("Failed to discover service: {}", e).to_string()))?;

        if response.status().is_success() {
            let endpoints: Vec<ServiceEndpoint> = response
                .json()
                .await
                .map_err(|e| BearDogError::system(format_args!("Failed to parse response: {}", e).to_string()))?;

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

#[derive(Debug, Serialize, Deserialize)]
struct ServiceRegistration {
    service_name: String,
    metadata: HashMap<String, String>,
    health_check_url: String,
}

#[allow(async_fn_in_trait)]
pub trait SongBirdService: Send + Sync {

    async fn initialize(&mut self) -> BearDogResult<()>;

    async fn handle_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse>;

    async fn health_check(&self) -> BearDogResult<ServiceHealth>;

    async fn shutdown(&mut self) -> BearDogResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub healthy: bool,
    pub message: Option<String>,
    pub uptime_seconds: u64,
    pub version: String,
}
