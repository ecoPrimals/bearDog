//! AI-First CLI Demo
//!
//! Pure Rust demonstration of AI-optimized BearDog operations:
//! - JSON-only output (no human UI)
//! - Batch processing
//! - Machine-readable error codes
//! - Automation-friendly interface
//!
//! Usage:
//! ```bash
//! cargo run --example ai_cli_demo
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};

use beardog_config::BearDogConfig;
use beardog_core::BearDogCore;
use beardog_errors::{BearDogError, BearDogResult};

/// AI-optimized response wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct AiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<AiError>,
    pub request_id: String,
    pub execution_time_ms: u64,
    pub timestamp: String,
}

/// Machine-readable error
#[derive(Debug, Serialize, Deserialize)]
pub struct AiError {
    pub code: String,
    pub message: String,
    pub category: String,
    pub retry_recommended: bool,
    pub retry_delay_ms: u64,
}

/// Security operation request
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityOpRequest {
    pub operation: String,
    pub key_id: String,
    pub data: String,
    pub algorithm: Option<String>,
}

/// Batch security operation
#[derive(Debug, Serialize, Deserialize)]
pub struct BatchSecurityRequest {
    pub operations: Vec<SecurityOpRequest>,
    pub max_parallel: u32,
    pub continue_on_error: bool,
}

/// Genetic spawn request
#[derive(Debug, Serialize, Deserialize)]
pub struct GeneticSpawnRequest {
    pub parent_id: String,
    pub co_parents: Vec<String>,
    pub purpose: String,
    pub resource_limits: HashMap<String, f64>,
}

/// System status response
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub health: String,
    pub uptime_seconds: u64,
    pub components: Vec<ComponentStatus>,
    pub hsm_status: HsmStatus,
    pub performance: PerformanceMetrics,
}

/// Component status
#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentStatus {
    pub name: String,
    pub healthy: bool,
    pub last_check: String,
    pub error_message: Option<String>,
}

/// HSM status
#[derive(Debug, Serialize, Deserialize)]
pub struct HsmStatus {
    pub available_tiers: Vec<String>,
    pub active_tier: String,
    pub health: String,
    pub operations_per_second: f64,
}

/// Performance metrics
#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_mb: u64,
    pub active_connections: u32,
    pub requests_per_second: f64,
    pub avg_response_time_ms: f64,
    pub error_rate_percent: f64,
}

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🤖 AI-First BearDog CLI Demo");
    
    // Initialize BearDog
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;
    core.start().await?;
    
    // Demo 1: System Status (Machine-readable)
    println!("=== SYSTEM STATUS ===");
    let status_response = get_system_status(&core).await;
    println!("{}", serde_json::to_string_pretty(&status_response).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
    
    // Demo 2: Batch Security Operations
    println!("\n=== BATCH SECURITY OPERATIONS ===");
    let batch_request = BatchSecurityRequest {
        operations: vec![
            SecurityOpRequest {
                operation: "encrypt".to_string(),
                key_id: "key_001".to_string(),
                data: "sensitive_data_1".to_string(),
                algorithm: Some("AES256".to_string()),
            },
            SecurityOpRequest {
                operation: "encrypt".to_string(),
                key_id: "key_002".to_string(),
                data: "sensitive_data_2".to_string(),
                algorithm: Some("AES256".to_string()),
            },
            SecurityOpRequest {
                operation: "sign".to_string(),
                key_id: "signing_key_001".to_string(),
                data: "document_to_sign".to_string(),
                algorithm: Some("Ed25519".to_string()),
            },
        ],
        max_parallel: 3,
        continue_on_error: true,
    };
    
    let batch_response = process_batch_security(&core, batch_request).await;
    println!("{}", serde_json::to_string_pretty(&batch_response).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
    
    // Demo 3: Genetic Spawning
    println!("\n=== GENETIC SPAWNING ===");
    let spawn_request = GeneticSpawnRequest {
        parent_id: "parent_node_001".to_string(),
        co_parents: vec!["co_parent_001".to_string()],
        purpose: "LocalProcessing".to_string(),
        resource_limits: {
            let mut limits = HashMap::new();
            limits.insert("cpu_percent".to_string(), 50.0);
            limits.insert("memory_mb".to_string(), 2048.0);
            limits
        },
    };
    
    let spawn_response = process_genetic_spawn(&core, spawn_request).await;
    println!("{}", serde_json::to_string_pretty(&spawn_response).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
    
    // Demo 4: HSM Operations
    println!("\n=== HSM OPERATIONS ===");
    let hsm_response = get_hsm_status(&core).await;
    println!("{}", serde_json::to_string_pretty(&hsm_response).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
    
    // Demo 5: Performance Benchmark
    println!("\n=== PERFORMANCE BENCHMARK ===");
    let benchmark_response = run_performance_benchmark(&core).await;
    println!("{}", serde_json::to_string_pretty(&benchmark_response).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
    
    // Graceful shutdown
    core.stop().await?;
    
    println!("\n✅ AI-First CLI Demo completed - all operations machine-readable!");
    
    Ok(())
}

/// Get system status (AI-optimized)
async fn get_system_status(core: &BearDogCore) -> AiResponse<SystemStatus> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    match core.health_check().await {
        Ok(health) => {
            let components = health.components.into_iter().map(|c| ComponentStatus {
                name: c.name,
                healthy: c.healthy,
                last_check: chrono::Utc::now().to_rfc3339(),
                error_message: c.error_message,
            }).collect();
            
            let status = SystemStatus {
                health: format!("{:?}", health.status),
                uptime_seconds: health.uptime.map(|d| d.num_seconds() as u64).unwrap_or(0),
                components,
                hsm_status: HsmStatus {
                    available_tiers: vec!["software".to_string(), "mobile".to_string()],
                    active_tier: "software".to_string(),
                    health: "healthy".to_string(),
                    operations_per_second: 1000.0,
                },
                performance: PerformanceMetrics {
                    cpu_usage_percent: health.metrics.cpu_usage_percent,
                    memory_usage_mb: health.metrics.memory_usage_bytes / 1024 / 1024,
                    active_connections: health.metrics.active_connections,
                    requests_per_second: health.metrics.requests_per_second,
                    avg_response_time_ms: health.metrics.avg_response_time_ms,
                    error_rate_percent: health.metrics.error_rate_percent,
                },
            };
            
            AiResponse {
                success: true,
                data: Some(status),
                error: None,
                request_id,
                execution_time_ms: start_time.elapsed().as_millis() as u64,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        }
        Err(e) => {
            AiResponse {
                success: false,
                data: None,
                error: Some(AiError {
                    code: "HEALTH_CHECK_FAILED".to_string(),
                    message: e.to_string(),
                    category: "system".to_string(),
                    retry_recommended: true,
                    retry_delay_ms: 1000,
                }),
                request_id,
                execution_time_ms: start_time.elapsed().as_millis() as u64,
                timestamp: chrono::Utc::now().to_rfc3339(),
            }
        }
    }
}

/// Process batch security operations
async fn process_batch_security(
    core: &BearDogCore,
    request: BatchSecurityRequest,
) -> AiResponse<serde_json::Value> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Simulate batch processing
    let mut results = Vec::new();
    let mut successful = 0;
    let mut failed = 0;
    
    for (index, op) in request.operations.iter().enumerate() {
        // Simulate operation processing
        let op_start = Instant::now();
        let op_result = simulate_security_operation(op).await;
        let op_time = op_start.elapsed().as_millis() as u64;
        
        match op_result {
            Ok(result) => {
                successful += 1;
                results.push(serde_json::json!({
                    "index": index,
                    "status": "success",
                    "operation": op.operation,
                    "result": result,
                    "execution_time_ms": op_time,
                }));
            }
            Err(e) => {
                failed += 1;
                results.push(serde_json::json!({
                    "index": index,
                    "status": "failed",
                    "operation": op.operation,
                    "error": e.to_string(),
                    "execution_time_ms": op_time,
                }));
                
                if !request.continue_on_error {
                    break;
                }
            }
        }
    }
    
    let batch_result = serde_json::json!({
        "total_operations": request.operations.len(),
        "successful_operations": successful,
        "failed_operations": failed,
        "max_parallel": request.max_parallel,
        "continue_on_error": request.continue_on_error,
        "results": results,
        "summary": {
            "success_rate": (successful as f64) / (request.operations.len() as f64) * 100.0,
            "total_time_ms": start_time.elapsed().as_millis(),
            "avg_time_per_operation_ms": start_time.elapsed().as_millis() as f64 / request.operations.len() as f64,
        }
    });
    
    AiResponse {
        success: true,
        data: Some(batch_result),
        error: None,
        request_id,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Process genetic spawning
async fn process_genetic_spawn(
    core: &BearDogCore,
    request: GeneticSpawnRequest,
) -> AiResponse<serde_json::Value> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Simulate genetic spawning
    let child_id = uuid::Uuid::new_v4().to_string();
    
    let spawn_result = serde_json::json!({
        "spawn_request_id": request_id,
        "parent_id": request.parent_id,
        "co_parents": request.co_parents,
        "purpose": request.purpose,
        "resource_limits": request.resource_limits,
        "approved": true,
        "child_id": child_id,
        "decision_reason": "Automated approval - sufficient resources available",
        "genetic_traits": {
            "generation": 2,
            "fitness_score": 0.85,
            "inherited_capabilities": ["encryption", "signing", "local_processing"],
            "mutations": ["performance_optimization"],
        },
        "spawning_metrics": {
            "genetic_recombination_time_ms": 45,
            "validation_time_ms": 12,
            "approval_time_ms": 8,
        }
    });
    
    AiResponse {
        success: true,
        data: Some(spawn_result),
        error: None,
        request_id,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Get HSM status
async fn get_hsm_status(core: &BearDogCore) -> AiResponse<serde_json::Value> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    let hsm_status = serde_json::json!({
        "available_tiers": [
            {
                "name": "software",
                "available": true,
                "priority": 2,
                "performance": {
                    "latency_ms": 1.0,
                    "throughput_ops_per_sec": 10000.0,
                    "reliability": 99.9
                }
            },
            {
                "name": "mobile",
                "available": false,
                "priority": 1,
                "performance": {
                    "latency_ms": 100.0,
                    "throughput_ops_per_sec": 500.0,
                    "reliability": 95.0
                }
            }
        ],
        "active_tier": "software",
        "health": "healthy",
        "current_metrics": {
            "queue_size": 0,
            "operations_per_second": 1000.0,
            "success_rate": 99.8,
            "avg_response_time_ms": 1.2
        },
        "routing_strategy": {
            "human_identity": "mobile_preferred",
            "file_encryption": "software_optimized",
            "genetic_spawning": "software_optimized",
            "backup_operations": "software_optimized"
        }
    });
    
    AiResponse {
        success: true,
        data: Some(hsm_status),
        error: None,
        request_id,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Run performance benchmark
async fn run_performance_benchmark(core: &BearDogCore) -> AiResponse<serde_json::Value> {
    let start_time = Instant::now();
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Simulate benchmark operations
    let mut encryption_times = Vec::new();
    let mut signing_times = Vec::new();
    
    for _ in 0..100 {
        let enc_start = Instant::now();
        simulate_encrypt_operation().await;
        encryption_times.push(enc_start.elapsed().as_millis() as u64);
        
        let sign_start = Instant::now();
        simulate_sign_operation().await;
        signing_times.push(sign_start.elapsed().as_millis() as u64);
    }
    
    let benchmark_result = serde_json::json!({
        "benchmark_type": "security_operations",
        "operations_tested": ["encryption", "signing"],
        "sample_size": 100,
        "results": {
            "encryption": {
                "avg_time_ms": encryption_times.iter().sum::<u64>() as f64 / encryption_times.len() as f64,
                "min_time_ms": *encryption_times.iter().min().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?,
                "max_time_ms": *encryption_times.iter().max().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?,
                "operations_per_second": 1000.0 / (encryption_times.iter().sum::<u64>() as f64 / encryption_times.len() as f64),
            },
            "signing": {
                "avg_time_ms": signing_times.iter().sum::<u64>() as f64 / signing_times.len() as f64,
                "min_time_ms": *signing_times.iter().min().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?,
                "max_time_ms": *signing_times.iter().max().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?,
                "operations_per_second": 1000.0 / (signing_times.iter().sum::<u64>() as f64 / signing_times.len() as f64),
            }
        },
        "system_metrics": {
            "memory_usage_mb": 245,
            "cpu_usage_percent": 15.2,
            "concurrent_operations": 10,
        },
        "performance_grade": "A",
        "recommendations": [
            "Software HSM performing optimally for routine operations",
            "Mobile HSM recommended for high-security operations when available",
            "Current configuration suitable for distributed workloads"
        ]
    });
    
    AiResponse {
        success: true,
        data: Some(benchmark_result),
        error: None,
        request_id,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
    }
}

/// Simulate security operation
async fn simulate_security_operation(op: &SecurityOpRequest) -> BearDogResult<serde_json::Value> {
    // Simulate processing delay
    tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
    
    match op.operation.as_str() {
        "encrypt" => Ok(serde_json::json!({
            "encrypted_data": format!("encrypted_{}", op.data),
            "algorithm": op.algorithm.as_ref().unwrap_or(&"AES256".to_string()),
            "key_id": op.key_id,
        })),
        "decrypt" => Ok(serde_json::json!({
            "decrypted_data": format!("decrypted_{}", op.data),
            "key_id": op.key_id,
        })),
        "sign" => Ok(serde_json::json!({
            "signature": format!("signature_of_{}", op.data),
            "algorithm": op.algorithm.as_ref().unwrap_or(&"Ed25519".to_string()),
            "key_id": op.key_id,
        })),
        _ => Err(BearDogError::Configuration {
            message: format!("Unknown operation: {}", op.operation),
        }),
    }
}

/// Simulate encrypt operation
async fn simulate_encrypt_operation() {
    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
}

/// Simulate sign operation
async fn simulate_sign_operation() {
    tokio::time::sleep(tokio::time::Duration::from_millis(3)).await;
} 