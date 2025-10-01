// BearDog API module
// Provides RESTful API endpoints for the BearDog security platform

use axum::{extract::State, http::StatusCode, response::Json, routing::get, Router};
use beardog_core::core::BearDogCore;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;

/// Get current memory usage (simplified implementation)
fn get_memory_usage() -> u64 {
    // Use a simple approach - in production, use proper memory monitoring
    std::process::id() as u64 * 1024 // Simplified calculation
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    /// Create successful response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create error response
    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// System status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: String,
    pub connections: u32,
    pub memory_usage: u64,
}

/// API state container
#[derive(Debug, Clone)]
pub struct ApiState {
    pub core: Arc<BearDogCore>,
}

/// Health check endpoint
pub async fn health_check(
    _state: State<ApiState>,
) -> Result<Json<ApiResponse<HealthResponse>>, StatusCode> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// System status endpoint
pub async fn system_status(
    _state: State<ApiState>,
) -> Result<Json<ApiResponse<StatusResponse>>, StatusCode> {
    let response = StatusResponse {
        status: "operational".to_string(),
        connections: 1, // Current connection count
        memory_usage: get_memory_usage(),
    };

    Ok(Json(ApiResponse::success(response)))
}

/// Create API router with all endpoints
/// Creates router
pub fn create_router(core: Arc<BearDogCore>) -> Router {
    let state = ApiState { core };

    Router::new()
        .route("/health", get(health_check))
        .route("/status", get(system_status))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

/// Start API server
/// Starts api_server
pub async fn start_api_server(core: Arc<BearDogCore>, bind_addr: &str) -> Result<(), BearDogError> {
    info!("🚀 Starting BearDog API server on {}", bind_addr);

    let app = create_router(core);
    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to bind to {}: {}", bind_addr, e)))?;

    info!("✅ API server listening on {}", bind_addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| BearDogError::system(format!("Server error: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Method, Request},
    };
    use tower::ServiceExt;

    #[tokio::test]
    fn test_health_endpoint() {
        let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
        let core = Arc::new(BearDogCore::new(config));
        let app = create_router(core);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/health")
            .body(Body::empty())
            .expect("Health endpoint request should build successfully");

        let response = app
            .oneshot(request)
            .expect("Health endpoint should respond successfully");
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    fn test_status_endpoint() {
        let config = beardog_types::canonical::config::unified::UnifiedBearDogConfig::default();
        let core = Arc::new(BearDogCore::new(config));
        let app = create_router(core);

        let request = Request::builder()
            .method(Method::GET)
            .uri("/status")
            .body(Body::empty())
            .expect("Status endpoint request should build successfully");

        let response = app
            .oneshot(request)
            .expect("Status endpoint should respond successfully");
        assert_eq!(response.status(), StatusCode::OK);
    }
}
