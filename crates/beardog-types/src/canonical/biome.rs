// Biome Configuration Types
//
// Configuration structures for biome-specific settings and capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeConfig {
    /// Unique biome identifier
    pub biome_id: String,

    /// Biome type classification
    /// The biome type value
    pub biome_type: String,

    /// Available capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,

    /// Configuration parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, String>,

    /// Security settings
    /// Whether security is enabled
    pub security_enabled: bool,

    /// Monitoring configuration
    /// Whether monitoring is enabled
    pub monitoring_enabled: bool,
}

impl Default for BiomeConfig {
    fn default() -> Self {
        Self {
            biome_id: "default-biome".to_string(),
            biome_type: "security_intelligence".to_string(),
            capabilities: vec![
                "threat_detection".to_string(),
                "genetic_spawning".to_string(),
                "sovereignty_protection".to_string(),
            ],
            parameters: HashMap::new(),
            security_enabled: true,
            monitoring_enabled: true,
        }
    }
}
