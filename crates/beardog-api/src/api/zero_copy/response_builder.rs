

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
use tokio::sync::RwLock;

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
    pub content_bytes_served: AtomicU64,}

impl Default for ZeroCopyResponseBuilder {}

    fn default() -> Self {
        Self::new()
    }
impl ZeroCopyResponseBuilder {

    pub fn new() -> Self {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = Arc::new(ZeroCopyJsonSerializer::new(buffer_pool.clone()));
        Self {
            buffer_pool,
            serializer,
            header_cache: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: ZeroCopyResponseStats::default(),
        }

    pub async fn json_response<T: Serialize>(
        &self,
        data: &T,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        self.stats.responses_built.fetch_add(1, Ordering::Relaxed);

        let json_bytes = self.serializer.serialize_zero_copy(data).await?;
        self.stats
            .content_bytes_served
            .fetch_add(json_bytes.len() as u64, Ordering::Relaxed);

        let response = Response::builder()
            .status(status)
            .header("content-type", "application/json")
            .header("content-length", json_bytes.len())
            .body(Body::from(json_bytes))
            .map_err(|e| beardog_errors::BearDogError::internal(format!("Failed to build response: {e)"),
            })?;
        Ok(response)

    pub async fn streaming_response<T: Serialize>(
        data: Vec<T>,
            .streaming_responses
            .fetch_add(1, Ordering::Relaxed);

        let json_bytes = self.serializer.serialize_zero_copy(&data).await?;
            .header("transfer-encoding", "chunked")
                message: format!("Failed to build streaming response: {e}"),

    pub async fn error_response(
        error_code: &str,
        error_message: &str,
        let error_data = serde_json::json!({
            "error": {
                "code": error_code,
                "message": error_message,
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        });
        self.json_response(&error_data, status).await

    pub fn get_stats(&self) -> &ZeroCopyResponseStats {
        &self.stats
