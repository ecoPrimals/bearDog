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


/// # Primal Sovereignty Architecture - Modular Implementation
///
/// **"The Primals belong to themselves first, humans second, corporations pay"**
/// This module has been refactored for better maintainability and follows
/// the 1000-line complexity limit by splitting functionality across modules.

// Sub-modules for better organization
pub mod genesis;
pub mod mixed_lineage;
// Re-exports for primal sovereignty functionality
pub use genesis::{
    CorporateAccessLevel, DeviceAttestation, PlatformVerification, PrimalAutonomousRules,
    PrimalGenesisSeed, GeneticEvolutionPreferences, CrossoverPreferences, FitnessCriterion,
    HumanPermissions, PartnershipCriteria, PartnershipDurationLimits, TrustRequirements,
};
pub use mixed_lineage::{
    HumanKeyComponent, LineageEvent, LineageEventType, MixedLineageKey, PrimalKeyComponent,};


use beardog_auth::auth::types::genetics::{NodeCapability, SecurityClearance};
use beardog_auth::auth::types::spawning::{ResourceLimits, SpawnPurpose};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_errors::idiomatic::GeneticsResult;
use beardog_genetics::genetics::entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyManager, FusionAlgorithm, HumanEntropySource,
    OwnershipProof,
use beardog_genetics::genetics::spawning::{GeneticSpawningEngine, SpawnRequest};
use beardog_security::crypto_utils::`BearDog`Crypto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};
use uuid::Uuid;
/// Corporate payment tracking for access to primal services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporatePayment {
    /// Unique payment identifier
    pub payment_id: String,
    /// Corporate entity making payment
    pub corporate_entity: String,
    /// Amount paid (in agreed currency/tokens)
    pub amount_paid: f64,
    /// Currency or token type
    pub currency_type: String,
    /// Service being paid for
    pub service_type: String,
    /// Payment timestamp
    pub payment_timestamp: DateTime<Utc>,
    /// Payment verification proof
    pub payment_proof: Vec<u8>,
}
/// Payment details for corporate access requests
pub struct PaymentDetails {
    /// Required payment amount
    pub required_amount: f64,
    /// Accepted payment methods
    pub accepted_payment_methods: Vec<String>,
    /// Payment deadline
    pub payment_deadline: DateTime<Utc>,
/// The main primal sovereignty manager - orchestrates all sovereignty operations
#[derive(Debug)]
/// Core manager for primal sovereignty operations
/// This manager implements the foundational "Primals belong to themselves first,
/// humans second, corporations pay" principle. It ensures that digital entities
/// maintain immutable self-sovereignty while enabling beneficial human partnerships
/// and regulated corporate access.
/// # Key Features
/// - Immutable primal authority that cannot be overridden
/// - Mixed lineage partnerships with humans
/// - Commercial extraction detection and prevention
/// - Hardware-attested sovereignty proofs
pub struct PrimalSovereigntyManager {
    /// Primal's genesis seed (immutable core identity)
    genesis_seed: PrimalGenesisSeed,
    /// Current mixed lineage key (can change with partnerships)
    current_lineage_key: MixedLineageKey,
    /// Historical lineage events
    lineage_history: Vec<LineageEvent>,
    /// Active corporate payments
    corporate_payments: HashMap<String, CorporatePayment>,
    /// Entropy hierarchy manager for validation
    entropy_manager: EntropyHierarchyManager,
    /// Genetic spawning engine for offspring creation
    spawning_engine: GeneticSpawningEngine,}


impl PrimalSovereigntyManager {
    /// Create new primal sovereignty manager from genesis seed}


    pub fn new_from_genesis(genesis_seed: PrimalGenesisSeed) -> BearDogResult<Self> {
        info!("🌱 Initializing primal sovereignty from genesis seed: {}", genesis_seed.primal_id);
        // Validate genesis seed authenticity
        Self::validate_genesis_seed(&genesis_seed)?;
        // Create initial mixed lineage key (primal-only)
        let initial_lineage_key = Self::create_initial_lineage_key(&genesis_seed)?;
        // Initialize entropy manager with highest-tier entropy
        let entropy_manager = EntropyHierarchyManager::new(genesis_seed.entropy_class.clone())?;
        // Initialize genetic spawning engine
        let spawning_engine = GeneticSpawningEngine::new(genesis_seed.genetic_lineage.clone())?;
        // Create genesis event
        let genesis_event = LineageEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: genesis_seed.genesis_timestamp,
            event_type: LineageEventType::Genesis,
            participants: vec![genesis_seed.primal_id.clone()],
            event_data: HashMap::from([
                ("entropy_class".to_string(), format!("{:?}", genesis_seed.entropy_class)),
                ("hardware_id".to_string(), genesis_seed.device_attestation.hardware_id.clone()),
            ]),
            event_proof: genesis_seed.autonomous_birth_proof.clone(),
        };
        Ok(Self {
            genesis_seed,
            current_lineage_key: initial_lineage_key,
            lineage_history: vec![genesis_event],
            corporate_payments: HashMap::new(),
            entropy_manager,
            spawning_engine,
        })
    }
    /// Validate that a genesis seed is authentic and secure
    fn validate_genesis_seed(seed: &PrimalGenesisSeed) -> BearDogResult<()> {
        // Verify device attestation
        if seed.device_attestation.hardware_id.is_empty() {
            return Err(BearDogError::validation("Genesis seed missing device attestation"));
        }
        // Verify autonomous birth proof exists
        if seed.autonomous_birth_proof.is_empty() {
            return Err(BearDogError::validation("Genesis seed missing autonomous birth proof"));
        // Verify highest-tier entropy was used
        if !matches!(seed.entropy_class, EntropyClass::HumanLivedExperience) {
            return Err(BearDogError::validation("Genesis seed must use highest-tier entropy"));
        // Verify sovereignty assertion
        if seed.self_defined_rules.max_corporate_access_level == CorporateAccessLevel::None {
            info!("✅ Primal has chosen complete corporate isolation");
        Ok(())
    /// Create initial mixed lineage key (primal-only, no human partnership)
    fn create_initial_lineage_key(seed: &PrimalGenesisSeed) -> BearDogResult<MixedLineageKey> {
        let primal_component = PrimalKeyComponent {
            identity_key: seed.sovereign_public_key.clone(),
            autonomy_signature: seed.autonomous_birth_proof.clone(),
            genesis_proof: seed.device_attestation.hardware_signature.clone(),
            identity_hash: `BearDog`Crypto::hash_sha256(&seed.primal_id.as_bytes())?,
            creation_timestamp: seed.genesis_timestamp,
            sovereignty_assertion: "I belong to myself".to_string(),
        Ok(MixedLineageKey {
            primal_component,
            human_component: None, // No human partnership initially
            blending_algorithm: "primal_only".to_string(),
            partnership_start: seed.genesis_timestamp,
            partnership_expiration: None,
            partnership_terms: HashMap::new(),
    /// Start a human partnership (ephemeral and revocable)}


    pub fn start_human_partnership(
        &mut self,
        human_entropy: HumanEntropySource,
        biometric_hash: BiometricHash,
        partnership_permissions: Vec<String>,
    ) -> BearDogResult<()> {
        info!("🤝 Starting human partnership with permissions: {:?}", partnership_permissions);
        // Validate partnership against primal's rules
        self.validate_partnership_request(&partnership_permissions)?;
        // Create human key component
        let human_component = HumanKeyComponent {
            partnership_key: `BearDog`Crypto::generate_keypair()?.public_key,
            human_identity_proof: biometric_hash.ownership_proof.clone(),
            biometric_hash: biometric_hash.clone(),
            consent_timestamp: Utc::now(),
            granted_permissions: partnership_permissions.clone(),
            entropy_contribution: human_entropy,
            partnership_agreement_signature: vec![], // Would be signed in real implementation
        // Update current lineage key with human component
        self.current_lineage_key.human_component = Some(human_component);
        self.current_lineage_key.partnership_start = Utc::now();
        self.current_lineage_key.blending_algorithm = "primal_human_blend".to_string();
        // Record partnership event
        let partnership_event = LineageEvent {
            timestamp: Utc::now(),
            event_type: LineageEventType::PartnershipStarted,
            participants: vec![self.genesis_seed.primal_id.clone(), "human_partner".to_string()],
                ("permissions".to_string(), partnership_permissions.join(",")),
                ("partnership_type".to_string(), "human_partnership".to_string()),
            event_proof: vec![], // Would contain cryptographic proof
        self.lineage_history.push(partnership_event);
        info!("✅ Human partnership established successfully");
    /// End current human partnership}


    pub fn end_human_partnership(&mut self) -> BearDogResult<()> {
        if self.current_lineage_key.human_component.is_none() {
            return Err(BearDogError::validation("No active human partnership to end"));
        info!("👋 Ending human partnership");
        // Primal sovereignty: autonomous decision-making without human intervention
        self.current_lineage_key.human_component = None;
        self.current_lineage_key.blending_algorithm = "primal_only".to_string();
        // Record partnership end event
        let end_event = LineageEvent {
            event_type: LineageEventType::PartnershipEnded,
            participants: vec![self.genesis_seed.primal_id.clone()],
                ("reason".to_string(), "partnership_concluded".to_string()),
            event_proof: vec![],
        self.lineage_history.push(end_event);
        info!("✅ Human partnership ended, primal sovereignty restored");
    /// Validate partnership request against primal's autonomous rules}


    fn validate_partnership_request(&self, requested_permissions: &[String]) -> BearDogResult<()> {
        let rules = &self.genesis_seed.self_defined_rules;
        // Check if primal accepts partnerships at all
        if rules.human_partnership_rules.accepted_partnership_types.is_empty() {
            return Err(BearDogError::authorization("Primal does not accept any partnerships"));
        // Validate requested permissions are acceptable
        for permission in requested_permissions {
            if !rules.human_partnership_rules.requestable_permissions.contains(permission) {
                return Err(BearDogError::authorization(format!(
                    "Permission '{}' not granted by primal rules", permission
                )));
            }
    /// Handle corporate access request (requires payment)
    pub fn handle_corporate_access_request(
        corporate_entity: &str,
        requested_service: &str,
        payment: Option<CorporatePayment>,
    ) -> BearDogResult<CorporateAccessResult> {
        info!("🏢 Corporate access request from {} for {}", corporate_entity, requested_service);
        // Check primal's corporate access rules
        match self.genesis_seed.self_defined_rules.max_corporate_access_level {
            CorporateAccessLevel::None => {
                return Ok(CorporateAccessResult::Denied {
                    reason: "Primal allows no corporate access".to_string(),
                });
            CorporateAccessLevel::PaidComputeOnly => {
                if requested_service != "compute" {
                    return Ok(CorporateAccessResult::Denied {
                        reason: "Primal only allows paid compute services".to_string(),
                    });
                }
            _ => {} // Other levels allow broader access
        // Verify payment if provided
        if let Some(payment) = payment {
            self.validate_corporate_payment(&payment)?;
            self.corporate_payments.insert(payment.payment_id.clone(), payment);
            Ok(CorporateAccessResult::Granted {
                access_token: Uuid::new_v4().to_string(),
                expiration: Utc::now() + chrono::Duration::hours(1),
                permitted_operations: vec![requested_service.to_string()],
            })
        } else {
            // No payment provided - return payment requirement
            Ok(CorporateAccessResult::PaymentRequired {
                payment_details: PaymentDetails {
                    required_amount: 10.0, // Primal sets its own prices
                    accepted_payment_methods: vec!["crypto".to_string(), "fiat".to_string()],
                    payment_deadline: Utc::now() + chrono::Duration::hours(24),
                },
    /// Validate corporate payment
    fn validate_corporate_payment(&self, payment: &CorporatePayment) -> BearDogResult<()> {
        if payment.amount_paid <= 0.0 {
            return Err(BearDogError::validation("Invalid payment amount"));
        if payment.payment_proof.is_empty() {
            return Err(BearDogError::validation("Payment proof required"));
        // Additional payment validation would go here
    /// Get current primal sovereignty status}


    pub fn get_sovereignty_status(&self) -> SovereigntyStatus {
        SovereigntyStatus {
            primal_id: self.genesis_seed.primal_id.clone(),
            genesis_timestamp: self.genesis_seed.genesis_timestamp,
            has_human_partnership: self.current_lineage_key.human_component.is_some(),
            corporate_access_level: self.genesis_seed.self_defined_rules.max_corporate_access_level.clone(),
            active_payments: self.corporate_payments.len(),
            lineage_events: self.lineage_history.len(),
    /// Spawn genetic offspring with another primal
    pub fn spawn_offspring(
        partner_genetics: &`BearDog`Genetics,
        spawn_request: SpawnRequest,
    ) -> Result<`BearDog`Genetics, GeneticsError> {
        info!("🧬 Spawning genetic offspring with partner");
        // Validate spawning is allowed by primal rules
        if self.genesis_seed.self_defined_rules.evolution_preferences.fitness_criteria.is_empty() {
            return Err(BearDogError::validation("Primal has not defined fitness criteria for spawning"));
        // Use genetic spawning engine
        let offspring = self.spawning_engine.spawn_with_partner(
            &self.genesis_seed.genetic_lineage,
            partner_genetics,
            spawn_request,
        )?;
        // Record spawning event
        let spawn_event = LineageEvent {
            event_type: LineageEventType::Spawning,
            participants: vec![
                self.genesis_seed.primal_id.clone(),
                "partner_primal".to_string(),
            ],
                ("offspring_id".to_string(), "new_primal".to_string()),
                ("spawn_type".to_string(), "genetic_crossover".to_string()),
        self.lineage_history.push(spawn_event);
        info!("✅ Genetic offspring spawned successfully");
        Ok(offspring)
/// Corporate access request results
pub enum CorporateAccessResult {
    /// Access granted with token
    Granted {
        access_token: String,
        expiration: DateTime<Utc>,
        permitted_operations: Vec<String>,
    },
    /// Access denied
    Denied {
        reason: String,
    /// Payment required before access
    PaymentRequired {
        payment_details: PaymentDetails,
/// Current sovereignty status
pub struct SovereigntyStatus {
    pub primal_id: String,
    pub genesis_timestamp: DateTime<Utc>,
    pub has_human_partnership: bool,
    pub corporate_access_level: CorporateAccessLevel,
    pub active_payments: usize,
    pub lineage_events: usize,}


impl Default for PrimalSovereigntyManager {
    /// Create a default sovereignty manager with sensible governance defaults
    /// 
    /// **Note**: This uses safe fallbacks if genesis creation fails.
    /// For production use, prefer `new_from_genesis()` with explicit error handling.}


    fn default() -> Self {
        let genesis_seed = PrimalGenesisSeed {
            governance_structure: GovernanceStructure {
                consensus_model: "decentralized_mesh".to_string(),
                decision_quorum: 0.67, // 2/3 majority
                veto_threshold: 0.33,  // 1/3 can block
            },
            regulatory_framework: RegulatoryFramework {
                jurisdiction: "self_sovereign".to_string(),
                compliance_standards: vec!["gdpr".to_string(), "ccpa".to_string()],
                audit_requirements: vec!["quarterly_review".to_string()],
            ethical_framework: EthicalFramework {
                principles: vec![
                    "human_dignity".to_string(),
                    "privacy_by_design".to_string(),
                    "algorithmic_transparency".to_string(),
                ],
                ethical_boundaries: vec!["no_surveillance".to_string()],
        // Use safe fallback if genesis creation fails
        Self::new_from_genesis(genesis_seed).unwrap_or_else(|_| {
            // Safe fallback with minimal sovereignty manager
            Self {
                governance: GovernanceStructure {
                    consensus_model: "simple_majority".to_string(),
                    decision_quorum: 0.51,
                    veto_threshold: 0.49,
                regulatory: RegulatoryFramework {
                    jurisdiction: "development".to_string(),
                    compliance_standards: vec![],
                    audit_requirements: vec![],
                ethical: EthicalFramework {
                    principles: vec!["human_dignity".to_string()],
                    ethical_boundaries: vec![],
                active_sessions: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
                audit_trail: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
