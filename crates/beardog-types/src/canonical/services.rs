

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceType {
    Core,
    Security,
    Storage,
    Network,
    Monitoring,
    Workflow,
    Generic,
}

impl Default for ServiceType {
    fn default() -> Self {
        Self::Generic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub url: String,
    pub service_type: ServiceType,
    pub health_status: ServiceHealthStatus,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl Default for ServiceHealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub request_id: String,
    pub service_type: ServiceType,
    pub operation: String,
    pub payload: serde_json::Value,
    pub priority: RequestPriority,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequestPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for RequestPriority {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    pub request_id: String,
    pub status: ResponseStatus,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResponseStatus {
    Success,
    Error,
    Timeout,
    NotFound,
}

impl Default for ResponseStatus {
    fn default() -> Self {
        Self::Success
    }
}
