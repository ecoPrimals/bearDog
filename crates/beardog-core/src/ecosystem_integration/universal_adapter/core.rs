// SPDX-License-Identifier: AGPL-3.0-or-later

// Core Universal Adapter Implementation

use super::config::UniversalAdapterConfig;
use super::metrics::AdapterMetrics;
use super::types::{
    AdapterRequest, AdapterResponse, ConnectionInfo, ResponseStatus, ServiceEndpoint,
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[inline]
fn elapsed_ms_u64(elapsed: std::time::Duration) -> u64 {
    u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
}

/// Universal adapter for ecosystem service integration
///
/// Provides a unified interface for connecting to any ecosystem service through
/// capability-based discovery and protocol-agnostic communication.
#[derive(Debug)]
pub struct UniversalAdapter {
    /// Adapter configuration
    config: UniversalAdapterConfig,
    /// Active connections to services
    connections: Arc<RwLock<HashMap<String, ConnectionInfo>>>,
    /// Discovered service endpoints
    endpoints: Arc<RwLock<HashMap<String, ServiceEndpoint>>>,
    /// Performance and usage metrics
    metrics: Arc<RwLock<AdapterMetrics>>,
}

impl UniversalAdapter {
    /// Create a new universal adapter
    /// Creates a new instance
    pub fn new(config: UniversalAdapterConfig) -> Self {
        info!("Creating universal adapter: {}", config.adapter_name);

        Self {
            config,
            connections: Arc::new(RwLock::new(HashMap::new())),
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(AdapterMetrics::default())),
        }
    }

    /// Process an adapter request
    ///
    /// # Errors
    /// Returns `Err(BearDogError)` if the request processing fails
    pub async fn process_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        debug!("Processing request: {:?}", request.request_id);

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_requests += 1;
        }

        // Process the request based on operation type
        match request.operation {
            super::types::AdapterOperation::Connect => self.handle_connect_request(request).await,
            super::types::AdapterOperation::Disconnect => {
                self.handle_disconnect_request(request).await
            }
            super::types::AdapterOperation::Request => self.handle_generic_request(request),
            super::types::AdapterOperation::HealthCheck => {
                self.handle_health_check_request(request).await
            }
            _ => {
                warn!("Unsupported operation: {:?}", request.operation);
                Err(BearDogError::business("Unsupported operation".to_string()))
            }
        }
    }

    /// Handle connect request
    /// Handles `connect_request`
    async fn handle_connect_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        let start = std::time::Instant::now();
        info!("Handling connect request to: {}", request.endpoint);

        // Create connection info
        let connection_info = ConnectionInfo {
            connection_id: Uuid::new_v4(),
            endpoint: request.endpoint.clone(),
            protocol: super::types::ProtocolType::Http, // Default for now
            status: super::types::ConnectionStatus::Active,
            established_at: chrono::Utc::now(),
            last_activity: chrono::Utc::now(),
            metadata: request.metadata.clone(),
        };

        // Store connection
        {
            let mut connections = self.connections.write().await;
            connections.insert(request.endpoint.clone(), connection_info);
        }

        // Create response
        Ok(AdapterResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            headers: HashMap::new(),
            payload: Some({
                use serde_json::{Map, Value};
                let mut payload = Map::new();
                payload.insert("connected".to_string(), Value::Bool(true));
                Value::Object(payload)
            }),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: elapsed_ms_u64(start.elapsed()),
        })
    }

    /// Handle disconnect request
    /// Handles `disconnect_request`
    async fn handle_disconnect_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        let start = std::time::Instant::now();
        info!("Handling disconnect request from: {}", request.endpoint);

        // Remove connection
        {
            let mut connections = self.connections.write().await;
            connections.remove(&request.endpoint);
        }

        // Create response
        Ok(AdapterResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            headers: HashMap::new(),
            payload: Some({
                use serde_json::{Map, Value};
                let mut payload = Map::new();
                payload.insert("disconnected".to_string(), Value::Bool(true));
                Value::Object(payload)
            }),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: elapsed_ms_u64(start.elapsed()),
        })
    }

    /// Handle generic request
    /// Handles `generic_request`
    fn handle_generic_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        let start = std::time::Instant::now();
        debug!("Handling generic request to: {}", request.endpoint);

        // Create response
        Ok(AdapterResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            headers: HashMap::new(),
            payload: request.payload,
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: elapsed_ms_u64(start.elapsed()),
        })
    }

    /// Handle health check request
    /// Handles `health_check_request`
    async fn handle_health_check_request(
        &self,
        request: AdapterRequest,
    ) -> Result<AdapterResponse, BearDogError> {
        let start = std::time::Instant::now();
        debug!("Handling health check request");

        let health_status = {
            use serde_json::{Map, Value};
            let mut status = Map::new();
            status.insert("status".to_string(), Value::String("healthy".to_string()));
            status.insert(
                "adapter_id".to_string(),
                Value::String(self.config.adapter_id.clone()),
            );
            status.insert(
                "connections".to_string(),
                Value::Number(self.connections.read().await.len().into()),
            );
            status.insert(
                "endpoints".to_string(),
                Value::Number(self.endpoints.read().await.len().into()),
            );
            Value::Object(status)
        };

        Ok(AdapterResponse {
            request_id: request.request_id,
            status: ResponseStatus::Success,
            headers: HashMap::new(),
            payload: Some(health_status),
            metadata: HashMap::new(),
            timestamp: chrono::Utc::now(),
            duration_ms: elapsed_ms_u64(start.elapsed()),
        })
    }

    /// Get adapter configuration
    #[must_use]
    pub const fn config(&self) -> &UniversalAdapterConfig {
        &self.config
    }

    /// Get current metrics
    pub async fn metrics(&self) -> AdapterMetrics {
        self.metrics.read().await.clone()
    }

    /// Get connection count
    pub async fn connection_count(&self) -> usize {
        self.connections.read().await.len()
    }

    /// Get endpoint count
    pub async fn endpoint_count(&self) -> usize {
        self.endpoints.read().await.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecosystem_integration::universal_adapter::types::{
        AdapterOperation, ProtocolType, ResponseStatus,
    };

    fn make_config() -> UniversalAdapterConfig {
        UniversalAdapterConfig {
            adapter_id: "test-adapter".to_string(),
            adapter_name: "Test Adapter".to_string(),
            supported_protocols: vec![ProtocolType::Http],
            connection_timeout_ms: 5000,
            request_timeout_ms: 3000,
            max_retries: 2,
            auto_reconnect: true,
            health_check_interval_secs: 10,
            custom_config: std::collections::HashMap::new(),
        }
    }

    #[test]
    fn test_universal_adapter_new() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.config().adapter_id, "test-adapter");
    }

    #[tokio::test]
    async fn test_process_request_connect() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request =
            AdapterRequest::new(AdapterOperation::Connect, "https://example.com".to_string());
        let response = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        assert_eq!(response.status, ResponseStatus::Success);
        assert!(response.payload.is_some());
        let payload = response
            .payload
            .expect("connect response should include payload in test");
        assert_eq!(
            payload
                .get("connected")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
    }

    #[tokio::test]
    async fn test_process_request_disconnect() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request = AdapterRequest::new(
            AdapterOperation::Disconnect,
            "https://example.com".to_string(),
        );
        let response = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(
            response
                .payload
                .expect("disconnect response should include payload in test")
                .get("disconnected")
                .and_then(serde_json::Value::as_bool),
            Some(true)
        );
    }

    #[tokio::test]
    async fn test_process_request_health_check() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request = AdapterRequest::new(
            AdapterOperation::HealthCheck,
            "https://example.com".to_string(),
        );
        let response = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        assert_eq!(response.status, ResponseStatus::Success);
        let payload = response
            .payload
            .expect("health check response should include payload in test");
        assert_eq!(
            payload.get("status").and_then(|v| v.as_str()),
            Some("healthy")
        );
    }

    #[tokio::test]
    async fn test_process_request_generic() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request =
            AdapterRequest::new(AdapterOperation::Request, "https://example.com".to_string())
                .with_payload(serde_json::json!({"key": "value"}));
        let response = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        assert_eq!(response.status, ResponseStatus::Success);
        assert_eq!(response.payload, Some(serde_json::json!({"key": "value"})));
    }

    #[tokio::test]
    async fn test_process_request_unsupported_operation() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request = AdapterRequest::new(
            AdapterOperation::Subscribe,
            "https://example.com".to_string(),
        );
        let result = adapter.process_request(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_connection_count() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        assert_eq!(adapter.connection_count().await, 0);
        let request =
            AdapterRequest::new(AdapterOperation::Connect, "https://example.com".to_string());
        let _ = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        assert_eq!(adapter.connection_count().await, 1);
    }

    #[tokio::test]
    async fn test_metrics() {
        let config = make_config();
        let adapter = UniversalAdapter::new(config);
        let request =
            AdapterRequest::new(AdapterOperation::Connect, "https://example.com".to_string());
        let _ = adapter
            .process_request(request)
            .await
            .expect("process_request should succeed in test");
        let metrics = adapter.metrics().await;
        assert!(metrics.total_requests >= 1);
    }
}
