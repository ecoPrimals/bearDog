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


/// HSM Tier Management System
///
/// This module handles HSM tiering, assignment, and selection logic based on
/// capabilities and operational requirements. It prioritizes HSMs with human
/// entropy ephemeral seed support for premium tiers.

use super::*;
use crate::tunnel::hsm::types::{
    capability::HsmCapabilities, status::HsmHealthStatus, tier::HsmTier,
};
use beardog_errors::BearDogResult;
use beardog_types::SecurityLevel as TamperResistanceLevel;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, error, info, warn};
/// HSM tier manager for classification and selection
#[derive(Debug)]
pub struct TierManager {
    tier_policies: TierPolicies,
    operation_requirements: HashMap<String, OperationRequirements>,
    tier_weights: HashMap<HsmTier, f64>,
}
impl TierManager {}


    pub fn new() -> BearDogResult<Self> {
        let mut operation_requirements = HashMap::new();
        // Define operation-specific requirements
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
        // Define tier weights (higher = better)
        let mut tier_weights = HashMap::new();
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
    /// Assign tier to an HSM based on its capabilities
    pub async fn assign_tier(&self, capabilities: &HsmCapabilities) -> BearDogResult<HsmTier> {
        debug!("📊 Assigning tier based on HSM capabilities");
        let mut tier_score = 0.0;
        // Base score from hardware capabilities
        tier_score += self.evaluate_hardware_security(capabilities).await?;
        // Security certifications bonus
        tier_score += self.evaluate_security_certifications(capabilities).await?;
        // Cryptographic capabilities bonus
        tier_score += self.evaluate_crypto_capabilities(capabilities).await?;
        // Performance considerations
        tier_score += self.evaluate_performance_factor(capabilities).await?;
        // Compliance bonus
        tier_score += self.evaluate_compliance_bonus(capabilities).await?;
        // Human entropy bonus - this is the key differentiator!
        if capabilities.human_entropy.supports_ephemeral_seeds {
            tier_score += self.tier_policies.human_entropy_bonus;
            info!(
                "🌟 Human entropy support detected: +{} tier score",
                self.tier_policies.human_entropy_bonus
            );
        }
        // Determine tier from score
        let base_tier = self.score_to_tier(tier_score).await?;
        // Apply human entropy tier elevation policy
        let final_tier = if capabilities.human_entropy.supports_ephemeral_seeds {
            self.apply_human_entropy_elevation(base_tier).await?
        } else {
            base_tier
        };
        info!(
            "✅ Assigned tier: {:?} (score: {:.2})",
            final_tier, tier_score
        Ok(final_tier)
    /// Select the best HSM for a specific operation
    pub async fn select_best_hsm_for_operation<'a>(
        &self,
        hsms: &'a HashMap<String, DiscoveredHsm>,
        operation_type: &str,
    ) -> BearDogResult<Option<&'a DiscoveredHsm>> {
        debug!("🎯 Selecting best HSM for operation: {}", operation_type);
        let requirements = match self.operation_requirements.get(operation_type) {
            Some(req) => req,
            None => {
                warn!("No requirements defined for operation: {}", operation_type);
                return Ok(None);
            }
        let mut candidates = Vec::new();
        // Filter HSMs that meet requirements
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
        // Sort by score (highest first)
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let best_hsm = candidates[0].0;
            "🏆 Selected HSM: {} (score: {:.2})",
            best_hsm.hsm_id, candidates[0].1
        Ok(Some(best_hsm))
    /// Get HSMs ranked by tier and suitability
    pub async fn get_ranked_hsms<'a>(
        operation_type: Option<&str>,
    ) -> BearDogResult<Vec<(&'a DiscoveredHsm, f64)>> {
        debug!("📈 Ranking HSMs by tier and suitability");
        let mut ranked = Vec::new();
            let mut score = self
                .tier_weights
                .get(&hsm.assigned_tier)
                .unwrap_or(&1.0)
                .clone();
            // Apply health status modifier
            if hsm.health_status.healthy {
                // No modifier for healthy status
            } else if hsm.health_status.error_message.is_some() {
                score *= 0.5; // Degraded/failed status
            } else {
                score *= 0.7; // Unknown status
            // Human entropy bonus
            if hsm.supports_human_entropy {
                score *= 1.2; // 20% bonus
            ranked.push((hsm, score));
        ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        Ok(ranked)
    /// Update tier policies
    pub fn update_tier_policies(&mut self, policies: TierPolicies) {
        self.tier_policies = policies;
    /// Add or update operation requirements}


    pub fn add_operation_requirements(
        &mut self,
        operation_type: String,
        requirements: OperationRequirements,
    ) {
        self.operation_requirements
            .insert(operation_type, requirements);
    // Private helper methods
    async fn evaluate_hardware_security(
        capabilities: &HsmCapabilities,
    ) -> BearDogResult<f64> {
        let mut score = 0.0;
        // FIPS 140-2 Level scoring
        match capabilities.security.fips_140_level {
            1 => score += 1.0,
            2 => score += 2.0,
            3 => score += 3.5,
            4 => score += 5.0,
            _ => {} // No FIPS certification
        // Common Criteria certification
        if capabilities.security.common_criteria_level.is_some() {
            score += 1.0;
        // Tamper resistance scoring
        match capabilities.security.tamper_resistance {
            TamperResistanceLevel::Software => {}
            TamperResistanceLevel::Medium => score += 0.5,
            TamperResistanceLevel::Tee => score += 1.0,
            TamperResistanceLevel::Hardware => score += 1.5,
            TamperResistanceLevel::CertifiedHardware => score += 2.0,
            TamperResistanceLevel::MaximumSecurity => score += 2.5,
        // Hardware key storage
        if capabilities.security.secure_key_storage {
        // Side channel resistance
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
        // Key generation in hardware
        if capabilities.key_generation.can_generate_in_hardware {
        // Key attestation support
        if capabilities.key_management.supports_key_attestation {
        // Advanced algorithms
        let advanced_algorithms = ["Ed25519", "RSA-PSS", "ECDSA", "X25519"];
        for algo in &capabilities.key_generation.supported_algorithms {
            if advanced_algorithms.contains(&algo.as_str()) {
                score += 0.1;
        // Key wrapping and derivation
        if capabilities.key_generation.supports_key_wrapping {
            score += 0.3;
        if capabilities.key_generation.supports_key_derivation {
    async fn evaluate_performance_factor(
        // Hardware acceleration bonus
        if capabilities.crypto_operations.hardware_acceleration {
        // Concurrent operations capability
        if capabilities.performance.concurrent_operations > 1 {
            score += 0.2;
        // Batch operations support
        if capabilities.crypto_operations.supports_batch_operations {}


    async fn evaluate_compliance_bonus(
        if capabilities.compliance.fips_140_certified {
        if capabilities.compliance.common_criteria_certified {
        if capabilities.compliance.pci_dss_compliant {
        if capabilities.compliance.hipaa_compliant {
        if capabilities.compliance.gdpr_compliant {
    async fn score_to_tier(&self, score: f64) -> BearDogResult<HsmTier> {
        match score {
            s if s >= 8.0 => Ok(HsmTier::HighSecurity),
            s if s >= 5.0 => Ok(HsmTier::CertifiedHardware),
            s if s >= 2.0 => Ok(HsmTier::BasicHardware),
            _ => Ok(HsmTier::Software),}


    async fn apply_human_entropy_elevation(&self, base_tier: HsmTier) -> BearDogResult<HsmTier> {
        // Elevate HSMs with human entropy ephemeral seed support
        match base_tier {
            HsmTier::Software => Ok(HsmTier::BasicHardware),
            HsmTier::BasicHardware => Ok(HsmTier::CertifiedHardware),
            HsmTier::CertifiedHardware => Ok(HsmTier::HighSecurity),
            HsmTier::HighSecurity => Ok(HsmTier::HumanEntropyPremium),
            HsmTier::HumanEntropyPremium => Ok(HsmTier::HumanEntropyPremium), // Already at top
            // Handle structured tiers
            HsmTier::SmartphoneHsm { .. } => Ok(HsmTier::BasicHardware),
            HsmTier::SoftwareHsm { .. } => Ok(HsmTier::CertifiedHardware),
            HsmTier::HardwareHsm { .. } => Ok(HsmTier::HighSecurity),
            HsmTier::HybridHsm { .. } => Ok(HsmTier::HumanEntropyPremium),
    async fn meets_operation_requirements(
        hsm: &DiscoveredHsm,
        requirements: &OperationRequirements,
    ) -> BearDogResult<bool> {
        // Check minimum tier
        if hsm.assigned_tier < requirements.min_tier {
            return Ok(false);
        // Check human entropy requirement
        if requirements.require_human_entropy && !hsm.supports_human_entropy {
        // Check attestation requirement
        if requirements.require_attestation
            && !hsm.capabilities.key_management.supports_key_attestation
        {
        // Check FIPS requirement
        if requirements.require_fips && !hsm.capabilities.compliance.fips_140_certified {
        // Check required algorithms
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
        // Check compliance requirements
        for compliance in &requirements.compliance_requirements {
            if !hsm
                .compliance
                .compliance_certifications
                .contains(compliance)
            {
        Ok(true)}


    async fn score_hsm_for_operation(
        // Tier bonus
        score += self.tier_weights.get(&hsm.assigned_tier).unwrap_or(&1.0) * 2.0;
        // Human entropy bonus for operations that benefit from it
        if hsm.supports_human_entropy {
            score += if requirements.require_human_entropy {
                3.0
                1.0
            };
        // Performance scoring
        if let Some(ops_per_sec) = hsm
            .capabilities
            .performance
            .operations_per_second
            .get("RSA-2048-sign")
            if *ops_per_sec >= requirements.min_ops_per_second {
                score += 1.0;
        // Latency scoring (lower is better)
        if let Some(latency) = hsm.capabilities.performance.latency_ms.get("RSA-2048-sign") {
            if *latency <= requirements.max_latency_ms {
                // Additional bonus for very low latency
                if *latency <= requirements.max_latency_ms * 0.5 {
                    score += 0.5;
                }
        // Algorithm support bonus
            if hsm
                score += 0.2;
/// Tier assignment policies
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
/// Requirements for specific operations
pub struct OperationRequirements {
    pub min_tier: HsmTier,
    pub require_human_entropy: bool,
    pub require_attestation: bool,
    pub require_fips: bool,
    pub max_latency_ms: f64,
    pub min_ops_per_second: u32,
    pub required_algorithms: Vec<String>,
    pub compliance_requirements: Vec<String>,
/// HSM selection criteria for operations
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
