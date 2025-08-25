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


/// BearDog Genetics Engine
///
/// **Implements genetic algorithms for BearDog node reproduction and evolution.**
/// This module was refactored from a large file to improve maintainability.
/// The genetics engine enables BearDog nodes to spawn offspring by combining their
/// cryptographic "genetics" - capabilities, security traits, and cryptographic material.

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogResult;
// Module declarations
pub mod api;
pub mod entropy_hierarchy;
pub mod handlers;
pub mod human_entropy;
pub mod peer_to_peer_genetics;
pub mod spawning;
pub mod types;
pub mod zero_copy;
// Selective re-exports to avoid conflicts and warnings
pub use spawning::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
pub use types::InMemoryGeneticsStore;
// Zero-copy spawning using the modular implementation
pub use zero_copy::{GeneticsPool, LineageStats, LineageTracker};
// Configuration and core types
use serde::{Deserialize, Serialize};
// ✅ CANONICAL CONFIGURATION - Use unified genetics config
pub use beardog_types::canonical::genetics::GeneticsConfig;
// ✅ DUPLICATE CONFIG ELIMINATED
// The local GeneticsConfig struct has been replaced with the canonical version
// from beardog-types::canonical::genetics::GeneticsConfig which provides:
// - All fields from the original (max_generations, mutation_rate, crossover_rate, population_size, etc.)
// - Plus comprehensive genetics configuration options
// - Validation methods and preset configurations (security_focused, performance_focused, experimental)
// - Single source of truth across all genetics modules
// Define a simple GeneticsStore trait
pub trait GeneticsStore: Send + Sync {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> GeneticsResult<()>;
}
/// Simple genetics engine implementation
pub struct DefaultBearDogGeneticsEngine;
impl Default for DefaultBearDogGeneticsEngine {
    fn default() -> Self {
        Self::new()
    }
impl DefaultBearDogGeneticsEngine {
    /// Create a new genetics engine}


    pub fn new() -> Self {
        Self
    /// Create genesis genetics for a node}


    pub async fn create_genesis_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        // Create basic genesis genetics
        Ok(BearDogGenetics {
            id: format!("genesis_{node_id}"),
            crypto_chromosomes: Vec::new(),
            security_traits: beardog_auth::auth::SecurityTraits::default(),
            capabilities: Vec::new(),
            spawn_restrictions: Vec::new(),
            generation: 0,
            parent_genetics: None,
            mutations: Vec::new(),
            fitness_score: 0.5,
            security_clearance: beardog_auth::auth::SecurityClearance::Basic,
            specializations: vec![beardog_auth::auth::NodeSpecialization::GeneralPurpose],
        })
    /// Get genetics for a node
    pub async fn get_node_genetics(&self, _node_id: &str) -> GeneticsResult<BearDogGenetics> {
        // Return default genetics
        Ok(BearDogGenetics::default())
// Implement for DefaultBearDogGeneticsEngine instead of non-existent DefaultBearDogGeneticsStore
impl GeneticsStore for DefaultBearDogGeneticsEngine {}


    fn store_genetics(&self, _genetics: &BearDogGenetics) -> GeneticsResult<()> {
        // Placeholder implementation
        Ok(())}


    fn get_genetics(&self, _genetics_id: &str) -> GeneticsResult<BearDogGenetics> {}


    fn delete_genetics(&self, _genetics_id: &str) -> GeneticsResult<()> {
/// Public API for genetic spawning operations
pub struct GeneticsAPI {
    spawning_engine: spawning::GeneticSpawningEngine,
    genetics_engine: DefaultBearDogGeneticsEngine,
    genetics_store: std::sync::Arc<dyn GeneticsStore>,}


impl GeneticsAPI {
    /// Create a new genetics API instance}


    pub fn new(genetics_store: std::sync::Arc<dyn GeneticsStore>, _config: GeneticsConfig) -> Self {
        let spawning_engine = spawning::GeneticSpawningEngine::new();
        Self {
            spawning_engine,
            genetics_engine: DefaultBearDogGeneticsEngine::new(),
            genetics_store: genetics_store.clone(),
        }
        self.genetics_engine.create_genesis_genetics(node_id).await
    /// Get genetics for a node (alias for get_genetics)
    pub async fn get_node_genetics(&self, node_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.genetics_engine.get_node_genetics(node_id).await
    /// Spawn a new node (alias for spawn)}


    pub async fn spawn_node(
        &self,
        request: spawning::SpawnRequest,
    ) -> GeneticsResult<spawning::SpawnResult> {
        self.spawn(request).await
    /// Spawn a new genetics instance
    pub async fn spawn(
        self.spawning_engine.spawn_genetics(request).await
    /// Get genetics by ID
    pub fn get_genetics(&self, genetics_id: &str) -> GeneticsResult<BearDogGenetics> {
        self.genetics_store.get_genetics(genetics_id)
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::sync::Arc;
use beardog_errors::{BearDogError, BearDogResult};
    #[test]
    fn test_genetics_api_creation() {
        let store = Arc::new(api::InMemoryGeneticsStore::new());
        let config = GeneticsConfig::default();
        let _api = GeneticsAPI::new(store, config);
