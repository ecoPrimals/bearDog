//! Genetics Types
//!
//! Core type definitions for the genetics system using canonical patterns.

use beardog_auth::auth::BearDogGenetics;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// In-memory genetics store for testing and development
#[derive(Debug, Default)]
pub struct InMemoryGeneticsStore {
    genetics: HashMap<String, BearDogGenetics>,
}

impl InMemoryGeneticsStore {
    pub fn new() -> Self {
        Self {
            genetics: HashMap::new(),
        }
    }

    pub fn insert(&mut self, id: String, genetics: BearDogGenetics) {
        self.genetics.insert(id, genetics);
    }

    pub fn get(&self, id: &str) -> Option<&BearDogGenetics> {
        self.genetics.get(id)
    }

    pub fn remove(&mut self, id: &str) -> Option<BearDogGenetics> {
        self.genetics.remove(id)
    }

    pub fn len(&self) -> usize {
        self.genetics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.genetics.is_empty()
    }

    pub fn clear(&mut self) {
        self.genetics.clear();
    }
}

impl super::GeneticsStore for InMemoryGeneticsStore {
    fn store_genetics(&self, genetics: &BearDogGenetics) -> Result<(), BearDogError> {
        // In a real implementation, this would be mutable or use interior mutability
        // For now, we'll just validate and return success
        if genetics.id.is_empty() {
            return Err(BearDogError::system("Genetics ID cannot be empty"));
        }
        Ok(())
    }

    fn get_genetics(&self, id: &str) -> Result<BearDogGenetics, BearDogError> {
        self.genetics
            .get(id)
            .cloned()
            .ok_or_else(|| BearDogError::system("Genetics not found"))
    }
}

/// Genetics metadata for tracking and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_modified: chrono::DateTime<chrono::Utc>,
    pub version: u32,
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

/// Genetics statistics for monitoring and analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsStats {
    pub total_genetics: usize,
    pub average_fitness: f64,
    pub max_generation: u32,
    pub capability_distribution: HashMap<String, usize>,
}

impl Default for GeneticsStats {
    fn default() -> Self {
        Self {
            total_genetics: 0,
            average_fitness: 0.0,
            max_generation: 0,
            capability_distribution: HashMap::new(),
        }
    }
}

/// Genetics configuration for system behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsSystemConfig {
    pub max_stored_genetics: usize,
    pub cleanup_interval_seconds: u64,
    pub enable_metrics_collection: bool,
    pub fitness_threshold: f64,
}

impl Default for GeneticsSystemConfig {
    fn default() -> Self {
        Self {
            max_stored_genetics: 1000,
            cleanup_interval_seconds: 3600, // 1 hour
            enable_metrics_collection: true,
            fitness_threshold: 0.5,
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

        let genetics = BearDogGenetics {
            id: "test-genetics".to_string(),
            capabilities: vec![NodeCapability::ComputeProvider],
            security_clearance: SecurityClearance::Basic,
            fitness_score: 0.8,
            ..Default::default()
        };

        store.insert(genetics.id.clone(), genetics.clone());

        assert_eq!(store.len(), 1);
        assert!(!store.is_empty());

        let retrieved = store.get("test-genetics");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, "test-genetics");

        let removed = store.remove("test-genetics");
        assert!(removed.is_some());
        assert!(store.is_empty());
    }

    #[test]
    fn test_genetics_metadata() {
        let metadata = GeneticsMetadata::default();
        assert_eq!(metadata.version, 1);
        assert!(metadata.tags.is_empty());
        assert!(metadata.created_at <= chrono::Utc::now());
    }
}
