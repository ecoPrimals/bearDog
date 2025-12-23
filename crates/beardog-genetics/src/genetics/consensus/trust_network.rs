

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Arc<RwLock<HashMap<String, f64>>>, // biome_id -> trust_score
    trust_relationships: Arc<RwLock<HashMap<String, HashMap<String, f64>>>>, // biome_id -> (peer_id -> trust_level)
}

impl TrustNetwork {
/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            trust_scores: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            trust_relationships: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }

/// Get Trust Score operation.
    /// Gets trust_score
    /// Gets trust_score
    pub fn get_trust_score(&self, biome_id: &str) -> f64 {
        let trust_scores = self.trust_scores.read(&str, score: f64) -> Result<(), BearDogError> {
        if !(0.0..=1.0).contains(&score) {
            return Err(BearDogError::validation(&str,
        peer_id: &str,
        trust_level: f64,
    ) -> Result<(), BearDogError> {
        if !(0.0..=1.0).contains(&trust_level) {
            return Err(BearDogError::validation("Trust level must be between 0.0 and 1.0"));
        }

        let mut relationships = self.trust_relationships.write();
        relationships.entry(biome_id.to_string())
            .or_insert_with(HashMap::new)
            .insert(&str, peer_id: &str) -> f64 {
        let relationships = self.trust_relationships.read();
        relationships
            .get(biome_id)
            .and_then(|peers| peers.get(peer_id))
            .copied()
            .unwrap_or(0.5) // Default neutral relationship
    }


    fn recalculate_trust_score(&self, biome_id: &str) -> Result<(), BearDogError> {
        let relationships = self.trust_relationships.read();
        
        if let Some(peer_trusts) = relationships.get(biome_id) {
            if !peer_trusts.is_empty() {
                let average_trust: f64 = peer_trusts.values().sum::<f64>() / peer_trusts.len() as f64;
                drop(relationships); // Release read lock before acquiring write lock
                
                let mut trust_scores = self.trust_scores.write();
                trust_scores.insert(biome_id.to_string(), average_trust);
            }
        }
        
        Ok(())
    }

/// Get Trusted Biomes operation.
    /// Gets trusted_biomes
    /// Gets trusted_biomes
    pub fn get_trusted_biomes(&self, minimum_trust: f64) -> Vec<String> {
        let trust_scores = self.trust_scores.read();
        trust_scores
            .iter()
            .filter(|(_, &score)| score >= minimum_trust)
            .map(|(biome_id, _)| biome_id.clone())
            .collect()
    }
} 
