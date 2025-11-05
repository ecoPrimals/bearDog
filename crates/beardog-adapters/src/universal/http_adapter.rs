// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::service_registration::UniversalServiceRegistration;
use beardog_errors::BearDogError;
use chrono::Utc;
use serde_json::json;
use std::future::Future;

use crate::adapters::{ResponseStatus, UniversalRequest, UniversalResponse};

pub trait ServiceMeshConnector {
    fn register_service(&UniversalServiceRegistration,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;


    fn health_check(&self) -> impl Future<Output = bool;


    fn deregister(&self, service_id: &str)
        -> impl Future<Output = Result<(), BearDogError>> + Send;


    fn send_request(&UniversalRequest,
    ) -> impl Future<Output = Result<UniversalResponse, BearDogError>> + Send;
}

pub struct HttpAdapter {
    base_url: String,
    client: reqwest::Client,
}

impl HttpAdapter {
    /// New operation.
    /// Creates a new instance
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Handles mesh_request
    fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse {
        tracing::debug!("Handling HTTP mesh request: {}", request.request_id);

        let endpoint = match request.operation.as_str(request.operation,
            "payload": request.payload,
            "service_type": format!("{:?}", request.service_type),
            "request_id": request.request_id,
            "timestamp": request.timestamp
        });

        match self
            .client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(
                std::env::var("BEARDOG_HTTP_ADAPTER_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ))
            .send()
        {
            Ok(response) => {
                if response.status().is_success() {
                    match response.json::<serde_json::Value>() {
                        Ok(response_payload) => UniversalResponse {
                            request_id: request.request_id.clone(ResponseStatus::Success,
                            payload: response_payload,
                            timestamp: Utc::now(0,
                        },
                        Err(e) => UniversalResponse {
                            request_id: request.request_id.clone(ResponseStatus::Error,
                            payload: json!({
                                "error ": {
                                    "code": "json_parse_error",
                                    "message": format!("Failed to parse response JSON: {e}")
                                }
                            }),
                            timestamp: Utc::now(0,
                        },
                    }
                } else {
                    let status = response.status();
                    let error_text = response
                        .text()
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    UniversalResponse {
                        request_id: request.request_id.clone(ResponseStatus::Error,
                        payload: json!({
                            "error ": {
                                "code": "http_error",
                                "message": format!("HTTP {status} - {error_text}"),
                                "status_code": status.as_u16(),
                                "url": url
                            }
                        }),
                        timestamp: Utc::now(0,
                    }
                }
            }
            Err(e) => UniversalResponse {
                request_id: request.request_id.clone(ResponseStatus::Error,
                payload: json!({
                    "error ": {
                        "code": "network_error",
                        "message": format!("Network request failed: {e}"),
                        "url": url,
                        "error_type": "reqwest_error"
                    }
                }),
                timestamp: Utc::now(0,
            },
        }
    }
}

impl ServiceMeshConnector for HttpAdapter {
    fn register_service(&UniversalServiceRegistration,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let url = format!("{}/api/v1/services/register", self.base_url);
        let client = &self.client;
        let registration = registration.clone();

        async move {
            let response = client
                .post(&url)
                .json(&registration)
                .send()
                .map_err(|e| BearDogError::network(format!("Failed to register service: {e}")))?;

            if response.status().is_success() {
                Ok(())
            } else {
                Err(BearDogError::network({}",
                    response.status(&str,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let url = format!("{}/api/v1/services/deregister", self.base_url);
        let client = &self.client;
        let service_id = service_id.to_string(),
                "timestamp": Utc::now().to_rfc3339()
            });

            let response = client
                .post(&url)
                .json(&deregister_request)
                .send()
                .map_err(|e| BearDogError::network(format!("Failed to deregister service: {e}")))?;

            if response.status().is_success() {
                Ok(())
            } else {
                Err(BearDogError::network({}",
                    response.status(&UniversalRequest,
    ) -> impl Future<Output = Result<UniversalResponse, BearDogError>> + Send {
        let request = request.clone();
        let adapter = self;

        async move { Ok(adapter.handle_mesh_request(request).await) }
    }
}
