// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Monitoring API Routes
///
/// Route definitions for the BearDog monitoring API

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
            "/health/components",
            get(super::handlers::get_component_health),
        .route("/status", get(super::handlers::get_system_status))
        .route("/readiness", get(super::handlers::get_readiness))
        .route("/liveness", get(super::handlers::get_liveness))
        // Metrics & Performance - IMPLEMENTED
        .route("/metrics", get(super::handlers::get_system_metrics))
            "/metrics/realtime",
            get(super::handlers::get_realtime_metrics),
    // NOTE: Additional monitoring endpoints can be added as handlers are implemented
}
