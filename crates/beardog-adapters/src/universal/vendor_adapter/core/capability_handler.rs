

use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::{UniversalVendorRequest, UniversalVendorResponse};

pub trait CapabilityHandler: Send + Sync + std::fmt::Debug {

    fn capability_type(&self) -> CapabilityType;

    async fn can_handle(&self, request: &UniversalVendorRequest) -> BearDogResult<f64>;

    async fn execute(
        &self,
        request: UniversalVendorRequest,
    ) -> BearDogResult<UniversalVendorResponse>;

    fn get_metadata(&self) -> CapabilityMetadata;

    async fn health_check(&self) -> BearDogResult<CapabilityHealth>;

    async fn initialize(&mut self, config: CapabilityConfig) -> BearDogResult<()>;

    async fn shutdown(&mut self) -> BearDogResult<()>;

    fn supported_operations(&self) -> Vec<String>;

    fn handler_version(&self) -> String {
        "1.0.0".to_string()
    }

    fn handler_description(&self) -> String {
        format_args!("Handler for {:?} capability", self.capability_type().to_string())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMetadata {

    pub instance_id: Uuid,

    pub capability: CapabilityType,

    pub handler_name: String,

    pub handler_version: String,

    pub description: String,

    pub performance: PerformanceProfile,

    pub quality: QualityProfile,

    pub cost: CostProfile,

    pub compliance: ComplianceProfile,

    pub location: Option<GeographicLocation>,

    pub supported_operations: Vec<String>,

    pub resource_requirements: ResourceRequirements,

    pub tags: Vec<String>,

    pub custom_metadata: HashMap<String, serde_json::Value>,

    pub created_at: DateTime<Utc>,

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
            custom_metadata: HashMap::with_capacity(16),
            created_at: now,
            updated_at: now,
        }

pub struct PerformanceProfile {

    pub average_response_time_ms: f64,

    pub p95_response_time_ms: f64,

    pub p99_response_time_ms: f64,

    pub throughput_ops_per_second: f64,

    pub success_rate: f64,

    pub cpu_usage: f64,

    pub memory_usage_mb: f64,

    pub network_bandwidth_mbps: f64,

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

pub struct QualityProfile {

    pub reliability_score: f64,

    pub availability_percentage: f64,

    pub consistency_level: ConsistencyLevel,

    pub security_rating: u8,

    pub durability_rating: u8,

    pub fault_tolerance_rating: u8,

    pub recovery_time_objective_seconds: u64,

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

pub enum ConsistencyLevel {

    Strong,

    Eventual,

    Session,

    BoundedStaleness,

    MonotonicRead,

pub struct CostProfile {

    pub cost_per_operation_usd: f64,

    pub fixed_monthly_cost_usd: f64,

    pub cost_per_mb_usd: f64,

    pub cost_per_hour_usd: f64,

    pub free_tier: Option<FreeTierLimits>,

    pub pricing_model: PricingModel,}

impl Default for CostProfile {
            cost_per_operation_usd: 0.001,
            fixed_monthly_cost_usd: 0.0,
            cost_per_mb_usd: 0.01,
            cost_per_hour_usd: 0.1,
            free_tier: None,
            pricing_model: PricingModel::PayPerUse,

pub struct FreeTierLimits {

    pub max_operations_per_month: u64,

    pub max_data_transfer_mb_per_month: u64,

    pub max_concurrent_requests: u32,

pub enum PricingModel {

    PayPerUse,

    Subscription,

    Tiered,

    Free,

    Custom,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]}

pub struct ComplianceProfile {

    pub soc2_type2: bool,

    pub iso27001: bool,

    pub gdpr_compliant: bool,

    pub hipaa_compliant: bool,

    pub pci_dss_compliant: bool,

    pub fedramp_authorized: bool,

    pub additional_certifications: Vec<String>,

    pub compliance_documentation: HashMap<String, String>,

pub struct GeographicLocation {

    pub country_code: String,

    pub region: String,

    pub city: String,

    pub latitude: f64,

    pub longitude: f64,

    pub availability_zone: Option<String>,

pub struct ResourceRequirements {

    pub min_cpu_cores: u32,

    pub recommended_cpu_cores: u32,

    pub min_memory_mb: u64,

    pub recommended_memory_mb: u64,

    pub min_disk_space_mb: u64,

    pub network_bandwidth_mbps: u32,

    pub special_hardware: Vec<String>,

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

pub struct CapabilityHealth {

    pub status: HealthStatus,

    pub health_score: f64,

    pub last_check: DateTime<Utc>,

    pub check_duration_ms: u64,

    pub details: HashMap<String, HealthDetail>,

    pub error_message: Option<String>,

pub enum HealthStatus {

    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

pub struct HealthDetail {

    pub component: String,

    pub message: String,

    pub metrics: HashMap<String, f64>,

pub struct CapabilityConfig {

    pub parameters: HashMap<String, serde_json::Value>,

    pub environment: String,

    pub feature_flags: HashMap<String, bool>,

    pub resource_limits: ResourceLimits,

    pub security_settings: SecuritySettings,}

impl Default for CapabilityConfig {
            parameters: HashMap::with_capacity(16),
            environment: "development".to_string(),
            feature_flags: HashMap::with_capacity(16),
            resource_limits: ResourceLimits::default(),
            security_settings: SecuritySettings::default(),

pub struct ResourceLimits {

    pub max_memory_mb: u64,

    pub max_cpu_percentage: f64,

    pub request_timeout_seconds: u64,}

impl Default for ResourceLimits {
            max_concurrent_requests: 100,
            max_memory_mb: 1024,
            max_cpu_percentage: 80.0,
            request_timeout_seconds: 30,

pub struct SecuritySettings {

    pub enable_tls: bool,

    pub tls_version: String,

    pub verify_certificates: bool,

    pub api_key_auth: Option<String>,

    pub oauth2_config: Option<OAuth2Config>,

    pub ip_whitelist: Vec<String>,

    pub rate_limiting: Option<RateLimitConfig>,}

impl Default for SecuritySettings {
            enable_tls: true,
            tls_version: "1.3".to_string(),
            verify_certificates: true,
            api_key_auth: None,
            oauth2_config: None,
            ip_whitelist: Vec::new(),
            rate_limiting: None,

pub struct OAuth2Config {

    pub client_id: String,

    pub client_secret: String,

    pub auth_url: String,

    pub token_url: String,

    pub scopes: Vec<String>,

