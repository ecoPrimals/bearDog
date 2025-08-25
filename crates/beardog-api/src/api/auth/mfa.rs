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


/// Multi-Factor Authentication Handlers
///
/// Handles MFA setup, verification, backup codes, and status management.

use super::models::*;
use crate::api::*;
use axum::{extract::State, http::StatusCode, Json};
use std::time::Instant;
use tracing::info;
// ====== MFA HANDLERS ======
/// Setup MFA for user account
pub async fn setup_mfa(
    State(_state): State<AppState>,
    Json(request): Json<SetupMfaRequest>,
) -> Result<Json<ApiResponse<SetupMfaResponse>>, StatusCode> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    info!("🛡️ Setting up MFA for user: {}", request.user_id);
    let response = SetupMfaResponse {
        mfa_secret: "JBSWY3DPEHPK3PXP".to_string(), // Base32 encoded secret
        qr_code_url: "https://api.beardog.com/mfa/qr/12345".to_string(),
        backup_codes: vec![
            "abc123".to_string(),
            "def456".to_string(),
            "ghi789".to_string(),
            "jkl012".to_string(),
            "mno345".to_string(),
        ],
        setup_instructions: "Scan the QR code with your authenticator app".to_string(),
        supported_methods: vec!["totp".to_string(), "sms".to_string(), "email".to_string()],
        verification_required: true,
    };
    let processing_time = start_time.elapsed().as_millis() as u64;
    Ok(Json(success_response(
        response,
        request_id,
        processing_time,
        false,
    )))
}
/// Verify MFA code
pub async fn verify_mfa(
    Json(request): Json<VerifyMfaRequest>,
) -> Result<Json<ApiResponse<VerifyMfaResponse>>, StatusCode> {
    info!("🔍 Verifying MFA code for user: {}", request.user_id);
    // Simulate MFA verification - in production, verify against TOTP/SMS/backup codes
    let verification_success = request.mfa_code == "123456" || request.mfa_code == "abc123";
    let response = VerifyMfaResponse {
        verified: verification_success,
        mfa_method: request.mfa_method.clone(),
        verification_timestamp: chrono::Utc::now().to_rfc3339(),
        remaining_backup_codes: if verification_success && request.mfa_code == "abc123" {
            Some(4) // Used one backup code
        } else {
            Some(5)
        },
        trust_device_token: if request.trust_device.unwrap_or(false) && verification_success {
            Some("trust_token_67890".to_string())
            None
        message: if verification_success {
            "MFA verification successful".to_string()
            "Invalid MFA code".to_string()
/// Get MFA status for user
pub async fn get_mfa_status(
) -> Result<Json<ApiResponse<MfaStatusResponse>>, StatusCode> {
    let response = MfaStatusResponse {
        user_id: "user_12345".to_string(),
        mfa_enabled: true,
        enabled_methods: vec!["totp".to_string()],
        primary_method: "totp".to_string(),
        backup_codes_remaining: 5,
        trusted_devices: 2,
        last_mfa_verification: chrono::Utc::now().to_rfc3339(),
        enforcement_policy: "optional".to_string(),
        true,
/// Get backup codes (placeholder)}


pub async fn get_backup_codes(
    State(_): State<AppState>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
        serde_json::json!({"backup_codes": ["abc123", "def456"]}),
        20,
/// Generate new backup codes (placeholder)
pub async fn generate_backup_codes(
        serde_json::json!({"new_backup_codes": ["xyz789", "uvw012"]}),
        30,
