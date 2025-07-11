//! Genetics API endpoints for RESTful genetic spawning operations
//!
//! This module provides HTTP endpoints for BearDog's genetic spawning functionality,
//! including genesis node creation, spawn requests, and lineage tracking.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{info, error};
use uuid::Uuid;

use super::types::{
    GeneticsConfig, InMemoryGeneticsStore,
    BearDogWorkflowType, ResourceLimits
};
use super::GeneticsAPI;
use crate::BearDogCore;

// ========================================================================================
// REQUEST/RESPONSE TYPES
// ========================================================================================

#[derive(Debug, Clone, Deserialize)]
pub struct CreateGenesisRequest {
    pub node_id: String,
}

#[derive(Debug, Serialize)]
pub struct CreateGenesisResponse {
    pub node_id: String,
    pub genetics_id: String,
    pub generation: u32,
    pub capabilities: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpawnNodeRequest {
    pub requesting_parent: String,
    pub co_parents: Option<Vec<String>>,
    pub purpose: crate::auth::SpawnPurpose,
    pub resource_requirements: Option<ResourceLimits>,
    pub workflow_type: Option<BearDogWorkflowType>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize)]
pub struct SpawnNodeResponse {
    pub request_id: String,
    pub approved: bool,
    pub child_node_id: Option<String>,
    pub decision_reason: String,
    pub decided_at: chrono::DateTime<chrono::Utc>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct NodeGeneticsResponse {
    pub node_id: String,
    pub genetics_id: String,
    pub generation: u32,
    pub lineage_depth: u32,
    pub parent_genomes: Vec<String>,
    pub capabilities: u32,
    pub crypto_chromosomes: u32,
    pub spawn_count: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct SpawnStatusResponse {
    pub request_id: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub requesting_parent: String,
    pub co_parents: Vec<String>,
}

// ========================================================================================
// API ENDPOINT IMPLEMENTATIONS
// ========================================================================================

/// Create genesis genetics for a new node
pub async fn create_genesis_node(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<CreateGenesisRequest>,
) -> Result<Json<CreateGenesisResponse>, StatusCode> {
    info!("🧬 Creating genesis node: {}", request.node_id);

    // Initialize genetics API (in production, this would be injected or singleton)
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig::default();
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);

    match genetics_api.create_genesis_node(&request.node_id).await {
        Ok(genetics) => {
            info!("🧬 Generated genesis genetics for node: {}, genetics ID: {}", request.node_id, genetics.id);

            Ok(Json(CreateGenesisResponse {
                node_id: request.node_id,
                genetics_id: genetics.id,
                generation: genetics.generation,
                capabilities: genetics.capabilities.len() as u32,
                created_at: chrono::Utc::now(),
            }))
        }
        Err(e) => {
            error!("❌ Failed to create genesis node {}: {}", request.node_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Submit a spawn request for processing
pub async fn spawn_node(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<SpawnNodeRequest>,
) -> Result<Json<SpawnNodeResponse>, StatusCode> {
    info!("🧬 Processing spawn request from parent: {}", request.requesting_parent);

    // Initialize genetics API
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig::default();
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);

    // Create the internal spawn request
    let spawn_request = super::SpawnRequest {
        request_id: Uuid::new_v4().to_string(),
        requesting_parent: request.requesting_parent.clone(),
        co_parents: request.co_parents.unwrap_or_default(),
        purpose: request.purpose,
        resource_requirements: request.resource_requirements.unwrap_or_else(|| {
            ResourceLimits {
                max_cpu_percent: 50.0,
                max_memory_mb: 2048,
                max_storage_gb: 10,
                max_network_mbps: 100,
                allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
                temporal_windows: vec![],
            }
        }),
        workflow_type: BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec![request.requesting_parent.clone()],
            consensus_threshold: 0.67,
            max_decision_time: chrono::Duration::minutes(5),
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        metadata: request.metadata.unwrap_or_default(),
    };

    let start_time = std::time::Instant::now();
    
    match genetics_api.spawn_node(spawn_request).await {
        Ok(result) => {
            let processing_time = start_time.elapsed().as_millis() as u64;
            info!("✅ Spawn request processed: approved={}, child_node={:?}", 
                  result.approved, result.child_node_id);
            
            Ok(Json(SpawnNodeResponse {
                request_id: result.request_id,
                approved: result.approved,
                child_node_id: result.child_node_id,
                decision_reason: result.decision_reason,
                decided_at: result.decided_at,
                processing_time_ms: processing_time,
            }))
        }
        Err(e) => {
            error!("❌ Failed to process spawn request from {}: {}", request.requesting_parent, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// Get genetics information for a specific node
pub async fn get_node_genetics(
    State(core): State<Arc<BearDogCore>>,
    Path(node_id): Path<String>,
) -> Result<Json<NodeGeneticsResponse>, StatusCode> {
    info!("🧬 Retrieving genetics for node: {}", node_id);

    // Initialize genetics API
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig::default();
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);

    match genetics_api.get_node_genetics(&node_id).await {
        Ok(genetics) => {
            info!("✅ Retrieved genetics for node {}: generation {}", node_id, genetics.generation);
            Ok(Json(NodeGeneticsResponse {
                node_id: node_id.clone(),
                genetics_id: genetics.id,
                generation: genetics.generation,
                lineage_depth: genetics.generation, // Use generation as lineage depth
                parent_genomes: genetics.parent_genetics.unwrap_or_default(),
                capabilities: genetics.capabilities.len() as u32,
                crypto_chromosomes: genetics.crypto_chromosomes.len() as u32,
                spawn_count: 0, // TODO: Track actual spawn count
                created_at: chrono::Utc::now(), // TODO: Track actual creation time in genetics
            }))
        }
        Err(e) => {
            error!("❌ Failed to get genetics for node {}: {}", node_id, e);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Get the status of a spawn request
pub async fn get_spawn_status(
    State(core): State<Arc<BearDogCore>>,
    Path(request_id): Path<String>,
) -> Result<Json<SpawnStatusResponse>, StatusCode> {
    info!("🧬 Getting spawn status for request: {}", request_id);

    // In a real implementation, this would check persistent storage for request status
    // For now, we return a placeholder response
    Ok(Json(SpawnStatusResponse {
        request_id: request_id.clone(),
        status: "completed_or_not_found".to_string(),
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now(),
        requesting_parent: "unknown".to_string(),
        co_parents: vec![],
    }))
}

// ========================================================================================
// HELPER FUNCTIONS FOR GENETICS API
// ========================================================================================

/// Initialize a genetics API instance with proper configuration
pub fn create_genetics_api() -> GeneticsAPI {
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = GeneticsConfig {
        base_mutation_rate: 0.05,
        max_genetic_diversity: 0.8,
        min_security_threshold: 0.7,
        capability_inheritance_weight: 0.8,
        trait_blending_factor: 0.6,
        enable_directed_evolution: true,
    };
    
    GeneticsAPI::new(genetics_store, genetics_config)
}

/// Validate spawn request parameters
pub fn validate_spawn_request(request: &SpawnNodeRequest) -> Result<(), String> {
    // Validate node ID format
    if request.requesting_parent.is_empty() {
        return Err("Requesting parent cannot be empty".to_string());
    }

    // Validate resource requirements if provided
    if let Some(ref resources) = request.resource_requirements {
        if resources.max_cpu_percent > 100.0 {
            return Err("CPU percentage cannot exceed 100%".to_string());
        }
        if resources.max_memory_mb == 0 {
            return Err("Memory requirement must be greater than 0".to_string());
        }
        if resources.max_storage_gb == 0 {
            return Err("Storage requirement must be greater than 0".to_string());
        }
    }

    Ok(())
}

/// Create a demo spawn request for testing purposes
pub fn create_demo_spawn_request(requesting_parent: &str) -> SpawnNodeRequest {
    SpawnNodeRequest {
        requesting_parent: requesting_parent.to_string(),
        co_parents: Some(vec!["demo-node-2".to_string()]),
        purpose: crate::auth::SpawnPurpose::EmergencyResponse,
        resource_requirements: Some(ResourceLimits {
            max_cpu_percent: 25.0,
            max_memory_mb: 1024,
            max_storage_gb: 5,
            max_network_mbps: 50,
            allowed_jurisdictions: vec!["US".to_string(), "EU".to_string()],
            temporal_windows: vec![],
        }),
        workflow_type: Some(BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec!["demo-node-1".to_string(), "demo-node-2".to_string()],
            consensus_threshold: 0.6,
            max_decision_time: chrono::Duration::minutes(2),
        }),
        metadata: Some({
            let mut metadata = HashMap::new();
            metadata.insert("demo".to_string(), "true".to_string());
            metadata.insert("purpose".to_string(), "testing".to_string());
            metadata
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_spawn_request() {
        let valid_request = SpawnNodeRequest {
            requesting_parent: "test-node".to_string(),
            co_parents: None,
            purpose: crate::auth::SpawnPurpose::EmergencyResponse,
            resource_requirements: Some(ResourceLimits {
                max_cpu_percent: 50.0,
                max_memory_mb: 1024,
                max_storage_gb: 10,
                max_network_mbps: 100,
                allowed_jurisdictions: vec!["US".to_string()],
                temporal_windows: vec![],
            }),
            workflow_type: None,
            metadata: None,
        };

        assert!(validate_spawn_request(&valid_request).is_ok());

        // Test invalid CPU percentage
        let mut invalid_request = valid_request.clone();
        invalid_request.resource_requirements.as_mut().unwrap().max_cpu_percent = 150.0;
        assert!(validate_spawn_request(&invalid_request).is_err());

        // Test empty requesting parent
        let mut invalid_request = valid_request.clone();
        invalid_request.requesting_parent = "".to_string();
        assert!(validate_spawn_request(&invalid_request).is_err());
    }

    #[test]
    fn test_create_demo_spawn_request() {
        let demo_request = create_demo_spawn_request("demo-parent");
        assert_eq!(demo_request.requesting_parent, "demo-parent");
        assert!(demo_request.co_parents.is_some());
        assert!(demo_request.resource_requirements.is_some());
        assert!(demo_request.metadata.is_some());
    }
} 