//! Data structures and types for human entropy collection
//!
//! This module contains all the data structures used to represent different
//! types of collected entropy and their associated features.

use super::ethics::{DigitalSignature, PrivacyMetadata};
use chrono::{DateTime, Utc};
use std::time::Duration;
use zeroize::Zeroize;

/// Secret bytes that are automatically zeroed on drop
///
/// This type provides secure handling of sensitive entropy data by automatically
/// zeroing the underlying bytes when the instance is dropped, preventing
/// sensitive data from remaining in memory.
#[derive(Debug, Clone)]
pub struct SecretBytes(Vec<u8>);

impl SecretBytes {
    /// Create a new SecretBytes instance
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    /// Get a reference to the underlying bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Get the length of the underlying bytes
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Check if the bytes are empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Drop for SecretBytes {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

/// Audio entropy data with extracted features
#[derive(Debug, Clone)]
pub struct AudioEntropy {
    /// Raw entropy bytes extracted from audio
    pub entropy_bytes: SecretBytes,
    /// Timestamp when entropy was collected
    pub collection_timestamp: DateTime<Utc>,
    /// Duration of audio collection in milliseconds
    pub duration_ms: u32,
    /// Sample rate used for collection
    pub sample_rate: u32,
    /// Bit depth used for collection
    pub bit_depth: u8,
    /// Extracted audio features
    pub entropy_features: AudioEntropyFeatures,
    /// Privacy protection metadata
    pub privacy_metadata: PrivacyMetadata,
    /// Reference to the user's consent
    pub consent_reference: DigitalSignature,
}

/// Features extracted from audio entropy
#[derive(Debug, Clone)]
pub struct AudioEntropyFeatures {
    /// Spectral analysis features
    pub spectral_features: SpectralFeatures,
    /// Temporal analysis features
    pub temporal_features: TemporalFeatures,
    /// Ambient environment features
    pub ambient_features: AmbientFeatures,
    /// Human-specific audio features
    pub human_features: HumanAudioFeatures,
    /// Uniqueness score of the audio sample
    pub uniqueness_score: f64,
    /// Irreproducibility score of the audio sample
    pub irreproducibility_score: f64,
}

/// Spectral analysis features
#[derive(Debug, Clone)]
pub struct SpectralFeatures {
    /// Frequency distribution across the spectrum
    pub frequency_distribution: Vec<f64>,
    /// Dominant frequency components
    pub dominant_frequencies: Vec<f64>,
    /// Spectral centroid (center of mass of spectrum)
    pub spectral_centroid: f64,
    /// Spectral rolloff (frequency below which 85% of energy is contained)
    pub spectral_rolloff: f64,
    /// Spectral entropy (measure of spectral complexity)
    pub spectral_entropy: f64,
}

/// Temporal analysis features
#[derive(Debug, Clone)]
pub struct TemporalFeatures {
    /// Zero crossing rate (measure of signal changes)
    pub zero_crossing_rate: f64,
    /// Amplitude variance over time
    pub amplitude_variance: f64,
    /// Temporal pattern analysis
    pub temporal_patterns: Vec<f64>,
    /// Rhythm detection results
    pub rhythm_detection: Vec<f64>,
}

/// Ambient environment features
#[derive(Debug, Clone)]
pub struct AmbientFeatures {
    /// Background noise level
    pub background_noise_level: f64,
    /// Detected environmental sounds
    pub environmental_sounds: Vec<String>,
    /// Acoustic signature of the environment
    pub acoustic_signature: Vec<f64>,
}

/// Human-specific audio features
#[derive(Debug, Clone)]
pub struct HumanAudioFeatures {
    /// Voice activity detection results
    pub voice_activity: VoiceActivity,
    /// Breathing pattern analysis
    pub breathing_pattern: BreathingPattern,
    /// Micro-movement detection
    pub micro_movements: MicroMovements,
    /// Environmental interaction sounds
    pub environmental_interactions: EnvironmentalInteractions,
    /// Confidence that human is present
    pub human_presence_confidence: f64,
    /// Indicators of uniqueness in human behavior
    pub uniqueness_indicators: Vec<String>,
}

/// Voice activity detection results
#[derive(Debug, Clone)]
pub struct VoiceActivity {
    /// Whether voice activity was detected
    pub detected: bool,
    /// Confidence level of detection
    pub confidence: f64,
    /// Time intervals where speech was detected
    pub speech_segments: Vec<TimeInterval>,
    /// Vocal characteristics analysis
    pub vocal_characteristics: VocalCharacteristics,
}

/// Vocal characteristics analysis
#[derive(Debug, Clone)]
pub struct VocalCharacteristics {
    /// Fundamental frequency of voice
    pub fundamental_frequency: f64,
    /// Formant frequencies
    pub formant_frequencies: Vec<f64>,
    /// Spectral tilt of voice
    pub spectral_tilt: f64,
    /// Harmonic richness measure
    pub harmonic_richness: f64,
}

/// Breathing pattern analysis
#[derive(Debug, Clone)]
pub struct BreathingPattern {
    /// Whether breathing was detected
    pub detected: bool,
    /// Breathing rate (breaths per minute)
    pub rate: f64,
    /// Regularity of breathing rhythm
    pub rhythm_regularity: f64,
    /// Variation in breathing depth
    pub depth_variation: f64,
}

/// Micro-movement detection
#[derive(Debug, Clone)]
pub struct MicroMovements {
    /// Device handling sounds
    pub device_handling_sounds: Vec<f64>,
    /// Clothing rustling sounds
    pub clothing_rustling: Vec<f64>,
    /// Environmental interaction sounds
    pub environmental_interactions: Vec<f64>,
}

/// Environmental interaction sounds
#[derive(Debug, Clone)]
pub struct EnvironmentalInteractions {
    /// Keyboard typing detected
    pub keyboard_typing: bool,
    /// Mouse clicking detected
    pub mouse_clicking: bool,
    /// Paper rustling detected
    pub paper_rustling: bool,
    /// Footsteps detected
    pub footsteps: bool,
    /// Other detected sounds
    pub other_sounds: Vec<String>,
}

/// Time interval representation
#[derive(Debug, Clone)]
pub struct TimeInterval {
    /// Start time in milliseconds
    pub start_ms: u32,
    /// End time in milliseconds
    pub end_ms: u32,
}

/// Visual entropy data with extracted features
#[derive(Debug, Clone)]
pub struct VisualEntropy {
    /// Raw entropy bytes extracted from visual data
    pub entropy_bytes: SecretBytes,
    /// Timestamp when entropy was collected
    pub collection_timestamp: DateTime<Utc>,
    /// Duration of visual collection in milliseconds
    pub duration_ms: u32,
    /// Resolution used for collection
    pub resolution: (u32, u32),
    /// Frames per second used for collection
    pub fps: u32,
    /// Extracted visual features
    pub entropy_features: VisualEntropyFeatures,
    /// Privacy protection metadata
    pub privacy_metadata: PrivacyMetadata,
    /// Reference to the user's consent
    pub consent_reference: DigitalSignature,
}

/// Features extracted from visual entropy
#[derive(Debug, Clone)]
pub struct VisualEntropyFeatures {
    /// Lighting condition features
    pub lighting_features: LightingFeatures,
    /// Motion detection features
    pub motion_features: MotionFeatures,
    /// Texture analysis features
    pub texture_features: TextureFeatures,
    /// Human-specific visual features
    pub human_features: HumanVisualFeatures,
    /// Uniqueness score of the visual sample
    pub uniqueness_score: f64,
    /// Irreproducibility score of the visual sample
    pub irreproducibility_score: f64,
}

/// Lighting condition features
#[derive(Debug, Clone)]
pub struct LightingFeatures {
    /// Brightness variation across the image
    pub brightness_variation: f64,
    /// Color temperature of lighting
    pub color_temperature: f64,
    /// Shadow pattern analysis
    pub shadow_patterns: Vec<f64>,
    /// Lighting changes over time
    pub lighting_changes: Vec<f64>,
}

/// Motion detection features
#[derive(Debug, Clone)]
pub struct MotionFeatures {
    /// Optical flow vectors
    pub optical_flow: Vec<f64>,
    /// Motion vectors between frames
    pub motion_vectors: Vec<(f64, f64)>,
    /// Overall motion intensity
    pub motion_intensity: f64,
    /// Directional patterns in motion
    pub directional_patterns: Vec<f64>,
}

/// Texture analysis features
#[derive(Debug, Clone)]
pub struct TextureFeatures {
    /// Texture complexity measure
    pub texture_complexity: f64,
    /// Pattern analysis results
    pub pattern_analysis: Vec<f64>,
    /// Edge density measure
    pub edge_density: f64,
    /// Surface characteristics
    pub surface_characteristics: Vec<f64>,
}

/// Human-specific visual features
#[derive(Debug, Clone)]
pub struct HumanVisualFeatures {
    /// Entropy from eye movements
    pub eye_movement_entropy: f64,
    /// Entropy from micro-expressions
    pub micro_expression_entropy: f64,
    /// Entropy from hand movements
    pub hand_movement_entropy: f64,
    /// Entropy from head/body orientation
    pub orientation_entropy: f64,
    /// Confidence that human is present
    pub human_presence_confidence: f64,
    /// Indicators of uniqueness in human behavior
    pub uniqueness_indicators: Vec<String>,
}

/// Haptic entropy data with extracted features
#[derive(Debug, Clone)]
pub struct HapticEntropy {
    /// Raw entropy bytes extracted from haptic data
    pub entropy_bytes: SecretBytes,
    /// Timestamp when entropy was collected
    pub collection_timestamp: DateTime<Utc>,
    /// Duration of haptic collection in milliseconds
    pub duration_ms: u32,
    /// Extracted haptic features
    pub entropy_features: HapticEntropyFeatures,
    /// Reference to the user's consent
    pub consent_reference: DigitalSignature,
}

/// Features extracted from haptic entropy
#[derive(Debug, Clone)]
pub struct HapticEntropyFeatures {
    /// Touch interaction patterns
    pub touch_patterns: TouchPatterns,
    /// Device motion patterns
    pub motion_patterns: MotionPatterns,
    /// Pressure application patterns
    pub pressure_patterns: PressurePatterns,
    /// Human-specific haptic features
    pub human_features: HumanHapticFeatures,
    /// Uniqueness score of the haptic sample
    pub uniqueness_score: f64,
    /// Irreproducibility score of the haptic sample
    pub irreproducibility_score: f64,
}

/// Touch interaction patterns
#[derive(Debug, Clone)]
pub struct TouchPatterns {
    /// Individual touch points recorded
    pub touch_points: Vec<TouchPoint>,
    /// Duration of each touch interaction
    pub touch_duration: Vec<Duration>,
    /// Pressure applied during touches
    pub touch_pressure: Vec<f64>,
    /// Frequency of touch interactions
    pub touch_frequency: f64,
}

/// Individual touch point data
#[derive(Debug, Clone)]
pub struct TouchPoint {
    /// X coordinate of touch
    pub x: f64,
    /// Y coordinate of touch
    pub y: f64,
    /// Pressure applied at this point
    pub pressure: f64,
    /// Timestamp of this touch
    pub timestamp: DateTime<Utc>,
}

/// Device motion patterns
#[derive(Debug, Clone)]
pub struct MotionPatterns {
    /// Acceleration data (x, y, z)
    pub acceleration: Vec<(f64, f64, f64)>,
    /// Gyroscope data (x, y, z)
    pub gyroscope: Vec<(f64, f64, f64)>,
    /// Magnetometer data (x, y, z)
    pub magnetometer: Vec<(f64, f64, f64)>,
    /// Device orientation changes
    pub device_orientation: Vec<f64>,
}

/// Pressure application patterns
#[derive(Debug, Clone)]
pub struct PressurePatterns {
    /// Distribution of pressure across touches
    pub pressure_distribution: Vec<f64>,
    /// Changes in pressure over time
    pub pressure_changes: Vec<f64>,
    /// Force patterns during interaction
    pub force_patterns: Vec<f64>,
}

/// Human-specific haptic features
#[derive(Debug, Clone)]
pub struct HumanHapticFeatures {
    /// Natural hand tremor patterns
    pub tremor_patterns: TremorPatterns,
    /// Rhythmic interaction patterns
    pub rhythm_patterns: RhythmPatterns,
    /// Device handling characteristics
    pub handling_patterns: HandlingPatterns,
    /// Pressure application patterns
    pub pressure_patterns: Vec<f64>,
    /// Consistency of human behavior
    pub human_consistency: f64,
    /// Indicators of uniqueness in human behavior
    pub uniqueness_indicators: Vec<String>,
}

/// Tremor pattern analysis
#[derive(Debug, Clone)]
pub struct TremorPatterns {
    /// Frequency of tremor oscillations
    pub tremor_frequency: f64,
    /// Amplitude of tremor oscillations
    pub tremor_amplitude: f64,
    /// Regularity of tremor patterns
    pub tremor_regularity: f64,
}

/// Rhythm pattern analysis
#[derive(Debug, Clone)]
pub struct RhythmPatterns {
    /// Typing rhythm patterns
    pub typing_rhythm: Vec<f64>,
    /// Tapping rhythm patterns
    pub tapping_rhythm: Vec<f64>,
    /// Gesture rhythm patterns
    pub gesture_rhythm: Vec<f64>,
}

/// Device handling patterns
#[derive(Debug, Clone)]
pub struct HandlingPatterns {
    /// How the device is gripped
    pub grip_patterns: Vec<f64>,
    /// Movement patterns while handling
    pub movement_patterns: Vec<f64>,
    /// Orientation changes during handling
    pub orientation_changes: Vec<f64>,
}

/// Biometric entropy data with extracted features
#[derive(Debug, Clone)]
pub struct BiometricEntropy {
    /// Raw entropy bytes extracted from biometric data
    pub entropy_bytes: SecretBytes,
    /// Type of biometric data collected
    pub biometric_type: BiometricType,
    /// Timestamp when entropy was collected
    pub collection_timestamp: DateTime<Utc>,
    /// Privacy-protected feature extraction
    pub privacy_protected_features: PrivacyProtectedFeatures,
    /// Reference to the user's consent
    pub consent_reference: DigitalSignature,
}

/// Types of biometric data
#[derive(Debug, Clone)]
pub enum BiometricType {
    /// Fingerprint biometric data
    Fingerprint,
    /// Facial geometry biometric data
    FaceGeometry,
    /// Voice print biometric data
    VoicePrint,
    /// Iris pattern biometric data
    IrisPattern,
    /// Heart rate variability biometric data
    HeartRateVariability,
    /// Gait pattern biometric data
    GaitPattern,
}

/// Privacy-protected features from biometric data
#[derive(Debug, Clone)]
pub enum PrivacyProtectedFeatures {
    /// Fingerprint features with privacy protection
    Fingerprint {
        /// Variance in ridge density
        ridge_density_variance: f64,
        /// Distribution of minutiae points
        minutiae_distribution: Vec<f64>,
        /// Complexity of pattern
        pattern_complexity: f64,
        /// Uniqueness score
        uniqueness_score: f64,
    },
    /// Voice print features with privacy protection
    VoicePrint {
        /// Spectral variance characteristics
        spectral_variance: f64,
        /// Formant stability measures
        formant_stability: f64,
        /// Pitch uniqueness characteristics
        pitch_uniqueness: f64,
        /// Vocal tract characteristics
        vocal_tract_characteristics: Vec<f64>,
    },
    /// Heart rate features with privacy protection
    HeartRate {
        /// Heart rate variability patterns
        hrv_patterns: Vec<f64>,
        /// Rhythm complexity measures
        rhythm_complexity: f64,
        /// Variability score
        variability_score: f64,
    },
}

/// Entropy quality assessment
#[derive(Debug, Clone)]
pub struct EntropyQualityAssessment {
    /// Overall quality score (0.0 to 1.0)
    pub overall_score: f64,
    /// Uniqueness score (0.0 to 1.0)
    pub uniqueness_score: f64,
    /// Irreproducibility score (0.0 to 1.0)
    pub irreproducibility_score: f64,
    /// Multimodal fusion score (0.0 to 1.0)
    pub multimodal_score: f64,
    /// Confidence that human is present (0.0 to 1.0)
    pub human_presence_confidence: f64,
    /// Quality classification
    pub quality_classification: QualityClassification,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
}

/// Quality classification levels
#[derive(Debug, Clone)]
pub enum QualityClassification {
    /// Excellent quality (>= 0.9)
    Excellent,
    /// Good quality (>= 0.7)
    Good,
    /// Acceptable quality (>= 0.5)
    Acceptable,
    /// Poor quality (>= 0.3)
    Poor,
    /// Insufficient quality (< 0.3)
    Insufficient,
}

/// Result of fused entropy from multiple sources
#[derive(Debug, Clone)]
pub struct FusedEntropyResult {
    /// Final fused entropy bytes
    pub entropy_bytes: SecretBytes,
    /// Number of entropy sources used
    pub component_count: usize,
    /// When the fusion was performed
    pub fusion_timestamp: DateTime<Utc>,
    /// Types of sources that were fused
    pub source_types: Vec<String>,
    /// Quality of the fusion result
    pub fusion_quality: f64,
}
