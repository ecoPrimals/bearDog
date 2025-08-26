

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

        assert_ne!(parent_key, evolved_key);
        assert!(!evolved_key.is_empty());}

    async fn test_genetic_lineage_tracking() -> GeneticsResult<()> {
        let parent_key = "lineage_test_key".as_bytes().to_vec();
        let generation_1 = engine.evolve_key(&parent_key, 1).await?;
        let generation_2 = engine.evolve_key(&generation_1, 2).await?;

        assert_ne!(parent_key, generation_1);
        assert_ne!(generation_1, generation_2);
        assert_ne!(parent_key, generation_2);
    async fn test_genetic_diversity() -> GeneticsResult<()> {
        let parent_key = "diversity_test_key".as_bytes().to_vec();
        let mut evolved_keys = Vec::new();

        for i in 0..5 {
            let evolved = engine.evolve_key(&parent_key, i).await?;
            evolved_keys.push(evolved);
        }

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

        let metadata = engine.extract_genetic_metadata(&evolved_key).await?;
        assert!(metadata.contains_key("generation"));
        assert!(metadata.contains_key("parent_hash"));
    async fn test_genetic_validation() -> GeneticsResult<()> {
        let parent_key = "validation_test_key".as_bytes().to_vec();

        let is_valid = engine.validate_genetic_key(&evolved_key).await?;
        assert!(is_valid);

        let invalid_key = "not_a_genetic_key".as_bytes().to_vec();
        let is_invalid = engine.validate_genetic_key(&invalid_key).await?;
        assert!(!is_invalid);}

    async fn test_genetic_optimization() -> GeneticsResult<()> {
        let mut config = GeneticsConfig::default();
        config.optimization_enabled = true;
        let parent_key = "optimization_test_key".as_bytes().to_vec();

}
