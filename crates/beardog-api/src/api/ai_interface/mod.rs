//! AI-optimized API interface for BearDog
//!
//! This module provides AI-friendly APIs designed for:
//! - Machine-readable responses
//! - Batch operations
//! - Streaming capabilities
//! - Comprehensive error handling
//! - Structured data formats

use axum::routing::{get, post};
use axum::Router;
use beardog_core::core::BearDogCore;
use std::sync::Arc;

pub mod batch;
pub mod genetics;
pub mod handlers;
pub mod health;
pub mod hsm;
pub mod security;
pub mod types;

pub use batch::*;
pub use genetics::*;
pub use handlers::*;
pub use health::*;
pub use hsm::*;
pub use security::*;
pub use types::*;

/// Create AI-first API router
pub fn create_ai_router() -> Router<Arc<BearDogCore>> {
    Router::new()
        // Health and status endpoints
        .route("/ai/health", get(health_check))
        .route("/ai/status", get(system_status))
        .route("/ai/metrics", get(metrics))
        // Security endpoints
        .route("/ai/encrypt", post(encrypt))
        .route("/ai/decrypt", post(decrypt))
        .route("/ai/sign", post(sign))
        .route("/ai/verify", post(verify))
        .route("/ai/keygen", post(generate_key))
        // Batch security endpoints
        .route("/ai/batch/security", post(batch_security))
        // Genetics endpoints
        .route("/ai/spawn", post(spawn_node))
        .route("/ai/batch/spawn", post(batch_spawn))
        .route("/ai/spawn/status/:node_id", get(spawn_status))
        // HSM endpoints
        .route("/ai/hsm/status", get(hsm_status))
        .route("/ai/hsm/tiers", get(hsm_tiers))
        .route("/ai/hsm/select", post(select_hsm_tier))
        // Streaming endpoints
        .route("/ai/stream/status", get(stream_status))
        .route("/ai/stream/metrics", get(stream_metrics))
}
