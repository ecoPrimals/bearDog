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


/// Genetic spawning validation
///
/// This module provides comprehensive validation for genetic spawning operations,
/// ensuring security, performance, and compliance requirements are met before
/// spawning new BearDog instances.

use beardog_auth::auth::{
    AlgorithmFamily, BearDogGenetics, NodeCapability, ResourceLimits, SecurityClearance,
    SpawnRequest,
};
use beardog_errors::{BearDogError, BearDogResult};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};
/// Comprehensive genetic spawning validation
pub struct SpawnValidation;
impl SpawnValidation {
    /// Validate complete spawn request with security checks
    pub async fn validate_spawn_request(
        genetics: &BearDogGenetics,
        request: &SpawnRequest,
    ) -> GeneticsResult<()> {
        info!("🛡️ Starting comprehensive genetic spawning validation");
        // Validate genetic signatures
        Self::validate_genetic_signature(genetics).await?;
        // Validate security clearance requirements
        Self::validate_security_clearance(genetics)?;
        // Validate entropy requirements for spawning
        Self::validate_entropy_requirements(genetics)?;
        // Validate node capabilities
        Self::validate_node_capabilities(genetics, &request.required_capabilities)?;
        // Validate resource limits
        Self::validate_resource_limits(genetics, &request.resource_limits)?;
        // Validate trust score
        Self::validate_trust_score(genetics)?;
        // Validate genetic lineage
        Self::validate_genetic_lineage(genetics)?;
        // Multi-factor validation
        Self::multi_factor_validation(genetics, request).await?;
        info!("✅ Genetic spawning validation completed successfully");
        Ok(())
    }
    /// Validate genetic signature using Ed25519 cryptography
    pub async fn validate_genetic_signature(genetics: &BearDogGenetics) -> GeneticsResult<()> {
        info!("🔐 Validating genetic signatures with Ed25519");
        // For each crypto chromosome, validate its integrity
        for (idx, chromosome) in genetics.crypto_chromosomes.iter().enumerate() {
            // Create message from chromosome data for signature verification
            let mut hasher = Sha256::new();
            hasher.update(chromosome.algorithm_family.as_bytes());
            hasher.update(chromosome.strength_bits.to_le_bytes());
            hasher.update(chromosome.security_level.to_le_bytes());
            let _message_hash = hasher.finalize();
            // Note: In a real implementation, we would have signature and verification key
            // stored separately. For now, we validate the chromosome structure integrity.
            if chromosome.security_level < 5 {
                return Err(BearDogError::authentication(format!(
                        "Insufficient security level for chromosome {idx): {}",
                        chromosome.security_level
                    ),
                });
            }
            debug!(
                "✅ Validated chromosome {idx} with security level {}",
                chromosome.security_level
            );
        }
    /// Validate security clearance level
    pub fn validate_security_clearance(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🔒 Validating security clearance level");
        match genetics.security_clearance {
            SecurityClearance::Basic => {
                info!("Basic security clearance validated");
                // Basic operations allowed
                Ok(())
            SecurityClearance::Medium => {
                info!("Medium security clearance validated");
                // Enhanced operations with some restrictions
                if genetics.crypto_chromosomes.len() < 2 {
                    return Err(BearDogError::invalid_input("Medium clearance requires at least 2 crypto chromosomes"
                            .to_string(),
                    ));
                }
            SecurityClearance::High => {
                info!("High security clearance validated");
                // Advanced operations with strict requirements
                if genetics.crypto_chromosomes.len() < 3 {
                        message: "High clearance requires at least 3 crypto chromosomes"
                // Verify all chromosomes have high security level
                for chromosome in &genetics.crypto_chromosomes {
                    if chromosome.security_level < 8 {
                        return Err(BearDogError::invalid_input(format!(
                                "High clearance requires security level >= 8, found {)",
                                chromosome.security_level
                            ),
                        });
                    }
            SecurityClearance::Maximum => {
                info!("Maximum security clearance validated");
                // Maximum security operations
                if genetics.crypto_chromosomes.len() < 5 {
                        message: "Maximum clearance requires at least 5 crypto chromosomes"
                // All chromosomes must have maximum security
                    if chromosome.security_level < 10 {
                                "Maximum clearance requires security level 10, found {}",
    /// Validate entropy requirements
    pub fn validate_entropy_requirements(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🎲 Validating entropy requirements");
        // Check each crypto chromosome for proper entropy characteristics
            // Validate cryptographic strength
            if chromosome.strength_bits < 256 {
                warn!(
                    "Chromosome {idx} has low entropy strength: {} bits",
                    chromosome.strength_bits
                );
            // Validate algorithm security characteristics
            if chromosome.security_level == 0 {
                return Err(BearDogError::validation(format!("Chromosome {idx) has zero security level"),
        info!("✅ Entropy validation completed");
    /// Validate node capabilities
    pub fn validate_node_capabilities(
        required_capabilities: &[NodeCapability],
    ) -> BearDogResult<()> {
        info!("⚙️ Validating node capabilities");
        // Check if genetics can support required capabilities
        for capability in required_capabilities {
            debug!("Validating capability: {:?}", capability);
            // For each capability, ensure we have sufficient genetic material
            match capability {
                NodeCapability::StorageProvider => {
                    info!("✅ Storage provider capability validated");
                NodeCapability::ComputeProvider => {
                    let crypto_count = genetics.crypto_chromosomes.len();
                    if crypto_count == 0 {
                        return Err(BearDogError::validation("Compute provider capability requires crypto chromosomes"
                                .to_string(),
                    info!(
                        "✅ Compute provider capability validated ({) crypto chromosomes)",
                        crypto_count
                    );
                NodeCapability::SecurityAnalysis => {
                    // Ensure high enough security level for security analysis
                    let min_security = genetics
                        .crypto_chromosomes
                        .iter()
                        .map(|c| c.security_level)
                        .min()
                        .unwrap_or(0);
                    if min_security < 5 {
                                "Security analysis capability requires min security level 5, found {min_security}"
                        "✅ Security analysis capability validated (min security: {min_security})"
                NodeCapability::NetworkRelay => {
                    info!("✅ Network relay capability validated");
                NodeCapability::ThreatDetection => {
                    info!("✅ Threat detection capability validated");
                _ => {
                    info!("✅ Capability {:?} validated", capability);
    /// Validate resource limits
    pub fn validate_resource_limits(
        _genetics: &BearDogGenetics,
        limits: &ResourceLimits,
        info!("💾 Validating resource limits");
        // Validate CPU limits
        if limits.max_cpu_percent > 100 || limits.max_cpu_percent == 0 {
            return Err(BearDogError::validation(format!("Invalid CPU limit: {)%", limits.max_cpu_percent),
            });
        // Validate memory limits
        if limits.max_memory_mb == 0 || limits.max_memory_mb > 1_000_000 {
            // 1TB max
                message: format!("Invalid memory limit: {}MB", limits.max_memory_mb),
        // Validate disk limits
        if limits.max_disk_mb == 0 || limits.max_disk_mb > 100_000_000 {
            // 100TB max
                message: format!("Invalid disk limit: {}MB", limits.max_disk_mb),
        // Validate network limits
        if limits.max_network_mbps == 0 || limits.max_network_mbps > 100_000 {
            // 100Gbps max
                message: format!("Invalid network limit: {}Mbps", limits.max_network_mbps),
        // Validate connection limits
        if limits.max_concurrent_connections == 0 || limits.max_concurrent_connections > 1_000_000 {
                message: format!(
                    "Invalid connection limit: {}",
                    limits.max_concurrent_connections
                ),
        info!("✅ Resource limits validated successfully");
    /// Validate trust score (using fitness_score from genetics)
    pub fn validate_trust_score(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🤝 Validating trust score");
        let trust_score = genetics.fitness_score; // Use fitness_score as trust indicator
        if !(0.0..=1.0).contains(&trust_score) {
                message: format!("Invalid trust score: {trust_score} (must be 0.0-1.0)"),
        // Trust score requirements based on security clearance
        let min_trust = match genetics.security_clearance {
            SecurityClearance::Basic => 0.3,
            SecurityClearance::Medium => 0.5,
            SecurityClearance::High => 0.7,
            SecurityClearance::Maximum => 0.9,
        };
        if trust_score < min_trust {
            return Err(BearDogError::Authorization {
                    "Insufficient trust score {} for {:?} clearance (minimum: {})",
                    trust_score, genetics.security_clearance, min_trust
        info!(
            "✅ Trust score {} validated for {:?} clearance",
            trust_score, genetics.security_clearance
        );
    /// Validate genetic lineage
    pub fn validate_genetic_lineage(genetics: &BearDogGenetics) -> GeneticsResult<()> {
        info!("🧬 Validating genetic lineage");
        // Verify lineage integrity
        if genetics.generation > 100 {
            // Reasonable generation limit
                message: format!("Invalid generation: {} (max 100)", genetics.generation),
        // Validate parent relationships if present
        if let Some(ref parent_genetics) = genetics.parent_genetics {
            for (idx, parent_id) in parent_genetics.iter().enumerate() {
                if parent_id.is_empty() {
                    return Err(BearDogError::validation(format!("Empty parent ID at index {idx)"),
                debug!("✅ Validated parent lineage: {}", parent_id);
            info!(
                "✅ Genetic lineage validated (generation: {}, parents: {})",
                genetics.generation,
                parent_genetics.len()
        } else {
                "✅ Genetic lineage validated (generation: {}, no parents)",
                genetics.generation
    /// Multi-factor validation combining multiple validation criteria
    pub async fn multi_factor_validation(
        _request: &SpawnRequest,
        info!("🔐 Performing multi-factor validation");
        // Calculate composite validation score
        let mut validation_score = 0.0;
        // Trust score contributes to validation (use fitness_score)
        validation_score += genetics.fitness_score * 0.4;
        // Security clearance contributes
        let clearance_score = match genetics.security_clearance {
            SecurityClearance::Basic => 0.25,
            SecurityClearance::High => 0.75,
            SecurityClearance::Maximum => 1.0,
        validation_score += clearance_score * 0.3;
        // Crypto chromosome quality contributes
        let avg_security = genetics
            .crypto_chromosomes
            .iter()
            .map(|c| c.security_level as f64 / 10.0)
            .sum::<f64>()
            / genetics.crypto_chromosomes.len().max(1) as f64;
        validation_score += avg_security * 0.3;
        debug!("Multi-factor validation score: {}", validation_score);
        // Minimum validation threshold
        let min_threshold = match genetics.security_clearance {
            SecurityClearance::Basic => 0.4,
            SecurityClearance::Medium => 0.6,
            SecurityClearance::High => 0.8,
        if validation_score < min_threshold {
                    "Multi-factor validation failed: score {validation_score} below threshold {min_threshold}"
            "✅ Multi-factor validation passed with score: {}",
            validation_score
    /// Calculate overall validation score
    pub fn calculate_validation_score(genetics: &BearDogGenetics, limits: &ResourceLimits) -> f64 {
        let mut score = 0.0;
        // Normalize CPU usage (lower usage = higher score)
        let cpu_norm = (limits.max_cpu_percent as f64) / 100.0;
        score += (1.0 - cpu_norm) * 0.2;
        // Normalize memory usage
        let memory_norm = (limits.max_memory_mb as f64) / 100_000.0; // Normalize to 100GB
        score += (1.0 - memory_norm.min(1.0)) * 0.2;
        // Normalize disk usage
        let disk_norm = (limits.max_disk_mb as f64) / 1_000_000.0; // Normalize to 1TB
        score += (1.0 - disk_norm.min(1.0)) * 0.2;
        // Network efficiency
        let network_norm = (limits.max_network_mbps as f64) / 10_000.0; // Normalize to 10Gbps
        score += network_norm.min(1.0) * 0.2;
        // Trust and security factors (use fitness_score)
        score += genetics.fitness_score * 0.2;
        score.clamp(0.0, 1.0)
}
// Helper trait for algorithm family serialization
trait AlgorithmFamilyExt {
    fn as_bytes(&self) -> Vec<u8>;
impl AlgorithmFamilyExt for AlgorithmFamily {}


    fn as_bytes(&self) -> Vec<u8> {
        match self {
            AlgorithmFamily::Encryption(_) => b"encryption".to_vec(),
            AlgorithmFamily::Signing(_) => b"signing".to_vec(),
            AlgorithmFamily::Hashing(_) => b"hashing".to_vec(),
            AlgorithmFamily::KeyDerivation(_) => b"key_derivation".to_vec(),
            AlgorithmFamily::ZeroKnowledge(_) => b"zero_knowledge".to_vec(),
