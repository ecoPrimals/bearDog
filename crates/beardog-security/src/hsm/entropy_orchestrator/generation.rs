// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy generation pipeline.

use super::discovery::HsmSource;
use super::orchestrator::HsmEntropyOrchestrator;
use super::quality::{EntropySourceReport, os_rng_fallback_report};
use super::types::{EntropyGenerationRequest, EntropyGenerationResult};
use beardog_errors::BearDogError;
use tracing::{debug, info};
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
    pub fn generate_entropy(
        &mut self,
        request: EntropyGenerationRequest,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        info!("🌱 Generating human entropy ({} bytes)", request.length);

        // Step 1: Select best available HSM
        let hsm_source = self.select_best_hsm(&request)?;

        // Step 2: Generate entropy (currently OS RNG fallback until hardware is wired)
        let (hardware_entropy, source_report) =
            self.generate_from_hsm(&hsm_source, request.length)?;

        // Step 3: Mix with human input if provided
        let mixed_entropy = if let Some(human_input) = request.human_input {
            self.mix_with_human_input(hardware_entropy, human_input)?
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

    /// Generate entropy from the selected HSM source.
    ///
    /// When a FIDO2 device is selected, the async path (`generate_from_hsm_async`)
    /// should be preferred for actual hardware entropy. This sync fallback always uses
    /// OS RNG, since HID I/O is inherently async.
    pub(super) fn generate_from_hsm(
        &self,
        _source: &HsmSource,
        length: usize,
    ) -> Result<(Vec<u8>, EntropySourceReport), BearDogError> {

        #[cfg(feature = "fido2")]
        #[expect(irrefutable_let_patterns, reason = "HsmSource has cfg-conditional variants")]
        if let HsmSource::Fido2(idx) = _source
            && self.fido2_providers.get(*idx).is_some()
        {
            debug!(
                "FIDO2 device selected (index {idx}) — use generate_from_hsm_async \
                 for hardware entropy; falling back to OS RNG in sync context"
            );
        }

        let mut rng = rand::rng();
        let mut entropy = vec![0u8; length];
        rand::RngCore::fill_bytes(&mut rng, &mut entropy);

        debug!(
            "Generated {} bytes of entropy via OS RNG ({})",
            length,
            super::config::OS_RNG_FALLBACK_DEVICE
        );
        Ok((entropy, os_rng_fallback_report()))
    }

    /// Generate entropy from the selected HSM source (async path).
    ///
    /// Prefers hardware-backed entropy from FIDO2 when available, mixed with
    /// OS RNG for defense-in-depth. Falls back to pure OS RNG if hardware
    /// entropy fails.
    #[expect(dead_code, reason = "Async FIDO2 entropy path used when callers migrate to async")]
    pub(super) async fn generate_from_hsm_async(
        &self,
        source: &HsmSource,
        length: usize,
    ) -> Result<(Vec<u8>, EntropySourceReport), BearDogError> {
        #[cfg(feature = "fido2")]
        #[expect(irrefutable_let_patterns, reason = "HsmSource has cfg-conditional variants")]
        if let HsmSource::Fido2(idx) = source
            && let Some(provider) = self.fido2_providers.get(*idx)
        {
            match provider.hsm_provider().await {
                Ok(hsm) => match hsm.generate_entropy(length).await {
                    Ok(hw_entropy) => {
                        let mut os_rng = rand::rng();
                        let mut os_bytes = vec![0u8; length];
                        rand::RngCore::fill_bytes(&mut os_rng, &mut os_bytes);

                        let mut mixed = Vec::with_capacity(length);
                        for i in 0..hw_entropy.len().min(os_bytes.len()) {
                            mixed.push(hw_entropy[i] ^ os_bytes[i]);
                        }

                        info!(
                            "Generated {} bytes of FIDO2 + OS RNG mixed entropy",
                            length
                        );
                        return Ok((
                            mixed,
                            EntropySourceReport {
                                source: "fido2_hardware",
                                device_used: "fido2_device",
                                hardware_backed: true,
                                quality_tier: 2,
                                quality_score: 0.85,
                            },
                        ));
                    }
                    Err(e) => {
                        tracing::warn!("FIDO2 hardware entropy failed: {e}");
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to create FIDO2 HSM provider: {e}");
                }
            }
        }

        self.generate_from_hsm(source, length)
    }
}
