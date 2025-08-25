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


/// Core Types for Universal HSM Adapter
///
/// Defines the fundamental types used throughout the universal adapter system

use beardog_errors::{BearDogError, BearDogResult};
use chrono;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
// HSM Discovery imports
use crate::universal_hsm_discovery::{
    DiscoveredHsm, EntropyCollectionMethod, HsmHealthStatus, HsmInterfaceType, HsmTier,
    PerformanceCapabilities,
};
#[derive(Debug, Clone)]
pub struct HsmConnection {
    pub hsm_id: String,
    pub interface_type: HsmInterfaceType,
    pub connection_handle: String,
    pub authentication_status: AuthenticationStatus,
    pub capabilities: PerformanceCapabilities,
    pub established_at: chrono::DateTime<chrono::Utc>,
}
pub enum AuthenticationStatus {
    Unauthenticated,
    BiometricRequired,
    Authenticated,}


pub struct UniversalOperation {
    pub operation_type: OperationType,
    pub parameters: HashMap<String, String>,
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OperationType {
    GenerateKey,
    Sign,
    Verify,
    Encrypt,
    Decrypt,
    HumanEntropyGeneration,
}


pub struct OperationResult {
    pub success: bool,
    pub result_data: Vec<u8>,
    pub metadata: HashMap<String, String>,
    pub performance_metrics: PerformanceMetrics,
pub struct PerformanceMetrics {
    pub duration_ms: f64,
    pub throughput_bps: Option<f64>,
    pub hsm_latency_ms: f64,
    pub error_count: u64,
}


pub struct HealthStatus {
    pub is_healthy: bool,
    pub response_time_ms: f64,
    pub error_message: Option<String>,
    pub last_check: chrono::DateTime<chrono::Utc>,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyRequirements {
    pub minimum_entropy_bits: u32,
    pub collection_timeout_seconds: u32,
}


pub struct EphemeralSeed {
    pub seed_data: Vec<u8>,
    pub entropy_estimate: f64,
    pub creation_timestamp: chrono::DateTime<chrono::Utc>,
