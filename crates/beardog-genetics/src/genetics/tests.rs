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


/// Unit tests for genetics module
///
/// Contains comprehensive test functions for genetic key evolution functionality.

#[cfg(test)]
mod tests {
    use super::super::*;
    use beardog_errors::BearDogResult;
    use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};
    #[tokio::test]
    async fn test_genetic_engine_creation() -> GeneticsResult<()> {
        let config = GeneticsConfig::default();
        let engine = GeneticsEngine::new(config).await?;
        
        assert!(engine.is_enabled());
        Ok(())
    }
    async fn test_key_evolution() -> GeneticsResult<()> {
        let parent_key = "parent_key_material".as_bytes().to_vec();
        let evolved_key = engine.evolve_key(&parent_key, 1).await?;
        // Evolved key should be different from parent
        assert_ne!(parent_key, evolved_key);
        assert!(!evolved_key.is_empty());}


    async fn test_genetic_lineage_tracking() -> GeneticsResult<()> {
        let parent_key = "lineage_test_key".as_bytes().to_vec();
        let generation_1 = engine.evolve_key(&parent_key, 1).await?;
        let generation_2 = engine.evolve_key(&generation_1, 2).await?;
        // Each generation should be unique
        assert_ne!(parent_key, generation_1);
        assert_ne!(generation_1, generation_2);
        assert_ne!(parent_key, generation_2);
    async fn test_genetic_diversity() -> GeneticsResult<()> {
        let parent_key = "diversity_test_key".as_bytes().to_vec();
        let mut evolved_keys = Vec::new();
        // Generate multiple evolved keys from same parent
        for i in 0..5 {
            let evolved = engine.evolve_key(&parent_key, i).await?;
            evolved_keys.push(evolved);
        }
        // All evolved keys should be different
        for i in 0..evolved_keys.len() {
            for j in i+1..evolved_keys.len() {
                assert_ne!(evolved_keys[i], evolved_keys[j]);
            }
    #[test]
    fn test_genetics_config_defaults() {
        assert!(config.enabled);
        assert!(config.evolution_strength > 0.0);
        assert!(config.mutation_rate > 0.0);
        assert!(config.max_generations > 0);}


    async fn test_genetic_metadata() -> GeneticsResult<()> {
        let parent_key = "metadata_test_key".as_bytes().to_vec();
        // Test that genetic metadata can be extracted
        let metadata = engine.extract_genetic_metadata(&evolved_key).await?;
        assert!(metadata.contains_key("generation"));
        assert!(metadata.contains_key("parent_hash"));
    async fn test_genetic_validation() -> GeneticsResult<()> {
        let parent_key = "validation_test_key".as_bytes().to_vec();
        // Test key validation
        let is_valid = engine.validate_genetic_key(&evolved_key).await?;
        assert!(is_valid);
        // Test invalid key
        let invalid_key = "not_a_genetic_key".as_bytes().to_vec();
        let is_invalid = engine.validate_genetic_key(&invalid_key).await?;
        assert!(!is_invalid);}


    async fn test_genetic_optimization() -> GeneticsResult<()> {
        let mut config = GeneticsConfig::default();
        config.optimization_enabled = true;
        let parent_key = "optimization_test_key".as_bytes().to_vec();
        // Optimized evolution should produce valid results
}
