//! # BearDog Comprehensive API System
//!
//! **AI-First Headless API Design**
//!
//! This module provides a complete REST API for all BearDog functionality,
//! designed primarily for AI/programmatic consumption with human-friendly
//! responses. All APIs follow consistent patterns for easy integration.
//!
//! ## API Organization
//!
//! * **`/api/v1/security/*`** - Security provider and threat detection
//! * **`/api/v1/genetics/*`** - Genetic spawning and node management  
//! * **`/api/v1/compliance/*`** - Audit and compliance workflows
//! * **`/api/v1/auth/*`** - Authentication and authorization
//! * **`/api/v1/monitoring/*`** - Health, metrics, and observability
//! * **`/api/v1/config/*`** - Configuration management
//! * **`/api/v1/nodes/*`** - Node registry and cross-node operations
//!
//! ## Performance Features
//!
//! * **Request/Response Caching** - Redis-backed intelligent caching
//! * **Rate Limiting** - Per-endpoint and per-user limits
//! * **Connection Pooling** - Optimized database connections
//! * **Async Processing** - Non-blocking operations throughout
//! * **Compression** - Automatic gzip/deflate for large responses
//! * **Pagination** - Efficient large dataset handling

use serde::{Deserialize, Serialize};

pub mod ai_interface;
pub mod auth; // Authentication and authorization management
pub mod cache;
pub mod compliance; // Enterprise compliance and audit management
pub mod error_handling; // Error handling and response formatting
pub mod genetics;
// pub mod handlers; // Generic handlers are implemented in individual modules
pub mod monitoring;
pub mod rate_limiting;
pub mod rpc; // Ecosystem RPC integration for sovereign network effects
pub mod security;
pub mod server;
pub mod sovereignty; // Individual empowerment and peer-to-peer sharing
pub mod types; // Common types and constants
               // pub mod webhooks; // Webhooks will be implemented based on specific integration needs
pub mod zero_copy; // Zero-copy optimization modules
pub mod zero_copy_handlers; // Add zero-copy optimizations

// Future modules for specific functionality
// pub mod routes;     // Route definitions handled per-module
// pub mod middleware; // Middleware handled per-module
// pub mod models;     // Models defined per-module
// pub mod config;     // Configuration handled by beardog-config crate
// pub mod nodes;      // Node management handled by beardog-node-registry crate

// Re-export main components
pub use error_handling::*;
pub use server::*;
pub use types::*;
// Additional re-exports will be added as modules are developed
// pub use routes::*;
// pub use middleware::*;
// pub use models::*;

/// API version constant
pub const API_VERSION: &str = "v1";

/// Standard API response wrapper for all endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success indicator
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message if any
    pub error: Option<String>,
    /// Request ID for tracing
    pub request_id: String,
    /// Response timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Performance metadata
    pub meta: ResponseMetadata,
}

/// Metadata included with API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Whether response was cached
    pub cached: bool,
    /// API version used
    pub version: String,
    /// Pagination info (if applicable)
    pub pagination: Option<PaginationMeta>,
}

/// Pagination metadata for paginated responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// Current page number
    pub page: u32,
    /// Items per page
    pub per_page: u32,
    /// Total number of items
    pub total: u64,
    /// Total number of pages
    pub total_pages: u32,
    /// Whether there's a next page
    pub has_next: bool,
    /// Whether there's a previous page
    pub has_prev: bool,
}

/// Standard pagination parameters
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: u32,
    /// Items per page
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 {
    1
}
fn default_per_page() -> u32 {
    50
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
        }
    }
}

/// Helper function to create success response
pub fn success_response<T>(
    data: T,
    request_id: String,
    processing_time_ms: u64,
    cached: bool,
) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        request_id,
        timestamp: chrono::Utc::now(),
        meta: ResponseMetadata {
            processing_time_ms,
            cached,
            version: API_VERSION.to_string(),
            pagination: None,
        },
    }
}

/// Helper function to create error response
pub fn error_response<T>(
    error: String,
    request_id: String,
    processing_time_ms: u64,
) -> ApiResponse<T> {
    ApiResponse {
        success: false,
        data: None,
        error: Some(error),
        request_id,
        timestamp: chrono::Utc::now(),
        meta: ResponseMetadata {
            processing_time_ms,
            cached: false,
            version: API_VERSION.to_string(),
            pagination: None,
        },
    }
}

/// Helper function to create paginated response
pub fn paginated_response<T>(
    data: T,
    request_id: String,
    processing_time_ms: u64,
    cached: bool,
    pagination: PaginationMeta,
) -> ApiResponse<T> {
    ApiResponse {
        success: true,
        data: Some(data),
        error: None,
        request_id,
        timestamp: chrono::Utc::now(),
        meta: ResponseMetadata {
            processing_time_ms,
            cached,
            version: API_VERSION.to_string(),
            pagination: Some(pagination),
        },
    }
}
