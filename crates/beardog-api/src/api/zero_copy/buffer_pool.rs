//! HTTP Buffer Pool for Zero-Copy Operations
//!
//! Provides buffer pooling to reduce memory allocations in hot HTTP paths.

use bytes::BytesMut;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::RwLock;
use tracing::trace;

/// Buffer pool for HTTP request/response handling
pub struct HttpBufferPool {
    /// Small buffers for headers and small payloads
    small_buffers: RwLock<Vec<BytesMut>>,
    /// Medium buffers for typical API responses
    medium_buffers: RwLock<Vec<BytesMut>>,
    /// Large buffers for bulk operations
    large_buffers: RwLock<Vec<BytesMut>>,
    /// Pool statistics
    stats: HttpBufferPoolStats,
}

#[derive(Debug, Default)]
pub struct HttpBufferPoolStats {
    pub small_buffer_hits: AtomicU64,
    pub small_buffer_misses: AtomicU64,
    pub medium_buffer_hits: AtomicU64,
    pub medium_buffer_misses: AtomicU64,
    pub large_buffer_hits: AtomicU64,
    pub large_buffer_misses: AtomicU64,
    pub total_allocations: AtomicU64,
    pub peak_memory_bytes: AtomicU64,
}

impl Default for HttpBufferPool {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpBufferPool {
    /// Create new buffer pool
    pub fn new() -> Self {
        trace!("🔄 Creating HTTP buffer pool for zero-copy operations");
        Self {
            small_buffers: RwLock::new(Vec::with_capacity(10)),
            medium_buffers: RwLock::new(Vec::with_capacity(10)),
            large_buffers: RwLock::new(Vec::with_capacity(5)),
            stats: HttpBufferPoolStats::default(),
        }
    }

    /// Get small buffer (< 4KB)
    pub async fn get_small_buffer(&self) -> BytesMut {
        {
            let mut buffers = self.small_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                self.stats.small_buffer_hits.fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        self.stats
            .small_buffer_misses
            .fetch_add(1, Ordering::Relaxed);
        self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        BytesMut::with_capacity(4096) // 4KB
    }

    /// Get medium buffer (4KB - 64KB)
    pub async fn get_medium_buffer(&self) -> BytesMut {
        {
            let mut buffers = self.medium_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                self.stats
                    .medium_buffer_hits
                    .fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        self.stats
            .medium_buffer_misses
            .fetch_add(1, Ordering::Relaxed);
        self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        BytesMut::with_capacity(65536) // 64KB
    }

    /// Get large buffer (> 64KB)
    pub async fn get_large_buffer(&self) -> BytesMut {
        {
            let mut buffers = self.large_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                self.stats.large_buffer_hits.fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        self.stats
            .large_buffer_misses
            .fetch_add(1, Ordering::Relaxed);
        self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        BytesMut::with_capacity(1024 * 1024) // 1MB
    }

    /// Get buffer of appropriate size
    pub async fn get_buffer(&self, size_hint: usize) -> BytesMut {
        match size_hint {
            0..=4096 => self.get_small_buffer().await,
            4097..=65536 => self.get_medium_buffer().await,
            _ => self.get_large_buffer().await,
        }
    }

    /// Return buffer to pool
    pub async fn return_buffer(&self, buffer: BytesMut) {
        if buffer.is_empty() || buffer.capacity() == 0 {
            return; // Don't pool empty or zero-capacity buffers
        }

        match buffer.capacity() {
            0..=8192 => {
                let mut buffers = self.small_buffers.write().await;
                if buffers.len() < 20 {
                    // Limit pool size
                    buffers.push(buffer);
                }
            }
            8193..=131072 => {
                let mut buffers = self.medium_buffers.write().await;
                if buffers.len() < 15 {
                    // Limit pool size
                    buffers.push(buffer);
                }
            }
            _ => {
                let mut buffers = self.large_buffers.write().await;
                if buffers.len() < 10 {
                    // Limit pool size
                    buffers.push(buffer);
                }
            }
        }
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> &HttpBufferPoolStats {
        &self.stats
    }

    /// Get pool efficiency metrics
    pub fn get_efficiency_metrics(&self) -> PoolEfficiencyMetrics {
        let small_hits = self.stats.small_buffer_hits.load(Ordering::Relaxed);
        let small_misses = self.stats.small_buffer_misses.load(Ordering::Relaxed);
        let medium_hits = self.stats.medium_buffer_hits.load(Ordering::Relaxed);
        let medium_misses = self.stats.medium_buffer_misses.load(Ordering::Relaxed);
        let large_hits = self.stats.large_buffer_hits.load(Ordering::Relaxed);
        let large_misses = self.stats.large_buffer_misses.load(Ordering::Relaxed);

        let total_hits = small_hits + medium_hits + large_hits;
        let total_requests = total_hits + small_misses + medium_misses + large_misses;

        let hit_rate = if total_requests > 0 {
            total_hits as f64 / total_requests as f64
        } else {
            0.0
        };

        PoolEfficiencyMetrics {
            hit_rate,
            total_allocations: self.stats.total_allocations.load(Ordering::Relaxed),
            peak_memory_bytes: self.stats.peak_memory_bytes.load(Ordering::Relaxed),
        }
    }
}

/// Pool efficiency metrics
#[derive(Debug, Clone)]
pub struct PoolEfficiencyMetrics {
    pub hit_rate: f64,
    pub total_allocations: u64,
    pub peak_memory_bytes: u64,
}
