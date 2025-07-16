//! Security API route definitions
//!
//! This module defines all the routes and their mappings for the security API,
//! providing a clean separation between route configuration and handler logic.

use crate::api::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use super::handlers::*;

/// Create security API routes
///
/// This function sets up all the security API endpoints with their corresponding
/// HTTP methods and handler functions.
///
/// # Returns
/// A configured `Router<AppState>` with all security endpoints
///
/// # Route Structure
/// - `/analyze` - Threat analysis endpoints
/// - `/ml` - Machine learning endpoints  
/// - `/intel` - Threat intelligence endpoints
/// - `/incidents` - Incident management endpoints
/// - `/rules` - Detection rules management
/// - `/stats` - Statistics and monitoring
/// - `/monitor` - Real-time monitoring
pub fn create_routes() -> Router<AppState> {
    Router::new()
        // Threat Analysis Endpoints
        .route("/analyze", post(analyze_security_event))
        .route("/analyze/batch", post(analyze_security_events_batch))
        .route("/analyze/:event_id", get(get_analysis_result))
        // ML-Powered Detection
        .route("/ml/predict", post(ml_threat_prediction))
        .route("/ml/behavioral", post(behavioral_analysis))
        .route("/ml/models", get(list_ml_models))
        .route("/ml/models/:model_id/stats", get(get_ml_model_stats))
        // Threat Intelligence
        .route("/intel/feeds", get(list_threat_feeds))
        .route("/intel/feeds", post(add_threat_feed))
        .route("/intel/feeds/:feed_id", put(update_threat_feed))
        .route("/intel/check", post(check_threat_intelligence))
        // Incident Management
        .route("/incidents", get(list_incidents))
        .route("/incidents/:incident_id", get(get_incident))
        .route(
            "/incidents/:incident_id/status",
            put(update_incident_status),
        )
        .route(
            "/incidents/:incident_id/response",
            post(execute_incident_response),
        )
        // Detection Rules Management
        .route("/rules", get(list_detection_rules))
        .route("/rules", post(create_detection_rule))
        .route("/rules/:rule_id", get(get_detection_rule))
        .route("/rules/:rule_id", put(update_detection_rule))
        .route("/rules/:rule_id", delete(delete_detection_rule))
        // Statistics and Monitoring
        .route("/stats", get(get_security_statistics))
        .route("/stats/threats", get(get_threat_statistics))
        .route("/stats/performance", get(get_security_performance))
        // Real-time Monitoring
        .route("/monitor/live", get(get_live_threats))
        .route("/monitor/dashboard", get(get_security_dashboard))
}

/// Route groups for better organization
pub mod route_groups {
    use super::*;

    /// Threat analysis routes
    pub fn analysis_routes() -> Router<AppState> {
        Router::new()
            .route("/analyze", post(analyze_security_event))
            .route("/analyze/batch", post(analyze_security_events_batch))
            .route("/analyze/:event_id", get(get_analysis_result))
    }

    /// Machine learning routes
    pub fn ml_routes() -> Router<AppState> {
        Router::new()
            .route("/ml/predict", post(ml_threat_prediction))
            .route("/ml/behavioral", post(behavioral_analysis))
            .route("/ml/models", get(list_ml_models))
            .route("/ml/models/:model_id/stats", get(get_ml_model_stats))
    }

    /// Threat intelligence routes
    pub fn intelligence_routes() -> Router<AppState> {
        Router::new()
            .route("/intel/feeds", get(list_threat_feeds))
            .route("/intel/feeds", post(add_threat_feed))
            .route("/intel/feeds/:feed_id", put(update_threat_feed))
            .route("/intel/check", post(check_threat_intelligence))
    }

    /// Incident management routes
    pub fn incident_routes() -> Router<AppState> {
        Router::new()
            .route("/incidents", get(list_incidents))
            .route("/incidents/:incident_id", get(get_incident))
            .route("/incidents/:incident_id/status", put(update_incident_status))
            .route("/incidents/:incident_id/response", post(execute_incident_response))
    }

    /// Detection rules routes
    pub fn rules_routes() -> Router<AppState> {
        Router::new()
            .route("/rules", get(list_detection_rules))
            .route("/rules", post(create_detection_rule))
            .route("/rules/:rule_id", get(get_detection_rule))
            .route("/rules/:rule_id", put(update_detection_rule))
            .route("/rules/:rule_id", delete(delete_detection_rule))
    }

    /// Statistics and monitoring routes
    pub fn stats_routes() -> Router<AppState> {
        Router::new()
            .route("/stats", get(get_security_statistics))
            .route("/stats/threats", get(get_threat_statistics))
            .route("/stats/performance", get(get_security_performance))
    }

    /// Real-time monitoring routes
    pub fn monitoring_routes() -> Router<AppState> {
        Router::new()
            .route("/monitor/live", get(get_live_threats))
            .route("/monitor/dashboard", get(get_security_dashboard))
    }
} 