// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_auth::auth::types::genetics::BearDogGenetics;
use beardog_genetics::EntropyClass;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Genetic algorithm configuration
///
/// Configuration for genetic algorithm-based optimization including
/// population parameters, mutation/crossover rates, and selection methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticAlgorithmConfig {
    /// Size of the population in each generation
    pub population_size: usize,
    /// Probability of mutation occurring (0.0 to 1.0)
    pub mutation_rate: f64,
    /// Probability of crossover occurring (0.0 to 1.0)
    pub crossover_rate: f64,
    /// Maximum number of generations to evolve
    pub max_generations: usize,
    /// Fitness threshold for early termination
    pub fitness_threshold: f64,
    /// Whether to preserve best individuals across generations
    pub elitism: bool,
    /// Selection method (e.g., "tournament", "roulette")
    pub selection_method: String,
    /// Parameters for maintaining genetic diversity
    pub diversity_parameters: DiversityConfig,
}

/// Partnership configuration
///
/// Configuration for partnerships between entities including
/// trust requirements, partner limits, and discovery settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipConfig {
    /// Maximum number of partners allowed in key sharing
    pub max_partners: usize,
    /// Minimum trust score required for partnerships (0.0 to 1.0)
    pub min_trust_score: f64,
    /// Duration of partnerships in seconds
    pub partnership_duration_secs: u64,
    /// Whether to enable automatic partner discovery
    pub auto_discovery: bool,
}

/// Corporate access control and sovereignty protection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateAccessControl {
    /// Block all corporate surveillance attempts
    /// Whether `block_corporate_access` is enabled
    pub block_corporate_access: bool,
    /// List of explicitly blocked corporate entities
    /// Collection of blocked entities
    pub blocked_entities: Vec<String>,
    /// Enable privacy protection mechanisms
    /// Whether `privacy_protection` is enabled
    pub privacy_protection: bool,
    /// The sovereignty level value
    pub sovereignty_level: SovereigntyLevel,
}

/// Privacy protection mechanism types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrivacyMechanism {
    /// Zero-knowledge proof systems for privacy-preserving authentication
    ZeroKnowledgeProofs,
    /// Homomorphic encryption for computation on encrypted data
    HomomorphicEncryption,
    /// Secure multi-party computation protocols
    SecureMultipartyComputation,
    /// Data minimization and retention policies
    DataMinimization,
    /// Anonymous credential systems
    AnonymousCredentials,
}

/// Privacy protection settings
///
/// Advanced privacy protection mechanisms including zero-knowledge proofs,
/// homomorphic encryption, and secure multi-party computation.
///
/// Uses a set-based approach for enabled mechanisms, following modern
/// idiomatic Rust patterns and avoiding excessive boolean fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyProtectionSettings {
    /// Enabled privacy protection mechanisms
    #[serde(default = "default_privacy_mechanisms")]
    pub enabled_mechanisms: std::collections::HashSet<PrivacyMechanism>,
}

fn default_privacy_mechanisms() -> std::collections::HashSet<PrivacyMechanism> {
    [
        PrivacyMechanism::ZeroKnowledgeProofs,
        PrivacyMechanism::DataMinimization,
    ]
    .into_iter()
    .collect()
}

impl Default for PrivacyProtectionSettings {
    fn default() -> Self {
        Self {
            enabled_mechanisms: default_privacy_mechanisms(),
        }
    }
}

impl PrivacyProtectionSettings {
    /// Check if zero-knowledge proofs are enabled
    #[must_use]
    pub fn uses_zero_knowledge_proofs(&self) -> bool {
        self.enabled_mechanisms
            .contains(&PrivacyMechanism::ZeroKnowledgeProofs)
    }

    /// Check if homomorphic encryption is enabled
    #[must_use]
    pub fn uses_homomorphic_encryption(&self) -> bool {
        self.enabled_mechanisms
            .contains(&PrivacyMechanism::HomomorphicEncryption)
    }

    /// Check if secure multiparty computation is enabled
    #[must_use]
    pub fn uses_secure_multiparty_computation(&self) -> bool {
        self.enabled_mechanisms
            .contains(&PrivacyMechanism::SecureMultipartyComputation)
    }

    /// Check if data minimization is enabled
    #[must_use]
    pub fn uses_data_minimization(&self) -> bool {
        self.enabled_mechanisms
            .contains(&PrivacyMechanism::DataMinimization)
    }

    /// Check if anonymous credentials are enabled
    #[must_use]
    pub fn uses_anonymous_credentials(&self) -> bool {
        self.enabled_mechanisms
            .contains(&PrivacyMechanism::AnonymousCredentials)
    }
}

/// Genetic diversity maintenance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityConfig {
    /// Minimum genetic diversity threshold
    /// The min diversity value
    pub min_diversity: f64,
    /// Diversity measurement method
    /// The diversity metric value
    pub diversity_metric: String,
    /// Enable adaptive mutation rates
    /// Whether `adaptive_mutation` is enabled
    pub adaptive_mutation: bool,
    /// The immigration rate value
    pub immigration_rate: f64,
}

/// Human identity verification and authentication settings
///
/// Configuration for human identity verification including biometric
/// authentication, multi-factor requirements, and continuous auth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanIdentityConfig {
    /// Biometric authentication methods enabled (e.g., "fingerprint", "face")
    pub biometric_auth: Vec<String>,
    /// Whether multi-factor authentication is required
    pub mfa_required: bool,
    /// Identity verification confidence threshold (0.0 to 1.0)
    pub verification_threshold: f64,
    /// Enable continuous authentication monitoring
    pub continuous_auth: bool,
}

/// Genetic key evolution and adaptation parameters
///
/// Configuration for how cryptographic keys evolve and adapt over time
/// based on environmental conditions and security requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEvolutionConfig {
    /// Speed multiplier for evolutionary changes (1.0 = normal speed)
    pub evolution_speed: f64,
    /// Whether keys adapt to environmental changes
    pub environmental_adaptation: bool,
    /// Whether key strength evolves over time
    pub strength_evolution: bool,
}

/// Sovereignty protection level
///
/// Defines the level of sovereignty protection applied to operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SovereigntyLevel {
    /// Minimal sovereignty protection
    Minimal,
    /// Standard sovereignty protection (default)
    Standard,
    /// Enhanced sovereignty with strict controls
    Enhanced,
    /// Maximum sovereignty with zero corporate access
    Maximum,
}

/// Primal genesis seed
///
/// The foundational seed for a primal's identity, containing its genesis
/// entropy, genetic lineage, device attestation, and autonomous rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalGenesisSeed {
    /// Unique identifier for this primal
    pub primal_id: String,
    /// Genesis entropy class for randomness generation
    pub genesis_entropy: EntropyClass,
    /// Genetic lineage information
    pub genetic_lineage: BearDogGenetics,
    /// Timestamp when the genesis seed was created
    pub genesis_timestamp: DateTime<Utc>,
    /// Device attestation proving hardware security
    pub device_attestation: DeviceAttestation,
    /// Sovereign public key for this primal
    pub sovereign_public_key: Vec<u8>,
    /// Cryptographic proof of autonomous birth/creation
    pub autonomous_birth_proof: Vec<u8>,
    /// Self-defined autonomous rules for this biome
    pub self_defined_rules: BiomeAutonomousRules,
}

/// Device attestation
///
/// Cryptographic proof that the device meets security requirements
/// including trusted execution environment and hardware verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAttestation {
    /// Unique device identifier
    pub device_id: String,
    /// Certificate chain proving trusted execution environment
    pub tee_certificate_chain: Vec<Vec<u8>>,
    /// Hardware-generated cryptographic signature
    pub hardware_signature: Vec<u8>,
    /// Platform verification information
    pub platform_verification: PlatformVerification,
}

/// Platform verification information
///
/// Information about the platform's security state including boot
/// verification, hardware security level, and bootloader status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVerification {
    /// Name of the platform (e.g., "Android", "iOS")
    pub platform_name: String,
    /// Verified boot state confirmation
    pub verified_boot_state: String,
    /// Hardware security level assessment
    pub hardware_security_level: String,
    /// Whether the bootloader is locked
    pub bootloader_locked: bool,
}

impl Default for PlatformVerification {
    fn default() -> Self {
        Self {
            platform_name: "Unknown".to_string(),
            verified_boot_state: "unverified".to_string(),
            hardware_security_level: "basic".to_string(),
            bootloader_locked: false,
        }
    }
}

/// Autonomous rules governing biome behavior and interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeAutonomousRules {
    /// Corporate access control level settings
    /// The corporate access level value
    pub corporate_access_level: CorporateAccessLevel,
    /// Human partnership rules and permissions
    /// The human partnership rules value
    pub human_partnership_rules: HumanPermissions,
    /// Genetic evolution preferences and constraints
    /// The evolution preferences value
    pub evolution_preferences: GeneticEvolutionPreferences,
    /// Resource usage limits and boundaries
    /// Collection of resource limits
    pub resource_limits: Vec<String>,
    /// Ethical boundaries and constraints
    /// Collection of ethical boundaries
    pub ethical_boundaries: Vec<String>,
}

/// Human permissions and partnership configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanPermissions {
    /// Whether `consent_required` is enabled
    pub consent_required: bool,
    /// The partnership criteria value
    pub partnership_criteria: PartnershipCriteria,
    /// The partnership duration limits value
    pub partnership_duration_limits: PartnershipDurationLimits,
    /// The trust requirements value
    pub trust_requirements: TrustRequirements,
    /// List of permissions that humans can request
    /// Collection of requestable permissions
    pub requestable_permissions: Vec<String>,
}

/// Criteria for establishing partnerships between biomes
///
/// Defines the minimum requirements and standards that must be met
/// before two biomes can form a partnership relationship.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipCriteria {
    /// Minimum trust level required (0.0-1.0)
    pub minimum_trust_level: f64,
    /// Required verification level (e.g., "basic", "enhanced", "sovereign")
    pub required_verification_level: String,
    /// Technical and operational compatibility requirements
    pub compatibility_requirements: Vec<String>,
    /// Mutual benefit and value exchange requirements
    pub mutual_benefit_requirements: Vec<String>,
}

/// Time limits and renewal policies for partnerships
///
/// Controls how long partnerships can last and under what conditions
/// they can be renewed or extended.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipDurationLimits {
    /// Maximum allowed partnership duration in seconds
    pub max_partnership_duration_seconds: u64,
    /// Default partnership duration when not otherwise specified
    pub default_partnership_duration_seconds: u64,
    /// Requirements that must be met to renew an expiring partnership
    pub renewal_requirements: Vec<String>,
}

/// Trust requirements for biome interactions
///
/// Defines the trust level and verification standards required
/// for various types of interactions and operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRequirements {
    /// Minimum trust score required (0.0-1.0)
    pub minimum_trust_score: f64,
    /// Additional requirements for enhanced trust relationships
    pub enhanced_trust_requirements: Vec<String>,
    /// Methods used to verify and establish trust
    pub trust_verification_methods: Vec<String>,
}

/// Preferences for genetic evolution and adaptation
///
/// Controls how biomes can evolve and adapt over time through
/// genetic algorithms, crossover, and mutation operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEvolutionPreferences {
    /// Whether genetic evolution operations are permitted
    pub allow_evolution: bool,
    /// Preferences for genetic crossover operations
    pub crossover_preferences: CrossoverPreferences,
    /// Criteria used to evaluate fitness for natural selection
    pub fitness_criteria: Vec<FitnessCriterion>,
    /// Preferred mutation rates for genetic variation
    pub mutation_rate_preferences: Vec<String>,
}

/// Preferences for genetic crossover operations
///
/// Controls how genetic traits are combined when creating offspring
/// or mixing genetic material between biomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossoverPreferences {
    /// Whether crossover operations are enabled
    pub enabled: bool,
    /// Rates at which crossover occurs (probability 0.0-1.0)
    pub crossover_rates: Vec<f64>,
    /// Settings for preserving heritage and lineage information
    /// Collection of heritage preservation
    pub heritage_preservation: Vec<String>,
}

/// Single criterion for evaluating genetic fitness
///
/// Defines a metric used to evaluate how well a biome or genetic
/// configuration performs, guiding natural selection in evolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriterion {
    /// Name identifying this fitness criterion
    pub name: String,
    /// Relative importance of this criterion (0.0-1.0)
    pub weight: f64,
    /// Method used to measure this fitness criterion
    /// The measurement method value
    pub measurement_method: String,
}

/// Levels of corporate access to biome resources
///
/// Defines the policy for how corporate entities may interact with
/// the biome, from complete exclusion to various paid access models.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorporateAccessLevel {
    /// No corporate access permitted (fully sovereign)
    None,
    /// Corporate access permitted with direct payment
    PaymentRequired,
    /// Corporate access through pre-established partnerships
    PartnershipBased,
    /// Limited corporate access with specific restrictions
    RestrictedAccess,
    /// Corporate entities can only purchase compute resources
    PaidComputeOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_privacy_protection_settings_default() {
        let settings = PrivacyProtectionSettings::default();
        assert!(settings.uses_zero_knowledge_proofs());
        assert!(settings.uses_data_minimization());
        assert!(!settings.uses_homomorphic_encryption());
        assert!(!settings.uses_secure_multiparty_computation());
        assert!(!settings.uses_anonymous_credentials());
    }

    #[test]
    fn test_privacy_mechanism_variants() {
        let _ = PrivacyMechanism::ZeroKnowledgeProofs;
        let _ = PrivacyMechanism::HomomorphicEncryption;
        let _ = PrivacyMechanism::SecureMultipartyComputation;
    }

    #[test]
    fn test_platform_verification_default() {
        let pv = PlatformVerification::default();
        assert_eq!(pv.platform_name, "Unknown");
        assert_eq!(pv.verified_boot_state, "unverified");
        assert!(!pv.bootloader_locked);
    }

    #[test]
    fn test_sovereignty_level_serialization() {
        let levels = [
            SovereigntyLevel::Minimal,
            SovereigntyLevel::Standard,
            SovereigntyLevel::Enhanced,
            SovereigntyLevel::Maximum,
        ];
        for level in levels {
            let json = serde_json::to_string(&level).unwrap();
            let _: SovereigntyLevel = serde_json::from_str(&json).unwrap();
        }
    }

    #[test]
    fn test_corporate_access_level_variants() {
        assert_eq!(CorporateAccessLevel::None, CorporateAccessLevel::None);
        assert_ne!(
            CorporateAccessLevel::None,
            CorporateAccessLevel::PaymentRequired
        );
    }

    #[test]
    fn test_diversity_config_construction() {
        let config = DiversityConfig {
            min_diversity: 0.5,
            diversity_metric: "shannon".to_string(),
            adaptive_mutation: true,
            immigration_rate: 0.1,
        };
        assert_eq!(config.min_diversity, 0.5);
        assert!(config.adaptive_mutation);
    }

    #[test]
    fn test_partnership_config_construction() {
        let config = PartnershipConfig {
            max_partners: 5,
            min_trust_score: 0.8,
            partnership_duration_secs: 3600,
            auto_discovery: true,
        };
        assert_eq!(config.max_partners, 5);
    }

    #[test]
    fn test_genetic_evolution_config_construction() {
        let config = GeneticEvolutionConfig {
            evolution_speed: 1.0,
            environmental_adaptation: true,
            strength_evolution: true,
        };
        assert_eq!(config.evolution_speed, 1.0);
    }

    #[test]
    fn test_crossover_preferences_construction() {
        let prefs = CrossoverPreferences {
            enabled: true,
            crossover_rates: vec![0.5, 0.7],
            heritage_preservation: vec!["lineage".to_string()],
        };
        assert!(prefs.enabled);
    }

    #[test]
    fn test_fitness_criterion_construction() {
        let criterion = FitnessCriterion {
            name: "security".to_string(),
            weight: 0.5,
            measurement_method: "score".to_string(),
        };
        assert_eq!(criterion.weight, 0.5);
    }
}
