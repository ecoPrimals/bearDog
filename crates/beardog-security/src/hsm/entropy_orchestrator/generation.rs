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
    /// Generate entropy (sync path, always OS RNG fallback).
    ///
    /// Prefer [`generate_entropy_async`] for hardware-backed entropy.
    ///
    /// # Errors
    ///
    /// Returns an error if HSM selection or mixing fails.
    pub fn generate_entropy(
        &mut self,
        request: EntropyGenerationRequest,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        let hsm_source = self.select_best_hsm(&request)?;
        let (hardware_entropy, source_report) =
            self.generate_from_hsm(&hsm_source, request.length)?;
        self.finalize_entropy(hsm_source, hardware_entropy, source_report, request.human_input)
    }

    /// Generate entropy (async path, hardware-backed when available).
    ///
    /// Attempts real hardware entropy from FIDO2/StrongBox/Secure Enclave
    /// before falling back to OS RNG.
    ///
    /// # Errors
    ///
    /// Returns an error if HSM selection or mixing fails.
    pub async fn generate_entropy_async(
        &mut self,
        request: EntropyGenerationRequest,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        let hsm_source = self.select_best_hsm(&request)?;
        let (hardware_entropy, source_report) = self
            .generate_from_hsm_async(&hsm_source, request.length)
            .await?;
        self.finalize_entropy(hsm_source, hardware_entropy, source_report, request.human_input)
    }

    fn finalize_entropy(
        &self,
        hsm_source: HsmSource,
        raw_entropy: Vec<u8>,
        source_report: EntropySourceReport,
        human_input: Option<super::types::HumanEntropyInput>,
    ) -> Result<EntropyGenerationResult, BearDogError> {
        let mixed_entropy = if let Some(input) = human_input {
            self.mix_with_human_input(raw_entropy, input)?
        } else {
            raw_entropy
        };

        let (quality_tier, quality_score, device_used, source, hardware_backed) =
            if source_report.hardware_backed {
                let tier = self.calculate_quality_tier(&hsm_source, mixed_entropy.len());
                let score = self.calculate_quality_score(tier);
                (
                    tier,
                    score,
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

    /// Sync fallback: always uses OS RNG since HID I/O is inherently async.
    pub(super) fn generate_from_hsm(
        &self,
        _source: &HsmSource,
        length: usize,
    ) -> Result<(Vec<u8>, EntropySourceReport), BearDogError> {
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

    /// Async path: attempts hardware entropy from FIDO2, mixed with OS RNG
    /// for defense-in-depth.  Falls back to pure OS RNG on failure.
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
                                quality_score: 0.75,
                            },
                        ));
                    }
                    Err(e) => {
                        tracing::warn!("FIDO2 hardware entropy failed, falling back to OS RNG: {e}");
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
