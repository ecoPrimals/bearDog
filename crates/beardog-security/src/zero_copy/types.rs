// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Zero-Copy Types
//!
//! This module provides types for zero-copy streaming operations,
//! enabling efficient data transfer without unnecessary copying.

use bytes::Bytes;
use futures::stream::Stream;
use std::pin::Pin;
use std::sync::atomic::AtomicU64;
use std::time::Instant;

// ============================================================
// Configuration
// ============================================================

/// Zero-copy configuration
#[derive(Debug, Clone)]
pub struct ZeroCopyConfig {
    /// Maximum chunk size in bytes
    pub max_chunk_size: usize,

    /// Parallelism level
    pub parallelism: usize,

    /// Enable progress streaming
    pub enable_progress_streaming: bool,

    /// Buffer pool size
    pub buffer_pool_size: usize,

    /// Enable bidirectional streaming
    pub enable_bidirectional_streaming: bool,
}

impl Default for ZeroCopyConfig {
    fn default() -> Self {
        Self {
            max_chunk_size: 1024 * 1024, // 1MB
            parallelism: 4,
            enable_progress_streaming: true,
            buffer_pool_size: 10,
            enable_bidirectional_streaming: true,
        }
    }
}

// ============================================================
// Progress Types
// ============================================================

/// Streaming phase
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamingPhase {
    /// Starting phase
    Starting,
    /// Processing phase
    Processing,
    /// Finalizing phase
    Finalizing,
    /// Complete phase
    Complete,
    /// Failed phase
    Failed,
}

impl Default for StreamingPhase {
    fn default() -> Self {
        Self::Starting
    }
}

/// Streaming progress
#[derive(Debug, Clone)]
pub struct StreamingProgress {
    /// Bytes processed
    pub bytes_processed: u64,

    /// Total bytes
    pub total_bytes: u64,

    /// Bytes per second
    pub bytes_per_second: f64,

    /// Estimated time remaining in seconds
    pub eta_seconds: Option<f64>,

    /// Current phase
    pub phase: StreamingPhase,

    /// Error message if any
    pub error: Option<String>,
}

impl Default for StreamingProgress {
    fn default() -> Self {
        Self {
            bytes_processed: 0,
            total_bytes: 0,
            bytes_per_second: 0.0,
            eta_seconds: None,
            phase: StreamingPhase::Starting,
            error: None,
        }
    }
}

// ============================================================
// Command Types
// ============================================================

/// Streaming command
#[derive(Debug, Clone)]
pub enum StreamingCommand {
    /// Pause streaming
    Pause,

    /// Resume streaming
    Resume,

    /// Cancel streaming
    Cancel,

    /// Set chunk size
    SetChunkSize(usize),

    /// Update priority
    SetPriority(u8),
}

// ============================================================
// Metrics Types
// ============================================================

/// Zero-copy metrics
#[derive(Debug)]
pub struct ZeroCopyMetrics {
    /// Total bytes transferred
    pub total_bytes_transferred: AtomicU64,

    /// Bytes processed
    pub bytes_processed: AtomicU64,

    /// Zero-copy operations count
    pub zero_copy_operations: AtomicU64,

    /// Buffer reuse count
    pub buffer_reuses: AtomicU64,

    /// Streaming operations count
    pub streaming_operations: AtomicU64,

    /// Average processing speed in MB/s (fixed-point)
    pub avg_processing_speed_mbps: AtomicU64,
}

impl Default for ZeroCopyMetrics {
    fn default() -> Self {
        Self {
            total_bytes_transferred: AtomicU64::new(0),
            bytes_processed: AtomicU64::new(0),
            zero_copy_operations: AtomicU64::new(0),
            buffer_reuses: AtomicU64::new(0),
            streaming_operations: AtomicU64::new(0),
            avg_processing_speed_mbps: AtomicU64::new(0),
        }
    }
}

// ============================================================
// Encryption Context
// ============================================================

/// Encryption context for streaming
#[derive(Debug, Clone)]
pub struct EncryptionContext {
    /// Algorithm name
    pub algorithm: String,

    /// Key material
    pub key_material: Bytes,

    /// Creation time
    pub created_at: Instant,

    /// Use count
    pub use_count: u64,
}

impl EncryptionContext {
    /// Create a new encryption context
    pub fn new(algorithm: impl Into<String>, key_material: Bytes) -> Self {
        Self {
            algorithm: algorithm.into(),
            key_material,
            created_at: Instant::now(),
            use_count: 0,
        }
    }

    /// Increment use count
    pub fn increment_use(&mut self) {
        self.use_count += 1;
    }

    /// Check if context is expired based on use count
    pub fn is_expired(&self, max_uses: u64) -> bool {
        self.use_count >= max_uses
    }
}

// ============================================================
// Type Aliases
// ============================================================

/// Progress stream type
pub type ProgressStream = Pin<Box<dyn Stream<Item = StreamingProgress> + Send>>;

/// Command receiver type
pub type CommandReceiver = tokio::sync::mpsc::UnboundedReceiver<StreamingCommand>;

/// Progress sender type
pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<StreamingProgress>;

/// Command sender type
pub type CommandSender = tokio::sync::mpsc::UnboundedSender<StreamingCommand>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_copy_config_default() {
        let config = ZeroCopyConfig::default();
        assert_eq!(config.max_chunk_size, 1024 * 1024);
        assert_eq!(config.parallelism, 4);
        assert!(config.enable_progress_streaming);
    }

    #[test]
    fn test_streaming_progress_default() {
        let progress = StreamingProgress::default();
        assert_eq!(progress.bytes_processed, 0);
        assert_eq!(progress.phase, StreamingPhase::Starting);
    }

    #[test]
    fn test_encryption_context() {
        let mut ctx = EncryptionContext::new("AES-256-GCM", Bytes::from_static(&[0u8; 32]));
        assert_eq!(ctx.use_count, 0);
        ctx.increment_use();
        assert_eq!(ctx.use_count, 1);
        assert!(!ctx.is_expired(100));
    }
}
