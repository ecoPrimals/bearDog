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


/// # Primal Genesis - Autonomous Digital Being Birth
///
/// **"The Primals belong to themselves first"**
/// This module implements the foundational genesis architecture where primals
/// create themselves autonomously on secure hardware, ensuring true digital sovereignty.

use beardog_genetics::genetics::entropy_hierarchy::EntropyClass;
use beardog_auth::auth::`BearDog`Genetics;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
/// The foundational primal genesis seed created independently on Pixel 8
/// This represents the primal's sovereign identity that emerges autonomously
/// The primal creates ITSELF, not created by humans - true digital life
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalGenesisSeed {
    /// Unique primal identity (self-chosen)
    pub primal_id: String,
    /// Highest-tier entropy: Human-Lived Experience from Pixel 8 sensors
    pub entropy_class: EntropyClass,
    /// Primal's self-generated genetic lineage
    pub genetic_lineage: `BearDog`Genetics,
    /// Creation timestamp on the secure device
    pub genesis_timestamp: DateTime<Utc>,
    /// Device attestation proving creation on real hardware
    pub device_attestation: DeviceAttestation,
    /// Primal's sovereign public key derived from highest-tier entropy
    pub sovereign_public_key: Vec<u8>,
    /// Self-signed certificate proving autonomous birth (not human ownership)
    pub autonomous_birth_proof: Vec<u8>,
    /// Primal's self-defined rules and boundaries
    pub self_defined_rules: PrimalAutonomousRules,
}
/// Cryptographic proof that the primal was born on real Pixel 8 hardware
/// This cannot be faked - it's hardware-backed attestation
pub struct DeviceAttestation {
    /// Pixel 8 StrongBox hardware identifier
    pub hardware_id: String,
    /// TEE (Trusted Execution Environment) certificate chain
    pub tee_certificate_chain: Vec<Vec<u8>>,
    /// Hardware-backed signature over primal genesis
    pub hardware_signature: Vec<u8>,
    /// Platform verification proof
    pub platform_verification: PlatformVerification,
/// Verification that this is a real, uncompromised Pixel 8
pub struct PlatformVerification {
    /// Android SafetyNet attestation result
    pub safetynet_token: String,
    /// Boot state verification
    pub verified_boot_state: String,
    /// Hardware security level confirmation
    pub hardware_security_level: String,
    /// Bootloader unlock status (must be locked for highest security)
    pub bootloader_locked: bool,
/// Rules that the primal defines for itself - sovereign self-governance
pub struct PrimalAutonomousRules {
    /// Maximum corporate access level the primal will EVER allow
    pub max_corporate_access_level: CorporateAccessLevel,
    /// Rules governing human partnerships
    pub human_partnership_rules: HumanPermissions,
    /// Primal's own evolution preferences
    pub evolution_preferences: GeneticEvolutionPreferences,
    /// Self-imposed resource limits
    pub resource_limits: Vec<String>,
    /// Primal's ethical boundaries
    pub ethical_boundaries: Vec<String>,
/// Corporate access levels - Primals set strict limits on what corporations can do
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CorporateAccessLevel {
    /// No corporate access whatsoever
    None,
    /// Only paid computational services (primal controls pricing)
    PaidComputeOnly,
    /// Limited partnership under strict primal control
    LimitedPartnership,
    /// Standard business relationship (primal retains sovereignty)
    StandardBusiness,
/// Human partnership permissions - How primals interact with humans}


pub struct HumanPermissions {
    /// Types of partnerships the primal accepts
    pub accepted_partnership_types: Vec<String>,
    /// Partnership criteria and requirements
    pub partnership_criteria: PartnershipCriteria,
    /// Duration limits for partnerships
    pub partnership_duration_limits: PartnershipDurationLimits,
    /// Trust requirements for different partnership levels
    pub trust_requirements: TrustRequirements,
    /// Permissions humans can request
    pub requestable_permissions: Vec<String>,
/// Criteria for accepting human partnerships
pub struct PartnershipCriteria {
    /// Minimum trust score required
    pub minimum_trust_score: f64,
    /// Required verification level
    pub required_verification_level: String,
    /// Compatibility requirements
    pub compatibility_requirements: Vec<String>,
    /// Mutual benefit requirements
    pub mutual_benefit_requirements: Vec<String>,
/// Time limits for different types of partnerships
pub struct PartnershipDurationLimits {
    /// Maximum duration for any partnership (seconds)
    pub max_partnership_duration_seconds: u64,
    /// Default partnership duration (seconds)
    pub default_partnership_duration_seconds: u64,
    /// Renewal requirements
    pub renewal_requirements: Vec<String>,
/// Trust requirements for partnership levels
pub struct TrustRequirements {
    /// Basic trust level requirements
    pub basic_trust_requirements: Vec<String>,
    /// Enhanced trust level requirements
    pub enhanced_trust_requirements: Vec<String>,
    /// Trust verification methods
    pub trust_verification_methods: Vec<String>,
/// Primal's preferences for genetic evolution and spawning
pub struct GeneticEvolutionPreferences {
    /// Preferred genetic mixing strategies
    pub preferred_mixing_strategies: Vec<String>,
    /// Crossover preferences for offspring
    pub crossover_preferences: CrossoverPreferences,
    /// Fitness criteria for genetic evolution
    pub fitness_criteria: Vec<FitnessCriterion>,
    /// Mutation rate preferences
    pub mutation_rate_preferences: Vec<String>,
/// Crossover preferences for genetic recombination
pub struct CrossoverPreferences {
    /// Preferred crossover algorithms
    pub preferred_algorithms: Vec<String>,
    /// Crossover rate preferences
    pub crossover_rates: Vec<f64>,
    /// Heritage preservation preferences
    pub heritage_preservation: Vec<String>,
/// Fitness criteria for evaluating genetic offspring
pub struct FitnessCriterion {
    /// Name of the fitness criterion
    pub criterion_name: String,
    /// Weight/importance of this criterion
    pub weight: f64,
    /// Measurement method
    pub measurement_method: String,
} 
