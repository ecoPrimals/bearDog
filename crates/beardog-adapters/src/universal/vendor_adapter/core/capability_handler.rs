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


/// Capability Handler Trait and Related Types
///
/// Defines the core trait that any vendor capability must implement
/// to work with the Universal Vendor Adapter.

use async_trait::async_trait;
use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::{UniversalVendorRequest, UniversalVendorResponse};
/// **CAPABILITY HANDLER TRAIT** - Implement for any vendor capability

pub trait CapabilityHandler: Send + Sync + std::fmt::Debug {
    /// What capability does this handler provide?
    fn capability_type(&self) -> CapabilityType;
    /// Can this handler satisfy the given request?
    /// Returns confidence score (0.0 - 1.0)
    async fn can_handle(&self, request: &UniversalVendorRequest) -> BearDogResult<f64>;
    /// Execute the capability operation
    async fn execute(
        &self,
        request: UniversalVendorRequest,
    ) -> BearDogResult<UniversalVendorResponse>;
    /// Get handler metadata (vendor-agnostic)
    fn get_metadata(&self) -> CapabilityMetadata;
    /// Health check for this capability
    async fn health_check(&self) -> BearDogResult<CapabilityHealth>;
    /// Initialize handler with configuration
    async fn initialize(&mut self, config: CapabilityConfig) -> BearDogResult<()>;
    /// Shutdown handler gracefully
    async fn shutdown(&mut self) -> BearDogResult<()>;
    /// Get supported operations for this handler
    fn supported_operations(&self) -> Vec<String>;
    /// Get handler version
    fn handler_version(&self) -> String {
        "1.0.0".to_string()
    }
    /// Get handler description
    fn handler_description(&self) -> String {
        format!("Handler for {:?} capability", self.capability_type())
}
/// **CAPABILITY METADATA** - Vendor-agnostic information about a capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMetadata {
    /// Unique capability instance ID
    pub instance_id: Uuid,
    /// Capability type
    pub capability: CapabilityType,
    /// Handler name/identifier
    pub handler_name: String,
    /// Handler version
    pub handler_version: String,
    /// Handler description
    pub description: String,
    /// Performance characteristics
    pub performance: PerformanceProfile,
    /// Quality characteristics
    pub quality: QualityProfile,
    /// Cost information
    pub cost: CostProfile,
    /// Compliance certifications
    pub compliance: ComplianceProfile,
    /// Geographic location
    pub location: Option<GeographicLocation>,
    /// Supported operations
    pub supported_operations: Vec<String>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom_metadata: HashMap<String, serde_json::Value>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,}


impl Default for CapabilityMetadata {}


    fn default() -> Self {
        let now = Utc::now();
        Self {
            instance_id: Uuid::new_v4(),
            capability: CapabilityType::Custom("unknown".to_string()),
            handler_name: "unknown".to_string(),
            handler_version: "1.0.0".to_string(),
            description: "Unknown capability handler".to_string(),
            performance: PerformanceProfile::default(),
            quality: QualityProfile::default(),
            cost: CostProfile::default(),
            compliance: ComplianceProfile::default(),
            location: None,
            supported_operations: Vec::new(),
            resource_requirements: ResourceRequirements::default(),
            tags: Vec::new(),
            custom_metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
/// **PERFORMANCE PROFILE** - Performance characteristics of a capability
pub struct PerformanceProfile {
    /// Average response time in milliseconds
    pub average_response_time_ms: f64,
    /// 95th percentile response time
    pub p95_response_time_ms: f64,
    /// 99th percentile response time
    pub p99_response_time_ms: f64,
    /// Throughput (operations per second)
    pub throughput_ops_per_second: f64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// CPU usage (0.0 - 1.0)
    pub cpu_usage: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// Network bandwidth usage in MB/s
    pub network_bandwidth_mbps: f64,
    /// Scalability rating (1-10)
    pub scalability_rating: u8,}


impl Default for PerformanceProfile {
            average_response_time_ms: 100.0,
            p95_response_time_ms: 200.0,
            p99_response_time_ms: 500.0,
            throughput_ops_per_second: 10.0,
            success_rate: 0.99,
            cpu_usage: 0.1,
            memory_usage_mb: 50.0,
            network_bandwidth_mbps: 1.0,
            scalability_rating: 5,
/// **QUALITY PROFILE** - Quality characteristics of a capability}


pub struct QualityProfile {
    /// Reliability score (0.0 - 1.0)
    pub reliability_score: f64,
    /// Availability percentage (0.0 - 100.0)
    pub availability_percentage: f64,
    /// Data consistency level
    pub consistency_level: ConsistencyLevel,
    /// Security rating (1-10)
    pub security_rating: u8,
    /// Durability rating (1-10)
    pub durability_rating: u8,
    /// Fault tolerance rating (1-10)
    pub fault_tolerance_rating: u8,
    /// Recovery time objective (RTO) in seconds
    pub recovery_time_objective_seconds: u64,
    /// Recovery point objective (RPO) in seconds
    pub recovery_point_objective_seconds: u64,}


impl Default for QualityProfile {
            reliability_score: 0.99,
            availability_percentage: 99.9,
            consistency_level: ConsistencyLevel::Eventual,
            security_rating: 8,
            durability_rating: 8,
            fault_tolerance_rating: 7,
            recovery_time_objective_seconds: 300, // 5 minutes
            recovery_point_objective_seconds: 60, // 1 minute
/// **CONSISTENCY LEVEL** - Data consistency guarantees}


pub enum ConsistencyLevel {
    /// Strong consistency - all reads receive the most recent write
    Strong,
    /// Eventual consistency - system will become consistent over time
    Eventual,
    /// Session consistency - consistency within a single session
    Session,
    /// Bounded staleness - reads may lag behind writes by a bounded amount
    BoundedStaleness,
    /// Monotonic read - reads never go backwards
    MonotonicRead,
/// **COST PROFILE** - Cost characteristics of a capability}


pub struct CostProfile {
    /// Cost per operation in USD
    pub cost_per_operation_usd: f64,
    /// Fixed monthly cost in USD
    pub fixed_monthly_cost_usd: f64,
    /// Cost per MB of data
    pub cost_per_mb_usd: f64,
    /// Cost per hour of usage
    pub cost_per_hour_usd: f64,
    /// Free tier limits
    pub free_tier: Option<FreeTierLimits>,
    /// Pricing model
    pub pricing_model: PricingModel,}


impl Default for CostProfile {
            cost_per_operation_usd: 0.001,
            fixed_monthly_cost_usd: 0.0,
            cost_per_mb_usd: 0.01,
            cost_per_hour_usd: 0.1,
            free_tier: None,
            pricing_model: PricingModel::PayPerUse,
/// **FREE TIER LIMITS**}


pub struct FreeTierLimits {
    /// Maximum operations per month
    pub max_operations_per_month: u64,
    /// Maximum data transfer in MB per month
    pub max_data_transfer_mb_per_month: u64,
    /// Maximum concurrent requests
    pub max_concurrent_requests: u32,
/// **PRICING MODEL**
pub enum PricingModel {
    /// Pay per operation
    PayPerUse,
    /// Fixed monthly subscription
    Subscription,
    /// Tiered pricing based on usage
    Tiered,
    /// Free service
    Free,
    /// Custom pricing
    Custom,
/// **COMPLIANCE PROFILE** - Compliance certifications and standards
#[derive(Debug, Clone, Serialize, Deserialize, Default)]}


pub struct ComplianceProfile {
    /// SOC 2 Type II compliance
    pub soc2_type2: bool,
    /// ISO 27001 certification
    pub iso27001: bool,
    /// GDPR compliance
    pub gdpr_compliant: bool,
    /// HIPAA compliance
    pub hipaa_compliant: bool,
    /// PCI DSS compliance
    pub pci_dss_compliant: bool,
    /// FedRAMP authorization
    pub fedramp_authorized: bool,
    /// Additional certifications
    pub additional_certifications: Vec<String>,
    /// Compliance documentation URLs
    pub compliance_documentation: HashMap<String, String>,
/// **GEOGRAPHIC LOCATION** - Physical location information
pub struct GeographicLocation {
    /// Country code (ISO 3166-1 alpha-2)
    pub country_code: String,
    /// Region/state
    pub region: String,
    /// City
    pub city: String,
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Data center/availability zone
    pub availability_zone: Option<String>,
/// **RESOURCE REQUIREMENTS** - Resource needs for the capability
pub struct ResourceRequirements {
    /// Minimum CPU cores required
    pub min_cpu_cores: u32,
    /// Recommended CPU cores
    pub recommended_cpu_cores: u32,
    /// Minimum RAM in MB
    pub min_memory_mb: u64,
    /// Recommended RAM in MB
    pub recommended_memory_mb: u64,
    /// Minimum disk space in MB
    pub min_disk_space_mb: u64,
    /// Network bandwidth requirements in Mbps
    pub network_bandwidth_mbps: u32,
    /// Special hardware requirements
    pub special_hardware: Vec<String>,
    /// Operating system requirements
    pub os_requirements: Vec<String>,}


impl Default for ResourceRequirements {
            min_cpu_cores: 1,
            recommended_cpu_cores: 2,
            min_memory_mb: 512,
            recommended_memory_mb: 1024,
            min_disk_space_mb: 1024,
            network_bandwidth_mbps: 10,
            special_hardware: Vec::new(),
            os_requirements: vec!["Linux".to_string()],
/// **CAPABILITY HEALTH** - Health status of a capability}


pub struct CapabilityHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health score (0.0 - 1.0)
    pub health_score: f64,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Health check duration in milliseconds
    pub check_duration_ms: u64,
    /// Detailed health information
    pub details: HashMap<String, HealthDetail>,
    /// Error message if unhealthy
    pub error_message: Option<String>,
/// **HEALTH STATUS**
pub enum HealthStatus {
    /// Capability is fully operational
    Healthy,
    /// Capability is operational but with some issues
    Degraded,
    /// Capability is not operational
    Unhealthy,
    /// Health status is unknown
    Unknown,
/// **HEALTH DETAIL** - Detailed health information for a specific component}


pub struct HealthDetail {
    /// Component name
    pub component: String,
    /// Component status
    /// Component message
    pub message: String,
    /// Component metrics
    pub metrics: HashMap<String, f64>,
/// **CAPABILITY CONFIG** - Configuration for a capability handler
pub struct CapabilityConfig {
    /// Configuration parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Environment-specific settings
    pub environment: String,
    /// Feature flags
    pub feature_flags: HashMap<String, bool>,
    /// Resource limits
    pub resource_limits: ResourceLimits,
    /// Security settings
    pub security_settings: SecuritySettings,}


impl Default for CapabilityConfig {
            parameters: HashMap::new(),
            environment: "development".to_string(),
            feature_flags: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            security_settings: SecuritySettings::default(),
/// **RESOURCE LIMITS** - Limits on resource usage}


pub struct ResourceLimits {
    /// Maximum memory usage in MB
    pub max_memory_mb: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percentage: f64,
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,}


impl Default for ResourceLimits {
            max_concurrent_requests: 100,
            max_memory_mb: 1024,
            max_cpu_percentage: 80.0,
            request_timeout_seconds: 30,
/// **SECURITY SETTINGS** - Security configuration for capabilities}


pub struct SecuritySettings {
    /// Enable TLS/SSL
    pub enable_tls: bool,
    /// TLS version
    pub tls_version: String,
    /// Certificate validation
    pub verify_certificates: bool,
    /// API key authentication
    pub api_key_auth: Option<String>,
    /// OAuth2 configuration
    pub oauth2_config: Option<OAuth2Config>,
    /// IP whitelist
    pub ip_whitelist: Vec<String>,
    /// Rate limiting
    pub rate_limiting: Option<RateLimitConfig>,}


impl Default for SecuritySettings {
            enable_tls: true,
            tls_version: "1.3".to_string(),
            verify_certificates: true,
            api_key_auth: None,
            oauth2_config: None,
            ip_whitelist: Vec::new(),
            rate_limiting: None,
/// **OAUTH2 CONFIG** - OAuth2 authentication configuration}


pub struct OAuth2Config {
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization URL
    pub auth_url: String,
    /// Token URL
    pub token_url: String,
    /// Scopes
    pub scopes: Vec<String>,
/// **RATE LIMIT CONFIG** - Rate limiting configuration
