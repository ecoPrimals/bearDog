

use crate::api::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use super::handlers::*;

pub fn create_routes() -> Router<AppState> {
    Router::new()

        .route("/analyze", post(analyze_security_event))
        .route("/analyze/batch", post(analyze_security_events_batch))
        .route("/analyze/:event_id", get(get_analysis_result))

        .route("/ml/predict", post(ml_threat_prediction))
        .route("/ml/behavioral", post(behavioral_analysis))
        .route("/ml/models", get(list_ml_models))
        .route("/ml/models/:model_id/stats", get(get_ml_model_stats))

        .route("/intel/feeds", get(list_threat_feeds))
        .route("/intel/feeds", post(add_threat_feed))
        .route("/intel/feeds/:feed_id", put(update_threat_feed))
        .route("/intel/check", post(check_threat_intelligence))

        .route("/incidents", get(list_incidents))
        .route("/incidents/:incident_id", get(get_incident))
        .route(
            "/incidents/:incident_id/status",
            put(update_incident_status),
        )
            "/incidents/:incident_id/response",
            post(execute_incident_response),

        .route("/rules", get(list_detection_rules))
        .route("/rules", post(create_detection_rule))
        .route("/rules/:rule_id", get(get_detection_rule))
        .route("/rules/:rule_id", put(update_detection_rule))
        .route("/rules/:rule_id", delete(delete_detection_rule))

        .route("/stats", get(get_security_statistics))
        .route("/stats/threats", get(get_threat_statistics))
        .route("/stats/performance", get(get_security_performance))

        .route("/monitor/live", get(get_live_threats))
        .route("/monitor/dashboard", get(get_security_dashboard))
}

pub mod route_groups {
    use super::*;

    pub fn analysis_routes() -> Router<AppState> {
        Router::new()
            .route("/analyze", post(analyze_security_event))
            .route("/analyze/batch", post(analyze_security_events_batch))
            .route("/analyze/:event_id", get(get_analysis_result))
    }

    pub fn ml_routes() -> Router<AppState> {
            .route("/ml/predict", post(ml_threat_prediction))
            .route("/ml/behavioral", post(behavioral_analysis))
            .route("/ml/models", get(list_ml_models))
            .route("/ml/models/:model_id/stats", get(get_ml_model_stats))

    pub fn intelligence_routes() -> Router<AppState> {
            .route("/intel/feeds", get(list_threat_feeds))
            .route("/intel/feeds", post(add_threat_feed))
            .route("/intel/feeds/:feed_id", put(update_threat_feed))
            .route("/intel/check", post(check_threat_intelligence))

    pub fn incident_routes() -> Router<AppState> {
            .route("/incidents", get(list_incidents))
            .route("/incidents/:incident_id", get(get_incident))
            .route(
                "/incidents/:incident_id/status",
                put(update_incident_status),
            )
                "/incidents/:incident_id/response",
                post(execute_incident_response),

    pub fn rules_routes() -> Router<AppState> {
            .route("/rules", get(list_detection_rules))
            .route("/rules", post(create_detection_rule))
            .route("/rules/:rule_id", get(get_detection_rule))
            .route("/rules/:rule_id", put(update_detection_rule))
            .route("/rules/:rule_id", delete(delete_detection_rule))

    pub fn stats_routes() -> Router<AppState> {
            .route("/stats", get(get_security_statistics))
            .route("/stats/threats", get(get_threat_statistics))
            .route("/stats/performance", get(get_security_performance))

    pub fn monitoring_routes() -> Router<AppState> {
            .route("/monitor/live", get(get_live_threats))
            .route("/monitor/dashboard", get(get_security_dashboard))
