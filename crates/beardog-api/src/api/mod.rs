

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub mod auth;
pub mod cache;
pub mod compliance;
pub mod error_handling;
// pub mod handlers; // Module not found - temporarily disabled
// pub mod middleware; // Module not found - temporarily disabled
// pub mod monitoring; // Temporarily disabled - has syntax issues in dashboard.rs
// pub mod rate_limiting; // Temporarily disabled - has syntax issues
// pub mod response; // Module not found - temporarily disabled
// pub mod routing; // Module not found - temporarily disabled
pub mod security;
pub mod sovereignty;
pub mod zero_copy;
pub mod zero_cost_server;

use beardog_types::constants::unified::api::VERSION as API_VERSION;

#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub request_id: String,
    pub timestamp: DateTime<Utc>,
    pub meta: ResponseMetadata,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseMetadata {
    pub processing_time_ms: u64,
    pub cached: bool,
    pub version: String,
    pub pagination: Option<PaginationMeta>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total: u64,
    pub total_pages: u32,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
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

pub fn error_response<T>(
    error: String,
    request_id: String,
) -> ApiResponse<T> {
    ApiResponse {
        success: false,
        data: None,
        error: Some(error),
        request_id,
        timestamp: chrono::Utc::now(),
        meta: ResponseMetadata {
            processing_time_ms: 0,
            cached: false,
            version: API_VERSION.to_string(),
            pagination: None,
        },
    }
}

pub fn paginated_response<T>(
    data: T,
    request_id: String,
    processing_time_ms: u64,
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
            cached: false,
            version: API_VERSION.to_string(),
            pagination: Some(pagination),
        },
    }
}
