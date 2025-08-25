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


/// Types and configuration for zero-copy cryptographic operations

use bytes::Bytes;
use futures::stream::Stream;
use std::{pin::Pin, sync::atomic::AtomicU64, time::Instant};
use beardog_errors::{BearDogError, BearDogResult};
/// Configuration for zero-copy operations
#[derive(Debug, Clone)]
pub struct ZeroCopyConfig {
    /// Chunk size for streaming operations (default: 1MB)
    pub chunk_size: usize,
    /// Number of concurrent chunks to process
    pub parallelism: usize,
    /// Enable progress reporting for tRPC streaming
    pub enable_progress_streaming: bool,
    /// Buffer pool size per size class
    pub buffer_pool_size: usize,
    /// Enable bidirectional streaming
    pub enable_bidirectional_streaming: bool,
}
impl Default for ZeroCopyConfig {}


    fn default() -> Self {
        Self {
            chunk_size: 1024 * 1024, // 1MB
            parallelism: 4,
            enable_progress_streaming: true,
            buffer_pool_size: 10,
            enable_bidirectional_streaming: true,
        }
    }
/// Progress information for streaming operations
pub struct StreamingProgress {
    /// Bytes processed so far
    pub bytes_processed: u64,
    /// Total bytes to process
    pub total_bytes: u64,
    /// Processing speed in bytes/second
    pub bytes_per_second: f64,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<f64>,
    /// Current operation phase
    pub phase: StreamingPhase,
    /// Optional error if operation failed
    pub error: Option<String>,
/// Phase of streaming operation
pub enum StreamingPhase {
    Starting,
    Processing,
    Finalizing,
    Complete,
    Failed,
/// Command for bidirectional streaming control}


pub enum StreamingCommand {
    /// Pause the operation
    Pause,
    /// Resume the operation
    Resume,
    /// Cancel the operation
    Cancel,
    /// Change chunk size dynamically
    SetChunkSize(usize),
    /// Change parallelism dynamically
    SetParallelism(usize),
/// Statistics for zero-copy operations
#[derive(Debug, Default)]
pub struct ZeroCryptoStats {
    pub operations_total: AtomicU64,
    pub bytes_processed: AtomicU64,
    pub zero_copy_operations: AtomicU64,
    pub buffer_reuses: AtomicU64,
    pub streaming_operations: AtomicU64,
    pub avg_processing_speed_mbps: AtomicU64, // In MB/s
/// Context for cached encryption operations
#[derive(Clone)]
pub struct EncryptionContext {
    pub algorithm: String,
    pub key_material: Bytes,
    pub created_at: Instant,
    pub use_count: u64,
/// Type aliases for streaming operations
pub type ProgressStream = Pin<Box<dyn Stream<Item = StreamingProgress> + Send>>;
pub type CommandReceiver = tokio::sync::mpsc::UnboundedReceiver<StreamingCommand>;
pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<StreamingProgress>;
pub type CommandSender = tokio::sync::mpsc::UnboundedSender<StreamingCommand>;
