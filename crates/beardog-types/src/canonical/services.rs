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


/// # Canonical Services Types - Minimal Version
///
/// **TEMPORARY MINIMAL IMPLEMENTATION** for build stability

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL** Service Type
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

/// **CANONICAL** Service Endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub url: String,
    pub service_type: ServiceType,
    pub health_status: ServiceHealthStatus,
    pub metadata: HashMap<String, String>,
}

/// **CANONICAL** Service Health Status
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

/// **CANONICAL** Universal Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalRequest {
    pub request_id: String,
    pub service_type: ServiceType,
    pub operation: String,
    pub payload: serde_json::Value,
    pub priority: RequestPriority,
    pub timestamp: DateTime<Utc>,
}

/// **CANONICAL** Request Priority
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

/// **CANONICAL** Universal Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalResponse {
    pub request_id: String,
    pub status: ResponseStatus,
    pub payload: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub processing_time_ms: u64,
}

/// **CANONICAL** Response Status
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
