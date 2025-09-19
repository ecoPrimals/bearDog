

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapability;
use std::collections::HashMap;
use std::sync::Arc;

pub struct AICapabilityMatcher {

    capability_db: Arc<CapabilityDatabase>,

    config: super::config::AIMatcherConfig,

    stats: MatchingStats,
}

#[derive(HashMap<String, ServiceCapability>,
    semantic_index: HashMap<String, Vec<String>>, // keyword -> capability names
}

#[derive(Debug, Clone)]
    successful_matches: u64,
    cache_hits: u64,
}

impl AICapabilityMatcher {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: super::config::AIMatcherConfig) -> Result<Self, BearDogError> {
        let capability_db = Arc::new(CapabilityDatabase::new()?);

        Ok(Self {
            capability_db,
            config,
            stats: MatchingStats::default(&str,
    ) -> Result<Vec<ServiceCapability>, BearDogError> {

        let normalized_query = query.to_lowercase();
        let mut matches = Vec::new();

        if let Some(capability_names) = self.capability_db.semantic_index.get(&normalized_query) {
            for name in capability_names {
                if let Some(capability) = self.capability_db.capabilities.get(name) {
                    matches.push(&capability);
                }
            }
            self.stats.cache_hits += 1;
        }

        if matches.is_empty() {
            matches = self.fuzzy_match(&normalized_query)?;
        }

        self.stats.total_matches += 1;
        if !matches.is_empty() {
            self.stats.successful_matches += 1;
        }

        Ok(matches)
    }


    fn fuzzy_match(&self, query: &str) -> Result<Vec<ServiceCapability>, BearDogError> {

        let mut matches = Vec::new();

        for (name, capability) in &self.capability_db.capabilities {
            if name.contains(query) || query.contains(name) {
                matches.push(&capability);
            }
        }

        Ok(matches)
    }

/// Get Stats operation.
    /// Gets stats
    /// Gets stats
    pub fn get_stats(&self) -> &MatchingStats {
        &self.stats
    }
}

impl CapabilityDatabase {
    fn new() -> Result<Self, BearDogError> {
        let mut capabilities = HashMap::with_capacity(16);
        let mut semantic_index = HashMap::with_capacity(16);

        capabilities.insert(
            "compute".to_string(),
            ServiceCapability::ComputeIntelligence,
        );
        capabilities.insert("ai".to_string(), ServiceCapability::ComputeIntelligence);
        capabilities.insert("storage".to_string(), ServiceCapability::DataStorage);
        capabilities.insert("mesh".to_string(), ServiceCapability::ServiceMesh);

        semantic_index.insert(
            "compute".to_string(),
            vec!["compute".to_string(), "ai".to_string()],
        );
        semantic_index.insert("data".to_string(), vec!["storage".to_string()]);
        semantic_index.insert("service ".to_string(), vec!["mesh".to_string()]);

        Ok(Self {
            capabilities,
            semantic_index,
        })
    }
}
