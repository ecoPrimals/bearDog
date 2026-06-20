// SPDX-License-Identifier: AGPL-3.0-or-later

//! Human-input entropy mixing.

use super::orchestrator::HsmEntropyOrchestrator;
use super::types::{EntropyGenerationRequest, HumanEntropyInput};
use beardog_errors::BearDogError;
use tracing::debug;
use uuid::Uuid;

impl HsmEntropyOrchestrator {
    /// Generate human entropy from best available HSM
    ///
    /// This method:
    /// 1. Selects the best available HSM device
    /// 2. Generates hardware entropy
    /// 3. Mixes with human input if provided
    /// 4. Returns seed ID for use in entropy hierarchy
    ///
    /// # Arguments
    ///
    /// * `length` - Length of entropy to generate (bytes)
    /// * `human_input` - Optional human biometric/behavioral data
    ///
    /// # Returns
    ///
    /// Seed ID in the entropy hierarchy system
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable HSM is available, entropy generation fails, or mixing fails.
    pub async fn generate_human_entropy(
        &mut self,
        length: usize,
        human_input: Option<HumanEntropyInput>,
    ) -> Result<Uuid, BearDogError> {
        let request = EntropyGenerationRequest {
            length,
            human_input,
            ..Default::default()
        };

        let result = self.generate_entropy(request).await?;
        Ok(result.seed_id)
    }

    /// Mix hardware entropy with human input
    pub(super) async fn mix_with_human_input(
        &self,
        hardware_entropy: Vec<u8>,
        human_input: HumanEntropyInput,
    ) -> Result<Vec<u8>, BearDogError> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();

        // Add hardware entropy
        hasher.update(&hardware_entropy);

        // Add biometric data if available
        if let Some(biometric) = human_input.biometric_data {
            hasher.update(&biometric);
            debug!("🔒 Mixed biometric data");
        }

        // Add behavioral data
        if let Some(behavioral) = human_input.behavioral_data {
            hasher.update(&behavioral);
            debug!("🔒 Mixed behavioral data");
        }

        // Add environmental data
        if let Some(environmental) = human_input.environmental_data {
            hasher.update(&environmental);
            debug!("🔒 Mixed environmental data");
        }

        // Add mixing salt
        hasher.update(b"beardog_hsm_entropy_orchestration_v1");

        Ok(hasher.finalize().to_vec())
    }
}
