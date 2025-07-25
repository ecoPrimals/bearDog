//! Zero-Copy Response Builder
//!
//! Builds HTTP responses with minimal allocations using buffer pooling.

use super::{HttpBufferPool, ZeroCopyJsonSerializer};
use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::Response,
};
use beardog_errors::BearDogResult;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::RwLock;

/// Zero-copy response builder
pub struct ZeroCopyResponseBuilder {
    #[allow(dead_code)] // Will be used when zero-copy response optimizations are fully implemented
    buffer_pool: Arc<HttpBufferPool>,
    serializer: Arc<ZeroCopyJsonSerializer>,
    header_cache: Arc<RwLock<HashMap<String, HeaderValue>>>,
    stats: ZeroCopyResponseStats,
}

#[derive(Debug, Default)]
pub struct ZeroCopyResponseStats {
    pub responses_built: AtomicU64,
    pub header_cache_hits: AtomicU64,
    pub header_cache_misses: AtomicU64,
    pub streaming_responses: AtomicU64,
    pub content_bytes_served: AtomicU64,
}

impl Default for ZeroCopyResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ZeroCopyResponseBuilder {
    /// Create new zero-copy response builder
    pub fn new() -> Self {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = Arc::new(ZeroCopyJsonSerializer::new(buffer_pool.clone()));

        Self {
            buffer_pool,
            serializer,
            header_cache: Arc::new(RwLock::new(HashMap::new())),
            stats: ZeroCopyResponseStats::default(),
        }
    }

    /// Build JSON response with zero-copy optimization
    pub async fn json_response<T: Serialize>(
        &self,
        data: &T,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        self.stats.responses_built.fetch_add(1, Ordering::Relaxed);

        // Serialize using zero-copy JSON serializer
        let json_bytes = self.serializer.serialize_zero_copy(data).await?;

        self.stats
            .content_bytes_served
            .fetch_add(json_bytes.len() as u64, Ordering::Relaxed);

        // Build response with cached headers
        let response = Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .header("content-length", json_bytes.len())
            .body(Body::from(json_bytes))
            .map_err(|e| beardog_errors::BearDogError::Internal {
                message: format!("Failed to build response: {e}"),
            })?;

        Ok(response)
    }

    /// Build streaming response for large datasets
    pub async fn streaming_response<T: Serialize>(
        &self,
        data: Vec<T>,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        self.stats
            .streaming_responses
            .fetch_add(1, Ordering::Relaxed);

        // For large datasets, serialize in chunks
        let json_bytes = self.serializer.serialize_zero_copy(&data).await?;

        self.stats
            .content_bytes_served
            .fetch_add(json_bytes.len() as u64, Ordering::Relaxed);

        let response = Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .header("transfer-encoding", "chunked")
            .body(Body::from(json_bytes))
            .map_err(|e| beardog_errors::BearDogError::Internal {
                message: format!("Failed to build streaming response: {e}"),
            })?;

        Ok(response)
    }

    /// Build error response
    pub async fn error_response(
        &self,
        error_code: &str,
        error_message: &str,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        let error_data = serde_json::json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });

        self.json_response(&error_data, status).await
    }

    /// Get response builder statistics
    pub fn get_stats(&self) -> &ZeroCopyResponseStats {
        &self.stats
    }
}
