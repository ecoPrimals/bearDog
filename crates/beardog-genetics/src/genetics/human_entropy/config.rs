

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct HumanEntropyConfig {

    pub collection_duration: Duration,

    pub max_collection_attempts: u32,

    pub require_multimodal: bool,

    pub min_quality_score: f64,

    pub audio_config: AudioConfig,

    pub visual_config: VisualConfig,

    pub haptic_config: HapticConfig,

    pub biometric_config: BiometricConfig,
}

pub struct AudioConfig {
    pub enabled: bool,
    pub sample_rate: u32,
    pub bit_depth: u8,
    pub privacy_filter: String,
}

pub struct VisualConfig {
    pub resolution: (u32, u32),
    pub fps: u32,
}

pub struct HapticConfig {
    pub touch_sensitivity: String,
    pub motion_sensitivity: String,
}

pub struct BiometricConfig {
    pub require_explicit_consent: bool,
    pub privacy_protection: String,
}
