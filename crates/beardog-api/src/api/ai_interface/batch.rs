//! Batch operations for AI interface

use crate::api::ai_interface::security::AISecurityOperation;
use serde::{Deserialize, Serialize};

/// Batch operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest<T> {
    pub operations: Vec<T>,
    pub options: BatchOptions,
}

/// Batch processing options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOptions {
    pub parallel: bool,
    pub max_concurrent: u32,
    pub timeout_seconds: u64,
    pub continue_on_error: bool,
}

/// Batch operation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResponse<T> {
    pub success: bool,
    pub results: Vec<BatchResult<T>>,
    pub summary: BatchSummary,
}

/// Individual batch result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult<T> {
    pub success: bool,
    pub result: Option<T>,
    pub error: Option<BatchFailure>,
}

/// Individual batch failure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFailure {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

/// Batch processing summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchSummary {
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub processing_time_ms: u64,
    pub average_time_per_operation_ms: f64,
}

/// AI Batch Security Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSecurityRequest {
    pub operations: Vec<AIBatchSecurityOperation>,
}

/// AI Security Operation for batch processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSecurityOperation {
    pub operation: AISecurityOperation,
    pub id: String,
}

/// AI Batch Security Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSecurityResponse {
    pub results: Vec<AIBatchSecurityResult>,
}

/// AI Batch Security Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSecurityResult {
    pub id: String,
    pub success: bool,
    pub result: Option<String>,
    pub error: Option<String>,
}

/// AI Batch Spawn Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSpawnRequest {
    pub spawn_requests: Vec<String>,
}

/// AI Batch Spawn Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSpawnResponse {
    pub results: Vec<AIBatchSpawnResult>,
}

/// AI Batch Spawn Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIBatchSpawnResult {
    pub spawn_id: String,
    pub node_ids: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}

impl Default for BatchOptions {
    fn default() -> Self {
        Self {
            parallel: true,
            max_concurrent: 10,
            timeout_seconds: 30,
            continue_on_error: true,
        }
    }
}

impl Default for BatchSummary {
    fn default() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            processing_time_ms: 0,
            average_time_per_operation_ms: 0.0,
        }
    }
}
