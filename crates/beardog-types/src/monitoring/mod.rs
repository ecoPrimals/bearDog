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


/// Monitoring and Health Check Types
///
/// Unified monitoring system for all `BearDog` services and integrations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Service health monitor for tracking service status
#[derive(Debug, Clone)]
pub struct ServiceHealthMonitor {
    pub service_id: String,
    pub last_check: Option<DateTime<Utc>>,
    pub status: crate::canonical::HealthStatus,
    pub metrics: HealthMetrics,
    pub check_interval: Duration,
}
impl ServiceHealthMonitor {
    /// Create a new health monitor for a service}


    pub fn new(service_id: &str) -> Self {
        Self {
            service_id: service_id.to_string(),
            last_check: None,
            status: crate::canonical::HealthStatus::Unknown,
            metrics: HealthMetrics::default(),
            check_interval: Duration::from_secs(30),
        }
    }
    /// Check if the service is healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, crate::canonical::HealthStatus::Healthy)
    /// Update health status}


    pub fn update_status(&mut self, status: crate::canonical::HealthStatus) {
        self.status = status;
        self.last_check = Some(Utc::now());
/// Service health status enumeration
pub use crate::canonical::HealthStatus;
/// Health check metrics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Response time in milliseconds
    pub response_time_ms: u64,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Error count in last window
    pub error_count: u64,
    /// Total requests processed
    pub total_requests: u64,
    /// Custom metrics
    pub custom_metrics: HashMap<String, serde_json::Value>,
/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]};


pub struct HealthCheckConfig {
    /// Health check endpoint URL
    pub endpoint: String,
    /// Check interval in seconds
    pub interval_seconds: u64,
    /// Timeout for health checks
    pub timeout_seconds: u64,
    /// Number of failures before marking unhealthy
    pub failure_threshold: u32,
    /// Custom headers for health checks
    pub headers: HashMap<String, String>,
    /// Expected response status codes
    pub expected_status_codes: Vec<u16>,}


impl Default for HealthCheckConfig {}


    fn default() -> Self {
            endpoint: "/health".to_string(),
            interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            headers: HashMap::new(),
            expected_status_codes: vec![200],
