//! Health and Status Endpoints

use axum::{extract::State, http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};

use crate::{ApiResponse, ApiState};

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: u64,
}

/// Detailed system status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub connections: u32,
    pub memory_usage_bytes: u64,
    pub capabilities_available: usize,
}

/// Health check endpoint
pub async fn health_check(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: 0, // Would track actual uptime
    };

    Ok(Json(ApiResponse::success(response)))
}

/// System status endpoint
pub async fn system_status(
    State(_state): State<ApiState>,
) -> Result<Json<ApiResponse<StatusResponse>>, StatusCode> {
    let response = StatusResponse {
        status: "operational".to_string(),
        connections: 1,
        memory_usage_bytes: std::process::id() as u64 * 1024,
        capabilities_available: 8, // Number of capabilities we advertise
    };

    Ok(Json(ApiResponse::success(response)))
}
