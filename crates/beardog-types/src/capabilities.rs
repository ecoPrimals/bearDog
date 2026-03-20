// SPDX-License-Identifier: AGPL-3.0-only

// Capability Types and Configurations
//
// This module provides capability-related types and configurations for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of capability
pub enum CapabilityType {
    /// Security capabilities
    Security(SecurityCapability),
    /// AI/ML capabilities
    AI(AICapability),
    /// Genetic algorithm capabilities
    Genetic(GeneticCapability),
    /// Custom capability
    Custom(String),
}

/// Security capability definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityCapability {
    /// Encryption and decryption
    Encryption {
        /// Supported encryption algorithms (e.g., AES, RSA, `ChaCha20`)
        algorithms: Vec<String>,
        /// Supported key sizes in bits
        key_sizes: Vec<usize>,
        /// Hardware acceleration support available
        hardware_acceleration: bool,
    },
    /// Authentication services
    Authentication {
        /// Available authentication methods
        methods: Vec<String>,
        /// Multi-factor authentication support
        mfa_support: bool,
        /// Biometric authentication support
        biometric_support: bool,
    },
    /// Threat detection
    ThreatDetection {
        /// Machine learning enabled
        ml_enabled: bool,
        /// Real-time processing capability
        real_time: bool,
        /// Minimum detector confidence (0.0–1.0) before raising alerts
        confidence_threshold: f64,
    },
}

/// AI capability definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AICapability {
    /// Machine Learning
    MachineLearning {
        /// Supported ML algorithms (e.g., neural networks, decision trees)
        algorithms: Vec<String>,
        /// Model training capability available
        training_support: bool,
        /// Model inference capability available
        inference_support: bool,
    },
    /// Natural Language Processing
    NLP {
        /// Supported natural languages (e.g., en, es, fr)
        languages: Vec<String>,
        /// Sentiment analysis capability
        sentiment_analysis: bool,
        /// Named entity extraction capability
        entity_extraction: bool,
    },
    /// Computer Vision
    Vision {
        /// Image classification and recognition
        image_recognition: bool,
        /// Object detection in images/video
        object_detection: bool,
        /// Facial recognition capability
        face_recognition: bool,
    },
}

/// Genetic capability definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneticCapability {
    /// Genetic optimization
    Optimization {
        /// Size of the genetic algorithm population
        population_size: usize,
        /// Rate of genetic mutations (0.0-1.0)
        mutation_rate: f64,
        /// Rate of genetic crossover (0.0-1.0)
        crossover_rate: f64,
    },
    /// Evolutionary algorithms
    Evolution {
        /// Adaptive parameter adjustment
        adaptive: bool,
        /// Multi-objective optimization support
        multi_objective: bool,
        /// Parallel processing capability
        parallel_processing: bool,
    },
    /// Genetic healing
    Healing {
        /// Automatic self-repair capability
        self_repair: bool,
        /// Adaptive recovery mechanisms
        adaptive_recovery: bool,
        /// Redundancy and failover management
        redundancy_management: bool,
    },
}

/// Capability metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMetadata {
    /// Human-readable capability name
    /// Name of the item
    pub name: String,
    /// Detailed description of the capability
    /// The description value
    pub description: String,
    /// Semantic version of the capability
    /// The version value
    pub version: String,
    /// Provider or vendor of this capability
    pub provider: String,
    /// List of capability dependencies
    /// Collection of dependencies
    pub dependencies: Vec<String>,
    /// The resource requirements value
    pub resource_requirements: ResourceRequirements,
    /// Key-value configuration parameters
    pub configuration: HashMap<String, String>,
}

/// CPU, memory, storage, GPU, and bandwidth needs for hosting a capability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Optional cpu cores
    pub cpu_cores: Option<f64>,
    /// Required memory in megabytes
    /// Optional memory mb
    pub memory_mb: Option<u64>,
    /// Required storage in gigabytes
    /// Optional storage gb
    pub storage_gb: Option<u64>,
    /// Whether GPU acceleration is required
    /// Whether `gpu_required` is enabled
    pub gpu_required: bool,
    /// Required network bandwidth in megabits per second
    pub network_bandwidth_mbps: Option<u64>,
}

/// Capability registry entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRegistration {
    /// The specific capability being registered
    /// The capability value
    pub capability: CapabilityType,
    /// Metadata describing the capability
    /// The metadata value
    pub metadata: CapabilityMetadata,
    /// Whether this capability is currently enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Priority level (higher numbers = higher priority)
    /// Number of priority
    pub priority: u32,
    /// Current health status of the capability
    /// Current status of the health
    pub health_status: CapabilityHealth,
}

/// Capability health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityHealth {
    /// Capability is fully operational
    Healthy,
    /// Reduced performance or partial feature set
    Degraded,
    /// Capability is not operational
    Unhealthy,
    /// Deliberately offline for upgrades or maintenance
    Maintenance,
}

/// `BearDog` AI architecture configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct BearDogAIArchitecture {
    /// Whether AI capabilities are enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// List of security ML capabilities
    /// Collection of security ml
    pub security_ml: Vec<SecurityMLCapability>,
    /// Enable ML-driven performance tuning of crypto and policy paths
    pub performance_optimization: bool,
    /// Adaptive learning capabilities
    /// Whether `adaptive_learning` is enabled
    pub adaptive_learning: bool,
    /// Federated learning support
    /// Whether `federated_learning` is enabled
    pub federated_learning: bool,
}

/// Security ML capability definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityMLCapability {
    /// Threat pattern recognition
    ThreatPatternML {
        /// Model family or checkpoint identifier
        model_type: String,
        /// Minimum score to treat a detection as positive
        confidence_threshold: f64,
        /// Frequency of model updates in hours
        update_frequency_hours: u32,
    },
    /// Behavioral anomaly detection
    BehavioralAnomalyML {
        /// Days of history used to establish a baseline
        baseline_period_days: u32,
        /// Detector sensitivity (higher = more alerts)
        sensitivity_level: f64,
        /// Online learning step size for model updates
        learning_rate: f64,
    },
    /// Cryptographic optimization
    CryptographicOptimizationML {
        /// Automatic algorithm selection based on context
        algorithm_selection: bool,
        /// Optimization of key rotation timing
        key_rotation_optimization: bool,
        /// Allow ML to suggest cipher suites or cost parameters
        performance_tuning: bool,
    },
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            cpu_cores: Some(1.0),
            memory_mb: Some(512),
            storage_gb: Some(1),
            gpu_required: false,
            network_bandwidth_mbps: Some(10),
        }
    }
}

impl Default for CapabilityMetadata {
    fn default() -> Self {
        Self {
            name: "unknown".to_string(),
            description: "Unknown capability".to_string(),
            version: "1.0.0".to_string(),
            provider: "beardog ".to_string(),
            dependencies: vec![],
            resource_requirements: ResourceRequirements::default(),
            configuration: HashMap::new(),
        }
    }
}

impl Default for BearDogAIArchitecture {
    fn default() -> Self {
        Self {
            enabled: false,
            security_ml: vec![
                SecurityMLCapability::ThreatPatternML {
                    model_type: "RandomForest".to_string(),
                    confidence_threshold: 0.85,
                    update_frequency_hours: 24,
                },
                SecurityMLCapability::BehavioralAnomalyML {
                    baseline_period_days: 30,
                    sensitivity_level: 0.7,
                    learning_rate: 0.01,
                },
            ],
            performance_optimization: true,
            adaptive_learning: false,
            federated_learning: false,
        }
    }
}

impl CapabilityType {
    /// Get the string representation of the capability type
    #[must_use]
    /// Returns as string
    pub fn as_string(&self) -> String {
        match self {
            Self::Security(_) => "security".to_string(),
            Self::AI(_) => "ai".to_string(),
            Self::Genetic(_) => "genetic".to_string(),
            Self::Custom(name) => format!("custom_{name}"),
        }
    }
}
