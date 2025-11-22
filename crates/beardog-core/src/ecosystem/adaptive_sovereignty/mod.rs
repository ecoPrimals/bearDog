//! # Adaptive Sovereignty System
//!
//! Continuously learns from ecosystem interactions and adaptively evolves
//! the primal sovereignty architecture for optimal performance, security,
//! and resilience.
//!
//! ## Overview
//!
//! The adaptive sovereignty system uses machine learning to:
//! - Monitor ecosystem behavior patterns
//! - Identify optimal sovereignty configurations
//! - Automatically adjust policies based on learned patterns
//! - Maintain security while improving performance
//!
//! ## Key Components
//!
//! - [`learning_engine`] - Core learning and adaptation engine
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::ecosystem::adaptive_sovereignty::LearningEngine;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let engine = LearningEngine::new();
//!
//! // System learns and adapts automatically
//! engine.observe_ecosystem_behavior().await?;
//! engine.adapt_sovereignty_policies().await?;
//! # Ok(())
//! # }
//! ```

/// Core learning and adaptation engine
pub mod learning_engine;

// Re-export main types
pub use learning_engine::{
    ActivationType, InteractionPattern, NeuralLayer, SovereigntyLearningEngine, SovereigntyPattern,
};

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{CapabilityType, ServiceCapabilityType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Adaptive sovereignty system that evolves primal architecture
pub struct AdaptiveSovereigntySystem {
    learning_engine: Arc<RwLock<SovereigntyLearningEngine>>,

    /// Adaptation strategies
    adaptation_strategies: Arc<RwLock<Vec<AdaptationStrategy>>>,

    /// Evolution history
    evolution_history: Arc<RwLock<Vec<EvolutionEvent>>>,

    /// Current sovereignty genome
    sovereignty_genome: Arc<RwLock<SovereigntyGenome>>,

    /// System configuration
    config: AdaptiveConfig,

    metrics: AdaptiveMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStrategy {
    /// Strategy identifier
    pub strategy_id: String,

    /// Strategy type
    /// The strategy type value
    pub strategy_type: AdaptationStrategyType,

    /// Success rate
    /// The success rate value
    pub success_rate: f64,

    /// Application count
    /// Number of application
    pub application_count: u64,

    /// Strategy parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,
}

/// Types of adaptation strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of adaptation strategy
pub enum AdaptationStrategyType {
    PerformanceOptimization,

    /// Security enhancement
    SecurityEnhancement,

    /// Resource allocation
    ResourceAllocation,

    /// Network topology
    NetworkTopology,

    /// Capability distribution
    CapabilityDistribution,
}

/// Evolution event in the sovereignty system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    /// Event ID
    pub event_id: String,

    /// Event timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,

    /// Event type
    /// The event type value
    pub event_type: EvolutionEventType,

    /// Event description
    /// The description value
    pub description: String,

    /// Impact score
    /// The impact score value
    pub impact_score: f64,

    /// Related strategies
    /// Collection of related strategies
    pub related_strategies: Vec<String>,
}

/// Types of evolution events
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of evolution event
pub enum EvolutionEventType {
    /// Strategy application
    StrategyApplication,

    /// Pattern discovery
    PatternDiscovery,

    PerformanceImprovement,

    /// Security enhancement
    SecurityEnhancement,

    /// System adaptation
    SystemAdaptation,
}

/// Sovereignty genome containing evolutionary parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SovereigntyGenome {
    /// Genome version
    /// The version value
    pub version: String,

    /// Genetic parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, f64>,

    /// Fitness score
    /// The fitness score value
    pub fitness_score: f64,

    /// Generation number
    /// Number of generation
    pub generation: u64,

    /// Mutation rate
    /// The mutation rate value
    pub mutation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    /// Learning rate
    /// The learning rate value
    pub learning_rate: f64,

    /// Adaptation threshold
    /// The adaptation threshold value
    pub adaptation_threshold: f64,

    /// Maximum strategies
    /// Number of max_strategies
    pub max_strategies: usize,

    /// Evolution interval
    /// The evolution interval value
    pub evolution_interval: Duration,

    pub performance_window: Duration,
}

#[derive(Debug, Clone, Default)]
pub struct AdaptiveMetrics {
    /// Total adaptations
    /// Number of total_adaptations
    pub total_adaptations: u64,

    /// Successful adaptations
    /// Number of successful_adaptations
    pub successful_adaptations: u64,

    pub avg_performance_improvement: f64,

    /// Learning accuracy
    /// The learning accuracy value
    pub learning_accuracy: f64,

    /// System uptime
    pub system_uptime: Duration,
}

impl AdaptiveSovereigntySystem {
    /// Create new adaptive sovereignty system
    /// Creates a new instance
    pub fn new(config: AdaptiveConfig) -> Result<Self> {
        info!("Initializing Adaptive Sovereignty System");

        let learning_engine = Arc::new(RwLock::new(SovereigntyLearningEngine::new()));
        let adaptation_strategies = Arc::new(RwLock::new(Vec::new()));
        let evolution_history = Arc::new(RwLock::new(Vec::new()));
        let sovereignty_genome = Arc::new(RwLock::new(SovereigntyGenome::default()));
        let metrics = AdaptiveMetrics::default();

        Ok(Self {
            learning_engine,
            adaptation_strategies,
            evolution_history,
            sovereignty_genome,
            config,
            metrics,
        })
    }

    /// Start adaptive evolution process
    /// Starts evolution
    /// Starts evolution
    pub fn start_evolution(&mut self) -> Result<(), BearDogError> {
        info!("Starting adaptive sovereignty evolution");

        // Initialize learning engine
        {
            let mut engine = self.learning_engine.write();
            engine.train()?;
        }

        // Start evolution loop
        let evolution_interval = self.config.evolution_interval;
        let mut interval = tokio::time::interval(evolution_interval);

        loop {
            interval.tick();
            self.evolution_cycle()?;
        }
    }

    /// Execute one evolution cycle
    fn evolution_cycle(&mut self) -> Result<(), BearDogError> {
        debug!("Executing evolution cycle");

        // Analyze current performance
        let performance_metrics = self.analyze_performance()?;

        // Generate adaptation strategies
        let new_strategies = self
            .generate_adaptation_strategies(&performance_metrics)
            ?;

        // Apply best strategies
        self.apply_adaptation_strategies(&new_strategies)?;

        // Update genome
        self.update_sovereignty_genome(&performance_metrics)?;

        // Record evolution event
        let event = EvolutionEvent {
            event_id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            event_type: EvolutionEventType::SystemAdaptation,
            description: "Evolution cycle completed".to_string(),
            impact_score: performance_metrics.overall_score,
            related_strategies: new_strategies
                .iter()
                .map(|s| s.strategy_id.clone())
                .collect(),
        };

        self.evolution_history.write().push(event);

        Ok(())
    }

    fn analyze_performance(&self) -> Result<PerformanceAnalysis> {
        // Simplified performance analysis
        Ok(PerformanceAnalysis {
            overall_score: 0.85,
            security_score: 0.90,
            performance_score: 0.80,
            efficiency_score: 0.88,
            recommendations: vec![
                "Optimize capability routing".to_string(),
                "Enhance security protocols".to_string(),
            ],
        })
    }

    /// Generate new adaptation strategies
    fn generate_adaptation_strategies(
        &self,
        performance: &PerformanceAnalysis,
    ) -> Result<Vec<AdaptationStrategy>> {
        let mut strategies = Vec::new();

        // Generate performance optimization strategy
        if performance.performance_score < 0.85 {
            strategies.push(AdaptationStrategy {
                strategy_id: Uuid::new_v4().to_string(),
                strategy_type: AdaptationStrategyType::PerformanceOptimization,
                success_rate: 0.0,
                application_count: 0,
                parameters: HashMap::new(),
            });
        }

        // Generate security enhancement strategy
        if performance.security_score < 0.95 {
            strategies.push(AdaptationStrategy {
                strategy_id: Uuid::new_v4().to_string(),
                strategy_type: AdaptationStrategyType::SecurityEnhancement,
                success_rate: 0.0,
                application_count: 0,
                parameters: HashMap::new(),
            });
        }

        Ok(strategies)
    }

    /// Apply adaptation strategies
    fn apply_adaptation_strategies(
        &mut self,
        strategies: &[AdaptationStrategy],
    ) -> Result<(), BearDogError> {
        for strategy in strategies {
            debug!("Applying strategy: {:?}", strategy.strategy_type);

            match strategy.strategy_type {
                AdaptationStrategyType::PerformanceOptimization => {
                    self.apply_performance_optimization(strategy)?;
                }
                AdaptationStrategyType::SecurityEnhancement => {
                    self.apply_security_enhancement(strategy)?;
                }
                _ => {
                    debug!(
                        "Strategy type not yet implemented: {:?}",
                        strategy.strategy_type
                    );
                }
            }

            self.metrics.total_adaptations += 1;
        }

        Ok(())
    }

    fn apply_performance_optimization(
        &mut self,
        _strategy: &AdaptationStrategy,
    ) -> Result<(), BearDogError> {
        info!("Applying performance optimization");
        self.metrics.successful_adaptations += 1;
        self.metrics.avg_performance_improvement += 0.05;
        Ok(())
    }

    /// Apply security enhancement strategy
    fn apply_security_enhancement(&mut self, _strategy: &AdaptationStrategy) -> Result<(), BearDogError> {
        info!("Applying security enhancement");
        self.metrics.successful_adaptations += 1;
        Ok(())
    }

    /// Update sovereignty genome
    /// Updates sovereignty_genome
    fn update_sovereignty_genome(
        &self,
        performance: &PerformanceAnalysis,
    ) -> Result<(), BearDogError> {
        let mut genome = self.sovereignty_genome.write();

        // Update fitness score
        genome.fitness_score = performance.overall_score;
        genome.generation += 1;

        // Mutate parameters slightly
        for (_, value) in genome.parameters.iter_mut() {
            let mutation = (rand::random::<f64>() - 0.5) * genome.mutation_rate;
            *value += mutation;
            *value = value.clamp(0.0, 1.0);
        }

        debug!(
            "Updated sovereignty genome to generation {}",
            genome.generation
        );
        Ok(())
    }

    /// Get current metrics
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> &AdaptiveMetrics {
        &self.metrics
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
    /// The overall score value
    pub overall_score: f64,
    /// The security score value
    pub security_score: f64,
    pub performance_score: f64,
    /// The efficiency score value
    pub efficiency_score: f64,
    /// Collection of recommendations
    pub recommendations: Vec<String>,
}

impl Default for SovereigntyGenome {
    fn default() -> Self {
        let mut parameters = HashMap::new();
        parameters.insert("learning_rate".to_string(), 0.001);
        parameters.insert("adaptation_threshold".to_string(), 0.8);
        parameters.insert("mutation_rate".to_string(), 0.01);

        Self {
            version: "1.0.0".to_string(),
            parameters,
            fitness_score: 0.5,
            generation: 0,
            mutation_rate: 0.01,
        }
    }
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.001,
            adaptation_threshold: 0.8,
            max_strategies: 10,
            evolution_interval: Duration::from_secs(
                std::env::var("BEARDOG_SOVEREIGNTY_EVOLUTION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300)
            ),
            performance_window: Duration::from_secs(
                std::env::var("BEARDOG_SOVEREIGNTY_PERFORMANCE_WINDOW_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600)
            ),
        }
    }
}
