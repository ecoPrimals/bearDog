

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ZeroCopyHandlerContext {
    pub response_builder: std::sync::Arc<super::ZeroCopyResponseBuilder>,
    pub request_parser: std::sync::Arc<super::ZeroCopyRequestParser>,
    pub buffer_pool: std::sync::Arc<super::HttpBufferPool>,
    pub stats: ZeroCopyHandlerStats,
}
impl ZeroCopyHandlerContext {

    pub fn new() -> Self {
        let buffer_pool = std::sync::Arc::new(super::HttpBufferPool::new());
        let serializer = std::sync::Arc::new(super::json_serializer::ZeroCopyJsonSerializer::new(
            buffer_pool.clone(),
        ));
        Self {
            response_builder: std::sync::Arc::new(super::ZeroCopyResponseBuilder::new()),
            request_parser: std::sync::Arc::new(super::ZeroCopyRequestParser::new(
                buffer_pool.clone(),
                serializer,
            )),
            buffer_pool,
            stats: ZeroCopyHandlerStats::default(),
        }
    }

    pub async fn get_comprehensive_stats(&self) -> &ZeroCopyHandlerStats {

        &self.stats

#[derive(Debug, Default)]
pub struct ZeroCopyHandlerStats {
    pub requests_processed: std::sync::atomic::AtomicU64,
    pub responses_sent: std::sync::atomic::AtomicU64,
    pub errors_handled: std::sync::atomic::AtomicU64,
    pub average_response_time_ms: std::sync::atomic::AtomicU64,
    pub buffer_pool_stats: BufferPoolStats,
    pub response_stats: ResponseStats,
    pub request_stats: RequestStats,

pub struct BufferPoolStats {
    pub small_buffer_hits: std::sync::atomic::AtomicU64,
    pub medium_buffer_hits: std::sync::atomic::AtomicU64,
    pub large_buffer_hits: std::sync::atomic::AtomicU64,
    pub total_allocations: std::sync::atomic::AtomicU64,

pub struct ResponseStats {
    pub responses_built: std::sync::atomic::AtomicU64,
    pub header_cache_hits: std::sync::atomic::AtomicU64,
    pub streaming_responses: std::sync::atomic::AtomicU64,
    pub content_bytes_served: std::sync::atomic::AtomicU64,

pub struct RequestStats {
    pub requests_parsed: std::sync::atomic::AtomicU64,
    pub zero_copy_parses: std::sync::atomic::AtomicU64,

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: ErrorDetails,
    pub request_id: Option<String>,

pub struct ErrorDetails {
    pub code: String,
    pub message: String,
    pub details: Option<HashMap<String, String>>,

pub struct SuccessResponse<T> {
    pub data: T,
    pub metadata: ResponseMetadata,

pub struct ResponseMetadata {
    pub request_id: String,
    pub timestamp: String,
    pub processing_time_ms: u64,

pub struct HealthCheckResponse {
    pub status: String,
    pub uptime_seconds: u64,
    pub version: String,
    pub components: Vec<ComponentStatus>,

pub struct ComponentStatus {
    pub name: String,
    pub response_time_ms: u64,

#[derive(Debug, Deserialize)]
pub struct BulkDataRequest {
    pub items: Vec<String>,
    pub processing_options: Option<HashMap<String, String>>,

pub struct BulkDataItem {
    pub id: String,
    pub data: String,
    pub processed_at: String,
