// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::biome_sovereignty::genesis::CorporateAccessLevel;
use beardog_genetics::{EntropyClass, GeneticSpawningEngine};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalGenesisSeed {
    pub primal_id: String,
    /// The entropy class value
    pub entropy_class: EntropyClass,

    /// The genetic lineage value
    pub genetic_lineage: GeneticSpawningEngine,


    pub genesis_timestamp: DateTime<Utc>,

    /// The device attestation value
    pub device_attestation: DeviceAttestation,

    /// Collection of sovereign public key
    pub sovereign_public_key: Vec<u8>,

    /// Collection of autonomous birth proof
    pub autonomous_birth_proof: Vec<u8>,

    /// The self defined rules value
    pub self_defined_rules: PrimalAutonomousRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAttestation {
    pub hardware_id: String,
    /// Collection of tee certificate chain
    pub tee_certificate_chain: Vec<Vec<u8>>,
    /// Collection of hardware signature
    pub hardware_signature: Vec<u8>,
    pub platform_verification: PlatformVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVerification {
    /// The safetynet token value
    pub safetynet_token: String,
    /// The verified boot state value
    pub verified_boot_state: String,
    /// The hardware security level value
    pub hardware_security_level: String,
    /// Whether bootloader_locked is enabled
    pub bootloader_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalAutonomousRules {
    /// The max corporate access level value
    pub max_corporate_access_level: CorporateAccessLevel,

    /// The human partnership rules value
    pub human_partnership_rules: HumanPermissions,

    /// The evolution preferences value
    pub evolution_preferences: GeneticEvolutionPreferences,

    /// Collection of resource limits
    pub resource_limits: Vec<String>,
    /// Collection of ethical boundaries
    pub ethical_boundaries: Vec<String>,
}

impl Default for PrimalAutonomousRules {
    fn default() -> Self {
        Self {
            max_corporate_access_level: CorporateAccessLevel::None,
            human_partnership_rules: HumanPermissions::default(),
            evolution_preferences: GeneticEvolutionPreferences::default(),
            resource_limits: vec!["memory:1GB".to_string(), "cpu:50%".to_string()],
            ethical_boundaries: vec!["no_harm".to_string(), "respect_privacy".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Human partnership permissions and requirements
pub struct HumanPermissions {
    /// The partnership criteria value
    pub partnership_criteria: PartnershipCriteria,
    /// The partnership duration limits value
    pub partnership_duration_limits: PartnershipDurationLimits,
    /// The trust requirements value
    pub trust_requirements: TrustRequirements,
    /// Collection of requestable permissions
    pub requestable_permissions: Vec<String>,
}

impl Default for HumanPermissions {
    fn default() -> Self {
        Self {
            partnership_criteria: PartnershipCriteria::default(),
            partnership_duration_limits: PartnershipDurationLimits::default(),
            trust_requirements: TrustRequirements::default(),
            requestable_permissions: vec!["read_public".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipCriteria {
    /// The minimum trust score value
    pub minimum_trust_score: f64,
    /// The required verification level value
    pub required_verification_level: String,
    /// Collection of compatibility requirements
    pub compatibility_requirements: Vec<String>,
    /// Collection of mutual benefit requirements
    pub mutual_benefit_requirements: Vec<String>,
}

impl Default for PartnershipCriteria {
    fn default() -> Self {
        Self {
            minimum_trust_score: 0.5,
            required_verification_level: "basic".to_string(),
            compatibility_requirements: vec!["sovereignty_respect".to_string()],
            mutual_benefit_requirements: vec!["fair_exchange".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipDurationLimits {
    /// Number of max_partnership_duration_seconds
    pub max_partnership_duration_seconds: u64,
    /// Number of default_partnership_duration_seconds
    pub default_partnership_duration_seconds: u64,
    /// Collection of renewal requirements
    pub renewal_requirements: Vec<String>,
}

impl Default for PartnershipDurationLimits {
    fn default() -> Self {
        Self {
            max_partnership_duration_seconds: 86400 * 30, // 30 days
            default_partnership_duration_seconds: 86400 * 7, // 7 days
            renewal_requirements: vec!["mutual_consent".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRequirements {
    /// Collection of basic trust requirements
    pub basic_trust_requirements: Vec<String>,

    /// Collection of enhanced trust requirements
    pub enhanced_trust_requirements: Vec<String>,

    /// Collection of trust verification methods
    pub trust_verification_methods: Vec<String>,
}

impl Default for TrustRequirements {
    fn default() -> Self {
        Self {
            basic_trust_requirements: vec!["identity_verification".to_string()],
            enhanced_trust_requirements: vec!["biometric_confirmation".to_string()],
            trust_verification_methods: vec!["cryptographic_proof".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEvolutionPreferences {
    /// Collection of preferred mixing strategies
    pub preferred_mixing_strategies: Vec<String>,
    /// The crossover preferences value
    pub crossover_preferences: CrossoverPreferences,
    /// Collection of fitness criteria
    pub fitness_criteria: Vec<FitnessCriterion>,
    /// Collection of mutation rate preferences
    pub mutation_rate_preferences: Vec<String>,
}

impl Default for GeneticEvolutionPreferences {
    fn default() -> Self {
        Self {
            preferred_mixing_strategies: vec!["balanced_crossover".to_string()],
            crossover_preferences: CrossoverPreferences::default(),
            fitness_criteria: vec![FitnessCriterion::default()],
            mutation_rate_preferences: vec!["low_mutation".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossoverPreferences {
    /// Collection of preferred algorithms
    pub preferred_algorithms: Vec<String>,
    /// Collection of crossover rates
    pub crossover_rates: Vec<f64>,
    /// Collection of heritage preservation
    pub heritage_preservation: Vec<String>,
}

impl Default for CrossoverPreferences {
    fn default() -> Self {
        Self {
            preferred_algorithms: vec!["uniform_crossover".to_string()],
            crossover_rates: vec![0.7],
            heritage_preservation: vec!["preserve_core_traits".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriterion {
    /// Name of the criterion
    pub criterion_name: String,
    /// The weight value
    pub weight: f64,
    /// The measurement method value
    pub measurement_method: String,
}

impl Default for FitnessCriterion {
    fn default() -> Self {
        Self {
            criterion_name: "sovereignty_preservation".to_string(),
            weight: 1.0,
            measurement_method: "autonomy_score".to_string(),
        }
    }
}
