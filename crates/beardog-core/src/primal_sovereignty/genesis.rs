

use beardog_genetics::genetics::entropy_hierarchy::EntropyClass;
use beardog_auth::auth::`BearDog`Genetics;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalGenesisSeed {

    pub primal_id: String,

    pub entropy_class: EntropyClass,

    pub genetic_lineage: `BearDog`Genetics,

    pub genesis_timestamp: DateTime<Utc>,

    pub device_attestation: DeviceAttestation,

    pub sovereign_public_key: Vec<u8>,

    pub autonomous_birth_proof: Vec<u8>,

    pub self_defined_rules: PrimalAutonomousRules,
}

pub struct DeviceAttestation {

    pub hardware_id: String,

    pub tee_certificate_chain: Vec<Vec<u8>>,

    pub hardware_signature: Vec<u8>,

    pub platform_verification: PlatformVerification,

pub struct PlatformVerification {

    pub safetynet_token: String,

    pub verified_boot_state: String,

    pub hardware_security_level: String,

    pub bootloader_locked: bool,

pub struct PrimalAutonomousRules {

    pub max_corporate_access_level: CorporateAccessLevel,

    pub human_partnership_rules: HumanPermissions,

    pub evolution_preferences: GeneticEvolutionPreferences,

    pub resource_limits: Vec<String>,

    pub ethical_boundaries: Vec<String>,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorporateAccessLevel {

    None,

    PaidComputeOnly,

    LimitedPartnership,

    StandardBusiness,

pub struct HumanPermissions {

    pub accepted_partnership_types: Vec<String>,

    pub partnership_criteria: PartnershipCriteria,

    pub partnership_duration_limits: PartnershipDurationLimits,

    pub trust_requirements: TrustRequirements,

    pub requestable_permissions: Vec<String>,

pub struct PartnershipCriteria {

    pub minimum_trust_score: f64,

    pub required_verification_level: String,

    pub compatibility_requirements: Vec<String>,

    pub mutual_benefit_requirements: Vec<String>,

pub struct PartnershipDurationLimits {

    pub max_partnership_duration_seconds: u64,

    pub default_partnership_duration_seconds: u64,

    pub renewal_requirements: Vec<String>,

pub struct TrustRequirements {

    pub basic_trust_requirements: Vec<String>,

    pub enhanced_trust_requirements: Vec<String>,

    pub trust_verification_methods: Vec<String>,

pub struct GeneticEvolutionPreferences {

    pub preferred_mixing_strategies: Vec<String>,

    pub crossover_preferences: CrossoverPreferences,

    pub fitness_criteria: Vec<FitnessCriterion>,

    pub mutation_rate_preferences: Vec<String>,

pub struct CrossoverPreferences {

    pub preferred_algorithms: Vec<String>,

    pub crossover_rates: Vec<f64>,

    pub heritage_preservation: Vec<String>,

pub struct FitnessCriterion {

    pub criterion_name: String,

    pub weight: f64,

    pub measurement_method: String,
} 
