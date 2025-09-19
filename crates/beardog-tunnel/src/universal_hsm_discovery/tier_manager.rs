

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::*;
use crate::tunnel::hsm::types::{
    capability::HsmCapabilities, status::HsmHealthStatus, tier::HsmTier,
};
use beardog_errors::BearDogError;
use beardog_types::SecurityLevel as TamperResistanceLevel;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
    operation_requirements: HashMap<String, OperationRequirements>,
    tier_weights: HashMap<HsmTier, f64>,
}
impl TierManager {}

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        let mut operation_requirements = HashMap::with_capacity(HsmTier::CertifiedHardware,
                require_human_entropy: true,
                require_attestation: true,
                require_fips: true,
                max_latency_ms: 1000.0,
                min_ops_per_second: 100,
                required_algorithms: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                compliance_requirements: vec!["FIPS 140-2".to_string(),
                required_algorithms: vec!["RSA".to_string(), "ECDSA".to_string()],
                compliance_requirements: vec![
                    "FIPS 140-2".to_string(),
                required_algorithms: vec!["HMAC".to_string(), "AES".to_string()],

        let mut tier_weights = HashMap::with_capacity(16);
        tier_weights.insert(HsmTier::Software, 1.0);
        tier_weights.insert(HsmTier::BasicHardware, 2.0);
        tier_weights.insert(HsmTier::CertifiedHardware, 3.0);
        tier_weights.insert(HsmTier::HighSecurity, 4.0);
        tier_weights.insert(HsmTier::HumanEntropyPremium, 5.0);
        Ok(Self {
            tier_policies: TierPolicies::default(),
            operation_requirements,
            tier_weights,
        })
    }

/// Assign Tier operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn assign_tier(&self, capabilities: &HsmCapabilities) -> Result<HsmTier, BearDogError> {
        debug!("📊 Assigning tier based on HSM capabilities");
        let mut tier_score = 0.0;

        tier_score += self.evaluate_hardware_security(+{} tier score",
                self.tier_policies.human_entropy_bonus
            );
        }

        let base_tier = self.score_to_tier(tier_score)?;

        let final_tier = if capabilities.human_entropy.supports_ephemeral_seeds {
            self.apply_human_entropy_elevation(base_tier)?
        } else {
            base_tier
        };
        info!(
            "✅ Assigned tier: {:?} (score: {:.2})",
            final_tier, tier_score
        Ok(&'a HashMap<String, DiscoveredHsm>,
        operation_type: &str,
    ) -> Result<Option<&'a DiscoveredHsm>, BearDogError>> {
        debug!("🎯 Selecting best HSM for operation: {}", operation_type);
        let requirements = match self.operation_requirements.get({}", operation_type);
                return Ok(None);
            }
        let mut candidates = Vec::new({}",
                operation_type
            return Ok(None);

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let best_hsm = candidates[0].0;
            "🏆 Selected HSM: {} (score: {:.2})",
            best_hsm.hsm_id, candidates[0].1
        Ok(Option<&str>,
    ) -> Result<Vec<(&'a DiscoveredHsm, f64)>> {
        debug!("📈 Ranking HSMs by tier and suitability");
        let mut ranked = Vec::new();
            let mut score = self
                .tier_weights
                .get(&hsm.assigned_tier)
                .unwrap_or(&1.0)
                .clone();

            if hsm.health_status.healthy {

            } else if hsm.health_status.error_message.is_some() {
                score *= 0.5; // Degraded/failed status
            } else {
                score *= 0.7; // Unknown status

            if hsm.supports_human_entropy {
                score *= 1.2; // 20% bonus
            ranked.push((hsm, score));
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(ranked)

/// Update Tier Policies operation.
    /// Updates tier_policies
    /// Updates tier_policies
    pub fn update_tier_policies(&mut self, policies: TierPolicies) {
        self.tier_policies = policies;

/// Add Operation Requirements operation.
    pub fn add_operation_requirements(&str,
        requirements: OperationRequirements,
    ) {
        self.operation_requirements
            .insert(&HsmCapabilities,
    ) -> Result<f64, BearDogError> {
        let mut score = 0.0;

        match capabilities.security.fips_140_level {
            1 => score += 1.0,
            2 => score += 2.0,
            3 => score += 3.5,
            4 => score += 5.0,
            _ => {} // No FIPS certification

        if capabilities.security.common_criteria_level.is_some() {
            score += 1.0;

        match capabilities.security.tamper_resistance {
            TamperResistanceLevel::Software => {}
            TamperResistanceLevel::Medium => score += 0.5,
            TamperResistanceLevel::Tee => score += 1.0,
            TamperResistanceLevel::Hardware => score += 1.5,
            TamperResistanceLevel::CertifiedHardware => score += 2.0,
            TamperResistanceLevel::MaximumSecurity => score += 2.5,

        if capabilities.security.secure_key_storage {

        if capabilities.security.side_channel_resistance {
            score += 0.5;
        Ok(score)
    fn evaluate_security_certifications(
        for cert in &capabilities.security.security_certifications {
            match cert.as_str() {
                "FIPS 140-2 Level 3" | "FIPS 140-2 Level 4" => score += 2.0,
                "FIPS 140-2 Level 2" => score += 1.5,
                "FIPS 140-2 Level 1" => score += 1.0,
                "Common Criteria EAL4+" => score += 1.5,
                "Common Criteria EAL4" => score += 1.0,
                _ => score += 0.5,}


    fn evaluate_crypto_capabilities(

        if capabilities.key_generation.can_generate_in_hardware {

        if capabilities.key_management.supports_key_attestation {

        let advanced_algorithms = ["Ed25519", "RSA-PSS", "ECDSA", "X25519"];
        for algo in &capabilities.key_generation.supported_algorithms {
            if advanced_algorithms.contains(&algo.as_str()) {
                score += 0.1;

        if capabilities.key_generation.supports_key_wrapping {
            score += 0.3;
        if capabilities.key_generation.supports_key_derivation {
    fn evaluate_performance_factor(

        if capabilities.crypto_operations.hardware_acceleration {

        if capabilities.performance.concurrent_operations > 1 {
            score += 0.2;

        if capabilities.crypto_operations.supports_batch_operations {}


    fn evaluate_compliance_bonus(
        if capabilities.compliance.fips_140_certified {
        if capabilities.compliance.common_criteria_certified {
        if capabilities.compliance.pci_dss_compliant {
        if capabilities.compliance.hipaa_compliant {
        if capabilities.compliance.gdpr_compliant {
    fn score_to_tier(&self, score: f64) -> Result<HsmTier, BearDogError> {
        match score {
            s if s >= 8.0 => Ok(HsmTier::HighSecurity),
            s if s >= 5.0 => Ok(HsmTier::CertifiedHardware),
            s if s >= 2.0 => Ok(HsmTier::BasicHardware),
            _ => Ok(HsmTier::Software),}


    fn apply_human_entropy_elevation(&self, base_tier: HsmTier) -> Result<HsmTier, BearDogError> {

        match base_tier {
            HsmTier::Software => Ok(HsmTier::BasicHardware),
            HsmTier::BasicHardware => Ok(HsmTier::CertifiedHardware),
            HsmTier::CertifiedHardware => Ok(HsmTier::HighSecurity),
            HsmTier::HighSecurity => Ok(HsmTier::HumanEntropyPremium),
            HsmTier::HumanEntropyPremium => Ok(HsmTier::HumanEntropyPremium), // Already at top

            HsmTier::SmartphoneHsm { .. } => Ok(HsmTier::BasicHardware),
            HsmTier::SoftwareHsm { .. } => Ok(HsmTier::CertifiedHardware),
            HsmTier::HardwareHsm { .. } => Ok(HsmTier::HighSecurity),
            HsmTier::HybridHsm { .. } => Ok(HsmTier::HumanEntropyPremium),
    fn meets_operation_requirements(&DiscoveredHsm,
        requirements: &OperationRequirements,
    ) -> Result<bool, BearDogError> {

        if hsm.assigned_tier < requirements.min_tier {
            return Ok(f64,
    /// The mobile hsm bonus value
    pub mobile_hsm_bonus: f64,
    /// The cloud hsm penalty value
    pub cloud_hsm_penalty: f64,
    /// The certification weight value
    pub certification_weight: f64,
    pub performance_weight: f64,
    /// Whether auto_elevation is enabled
    pub auto_elevation_enabled: bool,}

impl Default for TierPolicies {}

    fn default(2.0,     // Significant bonus for human entropy
            mobile_hsm_bonus: 0.5,        // Bonus for mobile HSMs
            cloud_hsm_penalty: -0.3,      // Small penalty for cloud dependency
            certification_weight: 1.5,    // Weight for security certifications
            performance_weight: 0.8,      // Weight for performance factors
            auto_elevation_enabled: true, // Enable automatic tier elevation

pub struct OperationRequirements {
    /// The min tier value
    pub min_tier: HsmTier,
    /// Whether require_human_entropy is enabled
    pub require_human_entropy: bool,
    /// Whether require_attestation is enabled
    pub require_attestation: bool,
    /// Whether require_fips is enabled
    pub require_fips: bool,
    /// The max latency ms value
    pub max_latency_ms: f64,
    /// Number of min_ops_per_second
    pub min_ops_per_second: u32,
    /// Collection of required algorithms
    pub required_algorithms: Vec<String>,
    /// Collection of compliance requirements
    pub compliance_requirements: Vec<String>,

pub struct SelectionCriteria {
    /// Whether prefer_human_entropy is enabled
    pub prefer_human_entropy: bool,
    /// Whether prefer_mobile is enabled
    pub prefer_mobile: bool,
    /// Whether prefer_local is enabled
    pub prefer_local: bool,
    /// The max latency tolerance value
    pub max_latency_tolerance: Duration,
    /// The min security level value
    pub min_security_level: HsmTier,
    /// Collection of required certifications
    pub required_certifications: Vec<String>,}

impl Default for SelectionCriteria {
            prefer_human_entropy: true,
            prefer_mobile: true,
            prefer_local: true,
            max_latency_tolerance: Duration::from_millis(HsmTier::BasicHardware,
            required_certifications: vec![],
