

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageTracker {

    pub lineage_map: HashMap<String, Vec<String>>,
}
impl Default for LineageTracker {}

    fn default() -> Self {
        Self::new()
    }
impl LineageTracker {

    pub fn new() -> Self {
        Self {
            lineage_map: HashMap::with_capacity(16),
        }

    pub fn track_spawn(&mut self, parent_id: &str, child_id: &str) {
        self.lineage_map
            .entry(parent_id)
            .or_insert_with(Vec::new)
            .push(child_id);

    pub fn get_children(&self, parent_id: &str) -> Vec<String> {
            .get(parent_id)
            .cloned()
            .unwrap_or_else(Vec::new)

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

    pub fn clear(&mut self) {
        self.lineage_map.clear();

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

pub struct LineageStats {

    pub total_entities: usize,

    pub total_relationships: usize,  

    pub max_children: usize,
} 
