

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use beardog_errors::BearDogResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub confidence: f64,
    pub method: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct AttestationVerificationResult {
    pub verified: bool,
    pub chain: Vec<String>,
    pub trust_level: String,

pub struct CapabilityHealthStatus {
    pub overall_status: String,
    pub individual_status: std::collections::HashMap<String, String>,

pub use beardog_types::canonical::HealthStatus;

pub struct KeyOperationStatus {
    pub healthy: bool,
    pub operations_tested: Vec<String>,
    pub response_times: serde_json::Value,

pub struct EndpointHealth {
    pub name: String,

pub struct ResponseTimeMetrics {
    pub average: f64,
    pub p95: f64,

pub struct PrimalMetadata {

    pub primal_type: PrimalType,

    pub version: String,

    pub capabilities: Vec<PrimalCapability>,

    pub dependencies: Vec<PrimalDependency>,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrimalType {

    `BearDog`,

    ToadStool,

    Songbird,

    NestGate,

    Squirrel,

    BiomeOS,

    Custom(String),

pub enum PrimalCapability {

    Security,

    Compute,

    Storage,

    ArtificialIntelligence,

    Networking,

    Monitoring,

pub enum PrimalDependency {

    Required {

        primal: PrimalType,

        min_version: String,

        reason: String,
    },

    Optional {

pub struct PrimalIntegrationConfig {

    pub enable_toadstool_integration: bool,

    pub enable_songbird_integration: bool,

    pub enable_squirrel_integration: bool,

    pub enable_nestgate_integration: bool,

    pub custom_config: HashMap<String, serde_json::Value>,

pub struct PrimalError {

    pub code: String,

    pub message: String,

    pub details: HashMap<String, serde_json::Value>,

pub struct PrimalRequest {

    pub id: String,

    pub params: HashMap<String, serde_json::Value>,

    pub metadata: HashMap<String, String>,

    pub timestamp: DateTime<Utc>,

pub struct PrimalResponse {

    pub data: Option<serde_json::Value>,

    pub error: Option<PrimalError>,

impl PrimalResponse {

    pub fn success<T: Serialize>(data: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
            error: None,
            metadata: ahash::HashMap::default(),
            timestamp: Utc::now(),
        }
    }

    pub fn error(error: PrimalError) -> Self {
            success: false,
            data: None,
            error: Some(error),

impl HealthStatus {

    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)

    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy | HealthStatus::Critical)

    pub fn is_critical(&self) -> bool {
        matches!(self, HealthStatus::Critical)

pub struct PrimalHealth {

    pub status: HealthStatus,

    pub components: HashMap<String, HealthStatus>,

    pub last_check: DateTime<Utc>,

    pub next_check: DateTime<Utc>,

pub struct ResourceUsageInfo {

    pub cpu_percent: f64,

    pub memory_bytes: u64,

    pub network_bytes_per_sec: u64,

    pub disk_bytes_per_sec: u64,

pub struct PrimalConfig {

    pub data: HashMap<String, serde_json::Value>,

} 
