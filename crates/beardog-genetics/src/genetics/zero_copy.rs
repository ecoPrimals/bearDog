// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct GeneticsPool {
    pools: HashMap<String, Vec<u8>>,
    metadata: HashMap<String, PoolMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetadata {
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// The last accessed value
    pub last_accessed: DateTime<Utc>,
    /// Number of access
    pub access_count: u64,
    /// Number of size_bytes
    pub size_bytes: usize,
}

impl Default for PoolMetadata {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            last_accessed: now,
            access_count: 0,
            size_bytes: 0,
        }
    }
}

impl GeneticsPool {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            pools: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(16),
        }
    }

    /// Allocate a new pool
    pub fn allocate(&mut self, pool_id: &str, size: usize) -> Result<(), BearDogError> {
        if self.pools.contains_key(pool_id) {
            return Err(BearDogError::system(format!(
                "Pool already exists: {}",
                pool_id
            )));
        }

        let buffer = vec![0u8; size];
        let metadata = PoolMetadata {
            size_bytes: size,
            ..Default::default()
        };

        self.pools.insert(pool_id.to_string(), buffer);
        self.metadata.insert(pool_id.to_string(), metadata);

        Ok(())
    }

    /// Get Pool operation.
    /// Gets pool
    /// Gets pool
    pub fn get_pool(&mut self, pool_id: &str) -> Option<&mut Vec<u8>> {
        if let Some(metadata) = self.metadata.get_mut(pool_id) {
            metadata.last_accessed = Utc::now();
            metadata.access_count += 1;
        }
        self.pools.get_mut(pool_id)
    }

    /// Deallocate operation.
    pub fn deallocate(&mut self, pool_id: &str) -> bool {
        let removed_pool = self.pools.remove(pool_id);
        self.metadata.remove(pool_id);
        removed_pool.is_some()
    }

    /// Get pool count
    pub fn pool_count(&self) -> usize {
        self.pools.len()
    }

    /// Get total allocated bytes
    pub fn total_allocated_bytes(&self) -> usize {
        self.metadata.values().map(|m| m.size_bytes).sum()
    }
}

#[derive(Debug, Clone)]
pub struct LineageTracker {
    lineages: HashMap<String, LineageInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageInfo {
    pub genetics_id: String,
    pub parent_ids: Vec<String>,
    pub child_ids: Vec<String>,
    /// Number of generation
    pub generation: u32,
    /// The created at value
    pub created_at: DateTime<Utc>,
}

impl Default for LineageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl LineageTracker {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            lineages: HashMap::with_capacity(100),
        }
    }

    /// Track genetics with parents
    pub fn track_genetics(&mut self, genetics_id: &str, parent_ids: Vec<&str>) {
        let generation = if parent_ids.is_empty() {
            0
        } else {
            parent_ids
                .iter()
                .filter_map(|id| self.lineages.get(&**id))
                .map(|info| info.generation)
                .max()
                .unwrap_or(0)
                + 1
        };

        let lineage_info = LineageInfo {
            genetics_id: genetics_id.to_string(),
            parent_ids: parent_ids
                .iter()
                .map(std::string::ToString::to_string)
                .collect(),
            child_ids: Vec::new(),
            generation,
            created_at: Utc::now(),
        };

        for parent_id in &parent_ids {
            if let Some(parent_info) = self.lineages.get_mut(&**parent_id) {
                parent_info.child_ids.push(genetics_id.to_string());
            }
        }

        self.lineages.insert(genetics_id.to_string(), lineage_info);
    }

    /// Get Lineage operation.
    /// Gets lineage
    /// Gets lineage
    pub fn get_lineage(&self, genetics_id: &str) -> Option<&LineageInfo> {
        self.lineages.get(genetics_id)
    }

    /// Get Descendants operation.
    /// Gets descendants
    /// Gets descendants
    pub fn get_descendants(&self, genetics_id: &str) -> Vec<String> {
        let mut descendants = Vec::new();
        if let Some(info) = self.lineages.get(genetics_id) {
            for child_id in &info.child_ids {
                descendants.push(child_id.clone());
                descendants.extend(self.get_descendants(child_id));
            }
        }
        descendants
    }

    /// Get Ancestors operation.
    /// Gets ancestors
    /// Gets ancestors
    pub fn get_ancestors(&self, genetics_id: &str) -> Vec<String> {
        let mut ancestors = Vec::new();
        if let Some(info) = self.lineages.get(genetics_id) {
            for parent_id in &info.parent_ids {
                ancestors.push(parent_id.clone());
                ancestors.extend(self.get_ancestors(parent_id));
            }
        }
        ancestors
    }

    /// Get lineage count
    pub fn lineage_count(&self) -> usize {
        self.lineages.len()
    }
}

/// Statistics about lineage tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageStats {
    /// Number of total_lineages
    pub total_lineages: usize,
    /// Number of max_generation
    pub max_generation: u32,
    /// The average children per genetics value
    pub average_children_per_genetics: f64,
    /// Number of orphan
    pub orphan_count: usize, // Genetics with no parents
    /// Number of leaf
    pub leaf_count: usize,   // Genetics with no children
}

impl LineageStats {
    /// Calculate operation.
    pub fn calculate(tracker: &LineageTracker) -> Self {
        let lineages = &tracker.lineages;
        let total_lineages = lineages.len();

        let max_generation = lineages
            .values()
            .map(|info| info.generation)
            .max()
            .unwrap_or(0);

        let total_children: usize = lineages.values().map(|info| info.child_ids.len()).sum();

        let average_children_per_genetics = if total_lineages > 0 {
            total_children as f64 / total_lineages as f64
        } else {
            0.0
        };

        let orphan_count = lineages
            .values()
            .filter(|info| info.parent_ids.is_empty())
            .count();

        let leaf_count = lineages
            .values()
            .filter(|info| info.child_ids.is_empty())
            .count();

        Self {
            total_lineages,
            max_generation,
            average_children_per_genetics,
            orphan_count,
            leaf_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genetics_pool() {
        let mut pool = GeneticsPool::new();

        assert!(pool.allocate("test-pool", 1024).is_ok());
        assert_eq!(pool.pool_count(), 1);
        assert_eq!(pool.total_allocated_bytes(), 1024);

        assert!(pool.allocate("test-pool", 512).is_err());

        let buffer = pool.get_pool("test-pool");
        assert!(buffer.is_some());
        assert_eq!(buffer?.len(), 1024);

        assert!(pool.deallocate("test-pool"));
        assert_eq!(pool.pool_count(), 0);
        assert_eq!(pool.total_allocated_bytes(), 0);
    }

    #[test]
    fn test_lineage_tracker() {
        let mut tracker = LineageTracker::new();

        tracker.track_genetics("genesis", vec![]);

        tracker.track_genetics("child1", vec!["genesis"]);
        tracker.track_genetics("child2", vec!["genesis"]);

        tracker.track_genetics("grandchild", vec!["child1", "child2"]);

        assert_eq!(tracker.lineage_count(), 4);

        assert_eq!(tracker.get_lineage("genesis")?.generation, 0);
        assert_eq!(tracker.get_lineage("child1")?.generation, 1);
        assert_eq!(tracker.get_lineage("grandchild")?.generation, 2);

        let descendants = tracker.get_descendants("genesis");
        assert!(descendants.contains(&"child1".to_string()));
        assert!(descendants.contains(&"child2".to_string()));
        assert!(descendants.contains(&"grandchild".to_string()));

        let ancestors = tracker.get_ancestors("grandchild");
        assert!(ancestors.contains(&"child1".to_string()));
        assert!(ancestors.contains(&"child2".to_string()));
        assert!(ancestors.contains(&"genesis".to_string()));
    }

    #[test]
    fn test_lineage_stats() {
        let mut tracker = LineageTracker::new();

        tracker.track_genetics("genesis", vec![]);
        tracker.track_genetics("child1", vec!["genesis"]);
        tracker.track_genetics("child2", vec!["genesis"]);

        let stats = LineageStats::calculate(&tracker);

        assert_eq!(stats.total_lineages, 3);
        assert_eq!(stats.max_generation, 1);
        assert_eq!(stats.orphan_count, 1); // Only genesis has no parents
        assert_eq!(stats.leaf_count, 2); // child1 and child2 have no children
    }
}
