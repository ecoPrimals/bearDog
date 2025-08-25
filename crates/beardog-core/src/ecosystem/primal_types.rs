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


/// # EcoPrimal Type Definitions
///
/// Contains all data structures, enums, and type definitions for the EcoPrimal interface.
/// This module was extracted from primal_interface.rs to improve maintainability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use beardog_errors::BearDogResult;
/// Authentication result structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResult {
    pub success: bool,
    pub confidence: f64,
    pub method: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
/// Attestation verification result
pub struct AttestationVerificationResult {
    pub verified: bool,
    pub chain: Vec<String>,
    pub trust_level: String,
/// Capability health status
pub struct CapabilityHealthStatus {
    pub overall_status: String,
    pub individual_status: std::collections::HashMap<String, String>,
/// HSM health status
pub use beardog_types::canonical::HealthStatus;
/// Key operation status
pub struct KeyOperationStatus {
    pub healthy: bool,
    pub operations_tested: Vec<String>,
    pub response_times: serde_json::Value,
/// Endpoint health};


pub struct EndpointHealth {
    pub name: String,
/// Response time metrics
pub struct ResponseTimeMetrics {
    pub average: f64,
    pub p95: f64,
/// EcoPrimal metadata for `BearDog`
pub struct PrimalMetadata {
    /// Primal type identifier
    pub primal_type: PrimalType,
    /// Primal name
    /// Primal version
    pub version: String,
    /// Primal capabilities
    pub capabilities: Vec<PrimalCapability>,
    /// Primal dependencies
    pub dependencies: Vec<PrimalDependency>,
/// Primal type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrimalType {
    /// `BearDog` Security Provider
    `BearDog`,
    /// ToadStool Compute Provider
    ToadStool,
    /// Songbird Service Mesh
    Songbird,
    /// NestGate Storage Provider
    NestGate,
    /// Squirrel AI Coordinator
    Squirrel,
    /// BiomeOS Orchestrator
    BiomeOS,
    /// Custom primal type
    Custom(String),
/// Primal capability enumeration}


pub enum PrimalCapability {
    /// Security capability
    Security,
    /// Compute capability
    Compute,
    /// Storage capability
    Storage,
    /// AI capability
    ArtificialIntelligence,
    /// Networking capability
    Networking,
    /// Monitoring capability
    Monitoring,
    /// Custom capability
/// Primal dependency specification
pub enum PrimalDependency {
    /// Required dependency
    Required {
        /// Primal type
        primal: PrimalType,
        /// Minimum version
        min_version: String,
        /// Reason for dependency
        reason: String,
    },
    /// Optional dependency
    Optional {
/// Primal ecosystem integration configuration
pub struct PrimalIntegrationConfig {
    /// Enable ToadStool integration
    pub enable_toadstool_integration: bool,
    /// Enable Songbird integration
    pub enable_songbird_integration: bool,
    /// Enable Squirrel integration
    pub enable_squirrel_integration: bool,
    /// Enable NestGate integration
    pub enable_nestgate_integration: bool,
    /// Custom configuration
    pub custom_config: HashMap<String, serde_json::Value>,
/// Primal error type
pub struct PrimalError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional error details
    pub details: HashMap<String, serde_json::Value>,
/// Primal request structure
pub struct PrimalRequest {
    /// Request ID
    pub id: String,
    /// Request method
    /// Request parameters
    pub params: HashMap<String, serde_json::Value>,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
/// Primal response structure
pub struct PrimalResponse {
    /// Response ID matching request
    /// Success indicator
    /// Response data
    pub data: Option<serde_json::Value>,
    /// Error information
    pub error: Option<PrimalError>,
    /// Response metadata
    /// Response timestamp}


impl PrimalResponse {
    /// Create successful response}


    pub fn success<T: Serialize>(data: T) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            success: true,
            data: Some(serde_json::to_value(data).unwrap_or(serde_json::Value::Null)),
            error: None,
            metadata: HashMap::new(),
            timestamp: Utc::now(),
        }
    }
    /// Create error response
    pub fn error(error: PrimalError) -> Self {
            success: false,
            data: None,
            error: Some(error),
/// Health status enumeration
impl HealthStatus {
    /// Check if status is healthy}


    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    /// Check if status is unhealthy}


    pub fn is_unhealthy(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy | HealthStatus::Critical)
    /// Check if status is critical
    pub fn is_critical(&self) -> bool {
        matches!(self, HealthStatus::Critical)
/// Primal health check result
pub struct PrimalHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Individual component health
    pub components: HashMap<String, HealthStatus>,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Next health check timestamp
    pub next_check: DateTime<Utc>,
    /// Additional health details
/// Resource usage information
pub struct ResourceUsageInfo {
    /// CPU usage percentage
    pub cpu_percent: f64,
    /// Memory usage in bytes
    pub memory_bytes: u64,
    /// Network bandwidth usage
    pub network_bytes_per_sec: u64,
    /// Disk I/O usage
    pub disk_bytes_per_sec: u64,
/// Primal configuration
pub struct PrimalConfig {
    /// Configuration version
    /// Configuration data
    pub data: HashMap<String, serde_json::Value>,
    /// Configuration metadata
} 
