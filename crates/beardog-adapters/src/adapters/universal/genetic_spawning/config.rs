

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::genetics::HybridCapability;

pub use beardog_genetics::genetics::types::GeneticsConfig;

pub use beardog_types::config::tunnel::GeneticHealingConfig;

pub use beardog_genetics::genetics::spawning::genesis::GenesisConfig;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawningStatistics {

    pub total_spawns: u32,

    pub successful_spawns: u32,

    pub failed_spawns: u32,

    pub active_spawns: u32,

    pub total_hybrid_nodes: u32,

    pub average_spawn_time_ms: u64,

    pub most_common_hybrid_capabilities: Vec<(HybridCapability, u32)>,

    pub ecosystem_combination_stats: HashMap<String, u32>,

    pub last_updated: chrono::DateTime<chrono::Utc>,
}
impl SpawningStatistics {

    pub fn new() -> Self {
        Self {
            last_updated: chrono::Utc::now(),
            ..Default::default()
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_spawns == 0 {
            0.0
        } else {
            (self.successful_spawns as f64 / self.total_spawns as f64) * 100.0

    pub fn failure_rate(&self) -> f64 {
            (self.failed_spawns as f64 / self.total_spawns as f64) * 100.0

    pub fn update_spawn_result(&mut self, success: bool, spawn_time_ms: u64) {
        self.total_spawns += 1;
        
        if success {
            self.successful_spawns += 1;
            self.total_hybrid_nodes += 1;
            self.failed_spawns += 1;

        if self.total_spawns == 1 {
            self.average_spawn_time_ms = spawn_time_ms;
            self.average_spawn_time_ms = (self.average_spawn_time_ms * (self.total_spawns - 1) as u64 + spawn_time_ms) / self.total_spawns as u64;
        self.last_updated = chrono::Utc::now();

    pub fn record_capability_usage(&mut self, capability: HybridCapability) {

        if let Some(entry) = self.most_common_hybrid_capabilities.iter_mut().find(|(cap, _)| cap == &capability) {
            entry.1 += 1;
            self.most_common_hybrid_capabilities.push((capability, 1));

        self.most_common_hybrid_capabilities.sort_by(|a, b| b.1.cmp(&a.1));

        self.most_common_hybrid_capabilities.truncate(10);

    pub fn record_ecosystem_combination(&mut self, ecosystem_ids: Vec<&str>) {
        let combination_key = ecosystem_ids.join("+");
        *self.ecosystem_combination_stats.entry(combination_key).or_insert(0) += 1;

    pub fn get_popular_combinations(&self, limit: usize) -> Vec<(String, u32)> {
        let mut combinations: Vec<_> = self.ecosystem_combination_stats.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        combinations.sort_by(|a, b| b.1.cmp(&a.1));
        combinations.truncate(limit);
        combinations

    pub fn is_healthy(&self) -> bool {
        self.success_rate() > 70.0

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

        for _ in 0..8 {
            stats.update_spawn_result(true, 1000);

        for _ in 0..2 {
            stats.update_spawn_result(false, 500);
        assert!(stats.is_healthy()); // 80% success rate
} 
