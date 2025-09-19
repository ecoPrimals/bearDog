

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use bytes::Bytes;
use futures::stream::Stream;
use std::{pin::Pin, sync::atomic::AtomicU64, time::Instant};
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
    /// Number of parallelism
    pub parallelism: usize,

    /// Whether enable_progress_streaming is enabled
    pub enable_progress_streaming: bool,

    /// Number of buffer_pool_size
    pub buffer_pool_size: usize,


    pub enable_bidirectional_streaming: bool,
}
impl Default for ZeroCopyConfig {}

    fn default(1024 * 1024, // 1MB
            parallelism: 4,
            enable_progress_streaming: true,
            buffer_pool_size: 10,
            enable_bidirectional_streaming: true,
        }
    }

pub struct StreamingProgress {

    /// Number of bytes_processed
    pub bytes_processed: u64,

    /// Number of total_bytes
    pub total_bytes: u64,

    /// The bytes per second value
    pub bytes_per_second: f64,

    /// Optional eta seconds
    pub eta_seconds: Option<f64>,

    /// The phase value
    pub phase: StreamingPhase,

    /// Optional error
    pub error: Option<String>,

pub enum StreamingPhase {
    /// Currently starting
    Starting,
    /// Currently processing
    Processing,
    /// Currently finalizing
    Finalizing,
    /// Successful completion state
    Complete,
    /// Error or failure state
    Failed,

pub enum StreamingCommand {


    /// Represents pause variant
    Pause,


    /// Represents resume variant
    Resume,


    /// Represents cancel variant
    Cancel,

    /// Represents set chunk size variant
    SetChunkSize(AtomicU64,
    /// The bytes processed value
    pub bytes_processed: AtomicU64,
    /// The zero copy operations value
    pub zero_copy_operations: AtomicU64,
    /// The buffer reuses value
    pub buffer_reuses: AtomicU64,
    /// The streaming operations value
    pub streaming_operations: AtomicU64,
    /// The avg processing speed mbps value
    pub avg_processing_speed_mbps: AtomicU64, // In MB/s

pub struct EncryptionContext {
    /// The algorithm value
    pub algorithm: String,
    /// The key material value
    pub key_material: Bytes,
    /// The created at value
    pub created_at: Instant,
    /// Number of use
    pub use_count: u64,

pub type ProgressStream = Pin<Box<dyn Stream<Item = StreamingProgress> + Send>>;
pub type CommandReceiver = tokio::sync::mpsc::UnboundedReceiver<StreamingCommand>;
pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<StreamingProgress>;
pub type CommandSender = tokio::sync::mpsc::UnboundedSender<StreamingCommand>;
