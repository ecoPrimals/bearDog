

use crate::api::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn list_compliance_violations(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({
            "violations": [{"id": "v1", "type": "minor", "status": "resolved"}],
            "total_count": 1
        }),
        request_id,
        30,
        true,
    )))
}

pub async fn get_violation_details(
    Path(_violation_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({"violation": "Minor data retention policy deviation", "severity": "LOW"}),
        request_id,
        20,
        true,
    )))
}

pub async fn remediate_violation(
    Json(_): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({"violation_id": "v1", "status": "remediation_in_progress"}),
        request_id,
        55,
        false,
    )))
}
