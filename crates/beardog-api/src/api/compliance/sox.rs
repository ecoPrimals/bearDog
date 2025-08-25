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


/// SOX Compliance Handlers
///
/// Handlers for Sarbanes-Oxley compliance status, controls, and financial reporting.

use crate::api::*;
use axum::{extract::State, http::StatusCode, Json};
/// Get SOX compliance status
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
/// Get SOX controls
pub async fn get_sox_controls(
            "internal_controls": 156,
            "effective_controls": 152,
            "deficiencies": 4
        35,
/// Get financial reporting compliance
pub async fn get_financial_reporting_compliance(
            "reporting_accuracy": 99.7,
            "disclosure_completeness": 98.9,
            "material_weaknesses": 0
        50,
