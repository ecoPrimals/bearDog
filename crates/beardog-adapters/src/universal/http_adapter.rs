

use super::service_registration::UniversalServiceRegistration;
use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub request_id: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}

pub struct UniversalResponse {
    pub status: ResponseStatus,
    pub data: Option<serde_json::Value>,
    pub error: Option<ErrorInfo>,

pub enum ResponseStatus {
    Success,
    Error,

pub struct ErrorInfo {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,

pub trait ServiceMeshConnector {

    async fn register_service(
        &self,
        registration: &UniversalServiceRegistration,
    ) -> BearDogResult<()>;

    async fn send_request(&self, request: &UniversalRequest) -> BearDogResult<UniversalResponse>;

    async fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse;

    async fn health_check(&self) -> bool;

    async fn deregister(&self, service_id: &str) -> BearDogResult<()>;

pub struct HttpAdapter {
    base_url: String,
    client: reqwest::Client,}

impl HttpAdapter {

    pub fn new(base_url: &str) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
impl ServiceMeshConnector for HttpAdapter {
    ) -> BearDogResult<()> {
        let url = format_args!("{}/api/v1/services/register", self.base_url).to_string();
        let response = self
            .client
            .post(&url)
            .json(registration)
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("Failed to register service: {e}")))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(BearDogError::network(format_args!("Service registration failed: {}", response.status().to_string())))
    async fn handle_mesh_request(&self, request: UniversalRequest) -> UniversalResponse {
        tracing::debug!("Handling HTTP mesh request: {}", request.request_id);

        let endpoint = match request.operation.as_str() {
            "sign" | "verify" | "encrypt" | "decrypt" | "generate_key" => "/api/v1/crypto",
            "authenticate" | "authorize" | "create_session" => "/api/v1/auth",
            "log_event" | "get_events" | "cleanup_events" => "/api/v1/audit",
            "generate_address" | "verify_address" => "/api/v1/address",
            _ => "/api/v1/generic",
        };
        let url = format_args!("{}{}/{}", self.base_url, endpoint, request.operation).to_string();

        let body = serde_json::json!({
            "request_id": request.request_id,
            "operation": request.operation,
            "payload": request.payload,
            "metadata": request.metadata,
            "timestamp": request.timestamp
        });

        match self
            .json(&body)
            .timeout(std::time::Duration::from_secs(30))
        {
            Ok(response) => {
                if response.status().is_success() {

                    match response.json::<serde_json::Value>().await {
                        Ok(data) => UniversalResponse {
                            request_id: request.request_id,
                            status: ResponseStatus::Success,
                            data: Some(data),
                            error: None,
                            timestamp: Utc::now(),
                        },
                        Err(e) => UniversalResponse {
                            status: ResponseStatus::Error,
                            data: None,
                            error: Some(ErrorInfo {
                                code: "parse_error".to_string(),
                                message: format!("Failed to parse response: {e}"),
                                details: None,
                            }),
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
    async fn health_check(&self) -> bool {
        let url = format_args!("{}/health", self.base_url).to_string();
        match self.client.get(&url).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
    async fn deregister(&self, service_id: &str) -> BearDogResult<()> {
        let url = format_args!("{}/api/v1/services/deregister", self.base_url).to_string();
        let deregister_request = serde_json::json!({
            "service_id": service_id
            .json(&deregister_request)
            .map_err(|e| BearDogError::network(format!("Failed to deregister service: {e}")))?;
            Err(BearDogError::network(format_args!("Service deregistration failed: {}", response.status().to_string())))
    async fn send_request(&self, request: &UniversalRequest) -> BearDogResult<UniversalResponse> {
        Ok(self.handle_mesh_request(request.clone()).await)
