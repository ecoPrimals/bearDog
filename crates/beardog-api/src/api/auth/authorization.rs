

use super::models::*;
use crate::api::*;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use std::time::Instant;
use tracing::info;
use beardog_auth::auth::{AuthorizationRequest, AuthorizationResult};
use beardog_errors::BearDogError;
use beardog_types::canonical::SecurityContext;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthorizationRequest {
    pub user_id: String,
    pub resource: String,
    pub action: String,
    pub context: Option<SecurityContext>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiAuthorizationResponse {
    pub authorized: bool,
    pub permissions: Vec<String>,
    pub restrictions: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyResponse {
    pub key_id: String,
    pub api_key: String,
    pub name: String,
    pub scopes: Vec<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub prefix: String,
    pub warning: String,
    pub usage_instructions: String,
}

pub struct AuthorizationHandler {
    auth_service: beardog_auth::auth::handlers::AuthHandler,
}

impl AuthorizationHandler {
    pub fn new() -> Self {
        Self {
            auth_service: beardog_auth::auth::handlers::AuthHandler::new(),
        }
    }

    pub async fn authorize(&self, request: ApiAuthorizationRequest) -> Result<ApiAuthorizationResponse, BearDogError> {
        let auth_request = AuthorizationRequest {
            user_id: request.user_id,
            resource: request.resource,
            action: request.action,
            context: request.context,
        };

        let result = self.auth_service.authorize(auth_request).await?;
        
        Ok(ApiAuthorizationResponse {
            authorized: matches!(result, AuthorizationResult::Granted),
            permissions: vec![], // TODO: Add actual permissions from result
            restrictions: vec![], // TODO: Add actual restrictions from result
            expires_at: None, // TODO: Add expiration from result
        })
    }
}

impl Default for AuthorizationHandler {
    fn default() -> Self {
        Self::new()
    }
}

// API Handlers
pub async fn authorize(
    State(_): State<AppState>,
    Json(request): Json<ApiAuthorizationRequest>,
) -> Result<Json<ApiResponse<ApiAuthorizationResponse>>, StatusCode> {
    let handler = AuthorizationHandler::new();
    
    match handler.authorize(request).await {
        Ok(response) => Ok(Json(ApiResponse::success(response))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn check_permissions(
    State(_): State<AppState>,
    Path(_user_id): Path<String>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "permissions": ["read:users", "write:users"],
        "roles": ["admin", "user"]
    }))))
}

pub async fn create_api_key(
    State(_): State<AppState>,
    Json(request): Json<CreateApiKeyRequest>,
) -> Result<Json<ApiResponse<CreateApiKeyResponse>>, StatusCode> {
    let key_id = uuid::Uuid::new_v4().to_string();
    let api_key = format!("bd_{}", &uuid::Uuid::new_v4().to_string().replace('-', "")[..24]);
    
    let response = CreateApiKeyResponse {
        key_id,
        api_key: api_key.clone(),
        name: request.name,
        scopes: request.scopes,
        created_at: chrono::Utc::now().to_rfc3339(),
        expires_at: request.expires_at,
        prefix: api_key[..12].to_string(),
        warning: "Store this API key securely. It will not be shown again.".to_string(),
        usage_instructions: "Include in Authorization header as 'Bearer <api_key>'".to_string(),
    };
    
    Ok(Json(ApiResponse::success(response)))
}

pub async fn get_api_key(
    Path(_key_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "key": "API Key details"
    }))))
}

pub async fn revoke_api_key(
    Path(_key_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "API key revoked successfully"
    }))))
}

pub async fn create_role(
    State(_): State<AppState>,
    Json(_): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "role_id": "new_role_123"
    }))))
}

pub async fn get_role(
    Path(_role_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "role": "Admin Role"
    }))))
}

pub async fn update_role(
    Path(_role_id): Path<String>,
    Json(_): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Role updated"
    }))))
}

pub async fn delete_role(
    Path(_role_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Role deleted"
    }))))
}

pub async fn get_user_roles(
    Path(_user_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "roles": ["admin", "user"]
    }))))
}

pub async fn assign_user_roles(
    Path(_user_id): Path<String>,
    Json(_): Json<serde_json::Value>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Roles assigned"
    }))))
}

pub async fn remove_user_role(
    Path((_user_id, _role_id)): Path<(String, String)>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Role removed"
    }))))
}

pub async fn get_user_permissions(
    Path(_user_id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    Ok(Json(ApiResponse::success(serde_json::json!({
        "permissions": ["read:users", "write:users", "admin:system"]
    }))))
}
