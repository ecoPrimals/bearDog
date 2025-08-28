use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    pub min_security_level: SecurityLevel,

    pub hardware_required: bool,

    pub attestation_required: bool,

    pub requires_touch_screen: bool,

    pub requires_pressure_sensitivity: bool,

    pub requires_motion_sensors: bool,

    pub requires_biometric_sensors: bool,

    pub requires_keyboard: bool,

    pub requires_environmental_sensors: bool,

    pub complexity_score: u8,

    pub required_algorithms: Vec<String>,

    pub required_key_sizes: Vec<u32>,

    pub required_operations: Vec<String>,

    pub min_operations_per_second: Option<f64>,

    pub max_latency_ms: Option<f64>,

    pub max_memory_usage_mb: Option<f64>,

    pub required_certifications: Vec<String>,

    pub audit_required: bool,

    pub data_residency: Option<String>,

    pub vendor_requirements: HashMap<String, serde_json::Value>,

    pub custom_requirements: HashMap<String, serde_json::Value>,
}
impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: SecurityLevel::Basic,
            hardware_required: false,
            attestation_required: false,
            requires_touch_screen: false,
            requires_pressure_sensitivity: false,
            requires_motion_sensors: false,
            requires_biometric_sensors: false,
            requires_keyboard: false,
            requires_environmental_sensors: false,
            complexity_score: 1,
            required_algorithms: vec!["AES-256".to_string()],
            required_key_sizes: vec![256],
            required_operations: vec!["encrypt".to_string(), "decrypt".to_string()],
            min_operations_per_second: None,
            max_latency_ms: None,
            max_memory_usage_mb: None,
            required_certifications: Vec::new(),
            audit_required: false,
            data_residency: None,
            vendor_requirements: HashMap::with_capacity(16),
            custom_requirements: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevel {
    Basic,

    Standard,

    High,

    Maximum,

    Custom {
        level: String,
        requirements: HashMap<String, String>,
    },
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalHsmCapabilities {
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,

    pub security_level: SecurityLevel,
    pub hardware_backed: bool,
    pub attestation_supported: bool,

    pub supported_algorithms: Vec<String>,
    pub supported_key_types: Vec<String>,
    pub supported_key_sizes: Vec<u32>,
    pub supported_operations: Vec<String>,

    pub human_entropy: HumanEntropyCapabilities,

    pub performance: PerformanceCapabilities,

    pub compliance: ComplianceCapabilities,

    pub vendor_capabilities: HashMap<String, serde_json::Value>,

    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    pub supports_human_entropy: bool,

    pub supports_ephemeral_seeds: bool,

    pub available_methods: Vec<EntropyCollectionMethod>,

    pub touch_capabilities: TouchCapabilities,

    pub motion_capabilities: MotionCapabilities,

    pub biometric_capabilities: BiometricCapabilities,

    pub environmental_capabilities: EnvironmentalCapabilities,

    pub entropy_quality_score: f64,
}

impl Default for HumanEntropyCapabilities {
    fn default() -> Self {
        Self {
            supports_human_entropy: false,
            supports_ephemeral_seeds: false,
            available_methods: Vec::new(),
            touch_capabilities: TouchCapabilities::default(),
            motion_capabilities: MotionCapabilities::default(),
            biometric_capabilities: BiometricCapabilities::default(),
            environmental_capabilities: EnvironmentalCapabilities::default(),
            entropy_quality_score: 0.0,
        }
    }
}

impl HumanEntropyCapabilities {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    KeyboardTiming,

    MouseMovement,

    TouchPatterns {
        pressure_sensitive: bool,
    },

    DeviceMotion,

    EnvironmentalSensors {
        sensor_types: Vec<String>,
    },

    BiometricVariation,

    Custom {
        method: String,
        parameters: HashMap<String, String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchCapabilities {
    pub available: bool,
    pub pressure_sensitive: bool,
    pub multi_touch: bool,
    pub max_touch_points: u32,
    pub resolution: Option<(u32, u32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MotionCapabilities {
    pub accelerometer: bool,

    pub gyroscope: bool,

    pub magnetometer: bool,

    pub precision_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiometricCapabilities {
    pub fingerprint: bool,

    pub face_recognition: bool,

    pub iris_scanning: bool,

    pub voice_recognition: bool,

    pub security_level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentalCapabilities {
    pub ambient_light: bool,
    pub temperature: bool,
    pub humidity: bool,
    pub pressure: bool,
    pub proximity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    pub operations_per_second: f64,
    pub concurrent_operations: u32,
    pub average_latency_ms: f64,
    pub memory_usage_mb: f64,
    pub cpu_utilization_percent: f64,
}

impl Default for PerformanceCapabilities {
    fn default() -> Self {
        Self {
            operations_per_second: 1000.0,
            concurrent_operations: 10,
            average_latency_ms: 10.0,
            memory_usage_mb: 100.0,
            cpu_utilization_percent: 10.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCapabilities {
    pub certifications: Vec<String>,
    pub audit_logging: bool,
    pub data_residency_control: bool,
    pub encryption_at_rest: bool,
    pub encryption_in_transit: bool,
    pub access_control: bool,
}

impl Default for ComplianceCapabilities {
    fn default() -> Self {
        Self {
            certifications: Vec::new(),
            audit_logging: false,
            data_residency_control: false,
            encryption_at_rest: true,
            encryption_in_transit: true,
            access_control: true,
        }
    }
}
