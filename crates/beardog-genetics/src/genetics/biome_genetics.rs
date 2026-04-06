// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::entropy_hierarchy::{EntropyClass, EntropyHierarchyManager};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait BiomeGenetics: Send + Sync {
    /// Gets genetic_signature
    fn get_genetic_signature(&self) -> &GeneticSignature;
    /// Validates entropy_quality
    fn validate_entropy_quality(&self, entropy: &EntropyClass) -> f64;
    fn can_authorize_operation(&self, operation: &str) -> bool;
    /// Gets biome_identity
    fn get_biome_identity(&self) -> &BiomeIdentity;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSignature {
    pub biome_id: String,
    /// The signature hash value
    pub signature_hash: String,
    /// The entropy class value
    pub entropy_class: EntropyClass,
    /// The quality score value
    pub quality_score: f64,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Number of lineage_depth
    pub lineage_depth: u32,
    /// Collection of mixed signatures
    pub mixed_signatures: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeIdentity {
    pub biome_id: String,
    /// The biome type value
    pub biome_type: String, // Generic type, not hardcoded primal names
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// The trust level value
    pub trust_level: TrustLevel,
    /// Collection of genetic lineage
    pub genetic_lineage: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Authorization {
    /// State indicating granted
    Granted {
        expires_at: Option<DateTime<Utc>>,
        authorized_operations: Vec<String>,
    },
    Denied {
        reason: String,
        required_quality: f64,
        current_quality: f64,
    },
    Conditional {
        conditions: Vec<String>,
        quality_score: f64,
        expires_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Represents minimal variant
    Minimal,
    /// Represents basic variant
    Basic,
    /// Represents standard variant
    Standard,
    /// Represents high variant
    High,
    /// Represents maximum variant
    Maximum,
}

pub struct GeneticAuthorizationEngine {
    entropy_hierarchy: EntropyHierarchyManager,
    config: GeneticAuthorizationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticAuthorizationConfig {
    /// Number of max_session_duration_hours
    pub max_session_duration_hours: u64,
    /// Whether require_human_entropy is enabled
    pub require_human_entropy: bool,
    /// The trust level thresholds value
    pub trust_level_thresholds: TrustLevelThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustLevelThresholds {
    /// The minimal value
    pub minimal: f64,
    /// The basic value
    pub basic: f64,
    /// The standard value
    pub standard: f64,
    /// The enhanced value
    pub enhanced: f64,
    /// The high value
    pub high: f64,
    /// The maximum value
    pub maximum: f64,
}

impl Default for GeneticAuthorizationConfig {
    fn default() -> Self {
        Self {
            max_session_duration_hours: 24,
            require_human_entropy: false,
            trust_level_thresholds: TrustLevelThresholds {
                minimal: 0.3,
                basic: 0.5,
                standard: 0.7,
                enhanced: 0.8,
                high: 0.85,
                maximum: 0.95,
            },
        }
    }
}

impl GeneticAuthorizationEngine {
    /// New operation.
    /// Creates a new instance
    pub fn new(
        entropy_hierarchy: EntropyHierarchyManager,
        config: GeneticAuthorizationConfig,
    ) -> Self {
        Self {
            entropy_hierarchy,
            config,
        }
    }

    /// Authorize Biome operation.
    pub fn authorize_biome(
        &self,
        biome: &dyn BiomeGenetics,
        requested_operations: &[&str],
    ) -> Result<Authorization, BearDogError> {
        let signature = biome.get_genetic_signature();
        let _identity = biome.get_biome_identity();

        let entropy_quality = self
            .entropy_hierarchy
            .validate_entropy_quality(&signature.entropy_class)?;

        if self.config.require_human_entropy {
            match &signature.entropy_class {
                EntropyClass::HumanLivedExperience { .. } => {}
                EntropyClass::HumanSupervisedMachine { .. } => {}
                EntropyClass::StoreBoughtMachine { .. } => {
                    return Ok(Authorization::Denied {
                        reason: "Human entropy required but only machine entropy provided"
                            .to_string(),
                    });
                }
            }
        }

        if entropy_quality < self.config.min_authorization_threshold {
            return Ok(Authorization::Denied {
                reason: "Insufficient genetic entropy quality".to_string(),
            });
        }

        let authorized_operations: Vec<String> = requested_operations
            .iter()
            .filter(|op| biome.can_authorize_operation(op))
            .cloned()
            .collect();

        if authorized_operations.is_empty() {
            return Ok(Authorization::Denied {
                reason: "No requested operations are authorized for this biome".to_string(),
            });
        }

        let session_hours = if entropy_quality >= 0.9 {
            self.config.max_session_duration_hours
        } else if entropy_quality >= 0.8 {
            self.config.max_session_duration_hours / 2
        } else {
            self.config.max_session_duration_hours / 4
        };

        Ok(Authorization::Granted {
            quality_score: entropy_quality,
            expires_at: Some(Utc::now() + chrono::Duration::hours(session_hours as i64)),
            authorized_operations,
        })
    }

    /// Calculate Trust Level operation.
    pub fn calculate_trust_level(&self, signature: &GeneticSignature) -> TrustLevel {
        let quality = signature.quality_score;
        let thresholds = &self.config.trust_level_thresholds;

        if quality >= thresholds.maximum {
            TrustLevel::Maximum
        } else if quality >= thresholds.high {
            TrustLevel::High
        } else if quality >= thresholds.enhanced {
            TrustLevel::Enhanced
        } else if quality >= thresholds.basic {
            TrustLevel::Basic
        } else {
            TrustLevel::Untrusted
        }
    }

    /// Mix Biome Genetics operation.
    pub fn mix_biome_genetics(
        &self,
        biomes: &[&dyn BiomeGenetics],
    ) -> Result<GeneticSignature, BearDogError> {
        if biomes.is_empty() {
            return Err(BearDogError::invalid_input(
                "No biomes provided for genetic mixing",
            ));
        }

        let signatures: Vec<GeneticSignature> = biomes
            .iter()
            .map(|biome| biome.get_genetic_signature())
            .collect();

        let entropy_classes: Vec<EntropyClass> =
            signatures.iter().map(&|sig| sig.entropy_class).collect();

        let mixed_entropy = self
            .entropy_hierarchy
            .mix_entropy_sources(entropy_classes)?;

        let total_quality: f64 =
            signatures.iter().map(|sig| sig.quality_score).sum::<f64>() / signatures.len() as f64;

        Ok(GeneticSignature {
            biome_id: format!("mixed_{}", Uuid::new_v4()),
            signature_hash: self.calculate_mixed_hash(&signatures),
            entropy_class: mixed_entropy,
            quality_score: total_quality,
            created_at: Utc::now(),
            lineage_depth: signatures
                .iter()
                .map(|sig| sig.lineage_depth)
                .max()
                .unwrap_or(0)
                + 1,
            mixed_signatures: signatures
                .iter()
                .map(|sig| sig.signature_hash.clone())
                .collect(),
        })
    }


    fn calculate_mixed_hash(&self, signatures: &[GeneticSignature]) -> String {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        for signature in signatures {
            hasher.update(signature.signature_hash.as_bytes());
        }
        hasher.update(Utc::now().timestamp().to_le_bytes());

        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug, Clone)]
pub struct UnknownBiome {
    /// The genetic signature value
    pub genetic_signature: GeneticSignature,
    pub identity: BiomeIdentity,
    /// Collection of capabilities
    pub capabilities: Vec<String>,
}

impl BiomeGenetics for UnknownBiome {
    /// Gets genetic_signature
    fn get_genetic_signature(&self) -> &GeneticSignature {
        &self.genetic_signature
    }

    /// Validates entropy_quality
    fn validate_entropy_quality(&self, entropy: &EntropyClass) -> f64 {
        match entropy {
            EntropyClass::HumanLivedExperience { .. } => 1.0,
            EntropyClass::HumanSupervisedMachine { .. } => 0.8,
            EntropyClass::StoreBoughtMachine {
                reproducibility_index,
                ..
            } => 1.0 - reproducibility_index.min(1.0),
        }
    }


    fn can_authorize_operation(&self, operation: &str) -> bool {
        self.capabilities.contains(&operation.to_string())
    }

    /// Gets biome_identity
    fn get_biome_identity(&self) -> BiomeIdentity {
        &self.identity
    }
}
