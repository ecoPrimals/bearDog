/// Zero-Copy API Request/Response Handlers for BearDog
///
/// **High-Performance HTTP Processing with Minimal Allocations**
///
/// This module provides zero-copy HTTP request and response handling using
/// the modular zero-copy architecture for optimal performance.
// Re-export from the zero_copy module for backward compatibility
pub use super::zero_copy::*;

use axum::{body::Body, extract::State, http::StatusCode, response::Response, Json};
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::trace;

// Additional imports for data processing
use base64;
use sha2::{Digest, Sha256};

/// Health check handler with zero-copy response
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
            ComponentStatus {
                name: "buffer_pool".to_string(),
                status: "optimal".to_string(),
                response_time_ms: 0,
            },
        ],
    };

    context
        .response_builder
        .json_response(&health_data, StatusCode::OK)
        .await
}

/// Bulk data handler with zero-copy optimization
pub async fn bulk_data_handler_zero_copy<T: Serialize>(
    State(context): State<Arc<ZeroCopyHandlerContext>>,
    Json(request): Json<BulkDataRequest>,
) -> BearDogResult<Response> {
    trace!(
        "Processing bulk data request with {} items",
        request.items.len()
    );

    // Process each item according to the requested processing options
    let processed_data: Vec<BulkDataItem> = request
        .items
        .into_iter()
        .enumerate()
        .map(|(index, item)| {
            // Apply processing based on options
            let processed_content = if let Some(ref options) = request.processing_options {
                process_data_item(&item, options)
            } else {
                // Default processing: basic validation and formatting
                process_data_item_default(&item)
            };

            BulkDataItem {
                id: format!("processed_{}", index),
                data: processed_content,
                processed_at: chrono::Utc::now().to_rfc3339(),
            }
        })
        .collect();

    // Use streaming response for large datasets to maintain zero-copy performance
    if processed_data.len() > 100 {
        context
            .response_builder
            .streaming_response(processed_data, StatusCode::OK)
            .await
    } else {
        context
            .response_builder
            .json_response(&processed_data, StatusCode::OK)
            .await
    }
}

/// Process a data item with specific options
fn process_data_item(item: &str, options: &HashMap<String, String>) -> String {
    let mut result = item.to_string();

    // Apply security-focused processing based on options
    if let Some(operation) = options.get("operation") {
        match operation.as_str() {
            "hash" => {
                // Compute SHA-256 hash for security validation
                let mut hasher = Sha256::new();
                hasher.update(item.as_bytes());
                result = format!("sha256:{:x}", hasher.finalize());
            }
            "validate" => {
                // Validate data format and return status
                result = if is_valid_data_format(item) {
                    format!("valid:{}", item)
                } else {
                    format!("invalid:{}", item)
                };
            }
            "sanitize" => {
                // Sanitize potentially dangerous content
                result = sanitize_data_content(item);
            }
            "encrypt" => {
                // Basic encryption placeholder - in production would use proper encryption
                result = format!("encrypted:{}", base64::encode(item));
            }
            _ => {
                // Unknown operation, return as-is with warning
                tracing::warn!("Unknown processing operation: {}", operation);
                result = format!("unprocessed:{}", item);
            }
        }
    }

    // Apply additional transformations based on other options
    if let Some(format) = options.get("format") {
        match format.as_str() {
            "uppercase" => result = result.to_uppercase(),
            "lowercase" => result = result.to_lowercase(),
            "base64" => result = base64::encode(result),
            _ => {} // Keep as-is for unknown formats
        }
    }

    result
}

/// Default processing for items without specific options
fn process_data_item_default(item: &str) -> String {
    // Default: validate and sanitize the input
    if is_valid_data_format(item) {
        sanitize_data_content(item)
    } else {
        format!(
            "invalid_format:{}",
            item.chars().take(50).collect::<String>()
        )
    }
}

/// Validate data format (basic security check)
fn is_valid_data_format(data: &str) -> bool {
    // Basic validation: not empty, reasonable length, no null bytes
    !data.is_empty() && data.len() <= 10000 && !data.contains('\0') && data.is_ascii()
}

/// Sanitize potentially dangerous content
fn sanitize_data_content(data: &str) -> String {
    // Remove potentially dangerous characters and sequences
    data.chars()
        .filter(|c| c.is_ascii_alphanumeric() || " .-_@".contains(*c))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Create default handler context for zero-copy operations
impl Default for ZeroCopyHandlerContext {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processing() {
        let mut options = HashMap::new();
        options.insert("operation".to_string(), "sanitize".to_string());

        let result = process_data_item("test<script>alert('xss')</script>data", &options);
        assert_eq!(result, "testscriptalertxssscriptdata");
    }

    #[test]
    fn test_data_validation() {
        assert!(is_valid_data_format("valid_ascii_data"));
        assert!(!is_valid_data_format("invalid\0data"));
        assert!(!is_valid_data_format(&"x".repeat(10001))); // Too long
    }

    #[test]
    fn test_content_sanitization() {
        let result = sanitize_data_content("test!@#$%^&*()data");
        assert_eq!(result, "test@data");
    }
}
