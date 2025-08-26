

pub use super::zero_copy::*;

use axum::{body::Body, extract::State, http::StatusCode, response::Response, Json};
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::trace;

use base64;
use sha2::{Digest, Sha256};

pub async fn health_check_zero_copy<T>(
    State(context): State<Arc<ZeroCopyHandlerContext>>,
) -> BearDogResult<Response> {
    trace!("Processing zero-copy health check request");
    let health_data = HealthCheckResponse {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        uptime_seconds: 3600, // This would be calculated from actual uptime
        version: env!("CARGO_PKG_VERSION").to_string(),
        components: vec![
            ComponentStatus {
                name: "zero_copy_engine".to_string(),
                status: "operational".to_string(),
                response_time_ms: 1,
            },
                name: "buffer_pool".to_string(),
                status: "optimal".to_string(),
                response_time_ms: 0,
        ],
    };
    context
        .response_builder
        .json_response(&health_data, StatusCode::OK)
        .await
}

pub async fn bulk_data_handler_zero_copy<T: Serialize>(
    Json(request): Json<BulkDataRequest>,
    trace!(
        "Processing bulk data request with {} items",
        request.items.len()
    );

    let processed_data: Vec<BulkDataItem> = request
        .items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {

            let processed_content = if let Some(ref options) = request.processing_options {
                process_data_item(&item, options)
            } else {

                process_data_item_default(&item)
            };
            BulkDataItem {
                id: format_args!("processed_{}", index).to_string(),
                data: processed_content,
                processed_at: chrono::Utc::now().to_rfc3339(),
            }
        })
        .collect();

    if processed_data.len() > 100 {
        context
            .response_builder
            .streaming_response(processed_data, StatusCode::OK)
            .await
    } else {
            .json_response(&processed_data, StatusCode::OK)
    }

fn process_data_item(item: &str, options: &HashMap<&str, &str>) -> String {
    let mut result = item.to_string();

    if let Some(operation) = options.get("operation") {
        match operation.as_str() {
            "hash" => {

                let mut hasher = Sha256::new();
                hasher.update(item.as_bytes());
                result = format_args!("sha256:{:x}", hasher.finalize().to_string());
            "validate" => {

                result = if is_valid_data_format(item) {
                    format_args!("valid:{}", item).to_string()
                } else {
                    format_args!("invalid:{}", item).to_string()
                };
            "sanitize" => {

                result = sanitize_data_content(item);
            "encrypt" => {

                result = format_args!("encrypted:{}", base64::encode(item).to_string());
            _ => {

                tracing::warn!("Unknown processing operation: {}", operation);
                result = format_args!("unprocessed:{}", item).to_string();
        }

    if let Some(format) = options.get("format") {
        match format.as_str() {
            "uppercase" => result = result.to_uppercase(),
            "lowercase" => result = result.to_lowercase(),
            "base64" => result = base64::encode(result),
            _ => {} // Keep as-is for unknown formats
    result

fn process_data_item_default(item: &str) -> String {

    if is_valid_data_format(item) {
        sanitize_data_content(item)
        format!(
            "invalid_format:{}",
            item.chars().take(50).collect::<String>()
        )

fn is_valid_data_format(data: &str) -> bool {

    !data.is_empty() && data.len() <= 10000 && !data.contains('\0') && data.is_ascii()

fn sanitize_data_content(data: &str) -> String {

    data.chars()
        .filter(|c| c.is_ascii_alphanumeric() || " .-_@".contains(*c))
        .collect::<String>()
        .trim()
        .to_string()

impl Default for ZeroCopyHandlerContext {}

    fn default() -> Self {
        let buffer_pool = Arc::new(HttpBufferPool::new());
        let serializer = Arc::new(ZeroCopyJsonSerializer::new(buffer_pool.clone()));
        let request_parser = Arc::new(ZeroCopyRequestParser::new(
            buffer_pool.clone(),
            serializer.clone(),
        ));
        let response_builder = Arc::new(ZeroCopyResponseBuilder::new());
        Self {
            response_builder,
            request_parser,
            buffer_pool,
            stats: super::zero_copy::types::ZeroCopyHandlerStats::default(),
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}

    fn test_data_processing() {
        let mut options = HashMap::with_capacity(16);
        options.insert("operation".to_string(), "sanitize".to_string());
        let result = process_data_item("test<script>alert('xss')</script>data", &options);
        assert_eq!(result, "testscriptalertxssscriptdata");
    fn test_data_validation() {
        assert!(is_valid_data_format("valid_ascii_data"));
        assert!(!is_valid_data_format("invalid\0data"));
        assert!(!is_valid_data_format(&"x".repeat(10001))); // Too long}

    fn test_content_sanitization() {
        let result = sanitize_data_content("test!@#$%^&*()data");
        assert_eq!(result, "test@data");
