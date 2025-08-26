

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalRegistrationRequest {
    pub ecosystem_id: String,
    pub registration_type: String,
    pub preferred_role: Option<String>,
}
pub struct PrimalRegistrationResponse {
    pub registration_id: String,
    pub status: String,
    pub ecosystem_role: String,
    pub network_position: NetworkPosition,
    pub available_integrations: Vec<String>,
}

pub struct SelfPrimalRegistration {
    pub primal_id: String,
    pub version: String,
    pub name: String, // Only self-knowledge allowed
    pub capabilities: Vec<PrimalCapability>,
    pub maintainer: MaintainerInfo,
pub struct MaintainerInfo {
    pub name: String,
    pub contact: String, // Could be email, matrix, etc.

pub struct PrimalCapability {
    pub capability_id: String,
    pub description: String,
    pub interfaces: Vec<CapabilityInterface>,
    pub requirements: Vec<String>,
    pub tags: Vec<String>,
}

pub struct CapabilityInterface {
    pub interface_type: String, // "rpc", "rest", "graphql", "message_queue", etc.
    pub endpoint: String,
    pub methods: Vec<String>,
    pub authentication_required: bool,

pub struct CapabilityDiscoveryRequest {
    pub requesting_primal_id: String,
    pub required_capabilities: Vec<String>,
    pub optional_capabilities: Vec<String>,
    pub compatibility_version: String,
}

pub struct DiscoveredPrimal {
    pub network_address: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub trust_score: Option<f64>,
pub struct CapabilityDiscoveryResponse {
    pub discovered_primals: Vec<DiscoveredPrimal>,
    pub total_count: usize,
    pub search_metadata: HashMap<String, String>,

pub struct CrossPrimalRequest {
    pub source_primal_id: String,
    pub target_capability: String,
    pub operation: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub priority: RequestPriority,
}

pub struct CrossPrimalResponse {
    pub request_id: String,
    pub responding_primal_id: String,
    pub result: serde_json::Value,
    pub execution_time: Duration,
    pub status: OperationStatus,

pub struct NetworkPosition {
    pub node_id: String,
    pub routing_priority: i32,
    pub available_bandwidth: Option<u64>,
    pub supported_protocols: Vec<String>,
}

pub struct NetworkEffect {
    pub effect_id: String,
    pub participating_capabilities: Vec<String>,
    pub efficiency_multiplier: f64,
    pub human_benefit_score: f64,

pub enum RequestPriority {
    Low,
    Normal,
    High,
    Urgent,}

pub enum OperationStatus {
    Success,
    Pending,
    Failed,
    Timeout,
    Unauthorized,
    CapabilityNotFound,

pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub processing_time_ms: u64,
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
