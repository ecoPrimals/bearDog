//! Configuration structures for human entropy collection
//!
//! This module defines all the configuration structures needed for different
//! types of entropy collection including audio, visual, haptic, and biometric.

use std::time::Duration;

/// Main configuration for human entropy collection
#[derive(Debug, Clone)]
pub struct HumanEntropyConfig {
    /// Duration for each entropy collection session
    pub collection_duration: Duration,
    /// Maximum number of collection attempts before giving up
    pub max_collection_attempts: u32,
    /// Whether to require multiple input modalities
    pub require_multimodal: bool,
    /// Minimum quality score to accept entropy
    pub min_quality_score: f64,
    /// Audio collection configuration
    pub audio_config: AudioConfig,
    /// Visual collection configuration
    pub visual_config: VisualConfig,
    /// Haptic collection configuration
    pub haptic_config: HapticConfig,
    /// Biometric collection configuration
    pub biometric_config: BiometricConfig,
}

/// Audio collection configuration
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Whether audio collection is enabled
    pub enabled: bool,
    /// Audio sampling rate in Hz
    pub sample_rate: u32,
    /// Audio bit depth
    pub bit_depth: u8,
    /// Privacy filter to apply to audio data
    pub privacy_filter: String,
}

/// Visual collection configuration
#[derive(Debug, Clone)]
pub struct VisualConfig {
    /// Whether visual collection is enabled
    pub enabled: bool,
    /// Video resolution (width, height)
    pub resolution: (u32, u32),
    /// Video frames per second
    pub fps: u32,
    /// Privacy filter to apply to visual data
    pub privacy_filter: String,
}

/// Haptic collection configuration
#[derive(Debug, Clone)]
pub struct HapticConfig {
    /// Whether haptic collection is enabled
    pub enabled: bool,
    /// Touch sensitivity settings
    pub touch_sensitivity: String,
    /// Motion sensitivity settings
    pub motion_sensitivity: String,
}

/// Biometric collection configuration
#[derive(Debug, Clone)]
pub struct BiometricConfig {
    /// Whether biometric collection is enabled
    pub enabled: bool,
    /// Whether explicit consent is required for biometric data
    pub require_explicit_consent: bool,
    /// Privacy protection level for biometric data
    pub privacy_protection: String,
} 