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


/// # Songbird Service Mesh Types
///
/// **SERVICE MESH DATA STRUCTURES AND TYPES**
/// Contains all data structures, types, and models used by the songbird service mesh client
/// extracted from the monolithic songbird_client.rs file.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_types::providers::ServiceHealth;
/// Information about a discovered service mesh primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMeshInfo {
    /// Service mesh primal name (e.g., "Songbird", "NewMesh", etc.)
    pub name: String,
    /// Service mesh endpoint URL
    pub endpoint: String,
    /// Service mesh capabilities
    pub capabilities: Vec<String>,
    /// API version supported
    pub api_version: String,
    /// Health status of the service mesh
    pub health: ServiceHealth,
    /// Last health check timestamp
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    /// Service mesh metadata
    pub metadata: HashMap<String, String>,
    /// Priority/preference for this mesh (higher = preferred)
    pub priority: u8,
}
/// `BearDog` registration information with service mesh
pub struct RegistrationInfo {
    /// Registration ID assigned by service mesh
    pub registration_id: String,
    /// `BearDog` service information
    pub service_info: beardog_types::canonical::services::UniversalServiceMetadata,
    /// Registration timestamp
    pub registered_at: chrono::DateTime<chrono::Utc>,
    /// Registration expiry (if applicable)
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Health check configuration
    pub health_check_config: Option<HealthCheckConfig>,
    /// Service tags for discovery
    pub tags: Vec<String>,
// Use canonical HealthCheckConfig from beardog-types
pub use beardog_types::canonical::monitoring::HealthCheckConfig;
/// Discovered service information
pub struct DiscoveredService {
    /// Service ID
    pub service_id: String,
    /// Service name
    pub service_name: String,
    /// Service type/category
    pub service_type: String,
    /// Service endpoint URL
    /// Service capabilities
    pub capabilities: Vec<ServiceMeshCapability>,
    /// Service health status
    /// Service metadata
    /// Service tags
    /// Service version
    pub version: Option<String>,
    /// When service was discovered
    pub discovered_at: chrono::DateTime<chrono::Utc>,
/// Service mesh capability definition};


pub struct ServiceMeshCapability {
    /// Capability name
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: Option<String>,
    /// Capability parameters
    pub parameters: HashMap<String, serde_json::Value>,
/// Service registration request
pub struct ServiceRegistrationRequest {
    /// Service information to register
    pub service: beardog_types::canonical::services::UniversalServiceMetadata,
    pub health_check: Option<HealthCheckConfig>,
    /// Registration metadata
    /// Registration TTL in seconds
    pub ttl_seconds: Option<u64>,
/// Service lookup request
pub struct ServiceLookupRequest {
    /// Service name pattern
    pub service_name: Option<String>,
    /// Service type filter
    pub service_type: Option<String>,
    /// Required capabilities
    pub required_capabilities: Vec<String>,
    /// Service tags filter
    /// Health status filter
    pub health_filter: Option<ServiceHealth>,
    /// Maximum results to return
    pub limit: Option<u32>,
/// Service update request
pub struct ServiceUpdateRequest {
    /// Service ID to update
    /// Updated service information
    /// Updated health check configuration
    /// Updated tags
    pub tags: Option<Vec<String>>,
    /// Updated metadata
    pub metadata: Option<HashMap<String, String>>,
/// Service mesh operation result
pub struct ServiceMeshResult<T> {
    /// Operation success status
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<T>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Operation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Operation ID for tracking
    pub operation_id: Uuid,
/// Service mesh statistics
pub struct ServiceMeshStats {
    /// Total registered services
    pub total_services: u64,
    /// Healthy services count
    pub healthy_services: u64,
    /// Unhealthy services count
    pub unhealthy_services: u64,
    /// Total service mesh nodes
    pub mesh_nodes: u64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,}


impl Default for ServiceMeshInfo {}


    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            endpoint: std::env::var("BEARDOG_SONGBIRD_ENDPOINT")
            .unwrap_or_else(|_| beardog_types::constants::network::TEST_LOCALHOST_HTTP.to_string()),
            capabilities: vec![],
            api_version: "v1".to_string(),
            health: ServiceHealth {
                is_healthy: false,
                last_check: chrono::Utc::now(),
                response_time_ms: 0.0,
                error_message: Some("Unknown status".to_string()),
            },
            last_health_check: None,
            metadata: HashMap::new(),
            priority: 0,
        }
    }
