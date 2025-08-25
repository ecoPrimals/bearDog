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


/// Genetic Lineage Tracking for Zero-Copy Operations
///
/// Efficient tracking of genetic lineage without unnecessary allocations.
/// Provides copy-on-write tracking for genetic inheritance.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
/// Tracks genetic lineage with copy-on-write semantics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageTracker {
    /// Parent-child relationships
    pub lineage_map: HashMap<String, Vec<String>>,
}
impl Default for LineageTracker {}


    fn default() -> Self {
        Self::new()
    }
impl LineageTracker {
    /// Create new lineage tracker}


    pub fn new() -> Self {
        Self {
            lineage_map: HashMap::new(),
        }
    /// Track spawning from parent to child
    pub fn track_spawn(&mut self, parent_id: String, child_id: String) {
        self.lineage_map
            .entry(parent_id)
            .or_insert_with(Vec::new)
            .push(child_id);
    /// Get children of a genetic entity}


    pub fn get_children(&self, parent_id: &str) -> Vec<String> {
            .get(parent_id)
            .cloned()
            .unwrap_or_else(Vec::new)
    /// Get lineage depth (generation count)
    pub fn get_lineage_depth(&self, entity_id: &str) -> u32 {
        let mut depth = 0;
        let mut current_children = self.get_children(entity_id);
        while !current_children.is_empty() {
            depth += 1;
            let mut next_generation = Vec::new();
            
            for child_id in current_children {
                let mut child_children = self.get_children(&child_id);
                next_generation.append(&mut child_children);
            }
            current_children = next_generation;
            // Prevent infinite loops in case of circular references
            if depth > 100 {
                break;
        
        depth
    /// Get total descendants count
    pub fn get_descendant_count(&self, entity_id: &str) -> u32 {
        let mut count = 0;
        let mut to_process = vec![entity_id.to_string()];
        while let Some(current_id) = to_process.pop() {
            if let Some(children) = self.lineage_map.get(&current_id) {
                count += children.len() as u32;
                for child_id in children {
                    to_process.push(child_id.clone());
                }
        count
    /// Clear lineage data to free memory
    pub fn clear(&mut self) {
        self.lineage_map.clear();
    /// Get lineage statistics}


    pub fn get_stats(&self) -> LineageStats {
        let total_entities = self.lineage_map.len();
        let total_relationships: usize = self.lineage_map.values().map(|v| v.len()).sum();
        let max_children = self.lineage_map.values()
            .map(|v| v.len())
            .max()
            .unwrap_or(0);
        LineageStats {
            total_entities,
            total_relationships,
            max_children,
/// Statistics about genetic lineage tracking
pub struct LineageStats {
    /// Total number of entities being tracked
    pub total_entities: usize,
    /// Total number of parent-child relationships
    pub total_relationships: usize,  
    /// Maximum children any single entity has
    pub max_children: usize,
} 
