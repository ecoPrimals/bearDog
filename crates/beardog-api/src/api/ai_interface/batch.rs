

use crate::api::ai_interface::security::AISecurityOperation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest<T> {
    pub operations: Vec<T>,
    pub options: BatchOptions,
}

pub struct BatchOptions {
    pub parallel: bool,
    pub max_concurrent: u32,
    pub timeout_seconds: u64,
    pub continue_on_error: bool,

pub struct BatchResponse<T> {
    pub success: bool,
    pub results: Vec<BatchResult<T>>,
    pub summary: BatchSummary,

pub struct BatchResult<T> {
    pub result: Option<T>,
    pub error: Option<BatchFailure>,

pub struct BatchFailure {
    pub code: String,
    pub message: String,
    pub details: Option<String>,

pub struct BatchSummary {
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub processing_time_ms: u64,
    pub average_time_per_operation_ms: f64,

pub struct AIBatchSecurityRequest {
    pub operations: Vec<AIBatchSecurityOperation>,

pub struct AIBatchSecurityOperation {
    pub operation: AISecurityOperation,
    pub id: String,

pub struct AIBatchSecurityResponse {
    pub results: Vec<AIBatchSecurityResult>,

pub struct AIBatchSecurityResult {
    pub result: Option<String>,
    pub error: Option<String>,

pub struct AIBatchSpawnRequest {
    pub spawn_requests: Vec<String>,

pub struct AIBatchSpawnResponse {
    pub results: Vec<AIBatchSpawnResult>,

pub struct AIBatchSpawnResult {
    pub spawn_id: String,
    pub node_ids: Vec<String>,}

impl Default for BatchOptions {}

    fn default() -> Self {
        Self {
            parallel: true,
            max_concurrent: 10,
            timeout_seconds: 30,
            continue_on_error: true,
        }
    }
impl Default for BatchSummary {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            processing_time_ms: 0,
            average_time_per_operation_ms: 0.0,
