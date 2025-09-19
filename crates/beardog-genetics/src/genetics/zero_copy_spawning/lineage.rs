

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(HashMap<String, Vec<String>>,
}
impl Default for LineageTracker {}

    fn default() -> Self {
        Self::new()
    }
impl LineageTracker {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            lineage_map: HashMap::with_capacity(&str, child_id: &str) {
        self.lineage_map
            .entry(parent_id)
            .or_insert_with(Vec::new)
            .push(child_id);

/// Get Children operation.
    /// Gets children
    /// Gets children
    pub fn get_children(&self, parent_id: &str) -> Vec<String> {
            .get(parent_id)
            .cloned()
            .unwrap_or_else(Vec::new)

/// Get Lineage Depth operation.
    /// Gets lineage_depth
    /// Gets lineage_depth
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

            if depth > 100 {
                break;
        
        depth

/// Get Descendant Count operation.
    /// Gets descendant_count
    /// Gets descendant_count
    pub fn get_descendant_count(&self, entity_id: &str) -> u32 {
        let mut count = 0;
        let mut to_process = vec![entity_id];
        while let Some(current_id) = to_process.pop() {
            if let Some(children) = self.lineage_map.get(&current_id) {
                count += children.len() as u32;
                for child_id in children {
                    to_process.push(&child_id);
                }
        count

/// Clear operation.
    pub fn clear(&mut self) {
        self.lineage_map.clear();

/// Get Stats operation.
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> LineageStats {
        let total_entities = self.lineage_map.len();
        let total_relationships: usize = self.lineage_map.values(usize,

    /// Number of total_relationships
    pub total_relationships: usize,  

    /// Number of max_children
    pub max_children: usize,
} 
