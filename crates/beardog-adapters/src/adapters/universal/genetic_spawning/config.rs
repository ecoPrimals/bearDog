

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;

pub use beardog_genetics::genetics::types::GeneticsConfig;

pub type GeneticHealingConfig = beardog_types::canonical::configuration::consolidated::TunnelConfig;

pub use beardog_genetics::genetics::spawning::genesis::GenesisConfig;

#[derive(Debug, Clone)]
    /// Number of successful_spawns
    pub successful_spawns: u32,

    /// Number of failed_spawns
    pub failed_spawns: u32,

    /// Number of active_spawns
    pub active_spawns: u32,


    pub total_hybrid_nodes: u32,


    pub average_spawn_time_ms: u64,


    pub most_common_hybrid_capabilities: Vec<(HybridCapability, u32)>,

    /// Mapping of ecosystem combination stats
    pub ecosystem_combination_stats: HashMap<String, u32>,

    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}
impl SpawningStatistics {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            last_updated: chrono::Utc::now(),
            ..Default::default(bool, spawn_time_ms: u64) {
        self.total_spawns += 1;
        
        if success {
            self.successful_spawns += 1;
            self.total_hybrid_nodes += 1;
            self.failed_spawns += 1;

        if self.total_spawns == 1 {
            self.average_spawn_time_ms = spawn_time_ms;
            self.average_spawn_time_ms = (self.average_spawn_time_ms * (self.total_spawns - 1) as u64 + spawn_time_ms) / self.total_spawns as u64;
        self.last_updated = chrono::Utc::now();

/// Record Capability Usage operation.
    pub fn record_capability_usage(&mut self, capability: HybridCapability) {

        if let Some(entry) = self.most_common_hybrid_capabilities.iter_mut().find(|(cap, _)| cap == &capability) {
            entry.1 += 1;
            self.most_common_hybrid_capabilities.push((capability, 1));

        self.most_common_hybrid_capabilities.sort_by(|a, b| b.1.cmp(&a.1));

        self.most_common_hybrid_capabilities.truncate(10);

/// Record Ecosystem Combination operation.
    pub fn record_ecosystem_combination(&mut self, ecosystem_ids: Vec<&str>) {
        let combination_key = ecosystem_ids.join("+");
        *self.ecosystem_combination_stats.entry(combination_key).or_insert(0) += 1;

/// Get Popular Combinations operation.
    /// Gets popular_combinations
    /// Gets popular_combinations
    pub fn get_popular_combinations(&self, limit: usize) -> Vec<(String, u32)> {
        let mut combinations: Vec<_> = self.ecosystem_combination_stats.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        combinations.sort_by(|a, b| b.1.cmp(&a.1));
        combinations.truncate(limit);
        combinations

/// Is Healthy operation.
    /// Checks if healthy
    /// Checks if healthy
    pub fn is_healthy(&self) -> bool {
        self.success_rate() > 70.0

#[cfg(test)]
mod tests {
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
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

        for _ in 0..8 {
            stats.update_spawn_result(true, 1000);

        for _ in 0..2 {
            stats.update_spawn_result(false, 500);
        assert!(stats.is_healthy()); // 80% success rate
} 
