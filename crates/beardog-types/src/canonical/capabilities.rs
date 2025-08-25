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


/// # Universal Canonical Capabilities
///
/// **UNIVERSAL SINGLE SOURCE OF TRUTH** for all capability requirements and assessments
/// across the entire BearDog ecosystem.
/// This module supports:
/// - All HSM vendor capabilities (Thales, SafeNet, AWS, etc.)
/// - Human entropy collection and assessment
/// - Mobile HSM capabilities (iOS, Android)
/// - Software HSM capabilities
/// - Cloud HSM capabilities
/// - Hybrid and federated HSM architectures
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **UNIVERSAL CAPABILITY REQUIREMENTS** - Agnostic requirements specification
/// This structure supports capability requirements across all HSM types and vendors,
/// enabling universal compatibility and extensibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    // === CORE SECURITY REQUIREMENTS ===
    /// Minimum security level required
    pub min_security_level: SecurityLevel,
    /// Whether hardware-backed security is required
    pub hardware_required: bool,
    /// Whether hardware attestation is required
    pub attestation_required: bool,
    // === HUMAN ENTROPY REQUIREMENTS ===
    /// Whether touch screen interaction is required
    pub requires_touch_screen: bool,
    /// Whether pressure-sensitive touch is required
    pub requires_pressure_sensitivity: bool,
    /// Whether motion sensors are required (accelerometer, gyroscope)
    pub requires_motion_sensors: bool,
    /// Whether biometric sensors are required (fingerprint, face, etc.)
    pub requires_biometric_sensors: bool,
    /// Whether keyboard/typing interaction is required
    pub requires_keyboard: bool,
    /// Whether environmental sensors are required (ambient light, temperature, etc.)
    pub requires_environmental_sensors: bool,
    /// Complexity score for human entropy collection (0-10)
    pub complexity_score: u8,
    // === CRYPTOGRAPHIC REQUIREMENTS ===
    /// Required cryptographic algorithms
    pub required_algorithms: Vec<String>,
    /// Required key sizes
    pub required_key_sizes: Vec<u32>,
    /// Required cryptographic operations
    pub required_operations: Vec<String>,
    // === PERFORMANCE REQUIREMENTS ===
    /// Minimum operations per second
    pub min_operations_per_second: Option<f64>,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: Option<f64>,
    /// Maximum memory usage in MB
    pub max_memory_usage_mb: Option<f64>,
    // === COMPLIANCE REQUIREMENTS ===
    /// Required compliance certifications
    pub required_certifications: Vec<String>,
    /// Required audit capabilities
    pub audit_required: bool,
    /// Data residency requirements
    pub data_residency: Option<String>,
    // === VENDOR-SPECIFIC REQUIREMENTS ===
    /// Vendor-specific capability requirements (extensible)
    pub vendor_requirements: HashMap<String, serde_json::Value>,
    /// Custom capability requirements (fully extensible)
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
            vendor_requirements: HashMap::new(),
            custom_requirements: HashMap::new(),
        }
    }
}

/// **UNIVERSAL SECURITY LEVELS** - Cross-vendor security classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLevel {
    /// Basic security (software-based, development use)
    Basic,
    /// Standard security (hardware-backed recommended)
    Standard,
    /// High security (hardware-backed required)
    High,
    /// Maximum security (certified hardware required)
    Maximum,
    /// Custom security level with specific requirements
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
/// **UNIVERSAL HSM CAPABILITIES** - Comprehensive capability assessment
/// This structure provides a complete view of HSM capabilities across all vendors
/// and implementation types, enabling intelligent capability-based routing and selection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalHsmCapabilities {
    // === CORE HSM CAPABILITIES ===
    /// Basic HSM information
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    /// Security characteristics
    pub security_level: SecurityLevel,
    pub hardware_backed: bool,
    pub attestation_supported: bool,
    /// Cryptographic capabilities
    pub supported_algorithms: Vec<String>,
    pub supported_key_types: Vec<String>,
    pub supported_key_sizes: Vec<u32>,
    pub supported_operations: Vec<String>,
    // === HUMAN ENTROPY CAPABILITIES ===
    /// Human entropy collection capabilities
    pub human_entropy: HumanEntropyCapabilities,
    // === PERFORMANCE CAPABILITIES ===
    /// Performance characteristics
    pub performance: PerformanceCapabilities,
    // === COMPLIANCE CAPABILITIES ===
    /// Compliance and certification information
    pub compliance: ComplianceCapabilities,
    // === VENDOR-SPECIFIC CAPABILITIES ===
    /// Vendor-specific capabilities (extensible)
    pub vendor_capabilities: HashMap<String, serde_json::Value>,
    /// Custom capabilities (fully extensible)
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}

/// **HUMAN ENTROPY CAPABILITIES** - Human entropy collection assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    /// Whether human entropy collection is supported
    pub supports_human_entropy: bool,
    /// Whether ephemeral seed generation is supported
    pub supports_ephemeral_seeds: bool,
    /// Available entropy collection methods
    pub available_methods: Vec<EntropyCollectionMethod>,
    /// Touch screen capabilities
    pub touch_capabilities: TouchCapabilities,
    /// Motion sensor capabilities
    pub motion_capabilities: MotionCapabilities,
    /// Biometric capabilities
    pub biometric_capabilities: BiometricCapabilities,
    /// Environmental sensor capabilities
    pub environmental_capabilities: EnvironmentalCapabilities,
    /// Overall entropy quality score (0.0 - 1.0)
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
    /// Create new HumanEntropyCapabilities with default values
    pub fn new() -> Self {
        Self::default()
    }
}

/// **ENTROPY COLLECTION METHODS** - Universal entropy collection patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    /// Keystroke dynamics and typing patterns
    KeyboardTiming,
    /// Mouse movement patterns and click timing
    MouseMovement,
    /// Touch patterns on touchscreen devices
    TouchPatterns { pressure_sensitive: bool },
    /// Device motion sensors (accelerometer, gyroscope)
    DeviceMotion,
    /// Environmental sensors (ambient light, temperature, etc.)
    EnvironmentalSensors { sensor_types: Vec<String> },
    /// Biometric variation patterns
    BiometricVariation,
    /// Custom entropy collection method
    Custom {
        method: String,
        parameters: HashMap<String, String>,
    },
}

/// **TOUCH CAPABILITIES** - Touch screen and interaction capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TouchCapabilities {
    pub available: bool,
    pub pressure_sensitive: bool,
    pub multi_touch: bool,
    pub max_touch_points: u32,
    pub resolution: Option<(u32, u32)>,
}

/// **MOTION SENSOR CAPABILITIES** - Device motion and orientation sensing
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MotionCapabilities {
    /// Accelerometer availability
    pub accelerometer: bool,
    /// Gyroscope availability
    pub gyroscope: bool,
    /// Magnetometer availability
    pub magnetometer: bool,
    /// Motion sensor precision level
    pub precision_level: u8,
}

/// **BIOMETRIC CAPABILITIES** - Biometric sensor and authentication support
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BiometricCapabilities {
    /// Fingerprint sensor availability
    pub fingerprint: bool,
    /// Face recognition availability
    pub face_recognition: bool,
    /// Iris scanning availability
    pub iris_scanning: bool,
    /// Voice recognition availability
    pub voice_recognition: bool,
    /// Overall biometric security level
    pub security_level: u8,
}

/// **ENVIRONMENTAL CAPABILITIES** - Environmental sensor capabilities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentalCapabilities {
    pub ambient_light: bool,
    pub temperature: bool,
    pub humidity: bool,
    pub pressure: bool,
    pub proximity: bool,
}

/// **PERFORMANCE CAPABILITIES** - Performance characteristics
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

/// **COMPLIANCE CAPABILITIES** - Compliance and certification capabilities
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
