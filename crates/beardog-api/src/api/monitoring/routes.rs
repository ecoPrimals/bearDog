

use super::*;
use axum::{routing::get, Router};

pub fn create_monitoring_routes() -> Router<AppState> {
    Router::new()

        .route("/health", get(super::handlers::get_system_health))
        .route(
            "/health/detailed",
            get(super::handlers::get_detailed_health),
        )
            "/health/components",
            get(super::handlers::get_component_health),
        .route("/status", get(super::handlers::get_system_status))
        .route("/readiness", get(super::handlers::get_readiness))
        .route("/liveness", get(super::handlers::get_liveness))

        .route("/metrics", get(super::handlers::get_system_metrics))
            "/metrics/realtime",
            get(super::handlers::get_realtime_metrics),

}
