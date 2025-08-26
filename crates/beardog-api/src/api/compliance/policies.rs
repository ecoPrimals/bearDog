

use crate::api::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn list_compliance_policies(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({
            "policies": ["Data Protection Policy", "Security Policy", "Retention Policy"],
            "total_count": 3
        }),
        request_id,
        20,
        true,
    )))
}

pub async fn create_compliance_policy(
    Json(_): Json<serde_json::Value>,
        serde_json::json!({"policy_id": "policy_12345", "status": "created"}),
        45,
        false,

pub async fn get_compliance_policy(
    Path(_policy_id): Path<String>,
        serde_json::json!({"policy": "Data Protection Policy", "version": "1.2"}),
        15,

pub async fn update_compliance_policy(
        serde_json::json!({"policy_id": "policy_12345", "status": "updated"}),
        35,

pub async fn delete_compliance_policy(
        serde_json::json!({"policy_id": "policy_12345", "status": "deleted"}),
        25,
