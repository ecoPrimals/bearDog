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


/// Core types and data structures for AI interface

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// AI-optimized API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<AIError>,
    pub metadata: ResponseMetadata,
}
/// Response metadata for AI processing
pub struct ResponseMetadata {
    pub request_id: String,
    pub timestamp: String,
    pub processing_time_ms: u64,
/// Structured error response for AI consumption
pub struct AIError {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
    pub retry_strategy: Option<RetryStrategy>,
    pub context: Option<String>,
/// Retry strategy for AI agents
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub exponential_backoff: bool,
    pub retry_conditions: Vec<String>,
/// System status for AI monitoring
pub struct AISystemStatus {
    pub overall_status: String,
    pub uptime_seconds: u64,
    pub components: Vec<ComponentStatus>,
    pub performance: PerformanceMetrics,
    pub hsm_status: HSMStatus,
/// Component status
pub struct ComponentStatus {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
    pub last_check: String,
/// Performance metrics
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub network_connections: u32,
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
/// HSM status
pub struct HSMStatus {
    pub available_tiers: Vec<String>,
    pub active_tier: Option<String>,
    pub tier_performance: HashMap<String, f64>,
    pub tier_availability: HashMap<String, bool>,
    pub failover_status: String,
}


impl<T> AIResponse<T> {}


    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            metadata: ResponseMetadata {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                processing_time_ms: 0,
            },
        }
    }
    pub fn error(error: AIError) -> Self {
            success: false,
            data: None,
            error: Some(error),
impl Default for RetryStrategy {}


    fn default() -> Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            exponential_backoff: true,
            retry_conditions: vec!["temporary_failure".to_string(), "rate_limited".to_string()],
impl Default for ComponentStatus {
            name: "unknown".to_string(),
            status: "unknown".to_string(),
            message: None,
            last_check: chrono::Utc::now().to_rfc3339(),}


impl Default for PerformanceMetrics {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            network_connections: 0,
            requests_per_second: 0.0,
            average_response_time_ms: 0.0,
            cache_hit_rate: 0.0,
            error_rate: 0.0,
impl Default for HSMStatus {
            available_tiers: vec!["software".to_string()],
            active_tier: Some("software".to_string()),
            tier_performance: HashMap::new(),
            tier_availability: HashMap::new(),
            failover_status: "active".to_string(),
