//! # AI Configuration Domain
//!
//! This module contains all AI/ML related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use crate::canonical::config::unified_trait::BearDogConfig;

/// **CONSOLIDATED AI CONFIGURATION** - Unifies all AI/ML related configs
///
/// Consolidates: `HybridIntelligenceConfig`, `MachineLearningConfig`, `TrainingConfig`,
/// `InferenceConfig`, `PredictionConfig`, `NeuralNetworkConfig`, `DecisionEngineConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(Default)]
pub struct ConsolidatedAiConfig {
    /// Enable AI/ML functionality
    pub enabled: bool,
    
    /// Hybrid intelligence configuration
    pub hybrid_intelligence: HybridIntelligenceConfig,
    
    /// Machine learning training configuration
    pub training: TrainingConfig,
    
    /// Model inference and serving configuration
    pub inference: InferenceConfig,
    
    /// Neural network architecture configuration
    pub neural_networks: NeuralNetworkConfig,
    
    /// Decision engine configuration
    pub decision_engine: DecisionEngineConfig,
    
    /// Model management and deployment
    pub model_management: ModelManagementConfig,
    
    /// AI performance optimization settings
    pub performance: AiPerformanceConfig,
    
    /// AI security and privacy settings
    pub security: AiSecurityConfig,
}

/// **HYBRID INTELLIGENCE CONFIGURATION** - Human-AI collaboration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HybridIntelligenceConfig {
    /// Enable hybrid intelligence mode
    pub enabled: bool,
    
    /// Human oversight level (0.0 = full automation, 1.0 = full human control)
    pub human_oversight_level: f64,
    
    /// Confidence threshold for automatic decisions
    pub auto_decision_threshold: f64,
    
    /// Enable human feedback learning
    pub feedback_learning: bool,
    
    /// Maximum time to wait for human input
    pub human_input_timeout: Duration,
}

/// **TRAINING CONFIGURATION** - ML model training settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TrainingConfig {
    /// Enable training mode
    pub enabled: bool,
    
    /// Training data sources
    pub data_sources: Vec<String>,
    
    /// Training batch size
    pub batch_size: usize,
    
    /// Learning rate
    pub learning_rate: f64,
    
    /// Number of training epochs
    pub epochs: u32,
    
    /// Validation split ratio
    pub validation_split: f64,
    
    /// Early stopping patience
    pub early_stopping_patience: u32,
    
    /// Model checkpoint frequency
    pub checkpoint_frequency: u32,
}

/// **INFERENCE CONFIGURATION** - Model serving and inference settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InferenceConfig {
    /// Enable inference mode
    pub enabled: bool,
    
    /// Model serving endpoints
    pub endpoints: Vec<String>,
    
    /// Maximum batch size for inference
    pub max_batch_size: usize,
    
    /// Inference timeout
    pub timeout: Duration,
    
    /// Enable model caching
    pub enable_caching: bool,
    
    /// Cache size limit
    pub cache_size_limit: usize,
    
    /// Enable GPU acceleration
    pub enable_gpu: bool,
}

/// **NEURAL NETWORK CONFIGURATION** - Network architecture settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NeuralNetworkConfig {
    /// Network architecture type
    pub architecture: NetworkArchitecture,
    
    /// Hidden layer sizes
    pub hidden_layers: Vec<usize>,
    
    /// Activation function
    pub activation: ActivationFunction,
    
    /// Dropout rate
    pub dropout_rate: f64,
    
    /// Regularization settings
    pub regularization: RegularizationConfig,
    
    /// Optimizer configuration
    pub optimizer: OptimizerConfig,
}

/// **DECISION ENGINE CONFIGURATION** - AI decision making settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DecisionEngineConfig {
    /// Enable decision engine
    pub enabled: bool,
    
    /// Decision strategies
    pub strategies: Vec<DecisionStrategy>,
    
    /// Confidence thresholds for different decision types
    pub confidence_thresholds: HashMap<String, f64>,
    
    /// Enable explainable AI
    pub enable_explainability: bool,
    
    /// Decision audit trail settings
    pub audit_trail: AuditTrailConfig,
}

/// **MODEL MANAGEMENT CONFIGURATION** - Model lifecycle management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelManagementConfig {
    /// Model registry endpoint
    pub registry_endpoint: String,
    
    /// Model versioning strategy
    pub versioning_strategy: VersioningStrategy,
    
    /// Automatic model deployment
    pub auto_deployment: bool,
    
    /// Model performance monitoring
    pub performance_monitoring: bool,
    
    /// Model drift detection
    pub drift_detection: DriftDetectionConfig,
    
    /// Model rollback settings
    pub rollback: RollbackConfig,
}

/// **AI PERFORMANCE CONFIGURATION** - Performance optimization settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiPerformanceConfig {
    /// Enable performance optimization
    pub enabled: bool,
    
    /// CPU thread pool size
    pub cpu_threads: usize,
    
    /// Memory limit for AI operations
    pub memory_limit_mb: usize,
    
    /// Enable mixed precision training
    pub mixed_precision: bool,
    
    /// Enable model quantization
    pub quantization: bool,
    
    /// Performance profiling settings
    pub profiling: ProfilingConfig,
}

/// **AI SECURITY CONFIGURATION** - AI security and privacy settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AiSecurityConfig {
    /// Enable AI security features
    pub enabled: bool,
    
    /// Data privacy settings
    pub privacy: PrivacyConfig,
    
    /// Model security settings
    pub model_security: ModelSecurityConfig,
    
    /// Adversarial attack protection
    pub adversarial_protection: bool,
    
    /// Differential privacy settings
    pub differential_privacy: DifferentialPrivacyConfig,
}

// Supporting types and enums

/// Neural network architecture types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkArchitecture {
    /// Dense/fully connected layers
    Dense,
    /// Convolutional neural network
    Convolutional,
    /// Recurrent neural network (RNN/LSTM/GRU)
    Recurrent,
    /// Transformer architecture
    Transformer,
    /// Custom architecture specification
    Custom(String),
}

/// Activation function types for neural networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivationFunction {
    /// Rectified Linear Unit
    ReLU,
    /// Sigmoid activation function
    Sigmoid,
    /// Hyperbolic tangent
    Tanh,
    /// Softmax activation function
    Softmax,
    /// Leaky Rectified Linear Unit
    LeakyReLU,
    /// Custom activation function
    Custom(String),
}

/// Regularization configuration for neural networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegularizationConfig {
    /// L1 regularization parameter
    pub l1_lambda: f64,
    /// L2 regularization parameter
    pub l2_lambda: f64,
    /// Whether to enable batch normalization
    pub enable_batch_norm: bool,
}

/// Configuration for neural network optimizers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OptimizerConfig {
    /// The type of optimizer to use
    pub optimizer_type: OptimizerType,
    /// Learning rate for gradient descent
    pub learning_rate: f64,
    /// Momentum factor for momentum-based optimizers
    pub momentum: f64,
    /// Weight decay coefficient for regularization
    pub weight_decay: f64,
}

/// Supported optimizer algorithms for neural network training
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizerType {
    /// Stochastic Gradient Descent
    SGD,
    /// Adaptive Moment Estimation
    Adam,
    /// Adam with decoupled weight decay
    AdamW,
    /// Root Mean Square Propagation
    RMSprop,
    /// Custom optimizer implementation
    Custom(String),
}

/// Decision-making strategy for AI systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionStrategy {
    /// Greedy algorithm - always chooses the locally optimal choice
    Greedy,
    /// Probabilistic approach using weighted random selection
    Probabilistic,
    /// Ensemble method combining multiple strategies
    Ensemble,
    /// Human-in-the-loop decision making
    HumanInTheLoop,
    /// Custom strategy with user-defined logic
    Custom(String),
}

/// Configuration for AI decision audit trails
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditTrailConfig {
    /// Whether audit trail logging is enabled
    pub enabled: bool,
    /// Number of days to retain audit trail data
    pub retention_days: u32,
    /// Whether to include intermediate decision steps in the audit trail
    pub include_intermediate_steps: bool,
}

/// Versioning strategy for AI models and configurations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersioningStrategy {
    /// Semantic versioning (major.minor.patch)
    Semantic,
    /// Timestamp-based versioning
    Timestamp,
    /// Hash-based versioning using content hash
    Hash,
    /// Custom versioning scheme
    Custom(String),
}

/// Configuration for AI model drift detection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DriftDetectionConfig {
    /// Whether drift detection is enabled
    pub enabled: bool,
    /// Time window for drift detection analysis
    pub detection_window: Duration,
    /// Threshold value for detecting significant drift
    pub threshold: f64,
    /// Whether to send alerts when drift is detected
    pub alert_on_drift: bool,
}

/// Configuration for AI model rollback functionality
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RollbackConfig {
    /// Whether rollback functionality is enabled
    pub enabled: bool,
    /// Whether to automatically rollback on performance degradation
    pub automatic_rollback: bool,
    /// Performance threshold below which to trigger rollback
    pub rollback_threshold: f64,
    /// Maximum number of previous versions to keep for rollback
    pub max_rollback_versions: u32,
}

/// Configuration for AI performance profiling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfilingConfig {
    /// Whether performance profiling is enabled
    pub enabled: bool,
    /// Sampling rate for profiling (0.0 to 1.0)
    pub sample_rate: f64,
    /// Whether to include memory profiling in addition to CPU profiling
    pub include_memory_profiling: bool,
    /// Whether to include GPU profiling for AI workloads
    pub include_gpu_profiling: bool,
}

/// Configuration for AI privacy and data protection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivacyConfig {
    /// Whether privacy protection features are enabled
    pub enabled: bool,
    /// Level of data anonymization to apply
    pub anonymization_level: AnonymizationLevel,
    /// Number of days to retain data before automatic deletion
    pub data_retention_days: u32,
    /// Whether to enable right-to-be-forgotten functionality
    pub enable_right_to_be_forgotten: bool,
}

/// Level of data anonymization for privacy protection
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnonymizationLevel {
    /// No anonymization applied
    None,
    /// Basic anonymization (remove direct identifiers)
    Basic,
    /// Advanced anonymization (statistical disclosure control)
    Advanced,
    /// Full anonymization (differential privacy)
    Full,
}

/// Configuration for AI model security features
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelSecurityConfig {
    /// Whether to encrypt AI models at rest and in transit
    pub enable_model_encryption: bool,
    /// Whether to use secure inference techniques
    pub enable_secure_inference: bool,
    /// Whether to use trusted execution environments for AI processing
    pub trusted_execution_environment: bool,
}

/// Configuration for differential privacy in AI systems
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DifferentialPrivacyConfig {
    /// Whether differential privacy is enabled
    pub enabled: bool,
    /// Privacy budget parameter (smaller = more private)
    pub epsilon: f64,
    /// Failure probability parameter for differential privacy
    pub delta: f64,
    /// Noise mechanism to use for differential privacy
    pub noise_mechanism: NoiseMechanism,
}

/// Noise mechanisms for differential privacy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoiseMechanism {
    /// Laplace noise mechanism
    Laplace,
    /// Gaussian noise mechanism  
    Gaussian,
    /// Exponential noise mechanism
    Exponential,
}

// Default implementations

impl Default for HybridIntelligenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            human_oversight_level: 0.5,
            auto_decision_threshold: 0.8,
            feedback_learning: true,
            human_input_timeout: Duration::from_secs(30),
        }
    }
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            data_sources: vec![],
            batch_size: 32,
            learning_rate: 0.001,
            epochs: 100,
            validation_split: 0.2,
            early_stopping_patience: 10,
            checkpoint_frequency: 10,
        }
    }
}

impl Default for InferenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoints: vec![],
            max_batch_size: 64,
            timeout: Duration::from_secs(30),
            enable_caching: true,
            cache_size_limit: 1000,
            enable_gpu: false,
        }
    }
}

impl Default for NeuralNetworkConfig {
    fn default() -> Self {
        Self {
            architecture: NetworkArchitecture::Dense,
            hidden_layers: vec![128, 64],
            activation: ActivationFunction::ReLU,
            dropout_rate: 0.1,
            regularization: RegularizationConfig::default(),
            optimizer: OptimizerConfig::default(),
        }
    }
}

impl Default for DecisionEngineConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            strategies: vec![DecisionStrategy::Greedy],
            confidence_thresholds: HashMap::new(),
            enable_explainability: true,
            audit_trail: AuditTrailConfig::default(),
        }
    }
}

impl Default for ModelManagementConfig {
    fn default() -> Self {
        use crate::constants::domains::network::defaults::DEFAULT_API_PORT;
        Self {
            registry_endpoint: format!("http://localhost:{DEFAULT_API_PORT}/models"),
            versioning_strategy: VersioningStrategy::Semantic,
            auto_deployment: false,
            performance_monitoring: true,
            drift_detection: DriftDetectionConfig::default(),
            rollback: RollbackConfig::default(),
        }
    }
}

impl Default for AiPerformanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cpu_threads: 4,
            memory_limit_mb: 2048,
            mixed_precision: false,
            quantization: false,
            profiling: ProfilingConfig::default(),
        }
    }
}

impl Default for AiSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            privacy: PrivacyConfig::default(),
            model_security: ModelSecurityConfig::default(),
            adversarial_protection: true,
            differential_privacy: DifferentialPrivacyConfig::default(),
        }
    }
}

impl Default for RegularizationConfig {
    fn default() -> Self {
        Self {
            l1_lambda: 0.0,
            l2_lambda: 0.001,
            enable_batch_norm: true,
        }
    }
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            optimizer_type: OptimizerType::Adam,
            learning_rate: 0.001,
            momentum: 0.9,
            weight_decay: 0.0001,
        }
    }
}

impl Default for AuditTrailConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 90,
            include_intermediate_steps: false,
        }
    }
}

impl Default for DriftDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            detection_window: Duration::from_secs(24 * 60 * 60),
            threshold: 0.1,
            alert_on_drift: true,
        }
    }
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            automatic_rollback: false,
            rollback_threshold: 0.05,
            max_rollback_versions: 5,
        }
    }
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sample_rate: 0.1,
            include_memory_profiling: true,
            include_gpu_profiling: false,
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            anonymization_level: AnonymizationLevel::Basic,
            data_retention_days: 365,
            enable_right_to_be_forgotten: true,
        }
    }
}

impl Default for ModelSecurityConfig {
    fn default() -> Self {
        Self {
            enable_model_encryption: true,
            enable_secure_inference: false,
            trusted_execution_environment: false,
        }
    }
}

impl Default for DifferentialPrivacyConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            epsilon: 1.0,
            delta: 1e-5,
            noise_mechanism: NoiseMechanism::Gaussian,
        }
    }
}

// BearDogConfig implementation for ConsolidatedAiConfig
impl BearDogConfig for ConsolidatedAiConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            if self.hybrid_intelligence.human_oversight_level < 0.0 || self.hybrid_intelligence.human_oversight_level > 1.0 {
                return Err(BearDogError::validation("Human oversight level must be between 0.0 and 1.0"));
            }
            
            if self.training.learning_rate <= 0.0 {
                return Err(BearDogError::validation("Learning rate must be positive"));
            }
            
            if self.training.batch_size == 0 {
                return Err(BearDogError::validation("Batch size must be greater than 0"));
            }
        }
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        Ok(Self {
            enabled: other.enabled,
            hybrid_intelligence: if other.hybrid_intelligence.enabled { 
                other.hybrid_intelligence.clone() 
            } else { 
                self.hybrid_intelligence.clone() 
            },
            training: if other.training.enabled { 
                other.training.clone() 
            } else { 
                self.training.clone() 
            },
            inference: if other.inference.enabled { 
                other.inference.clone() 
            } else { 
                self.inference.clone() 
            },
            neural_networks: other.neural_networks.clone(),
            decision_engine: if other.decision_engine.enabled { 
                other.decision_engine.clone() 
            } else { 
                self.decision_engine.clone() 
            },
            model_management: other.model_management.clone(),
            performance: if other.performance.enabled { 
                other.performance.clone() 
            } else { 
                self.performance.clone() 
            },
            security: if other.security.enabled { 
                other.security.clone() 
            } else { 
                self.security.clone() 
            },
        })
    }
    
    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();
        
        if let Ok(enabled) = std::env::var("BEARDOG_AI_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(false);
        }
        
        if let Ok(oversight) = std::env::var("BEARDOG_AI_HUMAN_OVERSIGHT") {
            config.hybrid_intelligence.human_oversight_level = oversight.parse().unwrap_or(0.5);
        }
        
        if let Ok(lr) = std::env::var("BEARDOG_AI_LEARNING_RATE") {
            config.training.learning_rate = lr.parse().unwrap_or(0.001);
        }
        
        config.validate()?;
        Ok(config)
    }
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize AI config to TOML: {e}")))
    }
    
    fn domain() -> &'static str {
        "ai"
    }
} 