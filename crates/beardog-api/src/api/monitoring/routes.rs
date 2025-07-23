//! Monitoring API Routes
//!
//! Route definitions for the BearDog monitoring API

use super::*;
use axum::{routing::get, Router};

/// Create all monitoring API routes
pub fn create_monitoring_routes() -> Router<AppState> {
    Router::new()
        // System Health & Status - IMPLEMENTED
        .route("/health", get(super::handlers::get_system_health))
        .route(
            "/health/detailed",
            get(super::handlers::get_detailed_health),
        )
        .route(
            "/health/components",
            get(super::handlers::get_component_health),
        )
        .route("/status", get(super::handlers::get_system_status))
        .route("/readiness", get(super::handlers::get_readiness))
        .route("/liveness", get(super::handlers::get_liveness))
        // Metrics & Performance - IMPLEMENTED
        .route("/metrics", get(super::handlers::get_system_metrics))
        .route(
            "/metrics/realtime",
            get(super::handlers::get_realtime_metrics),
        )
    // TODO: Add remaining endpoints when handlers are implemented
}
