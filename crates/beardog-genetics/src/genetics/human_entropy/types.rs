

use super::ethics::{DigitalSignature, PrivacyMetadata};
use chrono::{DateTime, Utc};
use std::time::Duration;
use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct SecretBytes(Vec<u8>);
impl SecretBytes {

    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0

    pub fn len(&self) -> usize {
        self.0.len()

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
}
impl Drop for SecretBytes {}

    fn drop(&mut self) {
        self.0.zeroize();

pub struct AudioEntropy {

    pub entropy_bytes: SecretBytes,

    pub collection_timestamp: DateTime<Utc>,

    pub duration_ms: u32,

    pub sample_rate: u32,

    pub bit_depth: u8,

    pub entropy_features: AudioEntropyFeatures,

    pub privacy_metadata: PrivacyMetadata,

    pub consent_reference: DigitalSignature,

pub struct AudioEntropyFeatures {

    pub spectral_features: SpectralFeatures,

    pub temporal_features: TemporalFeatures,

    pub ambient_features: AmbientFeatures,

    pub human_features: HumanAudioFeatures,

    pub uniqueness_score: f64,

    pub irreproducibility_score: f64,

pub struct SpectralFeatures {

    pub frequency_distribution: Vec<f64>,

    pub dominant_frequencies: Vec<f64>,

    pub spectral_centroid: f64,

    pub spectral_rolloff: f64,

    pub spectral_entropy: f64,

pub struct TemporalFeatures {

    pub zero_crossing_rate: f64,

    pub amplitude_variance: f64,

    pub temporal_patterns: Vec<f64>,

    pub rhythm_detection: Vec<f64>,

pub struct AmbientFeatures {

    pub background_noise_level: f64,

    pub environmental_sounds: Vec<String>,

    pub acoustic_signature: Vec<f64>,

pub struct HumanAudioFeatures {

    pub voice_activity: VoiceActivity,

    pub breathing_pattern: BreathingPattern,

    pub micro_movements: MicroMovements,

    pub environmental_interactions: EnvironmentalInteractions,

    pub human_presence_confidence: f64,

    pub uniqueness_indicators: Vec<String>,

pub struct VoiceActivity {

    pub detected: bool,

    pub confidence: f64,

    pub speech_segments: Vec<TimeInterval>,

    pub vocal_characteristics: VocalCharacteristics,

pub struct VocalCharacteristics {

    pub fundamental_frequency: f64,

    pub formant_frequencies: Vec<f64>,

    pub spectral_tilt: f64,

    pub harmonic_richness: f64,

pub struct BreathingPattern {

    pub rate: f64,

    pub rhythm_regularity: f64,

    pub depth_variation: f64,

pub struct MicroMovements {

    pub device_handling_sounds: Vec<f64>,

    pub clothing_rustling: Vec<f64>,
    pub environmental_interactions: Vec<f64>,

pub struct EnvironmentalInteractions {

    pub keyboard_typing: bool,

    pub mouse_clicking: bool,

    pub paper_rustling: bool,

    pub footsteps: bool,

    pub other_sounds: Vec<String>,

pub struct TimeInterval {

    pub start_ms: u32,

    pub end_ms: u32,

pub struct VisualEntropy {

    pub resolution: (u32, u32),

    pub fps: u32,

    pub entropy_features: VisualEntropyFeatures,

pub struct VisualEntropyFeatures {

    pub lighting_features: LightingFeatures,

    pub motion_features: MotionFeatures,

    pub texture_features: TextureFeatures,

    pub human_features: HumanVisualFeatures,

pub struct LightingFeatures {

    pub brightness_variation: f64,

    pub color_temperature: f64,

    pub shadow_patterns: Vec<f64>,

    pub lighting_changes: Vec<f64>,

pub struct MotionFeatures {

    pub optical_flow: Vec<f64>,

    pub motion_vectors: Vec<(f64, f64)>,

    pub motion_intensity: f64,

    pub directional_patterns: Vec<f64>,

pub struct TextureFeatures {

    pub texture_complexity: f64,

    pub pattern_analysis: Vec<f64>,

    pub edge_density: f64,

    pub surface_characteristics: Vec<f64>,

pub struct HumanVisualFeatures {

    pub eye_movement_entropy: f64,

    pub micro_expression_entropy: f64,

    pub hand_movement_entropy: f64,

    pub orientation_entropy: f64,

pub struct HapticEntropy {

    pub entropy_features: HapticEntropyFeatures,

pub struct HapticEntropyFeatures {

    pub touch_patterns: TouchPatterns,

    pub motion_patterns: MotionPatterns,

    pub pressure_patterns: PressurePatterns,

    pub human_features: HumanHapticFeatures,

pub struct TouchPatterns {

    pub touch_points: Vec<TouchPoint>,

    pub touch_duration: Vec<Duration>,

    pub touch_pressure: Vec<f64>,

    pub touch_frequency: f64,

pub struct TouchPoint {

    pub x: f64,

    pub y: f64,

    pub pressure: f64,

    pub timestamp: DateTime<Utc>,

pub struct MotionPatterns {

    pub acceleration: Vec<(f64, f64, f64)>,

    pub gyroscope: Vec<(f64, f64, f64)>,

    pub magnetometer: Vec<(f64, f64, f64)>,

    pub device_orientation: Vec<f64>,

pub struct PressurePatterns {

    pub pressure_distribution: Vec<f64>,

    pub pressure_changes: Vec<f64>,

    pub force_patterns: Vec<f64>,

pub struct HumanHapticFeatures {

    pub tremor_patterns: TremorPatterns,

    pub rhythm_patterns: RhythmPatterns,

    pub handling_patterns: HandlingPatterns,
    pub pressure_patterns: Vec<f64>,

    pub human_consistency: f64,

pub struct TremorPatterns {

    pub tremor_frequency: f64,

    pub tremor_amplitude: f64,

    pub tremor_regularity: f64,

pub struct RhythmPatterns {

    pub typing_rhythm: Vec<f64>,

    pub tapping_rhythm: Vec<f64>,

    pub gesture_rhythm: Vec<f64>,

pub struct HandlingPatterns {

    pub grip_patterns: Vec<f64>,

    pub movement_patterns: Vec<f64>,

    pub orientation_changes: Vec<f64>,

pub struct BiometricEntropy {

    pub biometric_type: BiometricType,

    pub privacy_protected_features: PrivacyProtectedFeatures,

pub enum BiometricType {

    Fingerprint,

    FaceGeometry,

    VoicePrint,

    IrisPattern,

    HeartRateVariability,

    GaitPattern,

pub enum PrivacyProtectedFeatures {

    Fingerprint {

        ridge_density_variance: f64,

        minutiae_distribution: Vec<f64>,

        pattern_complexity: f64,

        uniqueness_score: f64,
    },

    VoicePrint {

        spectral_variance: f64,

        formant_stability: f64,

        pitch_uniqueness: f64,

        vocal_tract_characteristics: Vec<f64>,

    HeartRate {

        hrv_patterns: Vec<f64>,

        rhythm_complexity: f64,

        variability_score: f64,

pub struct EntropyQualityAssessment {

    pub overall_score: f64,

    pub multimodal_score: f64,

    pub quality_classification: QualityClassification,

    pub recommendations: Vec<String>,

pub enum QualityClassification {

    Excellent,

    Good,

    Acceptable,

    Poor,

    Insufficient,

pub struct FusedEntropyResult {

    pub component_count: usize,

    pub fusion_timestamp: DateTime<Utc>,

    pub source_types: Vec<String>,

    pub fusion_quality: f64,
