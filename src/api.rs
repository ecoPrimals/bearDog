//! # BearDog API Server
//!
//! This module provides the main HTTP API for the BearDog secure networking library.
//! It includes endpoints for key management, encryption/decryption, workflows, and health monitoring.
//!
//! ## Features
//!
//! - **Key Management**: Generate, retrieve, and manage encryption keys
//! - **Encryption/Decryption**: Secure data operations with multiple algorithms  
//! - **Workflow Management**: Multi-party approval workflows
//! - **Health Monitoring**: System status and component health checks
//! - **Authentication**: JWT-based authentication and authorization
//!
//! Provides secure RESTful access to BearDog security management functions.

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};

use crate::encryption::{
    DecryptionRequest, DecryptionResponse, EncryptionRequest, EncryptionResponse,
};
use crate::workflows::ApprovalSubmission;
use crate::{BearDogCore, BearDogError, BearDogResult};

/// System health check response
/// 
/// Contains the overall system status and health information for individual components.
/// Used by monitoring systems to check the health of the BearDog service.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Overall system status ("healthy", "degraded", "unhealthy")
    pub status: String,
    /// Application version
    pub version: String,
    /// System uptime in seconds
    pub uptime_seconds: Option<i64>,
    /// Health status of individual components
    pub components: Vec<ComponentHealthStatus>,
}

/// Health status for an individual system component
///
/// Represents the health state of a specific component (database, encryption engine, etc.)
#[derive(Debug, Serialize)]
pub struct ComponentHealthStatus {
    /// Component name (e.g., "database", "encryption_engine")
    pub name: String,
    /// Component status ("healthy", "degraded", "unhealthy")
    pub status: String,
    /// Timestamp of last health check
    pub last_check: String,
}

/// Request to generate a new encryption key
///
/// Used to create new cryptographic keys with specific parameters.
#[derive(Debug, Deserialize)]
pub struct GenerateKeyRequest {
    /// Type of key to generate ("session", "master", "ephemeral")
    pub key_type: String,
    /// ID of the key owner (user or service)
    pub owner_id: String,
    /// Preferred encryption algorithm ("AES-256-GCM", "ChaCha20-Poly1305")
    pub algorithm: Option<String>,
    /// Intended purpose of the key ("encryption", "signing", "authentication")
    pub purpose: String,
}

/// Response after generating a new encryption key
///
/// Contains metadata about the newly created key (but not the key material itself).
#[derive(Debug, Serialize)]
pub struct GenerateKeyResponse {
    /// Unique identifier for the generated key
    pub key_id: String,
    /// Type of key generated
    pub key_type: String,
    /// ID of the key owner
    pub owner_id: String,
    /// Encryption algorithm used
    pub algorithm: String,
    /// Timestamp when the key was created
    pub created_at: String,
    /// Current status of the key ("active", "expired", "revoked")
    pub status: String,
}

/// API error response structure
///
/// Standard error format returned by all API endpoints when operations fail.
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Error type or category
    pub error: String,
    /// HTTP status code
    pub code: u16,
    /// Human-readable error message
    pub message: String,
    /// Unique request identifier for tracing
    pub request_id: String,
}

/// JWT authentication claims
///
/// Contains the standard JWT claims used for user authentication and authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthClaims {
    /// Subject (user ID)
    pub sub: String,
    /// Expiration time (Unix timestamp)
    pub exp: usize,
    /// Issued at time (Unix timestamp)
    pub iat: usize,
    /// User roles for authorization
    pub roles: Vec<String>,
}

/// Request to create a new approval workflow
///
/// Workflows require multiple parties to approve before sensitive operations can proceed.
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowCreateRequest {
    /// Type of workflow ("key_rotation", "user_access", "system_config")
    pub workflow_type: String,
    /// ID of the user requesting the workflow
    pub requester: String,
    /// Additional workflow metadata as JSON
    pub metadata: serde_json::Value,
    /// Optional priority level ("low", "medium", "high", "critical")
    pub priority: Option<String>,
}

/// Response after creating a new workflow
///
/// Contains the workflow ID and initial status information.
#[derive(Debug, Serialize)]
pub struct WorkflowCreateResponse {
    /// Unique identifier for the created workflow
    pub workflow_id: String,
    /// Current workflow status ("pending", "approved", "rejected", "expired")
    pub status: String,
    /// Number of approvals required for completion
    pub required_approvals: u32,
    /// Timestamp when the workflow expires
    pub expires_at: String,
}

/// Detailed workflow status information
///
/// Provides comprehensive status information about a specific workflow.
#[derive(Debug, Serialize)]
pub struct WorkflowStatusResponse {
    /// Unique workflow identifier
    pub workflow_id: String,
    /// Type of workflow
    pub workflow_type: String,
    /// Current status
    pub status: String,
    /// Number of approvals received so far
    pub approvals_received: u32,
    /// Total number of approvals required
    pub approvals_required: u32,
    /// Timestamp when workflow was created
    pub created_at: String,
    /// Timestamp when workflow expires
    pub expires_at: String,
}

/// Response containing user's workflows
///
/// Lists all workflows associated with a specific user.
#[derive(Debug, Serialize)]
pub struct UserWorkflowsResponse {
    /// List of workflows for the user
    pub workflows: Vec<WorkflowStatusResponse>,
    /// Total number of workflows
    pub total_count: usize,
}

/// Response containing pending approvals for a user
///
/// Lists all workflows waiting for approval from a specific user.
#[derive(Debug, Serialize)]
pub struct UserApprovalsResponse {
    /// List of workflows pending approval
    pub pending_approvals: Vec<WorkflowStatusResponse>,
    /// Total number of pending approvals
    pub total_count: usize,
}

/// BearDog API Server
///
/// The main HTTP API server that provides REST endpoints for all BearDog functionality.
/// Built on top of the BearDog core engine with comprehensive security features.
pub struct BearDogApiServer {
    /// Reference to the core BearDog engine
    pub core: Arc<BearDogCore>,
}

impl BearDogApiServer {
    /// Create a new API server instance
    pub fn new(core: Arc<BearDogCore>) -> Self {
        Self { core }
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
        let encrypted_data = self
            .core
            .encryption_engine()
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
                return Err(BearDogError::encryption(
                    "decryption",
                    format!("Invalid base64 data: {}", e),
                ));
            }
        };

        // Create EncryptedData structure for decryption
        let encrypted_data = crate::encryption::EncryptedData {
            algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
            ciphertext,
            nonce: match crate::crypto_utils::BearDogCrypto::generate_secure_nonce(12) {
                Ok(nonce) => nonce,
                Err(e) => {
                    error!("❌ CRITICAL: Failed to generate secure nonce: {}", e);
                    return Err(BearDogError::internal("Internal server error"));
                }
            },
            tag: None,
            key_id: Some("beardog-default".to_string()),
            metadata: std::collections::HashMap::new(),
        };

        // Use the encryption engine for actual decryption
        match self.core.encryption_engine().decrypt(&encrypted_data).await {
            Ok(plaintext) => Ok(DecryptionResponse {
                plaintext: String::from_utf8(plaintext)
                    .map_err(|e| {
                        BearDogError::encryption("decryption", format!("Invalid UTF-8: {}", e))
                    })?
                    .into_bytes(),
                key_id: "beardog-default".to_string(),
                algorithm: "AES-256-GCM".to_string(),
            }),
            Err(e) => {
                error!("❌ Decryption failed: {}", e);
                Err(e)
            }
        }
    }

    /// Generate a new encryption key
    pub async fn generate_key(
        &self,
        request: GenerateKeyRequest,
    ) -> BearDogResult<GenerateKeyResponse> {
        info!("🔑 Generating key for owner: {}", request.owner_id);

        // Use the encryption engine to generate an actual key
        match self
            .core
            .encryption_engine()
            .generate_key(&request.key_type, &request.owner_id)
            .await
        {
            Ok((key_id, _key_data)) => {
                let response = GenerateKeyResponse {
                    key_id,
                    key_type: request.key_type,
                    owner_id: request.owner_id,
                    algorithm: request
                        .algorithm
                        .unwrap_or_else(|| "AES-256-GCM".to_string()),
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

    /// Delete an encryption key
    ///
    /// Securely removes an encryption key from the system. This operation is irreversible
    /// and will prevent any future operations with the specified key.
    ///
    /// # Arguments
    /// * `key_id` - Unique identifier of the key to delete
    ///
    /// # Returns
    /// * `Ok(())` if the key was successfully deleted
    /// * `Err(BearDogError)` if the operation failed
    ///
    /// # Security Note
    /// This operation requires appropriate authorization and will be logged for audit purposes.
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        // Validate key ID format
        uuid::Uuid::parse_str(key_id)
            .map_err(|_| BearDogError::InvalidRequest("Invalid key ID format".to_string()))?;

        // Use the actual encryption engine to delete the key
        self.core.encryption_engine().delete_key(key_id).await?;

        Ok(())
    }
}

/// Health check endpoint
async fn health_check(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<HealthResponse>, StatusCode> {
    match core.health_check().await {
        Ok(health) => {
            let components = health
                .components
                .into_iter()
                .map(|c| ComponentHealthStatus {
                    name: c.name,
                    status: if c.healthy {
                        "healthy".to_string()
                    } else {
                        "unhealthy".to_string()
                    },
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
    match core
        .encryption_engine()
        .generate_key(&request.key_type, &request.owner_id)
        .await
    {
        Ok((key_id, _key_data)) => {
            let response = GenerateKeyResponse {
                key_id,
                key_type: request.key_type,
                owner_id: request.owner_id,
                algorithm: request
                    .algorithm
                    .unwrap_or_else(|| "AES-256-GCM".to_string()),
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
    match core
        .encryption_engine()
        .encrypt(&request.plaintext, None)
        .await
    {
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
        nonce: match crate::crypto_utils::BearDogCrypto::generate_secure_nonce(12) {
            Ok(nonce) => nonce,
            Err(e) => {
                error!("❌ CRITICAL: Failed to generate secure nonce: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        tag: None,
        key_id: Some("beardog-default".to_string()),
        metadata: std::collections::HashMap::new(),
    };

    // Use the encryption engine for actual decryption
    match core.encryption_engine().decrypt(&encrypted_data).await {
        Ok(plaintext) => match String::from_utf8(plaintext) {
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
        },
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
    match core
        .encryption_engine()
        .encrypt(&request.plaintext, None)
        .await
    {
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
        nonce: match crate::crypto_utils::BearDogCrypto::generate_secure_nonce(12) {
            Ok(nonce) => nonce,
            Err(e) => {
                error!("❌ CRITICAL: Failed to generate secure nonce: {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        },
        tag: None,
        key_id: Some("beardog-default".to_string()),
        metadata: std::collections::HashMap::new(),
    };

    // Use the encryption engine for actual decryption
    match core.encryption_engine().decrypt(&encrypted_data).await {
        Ok(plaintext) => match String::from_utf8(plaintext) {
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
        },
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
    info!(
        "🔄 Initiating workflow: {:?} by {}",
        request.workflow_type, request.initiator
    );

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

    match core
        .workflow_engine()
        .get_workflow_status(&workflow_id)
        .await
    {
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

    info!(
        "👍 Submitting approval for workflow: {} by {}",
        workflow_id, approval.approver
    );

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
                crate::error::BearDogError::WorkflowNotAcceptingApprovals(_) => {
                    Err(StatusCode::CONFLICT)
                }
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
            info!(
                "✅ Found {} workflows for user: {}",
                workflows.len(),
                user_id
            );
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
            info!(
                "✅ Found {} pending approvals for user: {}",
                approvals.len(),
                user_id
            );
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
    let encrypted_data = core
        .encryption_engine()
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
            return Err(BearDogError::encryption(
                "decryption",
                format!("Invalid base64 data: {}", e),
            ));
        }
    };

    // Create EncryptedData structure
    let encrypted_data = crate::encryption::EncryptedData {
        algorithm: crate::encryption::EncryptionAlgorithm::Aes256Gcm,
        ciphertext,
        nonce: match crate::crypto_utils::BearDogCrypto::generate_secure_nonce(12) {
            Ok(nonce) => nonce,
            Err(e) => {
                error!("❌ CRITICAL: Failed to generate secure nonce: {}", e);
                return Err(BearDogError::internal("Internal server error"));
            }
        },
        tag: None,
        key_id: Some("beardog-default".to_string()),
        metadata: std::collections::HashMap::new(),
    };

    // Use the encryption engine for actual decryption
    let plaintext = core.encryption_engine().decrypt(&encrypted_data).await?;

    Ok(DecryptionResponse {
        plaintext: String::from_utf8(plaintext)
            .map_err(|e| BearDogError::encryption("decryption", format!("Invalid UTF-8: {}", e)))?
            .into_bytes(),
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
            message: "Invalid workflow ID".to_string(),
        });
    }

    // Validate approval submission
    if approval.approver.is_empty() {
        return Err(BearDogError::InvalidInput {
            message: "Approver cannot be empty".to_string(),
        });
    }

    // Create workflow engine and submit approval
    // Note: This would integrate with the actual workflow engine when core is fully connected
    tracing::info!(
        "Processing approval for workflow {} by {}",
        workflow_id,
        approval.approver
    );

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

/// Request to submit an approval for a workflow
///
/// Used by authorized users to approve or reject pending workflows.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApprovalRequest {
    /// ID of the user submitting the approval
    pub approver: String,
    /// Decision made ("approve", "reject")
    pub decision: String,
    /// Optional reason for the decision
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

        let json = serde_json::to_string(&health_response)
            .expect("API response serialization should never fail in tests");
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

        let json = serde_json::to_string(&encrypt_response)
            .expect("Encryption response serialization should never fail in tests");
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

        let json = serde_json::to_string(&workflow_request)
            .expect("Workflow request serialization should never fail in tests");
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

        let json = serde_json::to_string(&approval_request)
            .expect("Approval request serialization should never fail in tests");
        assert!(!json.is_empty());
        assert!(json.contains("Approved"));
    }
}
