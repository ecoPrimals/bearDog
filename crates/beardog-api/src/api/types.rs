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


/// Common Types for BearDog API
///
/// Shared data types and constants used across the API layer.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// API version information
pub use beardog_types::constants::unified::api::VERSION as API_VERSION;
pub use beardog_types::constants::unified::api::VERSION_HEADER as API_VERSION_HEADER;
/// Common pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    pub page: Option<u32>,
    /// Number of items per page
    pub per_page: Option<u32>,
    /// Sort field
    pub sort_by: Option<String>,
    /// Sort order (asc/desc)
    pub sort_order: Option<String>,
}
impl Default for PaginationParams {}


    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            sort_by: None,
            sort_order: Some("asc".to_string()),
        }
    }
/// Common filter parameters
pub struct FilterParams {
    /// Date range start
    pub start_date: Option<String>,
    /// Date range end
    pub end_date: Option<String>,
    /// Status filter
    pub status: Option<String>,
    /// Category filter
    pub category: Option<String>,
    /// Search query
    pub search: Option<String>,
    /// Additional filters
    pub filters: Option<HashMap<String, String>>,
/// Common response metadata
pub struct ResponseMetadata {
    /// Request ID for tracing
    pub request_id: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Timestamp of response
    pub timestamp: String,
    /// API version
    pub api_version: String,
    /// Whether the response was cached
    pub cached: bool,}


impl ResponseMetadata {
    /// Create new response metadata}


    pub fn new(request_id: String, processing_time_ms: u64, cached: bool) -> Self {
            request_id,
            processing_time_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
            api_version: API_VERSION.to_string(),
            cached,
/// Health check status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some degradation but functional
    Degraded,
    /// Service unavailable
    Unhealthy,
/// Service component health}


pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Component status
    pub status: HealthStatus,
    /// Health percentage (0-100)
    pub health_percentage: f64,
    /// Last check timestamp
    pub last_check: String,
    /// Any issues or warnings
    pub issues: Vec<String>,
    /// Additional metadata
    pub metadata: Option<HashMap<String, String>>,
/// Rate limiting information
pub struct RateLimitInfo {
    /// Maximum requests per window
    pub limit: u32,
    /// Remaining requests in current window
    pub remaining: u32,
    /// Window reset time
    pub reset_time: String,
    /// Window duration in seconds
    pub window_seconds: u32,
/// Audit trail entry
pub struct AuditEntry {
    /// Entry ID
    pub id: String,
    /// Timestamp
    /// User or system performing the action
    pub actor: String,
    /// Action performed
    pub action: String,
    /// Resource affected
    pub resource: String,
    /// Outcome of the action
    pub outcome: String,
    /// Additional context
    pub context: Option<HashMap<String, String>>,
/// Feature flag status
pub struct FeatureFlag {
    /// Feature name
    /// Whether the feature is enabled
    pub enabled: bool,
    /// Feature description
    pub description: Option<String>,
    /// Rollout percentage (0-100)
    pub rollout_percentage: Option<f64>,
/// API endpoint information
pub struct EndpointInfo {
    /// HTTP method
    pub method: String,
    /// Endpoint path
    pub path: String,
    /// Description
    pub description: String,
    /// Whether authentication is required
    pub requires_auth: bool,
    /// Required permissions
    pub permissions: Vec<String>,
    /// Rate limit tier
    pub rate_limit_tier: Option<String>,
