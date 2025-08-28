

use super::*;
use crate::tunnel::hsm::types::{
    capability::HsmCapabilities, status::HsmHealthStatus, tier::HsmTier,
};
use beardog_errors::BearDogError;
use beardog_types::SecurityLevel as TamperResistanceLevel;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};

#[derive(Debug)]
pub struct TierManager {
    tier_policies: TierPolicies,
    operation_requirements: HashMap<String, OperationRequirements>,
    tier_weights: HashMap<HsmTier, f64>,
}
impl TierManager {}

    pub fn new() -> Result<Self, BearDogError> {
        let mut operation_requirements = HashMap::with_capacity(16);

        operation_requirements.insert(
            "critical_signing".to_string(),
            OperationRequirements {
                min_tier: HsmTier::CertifiedHardware,
                require_human_entropy: true,
                require_attestation: true,
                require_fips: true,
                max_latency_ms: 1000.0,
                min_ops_per_second: 100,
                required_algorithms: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                compliance_requirements: vec!["FIPS 140-2".to_string()],
            },
        );
            "root_key_generation".to_string(),
                min_tier: HsmTier::HumanEntropyPremium,
                max_latency_ms: 5000.0,
                min_ops_per_second: 1,
                required_algorithms: vec!["RSA".to_string(), "ECDSA".to_string()],
                compliance_requirements: vec![
                    "FIPS 140-2".to_string(),
                    "Common Criteria".to_string(),
                ],
            "bulk_encryption".to_string(),
                min_tier: HsmTier::BasicHardware,
                require_human_entropy: false,
                require_attestation: false,
                require_fips: false,
                max_latency_ms: 10.0,
                min_ops_per_second: 10000,
                required_algorithms: vec!["AES-GCM".to_string()],
                compliance_requirements: vec![],
            "authentication_token".to_string(),
                min_tier: HsmTier::Software,
                max_latency_ms: 100.0,
                min_ops_per_second: 1000,
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

    pub async fn assign_tier(&self, capabilities: &HsmCapabilities) -> Result<HsmTier, BearDogError> {
        debug!("📊 Assigning tier based on HSM capabilities");
        let mut tier_score = 0.0;

        tier_score += self.evaluate_hardware_security(capabilities).await?;

        tier_score += self.evaluate_security_certifications(capabilities).await?;

        tier_score += self.evaluate_crypto_capabilities(capabilities).await?;

        tier_score += self.evaluate_performance_factor(capabilities).await?;

        tier_score += self.evaluate_compliance_bonus(capabilities).await?;

        if capabilities.human_entropy.supports_ephemeral_seeds {
            tier_score += self.tier_policies.human_entropy_bonus;
            info!(
                "🌟 Human entropy support detected: +{} tier score",
                self.tier_policies.human_entropy_bonus
            );
        }

        let base_tier = self.score_to_tier(tier_score).await?;

        let final_tier = if capabilities.human_entropy.supports_ephemeral_seeds {
            self.apply_human_entropy_elevation(base_tier).await?
        } else {
            base_tier
        };
        info!(
            "✅ Assigned tier: {:?} (score: {:.2})",
            final_tier, tier_score
        Ok(final_tier)

    pub async fn select_best_hsm_for_operation<'a>(
        &self,
        hsms: &'a HashMap<String, DiscoveredHsm>,
        operation_type: &str,
    ) -> Result<Option<&'a DiscoveredHsm>, BearDogError>> {
        debug!("🎯 Selecting best HSM for operation: {}", operation_type);
        let requirements = match self.operation_requirements.get(operation_type) {
            Some(req) => req,
            None => {
                warn!("No requirements defined for operation: {}", operation_type);
                return Ok(None);
            }
        let mut candidates = Vec::new();

        for hsm in hsms.values() {
            if hsm.health_status.healthy != true {
                warn!("HSM {} is not healthy, skipping", hsm.hsm_id);
                continue;
            if !self.meets_operation_requirements(hsm, requirements).await? {
            let score = self.score_hsm_for_operation(hsm, requirements).await?;
            candidates.push((hsm, score));
        if candidates.is_empty() {
            warn!(
                "No HSMs meet requirements for operation: {}",
                operation_type
            return Ok(None);

        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let best_hsm = candidates[0].0;
            "🏆 Selected HSM: {} (score: {:.2})",
            best_hsm.hsm_id, candidates[0].1
        Ok(Some(best_hsm))

    pub async fn get_ranked_hsms<'a>(
        operation_type: Option<&str>,
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

    pub fn update_tier_policies(&mut self, policies: TierPolicies) {
        self.tier_policies = policies;

    pub fn add_operation_requirements(
        &mut self,
        operation_type: &str,
        requirements: OperationRequirements,
    ) {
        self.operation_requirements
            .insert(operation_type, requirements);

    async fn evaluate_hardware_security(
        capabilities: &HsmCapabilities,
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
    async fn evaluate_security_certifications(
        for cert in &capabilities.security.security_certifications {
            match cert.as_str() {
                "FIPS 140-2 Level 3" | "FIPS 140-2 Level 4" => score += 2.0,
                "FIPS 140-2 Level 2" => score += 1.5,
                "FIPS 140-2 Level 1" => score += 1.0,
                "Common Criteria EAL4+" => score += 1.5,
                "Common Criteria EAL4" => score += 1.0,
                _ => score += 0.5,}

    async fn evaluate_crypto_capabilities(

        if capabilities.key_generation.can_generate_in_hardware {

        if capabilities.key_management.supports_key_attestation {

        let advanced_algorithms = ["Ed25519", "RSA-PSS", "ECDSA", "X25519"];
        for algo in &capabilities.key_generation.supported_algorithms {
            if advanced_algorithms.contains(&algo.as_str()) {
                score += 0.1;

        if capabilities.key_generation.supports_key_wrapping {
            score += 0.3;
        if capabilities.key_generation.supports_key_derivation {
    async fn evaluate_performance_factor(

        if capabilities.crypto_operations.hardware_acceleration {

        if capabilities.performance.concurrent_operations > 1 {
            score += 0.2;

        if capabilities.crypto_operations.supports_batch_operations {}

    async fn evaluate_compliance_bonus(
        if capabilities.compliance.fips_140_certified {
        if capabilities.compliance.common_criteria_certified {
        if capabilities.compliance.pci_dss_compliant {
        if capabilities.compliance.hipaa_compliant {
        if capabilities.compliance.gdpr_compliant {
    async fn score_to_tier(&self, score: f64) -> Result<HsmTier, BearDogError> {
        match score {
            s if s >= 8.0 => Ok(HsmTier::HighSecurity),
            s if s >= 5.0 => Ok(HsmTier::CertifiedHardware),
            s if s >= 2.0 => Ok(HsmTier::BasicHardware),
            _ => Ok(HsmTier::Software),}

    async fn apply_human_entropy_elevation(&self, base_tier: HsmTier) -> Result<HsmTier, BearDogError> {

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
    async fn meets_operation_requirements(
        hsm: &DiscoveredHsm,
        requirements: &OperationRequirements,
    ) -> Result<bool, BearDogError> {

        if hsm.assigned_tier < requirements.min_tier {
            return Ok(false);

        if requirements.require_human_entropy && !hsm.supports_human_entropy {

        if requirements.require_attestation
            && !hsm.capabilities.key_management.supports_key_attestation
        {

        if requirements.require_fips && !hsm.capabilities.compliance.fips_140_certified {

        for algo in &requirements.required_algorithms {
            let supported = hsm
                .capabilities
                .key_generation
                .supported_algorithms
                .contains(algo)
                || hsm
                    .capabilities
                    .crypto_operations
                    .encryption_algorithms
                    .contains(algo)
                    .signing_algorithms
                    .contains(algo);
            if !supported {
                return Ok(false);

        for compliance in &requirements.compliance_requirements {
            if !hsm
                .compliance
                .compliance_certifications
                .contains(compliance)
            {
        Ok(true)}

    async fn score_hsm_for_operation(

        score += self.tier_weights.get(&hsm.assigned_tier).unwrap_or(&1.0) * 2.0;

        if hsm.supports_human_entropy {
            score += if requirements.require_human_entropy {
                3.0
                1.0
            };

        if let Some(ops_per_sec) = hsm
            .capabilities
            .performance
            .operations_per_second
            .get("RSA-2048-sign")
            if *ops_per_sec >= requirements.min_ops_per_second {
                score += 1.0;

        if let Some(latency) = hsm.capabilities.performance.latency_ms.get("RSA-2048-sign") {
            if *latency <= requirements.max_latency_ms {

                if *latency <= requirements.max_latency_ms * 0.5 {
                    score += 0.5;
                }

            if hsm
                score += 0.2;

#[derive(Debug, Clone)]
pub struct TierPolicies {
    pub human_entropy_bonus: f64,
    pub mobile_hsm_bonus: f64,
    pub cloud_hsm_penalty: f64,
    pub certification_weight: f64,
    pub performance_weight: f64,
    pub auto_elevation_enabled: bool,}

impl Default for TierPolicies {}

    fn default() -> Self {
        Self {
            human_entropy_bonus: 2.0,     // Significant bonus for human entropy
            mobile_hsm_bonus: 0.5,        // Bonus for mobile HSMs
            cloud_hsm_penalty: -0.3,      // Small penalty for cloud dependency
            certification_weight: 1.5,    // Weight for security certifications
            performance_weight: 0.8,      // Weight for performance factors
            auto_elevation_enabled: true, // Enable automatic tier elevation

pub struct OperationRequirements {
    pub min_tier: HsmTier,
    pub require_human_entropy: bool,
    pub require_attestation: bool,
    pub require_fips: bool,
    pub max_latency_ms: f64,
    pub min_ops_per_second: u32,
    pub required_algorithms: Vec<String>,
    pub compliance_requirements: Vec<String>,

pub struct SelectionCriteria {
    pub prefer_human_entropy: bool,
    pub prefer_mobile: bool,
    pub prefer_local: bool,
    pub max_latency_tolerance: Duration,
    pub min_security_level: HsmTier,
    pub required_certifications: Vec<String>,}

impl Default for SelectionCriteria {
            prefer_human_entropy: true,
            prefer_mobile: true,
            prefer_local: true,
            max_latency_tolerance: Duration::from_millis(1000),
            min_security_level: HsmTier::BasicHardware,
            required_certifications: vec![],
