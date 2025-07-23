//! HTTP Adapter for Universal Service Integration
//!
//! Provides HTTP-based connectivity to service meshes.

// HTTP adapter implementation

use super::service_registration::UniversalServiceRegistration;
use crate::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub request_id: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

/// Universal response structure  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    pub request_id: String,
    pub status: ResponseStatus,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorInfo>,
    pub timestamp: DateTime<Utc>,
}

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    Success,
    Error,
}

/// Error information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

/// Service mesh connector trait
#[async_trait::async_trait]
pub trait ServiceMeshConnector {
    /// Register service with the mesh
    async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> BearDogResult<()>;

    /// Send request through the mesh
    async fn send_request(&self, request: &UniversalRequest) -> BearDogResult<UniversalResponse>;

    /// Handle mesh request
    async fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse;

    /// Health check
    async fn health_check(&self) -> bool;

    /// Deregister service
    async fn deregister(&self, service_id: &str) -> BearDogResult<()>;
}

/// HTTP adapter for service mesh connectivity
pub struct HttpAdapter {
    base_url: String,
    client: reqwest::Client,
}

impl HttpAdapter {
    /// Create new HTTP adapter
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ServiceMeshConnector for HttpAdapter {
    async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> BearDogResult<()> {
        let url = format!("{}/api/v1/services/register", self.base_url);

        let response = self
            .client
            .post(&url)
            .json(registration)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to register service: {e}"),
            })?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BearDogError::Federation {
                message: format!("Service registration failed: {}", response.status()),
            })
        }
    }

    async fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse {
        tracing::debug!("Handling HTTP mesh request: {}", request.request_id);

        // Build the service mesh API endpoint based on operation
        let endpoint = match request.operation.as_str() {
            "sign" | "verify" | "encrypt" | "decrypt" | "generate_key" => "/api/v1/crypto",
            "authenticate" | "authorize" | "create_session" => "/api/v1/auth",
            "log_event" | "get_events" | "cleanup_events" => "/api/v1/audit",
            "generate_address" | "verify_address" => "/api/v1/address",
            _ => "/api/v1/generic",
        };

        let url = format!("{}{}/{}", self.base_url, endpoint, request.operation);

        // Create HTTP request body
        let body = serde_json::json!({
            "request_id": request.request_id,
            "operation": request.operation,
            "payload": request.payload,
            "metadata": request.metadata,
            "timestamp": request.timestamp
        });

        // Make HTTP request to service mesh
        match self
            .client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    // Parse successful response
                    match response.json::<serde_json::Value>().await {
                        Ok(data) => UniversalResponse {
                            request_id: request.request_id,
                            status: ResponseStatus::Success,
                            data: Some(data),
                            error: None,
                            timestamp: Utc::now(),
                        },
                        Err(e) => UniversalResponse {
                            request_id: request.request_id,
                            status: ResponseStatus::Error,
                            data: None,
                            error: Some(ErrorInfo {
                                code: "parse_error".to_string(),
                                message: format!("Failed to parse response: {e}"),
                                details: None,
                            }),
                            timestamp: Utc::now(),
                        },
                    }
                } else {
                    // Handle HTTP error response
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    UniversalResponse {
                        request_id: request.request_id,
                        status: ResponseStatus::Error,
                        data: None,
                        error: Some(ErrorInfo {
                            code: "http_error".to_string(),
                            message: format!("HTTP {status} - {error_text}"),
                            details: Some(serde_json::json!({
                                "status_code": status.as_u16(),
                                "url": url
                            })),
                        }),
                        timestamp: Utc::now(),
                    }
                }
            }
            Err(e) => {
                tracing::error!("HTTP request failed: {}", e);
                UniversalResponse {
                    request_id: request.request_id,
                    status: ResponseStatus::Error,
                    data: None,
                    error: Some(ErrorInfo {
                        code: "connection_error".to_string(),
                        message: format!("Failed to connect to service mesh: {e}"),
                        details: Some(serde_json::json!({"url": url})),
                    }),
                    timestamp: Utc::now(),
                }
            }
        }
    }

    async fn health_check(&self) -> bool {
        let url = format!("{}/health", self.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn deregister(&self, service_id: &str) -> BearDogResult<()> {
        let url = format!("{}/api/v1/services/deregister", self.base_url);

        let deregister_request = serde_json::json!({
            "service_id": service_id
        });

        let response = self
            .client
            .post(&url)
            .json(&deregister_request)
            .send()
            .await
            .map_err(|e| BearDogError::Network {
                message: format!("Failed to deregister service: {e}"),
            })?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(BearDogError::Federation {
                message: format!("Service deregistration failed: {}", response.status()),
            })
        }
    }

    async fn send_request(&self, request: &UniversalRequest) -> BearDogResult<UniversalResponse> {
        Ok(self.handle_mesh_request(request.clone()).await)
    }
}
