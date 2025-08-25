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


/// Human entropy collection system for genetic spawning
///
/// This module implements an ethical, privacy-preserving system for collecting
/// human-generated entropy from various sources including audio, visual, haptic,
/// and biometric inputs. The entropy is used to enhance the security and
/// unpredictability of genetic spawning operations.
/// All entropy collection operates under strict ethical guidelines with:
/// - Informed consent requirements
/// - Privacy protection by default
/// - Data minimization principles
/// - User control and transparency
/// - Immediate raw data deletion after feature extraction

pub mod collectors;
pub mod config;
pub mod ethics;
pub mod processors;
pub mod types;
// Re-export the main types and functions for easy access
pub use collectors::*;
pub use config::*;
pub use ethics::*;
pub use processors::*;
pub use types::*;
use chrono::Utc;
use std::time::Duration;
/// Create a default informed consent structure for testing
pub fn create_default_consent() -> InformedConsent {
    InformedConsent {
        collection_description: "Test entropy collection for development".to_string(),
        usage_description: "Used for testing genetic spawning entropy system".to_string(),
        retention_period: chrono::Duration::hours(1),
        withdrawal_rights: WithdrawalRights {
            can_withdraw: true,
            withdrawal_process: "Contact support to withdraw consent".to_string(),
            data_deletion_timeline: chrono::Duration::hours(24),
        },
        consent_timestamp: Utc::now(),
        consent_signature: DigitalSignature {
            signature_bytes: vec![0u8; 64],
            algorithm: "ed25519".to_string(),
            key_id: "test-key".to_string(),
    }
}
/// Create a default human entropy configuration for testing
pub fn create_default_config() -> HumanEntropyConfig {
    HumanEntropyConfig {
        collection_duration: Duration::from_secs(5),
        max_collection_attempts: 3,
        require_multimodal: false,
        min_quality_score: 0.5,
        audio_config: AudioConfig {
            enabled: true,
            sample_rate: 44100,
            bit_depth: 16,
            privacy_filter: "basic".to_string(),
        visual_config: VisualConfig {
            enabled: false,
            resolution: (640, 480),
            fps: 30,
        haptic_config: HapticConfig {
            touch_sensitivity: "medium".to_string(),
            motion_sensitivity: "medium".to_string(),
        biometric_config: BiometricConfig {
            require_explicit_consent: true,
            privacy_protection: "maximum".to_string(),
