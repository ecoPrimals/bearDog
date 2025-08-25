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


/// Unified Service Types
///
/// **EXTENSIBLE SERVICE TYPE SYSTEM**
/// This module provides a unified, extensible type system for all services
/// that can seamlessly incorporate new type systems without breaking changes.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// **UNIVERSAL SERVICE METADATA** - Extensible for any service type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceMetadata {
    /// Service identifier (UUID for uniqueness)
    pub service_id: Uuid,
    /// Human-readable service name
    pub name: String,
    /// Service description
    pub description: String,
    /// Service version (semantic versioning)
    pub version: String,
    /// Service capabilities (extensible list)
    pub capabilities: Vec<ServiceCapability>,
    /// Service endpoints (multiple protocols supported)
    pub endpoints: Vec<ServiceEndpoint>,
    /// Service health status
    pub health_status: super::providers::ServiceHealth,
    /// Service category for organization
    pub category: ServiceCategory,
    /// Security domain classification
    pub security_domain: SecurityDomain,
    /// Service tags for discovery and filtering
    pub tags: Vec<String>,
    /// Contact information
    pub contact_info: ContactInfo,
    /// Service dependencies
    pub dependencies: Vec<ServiceDependency>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Service-specific metadata (fully extensible)
    pub metadata: HashMap<String, serde_json::Value>,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Service priority for selection
    pub priority: u32,
}
/// **UNIVERSAL REQUEST** - Can handle any operation type
pub struct UniversalRequest {
    /// Unique request identifier
    pub request_id: Uuid,
    /// System/service identifier making the request
    pub system_id: String,
    /// Operation being requested
    pub operation: String,
    /// Request parameters (fully extensible)
    pub parameters: HashMap<String, serde_json::Value>,
    /// Binary data payload (for file transfers, etc.)
    pub data: Vec<u8>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Source service identifier
    pub source: Option<String>,
    /// Target service identifier
    pub target: Option<String>,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Request metadata (fully extensible)
    pub metadata: HashMap<String, String>,
    /// Request priority
    pub priority: RequestPriority,
    /// Request timeout
    pub timeout_seconds: Option<u64>,
/// **UNIVERSAL RESPONSE** - Can handle any response type
pub struct UniversalResponse {
    /// Request ID this response corresponds to
    /// Success indicator
    pub success: bool,
    /// Response data (fully extensible)
    pub data: HashMap<String, serde_json::Value>,
    /// Binary response data
    pub binary_data: Vec<u8>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Error code for programmatic handling
    pub error_code: Option<String>,
    /// Response timestamp
    /// Response metadata (fully extensible)
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
/// Service capability definition (extensible)
pub struct ServiceCapability {
    /// Capability identifier
    /// Capability version
    /// Capability description
    /// Supported operations for this capability
    pub operations: Vec<String>,
    /// Capability-specific parameters
    /// Whether this capability is required
    pub required: bool,
/// Service endpoint definition (multi-protocol)
pub struct ServiceEndpoint {
    /// Endpoint identifier
    /// Endpoint URL
    pub url: String,
    /// Protocol type
    pub protocol: EndpointProtocol,
    /// Whether this is the primary endpoint
    pub primary: bool,
    /// Endpoint-specific metadata
    /// Health check path for this endpoint
    pub health_check_path: Option<String>,
/// Supported endpoint protocols (extensible)
pub enum EndpointProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,
    Custom(String),
/// Service categories (extensible)}


pub enum ServiceCategory {
    Security,
    Storage,
    Compute,
    Network,
    AI,
    Compliance,
    Monitoring,
    Analytics,
    Integration,
/// Security domain classification (extensible)
pub enum SecurityDomain {
    Cryptography,
    IAM,
    ThreatDetection,
    DataProtection,
    NetworkSecurity,
    General,
/// Contact information for service support
#[derive(Debug, Clone, Serialize, Deserialize, Default)]}


pub struct ContactInfo {
    /// Support email address
    pub email: Option<String>,
    /// Support phone number
    pub phone: Option<String>,
    /// Support website URL
    pub website: Option<String>,
    /// Emergency contact information
    pub emergency_contact: Option<String>,
/// Service dependency definition
pub struct ServiceDependency {
    /// Dependency service name
    pub service_name: String,
    /// Required version range
    pub version_requirement: String,
    /// Whether this dependency is optional
    pub optional: bool,
    /// Dependency type
    pub dependency_type: DependencyType,
/// Dependency types
pub enum DependencyType {
    Runtime,
    BuildTime,
    Optional,
    Development,
/// Resource requirements for service deployment}


pub struct ResourceRequirements {
    /// CPU cores required
    pub cpu_cores: Option<f64>,
    /// Memory in MB required
    pub memory_mb: Option<u64>,
    /// Storage in GB required
    pub storage_gb: Option<u64>,
    /// Network bandwidth in Mbps
    pub network_mbps: Option<u64>,
    /// Custom resource requirements
    pub custom: HashMap<String, String>,
/// Request priority levels
pub enum RequestPriority {
    Low,
    Normal,
    High,
    Critical,}


impl Default for RequestPriority {}


    fn default() -> Self {
        Self::Normal
    }
impl Default for ServiceCategory {
        Self::Custom("unknown".to_string())}


impl Default for SecurityDomain {
        Self::General
