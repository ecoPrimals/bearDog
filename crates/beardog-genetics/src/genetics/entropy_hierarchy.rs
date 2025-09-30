// Entropy Hierarchy Module
//
// This module provides entropy hierarchy management and validation capabilities,
// supporting human-centric entropy classification.

pub mod engine;
pub mod monitoring;
pub mod sources;
pub mod types;
pub mod validation;

// Re-export all types
pub use engine::*;
pub use monitoring::*;
pub use sources::*;
pub use types::*;
pub use validation::*;

// Note: BearDogError, Deserialize, and Serialize imports removed as they are unused in this module
// They can be re-added when needed for future implementations

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_entropy_hierarchy_creation() {
        let config = EntropyHierarchyConfig::default();
        let _manager = EntropyHierarchyManager::new(config);
    }
}
