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


/// Ecosystem RPC Models - Dynamic Primal Discovery Data Structures
///
/// Data structures for sovereign ecosystem integration that maintains human dignity
/// and individual autonomy while enabling powerful network effects through capability-based discovery.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// ====== REGISTRATION MODELS ======
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
// ====== CAPABILITY MODELS ======
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
// ====== DISCOVERY MODELS ======
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
// ====== COLLABORATION MODELS ======
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
// ====== NETWORK MODELS ======
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
// ====== ENUMS ======
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
// ====== RESPONSE WRAPPERS ======
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub processing_time_ms: u64,
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
