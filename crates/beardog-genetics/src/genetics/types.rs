// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct InMemoryGeneticsStore {
    genetics: HashMap<String, BearDogGenetics>,
}

impl InMemoryGeneticsStore {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            genetics: HashMap::with_capacity(100),
        }
    }

    /// Insert genetics into the store
    pub fn insert(&mut self, id: &str, genetics: BearDogGenetics) {
        self.genetics.insert(id.to_string(), genetics);
    }

    /// Get operation.
    /// Gets value
    /// Gets value
    pub fn get(&self, id: &str) -> Option<&BearDogGenetics> {
        self.genetics.get(id)
    }

    /// Remove operation.
    /// Removes item
    /// Removes item
    pub fn remove(&mut self, id: &str) -> Option<BearDogGenetics> {
        self.genetics.remove(id)
    }

    /// Len operation.
    pub fn len(&self) -> usize {
        self.genetics.len()
    }

    /// Is Empty operation.
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.genetics.is_empty()
    }

    /// Clear operation.
    pub fn clear(&mut self) {
        self.genetics.clear();
    }
}

impl super::GeneticsStore for InMemoryGeneticsStore {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> Result<(), BearDogError> {
        if genetics.id.is_empty() {
            return Err(BearDogError::system(
                "Genetics ID cannot be empty".to_string(),
            ));
        }
        Ok(())
    }

    /// Gets genetics
    fn get_genetics(&self, id: &str) -> Result<BearDogGenetics, BearDogError> {
        self.genetics
            .get(id)
            .cloned()
            .ok_or_else(|| BearDogError::system(format!("Genetics not found: {}", id)))
    }
}

/// Metadata associated with genetics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsMetadata {
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The last modified value
    pub last_modified: chrono::DateTime<chrono::Utc>,
    /// Number of version
    pub version: u32,
    /// Collection of tags
    pub tags: Vec<String>,
}

impl Default for GeneticsMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            created_at: now,
            last_modified: now,
            version: 1,
            tags: vec![],
        }
    }
}

/// Statistics about the genetics system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsStats {
    /// Number of total_genetics
    pub total_genetics: usize,
    /// The average fitness value
    pub average_fitness: f64,
    /// Number of max_generation
    pub max_generation: u32,
    /// Mapping of capability distribution
    pub capability_distribution: HashMap<String, usize>,
}

impl Default for GeneticsStats {
    fn default() -> Self {
        Self {
            total_genetics: 0,
            average_fitness: 0.0,
            max_generation: 0,
            capability_distribution: HashMap::with_capacity(10),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsSystemConfig {
    /// Number of max_genetics_stored
    pub max_genetics_stored: usize,
    /// Number of cleanup_interval_seconds
    pub cleanup_interval_seconds: u64,
    /// Whether enable_metrics_collection is enabled
    pub enable_metrics_collection: bool,
    /// The fitness threshold value
    pub fitness_threshold: f64,
}

impl Default for GeneticsSystemConfig {
    fn default() -> Self {
        Self {
            max_genetics_stored: 1000,
            cleanup_interval_seconds: 3600,
            enable_metrics_collection: true,
            fitness_threshold: 0.5,
        }
    }
}

impl GeneticsSystemConfig {
    /// Load from `BEARDOG_MAX_GENETICS_STORED`, `BEARDOG_GENETICS_CLEANUP_INTERVAL_SECS`, `BEARDOG_GENETICS_TYPES_FITNESS_THRESHOLD`.
    pub fn from_env() -> Self {
        Self {
            max_genetics_stored: std::env::var("BEARDOG_MAX_GENETICS_STORED")
                .ok()
                .and_then(|g| g.parse().ok())
                .unwrap_or(1000),
            cleanup_interval_seconds: std::env::var("BEARDOG_GENETICS_CLEANUP_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600),
            enable_metrics_collection: true,
            fitness_threshold: std::env::var("BEARDOG_GENETICS_TYPES_FITNESS_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_auth::auth::{NodeCapability, SecurityClearance};

    #[test]
    fn test_in_memory_store() {
        let mut store = InMemoryGeneticsStore::new();
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: genetics
 // TEST_PRIORITY: normal

        let genetics = BearDogGenetics {
            id: "test-genetics".to_string(),
            capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            fitness_score: 0.8,
            ..Default::default()
        };

        store.insert(&genetics.id.clone(), genetics);

        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());

        let retrieved = store.get("test-genetics");
        assert!(retrieved.is_some());
        assert_eq!(retrieved?.id, "test-genetics");

        let removed = store.remove("test-genetics");
        assert!(removed.is_some());
        assert!(store.is_empty());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: genetics
    // TEST_PRIORITY: normal
    #[test]
    fn test_genetics_metadata() {
        let metadata = GeneticsMetadata::default();
        assert_eq!(metadata.version, 1);
        assert!(metadata.tags.is_empty());
        assert!(metadata.created_at <= chrono::Utc::now());
    }
}
