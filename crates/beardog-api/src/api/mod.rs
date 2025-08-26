

use serde::{Deserialize, Serialize};
pub mod ai_interface;
pub mod auth; // Authentication and authorization management
pub mod cache;
pub mod compliance; // Enterprise compliance and audit management
pub mod error_handling; // Error handling and response formatting
pub mod genetics;

pub mod monitoring;
pub mod rate_limiting;
pub mod rpc; // Ecosystem RPC integration for sovereign network effects
pub mod security;
pub mod server;
pub mod sovereignty; // Individual empowerment and peer-to-peer sharing
pub mod types; // Common types and constants

pub mod zero_copy; // Zero-copy optimization modules
pub mod zero_copy_handlers; // Add zero-copy optimizations

pub use error_handling::*;
pub use server::*;
pub use types::*;

pub use beardog_types::constants::unified::api::VERSION as API_VERSION;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {

    pub success: bool,

    pub data: Option<T>,

    pub error: Option<String>,

    pub request_id: String,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub meta: ResponseMetadata,
}

pub struct ResponseMetadata {

    pub processing_time_ms: u64,

    pub cached: bool,

    pub version: String,

    pub pagination: Option<PaginationMeta>,

pub struct PaginationMeta {

    pub page: u32,

    pub per_page: u32,

    pub total: u64,

    pub total_pages: u32,

    pub has_next: bool,

    pub has_prev: bool,

#[derive(Debug, Clone, Deserialize)]
pub struct PaginationParams {

    #[serde(default = "default_page")]
    #[serde(default = "default_per_page")]
fn default_page() -> u32 {
    1}

fn default_per_page() -> u32 {
    50}

impl Default for PaginationParams {}

    fn default() -> Self {
        Self {
            page: 1,
            per_page: 50,
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

pub fn error_response<T>(
    error: String,
        success: false,
        data: None,
        error: Some(error),
            cached: false,

pub fn paginated_response<T>(
    pagination: PaginationMeta,
            pagination: Some(pagination),
