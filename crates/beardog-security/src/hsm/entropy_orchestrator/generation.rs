// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy generation pipeline.

use super::config::OS_RNG_SOURCE;
use super::discovery::HsmSource;
use super::orchestrator::HsmEntropyOrchestrator;
use super::quality::{EntropySourceReport, os_rng_fallback_report};
use super::types::{EntropyGenerationRequest, EntropyGenerationResult};
use beardog_errors::BearDogError;
use tracing::{debug, info, warn};
use uuid::Uuid;

impl HsmEntropyOrchestrator {
    /// Generate entropy with full control
    ///
    /// Advanced API that provides detailed control over entropy generation
    /// and returns comprehensive result information.
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable HSM is available, entropy generation fails, or mixing fails.
    pub async fn generate_entropy(
        &mut self,
        request: EntropyGenerationRequest,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        info!("🌱 Generating human entropy ({} bytes)", request.length);

        // Step 1: Select best available HSM
        let hsm_source = self.select_best_hsm(&request).await?;

        // Step 2: Generate entropy (currently OS RNG fallback until hardware is wired)
        let (hardware_entropy, source_report) =
            self.generate_from_hsm(&hsm_source, request.length).await?;

        // Step 3: Mix with human input if provided
        let mixed_entropy = if let Some(human_input) = request.human_input {
            self.mix_with_human_input(hardware_entropy, human_input)
                .await?
        } else {
            hardware_entropy
        };

        // Step 4: Classify and create result using the actual entropy path
        let (quality_tier, quality_score, device_used, source, hardware_backed) =
            if source_report.hardware_backed {
                let tier = self.calculate_quality_tier(&hsm_source, mixed_entropy.len());
                (
                    tier,
                    self.calculate_quality_score(tier),
                    self.get_device_name(&hsm_source),
                    source_report.source.to_string(),
                    true,
                )
            } else {
                (
                    source_report.quality_tier,
                    source_report.quality_score,
                    source_report.device_used.to_string(),
                    source_report.source.to_string(),
                    false,
                )
            };

        // Opaque seed identifier for this generation session (UUID v4).
        let seed_id = Uuid::new_v4();

        info!(
            "✅ Generated entropy seed {} (source {}, tier {}, quality {:.2}, hardware_backed {})",
            seed_id, source, quality_tier, quality_score, hardware_backed
        );

        Ok(EntropyGenerationResult {
            seed_id,
            quality_tier,
            quality_score,
            device_used,
            source,
            hardware_backed,
            timestamp: chrono::Utc::now(),
        })
    }

    /// Generate entropy from specific HSM source
    ///
    /// **Current behavior:** Always uses the OS CSPRNG fallback and returns honest
    /// software-only metadata. Future Phase 2 work will call FIDO2/TPM/StrongBox
    /// hardware RNG when the selected provider is wired.
    pub(super) async fn generate_from_hsm(
        &self,
        source: &HsmSource,
        length: usize,
    ) -> Result<(Vec<u8>, EntropySourceReport), BearDogError> {
        use rand::RngCore;

        warn!(
            "Hardware entropy requested from {:?} but using OS RNG fallback (source={:?}); \
             FIDO2/TPM/StrongBox providers not yet wired",
            source, OS_RNG_SOURCE
        );
        let mut rng = rand::rng();
        let mut entropy = vec![0u8; length];
        rng.fill_bytes(&mut entropy);

        debug!(
            "Generated {} bytes of entropy via OS RNG fallback ({})",
            length,
            super::config::OS_RNG_FALLBACK_DEVICE
        );
        Ok((entropy, os_rng_fallback_report()))
    }
}
