//! Zero-Copy Genetics Module
//!
//! This module provides zero-copy operations for genetics data using canonical patterns.

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Zero-copy genetics pool for efficient memory management
#[derive(Debug, Default)]
pub struct GeneticsPool {
    pools: HashMap<String, Vec<u8>>,
    metadata: HashMap<String, PoolMetadata>,
}

/// Pool metadata for tracking usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetadata {
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
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
    pub fn new() -> Self {
        Self {
            pools: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, pool_id: String, size: usize) -> Result<(), BearDogError> {
        if self.pools.contains_key(&pool_id) {
            return Err(BearDogError::system("Pool already exists"));
        }

        let buffer = vec![0u8; size];
        let metadata = PoolMetadata {
            size_bytes: size,
            ..Default::default()
        };

        self.pools.insert(pool_id.clone(), buffer);
        self.metadata.insert(pool_id, metadata);

        Ok(())
    }

    pub fn get_pool(&mut self, pool_id: &str) -> Option<&mut Vec<u8>> {
        if let Some(metadata) = self.metadata.get_mut(pool_id) {
            metadata.last_accessed = Utc::now();
            metadata.access_count += 1;
        }
        self.pools.get_mut(pool_id)
    }

    pub fn deallocate(&mut self, pool_id: &str) -> bool {
        let removed_pool = self.pools.remove(pool_id).is_some();
        let removed_metadata = self.metadata.remove(pool_id).is_some();
        removed_pool && removed_metadata
    }

    pub fn pool_count(&self) -> usize {
        self.pools.len()
    }

    pub fn total_allocated_bytes(&self) -> usize {
        self.metadata.values().map(|m| m.size_bytes).sum()
    }
}

/// Lineage tracking for genetics relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageTracker {
    pub lineages: HashMap<String, LineageInfo>,
}

/// Information about a genetics lineage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageInfo {
    pub genetics_id: String,
    pub parent_ids: Vec<String>,
    pub child_ids: Vec<String>,
    pub generation: u32,
    pub created_at: DateTime<Utc>,
}

impl Default for LineageTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl LineageTracker {
    pub fn new() -> Self {
        Self {
            lineages: HashMap::new(),
        }
    }

    pub fn track_genetics(&mut self, genetics_id: String, parent_ids: Vec<String>) {
        let generation = if parent_ids.is_empty() {
            0
        } else {
            parent_ids
                .iter()
                .filter_map(|id| self.lineages.get(id))
                .map(|info| info.generation)
                .max()
                .unwrap_or(0)
                + 1
        };

        let lineage_info = LineageInfo {
            genetics_id: genetics_id.clone(),
            parent_ids: parent_ids.clone(),
            child_ids: Vec::new(),
            generation,
            created_at: Utc::now(),
        };

        // Update parent records to include this child
        for parent_id in &parent_ids {
            if let Some(parent_info) = self.lineages.get_mut(parent_id) {
                parent_info.child_ids.push(genetics_id.clone());
            }
        }

        self.lineages.insert(genetics_id, lineage_info);
    }

    pub fn get_lineage(&self, genetics_id: &str) -> Option<&LineageInfo> {
        self.lineages.get(genetics_id)
    }

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

    pub fn lineage_count(&self) -> usize {
        self.lineages.len()
    }
}

/// Statistics for lineage analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageStats {
    pub total_lineages: usize,
    pub max_generation: u32,
    pub average_children_per_genetics: f64,
    pub orphan_count: usize, // Genetics with no parents
    pub leaf_count: usize,   // Genetics with no children
}

impl LineageStats {
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

        // Test allocation
        assert!(pool.allocate("test-pool".to_string(), 1024).is_ok());
        assert_eq!(pool.pool_count(), 1);
        assert_eq!(pool.total_allocated_bytes(), 1024);

        // Test duplicate allocation fails
        assert!(pool.allocate("test-pool".to_string(), 512).is_err());

        // Test access
        let buffer = pool.get_pool("test-pool");
        assert!(buffer.is_some());
        assert_eq!(buffer.unwrap().len(), 1024);

        // Test deallocation
        assert!(pool.deallocate("test-pool"));
        assert_eq!(pool.pool_count(), 0);
        assert_eq!(pool.total_allocated_bytes(), 0);
    }

    #[test]
    fn test_lineage_tracker() {
        let mut tracker = LineageTracker::new();

        // Track genesis genetics
        tracker.track_genetics("genesis".to_string(), vec![]);

        // Track first generation
        tracker.track_genetics("child1".to_string(), vec!["genesis".to_string()]);
        tracker.track_genetics("child2".to_string(), vec!["genesis".to_string()]);

        // Track second generation
        tracker.track_genetics(
            "grandchild".to_string(),
            vec!["child1".to_string(), "child2".to_string()],
        );

        assert_eq!(tracker.lineage_count(), 4);

        // Test generation tracking
        assert_eq!(tracker.get_lineage("genesis").unwrap().generation, 0);
        assert_eq!(tracker.get_lineage("child1").unwrap().generation, 1);
        assert_eq!(tracker.get_lineage("grandchild").unwrap().generation, 2);

        // Test descendant tracking
        let descendants = tracker.get_descendants("genesis");
        assert!(descendants.contains(&"child1".to_string()));
        assert!(descendants.contains(&"child2".to_string()));
        assert!(descendants.contains(&"grandchild".to_string()));

        // Test ancestor tracking
        let ancestors = tracker.get_ancestors("grandchild");
        assert!(ancestors.contains(&"child1".to_string()));
        assert!(ancestors.contains(&"child2".to_string()));
        assert!(ancestors.contains(&"genesis".to_string()));
    }

    #[test]
    fn test_lineage_stats() {
        let mut tracker = LineageTracker::new();

        tracker.track_genetics("genesis".to_string(), vec![]);
        tracker.track_genetics("child1".to_string(), vec!["genesis".to_string()]);
        tracker.track_genetics("child2".to_string(), vec!["genesis".to_string()]);

        let stats = LineageStats::calculate(&tracker);

        assert_eq!(stats.total_lineages, 3);
        assert_eq!(stats.max_generation, 1);
        assert_eq!(stats.orphan_count, 1); // Only genesis has no parents
        assert_eq!(stats.leaf_count, 2); // child1 and child2 have no children
    }
}
