// SPDX-License-Identifier: AGPL-3.0-or-later

//! Human-input entropy mixing.

use super::orchestrator::HsmEntropyOrchestrator;
use super::types::{EntropyGenerationRequest, HumanEntropyInput};
use beardog_errors::BearDogError;
use tracing::debug;
use uuid::Uuid;

impl HsmEntropyOrchestrator {
    /// Generate human entropy from best available HSM.
    ///
    /// Selects the best available HSM, generates hardware entropy (async path
    /// for FIDO2), mixes with human input if provided, and returns a seed ID
    /// for use in the entropy hierarchy.
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

        let result = self.generate_entropy_async(request).await?;
        Ok(result.seed_id)
    }

    /// Mix hardware entropy with human input using BLAKE3.
    pub(super) fn mix_with_human_input(
        &self,
        hardware_entropy: Vec<u8>,
        human_input: HumanEntropyInput,
    ) -> Result<Vec<u8>, BearDogError> {
        let mut hasher = blake3::Hasher::new_derive_key("beardog-hsm-entropy-human-mix-v2");

        hasher.update(&hardware_entropy);

        if let Some(biometric) = human_input.biometric_data {
            hasher.update(b"BIOMETRIC");
            hasher.update(&biometric);
            debug!("🔒 Mixed biometric data");
        }

        if let Some(behavioral) = human_input.behavioral_data {
            hasher.update(b"BEHAVIORAL");
            hasher.update(&behavioral);
            debug!("🔒 Mixed behavioral data");
        }

        if let Some(environmental) = human_input.environmental_data {
            hasher.update(b"ENVIRONMENTAL");
            hasher.update(&environmental);
            debug!("🔒 Mixed environmental data");
        }

        Ok(hasher.finalize().as_bytes().to_vec())
    }
}
