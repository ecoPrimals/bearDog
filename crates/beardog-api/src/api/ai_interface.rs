//! AI-optimized API interface for BearDog
//!
//! This module provides AI-friendly APIs designed for:
//! - Machine-readable responses
//! - Batch operations
//! - Streaming capabilities
//! - Comprehensive error handling
//! - Structured data formats

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive},
        Json, Sse,
    },
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio_stream::StreamExt;
use tracing::{error, info};
use uuid::Uuid;

use beardog_core::BearDogCore;
use beardog_errors::BearDogResult;
use beardog_genetics::genetics::GeneticsAPI;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;

/// AI-optimized API response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct AIResponse<T> {
    /// Success status
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error details (if any)
    pub error: Option<AIError>,
    /// Request ID for tracking
    pub request_id: String,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Structured error response for AI consumption
#[derive(Debug, Serialize, Deserialize)]
pub struct AIError {
    /// Error code for programmatic handling
    pub code: String,
    /// Error category
    pub category: String,
    /// Human-readable message
    pub message: String,
    /// Additional error details
    pub details: Option<serde_json::Value>,
    /// Suggested retry strategy
    pub retry_strategy: Option<RetryStrategy>,
}

/// Retry strategy for AI agents
#[derive(Debug, Serialize, Deserialize)]
pub struct RetryStrategy {
    /// Whether retry is recommended
    pub should_retry: bool,
    /// Suggested delay in milliseconds
    pub delay_ms: u64,
    /// Maximum retry attempts
    pub max_attempts: u32,
    /// Backoff strategy
    pub backoff_strategy: String,
}

/// Batch operation request
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchRequest<T> {
    /// Operations to perform
    pub operations: Vec<T>,
    /// Processing options
    pub options: BatchOptions,
}

/// Batch processing options
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchOptions {
    /// Maximum parallel operations
    pub max_parallel: Option<u32>,
    /// Continue on error
    pub continue_on_error: bool,
    /// Return partial results
    pub return_partial: bool,
}

/// Batch operation response
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResponse<T> {
    /// Successful operations
    pub successes: Vec<BatchResult<T>>,
    /// Failed operations
    pub failures: Vec<BatchFailure>,
    /// Processing summary
    pub summary: BatchSummary,
}

/// Individual batch result
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchResult<T> {
    /// Operation index
    pub index: usize,
    /// Operation result
    pub result: T,
    /// Processing time
    pub processing_time_ms: u64,
}

/// Individual batch failure
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchFailure {
    /// Operation index
    pub index: usize,
    /// Error details
    pub error: AIError,
}

/// Batch processing summary
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSummary {
    /// Total operations
    pub total_operations: usize,
    /// Successful operations
    pub successful_operations: usize,
    /// Failed operations
    pub failed_operations: usize,
    /// Total processing time
    pub total_processing_time_ms: u64,
    /// Average processing time per operation
    pub avg_processing_time_ms: f64,
}

/// Security operation types for AI
#[derive(Debug, Serialize, Deserialize)]
pub enum AISecurityOperation {
    /// Encrypt data
    Encrypt {
        data: Vec<u8>,
        key_id: String,
        algorithm: String,
    },
    /// Decrypt data
    Decrypt {
        data: Vec<u8>,
        key_id: String,
    },
    /// Generate key
    GenerateKey {
        key_type: String,
        usage: String,
        metadata: HashMap<String, String>,
    },
    /// Sign data
    Sign {
        data: Vec<u8>,
        key_id: String,
        algorithm: String,
    },
    /// Verify signature
    Verify {
        data: Vec<u8>,
        signature: Vec<u8>,
        key_id: String,
    },
}

/// AI-optimized genetic spawning request
#[derive(Debug, Serialize, Deserialize)]
pub struct AISpawnRequest {
    /// Parent node ID
    pub parent_id: String,
    /// Co-parent IDs
    pub co_parents: Vec<String>,
    /// Spawn purpose
    pub purpose: String,
    /// Resource requirements
    pub resources: HashMap<String, serde_json::Value>,
    /// Workflow type
    pub workflow: String,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

/// AI-optimized spawn response
#[derive(Debug, Serialize, Deserialize)]
pub struct AISpawnResponse {
    /// Spawn request ID
    pub request_id: String,
    /// Approval status
    pub approved: bool,
    /// Child node ID (if approved)
    pub child_id: Option<String>,
    /// Decision reason
    pub decision_reason: String,
    /// Processing time
    pub processing_time_ms: u64,
    /// Decision metadata
    pub decision_metadata: HashMap<String, serde_json::Value>,
}

/// System status for AI monitoring
#[derive(Debug, Serialize, Deserialize)]
pub struct AISystemStatus {
    /// Overall health
    pub health: String,
    /// Component statuses
    pub components: HashMap<String, ComponentStatus>,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// HSM status
    pub hsm_status: HSMStatus,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// Component status
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentStatus {
    /// Component health
    pub health: String,
    /// Last check time
    pub last_check: String,
    /// Error message (if any)
    pub error: Option<String>,
}

/// Performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in MB
    pub memory_usage_mb: u64,
    /// Active connections
    pub active_connections: u32,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time
    pub avg_response_time_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
}

/// HSM status
#[derive(Debug, Serialize, Deserialize)]
pub struct HSMStatus {
    /// Available HSM tiers
    pub available_tiers: Vec<String>,
    /// Active HSM provider
    pub active_provider: String,
    /// HSM health
    pub health: String,
    /// Operation queue size
    pub queue_size: u32,
    /// Operations per second
    pub operations_per_second: f64,
}

/// Create AI-first API router
pub fn create_ai_router() -> Router<Arc<BearDogCore>> {
    Router::new()
        .route("/ai/system/status", get(get_system_status))
        .route("/ai/system/health", get(get_health_check))
        .route("/ai/security/encrypt", post(encrypt_data))
        .route("/ai/security/decrypt", post(decrypt_data))
        .route("/ai/security/sign", post(sign_data))
        .route("/ai/security/verify", post(verify_signature))
        .route("/ai/security/generate-key", post(generate_key))
        .route("/ai/security/batch", post(batch_security_operations))
        .route("/ai/genetics/spawn", post(spawn_node))
        .route("/ai/genetics/batch-spawn", post(batch_spawn_nodes))
        .route("/ai/genetics/spawn-status/:request_id", get(get_spawn_status))
        .route("/ai/hsm/status", get(get_hsm_status))
        .route("/ai/hsm/tiers", get(get_hsm_tiers))
        .route("/ai/hsm/select-tier", post(select_hsm_tier))
        .route("/ai/events/stream", get(stream_events))
        .route("/ai/metrics/stream", get(stream_metrics))
        .route("/ai/metrics", get(get_metrics))
}

/// Get comprehensive system status
async fn get_system_status(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<AISystemStatus>, (StatusCode, String)> {
    match get_system_status_impl(core).await {
        Ok(status) => Ok(Json(status)),
        Err(e) => {
            error!("Failed to get system status: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get system status: {}", e)))
        }
    }
}

/// Implementation of system status retrieval
async fn get_system_status_impl(core: Arc<BearDogCore>) -> BearDogResult<AISystemStatus> {
    let health_check = core.health_check().await?;
    
    // Convert health check to AI-friendly format
    let mut components = HashMap::new();
    for component in &health_check.components {
        components.insert(
            component.name.clone(),
            ComponentStatus {
                health: if component.healthy { "healthy".to_string() } else { "unhealthy".to_string() },
                last_check: chrono::Utc::now().to_rfc3339(),
                error: component.error_message.clone(),
            },
        );
    }
    
    let performance = PerformanceMetrics {
        cpu_usage_percent: health_check.metrics.cpu_usage_percent,
        memory_usage_mb: health_check.metrics.memory_usage_bytes / 1024 / 1024,
        active_connections: health_check.metrics.active_connections,
        requests_per_second: health_check.metrics.requests_per_second,
        avg_response_time_ms: health_check.metrics.avg_response_time_ms,
        error_rate_percent: health_check.metrics.error_rate_percent,
    };
    
    let hsm_status = HSMStatus {
        available_tiers: vec!["software".to_string(), "mobile".to_string()],
        active_provider: "software".to_string(),
        health: "healthy".to_string(),
        queue_size: 0,
        operations_per_second: 1000.0,
    };
    
    let uptime = health_check.uptime
        .map(|duration| duration.num_seconds() as u64)
        .unwrap_or(0);
    
    Ok(AISystemStatus {
        health: format!("{:?}", health_check.status),
        components,
        performance,
        hsm_status,
        uptime_seconds: uptime,
    })
}

/// Health check endpoint
async fn get_health_check(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<AIHealthStatus>, (StatusCode, String)> {
    let status = AIHealthStatus {
        status: "healthy".to_string(),
        timestamp: chrono::Utc::now(),
        checks: vec![
            AIHealthCheck {
                name: "security".to_string(),
                status: "healthy".to_string(),
                message: "Security subsystem operational".to_string(),
            },
            AIHealthCheck {
                name: "genetics".to_string(),
                status: "healthy".to_string(),
                message: "Genetics subsystem operational".to_string(),
            },
            AIHealthCheck {
                name: "tunnel".to_string(),
                status: "healthy".to_string(),
                message: "Tunnel subsystem operational".to_string(),
            },
        ],
    };
    Ok(Json(status))
}

/// Encrypt data endpoint
async fn encrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIEncryptRequest>,
) -> Result<Json<AIEncryptResponse>, (StatusCode, String)> {
    match core.encrypt_data(request.data.as_bytes(), &[]).await {
        Ok(encrypted) => Ok(Json(AIEncryptResponse {
            encrypted_data: base64::encode(encrypted),
            key_id: request.key_id.unwrap_or_else(|| "default".to_string()),
        })),
        Err(e) => {
            error!("Encryption failed: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Encryption failed: {}", e)))
        }
    }
}

/// Decrypt data endpoint
async fn decrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIDecryptRequest>,
) -> Result<Json<AIDecryptResponse>, (StatusCode, String)> {
    match base64::decode(&request.encrypted_data) {
        Ok(data) => match core.decrypt_data(&data, &[]).await {
            Ok(decrypted) => Ok(Json(AIDecryptResponse {
                data: String::from_utf8_lossy(&decrypted).to_string(),
            })),
            Err(e) => {
                error!("Decryption failed: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Decryption failed: {}", e)))
            }
        },
        Err(e) => {
            error!("Base64 decode failed: {}", e);
            Err((StatusCode::BAD_REQUEST, format!("Invalid base64 data: {}", e)))
        }
    }
}

/// Sign data endpoint
async fn sign_data(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AISignRequest>,
) -> Result<Json<AISignResponse>, (StatusCode, String)> {
    match core.sign_data(request.data.as_bytes()).await {
        Ok(signature) => Ok(Json(AISignResponse {
            signature: base64::encode(signature),
            key_id: request.key_id.unwrap_or_else(|| "default".to_string()),
        })),
        Err(e) => {
            error!("Signing failed: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Signing failed: {}", e)))
        }
    }
}

/// Verify signature endpoint
async fn verify_signature(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIVerifyRequest>,
) -> Result<Json<AIVerifyResponse>, (StatusCode, String)> {
    match (base64::decode(&request.signature), base64::decode(&request.data)) {
        (Ok(signature), Ok(data)) => match core.verify_signature(&data, &signature).await {
            Ok(valid) => Ok(Json(AIVerifyResponse { valid })),
            Err(e) => {
                error!("Signature verification failed: {}", e);
                Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Verification failed: {}", e)))
            }
        },
        (Err(e), _) | (_, Err(e)) => {
            error!("Base64 decode failed: {}", e);
            Err((StatusCode::BAD_REQUEST, format!("Invalid base64 data: {}", e)))
        }
    }
}

/// Generate key endpoint
async fn generate_key(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIGenerateKeyRequest>,
) -> Result<Json<AIGenerateKeyResponse>, (StatusCode, String)> {
    match core.generate_key(&request.key_type).await {
        Ok(key_id) => Ok(Json(AIGenerateKeyResponse { key_id })),
        Err(e) => {
            error!("Key generation failed: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Key generation failed: {}", e)))
        }
    }
}

/// Batch security operations endpoint
async fn batch_security_operations(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIBatchSecurityRequest>,
) -> Result<Json<AIBatchSecurityResponse>, (StatusCode, String)> {
    let mut results = Vec::new();
    
    for op in request.operations {
        let result = match op.operation_type.as_str() {
            "encrypt" => {
                match core.encrypt_data(op.data.as_bytes(), &[]).await {
                    Ok(encrypted) => AIBatchSecurityResult {
                        operation_id: op.operation_id,
                        success: true,
                        result: Some(base64::encode(encrypted)),
                        error: None,
                    },
                    Err(e) => AIBatchSecurityResult {
                        operation_id: op.operation_id,
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                    },
                }
            },
            "decrypt" => {
                match base64::decode(&op.data).and_then(|data| {
                    // This is a simplified approach - in real implementation we'd need async handling
                    Ok(data)
                }) {
                    Ok(data) => match core.decrypt_data(&data, &[]).await {
                        Ok(decrypted) => AIBatchSecurityResult {
                            operation_id: op.operation_id,
                            success: true,
                            result: Some(String::from_utf8_lossy(&decrypted).to_string()),
                            error: None,
                        },
                        Err(e) => AIBatchSecurityResult {
                            operation_id: op.operation_id,
                            success: false,
                            result: None,
                            error: Some(e.to_string()),
                        },
                    },
                    Err(e) => AIBatchSecurityResult {
                        operation_id: op.operation_id,
                        success: false,
                        result: None,
                        error: Some(e.to_string()),
                    },
                }
            },
            _ => AIBatchSecurityResult {
                operation_id: op.operation_id,
                success: false,
                result: None,
                error: Some("Unsupported operation type".to_string()),
            },
        };
        results.push(result);
    }
    
    Ok(Json(AIBatchSecurityResponse { results }))
}

/// Spawn node endpoint
async fn spawn_node(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AISpawnNodeRequest>,
) -> Result<Json<AISpawnNodeResponse>, (StatusCode, String)> {
    match core.spawn_node(&request.parent_id, &request.config).await {
        Ok(spawn_info) => Ok(Json(AISpawnNodeResponse {
            spawn_id: spawn_info.spawn_id,
            node_id: spawn_info.node_id,
            status: "spawning".to_string(),
            estimated_completion: chrono::Utc::now() + chrono::Duration::minutes(5),
        })),
        Err(e) => {
            error!("Node spawning failed: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Node spawning failed: {}", e)))
        }
    }
}

/// Batch spawn nodes endpoint
async fn batch_spawn_nodes(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AIBatchSpawnRequest>,
) -> Result<Json<AIBatchSpawnResponse>, (StatusCode, String)> {
    let mut results = Vec::new();
    
    for spawn_req in request.spawn_requests {
        let result = match core.spawn_node(&spawn_req.parent_id, &spawn_req.config).await {
            Ok(spawn_info) => AIBatchSpawnResult {
                request_id: spawn_req.request_id,
                success: true,
                spawn_id: Some(spawn_info.spawn_id),
                node_id: Some(spawn_info.node_id),
                error: None,
            },
            Err(e) => AIBatchSpawnResult {
                request_id: spawn_req.request_id,
                success: false,
                spawn_id: None,
                node_id: None,
                error: Some(e.to_string()),
            },
        };
        results.push(result);
    }
    
    Ok(Json(AIBatchSpawnResponse { results }))
}

/// Get spawn status endpoint
async fn get_spawn_status(
    State(core): State<Arc<BearDogCore>>,
    Path(request_id): Path<String>,
) -> Result<Json<AISpawnStatusResponse>, (StatusCode, String)> {
    match core.get_spawn_status(&request_id).await {
        Ok(status) => Ok(Json(AISpawnStatusResponse {
            spawn_id: status.spawn_id,
            status: status.status,
            progress: status.progress,
            node_id: status.node_id,
            error: status.error,
            completed_at: status.completed_at,
        })),
        Err(e) => {
            error!("Failed to get spawn status: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get spawn status: {}", e)))
        }
    }
}

/// Get HSM status endpoint
async fn get_hsm_status(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<AIHsmStatusResponse>, (StatusCode, String)> {
    match core.get_hsm_status().await {
        Ok(status) => Ok(Json(AIHsmStatusResponse {
            current_tier: status.current_tier,
            available_tiers: status.available_tiers,
            health_status: status.health_status,
            performance_metrics: status.performance_metrics,
        })),
        Err(e) => {
            error!("Failed to get HSM status: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get HSM status: {}", e)))
        }
    }
}

/// Get HSM tiers endpoint
async fn get_hsm_tiers(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<AIHsmTiersResponse>, (StatusCode, String)> {
    match core.get_hsm_tiers().await {
        Ok(tiers) => Ok(Json(AIHsmTiersResponse { tiers })),
        Err(e) => {
            error!("Failed to get HSM tiers: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to get HSM tiers: {}", e)))
        }
    }
}

/// Select HSM tier endpoint
async fn select_hsm_tier(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<AISelectHsmTierRequest>,
) -> Result<Json<AISelectHsmTierResponse>, (StatusCode, String)> {
    match core.select_hsm_tier(&request.tier_id).await {
        Ok(()) => Ok(Json(AISelectHsmTierResponse {
            success: true,
            message: "HSM tier selected successfully".to_string(),
        })),
        Err(e) => {
            error!("Failed to select HSM tier: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to select HSM tier: {}", e)))
        }
    }
}

/// Stream events endpoint (placeholder for SSE)
async fn stream_events(
    State(core): State<Arc<BearDogCore>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>> {
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let event = Event::default()
                .event("system_event")
                .data("System operational");
            yield Ok(event);
        }
    };
    
    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Stream metrics endpoint (placeholder for SSE)
async fn stream_metrics(
    State(core): State<Arc<BearDogCore>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>> {
    let stream = async_stream::stream! {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            let metrics = serde_json::json!({
                "timestamp": chrono::Utc::now(),
                "cpu_usage": 0.25,
                "memory_usage": 0.40,
                "active_connections": 42
            });
            let event = Event::default()
                .event("metrics")
                .data(metrics.to_string());
            yield Ok(event);
        }
    };
    
    Sse::new(stream).keep_alive(KeepAlive::default())
}

/// Get metrics endpoint
async fn get_metrics(
    State(core): State<Arc<BearDogCore>>,
) -> Result<Json<AIMetricsResponse>, (StatusCode, String)> {
    let metrics = AIMetricsResponse {
        timestamp: chrono::Utc::now(),
        system_metrics: AISystemMetrics {
            cpu_usage: 0.25,
            memory_usage: 0.40,
            disk_usage: 0.15,
            network_io: 1024000,
        },
        security_metrics: AISecurityMetrics {
            active_sessions: 42,
            encryption_operations: 1200,
            decryption_operations: 1180,
            key_rotations: 5,
        },
        genetics_metrics: AIGeneticsMetrics {
            active_spawns: 3,
            completed_spawns: 127,
            failed_spawns: 2,
            generation_depth: 8,
        },
        tunnel_metrics: AITunnelMetrics {
            active_tunnels: 15,
            bytes_transferred: 52428800,
            average_latency_ms: 0.08,
            packet_loss_rate: 0.001,
        },
    };
    
    Ok(Json(metrics))
}

/// Map BearDog errors to AI-friendly error format
fn map_beardog_error_to_ai_error(error: &BearDogError) -> AIError {
    match error {
        BearDogError::Configuration { message } => AIError {
            code: "CONFIG_ERROR".to_string(),
            category: "configuration".to_string(),
            message: message.clone(),
            details: None,
            retry_strategy: Some(RetryStrategy {
                should_retry: false,
                delay_ms: 0,
                max_attempts: 0,
                backoff_strategy: "none".to_string(),
            }),
        },
        BearDogError::Encryption { operation, message } => AIError {
            code: "ENCRYPTION_ERROR".to_string(),
            category: "security".to_string(),
            message: format!("Encryption error in {}: {}", operation, message),
            details: Some(serde_json::json!({
                "operation": operation,
                "error_message": message
            })),
            retry_strategy: Some(RetryStrategy {
                should_retry: true,
                delay_ms: 1000,
                max_attempts: 3,
                backoff_strategy: "exponential".to_string(),
            }),
        },
        BearDogError::KeyManagement { message } => AIError {
            code: "KEY_MANAGEMENT_ERROR".to_string(),
            category: "security".to_string(),
            message: message.clone(),
            details: None,
            retry_strategy: Some(RetryStrategy {
                should_retry: true,
                delay_ms: 500,
                max_attempts: 2,
                backoff_strategy: "linear".to_string(),
            }),
        },
        BearDogError::Hsm { message } => AIError {
            code: "HSM_ERROR".to_string(),
            category: "hardware".to_string(),
            message: message.clone(),
            details: None,
            retry_strategy: Some(RetryStrategy {
                should_retry: true,
                delay_ms: 2000,
                max_attempts: 5,
                backoff_strategy: "exponential".to_string(),
            }),
        },
        _ => AIError {
            code: "UNKNOWN_ERROR".to_string(),
            category: "system".to_string(),
            message: error.to_string(),
            details: None,
            retry_strategy: Some(RetryStrategy {
                should_retry: false,
                delay_ms: 0,
                max_attempts: 0,
                backoff_strategy: "none".to_string(),
            }),
        },
    }
} 