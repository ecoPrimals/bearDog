

use super::types::*;

use beardog_errors::BearDogError;
use chrono::Utc;
use sha3::{Digest, Sha3_256};

pub struct EntropyValidator {

    config: EntropyHierarchyConfig,
}
impl EntropyValidator {

    pub fn new(/* hsm_manager: Arc<HsmManager>, */ config: EntropyHierarchyConfig) -> Self {
        Self {

            config,
        }
    }

    pub async fn generate_ownership_proof(
        &self,
        owner_identity: &HumanIdentity,
        entropy_data: &[u8],
    ) -> Result<OwnershipProof, BearDogError> {

        let proof_data = self.create_ownership_proof_data(owner_identity, entropy_data)?;

        let signature = {
            let mut hasher = Sha3_256::new();
            hasher.update(&proof_data);
            hasher.update(b"ownership_signature");
            hasher.finalize().to_vec()
        };
        Ok(OwnershipProof {
            signature,
            timestamp: Utc::now(),
            verification_key: owner_identity.public_key.clone(),
        })

    pub async fn generate_irreproducibility_proof(
        entropy_class: &EntropyClass,
    ) -> Result<IrreproducibilityProof, BearDogError> {

        let entropy_commitment = self.generate_entropy_commitment(entropy_data)?;

        let temporal_proof = self.generate_temporal_proof(entropy_class).await?;

        let uniqueness_proof = self
            .generate_uniqueness_proof(entropy_data, entropy_class)
            .await?;
        Ok(IrreproducibilityProof {
            entropy_commitment,
            temporal_proof,
            uniqueness_proof,

    fn create_ownership_proof_data(
    ) -> Result<Vec<u8>, BearDogError>> {
        let mut hasher = Sha3_256::new();

        hasher.update(owner_identity.identity_id.as_bytes());
        hasher.update(&owner_identity.public_key);

        if let Some(bio_hash) = &owner_identity.biometric_hash {
            hasher.update(bio_hash);

        let entropy_hash = Sha3_256::digest(entropy_data);
        hasher.update(entropy_hash);

        hasher.update(Utc::now().timestamp().to_le_bytes());

        hasher.update(b"entropy_ownership_proof");
        Ok(hasher.finalize().to_vec())

    fn generate_entropy_commitment(&self, entropy_data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        hasher.update(entropy_data);
        hasher.update(b"entropy_commitment");

    async fn generate_temporal_proof(
        let timestamp = match entropy_class {
            EntropyClass::HumanLivedExperience {
                capture_timestamp, ..
            } => *capture_timestamp,
            EntropyClass::HumanSupervisedMachine {
                validation_timestamp,
                ..
            } => *validation_timestamp,
            EntropyClass::StoreBoughtMachine {
                generation_timestamp,
            } => *generation_timestamp,

        hasher.update(timestamp.timestamp().to_le_bytes());
        hasher.update(b"temporal_proof");

        let class_type = match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => "human_lived_experience",
            EntropyClass::HumanSupervisedMachine { .. } => "human_supervised_machine",
            EntropyClass::StoreBoughtMachine { .. } => "store_bought_machine",
        hasher.update(class_type.as_bytes());
        let temporal_data = hasher.finalize().to_vec();

            hasher.update(&temporal_data);
            hasher.update(b"temporal_signature");
        Ok(signature)

    async fn generate_uniqueness_proof(

        hasher.update(Sha3_256::digest(entropy_data));

        match entropy_class {
                source_type,
                biometric_signature,
            } => {
                hasher.update(b"human_lived_experience");

                match source_type {
                    HumanEntropySource::Microphone {
                        spectral_features, ..
                    } => {
                        hasher.update(b"microphone");
                        for feature in spectral_features {
                            hasher.update(feature.to_le_bytes());
                        }
                    }
                    HumanEntropySource::Camera {
                        lighting_variations,
                        ..
                        hasher.update(b"camera");
                        for variation in lighting_variations {
                            hasher.update(variation.to_le_bytes());
                    HumanEntropySource::Haptic {
                        motion_patterns, ..
                        hasher.update(b"haptic");
                        for pattern in motion_patterns {
                            hasher.update(pattern.to_le_bytes());
                    HumanEntropySource::Biometric { entropy_hash, .. } => {
                        hasher.update(b"biometric");
                        hasher.update(entropy_hash);
                    HumanEntropySource::MultiModalHuman { sources, .. } => {
                        hasher.update(b"multimodal");
                        hasher.update(sources.len().to_le_bytes());
                }

                hasher.update(&biometric_signature.0);
            }
                machine_source,
                human_validator,
                hasher.update(b"human_supervised_machine");
                hasher.update(human_validator.identity_id.as_bytes());

                match machine_source {
                    MachineEntropySource::HardwareRNG { device_id, .. } => {
                        hasher.update(device_id.as_bytes());
                    MachineEntropySource::Csprng { algorithm, .. } => {
                        hasher.update(algorithm.as_bytes());
                    MachineEntropySource::DerivedFromHuman {
                        transition_timestamp,
                        hasher.update(transition_timestamp.timestamp().to_le_bytes());
                reproducibility_index,
                hasher.update(b"store_bought_machine");
                hasher.update(reproducibility_index.to_le_bytes());
                    MachineEntropySource::Csprng {
                        algorithm,
                        state_size,
                        hasher.update(state_size.to_le_bytes());
                    MachineEntropySource::DerivedFromHuman { .. } => {
                        hasher.update(b"derived_from_human");

        hasher.update(b"uniqueness_proof");
        let uniqueness_data = hasher.finalize().to_vec();
            hasher.update(&uniqueness_data);
            hasher.update(b"uniqueness_signature");

    pub fn validate_entropy_quality(&self, entropy_class: &EntropyClass) -> Result<f64, BearDogError> {
        let quality_score = match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => {
                self.calculate_human_entropy_quality(source_type)?
            EntropyClass::HumanSupervisedMachine { .. } => {

                0.8

                1.0 - reproducibility_index
        if quality_score < self.config.min_entropy_quality {
            return Err(BearDogError::invalid_input(format!(
                    "Entropy quality {} below minimum threshold {}",
                    quality_score, self.config.min_entropy_quality
                )));
        Ok(quality_score)

    fn calculate_human_entropy_quality(&self, source: &HumanEntropySource) -> Result<f64, BearDogError> {
        let quality = match source {
            HumanEntropySource::MultiModalHuman {
                confidence_score,
                sources,

                let diversity_bonus = (sources.len() as f64 / 10.0).min(0.2);
                confidence_score + diversity_bonus
            HumanEntropySource::Biometric { quality_score, .. } => *quality_score,
            HumanEntropySource::Microphone {
                spectral_features,
                duration_ms,

                let feature_diversity = spectral_features.len() as f64 / 100.0;
                let duration_factor = (*duration_ms as f64 / 10000.0).min(1.0); // 10 seconds max
                (feature_diversity * 0.7 + duration_factor * 0.3).clamp(0.5, 1.0)
            HumanEntropySource::Camera {
                lighting_variations,

                if lighting_variations.is_empty() {
                    return Ok(0.6); // Base quality for camera without variations
                let variation_score = lighting_variations.iter().sum::<f32>() as f64
                    / lighting_variations.len() as f64;
                let duration_factor = (*duration_ms as f64 / 5000.0).min(1.0); // 5 seconds max
                (variation_score * 0.8 + duration_factor * 0.2).clamp(0.6, 1.0)
            HumanEntropySource::Haptic {
                motion_patterns,
                touch_points,

                let motion_complexity = motion_patterns.len() as f64 / 50.0;
                let touch_diversity = touch_points.len() as f64 / 20.0;
                (motion_complexity * 0.6 + touch_diversity * 0.4).clamp(0.7, 1.0)
        Ok(quality.min(1.0))

    pub async fn verify_ownership_proof(
        _proof: &OwnershipProof,
        _owner_identity: &HumanIdentity,
        _entropy_data: &[u8],
    ) -> Result<bool, BearDogError> {

        Ok(true)

    pub async fn verify_irreproducibility_proof(
        proof: &IrreproducibilityProof,
        _entropy_class: &EntropyClass,

        let expected_commitment = self.generate_entropy_commitment(entropy_data)?;
        if proof.entropy_commitment != expected_commitment {
            return Ok(false);

        if proof.temporal_proof.is_empty() || proof.uniqueness_proof.is_empty() {

    pub fn validate_usage_policy(
        policy: &SeedUsagePolicy,
    ) -> Result<(), BearDogError> {

            EntropyClass::HumanLivedExperience { .. } => {

                Ok(())

                if policy
                    .allowed_operations
                    .contains(&"high_security_signing".to_string())
                    && !policy.requires_approval
                {
                    return Err(BearDogError::invalid_input("High security operations with supervised entropy require approval"
                                .to_string(),
                    ));
            EntropyClass::StoreBoughtMachine { .. } => {

                if policy.max_uses.is_none() {
                    return Err(BearDogError::invalid_input("Machine entropy must have usage limits".to_string(),
                if !policy.requires_approval {
                    return Err(BearDogError::invalid_input("Machine entropy operations require approval".to_string(),

    pub fn check_security_requirements(&self, entropy_class: &EntropyClass) -> Result<u8, BearDogError> {
        let security_level = match entropy_class {
            EntropyClass::HumanLivedExperience { .. } => 100, // SecurityLevel::Maximum,
            EntropyClass::HumanSupervisedMachine { .. } => 80, // SecurityLevel::High,
                if *reproducibility_index < 0.3 {

                    80
                } else if *reproducibility_index < 0.7 {

                    60
                } else {

                    40
        Ok(security_level)
