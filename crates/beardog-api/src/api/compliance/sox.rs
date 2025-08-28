

use crate::api::*;
use axum::{extract::State, http::StatusCode, Json};

pub async fn get_sox_compliance(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({
            "compliance_status": "COMPLIANT",
            "score": 93.1,
            "controls_tested": 156
        }),
        request_id,
        40,
        true,
    )))
}

pub async fn get_sox_controls() -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({
            "internal_controls": 156,
            "effective_controls": 152,
            "deficiencies": 4
        }),
        request_id,
        35,
        true,
    )))
}

pub async fn get_financial_reporting_compliance() -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(Json(success_response(
        serde_json::json!({
            "reporting_accuracy": 99.7,
            "disclosure_completeness": 98.9,
            "material_weaknesses": 0
        }),
        request_id,
        50,
        true,
    )))
}
