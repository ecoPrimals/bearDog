

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use std::time::Duration;

#[derive(Debug, Clone)]
    /// Number of max_collection_attempts
    pub max_collection_attempts: u32,

    /// Whether require_multimodal is enabled
    pub require_multimodal: bool,

    /// The min quality score value
    pub min_quality_score: f64,


    pub audio_config: AudioConfig,


    pub visual_config: VisualConfig,


    pub haptic_config: HapticConfig,


    pub biometric_config: BiometricConfig,
}

pub struct AudioConfig {
    /// Whether feature is enabled
    pub enabled: bool,
    /// Number of sample_rate
    pub sample_rate: u32,
    /// Number of bit_depth
    pub bit_depth: u8,
    /// The privacy filter value
    pub privacy_filter: String,
}

pub struct VisualConfig {
    /// The resolution value
    pub resolution: (u32, u32),
    /// Number of fps
    pub fps: u32,
}

pub struct HapticConfig {
    /// The touch sensitivity value
    pub touch_sensitivity: String,
    /// The motion sensitivity value
    pub motion_sensitivity: String,
}

pub struct BiometricConfig {
    /// Whether require_explicit_consent is enabled
    pub require_explicit_consent: bool,
    /// The privacy protection value
    pub privacy_protection: String,
}
