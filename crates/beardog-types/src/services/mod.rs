

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceMetadata {

    pub service_id: Uuid,

    pub name: String,

    pub description: String,

    pub version: String,

    pub capabilities: Vec<ServiceCapability>,

    pub endpoints: Vec<ServiceEndpoint>,

    pub health_status: super::providers::ServiceHealth,

    pub category: ServiceCategory,

    pub security_domain: SecurityDomain,

    pub tags: Vec<String>,

    pub contact_info: ContactInfo,

    pub dependencies: Vec<ServiceDependency>,

    pub resource_requirements: ResourceRequirements,

    pub metadata: HashMap<String, serde_json::Value>,

    pub registered_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub priority: u32,
}

pub struct UniversalRequest {

    pub request_id: Uuid,

    pub system_id: String,

    pub operation: String,

    pub parameters: HashMap<String, serde_json::Value>,

    pub data: Vec<u8>,

    pub timestamp: DateTime<Utc>,

    pub source: Option<String>,

    pub target: Option<String>,

    pub auth_token: Option<String>,

    pub metadata: HashMap<String, String>,

    pub priority: RequestPriority,

    pub timeout_seconds: Option<u64>,

pub struct UniversalResponse {

    pub success: bool,

    pub data: HashMap<String, serde_json::Value>,

    pub binary_data: Vec<u8>,

    pub error_message: Option<String>,

    pub error_code: Option<String>,

    pub processing_time_ms: u64,

pub struct ServiceCapability {

    pub operations: Vec<String>,

    pub required: bool,

pub struct ServiceEndpoint {

    pub url: String,

    pub protocol: EndpointProtocol,

    pub primary: bool,

    pub health_check_path: Option<String>,

pub enum EndpointProtocol {
    Http,
    Https,
    Grpc,
    WebSocket,
    Tcp,
    Udp,
    Custom(String),

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

pub enum SecurityDomain {
    Cryptography,
    IAM,
    ThreatDetection,
    DataProtection,
    NetworkSecurity,
    General,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]}

pub struct ContactInfo {

    pub email: Option<String>,

    pub phone: Option<String>,

    pub website: Option<String>,

    pub emergency_contact: Option<String>,

pub struct ServiceDependency {

    pub service_name: String,

    pub version_requirement: String,

    pub optional: bool,

    pub dependency_type: DependencyType,

pub enum DependencyType {
    Runtime,
    BuildTime,
    Optional,
    Development,

pub struct ResourceRequirements {

    pub cpu_cores: Option<f64>,

    pub memory_mb: Option<u64>,

    pub storage_gb: Option<u64>,

    pub network_mbps: Option<u64>,

    pub custom: HashMap<String, String>,

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
