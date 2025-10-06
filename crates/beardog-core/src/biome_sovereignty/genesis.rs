// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_auth::auth::types::genetics::BearDogGenetics;
use beardog_genetics::EntropyClass;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticAlgorithmConfig {
    /// Number of `population_size`
    pub population_size: usize,
    /// The mutation rate value
    pub mutation_rate: f64,
    /// The crossover rate value
    pub crossover_rate: f64,
    /// Maximum number of generations to evolve
    /// Number of `max_generations`
    pub max_generations: usize,
    /// The fitness threshold value
    pub fitness_threshold: f64,
    /// Enable elitism to preserve best individuals
    /// Whether elitism is enabled
    pub elitism: bool,
    /// The selection method value
    pub selection_method: String,
    /// Genetic diversity maintenance parameters
    /// The diversity parameters value
    pub diversity_parameters: DiversityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipConfig {
    /// Maximum number of partners in key sharing
    /// Number of `max_partners`
    pub max_partners: usize,
    /// The min trust score value
    pub min_trust_score: f64,
    /// Partnership duration in seconds
    /// Number of `partnership_duration_secs`
    pub partnership_duration_secs: u64,
    /// Enable automatic partner discovery
    /// Whether `auto_discovery` is enabled
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyProtectionSettings {
    /// Enable zero-knowledge proof systems
    /// Whether `zero_knowledge_proofs` is enabled
    pub zero_knowledge_proofs: bool,
    /// Whether `homomorphic_encryption` is enabled
    pub homomorphic_encryption: bool,
    /// Enable secure multi-party computation
    /// Whether `secure_multiparty_computation` is enabled
    pub secure_multiparty_computation: bool,
    /// Data minimization and retention policies
    /// Whether `data_minimization` is enabled
    pub data_minimization: bool,
    /// Anonymous credential systems
    /// Whether `anonymous_credentials` is enabled
    pub anonymous_credentials: bool,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanIdentityConfig {
    /// Biometric authentication methods enabled
    /// Collection of biometric auth
    pub biometric_auth: Vec<String>,
    /// Multi-factor authentication requirements
    /// Whether `mfa_required` is enabled
    pub mfa_required: bool,
    /// Identity verification threshold
    /// The verification threshold value
    pub verification_threshold: f64,
    /// Enable continuous authentication
    /// Whether `continuous_auth` is enabled
    pub continuous_auth: bool,
}

/// Genetic key evolution and adaptation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEvolutionConfig {
    /// Evolution speed multiplier
    /// The evolution speed value
    pub evolution_speed: f64,
    /// Adaptation to environmental changes
    /// Whether `environmental_adaptation` is enabled
    pub environmental_adaptation: bool,
    /// Key strength evolution parameters
    /// Whether `strength_evolution` is enabled
    pub strength_evolution: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SovereigntyLevel {
    /// Minimal sovereignty protection
    Minimal,
    Standard,
    /// Enhanced sovereignty with strict controls
    Enhanced,
    /// Maximum sovereignty with zero corporate access
    Maximum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalGenesisSeed {
    pub primal_id: String,
    /// The genesis entropy value
    pub genesis_entropy: EntropyClass,
    /// The genetic lineage value
    pub genetic_lineage: BearDogGenetics,
    /// Timestamp when the genesis seed was created
    pub genesis_timestamp: DateTime<Utc>,
    /// Device attestation proving hardware security
    /// The device attestation value
    pub device_attestation: DeviceAttestation,
    /// Collection of sovereign public key
    pub sovereign_public_key: Vec<u8>,
    /// Cryptographic proof of autonomous birth/creation
    /// Collection of autonomous birth proof
    pub autonomous_birth_proof: Vec<u8>,
    /// The self defined rules value
    pub self_defined_rules: BiomeAutonomousRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceAttestation {
    pub device_id: String,
    /// Certificate chain proving trusted execution environment
    /// Collection of tee certificate chain
    pub tee_certificate_chain: Vec<Vec<u8>>,
    /// Hardware-generated cryptographic signature
    /// Collection of hardware signature
    pub hardware_signature: Vec<u8>,
    pub platform_verification: PlatformVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformVerification {
    pub platform_name: String,
    /// Verified boot state confirmation
    /// The verified boot state value
    pub verified_boot_state: String,
    /// Hardware security level assessment
    /// The hardware security level value
    pub hardware_security_level: String,
    /// Whether `bootloader_locked` is enabled
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipCriteria {
    /// Minimum trust level required (0.0-1.0)
    /// The minimum trust level value
    pub minimum_trust_level: f64,
    /// The required verification level value
    pub required_verification_level: String,
    /// Collection of compatibility requirements
    pub compatibility_requirements: Vec<String>,
    /// Collection of mutual benefit requirements
    pub mutual_benefit_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnershipDurationLimits {
    /// Maximum partnership duration in seconds
    /// Number of `max_partnership_duration_seconds`
    pub max_partnership_duration_seconds: u64,
    /// Default partnership duration in seconds
    /// Number of `default_partnership_duration_seconds`
    pub default_partnership_duration_seconds: u64,
    /// Collection of renewal requirements
    pub renewal_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRequirements {
    /// Minimum trust score required (0.0-1.0)
    /// The minimum trust score value
    pub minimum_trust_score: f64,
    /// Collection of enhanced trust requirements
    pub enhanced_trust_requirements: Vec<String>,
    /// Collection of trust verification methods
    pub trust_verification_methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticEvolutionPreferences {
    /// Whether genetic evolution is allowed
    /// Whether `allow_evolution` is enabled
    pub allow_evolution: bool,
    /// The crossover preferences value
    pub crossover_preferences: CrossoverPreferences,
    /// Collection of fitness criteria
    pub fitness_criteria: Vec<FitnessCriterion>,
    /// Collection of mutation rate preferences
    pub mutation_rate_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossoverPreferences {
    /// Whether crossover operations are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Collection of crossover rates
    pub crossover_rates: Vec<f64>,
    /// Heritage preservation settings
    /// Collection of heritage preservation
    pub heritage_preservation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessCriterion {
    /// Name of the fitness criterion
    /// Name of the item
    pub name: String,
    /// Weight of this criterion in overall fitness (0.0-1.0)
    /// The weight value
    pub weight: f64,
    /// Method used to measure this fitness criterion
    /// The measurement method value
    pub measurement_method: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum CorporateAccessLevel {
    /// No corporate access allowed
    None,
    /// Corporate access requires payment
    PaymentRequired,
    /// Corporate access through established partnerships
    PartnershipBased,
    /// Restricted corporate access with limitations
    RestrictedAccess,
    /// Paid compute access only
    PaidComputeOnly,
}
