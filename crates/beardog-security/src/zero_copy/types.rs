

use bytes::Bytes;
use futures::stream::Stream;
use std::{pin::Pin, sync::atomic::AtomicU64, time::Instant};
use beardog_errors::BearDogError;

#[derive(Debug, Clone)]
pub struct ZeroCopyConfig {

    pub chunk_size: usize,

    pub parallelism: usize,

    pub enable_progress_streaming: bool,

    pub buffer_pool_size: usize,

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

pub struct StreamingProgress {

    pub bytes_processed: u64,

    pub total_bytes: u64,

    pub bytes_per_second: f64,

    pub eta_seconds: Option<f64>,

    pub phase: StreamingPhase,

    pub error: Option<String>,

pub enum StreamingPhase {
    Starting,
    Processing,
    Finalizing,
    Complete,
    Failed,

pub enum StreamingCommand {

    Pause,

    Resume,

    Cancel,

    SetChunkSize(usize),

    SetParallelism(usize),

#[derive(Debug, Default)]
pub struct ZeroCryptoStats {
    pub operations_total: AtomicU64,
    pub bytes_processed: AtomicU64,
    pub zero_copy_operations: AtomicU64,
    pub buffer_reuses: AtomicU64,
    pub streaming_operations: AtomicU64,
    pub avg_processing_speed_mbps: AtomicU64, // In MB/s

#[derive(Clone)]
pub struct EncryptionContext {
    pub algorithm: String,
    pub key_material: Bytes,
    pub created_at: Instant,
    pub use_count: u64,

pub type ProgressStream = Pin<Box<dyn Stream<Item = StreamingProgress> + Send>>;
pub type CommandReceiver = tokio::sync::mpsc::UnboundedReceiver<StreamingCommand>;
pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<StreamingProgress>;
pub type CommandSender = tokio::sync::mpsc::UnboundedSender<StreamingCommand>;
