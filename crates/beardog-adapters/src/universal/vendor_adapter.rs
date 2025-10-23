//! # Universal Vendor Adapter
//!
//! This module provides vendor-specific adapter functionality for the BearDog ecosystem.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Vendor adapter for handling vendor-specific operations
#[derive(Debug, Clone)]
pub struct VendorAdapter {
    /// Vendor identifier
    pub vendor_id: String,
    /// Vendor configuration
    pub config: VendorConfig,
    /// Adapter metrics
    pub metrics: VendorMetrics,
}

/// Vendor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorConfig {
    /// Vendor name
    pub vendor_name: String,
    /// API endpoint
    pub api_endpoint: String,
    /// Timeout in seconds
    pub timeout_seconds: u64,
    /// Enable monitoring
    pub enable_monitoring: bool,
}

/// Vendor adapter metrics
#[derive(Debug, Clone, Default)]
pub struct VendorMetrics {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful requests
    pub successful_requests: u64,
    /// Failed requests
    pub failed_requests: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}

/// Vendor request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorRequest {
    /// Request identifier
    pub request_id: String,
    /// Vendor-specific operation
    pub operation: String,
    /// Request payload
    pub payload: serde_json::Value,
}

/// Vendor response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorResponse {
    /// Request identifier
    pub request_id: String,
    /// Response status
    pub status: ResponseStatus,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if any
    pub error: Option<String>,
}

/// Response status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStatus {
    /// Request completed successfully
    Success,
    /// Request failed
    Error,
    /// Request is being processed
    Pending,
}

impl VendorAdapter {
    /// Creates a new vendor adapter
    pub fn new() -> Self {
        Self {
            vendor_id: Uuid::new_v4().to_string(),
            config: VendorConfig::default(),
            metrics: VendorMetrics::default(),
        }
    }

    /// Creates a new vendor adapter with configuration
    pub fn with_config(config: VendorConfig) -> Self {
        Self {
            vendor_id: Uuid::new_v4().to_string(),
            config,
            metrics: VendorMetrics::default(),
        }
    }

    /// Processes a vendor request
    pub async fn process_request(&mut self, request: VendorRequest) -> BearDogResult<VendorResponse> {
        self.metrics.total_requests += 1;

        let response = match request.operation.as_str() {
            "ping" => VendorResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: serde_json::json!({"message": "pong"}),
                error: None,
            },
            "echo" => VendorResponse {
                request_id: request.request_id,
                status: ResponseStatus::Success,
                data: request.payload,
                error: None,
            },
            _ => VendorResponse {
                request_id: request.request_id,
                status: ResponseStatus::Error,
                data: serde_json::Value::Null,
                error: Some(format!("Unsupported operation: {}", request.operation)),
            },
        };

        if matches!(response.status, ResponseStatus::Success) {
            self.metrics.successful_requests += 1;
        } else {
            self.metrics.failed_requests += 1;
        }

        Ok(response)
    }

    /// Gets adapter metrics
    pub fn get_metrics(&self) -> &VendorMetrics {
        &self.metrics
    }
}

impl Default for VendorConfig {
    fn default() -> Self {
        Self {
            vendor_name: std::env::var("BEARDOG_VENDOR_NAME")
                .unwrap_or_else(|_| "default".to_string()),
            api_endpoint: std::env::var("BEARDOG_VENDOR_API_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            timeout_seconds: std::env::var("BEARDOG_VENDOR_TIMEOUT_SECONDS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            enable_monitoring: std::env::var("BEARDOG_VENDOR_ENABLE_MONITORING")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl Default for VendorAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Universal Vendor Adapter for handling multiple vendors
pub struct UniversalVendorAdapter {
    /// Registered vendor adapters
    adapters: HashMap<String, VendorAdapter>,
    /// Universal adapter configuration
    config: UniversalAdapterConfig,
    /// Adapter ID
    adapter_id: Uuid,
}

impl UniversalVendorAdapter {
    /// Creates a new universal vendor adapter
    pub fn new(config: UniversalAdapterConfig) -> BearDogResult<Self> {
        Ok(Self {
            adapters: HashMap::new(),
            config,
            adapter_id: Uuid::new_v4(),
        })
    }

    /// Registers a vendor adapter
    pub fn register_vendor(&mut self, vendor_id: String, adapter: VendorAdapter) {
        self.adapters.insert(vendor_id, adapter);
    }

    /// Processes a request for a specific vendor
    pub async fn process_vendor_request(
        &mut self,
        vendor_id: &str,
        request: VendorRequest,
    ) -> BearDogResult<VendorResponse> {
        match self.adapters.get_mut(vendor_id) {
            Some(adapter) => adapter.process_request(request).await,
            None => Err(BearDogError::system(format!("Vendor not found: {}", vendor_id))),
        }
    }

    /// Gets the adapter ID
    pub fn adapter_id(&self) -> Uuid {
        self.adapter_id
    }
}

pub struct UniversalAdapterConfig {
    pub discovery_interval_seconds: u64,
    pub max_concurrent_operations: usize,
    pub default_timeout_seconds: u64,
    pub enable_monitoring: bool,
    pub enable_health_checks: bool,
    pub health_check_interval_seconds: u64,
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            discovery_interval_seconds: 300, // 5 minutes
            max_concurrent_operations: 100,
            default_timeout_seconds: 30,
            enable_monitoring: true,
            enable_health_checks: true,
            health_check_interval_seconds: 60, // 1 minute
        }
    }
}

pub struct AdapterStatistics {
    pub adapter_id: Uuid,
    /// Number of total_capabilities
    pub total_capabilities: usize,
    /// Number of total_handlers
    pub total_handlers: usize,
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of successful_requests
    pub successful_requests: u64,
    /// Number of failed_requests
    pub failed_requests: u64,
    pub average_response_time_ms: f64,
}
