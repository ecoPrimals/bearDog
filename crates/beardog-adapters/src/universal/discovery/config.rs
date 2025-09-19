

use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
    pub mesh_config: ServiceMeshConfig,
    pub protocol_config: ProtocolTranslatorConfig,
    pub scaling_config: PredictiveScalingConfig,
    pub quantum_config: QuantumCommConfig,
}

#[derive(Debug, Clone)]
    /// The similarity threshold value
    pub similarity_threshold: f64,
    pub learning_config: LearningConfig,
    /// The cache ttl value
    pub cache_ttl: Duration,
}

#[derive(Debug, Clone)]
    /// The learning rate value
    pub learning_rate: f64,
    /// Number of batch_size
    pub batch_size: usize,
}

#[derive(Debug, Clone)]
    /// The optimization strategy value
    pub optimization_strategy: OptimizationStrategy,
    /// The health check interval value
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone)]
    /// Number of translation_cache_size
    pub translation_cache_size: usize,
    /// Number of max_translation_depth
    pub max_translation_depth: usize,
}

#[derive(Debug, Clone)]
    /// Mapping of scaling factors
    pub scaling_factors: HashMap<String, f64>,
    /// Number of min_instances
    pub min_instances: usize,
    /// Number of max_instances
    pub max_instances: usize,
}

#[derive(Debug, Clone)]
    pub quantum_channel_timeout: Duration,
    /// Whether fallback_to_classical is enabled
    pub fallback_to_classical: bool,
}

impl Default for AIMatcherConfig {
    fn default() -> Self {
        Self {
            model_path: std::env::var("BEARDOG_AI_MODEL_PATH")
                .unwrap_or_else(|_| "/opt/beardog/models/semantic.bin".to_string()),
            similarity_threshold: std::env::var("BEARDOG_SIMILARITY_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.8),
            learning_config: LearningConfig::default(),
            cache_ttl: Duration::from_secs(
                std::env::var("BEARDOG_AI_CACHE_TTL_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
        }
    }
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            enable_online_learning: std::env::var("BEARDOG_ENABLE_ONLINE_LEARNING")
                .map(|s| s.to_lowercase() == "true".to_string())
                .unwrap_or(false),
            learning_rate: std::env::var("BEARDOG_LEARNING_RATE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.001),
            batch_size: std::env::var("BEARDOG_LEARNING_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
        }
    }
}

impl Default for ServiceMeshConfig {
    fn default() -> Self {
        let mesh_type = std::env::var("BEARDOG_MESH_TYPE").unwrap_or_else(|_| "Custom".to_string());

        let mesh_type = match mesh_type.as_str() {
            "Istio" => MeshType::Istio,
            "Linkerd" => MeshType::Linkerd,
            "ConsulConnect" => MeshType::ConsulConnect,
            _ => MeshType::Custom,
        };

        Self {
            mesh_type,
            optimization_strategy: OptimizationStrategy::Balanced,
            health_check_interval: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}

impl Default for ProtocolTranslatorConfig {
    fn default() -> Self {
        let supported_protocols = std::env::var("BEARDOG_SUPPORTED_PROTOCOLS")
            .unwrap_or_else(|_| "http,grpc,mqtt,amqp".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        Self {
            supported_protocols,
            translation_cache_size: std::env::var("BEARDOG_TRANSLATION_CACHE_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000),
            max_translation_depth: std::env::var("BEARDOG_MAX_TRANSLATION_DEPTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }
}

impl Default for PredictiveScalingConfig {
    fn default() -> Self {
        Self {
            prediction_window: Duration::from_secs(
                std::env::var("BEARDOG_PREDICTION_WINDOW_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
            scaling_factors: HashMap::with_capacity(16),
            min_instances: std::env::var("BEARDOG_MIN_INSTANCES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            max_instances: std::env::var("BEARDOG_MAX_INSTANCES")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }
}

impl Default for QuantumCommConfig {
    fn default() -> Self {
        Self {
            enable_quantum_key_distribution: std::env::var("BEARDOG_ENABLE_QUANTUM_KD")
                .map(|s| s.to_lowercase() == "true".to_string())
                .unwrap_or(false),
            quantum_channel_timeout: Duration::from_secs(
                std::env::var("BEARDOG_QUANTUM_TIMEOUT_SECONDS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            fallback_to_classical: std::env::var("BEARDOG_QUANTUM_FALLBACK")
                .map(|s| s.to_lowercase() == "true".to_string())
                .unwrap_or(true),
        }
    }
}
