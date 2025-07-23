//! Ecosystem RPC Integration - Dynamic Primal Discovery
//!
//! Rust RPC integration that enables sovereign participation in the
//! ecoPrimals ecosystem while maintaining complete independence and human dignity.
//!
//! ## Core Principles
//! - **Sovereignty First**: Primal remains fully functional standalone
//! - **Dynamic Discovery**: Discover other primals through capabilities, not hardcoded names
//! - **Capability-Based**: Discovery and integration based on what services can do
//! - **Human-Centered**: All RPC operations serve individual human dignity

use super::AppState;
use axum::{
    routing::{get, post},
    Router,
};

pub mod cross_primal_operations;
pub mod ecosystem_collaboration;
pub mod models;
pub mod placeholder_handlers;
pub mod primal_registration;

// Re-export commonly used types
pub use cross_primal_operations::*;
pub use ecosystem_collaboration::*;
pub use models::*;
pub use placeholder_handlers::*;
pub use primal_registration::*;

/// Create the RPC router with all ecosystem integration endpoints
pub fn create_rpc_router() -> Router<AppState> {
    Router::new()
        // Registration & Discovery
        .route("/rpc/register", post(register_with_ecosystem))
        .route("/rpc/status", get(get_primal_status))
        .route("/rpc/capabilities", get(get_self_primal_capabilities))
        .route("/rpc/discover", post(discover_capabilities))
        // Collaboration Services
        .route("/rpc/services/security", post(provide_security_service))
        .route(
            "/rpc/services/sovereignty",
            post(provide_sovereignty_service),
        )
        .route("/rpc/network-effects", get(get_network_effects))
        // Cross-Primal Operations
        .route(
            "/rpc/operations/execute",
            post(execute_cross_primal_operation),
        )
        .route("/rpc/resources/compute", post(request_compute_resources))
        .route("/rpc/resources/storage", post(request_storage_resources))
        .route("/rpc/operations/call", post(call_ecosystem_service))
        .route("/rpc/operations/broadcast", post(broadcast_to_ecosystem))
        // Management
        .route("/rpc/deregister", post(deregister_from_ecosystem))
        .route("/rpc/metadata", get(get_primal_metadata))
        .route("/rpc/health", get(primal_health_check))
        .route("/rpc/services/discover", post(discover_ecosystem_services))
        .route("/rpc/services/subscribe", post(subscribe_to_service))
        .route("/rpc/services/unsubscribe", post(unsubscribe_from_service))
        .route("/rpc/metrics", get(get_ecosystem_metrics))
        .route("/rpc/topology", get(get_ecosystem_topology))
}
