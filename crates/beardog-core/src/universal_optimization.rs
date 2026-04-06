// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Optimization request types for universal optimization service
///
/// Defines different optimization strategies available through the ecosystem,
/// from genetic algorithms to performance acceleration and cryptographic optimization.
#[derive(Debug, Clone)]
pub enum OptimizationRequest {
    /// Genetic algorithm optimization for evolving solutions
    GeneticAlgorithm {
        /// Target for genetic optimization
        optimization_target: GeneticTarget,
        /// Quality requirements for solutions
        quality_requirements: GeneticQualityRequirements,
    },
    /// Performance acceleration optimization for workloads
    PerformanceAcceleration {
        /// Type of workload to optimize
        workload_type: WorkloadType,
        /// Current performance metrics baseline
        current_metrics: PerformanceMetrics,
        /// Target improvement ratio (0.0 to 1.0)
        target_improvement: f64,
    },
    /// Cryptographic algorithm optimization
    CryptographicOptimization {
        /// Type of cryptographic algorithm
        algorithm_type: CryptoAlgorithmType,
        /// Security requirements to maintain
        security_requirements: SecurityRequirements,
        /// Performance constraints to meet
        performance_constraints: CryptoPerformanceConstraints,
    },
    /// Gaming workload optimization
    GamingOptimization {
        /// Type of game workload
        game_type: GameType,
        /// Latency requirements
        latency_requirements: LatencyRequirements,
        /// Throughput requirements
        throughput_requirements: ThroughputRequirements,
    },
    /// Machine learning model optimization
    MLOptimization {
        /// Type of ML model
        model_type: MLModelType,
        /// Size of training dataset
        training_data_size: u64,
        /// Target accuracy to achieve
        accuracy_target: f64,
    },
}

/// Genetic algorithm optimization targets and convergence criteria
///
/// Defines parameters for evolutionary optimization processes, including
/// generation limits and convergence thresholds for termination.
#[derive(Debug, Clone)]
pub struct GeneticTarget {
    /// Number of `max_generations`
    pub max_generations: u32,
    /// The convergence threshold value
    pub convergence_threshold: f64,
}

/// Performance metrics for monitoring and optimization
///
/// Tracks key performance indicators including throughput, resource utilization,
/// and error rates across system operations.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// The throughput ops per sec value
    pub throughput_ops_per_sec: f64,
    /// The cpu utilization value
    pub cpu_utilization: f64,
    /// The memory utilization value
    pub memory_utilization: f64,
    /// The network utilization value
    pub network_utilization: f64,
    /// The error rate value
    pub error_rate: f64,
}

/// Security requirements for cryptographic operations
///
/// Defines security constraints including quantum resistance and performance
/// priorities for balancing security with operational efficiency.
#[derive(Debug, Clone)]
pub struct SecurityRequirements {
    /// Whether `quantum_resistant` is enabled
    pub quantum_resistant: bool,
    /// Whether performance should be prioritized over maximum security
    pub performance_priority: bool,
}

/// Performance constraints for cryptographic operations
///
/// Specifies minimum throughput and maximum memory usage requirements
/// for cryptographic operation optimization.
#[derive(Debug, Clone)]
pub struct CryptoPerformanceConstraints {
    /// The min throughput value
    pub min_throughput: f64,
    /// Number of `max_memory_usage`
    pub max_memory_usage: u64,
}

/// Latency requirements for network and rendering operations
///
/// Defines maximum acceptable latency thresholds for network communication
/// and rendering operations to ensure responsive performance.
#[derive(Debug, Clone)]
pub struct LatencyRequirements {
    /// The max network latency ms value
    pub max_network_latency_ms: f64,
    /// The max render latency ms value
    pub max_render_latency_ms: f64,
}

/// Throughput requirements for network and concurrent operations
///
/// Specifies minimum bandwidth and concurrent capacity requirements
/// for maintaining performance under load.
#[derive(Debug, Clone)]
pub struct ThroughputRequirements {
    /// Minimum network bandwidth in megabits per second
    pub min_network_bandwidth_mbps: u64,
    /// Number of `concurrent_players`
    pub concurrent_players: u32,
}

/// Response from an optimization request
///
/// Contains the results of an optimization operation, including success status,
/// improvement metrics, optimized parameters, and recommendations for further optimization.
#[derive(Debug, Clone)]
pub struct OptimizationResponse {
    /// Unique identifier for the optimization request
    pub request_id: Uuid,
    /// The optimization type value
    pub optimization_type: String,
    /// Whether success is enabled
    pub success: bool,
    /// The improvement factor value
    pub improvement_factor: f64, // Actual improvement achieved
    /// Mapping of optimized parameters
    pub optimized_parameters: HashMap<String, serde_json::Value>,
    /// Performance metrics after optimization
    pub performance_metrics: PerformanceMetrics,
    /// Collection of recommendations
    pub recommendations: Vec<OptimizationRecommendation>,
    /// The estimated duration value
    pub estimated_duration: Duration,
    /// Confidence score (0.0-1.0) in the optimization results
    pub confidence_score: f64,
}

/// Recommendation for further optimization
///
/// Provides actionable recommendations with expected benefits and
/// implementation effort estimates to guide optimization decisions.
#[derive(Debug, Clone)]
pub struct OptimizationRecommendation {
    /// The description value
    pub description: String,
    /// The expected benefit value
    pub expected_benefit: f64,
    /// Level of effort required to implement this recommendation
    pub implementation_effort: EffortLevel,
    /// The priority value
    pub priority: RecommendationPriority,
}

/// Universal optimization service interface
///
/// Provides optimization capabilities across the ecosystem, enabling
/// distributed optimization requests and capability discovery.
pub trait UniversalOptimizationService: Send + Sync {
    /// Request optimization from the service
    ///
    /// # Errors
    /// Returns error if optimization request fails
    fn request_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResponse, BearDogError>;

    /// Gets `available_capabilities`
    ///
    /// # Errors
    /// Returns error if capabilities cannot be retrieved
    fn get_available_capabilities(&self) -> Result<Vec<String>, BearDogError>;

    /// Checks `optimization_health`
    ///
    /// # Errors
    /// Returns an error if health check fails
    fn check_optimization_health(&self) -> Result<HashMap<String, String>, BearDogError>;
}

/// Ecosystem-wide optimization service implementation
///
/// Coordinates optimization requests across the ecosystem using service
/// discovery and fallback optimization strategies.
pub struct EcosystemOptimizationService<D, L> {
    #[expect(
        dead_code,
        reason = "Discovery service wired when distributed optimization is on"
    )]
    discovery_service: D,
    fallback_optimizer: L,
}

/// Local optimization fallback interface
///
/// Provides local optimization capabilities when distributed optimization
/// services are unavailable or unsuitable.
pub trait LocalOptimizer: Send + Sync {
    /// Perform genetic optimization locally
    ///
    /// # Errors
    /// Returns an error if genetic optimization fails
    fn local_genetic_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResult, BearDogError>;

    /// Perform performance optimization locally
    ///
    /// # Errors
    /// Returns an error if performance optimization fails
    fn local_performance_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResult, BearDogError>;

    /// Perform cryptographic optimization locally
    ///
    /// # Errors
    /// Returns an error if crypto optimization fails
    fn local_crypto_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResult, BearDogError>;
}

/// Result of a local optimization operation
///
/// Contains the improvement factor achieved and associated performance
/// metrics from a local optimization run.
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// The improvement factor value
    pub improvement_factor: f64,
    /// The metrics value
    pub metrics: PerformanceMetrics,
}

impl<D, L> EcosystemOptimizationService<D, L>
where
    L: LocalOptimizer,
{
    /// Creates a new instance
    pub const fn new(discovery_service: D, fallback_optimizer: L) -> Self {
        Self {
            discovery_service,
            fallback_optimizer,
        }
    }

    fn fallback_to_local_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResponse, BearDogError> {
        let start_time = Instant::now();
        let result = match &request {
            OptimizationRequest::GeneticAlgorithm { .. } => self
                .fallback_optimizer
                .local_genetic_optimization(request)?,
            OptimizationRequest::CryptographicOptimization { .. } => {
                self.fallback_optimizer.local_crypto_optimization(request)?
            }
            // All other cases use performance optimization
            _ => self
                .fallback_optimizer
                .local_performance_optimization(request)?,
        };

        Ok(OptimizationResponse {
            request_id: Uuid::new_v4(),
            optimization_type: "Local".to_string(),
            success: true,
            improvement_factor: result.improvement_factor,
            optimized_parameters: HashMap::new(),
            performance_metrics: result.metrics,
            recommendations: Vec::new(),
            estimated_duration: start_time.elapsed(),
            confidence_score: 0.8,
        })
    }
}

impl<D, L> UniversalOptimizationService for EcosystemOptimizationService<D, L>
where
    D: Send + Sync,
    L: LocalOptimizer + Send + Sync,
{
    fn request_optimization(
        &self,
        request: OptimizationRequest,
    ) -> Result<OptimizationResponse, BearDogError> {
        self.fallback_to_local_optimization(request)
    }

    /// Gets `available_capabilities`
    fn get_available_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![
            "Genetic".to_string(),
            "Performance".to_string(),
            "Cryptographic".to_string(),
        ])
    }

    fn check_optimization_health(&self) -> Result<HashMap<String, String>, BearDogError> {
        let mut health = HashMap::with_capacity(16);
        health.insert("status".to_string(), "healthy".to_string());
        health.insert("local_optimizer".to_string(), "available".to_string());
        Ok(health)
    }
}

/// Quality requirements for genetic algorithm optimization
///
/// Defines minimum fitness and diversity thresholds that solutions must meet
/// during genetic evolution to ensure solution quality and population diversity.
#[derive(Debug, Clone)]
pub struct GeneticQualityRequirements {
    /// Minimum fitness score required for solutions (0.0 to 1.0)
    pub min_fitness: f64,
    /// Minimum diversity threshold to prevent premature convergence (0.0 to 1.0)
    pub diversity_threshold: f64,
}

/// Type of workload being optimized
///
/// Classifies workload patterns to enable targeted optimization strategies.
#[derive(Debug, Clone, Copy)]
pub enum WorkloadType {
    /// CPU-intensive computational workload
    Computational,
    /// Network-intensive communication workload
    Network,
    /// Storage-intensive data workload
    Storage,
    /// Mixed workload with multiple components
    Mixed,
}

/// Cryptographic algorithm types for optimization
///
/// Supported cryptographic algorithms that can be optimized for performance.
#[derive(Debug, Clone, Copy)]
pub enum CryptoAlgorithmType {
    /// AES-256 symmetric encryption
    Aes256,
    /// `ChaCha20` stream cipher
    ChaCha20,
    /// Ed25519 digital signature algorithm
    Ed25519,
    /// X25519 key exchange algorithm
    X25519,
}

/// Game type classification for optimization
///
/// Categorizes games by type to apply appropriate optimization strategies.
#[derive(Debug, Clone, Copy)]
pub enum GameType {
    /// First-person shooter
    Fps,
    /// Real-time strategy
    Rts,
    /// Massively multiplayer online role-playing game
    Mmorpg,
    /// Casual game
    Casual,
}

/// Machine learning model types for optimization
///
/// Specifies ML model architectures that can be optimized.
#[derive(Debug, Clone, Copy)]
pub enum MLModelType {
    /// Neural network model
    NeuralNetwork,
    /// Decision tree model
    DecisionTree,
    /// Random forest ensemble
    RandomForest,
    /// Support vector machine
    Svm,
}

/// Implementation effort level for optimization recommendations
///
/// Indicates the complexity and time required to implement an optimization.
#[derive(Debug, Clone, Copy)]
pub enum EffortLevel {
    /// Low effort, quick to implement
    Low,
    /// Medium effort, moderate implementation time
    Medium,
    /// High effort, significant implementation work
    High,
}

/// Priority level for optimization recommendations
///
/// Ranks recommendations by importance and urgency.
#[derive(Debug, Clone, Copy)]
pub enum RecommendationPriority {
    /// Low priority recommendation
    Low,
    /// Medium priority recommendation
    Medium,
    /// High priority recommendation
    High,
    /// Critical priority requiring immediate action
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns fixed improvement factors and metrics (test double only).
    struct DeterministicLocalOptimizer;

    impl LocalOptimizer for DeterministicLocalOptimizer {
        fn local_genetic_optimization(
            &self,
            request: OptimizationRequest,
        ) -> Result<OptimizationResult, BearDogError> {
            let _ = request;
            Ok(OptimizationResult {
                improvement_factor: 1.5,
                metrics: PerformanceMetrics {
                    throughput_ops_per_sec: 100.0,
                    cpu_utilization: 50.0,
                    memory_utilization: 60.0,
                    network_utilization: 40.0,
                    error_rate: 0.01,
                },
            })
        }

        fn local_performance_optimization(
            &self,
            request: OptimizationRequest,
        ) -> Result<OptimizationResult, BearDogError> {
            let _ = request;
            Ok(OptimizationResult {
                improvement_factor: 1.2,
                metrics: PerformanceMetrics {
                    throughput_ops_per_sec: 80.0,
                    cpu_utilization: 45.0,
                    memory_utilization: 55.0,
                    network_utilization: 35.0,
                    error_rate: 0.02,
                },
            })
        }

        fn local_crypto_optimization(
            &self,
            request: OptimizationRequest,
        ) -> Result<OptimizationResult, BearDogError> {
            let _ = request;
            Ok(OptimizationResult {
                improvement_factor: 1.1,
                metrics: PerformanceMetrics {
                    throughput_ops_per_sec: 50.0,
                    cpu_utilization: 30.0,
                    memory_utilization: 40.0,
                    network_utilization: 20.0,
                    error_rate: 0.0,
                },
            })
        }
    }

    #[test]
    fn test_ecosystem_optimization_service_request() {
        let service: EcosystemOptimizationService<(), DeterministicLocalOptimizer> =
            EcosystemOptimizationService::new((), DeterministicLocalOptimizer);
        let request = OptimizationRequest::GeneticAlgorithm {
            optimization_target: GeneticTarget {
                max_generations: 100,
                convergence_threshold: 0.001,
            },
            quality_requirements: GeneticQualityRequirements {
                min_fitness: 0.8,
                diversity_threshold: 0.2,
            },
        };
        let result = service.request_optimization(request);
        assert!(result.is_ok());
        let response = result.expect("optimization response");
        assert!(response.success);
        assert!((response.improvement_factor - 1.5).abs() < 1e-10);
    }

    #[test]
    fn test_ecosystem_optimization_service_capabilities() {
        let service: EcosystemOptimizationService<(), DeterministicLocalOptimizer> =
            EcosystemOptimizationService::new((), DeterministicLocalOptimizer);
        let caps = service
            .get_available_capabilities()
            .expect("optimization capabilities");
        assert!(caps.contains(&"Genetic".to_string()));
        assert!(caps.contains(&"Performance".to_string()));
        assert!(caps.contains(&"Cryptographic".to_string()));
    }

    #[test]
    fn test_ecosystem_optimization_service_health() {
        let service: EcosystemOptimizationService<(), DeterministicLocalOptimizer> =
            EcosystemOptimizationService::new((), DeterministicLocalOptimizer);
        let health = service
            .check_optimization_health()
            .expect("optimization health");
        assert_eq!(health.get("status"), Some(&"healthy".to_string()));
    }

    #[test]
    fn test_optimization_request_variants() {
        let _ = WorkloadType::Computational;
        let _ = WorkloadType::Network;
        let _ = CryptoAlgorithmType::Aes256;
        let _ = GameType::Fps;
        let _ = MLModelType::NeuralNetwork;
        let _ = EffortLevel::Low;
        let _ = RecommendationPriority::Critical;
    }
}
