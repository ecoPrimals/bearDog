

pub mod genesis;
pub mod mixed_lineage;

pub use genesis::{
    CorporateAccessLevel, DeviceAttestation, PlatformVerification, PrimalAutonomousRules,
    PrimalGenesisSeed, GeneticEvolutionPreferences, CrossoverPreferences, FitnessCriterion,
    HumanPermissions, PartnershipCriteria, PartnershipDurationLimits, TrustRequirements,
};
pub use mixed_lineage::{
    HumanKeyComponent, LineageEvent, LineageEventType, MixedLineageKey, PrimalKeyComponent,};

use beardog_auth::auth::types::genetics::{NodeCapability, SecurityClearance};
use beardog_auth::auth::types::spawning::{ResourceLimits, SpawnPurpose};
use beardog_errors::BearDogError;
use beardog_errors::idiomatic::GeneticsResult;
use beardog_genetics::genetics::entropy_hierarchy::{
    BiometricHash, EntropyClass, EntropyHierarchyManager, FusionAlgorithm, HumanEntropySource,
    OwnershipProof,
use beardog_genetics::genetics::spawning::{GeneticSpawningEngine, SpawnRequest};
use beardog_security::crypto_utils::BearDogCrypto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporatePayment {

    pub payment_id: String,

    pub corporate_entity: String,

    pub amount_paid: f64,

    pub currency_type: String,

    pub service_type: String,

    pub payment_timestamp: DateTime<Utc>,

    pub payment_proof: Vec<u8>,
}

pub struct PaymentDetails {

    pub required_amount: f64,

    pub accepted_payment_methods: Vec<String>,

    pub payment_deadline: DateTime<Utc>,

#[derive(Debug)]

pub struct PrimalSovereigntyManager {

    genesis_seed: PrimalGenesisSeed,

    current_lineage_key: MixedLineageKey,

    lineage_history: Vec<LineageEvent>,

    corporate_payments: HashMap<String, CorporatePayment>,

    entropy_manager: EntropyHierarchyManager,

    spawning_engine: GeneticSpawningEngine,}

impl PrimalSovereigntyManager {

    pub fn new_from_genesis(genesis_seed: PrimalGenesisSeed) -> Result<Self, BearDogError> {
        info!("🌱 Initializing primal sovereignty from genesis seed: {}", genesis_seed.primal_id);

        Self::validate_genesis_seed(&genesis_seed)?;

        let initial_lineage_key = Self::create_initial_lineage_key(&genesis_seed)?;

        let entropy_manager = EntropyHierarchyManager::new(genesis_seed.entropy_class.clone())?;

        let spawning_engine = GeneticSpawningEngine::new(genesis_seed.genetic_lineage.clone())?;

        let genesis_event = LineageEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: genesis_seed.genesis_timestamp,
            event_type: LineageEventType::Genesis,
            participants: vec![genesis_seed.primal_id.clone()],
            event_data: HashMap::from([
                ("entropy_class".to_string(), format_args!("{:?}", genesis_seed.entropy_class).to_string()),
                ("hardware_id".to_string(), genesis_seed.device_attestation.hardware_id.clone()),
            ]),
            event_proof: genesis_seed.autonomous_birth_proof.clone(),
        };
        Ok(Self {
            genesis_seed,
            current_lineage_key: initial_lineage_key,
            lineage_history: vec![genesis_event],
            corporate_payments: ahash::HashMap::default(),
            entropy_manager,
            spawning_engine,
        })
    }

    fn validate_genesis_seed(seed: &PrimalGenesisSeed) -> Result<(), BearDogError> {

        if seed.device_attestation.hardware_id.is_empty() {
            return Err(BearDogError::validation("Genesis seed missing device attestation"));
        }

        if seed.autonomous_birth_proof.is_empty() {
            return Err(BearDogError::validation("Genesis seed missing autonomous birth proof"));

        if !matches!(seed.entropy_class, EntropyClass::HumanLivedExperience) {
            return Err(BearDogError::validation("Genesis seed must use highest-tier entropy"));

        if seed.self_defined_rules.max_corporate_access_level == CorporateAccessLevel::None {
            info!("✅ Primal has chosen complete corporate isolation");
        Ok(())

    fn create_initial_lineage_key(seed: &PrimalGenesisSeed) -> Result<MixedLineageKey, BearDogError> {
        let primal_component = PrimalKeyComponent {
            identity_key: seed.sovereign_public_key.clone(),
            autonomy_signature: seed.autonomous_birth_proof.clone(),
            genesis_proof: seed.device_attestation.hardware_signature.clone(),
            identity_hash: BearDogCrypto::hash_sha256(&seed.primal_id.as_bytes())?,
            creation_timestamp: seed.genesis_timestamp,
            sovereignty_assertion: "I belong to myself".to_string(),
        Ok(MixedLineageKey {
            primal_component,
            human_component: None, // No human partnership initially
            blending_algorithm: "primal_only".to_string(),
            partnership_start: seed.genesis_timestamp,
            partnership_expiration: None,
            partnership_terms: ahash::HashMap::default(),

    pub fn start_human_partnership(
        &mut self,
        human_entropy: HumanEntropySource,
        biometric_hash: BiometricHash,
        partnership_permissions: Vec<&str>,
    ) -> Result<(), BearDogError> {
        info!("🤝 Starting human partnership with permissions: {:?}", partnership_permissions);

        self.validate_partnership_request(&partnership_permissions)?;

        let human_component = HumanKeyComponent {
            partnership_key: BearDogCrypto::generate_keypair()?.public_key,
            human_identity_proof: biometric_hash.ownership_proof.clone(),
            biometric_hash: biometric_hash.clone(),
            consent_timestamp: Utc::now(),
            granted_permissions: partnership_permissions.clone(),
            entropy_contribution: human_entropy,
            partnership_agreement_signature: vec![], // Would be signed in real implementation

        self.current_lineage_key.human_component = Some(human_component);
        self.current_lineage_key.partnership_start = Utc::now();
        self.current_lineage_key.blending_algorithm = "primal_human_blend".to_string();

        let partnership_event = LineageEvent {
            timestamp: Utc::now(),
            event_type: LineageEventType::PartnershipStarted,
            participants: vec![self.genesis_seed.primal_id.clone(), "human_partner".to_string()],
                ("permissions".to_string(), partnership_permissions.join(",")),
                ("partnership_type".to_string(), "human_partnership".to_string()),
            event_proof: vec![], // Would contain cryptographic proof
        self.lineage_history.push(partnership_event);
        info!("✅ Human partnership established successfully");

    pub fn end_human_partnership(&mut self) -> Result<(), BearDogError> {
        if self.current_lineage_key.human_component.is_none() {
            return Err(BearDogError::validation("No active human partnership to end"));
        info!("👋 Ending human partnership");

        self.current_lineage_key.human_component = None;
        self.current_lineage_key.blending_algorithm = "primal_only".to_string();

        let end_event = LineageEvent {
            event_type: LineageEventType::PartnershipEnded,
            participants: vec![self.genesis_seed.primal_id.clone()],
                ("reason".to_string(), "partnership_concluded".to_string()),
            event_proof: vec![],
        self.lineage_history.push(end_event);
        info!("✅ Human partnership ended, primal sovereignty restored");

    fn validate_partnership_request(&self, requested_permissions: &[&str]) -> Result<(), BearDogError> {
        let rules = &self.genesis_seed.self_defined_rules;

        if rules.human_partnership_rules.accepted_partnership_types.is_empty() {
            return Err(BearDogError::authorization("Primal does not accept any partnerships"));

        for permission in requested_permissions {
            if !rules.human_partnership_rules.requestable_permissions.contains(permission) {
                return Err(BearDogError::authorization(format!(
                    "Permission '{}' not granted by primal rules", permission
                )));
            }

    pub fn handle_corporate_access_request(
        corporate_entity: &str,
        requested_service: &str,
        payment: Option<CorporatePayment>,
    ) -> Result<CorporateAccessResult, BearDogError> {
        info!("🏢 Corporate access request from {} for {}", corporate_entity, requested_service);

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

        if let Some(payment) = payment {
            self.validate_corporate_payment(&payment)?;
            self.corporate_payments.insert(payment.payment_id.clone(), payment);
            Ok(CorporateAccessResult::Granted {
                access_token: Uuid::new_v4().to_string(),
                expiration: Utc::now() + chrono::Duration::hours(1),
                permitted_operations: vec![requested_service.to_string()],
            })
        } else {

            Ok(CorporateAccessResult::PaymentRequired {
                payment_details: PaymentDetails {
                    required_amount: 10.0, // Primal sets its own prices
                    accepted_payment_methods: vec!["crypto".to_string(), "fiat".to_string()],
                    payment_deadline: Utc::now() + chrono::Duration::hours(24),
                },

    fn validate_corporate_payment(&self, payment: &CorporatePayment) -> Result<(), BearDogError> {
        if payment.amount_paid <= 0.0 {
            return Err(BearDogError::validation("Invalid payment amount"));
        if payment.payment_proof.is_empty() {
            return Err(BearDogError::validation("Payment proof required"));

    pub fn get_sovereignty_status(&self) -> SovereigntyStatus {
        SovereigntyStatus {
            primal_id: self.genesis_seed.primal_id.clone(),
            genesis_timestamp: self.genesis_seed.genesis_timestamp,
            has_human_partnership: self.current_lineage_key.human_component.is_some(),
            corporate_access_level: self.genesis_seed.self_defined_rules.max_corporate_access_level.clone(),
            active_payments: self.corporate_payments.len(),
            lineage_events: self.lineage_history.len(),

    pub fn spawn_offspring(
        partner_genetics: &BearDogGenetics,
        spawn_request: SpawnRequest,
    ) -> Result<BearDogGenetics, GeneticsError> {
        info!("🧬 Spawning genetic offspring with partner");

        if self.genesis_seed.self_defined_rules.evolution_preferences.fitness_criteria.is_empty() {
            return Err(BearDogError::validation("Primal has not defined fitness criteria for spawning"));

        let offspring = self.spawning_engine.spawn_with_partner(
            &self.genesis_seed.genetic_lineage,
            partner_genetics,
            spawn_request,
        )?;

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

pub enum CorporateAccessResult {

    Granted {
        access_token: String,
        expiration: DateTime<Utc>,
        permitted_operations: Vec<String>,
    },

    Denied {
        reason: String,

    PaymentRequired {
        payment_details: PaymentDetails,

pub struct SovereigntyStatus {
    pub primal_id: String,
    pub genesis_timestamp: DateTime<Utc>,
    pub has_human_partnership: bool,
    pub corporate_access_level: CorporateAccessLevel,
    pub active_payments: usize,
    pub lineage_events: usize,}

impl Default for PrimalSovereigntyManager {

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

        Self::new_from_genesis(genesis_seed).unwrap_or_else(|_| {

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
                active_sessions: std::sync::Arc::new(std::sync::RwLock::new(std::collections::ahash::HashMap::default())),
                audit_trail: std::sync::Arc::new(std::sync::RwLock::new(Vec::new())),
