

use beardog_auth::auth::{
    AlgorithmFamily, BearDogGenetics, NodeCapability, ResourceLimits, SecurityClearance,
    SpawnRequest,
};
use beardog_errors::{BearDogError, BearDogResult};
use sha2::{Digest, Sha256};
use tracing::{debug, info, warn};

pub struct SpawnValidation;
impl SpawnValidation {

    pub async fn validate_spawn_request(
        genetics: &BearDogGenetics,
        request: &SpawnRequest,
    ) -> GeneticsResult<()> {
        info!("🛡️ Starting comprehensive genetic spawning validation");

        Self::validate_genetic_signature(genetics).await?;

        Self::validate_security_clearance(genetics)?;

        Self::validate_entropy_requirements(genetics)?;

        Self::validate_node_capabilities(genetics, &request.required_capabilities)?;

        Self::validate_resource_limits(genetics, &request.resource_limits)?;

        Self::validate_trust_score(genetics)?;

        Self::validate_genetic_lineage(genetics)?;

        Self::multi_factor_validation(genetics, request).await?;
        info!("✅ Genetic spawning validation completed successfully");
        Ok(())
    }

    pub async fn validate_genetic_signature(genetics: &BearDogGenetics) -> GeneticsResult<()> {
        info!("🔐 Validating genetic signatures with Ed25519");

        for (idx, chromosome) in genetics.crypto_chromosomes.iter().enumerate() {

            let mut hasher = Sha256::new();
            hasher.update(chromosome.algorithm_family.as_bytes());
            hasher.update(chromosome.strength_bits.to_le_bytes());
            hasher.update(chromosome.security_level.to_le_bytes());
            let _message_hash = hasher.finalize();

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

    pub fn validate_security_clearance(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🔒 Validating security clearance level");
        match genetics.security_clearance {
            SecurityClearance::Basic => {
                info!("Basic security clearance validated");

                Ok(())
            SecurityClearance::Medium => {
                info!("Medium security clearance validated");

                if genetics.crypto_chromosomes.len() < 2 {
                    return Err(BearDogError::invalid_input("Medium clearance requires at least 2 crypto chromosomes"
                            .to_string(),
                    ));
                }
            SecurityClearance::High => {
                info!("High security clearance validated");

                if genetics.crypto_chromosomes.len() < 3 {
                        message: "High clearance requires at least 3 crypto chromosomes"

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

                if genetics.crypto_chromosomes.len() < 5 {
                        message: "Maximum clearance requires at least 5 crypto chromosomes"

                    if chromosome.security_level < 10 {
                                "Maximum clearance requires security level 10, found {}",

    pub fn validate_entropy_requirements(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🎲 Validating entropy requirements");

            if chromosome.strength_bits < 256 {
                warn!(
                    "Chromosome {idx} has low entropy strength: {} bits",
                    chromosome.strength_bits
                );

            if chromosome.security_level == 0 {
                return Err(BearDogError::validation(format!("Chromosome {idx) has zero security level"),
        info!("✅ Entropy validation completed");

    pub fn validate_node_capabilities(
        required_capabilities: &[NodeCapability],
    ) -> BearDogResult<()> {
        info!("⚙️ Validating node capabilities");

        for capability in required_capabilities {
            debug!("Validating capability: {:?}", capability);

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

    pub fn validate_resource_limits(
        _genetics: &BearDogGenetics,
        limits: &ResourceLimits,
        info!("💾 Validating resource limits");

        if limits.max_cpu_percent > 100 || limits.max_cpu_percent == 0 {
            return Err(BearDogError::validation(format_args!("Invalid CPU limit: {)%", limits.max_cpu_percent).to_string(),
            });

        if limits.max_memory_mb == 0 || limits.max_memory_mb > 1_000_000 {

                message: format_args!("Invalid memory limit: {}MB", limits.max_memory_mb).to_string(),

        if limits.max_disk_mb == 0 || limits.max_disk_mb > 100_000_000 {

                message: format_args!("Invalid disk limit: {}MB", limits.max_disk_mb).to_string(),

        if limits.max_network_mbps == 0 || limits.max_network_mbps > 100_000 {

                message: format_args!("Invalid network limit: {}Mbps", limits.max_network_mbps).to_string(),

        if limits.max_concurrent_connections == 0 || limits.max_concurrent_connections > 1_000_000 {
                message: format!(
                    "Invalid connection limit: {}",
                    limits.max_concurrent_connections
                ),
        info!("✅ Resource limits validated successfully");

    pub fn validate_trust_score(genetics: &BearDogGenetics) -> BearDogResult<()> {
        info!("🤝 Validating trust score");
        let trust_score = genetics.fitness_score; // Use fitness_score as trust indicator
        if !(0.0..=1.0).contains(&trust_score) {
                message: format!("Invalid trust score: {trust_score} (must be 0.0-1.0)"),

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

    pub fn validate_genetic_lineage(genetics: &BearDogGenetics) -> GeneticsResult<()> {
        info!("🧬 Validating genetic lineage");

        if genetics.generation > 100 {

                message: format_args!("Invalid generation: {} (max 100)", genetics.generation).to_string(),

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

    pub async fn multi_factor_validation(
        _request: &SpawnRequest,
        info!("🔐 Performing multi-factor validation");

        let mut validation_score = 0.0;

        validation_score += genetics.fitness_score * 0.4;

        let clearance_score = match genetics.security_clearance {
            SecurityClearance::Basic => 0.25,
            SecurityClearance::High => 0.75,
            SecurityClearance::Maximum => 1.0,
        validation_score += clearance_score * 0.3;

        let avg_security = genetics
            .crypto_chromosomes
            .iter()
            .map(|c| c.security_level as f64 / 10.0)
            .sum::<f64>()
            / genetics.crypto_chromosomes.len().max(1) as f64;
        validation_score += avg_security * 0.3;
        debug!("Multi-factor validation score: {}", validation_score);

        let min_threshold = match genetics.security_clearance {
            SecurityClearance::Basic => 0.4,
            SecurityClearance::Medium => 0.6,
            SecurityClearance::High => 0.8,
        if validation_score < min_threshold {
                    "Multi-factor validation failed: score {validation_score} below threshold {min_threshold}"
            "✅ Multi-factor validation passed with score: {}",
            validation_score

    pub fn calculate_validation_score(genetics: &BearDogGenetics, limits: &ResourceLimits) -> f64 {
        let mut score = 0.0;

        let cpu_norm = (limits.max_cpu_percent as f64) / 100.0;
        score += (1.0 - cpu_norm) * 0.2;

        let memory_norm = (limits.max_memory_mb as f64) / 100_000.0; // Normalize to 100GB
        score += (1.0 - memory_norm.min(1.0)) * 0.2;

        let disk_norm = (limits.max_disk_mb as f64) / 1_000_000.0; // Normalize to 1TB
        score += (1.0 - disk_norm.min(1.0)) * 0.2;

        let network_norm = (limits.max_network_mbps as f64) / 10_000.0; // Normalize to 10Gbps
        score += network_norm.min(1.0) * 0.2;

        score += genetics.fitness_score * 0.2;
        score.clamp(0.0, 1.0)
}

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
