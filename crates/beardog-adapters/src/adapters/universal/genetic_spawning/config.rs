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


/// Genetic spawning configuration and statistics
///
/// **CANONICAL MIGRATION COMPLETE** ✅
/// This module now uses canonical genetic configurations while preserving unique
/// spawning statistics and business logic functionality.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;
// ============================================================================
// CANONICAL CONFIGURATION IMPORTS - Use unified system
/// **CANONICAL GENETIC ALGORITHM CONFIG** - Use this for genetic operations
// MIGRATION COMPLETE: Use workflow config types from canonical modules
/// **CANONICAL GENETICS CONFIG** - Core genetics configuration
pub use beardog_genetics::genetics::types::GeneticsConfig;
/// **CANONICAL GENETIC HEALING CONFIG** - Tunnel genetic healing
pub use beardog_types::config::tunnel::GeneticHealingConfig;
/// **CANONICAL GENESIS CONFIG** - Genesis spawning configuration
pub use beardog_genetics::genetics::spawning::genesis::GenesisConfig;
// SPECIALIZED SPAWNING STATISTICS - Unique business logic preserved
/// **SPAWNING STATISTICS** - Specialized statistics tracking for genetic spawning
/// This preserves unique spawning statistics functionality that doesn't exist
/// in the canonical configuration system.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawningStatistics {
    /// Total number of spawning operations attempted
    pub total_spawns: u32,
    /// Number of successful spawning operations
    pub successful_spawns: u32,
    /// Number of failed spawning operations
    pub failed_spawns: u32,
    /// Number of currently active spawning operations
    pub active_spawns: u32,
    /// Total number of hybrid nodes created
    pub total_hybrid_nodes: u32,
    /// Average time taken for spawning operations (milliseconds)
    pub average_spawn_time_ms: u64,
    /// Most commonly requested hybrid capabilities
    pub most_common_hybrid_capabilities: Vec<(HybridCapability, u32)>,
    /// Statistics on ecosystem combinations
    pub ecosystem_combination_stats: HashMap<String, u32>,
    /// Last time statistics were updated
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
impl SpawningStatistics {
    /// Create new spawning statistics}


    pub fn new() -> Self {
        Self {
            last_updated: chrono::Utc::now(),
            ..Default::default()
        }
    }
    /// Calculate success rate as percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_spawns == 0 {
            0.0
        } else {
            (self.successful_spawns as f64 / self.total_spawns as f64) * 100.0
    /// Calculate failure rate as percentage
    pub fn failure_rate(&self) -> f64 {
            (self.failed_spawns as f64 / self.total_spawns as f64) * 100.0
    /// Update statistics after a spawning operation}


    pub fn update_spawn_result(&mut self, success: bool, spawn_time_ms: u64) {
        self.total_spawns += 1;
        
        if success {
            self.successful_spawns += 1;
            self.total_hybrid_nodes += 1;
            self.failed_spawns += 1;
        // Update average spawn time
        if self.total_spawns == 1 {
            self.average_spawn_time_ms = spawn_time_ms;
            self.average_spawn_time_ms = (self.average_spawn_time_ms * (self.total_spawns - 1) as u64 + spawn_time_ms) / self.total_spawns as u64;
        self.last_updated = chrono::Utc::now();
    /// Record usage of a hybrid capability
    pub fn record_capability_usage(&mut self, capability: HybridCapability) {
        // Find existing entry or create new one
        if let Some(entry) = self.most_common_hybrid_capabilities.iter_mut().find(|(cap, _)| cap == &capability) {
            entry.1 += 1;
            self.most_common_hybrid_capabilities.push((capability, 1));
        // Sort by usage count (descending)
        self.most_common_hybrid_capabilities.sort_by(|a, b| b.1.cmp(&a.1));
        // Keep only top 10
        self.most_common_hybrid_capabilities.truncate(10);
    /// Record usage of an ecosystem combination}


    pub fn record_ecosystem_combination(&mut self, ecosystem_ids: Vec<String>) {
        let combination_key = ecosystem_ids.join("+");
        *self.ecosystem_combination_stats.entry(combination_key).or_insert(0) += 1;
    /// Get most popular ecosystem combinations
    pub fn get_popular_combinations(&self, limit: usize) -> Vec<(String, u32)> {
        let mut combinations: Vec<_> = self.ecosystem_combination_stats.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        combinations.sort_by(|a, b| b.1.cmp(&a.1));
        combinations.truncate(limit);
        combinations
    /// Check if statistics are healthy (success rate > 70%)}


    pub fn is_healthy(&self) -> bool {
        self.success_rate() > 70.0
// MIGRATION COMPLETE NOTICE
/// **MIGRATION COMPLETE** ✅
/// Genetic algorithm configuration now uses canonical types from:
/// - `beardog-types::config::complete_unified::GeneticWorkflowConfig`
/// - `beardog-genetics::genetics::types::GeneticsConfig`
/// - `beardog-types::config::tunnel::GeneticHealingConfig`
/// - `beardog-genetics::genetics::spawning::genesis::GenesisConfig`
/// **Benefits of Canonical Genetic Configuration:**
/// - Eliminates duplicate genetic algorithm configurations
/// - Single source of truth for genetic parameters
/// - Consistent genetic operations across ecosystem
/// - Unified validation and optimization patterns
/// - Environment-driven genetic configuration
/// **Unique Features Preserved:**
/// - `SpawningStatistics` - Specialized spawning metrics and analytics
/// - Hybrid capability tracking and ecosystem combination analysis
/// - Performance monitoring and health checks for spawning operations
/// **Usage:**
/// ```rust
/// use beardog_types::config::complete_unified::GeneticWorkflowConfig;
/// use super::config::SpawningStatistics;
/// 
/// let genetic_config = GeneticWorkflowConfig::default();
/// let mut stats = SpawningStatistics::new();
/// ```
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spawning_statistics_success_rate() {
        let mut stats = SpawningStatistics::new();
        assert_eq!(stats.success_rate(), 0.0);
        stats.update_spawn_result(true, 1000);
        assert_eq!(stats.success_rate(), 100.0);
        stats.update_spawn_result(false, 500);
        assert_eq!(stats.success_rate(), 50.0);}


    fn test_spawning_statistics_average_time() {
        assert_eq!(stats.average_spawn_time_ms, 1000);
        stats.update_spawn_result(true, 2000);
        assert_eq!(stats.average_spawn_time_ms, 1500);
    fn test_spawning_statistics_health() {
        // Add successful spawns
        for _ in 0..8 {
            stats.update_spawn_result(true, 1000);
        // Add failed spawns
        for _ in 0..2 {
            stats.update_spawn_result(false, 500);
        assert!(stats.is_healthy()); // 80% success rate
} 
