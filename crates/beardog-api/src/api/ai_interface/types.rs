//! Core types and data structures for AI interface

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub timestamp: String,
    pub processing_time_ms: u64,
}

/// Structured error response for AI consumption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIError {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
    pub retry_strategy: Option<RetryStrategy>,
    pub context: Option<String>,
}

/// Retry strategy for AI agents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub exponential_backoff: bool,
    pub retry_conditions: Vec<String>,
}

/// System status for AI monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISystemStatus {
    pub overall_status: String,
    pub uptime_seconds: u64,
    pub components: Vec<ComponentStatus>,
    pub performance: PerformanceMetrics,
    pub hsm_status: HSMStatus,
}

/// Component status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub name: String,
    pub status: String,
    pub message: Option<String>,
    pub last_check: String,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub network_connections: u32,
    pub requests_per_second: f64,
    pub average_response_time_ms: f64,
    pub cache_hit_rate: f64,
    pub error_rate: f64,
}

/// HSM status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HSMStatus {
    pub available_tiers: Vec<String>,
    pub active_tier: Option<String>,
    pub tier_performance: HashMap<String, f64>,
    pub tier_availability: HashMap<String, bool>,
    pub failover_status: String,
}

impl<T> AIResponse<T> {
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
        Self {
            success: false,
            data: None,
            error: Some(error),
            metadata: ResponseMetadata {
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                processing_time_ms: 0,
            },
        }
    }
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            exponential_backoff: true,
            retry_conditions: vec!["temporary_failure".to_string(), "rate_limited".to_string()],
        }
    }
}

impl Default for ComponentStatus {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            status: "unknown".to_string(),
            message: None,
            last_check: chrono::Utc::now().to_rfc3339(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_bytes: 0,
            network_connections: 0,
            requests_per_second: 0.0,
            average_response_time_ms: 0.0,
            cache_hit_rate: 0.0,
            error_rate: 0.0,
        }
    }
}

impl Default for HSMStatus {
    fn default() -> Self {
        Self {
            available_tiers: vec!["software".to_string()],
            active_tier: Some("software".to_string()),
            tier_performance: HashMap::new(),
            tier_availability: HashMap::new(),
            failover_status: "active".to_string(),
        }
    }
}
