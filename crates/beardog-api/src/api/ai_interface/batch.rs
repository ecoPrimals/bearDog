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


/// Batch operations for AI interface

use crate::api::ai_interface::security::AISecurityOperation;
use serde::{Deserialize, Serialize};
/// Batch operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRequest<T> {
    pub operations: Vec<T>,
    pub options: BatchOptions,
}
/// Batch processing options
pub struct BatchOptions {
    pub parallel: bool,
    pub max_concurrent: u32,
    pub timeout_seconds: u64,
    pub continue_on_error: bool,
/// Batch operation response
pub struct BatchResponse<T> {
    pub success: bool,
    pub results: Vec<BatchResult<T>>,
    pub summary: BatchSummary,
/// Individual batch result
pub struct BatchResult<T> {
    pub result: Option<T>,
    pub error: Option<BatchFailure>,
/// Individual batch failure
pub struct BatchFailure {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
/// Batch processing summary
pub struct BatchSummary {
    pub total_operations: u32,
    pub successful_operations: u32,
    pub failed_operations: u32,
    pub processing_time_ms: u64,
    pub average_time_per_operation_ms: f64,
/// AI Batch Security Request
pub struct AIBatchSecurityRequest {
    pub operations: Vec<AIBatchSecurityOperation>,
/// AI Security Operation for batch processing
pub struct AIBatchSecurityOperation {
    pub operation: AISecurityOperation,
    pub id: String,
/// AI Batch Security Response
pub struct AIBatchSecurityResponse {
    pub results: Vec<AIBatchSecurityResult>,
/// AI Batch Security Result
pub struct AIBatchSecurityResult {
    pub result: Option<String>,
    pub error: Option<String>,
/// AI Batch Spawn Request
pub struct AIBatchSpawnRequest {
    pub spawn_requests: Vec<String>,
/// AI Batch Spawn Response
pub struct AIBatchSpawnResponse {
    pub results: Vec<AIBatchSpawnResult>,
/// AI Batch Spawn Result
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
