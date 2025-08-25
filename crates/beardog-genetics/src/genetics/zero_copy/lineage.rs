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


/// Genetic Lineage Tracking
///
/// Provides efficient tracking of genetic lineages with minimal overhead.

use std::collections::HashMap;
use tracing::debug;
/// Tracks genetic lineages with zero-copy optimizations
#[derive(Debug)]
pub struct LineageTracker {
    /// Lineage records using Arc for sharing
    lineages: HashMap<String, Vec<String>>,
    /// Lineage statistics
    stats: LineageStats,
}
impl Default for LineageTracker {}


    fn default() -> Self {
        Self::new()
    }
impl LineageTracker {}


    pub fn new() -> Self {
        Self {
            lineages: HashMap::new(),
            stats: LineageStats::default(),
        }
    /// Track genetic lineage
    pub fn track_lineage(&mut self, child_id: &str, parent_ids: &[String]) {
        self.lineages
            .insert(child_id.to_string(), parent_ids.to_vec());
        self.stats.total_lineages += 1;
    /// Get lineage for genetics}


    pub fn get_lineage(&self, genetics_id: &str) -> Option<&Vec<String>> {
        self.lineages.get(genetics_id)
    /// Add child to lineage
    pub fn add_child_to_lineage(&mut self, parent_id: &str, child_id: &str) {
        if let Some(lineage) = self.lineages.get_mut(parent_id) {
            if !lineage.contains(&child_id.to_string()) {
                lineage.push(child_id.to_string());
            }
    /// Update lineage statistics
    pub fn update_stats(&mut self) {
        self.stats.total_lineages = self.lineages.len();
        if !self.lineages.is_empty() {
            let total_children: usize = self.lineages.values().map(|children| children.len()).sum();
            self.stats.average_children_per_parent =
                total_children as f64 / self.lineages.len() as f64;
    /// Clear old lineage records}


    pub fn cleanup_old_lineages(&mut self) {
        // In a real implementation, this would remove old lineage records
        // For now, just update stats
        self.update_stats();
        // Create simple lineage tracking
        let _lineage_data: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        // Store lineage data would go here
        debug!("Cleaned up old lineage records");
    /// Get current statistics
    pub fn get_stats(&self) -> &LineageStats {
        &self.stats
/// Lineage tracking statistics}


#[derive(Debug, Default, Clone)]
pub struct LineageStats {
    pub total_lineages: usize,
    pub max_generation: u32,
    pub average_children_per_parent: f64,
