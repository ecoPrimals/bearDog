// SPDX-License-Identifier: AGPL-3.0-or-later

// iOS HSM Provider with Secure Enclave Support  
// CRITICAL: NO SIMULATED ENTROPY ALLOWED FOR HUMAN KEYS

use super::traits::{HumanEntropyData, HumanEntropyMethod, HumanEntropyProvider};
use crate::universal_hsm::entropy::live_feed_validator::LiveFeedValidator;
use beardog_errors::BearDogError;
use std::collections::HashMap;

pub struct IosHsmProvider {
    live_feed_validator: LiveFeedValidator,
}

impl IosHsmProvider {
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            live_feed_validator: LiveFeedValidator::new(),
        }
    }
}

impl HumanEntropyProvider for IosHsmProvider {
    fn collect_human_entropy(
        &self,
        method: &HumanEntropyMethod,
        bytes_needed: u32,
    ) -> Result<HumanEntropyData, BearDogError> {
        let start_time = std::time::Instant::now();

        // CRITICAL: ALL human entropy MUST come from live feed sources
        let entropy_bytes = match method {
            HumanEntropyMethod::TouchInteraction => {
                self.collect_live_touch_entropy(bytes_needed)?
            }
            HumanEntropyMethod::DeviceMovement => {
                self.collect_live_motion_entropy(bytes_needed)?
            }
            HumanEntropyMethod::BiometricVariation => {
                self.collect_live_biometric_entropy(bytes_needed)?
            }
            _ => {
                return Err(BearDogError::security(format!(
                    "iOS entropy method not supported: {:?}",
                    method
                )));
            }
        };

        // MANDATORY: Validate that entropy is from live feed only
        let mut source_metadata = HashMap::new();
        source_metadata.insert("platform".to_string(), "ios".to_string());
        source_metadata.insert("method".to_string(), format!("{:?}", method));
        source_metadata.insert(
            "collection_timestamp".to_string(),
            chrono::Utc::now().timestamp().to_string(),
        );
        source_metadata.insert(
            "anti_replay_nonce".to_string(),
            uuid::Uuid::new_v4().to_string(),
        );
        source_metadata.insert("sequence_number".to_string(), "1".to_string());

        let validation_result = self
            .live_feed_validator
            .validate_live_feed_only(&entropy_bytes, &source_metadata)
            ?;

        if !validation_result.is_live {
            return Err(BearDogError::security(
                "CRITICAL SECURITY VIOLATION: Non-live entropy detected for human key creation",
            ));
        }

        let collection_duration_ms = start_time.elapsed().as_millis() as u64;
        let estimated_entropy_bits =
            (entropy_bytes.len() * 8) as f64 * validation_result.feed_quality;
        let quality_score = validation_result.feed_quality;

        let entropy_data = HumanEntropyData::new(
            entropy_bytes,
            method.clone(),
            estimated_entropy_bits,
            quality_score,
            collection_duration_ms,
        );

        tracing::info!(
            "✅ iOS LIVE entropy collected: {:.1} bits (quality: {:.2})",
            estimated_entropy_bits,
            quality_score
        );

        Ok(entropy_data)
    }
}

impl IosHsmProvider {
    /// Collect live touch entropy from iOS sensors
    fn collect_live_touch_entropy(&self, bytes_needed: u32) -> Result<Vec<u8>, BearDogError> {
        #[cfg(target_os = "ios")]
        {
            // Real iOS implementation would collect from actual touch sensors
            return Err(BearDogError::security(
                "LIVE TOUCH ENTROPY REQUIRED: Must use actual iOS touch sensors, not simulation",
            ));
        }

        #[cfg(not(target_os = "ios"))]
        {
            Err(BearDogError::security(
                "SECURITY: iOS touch entropy only available on actual iOS devices with live sensors"
            ))
        }
    }

    /// Collect live motion entropy from iOS sensors
    fn collect_live_motion_entropy(&self, bytes_needed: u32) -> Result<Vec<u8>, BearDogError> {
        #[cfg(target_os = "ios")]
        {
            // Real iOS implementation would collect from actual motion sensors
            return Err(BearDogError::security(
                "LIVE MOTION ENTROPY REQUIRED: Must use actual iOS motion sensors, not simulation",
            ));
        }

        #[cfg(not(target_os = "ios"))]
        {
            Err(BearDogError::security(
                "SECURITY: iOS motion entropy only available on actual iOS devices with live sensors"
            ))
        }
    }

    /// Collect live biometric entropy from iOS sensors
    fn collect_live_biometric_entropy(&self, bytes_needed: u32) -> Result<Vec<u8>, BearDogError> {
        #[cfg(target_os = "ios")]
        {
            // Real iOS implementation would collect from actual biometric sensors
            return Err(BearDogError::security(
                "LIVE BIOMETRIC ENTROPY REQUIRED: Must use actual iOS biometric sensors, not simulation"
            ));
        }

        #[cfg(not(target_os = "ios"))]
        {
            Err(BearDogError::security(
                "SECURITY: iOS biometric entropy only available on actual iOS devices with live sensors"
            ))
        }
    }
}

impl Default for IosHsmProvider {
    fn default() -> Self {
        Self::new()
    }
}
