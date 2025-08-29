use super::service_registration::UniversalServiceRegistration;
use beardog_errors::BearDogError;
use chrono::Utc;
use serde_json::json;
use std::future::Future;

use crate::adapters::{ResponseStatus, UniversalRequest, UniversalResponse};

pub trait ServiceMeshConnector {
    fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;

    fn health_check(&self) -> impl Future<Output = bool> + Send;

    fn deregister(&self, service_id: &str)
        -> impl Future<Output = Result<(), BearDogError>> + Send;

    fn send_request(
        &self,
        request: &UniversalRequest,
    ) -> impl Future<Output = Result<UniversalResponse, BearDogError>> + Send;
}

#[derive(Clone)]
pub struct HttpAdapter {
    base_url: String,
    client: reqwest::Client,
}

impl HttpAdapter {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    async fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse {
        tracing::debug!("Handling HTTP mesh request: {}", request.request_id);

        let endpoint = match request.operation.as_str() {
            "sign" | "verify" | "encrypt" | "decrypt" | "generate_key" => "/api/v1/crypto",
            "authenticate" | "authorize" | "create_session" => "/api/v1/auth",
            "log_event" | "get_events" | "cleanup_events" => "/api/v1/audit",
            "generate_address" | "verify_address" => "/api/v1/address",
            _ => "/api/v1/generic",
        };

        let url = format!("{}{}", self.base_url, endpoint);

        let body = json!({
            "operation": request.operation,
            "payload": request.payload,
            "service_type": format!("{:?}", request.service_type),
            "request_id": request.request_id,
            "timestamp": request.timestamp
        });

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
                    match response.json::<serde_json::Value>().await {
                        Ok(data) => UniversalResponse {
                            request_id: request.request_id,
                            status: ResponseStatus::Success,
                            payload: data,
                            timestamp: Utc::now(),
                            processing_time_ms: 0,
                        },
                        Err(e) => UniversalResponse {
                            request_id: request.request_id,
                            status: ResponseStatus::Error,
                            payload: json!({
                                "error": {
                                    "code": "json_parse_error",
                                    "message": format!("Failed to parse response JSON: {}", e)
                                }
                            }),
                            timestamp: Utc::now(),
                            processing_time_ms: 0,
                        },
                    }
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    UniversalResponse {
                        request_id: request.request_id,
                        status: ResponseStatus::Error,
                        payload: json!({
                            "error": {
                                "code": "http_error",
                                "message": format!("HTTP {} - {}", status, error_text),
                                "status_code": status.as_u16(),
                                "url": url
                            }
                        }),
                        timestamp: Utc::now(),
                        processing_time_ms: 0,
                    }
                }
            }
            Err(e) => UniversalResponse {
                request_id: request.request_id,
                status: ResponseStatus::Error,
                payload: json!({
                    "error": {
                        "code": "network_error",
                        "message": format!("Network request failed: {}", e),
                        "url": url,
                        "error_type": "reqwest_error"
                    }
                }),
                timestamp: Utc::now(),
                processing_time_ms: 0,
            },
        }
    }
}

impl ServiceMeshConnector for HttpAdapter {
    fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let url = format!("{}/api/v1/services/register", self.base_url);
        let client = self.client.clone();
        let registration = registration.clone();

        async move {
            let response = client
                .post(&url)
                .json(&registration)
                .send()
                .await
                .map_err(|e| BearDogError::network(format!("Failed to register service: {e}")))?;

            if response.status().is_success() {
                Ok(())
            } else {
                Err(BearDogError::network(format!(
                    "Service registration failed with status: {}",
                    response.status()
                )))
            }
        }
    }

    fn health_check(&self) -> impl Future<Output = bool> + Send {
        let url = format!("{}/health", self.base_url);
        let client = self.client.clone();

        async move {
            match client.get(&url).send().await {
                Ok(response) => response.status().is_success(),
                Err(_) => false,
            }
        }
    }

    fn deregister(
        &self,
        service_id: &str,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let url = format!("{}/api/v1/services/deregister", self.base_url);
        let client = self.client.clone();
        let service_id = service_id.to_string();

        async move {
            let deregister_request = json!({
                "service_id": service_id,
                "timestamp": Utc::now().to_rfc3339()
            });

            let response = client
                .post(&url)
                .json(&deregister_request)
                .send()
                .await
                .map_err(|e| {
                    BearDogError::network(format!("Failed to deregister service: {e}"))
                })?;

            if response.status().is_success() {
                Ok(())
            } else {
                Err(BearDogError::network(format!(
                    "Service deregistration failed with status: {}",
                    response.status()
                )))
            }
        }
    }

    fn send_request(
        &self,
        request: &UniversalRequest,
    ) -> impl Future<Output = Result<UniversalResponse, BearDogError>> + Send {
        let request = request.clone();
        let adapter = self.clone();

        async move { Ok(adapter.handle_mesh_request(request).await) }
    }
}
