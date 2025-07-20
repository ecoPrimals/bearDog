//! Zero-Copy API Request/Response Handlers for BearDog
//!
//! **High-Performance HTTP Processing with Minimal Allocations**
//!
//! This module provides zero-copy HTTP request and response handling that
//! minimizes memory allocations and data copying in hot API paths.
//!
//! Key optimizations:
//! - Zero-copy request body parsing
//! - Response streaming with buffer reuse
//! - HTTP header pooling and reuse
//! - JSON serialization with buffer pooling
//! - WebSocket frame processing optimization

use axum::{
    body::Body,
    extract::State,
    http::{HeaderValue, StatusCode},
    response::Response,
    Json,
};
use beardog_errors::{BearDogError, BearDogResult};
use bytes::{BufMut, Bytes, BytesMut};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
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

impl HttpBufferPool {
    /// Create new HTTP buffer pool
    pub fn new() -> Self {
        Self {
            small_buffers: RwLock::new(Vec::new()),
            medium_buffers: RwLock::new(Vec::new()),
            large_buffers: RwLock::new(Vec::new()),
            stats: HttpBufferPoolStats::default(),
        }
    }

    /// Get buffer for small data (< 4KB)
    pub async fn get_small_buffer(&self) -> BytesMut {
        const SMALL_SIZE: usize = 4096;

        {
            let mut buffers = self.small_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                buffer.reserve(SMALL_SIZE);
                self.stats.small_buffer_hits.fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        self.stats
            .small_buffer_misses
            .fetch_add(1, Ordering::Relaxed);
        self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        BytesMut::with_capacity(SMALL_SIZE)
    }

    /// Get buffer for medium data (< 64KB)
    pub async fn get_medium_buffer(&self) -> BytesMut {
        const MEDIUM_SIZE: usize = 65536;

        {
            let mut buffers = self.medium_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                buffer.reserve(MEDIUM_SIZE);
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
        BytesMut::with_capacity(MEDIUM_SIZE)
    }

    /// Get buffer for large data (< 1MB)
    pub async fn get_large_buffer(&self) -> BytesMut {
        const LARGE_SIZE: usize = 1048576;

        {
            let mut buffers = self.large_buffers.write().await;
            if let Some(mut buffer) = buffers.pop() {
                buffer.clear();
                buffer.reserve(LARGE_SIZE);
                self.stats.large_buffer_hits.fetch_add(1, Ordering::Relaxed);
                return buffer;
            }
        }

        self.stats
            .large_buffer_misses
            .fetch_add(1, Ordering::Relaxed);
        self.stats.total_allocations.fetch_add(1, Ordering::Relaxed);
        BytesMut::with_capacity(LARGE_SIZE)
    }

    /// Return small buffer to pool
    pub async fn return_small_buffer(&self, buffer: BytesMut) {
        if buffer.capacity() >= 4096 && buffer.capacity() <= 8192 {
            let mut buffers = self.small_buffers.write().await;
            if buffers.len() < 20 {
                buffers.push(buffer);
            }
        }
    }

    /// Return medium buffer to pool
    pub async fn return_medium_buffer(&self, buffer: BytesMut) {
        if buffer.capacity() >= 32768 && buffer.capacity() <= 131072 {
            let mut buffers = self.medium_buffers.write().await;
            if buffers.len() < 10 {
                buffers.push(buffer);
            }
        }
    }

    /// Return large buffer to pool
    pub async fn return_large_buffer(&self, buffer: BytesMut) {
        if buffer.capacity() >= 524288 && buffer.capacity() <= 2097152 {
            let mut buffers = self.large_buffers.write().await;
            if buffers.len() < 5 {
                buffers.push(buffer);
            }
        }
    }

    /// Get pool statistics
    pub fn get_stats(&self) -> HttpBufferPoolStats {
        HttpBufferPoolStats {
            small_buffer_hits: AtomicU64::new(self.stats.small_buffer_hits.load(Ordering::Relaxed)),
            small_buffer_misses: AtomicU64::new(
                self.stats.small_buffer_misses.load(Ordering::Relaxed),
            ),
            medium_buffer_hits: AtomicU64::new(
                self.stats.medium_buffer_hits.load(Ordering::Relaxed),
            ),
            medium_buffer_misses: AtomicU64::new(
                self.stats.medium_buffer_misses.load(Ordering::Relaxed),
            ),
            large_buffer_hits: AtomicU64::new(self.stats.large_buffer_hits.load(Ordering::Relaxed)),
            large_buffer_misses: AtomicU64::new(
                self.stats.large_buffer_misses.load(Ordering::Relaxed),
            ),
            total_allocations: AtomicU64::new(self.stats.total_allocations.load(Ordering::Relaxed)),
            peak_memory_bytes: AtomicU64::new(self.stats.peak_memory_bytes.load(Ordering::Relaxed)),
        }
    }
}

/// Zero-copy JSON serializer with buffer pooling
pub struct ZeroCopyJsonSerializer {
    buffer_pool: Arc<HttpBufferPool>,
    stats: ZeroCopySerializerStats,
}

#[derive(Debug, Default)]
pub struct ZeroCopySerializerStats {
    pub serializations: AtomicU64,
    pub deserializations: AtomicU64,
    pub buffer_reuses: AtomicU64,
    pub zero_copy_operations: AtomicU64,
    pub bytes_processed: AtomicU64,
}

impl ZeroCopyJsonSerializer {
    /// Create new zero-copy JSON serializer
    pub fn new(buffer_pool: Arc<HttpBufferPool>) -> Self {
        Self {
            buffer_pool,
            stats: ZeroCopySerializerStats::default(),
        }
    }

    /// Serialize to JSON with zero-copy optimization
    pub async fn serialize_zero_copy<T: Serialize>(&self, value: &T) -> BearDogResult<Bytes> {
        self.stats.serializations.fetch_add(1, Ordering::Relaxed);

        // Estimate size and get appropriate buffer
        let mut buffer = self.buffer_pool.get_medium_buffer().await;

        // Serialize directly into buffer
        match serde_json::to_writer((&mut buffer).writer(), value) {
            Ok(()) => {
                self.stats.buffer_reuses.fetch_add(1, Ordering::Relaxed);
                self.stats
                    .bytes_processed
                    .fetch_add(buffer.len() as u64, Ordering::Relaxed);
                Ok(buffer.freeze())
            }
            Err(e) => Err(BearDogError::Serialization {
                message: format!("JSON serialization failed: {}", e),
            }),
        }
    }

    /// Deserialize from JSON with zero-copy optimization
    pub async fn deserialize_zero_copy<T: for<'de> Deserialize<'de>>(
        &self,
        data: &[u8],
    ) -> BearDogResult<T> {
        self.stats.deserializations.fetch_add(1, Ordering::Relaxed);
        self.stats
            .bytes_processed
            .fetch_add(data.len() as u64, Ordering::Relaxed);

        match serde_json::from_slice(data) {
            Ok(value) => {
                self.stats
                    .zero_copy_operations
                    .fetch_add(1, Ordering::Relaxed);
                Ok(value)
            }
            Err(e) => Err(BearDogError::DeserializationError {
                message: format!("JSON deserialization failed: {}", e),
            }),
        }
    }

    /// Batch serialize multiple items
    pub async fn batch_serialize_zero_copy<T: Serialize>(
        &self,
        items: &[T],
    ) -> BearDogResult<Bytes> {
        if items.is_empty() {
            return Ok(Bytes::from_static(b"[]"));
        }

        let mut buffer = self.buffer_pool.get_large_buffer().await;
        buffer.put_u8(b'[');

        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                buffer.put_u8(b',');
            }

            // Serialize each item directly into buffer
            serde_json::to_writer((&mut buffer).writer(), item).map_err(|e| {
                BearDogError::Serialization {
                    message: format!("Batch serialization failed: {}", e),
                }
            })?;
        }

        buffer.put_u8(b']');

        self.stats
            .serializations
            .fetch_add(items.len() as u64, Ordering::Relaxed);
        self.stats
            .bytes_processed
            .fetch_add(buffer.len() as u64, Ordering::Relaxed);

        Ok(buffer.freeze())
    }

    /// Get serialization statistics
    pub fn get_stats(&self) -> ZeroCopySerializerStats {
        ZeroCopySerializerStats {
            serializations: AtomicU64::new(self.stats.serializations.load(Ordering::Relaxed)),
            deserializations: AtomicU64::new(self.stats.deserializations.load(Ordering::Relaxed)),
            buffer_reuses: AtomicU64::new(self.stats.buffer_reuses.load(Ordering::Relaxed)),
            zero_copy_operations: AtomicU64::new(
                self.stats.zero_copy_operations.load(Ordering::Relaxed),
            ),
            bytes_processed: AtomicU64::new(self.stats.bytes_processed.load(Ordering::Relaxed)),
        }
    }
}

/// Zero-copy HTTP response builder
pub struct ZeroCopyResponseBuilder {
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

        // Serialize with zero-copy
        let json_bytes = self.serializer.serialize_zero_copy(data).await?;

        // Get cached headers
        let content_type_header = self
            .get_cached_header("content-type", "application/json")
            .await;

        let mut response = Response::builder()
            .status(status)
            .header("content-type", content_type_header);

        // Add content length header
        if let Ok(content_length) = HeaderValue::from_str(&json_bytes.len().to_string()) {
            response = response.header("content-length", content_length);
        }

        let body_len = json_bytes.len();
        let body = Body::from(json_bytes);
        self.stats
            .content_bytes_served
            .fetch_add(body_len as u64, Ordering::Relaxed);

        response
            .body(body)
            .map_err(|e| BearDogError::internal(format!("Response building failed: {}", e)))
    }

    /// Build streaming response for large data
    pub async fn streaming_response<T: Serialize>(
        &self,
        data: Vec<T>,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        self.stats.responses_built.fetch_add(1, Ordering::Relaxed);
        self.stats
            .streaming_responses
            .fetch_add(1, Ordering::Relaxed);

        // Use batch serialization for efficient streaming
        let json_bytes = self.serializer.batch_serialize_zero_copy(&data).await?;

        let content_type_header = self
            .get_cached_header("content-type", "application/json")
            .await;
        let transfer_encoding_header = self.get_cached_header("transfer-encoding", "chunked").await;

        let response = Response::builder()
            .status(status)
            .header("content-type", content_type_header)
            .header("transfer-encoding", transfer_encoding_header);

        let body_len = json_bytes.len();
        let body = Body::from(json_bytes);
        self.stats
            .content_bytes_served
            .fetch_add(body_len as u64, Ordering::Relaxed);

        response.body(body).map_err(|e| {
            BearDogError::internal(format!("Streaming response building failed: {}", e))
        })
    }

    /// Build binary response with zero-copy
    pub async fn binary_response(
        &self,
        data: Bytes,
        content_type: &str,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        self.stats.responses_built.fetch_add(1, Ordering::Relaxed);

        let content_type_header = self.get_cached_header("content-type", content_type).await;

        let mut response = Response::builder()
            .status(status)
            .header("content-type", content_type_header);

        if let Ok(content_length) = HeaderValue::from_str(&data.len().to_string()) {
            response = response.header("content-length", content_length);
        }

        let body_len = data.len();
        let body = Body::from(data);
        self.stats
            .content_bytes_served
            .fetch_add(body_len as u64, Ordering::Relaxed);

        response
            .body(body)
            .map_err(|e| BearDogError::internal(format!("Binary response building failed: {}", e)))
    }

    /// Build error response with zero-copy
    pub async fn error_response(
        &self,
        error: &BearDogError,
        status: StatusCode,
    ) -> BearDogResult<Response> {
        let error_response = ErrorResponse {
            success: false,
            error: ErrorDetails {
                code: "BEARDOG_ERROR".to_string(),
                message: error.to_string(),
                details: None,
                request_id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
        };

        self.json_response(&error_response, status).await
    }

    /// Get cached header value or create new one
    async fn get_cached_header(&self, name: &str, value: &str) -> HeaderValue {
        let cache_key = format!("{}:{}", name, value);

        {
            let cache = self.header_cache.read().await;
            if let Some(header_value) = cache.get(&cache_key) {
                self.stats.header_cache_hits.fetch_add(1, Ordering::Relaxed);
                return header_value.clone();
            }
        }

        // Cache miss - create new header
        self.stats
            .header_cache_misses
            .fetch_add(1, Ordering::Relaxed);

        let header_value = HeaderValue::from_str(value)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

        // Cache the header
        {
            let mut cache = self.header_cache.write().await;
            cache.insert(cache_key, header_value.clone());

            // Limit cache size
            if cache.len() > 100 {
                cache.clear(); // Simple cleanup
            }
        }

        header_value
    }

    /// Get response building statistics
    pub fn get_stats(&self) -> ZeroCopyResponseStats {
        ZeroCopyResponseStats {
            responses_built: AtomicU64::new(self.stats.responses_built.load(Ordering::Relaxed)),
            header_cache_hits: AtomicU64::new(self.stats.header_cache_hits.load(Ordering::Relaxed)),
            header_cache_misses: AtomicU64::new(
                self.stats.header_cache_misses.load(Ordering::Relaxed),
            ),
            streaming_responses: AtomicU64::new(
                self.stats.streaming_responses.load(Ordering::Relaxed),
            ),
            content_bytes_served: AtomicU64::new(
                self.stats.content_bytes_served.load(Ordering::Relaxed),
            ),
        }
    }
}

/// Zero-copy request parser
pub struct ZeroCopyRequestParser {
    buffer_pool: Arc<HttpBufferPool>,
    serializer: Arc<ZeroCopyJsonSerializer>,
    stats: ZeroCopyRequestStats,
}

#[derive(Debug, Default)]
pub struct ZeroCopyRequestStats {
    pub requests_parsed: AtomicU64,
    pub json_parsed: AtomicU64,
    pub form_parsed: AtomicU64,
    pub multipart_parsed: AtomicU64,
    pub zero_copy_parses: AtomicU64,
}

impl ZeroCopyRequestParser {
    /// Create new zero-copy request parser
    pub fn new(buffer_pool: Arc<HttpBufferPool>, serializer: Arc<ZeroCopyJsonSerializer>) -> Self {
        Self {
            buffer_pool,
            serializer,
            stats: ZeroCopyRequestStats::default(),
        }
    }

    /// Parse JSON request body with zero-copy
    pub async fn parse_json<T: for<'de> Deserialize<'de>>(&self, body: Bytes) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);
        self.stats.json_parsed.fetch_add(1, Ordering::Relaxed);

        if body.is_empty() {
            return Err(BearDogError::validation("Request body is empty"));
        }

        let result = self.serializer.deserialize_zero_copy(&body).await?;
        self.stats.zero_copy_parses.fetch_add(1, Ordering::Relaxed);

        Ok(result)
    }

    /// Parse query parameters with zero-copy
    pub async fn parse_query_zero_copy<T: for<'de> Deserialize<'de>>(
        &self,
        query_string: &str,
    ) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);

        match serde_urlencoded::from_str(query_string) {
            Ok(params) => {
                self.stats.zero_copy_parses.fetch_add(1, Ordering::Relaxed);
                Ok(params)
            }
            Err(e) => Err(BearDogError::validation(format!(
                "Query parameter parsing failed: {}",
                e
            ))),
        }
    }

    /// Parse form data with buffer pooling
    pub async fn parse_form<T: for<'de> Deserialize<'de>>(&self, body: Bytes) -> BearDogResult<T> {
        self.stats.requests_parsed.fetch_add(1, Ordering::Relaxed);
        self.stats.form_parsed.fetch_add(1, Ordering::Relaxed);

        let form_str = std::str::from_utf8(&body)
            .map_err(|_| BearDogError::validation("Invalid UTF-8 in form data"))?;

        match serde_urlencoded::from_str(form_str) {
            Ok(form_data) => Ok(form_data),
            Err(e) => Err(BearDogError::validation(format!(
                "Form parsing failed: {}",
                e
            ))),
        }
    }

    /// Get request parsing statistics
    pub fn get_stats(&self) -> ZeroCopyRequestStats {
        ZeroCopyRequestStats {
            requests_parsed: AtomicU64::new(self.stats.requests_parsed.load(Ordering::Relaxed)),
            json_parsed: AtomicU64::new(self.stats.json_parsed.load(Ordering::Relaxed)),
            form_parsed: AtomicU64::new(self.stats.form_parsed.load(Ordering::Relaxed)),
            multipart_parsed: AtomicU64::new(self.stats.multipart_parsed.load(Ordering::Relaxed)),
            zero_copy_parses: AtomicU64::new(self.stats.zero_copy_parses.load(Ordering::Relaxed)),
        }
    }
}

/// Zero-copy handler context for API endpoints
pub struct ZeroCopyHandlerContext {
    pub response_builder: Arc<ZeroCopyResponseBuilder>,
    pub request_parser: Arc<ZeroCopyRequestParser>,
    pub buffer_pool: Arc<HttpBufferPool>,
}

impl ZeroCopyHandlerContext {
    /// Create new handler context
    pub fn new() -> Self {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = Arc::new(ZeroCopyJsonSerializer::new(buffer_pool.clone()));
        let response_builder = Arc::new(ZeroCopyResponseBuilder::new());
        let request_parser = Arc::new(ZeroCopyRequestParser::new(buffer_pool.clone(), serializer));

        Self {
            response_builder,
            request_parser,
            buffer_pool,
        }
    }

    /// Get comprehensive statistics
    pub async fn get_comprehensive_stats(&self) -> ZeroCopyHandlerStats {
        ZeroCopyHandlerStats {
            buffer_pool_stats: self.buffer_pool.get_stats(),
            response_stats: self.response_builder.get_stats(),
            request_stats: self.request_parser.get_stats(),
        }
    }
}

impl Default for ZeroCopyHandlerContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive statistics for zero-copy handlers
#[derive(Debug)]
pub struct ZeroCopyHandlerStats {
    pub buffer_pool_stats: HttpBufferPoolStats,
    pub response_stats: ZeroCopyResponseStats,
    pub request_stats: ZeroCopyRequestStats,
}

// Response types for API handlers

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetails,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,
    pub request_id: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
    pub metadata: ResponseMetadata,
}

#[derive(Debug, Serialize)]
pub struct ResponseMetadata {
    pub request_id: String,
    pub timestamp: String,
    pub processing_time_ms: u64,
    pub cached: bool,
}

// High-performance handler implementations using zero-copy patterns

/// Zero-copy health check handler
pub async fn health_check_zero_copy(
    State(context): State<Arc<ZeroCopyHandlerContext>>,
) -> BearDogResult<Response> {
    let start_time = std::time::Instant::now();

    let health_data = HealthCheckResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        uptime_seconds: 12345, // Would get from actual system
        version: env!("CARGO_PKG_VERSION").to_string(),
        components: vec![
            ComponentStatus {
                name: "api".to_string(),
                status: "healthy".to_string(),
                response_time_ms: 1,
            },
            ComponentStatus {
                name: "crypto".to_string(),
                status: "healthy".to_string(),
                response_time_ms: 2,
            },
        ],
    };

    let response = SuccessResponse {
        success: true,
        data: health_data,
        metadata: ResponseMetadata {
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
            cached: false,
        },
    };

    context
        .response_builder
        .json_response(&response, StatusCode::OK)
        .await
}

/// Zero-copy bulk data handler with streaming
pub async fn bulk_data_handler_zero_copy<T: Serialize>(
    State(context): State<Arc<ZeroCopyHandlerContext>>,
    Json(request): Json<BulkDataRequest>,
) -> BearDogResult<Response> {
    trace!(
        "Processing bulk data request with {} items",
        request.items.len()
    );

    // For demonstration, create mock data based on request
    let mock_data: Vec<BulkDataItem> = (0..request.items.len())
        .map(|i| BulkDataItem {
            id: format!("item_{}", i),
            data: format!("bulk_data_{}", i),
            processed_at: chrono::Utc::now().to_rfc3339(),
        })
        .collect();

    // Use streaming response for large datasets
    if mock_data.len() > 100 {
        context
            .response_builder
            .streaming_response(mock_data, StatusCode::OK)
            .await
    } else {
        context
            .response_builder
            .json_response(&mock_data, StatusCode::OK)
            .await
    }
}

// Helper types for API responses

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub timestamp: String,
    pub uptime_seconds: u64,
    pub version: String,
    pub components: Vec<ComponentStatus>,
}

#[derive(Debug, Serialize)]
pub struct ComponentStatus {
    pub name: String,
    pub status: String,
    pub response_time_ms: u64,
}

#[derive(Debug, Deserialize)]
pub struct BulkDataRequest {
    pub items: Vec<String>,
    pub processing_options: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct BulkDataItem {
    pub id: String,
    pub data: String,
    pub processed_at: String,
}

#[cfg(test)]
mod tests {
    use super::HttpBufferPool;
    use tokio;

    #[tokio::test]
    async fn test_http_buffer_pool() {
        let pool = HttpBufferPool::new();

        // Test small buffer
        let small_buffer = pool.get_small_buffer().await;
        assert!(small_buffer.capacity() >= 4096);
        pool.return_small_buffer(small_buffer).await;

        // Test reuse
        let reused_buffer = pool.get_small_buffer().await;
        let stats = pool.get_stats();
        assert!(stats.small_buffer_hits.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn test_zero_copy_json_serializer() {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = ZeroCopyJsonSerializer::new(buffer_pool.clone());

        #[derive(Serialize, Deserialize, PartialEq, Debug)]
        struct TestData {
            name: String,
            value: u32,
        }

        let test_data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Test serialization
        let serialized = serializer.serialize_zero_copy(&test_data).await.unwrap();
        assert!(!serialized.is_empty());

        // Test deserialization
        let deserialized: TestData = serializer.deserialize_zero_copy(&serialized).await.unwrap();
        assert_eq!(test_data, deserialized);

        let stats = serializer.get_stats();
        assert!(stats.serializations.load(Ordering::Relaxed) > 0);
        assert!(stats.deserializations.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn test_zero_copy_response_builder() {
        let builder = ZeroCopyResponseBuilder::new();

        #[derive(Serialize)]
        struct TestResponse {
            message: String,
        }

        let test_response = TestResponse {
            message: "Hello, zero-copy!".to_string(),
        };

        let response = builder
            .json_response(&test_response, StatusCode::OK)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let stats = builder.get_stats();
        assert!(stats.responses_built.load(Ordering::Relaxed) > 0);
    }

    #[tokio::test]
    async fn test_batch_serialization() {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = ZeroCopyJsonSerializer::new(buffer_pool.clone());

        let items = vec!["item1", "item2", "item3"];
        let serialized = serializer.batch_serialize_zero_copy(&items).await.unwrap();

        // Should be valid JSON array
        let json_str = std::str::from_utf8(&serialized).unwrap();
        assert!(json_str.starts_with('['));
        assert!(json_str.ends_with(']'));
        assert!(json_str.contains("item1"));
        assert!(json_str.contains("item2"));
        assert!(json_str.contains("item3"));
    }

    #[tokio::test]
    async fn test_handler_context() {
        let context = ZeroCopyHandlerContext::new();
        let stats = context.get_comprehensive_stats().await;

        // Should have initialized all components
        assert!(
            stats
                .buffer_pool_stats
                .total_allocations
                .load(Ordering::Relaxed)
                == 0
        );
        assert!(stats.response_stats.responses_built.load(Ordering::Relaxed) == 0);
        assert!(stats.request_stats.requests_parsed.load(Ordering::Relaxed) == 0);
    }
}

// Additional utility functions for zero-copy operations

/// Zero-copy header parsing utilities
pub mod header_utils {

    use axum::http::HeaderMap;

    /// Extract authorization header with zero-copy
    pub fn extract_auth_header(headers: &HeaderMap) -> Option<&str> {
        headers
            .get("authorization")
            .and_then(|value| value.to_str().ok())
    }

    /// Extract content type with zero-copy
    pub fn extract_content_type(headers: &HeaderMap) -> Option<&str> {
        headers
            .get("content-type")
            .and_then(|value| value.to_str().ok())
    }

    /// Parse bearer token from authorization header
    pub fn parse_bearer_token(auth_header: &str) -> Option<&str> {
        if auth_header.starts_with("Bearer ") {
            Some(&auth_header[7..])
        } else {
            None
        }
    }
}

/// Zero-copy URL and path utilities
pub mod url_utils {
    /// Extract path segments without allocation
    pub fn extract_path_segments(path: &str) -> Vec<&str> {
        path.split('/')
            .filter(|segment| !segment.is_empty())
            .collect()
    }

    /// Parse query parameters without unnecessary allocation
    pub fn parse_query_params(query: &str) -> std::collections::HashMap<&str, &str> {
        query
            .split('&')
            .filter_map(|pair| {
                let mut split = pair.split('=');
                match (split.next(), split.next()) {
                    (Some(key), Some(value)) => Some((key, value)),
                    _ => None,
                }
            })
            .collect()
    }
}
