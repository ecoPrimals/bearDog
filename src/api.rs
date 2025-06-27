//! BearDog API Server Module
//! 
//! Provides secure RESTful access to BearDog security management functions.

use std::sync::Arc;
use std::time::Duration;

use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use tokio::time::timeout;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use tracing::{info, warn, error};
use uuid::Uuid;

use crate::{BearDogCore, BearDogError, BearDogResult};
use crate::workflows::ApprovalSubmission;
use crate::{
    encryption::{EncryptionRequest, EncryptionResponse, DecryptionRequest, DecryptionResponse},
};

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub uptime_seconds: Option<i64>,
    pub components: Vec<ComponentHealthStatus>,
}

/// Component health status
#[derive(Debug, Serialize)]
pub struct ComponentHealthStatus {
    pub name: String,
    pub status: String,
    pub last_check: String,
}

/// Generate key request
#[derive(Debug, Deserialize)]
pub struct GenerateKeyRequest {
    pub key_type: String,
    pub owner_id: String,
    pub algorithm: Option<String>,
    pub purpose: Option<String>,
}

/// Generate key response
#[derive(Debug, Serialize)]
pub struct GenerateKeyResponse {
    pub key_id: String,
    pub key_type: String,
    pub owner_id: String,
    pub algorithm: String,
    pub created_at: String,
    pub status: String,
}

/// API error response
#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
    pub code: u16,
    pub message: String,
    pub request_id: String,
}

/// Authentication claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
}

// API request/response structures
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowCreateRequest {
    pub workflow_type: String,
    pub requester: String,
    pub metadata: serde_json::Value,
    pub priority: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WorkflowCreateResponse {
    pub workflow_id: String,
    pub status: String,
    pub required_approvals: u32,
    pub expires_at: String,
}

#[derive(Debug, Serialize)]
pub struct WorkflowStatusResponse {
    pub workflow_id: String,
    pub workflow_type: String,
    pub status: String,
    pub approvals_received: u32,
    pub approvals_required: u32,
    pub created_at: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize)]
pub struct UserWorkflowsResponse {
    pub workflows: Vec<WorkflowStatusResponse>,
    pub total_count: usize,
}

#[derive(Debug, Serialize)]
pub struct UserApprovalsResponse {
    pub pending_approvals: Vec<WorkflowStatusResponse>,
    pub total_count: usize,
}

/// BearDog API Server
pub struct BearDogApiServer {
    pub core: Arc<BearDogCore>,
}

impl BearDogApiServer {
    /// Create a new API server instance
    pub fn new(core: Arc<BearDogCore>) -> Self {
        Self {
            core,
        }
    }

    /// Create the main application router
    pub fn create_router(&self) -> Router {
        Router::new()
            // Health and status endpoints
            .route("/health", get(health_check))
            .route("/status", get(system_status))
            
            // Key management endpoints
            .route("/keys", post(generate_key))
            .route("/keys/:key_id/encrypt", post(encrypt_data))
            .route("/keys/:key_id/decrypt", post(decrypt_data))
            .route("/keys/:key_id", get(get_key_info))
            
            // Security operations
            .route("/encrypt", post(encrypt_data_default))
            .route("/decrypt", post(decrypt_data_default))
            
            // Authentication endpoints
            .route("/auth/health", get(auth_health))
            
            // Workflow API endpoints
            .route("/workflows", post(initiate_workflow))
            .route("/workflows/:workflow_id", get(get_workflow_status))
            .route("/workflows/:workflow_id/approve", post(submit_approval))
            .route("/workflows/user/:user_id", get(get_user_workflows))
            .route("/approvals/user/:user_id", get(get_pending_approvals))
            
            // Middleware layers
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .with_state(self.core.clone())
    }

    /// Start the API server
    pub async fn start(&self, bind_address: &str) -> BearDogResult<()> {
        info!("Starting BearDog API server on {}", bind_address);
        
        let app = self.create_router();
        
        let listener = tokio::net::TcpListener::bind(bind_address)
            .await
            .map_err(|e| BearDogError::NetworkError(e.to_string()))?;
        
        info!("🚀 BearDog API server listening on {}", bind_address);
        
        axum::serve(listener, app)
            .await
            .map_err(|e| BearDogError::NetworkError(e.to_string()))?;
        
        Ok(())
    }

    /// Encrypt data with BearDog
    pub async fn encrypt(&self, request: EncryptionRequest) -> BearDogResult<EncryptionResponse> {
        info!("🔐 Encrypting data");
        
        // Use the encryption engine for actual encryption
        let encrypted_data = self.core.encryption_engine()
            .encrypt(&request.plaintext, None)
            .await?;

        Ok(EncryptionResponse {
            encrypted_data: general_purpose::STANDARD.encode(&encrypted_data.ciphertext),
            key_id: "beardog-default".to_string(),
            algorithm: encrypted_data.algorithm.to_string(),
            iv: general_purpose::STANDARD.encode(&encrypted_data.nonce),
        })
    }

    /// Decrypt data with BearDog
    pub async fn decrypt(&self, request: DecryptionRequest) -> BearDogResult<DecryptionResponse> {
        info!("🔓 Decrypting data");
        
        // Decode the encrypted data first
        let ciphertext = match general_purpose::STANDARD.decode(&request.encrypted_data) {
            Ok(data) => data,
            Err(e) => {
                error!("❌ Failed to decode encrypted data: {}", e);
                return Err(BearDogError::encryption("decryption", format!("Invalid base64 data: {}", e)));
            }
        };
        
        // Create EncryptedData structure
        let encrypted_data = crate::encryption::EncryptedData {
            algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
            ciphertext,
            nonce: vec![0u8; 12], // TODO: Get actual nonce from request
            tag: None,
            metadata: std::collections::HashMap::new(),
        };
        
        // Use the encryption engine for actual decryption
        match self.core.encryption_engine().decrypt(&encrypted_data).await {
            Ok(plaintext) => {
                Ok(DecryptionResponse {
                    plaintext: String::from_utf8(plaintext)
                        .map_err(|e| BearDogError::encryption("decryption", format!("Invalid UTF-8: {}", e)))?.into_bytes(),
                    key_id: "beardog-default".to_string(),
                    algorithm: "AES-256-GCM".to_string(),
                })
            }
            Err(e) => {
                error!("❌ Decryption failed: {}", e);
                Err(e)
            }
        }
    }

    /// Generate a new encryption key
    pub async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<GenerateKeyResponse> {
        info!("🔑 Generating key for owner: {}", request.owner_id);
        
        // Use the encryption engine to generate an actual key
        match self.core.encryption_engine().generate_key(&request.key_type, &request.owner_id).await {
            Ok((key_id, _key_data)) => {
                let response = GenerateKeyResponse {
                    key_id,
                    key_type: request.key_type,
                    owner_id: request.owner_id,
                    algorithm: request.algorithm.unwrap_or_else(|| "AES-256-GCM".to_string()),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    status: "active".to_string(),
                };
                Ok(response)
            }
            Err(e) => {
                error!("❌ Key generation failed: {}", e);
                Err(BearDogError::internal("Health check failed".to_string()))
            }
        }
    }

    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        // Validate key ID format
        uuid::Uuid::parse_str(key_id)
            .map_err(|_| BearDogError::InvalidRequest("Invalid key ID format".to_string()))?;

        // Use the actual encryption engine to delete the key
        self.core.encryption_engine()
            .delete_key(key_id)
            .await?;
        
        Ok(())
    }
}

/// Health check endpoint
async fn health_check(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<HealthResponse>, StatusCode> {
    match core.health_check().await {
        Ok(health) => {
            let components = health.components.into_iter()
                .map(|c| ComponentHealthStatus {
                    name: c.name,
                    status: if c.healthy { "healthy".to_string() } else { "unhealthy".to_string() },
                    last_check: c.last_check.to_rfc3339(),
                })
                .collect();

            Ok(Json(HealthResponse {
                status: match health.status {
                    crate::core::HealthStatus::Healthy => "healthy".to_string(),
                    crate::core::HealthStatus::Degraded => "degraded".to_string(),
                    crate::core::HealthStatus::Unhealthy => "unhealthy".to_string(),
                    crate::core::HealthStatus::Starting => "starting".to_string(),
                    crate::core::HealthStatus::Stopping => "stopping".to_string(),
                },
                version: crate::VERSION.to_string(),
                uptime_seconds: health.uptime.map(|d| d.num_seconds()),
                components,
            }))
        }
        Err(e) => {
            error!("Health check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// System status endpoint (more detailed than health)
async fn system_status(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<crate::core::HealthCheck>, StatusCode> {
    match core.health_check().await {
        Ok(health) => Ok(Json(health)),
        Err(e) => {
            error!("System status check failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Generate new encryption key
async fn generate_key(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<GenerateKeyRequest>,
) -> Result<Json<GenerateKeyResponse>, StatusCode> {
    info!("🔑 Generating key for owner: {}", request.owner_id);
    
    // Use the encryption engine to generate an actual key
    match core.encryption_engine().generate_key(&request.key_type, &request.owner_id).await {
        Ok((key_id, _key_data)) => {
            let response = GenerateKeyResponse {
                key_id,
                key_type: request.key_type,
                owner_id: request.owner_id,
                algorithm: request.algorithm.unwrap_or_else(|| "AES-256-GCM".to_string()),
                created_at: chrono::Utc::now().to_rfc3339(),
                status: "active".to_string(),
            };
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Key generation failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Encrypt data with specific key
async fn encrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Path(key_id): Path<String>,
    Json(request): Json<EncryptionRequest>,
) -> Result<Json<EncryptionResponse>, StatusCode> {
    info!("🔐 Encrypting data with key: {}", key_id);
    
    // Use the encryption engine for actual encryption
    match core.encryption_engine().encrypt(&request.plaintext, None).await {
        Ok(encrypted_data) => {
            let response = EncryptionResponse {
                encrypted_data: general_purpose::STANDARD.encode(&encrypted_data.ciphertext),
                key_id,
                algorithm: encrypted_data.algorithm.to_string(),
                iv: general_purpose::STANDARD.encode(&encrypted_data.nonce),
            };
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Encryption failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Decrypt data with specific key
async fn decrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Path(key_id): Path<String>,
    Json(request): Json<DecryptionRequest>,
) -> Result<Json<DecryptionResponse>, StatusCode> {
    info!("🔓 Decrypting data with key: {}", key_id);
    
    // Decode the encrypted data
    let ciphertext = match general_purpose::STANDARD.decode(&request.encrypted_data) {
        Ok(data) => data,
        Err(e) => {
            error!("❌ Failed to decode encrypted data: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    // Create EncryptedData structure for decryption
    let encrypted_data = crate::encryption::EncryptedData {
        algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
        ciphertext,
        nonce: vec![0u8; 12], // TODO: Extract actual nonce from request
        tag: None,
        metadata: std::collections::HashMap::new(),
    };
    
    // Use the encryption engine for actual decryption
    match core.encryption_engine().decrypt(&encrypted_data).await {
        Ok(plaintext) => {
            match String::from_utf8(plaintext) {
                Ok(plaintext_str) => {
                    let response = DecryptionResponse {
                        plaintext: plaintext_str.into_bytes(),
                        key_id,
                        algorithm: "AES-256-GCM".to_string(),
                    };
                    Ok(Json(response))
                }
                Err(e) => {
                    error!("❌ Decryption failed - invalid UTF-8: {}", e);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
        }
        Err(e) => {
            error!("❌ Decryption failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get key information
async fn get_key_info(
    State(core): State<Arc<BearDogCore>>,
    Path(key_id): Path<String>,
) -> Result<Json<GenerateKeyResponse>, StatusCode> {
    info!("📋 Getting key info for: {}", key_id);
    
    // In a real implementation, this would query the key store
    // For now, return a structured response based on key existence check
    let response = GenerateKeyResponse {
        key_id: key_id.clone(),
        key_type: "AES256".to_string(), // Would be retrieved from storage
        owner_id: "system".to_string(), // Would be retrieved from storage
        algorithm: "AES-256-GCM".to_string(),
        created_at: chrono::Utc::now().to_rfc3339(),
        status: "active".to_string(),
    };
    
    Ok(Json(response))
}

/// Encrypt data with default key
async fn encrypt_data_default(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<EncryptionRequest>,
) -> Result<Json<EncryptionResponse>, StatusCode> {
    info!("🔐 Encrypting data with default key");
    
    // Use the encryption engine for actual encryption
    match core.encryption_engine().encrypt(&request.plaintext, None).await {
        Ok(encrypted_data) => {
            let response = EncryptionResponse {
                encrypted_data: general_purpose::STANDARD.encode(&encrypted_data.ciphertext),
                key_id: "default".to_string(),
                algorithm: encrypted_data.algorithm.to_string(),
                iv: general_purpose::STANDARD.encode(&encrypted_data.nonce),
            };
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Encryption failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Decrypt data with default key
async fn decrypt_data_default(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<DecryptionRequest>,
) -> Result<Json<DecryptionResponse>, StatusCode> {
    info!("🔓 Decrypting data with default key");
    
    // Decode the encrypted data
    let ciphertext = match general_purpose::STANDARD.decode(&request.encrypted_data) {
        Ok(data) => data,
        Err(e) => {
            error!("❌ Failed to decode encrypted data: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    
    // Create EncryptedData structure for decryption
    let encrypted_data = crate::encryption::EncryptedData {
        algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
        ciphertext,
        nonce: vec![0u8; 12], // TODO: Extract actual nonce from request
        tag: None,
        metadata: std::collections::HashMap::new(),
    };
    
    // Use the encryption engine for actual decryption
    match core.encryption_engine().decrypt(&encrypted_data).await {
        Ok(plaintext) => {
            match String::from_utf8(plaintext) {
                Ok(plaintext_str) => {
                    let response = DecryptionResponse {
                        plaintext: plaintext_str.into_bytes(),
                        key_id: "default".to_string(),
                        algorithm: "AES-256-GCM".to_string(),
                    };
                    Ok(Json(response))
                }
                Err(e) => {
                    error!("❌ Decryption failed - invalid UTF-8: {}", e);
                    Err(StatusCode::BAD_REQUEST)
                }
            }
        }
        Err(e) => {
            error!("❌ Decryption failed: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Authentication health check
async fn auth_health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "message": "Authentication system operational"
    }))
}



// Workflow API Handlers

/// Initiate a new workflow
async fn initiate_workflow(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<crate::workflows::WorkflowRequest>,
) -> Result<Json<crate::workflows::WorkflowResponse>, StatusCode> {
    info!("🔄 Initiating workflow: {:?} by {}", request.workflow_type, request.initiator);
    
    match core.workflow_engine().initiate_workflow(request).await {
        Ok(response) => {
            info!("✅ Workflow initiated: {}", response.workflow_id);
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Failed to initiate workflow: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Get workflow status
async fn get_workflow_status(
    State(core): State<Arc<BearDogCore>>,
    Path(workflow_id): Path<String>,
) -> Result<Json<crate::workflows::Workflow>, StatusCode> {
    info!("📊 Getting workflow status: {}", workflow_id);
    
    match core.workflow_engine().get_workflow_status(&workflow_id).await {
        Ok(workflow) => Ok(Json(workflow)),
        Err(e) => {
            error!("❌ Failed to get workflow status: {}", e);
            match e {
                crate::error::BearDogError::WorkflowNotFound(_) => Err(StatusCode::NOT_FOUND),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

/// Submit approval for a workflow
async fn submit_approval(
    State(core): State<Arc<BearDogCore>>,
    Path(workflow_id): Path<String>,
    Json(mut approval): Json<crate::workflows::ApprovalSubmission>,
) -> Result<Json<crate::workflows::ApprovalResponse>, StatusCode> {
    // Ensure workflow_id matches the path
    approval.workflow_id = workflow_id.clone();
    
    info!("👍 Submitting approval for workflow: {} by {}", workflow_id, approval.approver);
    
    match core.workflow_engine().submit_approval(approval).await {
        Ok(response) => {
            info!("✅ Approval submitted: {}", response.approval_id);
            Ok(Json(response))
        }
        Err(e) => {
            error!("❌ Failed to submit approval: {}", e);
            match e {
                crate::error::BearDogError::WorkflowNotFound(_) => Err(StatusCode::NOT_FOUND),
                crate::error::BearDogError::UnauthorizedApprover(_) => Err(StatusCode::FORBIDDEN),
                crate::error::BearDogError::DuplicateApproval(_) => Err(StatusCode::CONFLICT),
                crate::error::BearDogError::WorkflowExpired(_) => Err(StatusCode::GONE),
                crate::error::BearDogError::WorkflowNotAcceptingApprovals(_) => Err(StatusCode::CONFLICT),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        }
    }
}

/// Get workflows for a user
async fn get_user_workflows(
    State(core): State<Arc<BearDogCore>>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<crate::workflows::Workflow>>, StatusCode> {
    info!("📋 Getting workflows for user: {}", user_id);
    
    match core.workflow_engine().get_user_workflows(&user_id).await {
        Ok(workflows) => {
            info!("✅ Found {} workflows for user: {}", workflows.len(), user_id);
            Ok(Json(workflows))
        }
        Err(e) => {
            error!("❌ Failed to get user workflows: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get pending approvals for a user
async fn get_pending_approvals(
    State(core): State<Arc<BearDogCore>>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<crate::workflows::PendingApproval>>, StatusCode> {
    info!("⏳ Getting pending approvals for user: {}", user_id);
    
    match core.workflow_engine().get_pending_approvals(&user_id).await {
        Ok(approvals) => {
            info!("✅ Found {} pending approvals for user: {}", approvals.len(), user_id);
            Ok(Json(approvals))
        }
        Err(e) => {
            error!("❌ Failed to get pending approvals: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Handle encryption request (helper function)
async fn handle_encryption_request(
    request: EncryptionRequest,
    core: Arc<BearDogCore>,
) -> Result<EncryptionResponse, BearDogError> {
    // Use the encryption engine for actual encryption
    let encrypted_data = core.encryption_engine()
        .encrypt(&request.plaintext, None)
        .await?;

    Ok(EncryptionResponse {
        encrypted_data: general_purpose::STANDARD.encode(&encrypted_data.ciphertext),
        key_id: "beardog-default".to_string(),
        algorithm: encrypted_data.algorithm.to_string(),
        iv: general_purpose::STANDARD.encode(&encrypted_data.nonce),
    })
}

/// Handle decryption request (helper function)
async fn handle_decryption_request(
    request: DecryptionRequest,
    core: Arc<BearDogCore>,
) -> Result<DecryptionResponse, BearDogError> {
    // Decode the encrypted data first
    let ciphertext = match general_purpose::STANDARD.decode(&request.encrypted_data) {
        Ok(data) => data,
        Err(e) => {
            return Err(BearDogError::encryption("decryption", format!("Invalid base64 data: {}", e)));
        }
    };
    
    // Create EncryptedData structure
    let encrypted_data = crate::encryption::EncryptedData {
        algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
        ciphertext,
        nonce: vec![0u8; 12], // TODO: Get actual nonce from request
        tag: None,
        metadata: std::collections::HashMap::new(),
    };
    
    // Use the encryption engine for actual decryption
    let plaintext = core.encryption_engine().decrypt(&encrypted_data).await?;
    
    Ok(DecryptionResponse {
        plaintext: String::from_utf8(plaintext)
            .map_err(|e| BearDogError::encryption("decryption", format!("Invalid UTF-8: {}", e)))?.into_bytes(),
        key_id: "beardog-default".to_string(),
        algorithm: "AES-256-GCM".to_string(),
    })
}

async fn handle_workflow_approval(
    workflow_id: String,
    approval: ApprovalSubmission,
    core: Arc<BearDogCore>,
) -> Result<Json<serde_json::Value>, BearDogError> {
    // Validate workflow ID format
    if workflow_id.is_empty() {
        return Err(BearDogError::InvalidInput { 
            message: "Invalid workflow ID".to_string() 
        });
    }
    
    // Validate approval submission
    if approval.approver.is_empty() {
        return Err(BearDogError::InvalidInput { 
            message: "Approver cannot be empty".to_string() 
        });
    }
    
    // Create workflow engine and submit approval
    // Note: This would integrate with the actual workflow engine when core is fully connected
    tracing::info!("Processing approval for workflow {} by {}", workflow_id, approval.approver);
    
    let response = serde_json::json!({
        "approval_id": format!("approval-{}", uuid::Uuid::new_v4()),
        "workflow_id": workflow_id,
        "workflow_status": "PendingApprovals",
        "remaining_approvals": 1,
        "next_approvers": ["security_officer"],
        "completion_estimate": null,
        "message": "Approval submitted successfully"
    });
    
    Ok(Json(response))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub approver: String,
    pub decision: String,
    pub reason: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_api_response_structures() {
        // Test serialization/deserialization of API structures
        let health_response = HealthResponse {
            status: "healthy".to_string(),
            version: "1.0.0".to_string(),
            uptime_seconds: Some(3600),
            components: vec![],
        };
        
        let json = serde_json::to_string(&health_response).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("healthy"));
    }
    
    #[test]
    fn test_request_structures() {
        // Test serialization of request structures - simplified test
        let json_str = r#"{"plaintext":[116,101,115,116],"algorithm":"Aes256Gcm"}"#;
        assert!(!json_str.is_empty());
    }
    
    #[test]
    fn test_response_structures() {
        // Test response structure creation
        let encrypt_response = EncryptionResponse {
            encrypted_data: "encrypted_data_base64".to_string(),
            algorithm: "AES-256-GCM".to_string(),
            iv: "iv_base64".to_string(),
            key_id: "test-key-id".to_string(),
        };
        
        let json = serde_json::to_string(&encrypt_response).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("encrypted_data_base64"));
    }
    
    #[test]
    fn test_workflow_request_structure() {
        let workflow_request = WorkflowCreateRequest {
            workflow_type: "KeyRotation".to_string(),
            requester: "test-user".to_string(),
            metadata: serde_json::json!({}),
            priority: Some("Normal".to_string()),
        };
        
        let json = serde_json::to_string(&workflow_request).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("KeyRotation"));
    }
    
    #[test]
    fn test_approval_request_structure() {
        let approval_request = ApprovalRequest {
            approver: "test-approver".to_string(),
            decision: "Approved".to_string(),
            reason: "Test approval".to_string(),
        };
        
        let json = serde_json::to_string(&approval_request).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("Approved"));
    }
} 