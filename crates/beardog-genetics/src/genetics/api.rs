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


/// Genetics API endpoints for RESTful genetic spawning operations
///
/// This module provides HTTP endpoints for BearDog's genetic spawning functionality,
/// including genesis node creation, spawn requests, and lineage tracking.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use beardog_auth::auth::{BearDogGenetics, BearDogWorkflowType, ResourceLimits, SpawnPurpose};
use beardog_errors::BearDogResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{error, info};
use super::{GeneticsAPI, GeneticsStore};
// ✅ CANONICAL CONFIGURATION - Use unified genetics config
pub use beardog_types::canonical::genetics::GeneticsConfig;
// ✅ DUPLICATE CONFIG ELIMINATED
// The local GeneticsConfig struct has been replaced with the canonical version
// from beardog-types::canonical::genetics::GeneticsConfig which provides:
// - All fields from the original (max_population_size, mutation_rate, crossover_rate, fitness_threshold)
// - Plus comprehensive genetics configuration options
// - Validation methods and preset configurations
// - Single source of truth across all genetics modules
#[derive(Debug)]
pub struct InMemoryGeneticsStore {
    pub storage: std::collections::HashMap<String, beardog_auth::auth::BearDogGenetics>,
}
impl InMemoryGeneticsStore {
    /// Create a new in-memory genetics store}


    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
impl Default for InMemoryGeneticsStore {}


    fn default() -> Self {
        Self::new()
impl GeneticsStore for InMemoryGeneticsStore {}


    fn store_genetics(&self, _genetics: &BearDogGenetics) -> GeneticsResult<()> {
        // In a real implementation, this would actually store the genetics
        Ok(())}


    fn get_genetics(&self, _genetics_id: &str) -> GeneticsResult<BearDogGenetics> {
        // In a real implementation, this would retrieve from storage
        Ok(BearDogGenetics::default())}


    fn delete_genetics(&self, _genetics_id: &str) -> GeneticsResult<()> {
        // In a real implementation, this would delete the genetics
/// Create a default genetics configuration for examples}


pub fn create_default_genetics_config() -> GeneticsConfig {
    GeneticsConfig {
        max_population_size: 1000,
        mutation_rate: 0.05,
        crossover_rate: 0.6,
        fitness_threshold: 0.8,
// ========================================================================================
// REQUEST/RESPONSE TYPES
/// Request to create a genesis node with initial genetics}


#[derive(Debug, Clone, Deserialize)]
pub struct CreateGenesisRequest {
    /// The unique identifier for the new node
    pub node_id: String,
/// Response containing the newly created genesis node information
#[derive(Debug, Serialize)]
pub struct CreateGenesisResponse {
    /// The unique identifier for the node
    /// The unique identifier for the genetics configuration
    pub genetics_id: String,
    /// The generation number (0 for genesis nodes)
    pub generation: u32,
    /// The number of capabilities inherited by this node
    pub capabilities: u32,
    /// The timestamp when the genesis node was created
    pub created_at: chrono::DateTime<chrono::Utc>,
/// Request to spawn a new child node from existing parent(s)
pub struct SpawnNodeRequest {
    /// The primary parent node requesting the spawn
    pub requesting_parent: String,
    /// Optional additional parent nodes for multi-parent spawning
    pub co_parents: Option<Vec<String>>,
    /// The purpose/reason for spawning this new node
    pub purpose: SpawnPurpose,
    /// Optional resource limits for the child node
    pub resource_requirements: Option<ResourceLimits>,
    /// Optional workflow type for the spawning process
    pub workflow_type: Option<BearDogWorkflowType>,
    /// Optional additional metadata for the spawn request
    pub metadata: Option<HashMap<String, String>>,
/// Response containing the result of a spawn request
pub struct SpawnNodeResponse {
    /// The unique identifier for the spawn request
    pub request_id: String,
    /// Whether the spawn request was approved
    pub approved: bool,
    /// The ID of the newly created child node (if approved)
    pub child_node_id: Option<String>,
    /// The reason for the approval/rejection decision
    pub decision_reason: String,
    /// The timestamp when the decision was made
    pub decided_at: chrono::DateTime<chrono::Utc>,
    /// The time taken to process the request in milliseconds
    pub processing_time_ms: u64,
/// Response containing genetics information for a node
pub struct NodeGeneticsResponse {
    /// The generation number of this node
    /// The depth of the lineage tree from genesis
    pub lineage_depth: u32,
    /// The list of parent genome identifiers
    pub parent_genomes: Vec<String>,
    /// The number of cryptographic chromosomes
    pub crypto_chromosomes: u32,
    /// The number of nodes spawned by this node
    pub spawn_count: u32,
    /// The timestamp when the node was created
/// Response containing the status of a spawn request
pub struct SpawnStatusResponse {
    /// The current status of the spawn request
    pub status: String,
    /// The timestamp when the request was created
    /// The timestamp when the request expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// The primary parent node that made the request
    /// The list of co-parent nodes involved in the request
    pub co_parents: Vec<String>,
// API ENDPOINT IMPLEMENTATIONS
/// Create genesis genetics for a new node
pub async fn create_genesis_node(
    State(_core): State<Arc<()>>,
    Json(request): Json<CreateGenesisRequest>,
) -> Result<Json<CreateGenesisResponse>, StatusCode> {
    info!("🧬 Creating genesis node: {}", request.node_id);
    // Initialize genetics API (in production, this would be injected or singleton)
    let genetics_store = Arc::new(InMemoryGeneticsStore::new());
    let genetics_config = super::GeneticsConfig::default();
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);
    match genetics_api.create_genesis_genetics(&request.node_id).await {
        Ok(genetics) => {
            info!(
                "🧬 Generated genesis genetics for node: {}, genetics ID: {}",
                request.node_id, genetics.id
            );
            Ok(Json(CreateGenesisResponse {
                node_id: request.node_id,
                genetics_id: genetics.id,
                generation: genetics.generation,
                capabilities: genetics.capabilities.len() as u32,
                created_at: chrono::Utc::now(),
            }))
        Err(e) => {
            error!(
                "❌ Failed to create genesis node {}: {}",
                request.node_id, e
            Err(StatusCode::INTERNAL_SERVER_ERROR)
/// Submit a spawn request for processing
pub async fn spawn_node(
    Json(request): Json<SpawnNodeRequest>,
) -> Result<Json<SpawnNodeResponse>, StatusCode> {
    info!(
        "🧬 Processing spawn request from parent: {}",
        request.requesting_parent
    );
    // Initialize genetics API
    // Create the internal spawn request
    let spawn_request = crate::genetics::spawning::SpawnRequest {
        purpose: beardog_auth::auth::SpawnPurpose::LoadBalancing,
        required_capabilities: vec![],
        resource_requirements: beardog_auth::auth::ResourceLimits::default(),
        security_clearance: beardog_auth::auth::SecurityClearance::Basic,
        parent_genetics: vec![],
        metadata: std::collections::HashMap::new(),
    };
    let start_time = std::time::Instant::now();
    match genetics_api.spawn_node(spawn_request).await {
        Ok(result) => {
            let processing_time = start_time.elapsed().as_millis() as u64;
                "✅ Spawn request processed: approved={}, child_node={:?}",
                result.success, result.genetics.id
            Ok(Json(SpawnNodeResponse {
                request_id: "genetics-api".to_string(),
                approved: result.success,
                child_node_id: Some(result.genetics.id.clone()),
                decision_reason: result.messages.join("; "),
                decided_at: chrono::Utc::now(),
                processing_time_ms: processing_time,
                "❌ Failed to process spawn request from {}: {}",
                request.requesting_parent, e
/// Get genetics information for a specific node
pub async fn get_node_genetics(
    Path(node_id): Path<String>,
) -> Result<Json<NodeGeneticsResponse>, StatusCode> {
    info!("🧬 Retrieving genetics for node: {}", node_id);
    match genetics_api.get_node_genetics(&node_id).await {
                "✅ Retrieved genetics for node {}: generation {}",
                node_id, genetics.generation
            Ok(Json(NodeGeneticsResponse {
                node_id: node_id.clone(),
                lineage_depth: genetics.generation, // Use generation as lineage depth
                parent_genomes: genetics.parent_genetics.unwrap_or_default(),
                crypto_chromosomes: genetics.crypto_chromosomes.len() as u32,
                spawn_count: 0, // Placeholder for future spawn tracking
                created_at: chrono::Utc::now(), // Placeholder for future genetics timestamp
            error!("❌ Failed to get genetics for node {}: {}", node_id, e);
            Err(StatusCode::NOT_FOUND)
/// Get the status of a spawn request
pub async fn get_spawn_status(
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
// HELPER FUNCTIONS FOR GENETICS API
/// Initialize a genetics API instance with proper configuration
pub fn create_genetics_api() -> GeneticsAPI {
    GeneticsAPI::new(genetics_store, genetics_config)
/// Validate spawn request parameters}


pub fn validate_spawn_request(request: &SpawnNodeRequest) -> Result<(), String> {
    // Validate node ID format
    if request.requesting_parent.is_empty() {
        return Err("Requesting parent cannot be empty".to_string());
    // Validate resource requirements if provided
    if let Some(ref resources) = request.resource_requirements {
        if resources.max_cpu_percent > 100 {
            return Err("CPU percentage cannot exceed 100%".to_string());
        if resources.max_memory_mb == 0 {
            return Err("Memory requirement must be greater than 0".to_string());
        if resources.max_disk_mb == 0 {
            return Err("Storage requirement must be greater than 0".to_string());
    Ok(())
/// Create a demo spawn request for testing purposes
pub fn create_demo_spawn_request(requesting_parent: &str) -> SpawnNodeRequest {
    SpawnNodeRequest {
        requesting_parent: requesting_parent.to_string(),
        co_parents: Some(vec!["demo-node-2".to_string()]),
        purpose: SpawnPurpose::EmergencyResponse,
        resource_requirements: Some(ResourceLimits {
            max_cpu_percent: 25,
            max_memory_mb: 1024,
            max_disk_mb: 5120,
            max_network_mbps: 50,
            max_concurrent_connections: 500,
        }),
        workflow_type: Some(BearDogWorkflowType::GeneticSpawning {
            parent_genetics: vec!["demo-node-1".to_string(), "demo-node-2".to_string()],
            spawn_purpose: SpawnPurpose::EmergencyResponse,
            target_capabilities: vec![],
        metadata: Some({
            let mut metadata = HashMap::new();
            metadata.insert("demo".to_string(), "true".to_string());
            metadata.insert("purpose".to_string(), "testing".to_string());
            metadata
#[cfg(test)]
mod tests {
    use super::*;
use beardog_errors::{BearDogError, BearDogResult};
    #[test]
    fn test_validate_spawn_request() -> beardog_errors::BearDogResult<()> {
        let valid_request = SpawnNodeRequest {
            requesting_parent: "test-node".to_string(),
            co_parents: None,
            purpose: SpawnPurpose::EmergencyResponse,
            resource_requirements: Some(ResourceLimits {
                max_cpu_percent: 50,
                max_memory_mb: 1024,
                max_disk_mb: 10240,
                max_network_mbps: 100,
                max_concurrent_connections: 1000,
            }),
            workflow_type: None,
            metadata: None,
        };
        assert!(validate_spawn_request(&valid_request).is_ok());
        // Test invalid CPU percentage
        let mut invalid_request = valid_request.clone();
        invalid_request
            .resource_requirements
            .as_mut()
            .ok_or_else(|| {
                tracing::error!("Operation failed: resource_requirements is None");
                beardog_errors::GeneticsError::InternalError { reason: 
                    "Operation failed: resource_requirements is None".to_string(), context: create_genetics_context(), metadata: GeneticsMetadata::default(), improvement: None }
            })?
            .max_cpu_percent = 150;
        assert!(validate_spawn_request(&invalid_request).is_err());
        // Test empty requesting parent
        invalid_request.requesting_parent = "".to_string();
    fn test_create_demo_spawn_request() {
        let demo_request = create_demo_spawn_request("demo-parent");
        assert_eq!(demo_request.requesting_parent, "demo-parent");
        assert!(demo_request.co_parents.is_some());
        assert!(demo_request.resource_requirements.is_some());
        assert!(demo_request.metadata.is_some());
