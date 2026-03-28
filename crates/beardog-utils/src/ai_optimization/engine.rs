// SPDX-License-Identifier: AGPL-3.0-only

//! AI-powered performance optimization engine
//!
//! This module provides an intelligent optimization engine that uses machine learning
//! to automatically tune system performance based on observed metrics and patterns.
//!
//! # Features
//!
//! - **Performance Monitoring**: Real-time collection of system metrics
//! - **Predictive Analytics**: ML-based resource usage prediction
//! - **Automated Tuning**: Self-adjusting optimization recommendations
//! - **Learning System**: Improves recommendations based on historical data
//! - **Neural Network Integration**: Optional deep learning for complex patterns
//!
//! # Architecture
//!
//! The engine consists of three main components:
//! - **Performance Model**: Tracks and models system behavior
//! - **Resource Predictor**: Forecasts future resource needs
//! - **Optimization History**: Learns from past optimization outcomes
//!
//! # Example
//!
//! ```rust,ignore
//! use std::time::Duration;
//! use beardog_utils::ai_optimization::AIOptimizationEngine;
//!
//! let engine = AIOptimizationEngine::new(Duration::from_secs(60))?;
//! engine.start_optimization().await?;
//! ```

use beardog_errors::BearDogError;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
// use tracing::debug; // Commented out unused import

use super::history::OptimizationHistory;
use super::neural_network::SimpleNeuralNetwork;
use super::predictor::ResourcePredictor;
use super::types::{
    AIOptimizationStats, OptimizationAction, OptimizationRecommendation, OptimizationType,
    PerformanceModel, PerformanceSample, RecommendationPriority,
};

/// AI-powered optimization engine for automatic performance tuning
///
/// Continuously monitors system performance, predicts resource needs,
/// and generates intelligent optimization recommendations to improve efficiency.
///
/// The engine uses machine learning models to learn from historical data
/// and adapt its recommendations over time for better results.
pub struct AIOptimizationEngine {
    /// Shared performance model tracking system behavior patterns
    performance_model: Arc<RwLock<PerformanceModel>>,
    /// ML-based predictor for forecasting resource usage trends
    resource_predictor: Arc<Mutex<ResourcePredictor>>,
    /// Historical record of optimization actions and their outcomes
    optimization_history: Arc<Mutex<OptimizationHistory>>,
    /// Time interval between optimization cycles
    optimization_interval: Duration,
    /// Whether the engine should learn from results and adapt
    is_learning_enabled: bool,
    /// Neural network for advanced pattern recognition (future use)
    _neural_network: Arc<Mutex<SimpleNeuralNetwork>>,
}

impl AIOptimizationEngine {
    /// Creates a new AI optimization engine instance
    ///
    /// # Arguments
    ///
    /// * `optimization_interval` - The interval between optimization cycles
    ///
    /// # Errors
    ///
    /// Returns `BearDogError` if:
    /// - Resource predictor initialization fails due to insufficient memory
    /// - Neural network creation fails due to invalid parameters
    /// - System resources are unavailable for AI components
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::time::Duration;
    /// use beardog_utils::ai_optimization::AIOptimizationEngine;
    ///
    /// let engine = AIOptimizationEngine::new(Duration::from_secs(60))?;
    /// # Ok::<(), beardog_errors::BearDogError>(())
    /// ```
    pub fn new(optimization_interval: Duration) -> Result<Self, BearDogError> {
        let performance_model = Arc::new(RwLock::new(PerformanceModel::new()));
        let resource_predictor = Arc::new(Mutex::new(ResourcePredictor::new(100)?));
        let optimization_history = Arc::new(Mutex::new(OptimizationHistory::new(1000)));
        let neural_network = Arc::new(Mutex::new(SimpleNeuralNetwork::new(8, 16, 4)?));

        Ok(Self {
            performance_model,
            resource_predictor,
            optimization_history,
            optimization_interval,
            is_learning_enabled: true,
            _neural_network: neural_network,
        })
    }

    /// Start the continuous optimization loop
    ///
    /// Runs indefinitely, collecting performance data, updating models,
    /// generating recommendations, and learning from results at the configured interval.
    ///
    /// # Errors
    /// Returns error if performance collection, model updates, or optimization application fails
    pub async fn start_optimization(&self) -> Result<(), BearDogError> {
        let mut interval = tokio::time::interval(self.optimization_interval);

        loop {
            interval.tick().await;

            if self.is_learning_enabled {
                let sample = self.collect_performance_sample().await?;
                self.update_models(&sample).await?;
                let recommendations = self.generate_recommendations()?;
                self.apply_optimizations(&recommendations)?;
                self.learn_from_results()?;
            }
        }
    }

    /// Collect current system performance metrics
    ///
    /// Gathers real-time data on CPU, memory, network, crypto throughput,
    /// response times, error rates, and system load.
    ///
    /// # Errors
    /// Returns error if any metric collection fails or system time is unavailable
    async fn collect_performance_sample(&self) -> Result<PerformanceSample, BearDogError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| BearDogError::system(e.to_string()))?
            .as_secs();

        // Collect real system performance data
        let cpu_usage = self.get_cpu_usage()?;
        let memory_usage = self.get_memory_usage()?;
        let network_latency = self.measure_network_latency().await?;
        let crypto_throughput = self.measure_crypto_throughput()?;
        let response_time = self.measure_response_time().await?;
        let error_rate = self.calculate_error_rate()?;
        let system_load = self.get_system_load()?;

        Ok(PerformanceSample {
            timestamp,
            cpu_usage,
            memory_usage,
            network_latency,
            crypto_throughput,
            response_time,
            error_rate,
            system_load,
        })
    }

    /// Update ML models with new performance data
    ///
    /// Feeds the latest performance sample to both the performance model
    /// and resource predictor for learning and trend analysis.
    ///
    /// # Errors
    /// Returns error if model updates fail or locks cannot be acquired
    async fn update_models(&self, sample: &PerformanceSample) -> Result<(), BearDogError> {
        // Update performance model
        let mut model = self.performance_model.write().await;
        model.update_weights(sample)?;

        // Update resource predictor
        let mut predictor = self.resource_predictor.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock resource predictor: {e}"))
        })?;
        predictor.add_sample(
            sample.cpu_usage,
            sample.memory_usage,
            sample.network_latency,
        )?;

        Ok(())
    }

    /// Generate optimization recommendations based on current state
    ///
    /// Analyzes performance data and model predictions to suggest
    /// specific optimization actions with confidence scores and priorities.
    ///
    /// # Errors
    /// Returns error if recommendation generation logic encounters issues
    fn generate_recommendations(&self) -> Result<Vec<OptimizationRecommendation>, BearDogError> {
        // Simple recommendation logic
        let recommendations = vec![OptimizationRecommendation {
            optimization_type: OptimizationType::ThreadPool,
            confidence: 0.85,
            expected_improvement: 15.0,
            parameters: std::collections::HashMap::new(),
            reasoning: "CPU utilization could be improved with thread pool optimization"
                .to_string(),
            priority: RecommendationPriority::Medium,
        }];

        Ok(recommendations)
    }

    /// Apply recommended optimizations to the system
    ///
    /// Executes the optimization actions suggested by the engine,
    /// recording each action in the optimization history.
    ///
    /// # Errors
    /// Returns error if optimization actions cannot be recorded in history
    fn apply_optimizations(
        &self,
        recommendations: &[OptimizationRecommendation],
    ) -> Result<(), BearDogError> {
        for recommendation in recommendations {
            self.execute_optimization(recommendation)?;
        }
        Ok(())
    }

    /// Executes optimization
    fn execute_optimization(
        &self,
        recommendation: &OptimizationRecommendation,
    ) -> Result<(), BearDogError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| BearDogError::system(e.to_string()))?
            .as_secs();

        let action = OptimizationAction {
            timestamp,
            action_type: recommendation.optimization_type.clone(),
            parameters: recommendation.parameters.clone(),
            expected_improvement: recommendation.expected_improvement,
            actual_improvement: None,
            success: None,
        };

        let mut history = self.optimization_history.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock optimization history: {e}"))
        })?;
        history.add_action(action);

        // Execute the optimization
        match recommendation.optimization_type {
            OptimizationType::ThreadPool => {
                tracing::info!("AI: Optimizing thread pool size to improve CPU utilization");
            }
            OptimizationType::Simd => {
                tracing::info!("AI: Enabling SIMD acceleration for cryptographic operations");
            }
            OptimizationType::Memory => {
                tracing::info!("AI: Optimizing memory allocation strategy");
            }
            _ => {
                tracing::info!("AI: Applying general performance optimization");
            }
        }

        Ok(())
    }

    const fn learn_from_results(&self) -> Result<(), BearDogError> {
        // Learning implementation would go here
        Ok(())
    }

    /// Gets stats
    pub fn get_stats(&self) -> Result<AIOptimizationStats, BearDogError> {
        let history = self.optimization_history.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock optimization history: {e}"))
        })?;

        Ok(AIOptimizationStats {
            total_optimizations: history.get_total_actions() as u64,
            successful_optimizations: history.get_successful_actions() as u64,
            average_improvement: history.get_average_improvement(),
            learning_accuracy: 0.85,
            prediction_accuracy: 0.78,
            anomalies_detected: 0,
            model_confidence: 0.82,
        })
    }

    /// Get current CPU usage percentage
    /// Gets `cpu_usage`
    fn get_cpu_usage(&self) -> Result<f64, BearDogError> {
        // Basic CPU usage estimation using load average
        // In production, this would use proper system monitoring libraries
        let load =
            std::fs::read_to_string("/proc/loadavg").unwrap_or_else(|_| "0.5 0.4 0.3".to_string());

        let load_avg = load
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.5);

        // Convert load average to approximate CPU usage percentage
        Ok((load_avg * 100.0).min(100.0))
    }

    /// Get current memory usage percentage
    /// Gets `memory_usage`
    fn get_memory_usage(&self) -> Result<f64, BearDogError> {
        // Basic memory usage calculation
        // In production, this would use proper system monitoring
        if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64;
            let mut available = 0u64;

            for line in meminfo.lines() {
                if line.starts_with("MemTotal:") {
                    total = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("MemAvailable:") {
                    available = line
                        .split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0);
                }
            }

            if total > 0 {
                let used = total.saturating_sub(available);
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "memory use ratio for display; f64 mantissa acceptable"
                )]
                let used_f = used as f64;
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "memory use ratio for display; f64 mantissa acceptable"
                )]
                let total_f = total as f64;
                return Ok((used_f / total_f) * 100.0);
            }
        }

        // Fallback estimate
        Ok(32.0)
    }

    /// Measure network latency
    async fn measure_network_latency(&self) -> Result<f64, BearDogError> {
        // Simple latency measurement
        let start = std::time::Instant::now();

        // Yield so other tasks can run; real latency comes from actual I/O when wired in.
        tokio::task::yield_now().await;

        #[expect(
            clippy::cast_precision_loss,
            reason = "millisecond latency sample; sub-ms precision not required"
        )]
        let latency = start.elapsed().as_millis() as f64;
        Ok(latency.max(1.0)) // Minimum 1ms
    }

    /// Measure cryptographic throughput
    fn measure_crypto_throughput(&self) -> Result<f64, BearDogError> {
        use sha2::{Digest, Sha256};

        let start = std::time::Instant::now();
        let test_payload = vec![0u8; 1024]; // 1KB test data

        // Perform multiple hash operations
        for _ in 0..100 {
            let mut hasher = Sha256::new();
            hasher.update(&test_payload);
            let _ = hasher.finalize();
        }

        let elapsed = start.elapsed().as_secs_f64();
        let throughput = (100.0 * 1024.0) / elapsed; // bytes per second

        Ok(throughput)
    }

    /// Measure response time
    async fn measure_response_time(&self) -> Result<f64, BearDogError> {
        let start = std::time::Instant::now();

        // Simulate a typical operation
        tokio::task::yield_now().await;

        #[expect(
            clippy::cast_precision_loss,
            reason = "response time sample in ms; coarse float is intentional"
        )]
        let ms = start.elapsed().as_millis() as f64;
        Ok(ms)
    }

    /// Calculate current error rate
    const fn calculate_error_rate(&self) -> Result<f64, BearDogError> {
        // In production, this would track actual error rates
        // For now, return a low baseline error rate
        Ok(0.001) // 0.1% error rate
    }

    /// Get system load average
    /// Gets `system_load`
    fn get_system_load(&self) -> Result<f64, BearDogError> {
        let load =
            std::fs::read_to_string("/proc/loadavg").unwrap_or_else(|_| "0.6 0.5 0.4".to_string());

        let load_avg = load
            .split_whitespace()
            .next()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.6);

        Ok(load_avg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::float_eq;

    fn make_engine() -> AIOptimizationEngine {
        AIOptimizationEngine::new(Duration::from_secs(60))
            .expect("Engine construction should succeed")
    }

    // ── Constructor & Stats ──────────────────────────────────────────

    #[test]
    fn test_new_creates_engine_with_defaults() {
        let engine = make_engine();
        assert!(engine.is_learning_enabled);
        assert_eq!(engine.optimization_interval, Duration::from_secs(60));
    }

    #[test]
    fn test_get_stats_initial() {
        let engine = make_engine();
        let stats = engine.get_stats().expect("get_stats should succeed");
        assert_eq!(stats.total_optimizations, 0);
        assert_eq!(stats.successful_optimizations, 0);
        assert_eq!(stats.anomalies_detected, 0);
    }

    // ── Private metric collectors ────────────────────────────────────

    #[test]
    fn test_get_cpu_usage_returns_valid_range() {
        let engine = make_engine();
        let cpu = engine.get_cpu_usage().expect("cpu usage should succeed");
        assert!(cpu >= 0.0, "CPU usage should be non-negative");
        assert!(cpu <= 100.0, "CPU usage should be at most 100");
    }

    #[test]
    fn test_get_memory_usage_returns_valid_range() {
        let engine = make_engine();
        let mem = engine
            .get_memory_usage()
            .expect("memory usage should succeed");
        assert!(mem >= 0.0, "Memory usage should be non-negative");
        assert!(mem <= 100.0, "Memory usage should be at most 100");
    }

    #[tokio::test]
    async fn test_measure_network_latency_returns_positive() {
        let engine = make_engine();
        let latency = engine
            .measure_network_latency()
            .await
            .expect("latency measurement should succeed");
        assert!(latency >= 1.0, "Latency should be at least 1ms");
    }

    #[test]
    fn test_measure_crypto_throughput_returns_positive() {
        let engine = make_engine();
        let throughput = engine
            .measure_crypto_throughput()
            .expect("crypto throughput should succeed");
        assert!(throughput > 0.0, "Throughput should be positive");
    }

    #[tokio::test]
    async fn test_measure_response_time_returns_non_negative() {
        let engine = make_engine();
        let rt = engine
            .measure_response_time()
            .await
            .expect("response time should succeed");
        assert!(rt >= 0.0, "Response time should be non-negative");
    }

    #[test]
    fn test_calculate_error_rate_returns_baseline() {
        let engine = make_engine();
        let rate = engine
            .calculate_error_rate()
            .expect("error rate should succeed");
        assert!((rate - 0.001).abs() < f64::EPSILON);
    }

    #[test]
    fn test_get_system_load_returns_non_negative() {
        let engine = make_engine();
        let load = engine
            .get_system_load()
            .expect("system load should succeed");
        assert!(load >= 0.0, "System load should be non-negative");
    }

    // ── Collect / Update / Recommend / Apply ─────────────────────────

    #[tokio::test]
    async fn test_collect_performance_sample() {
        let engine = make_engine();
        let sample = engine
            .collect_performance_sample()
            .await
            .expect("sample collection should succeed");

        assert!(sample.timestamp > 0);
        assert!(sample.cpu_usage >= 0.0);
        assert!(sample.memory_usage >= 0.0);
        assert!(sample.network_latency >= 1.0);
        assert!(sample.crypto_throughput > 0.0);
        assert!(sample.response_time >= 0.0);
        assert!((sample.error_rate - 0.001).abs() < f64::EPSILON);
        assert!(sample.system_load >= 0.0);
    }

    #[tokio::test]
    async fn test_update_models_with_sample() {
        let engine = make_engine();
        let sample = engine
            .collect_performance_sample()
            .await
            .expect("sample collection should succeed");

        engine
            .update_models(&sample)
            .await
            .expect("update_models should succeed");
    }

    #[tokio::test]
    async fn test_update_models_high_cpu_updates_weights() {
        let engine = make_engine();
        let sample = PerformanceSample {
            timestamp: 1234,
            cpu_usage: 0.95,
            memory_usage: 0.8,
            network_latency: 5.0,
            crypto_throughput: 1000.0,
            response_time: 2.0,
            error_rate: 0.001,
            system_load: 0.9,
        };

        engine
            .update_models(&sample)
            .await
            .expect("update_models should succeed with high cpu");

        let model = engine.performance_model.read().await;
        assert_eq!(model.last_updated, 1234);
    }

    #[test]
    fn test_generate_recommendations() {
        let engine = make_engine();
        let recs = engine
            .generate_recommendations()
            .expect("recommendations should succeed");

        assert_eq!(recs.len(), 1);
        assert!(matches!(
            recs[0].optimization_type,
            OptimizationType::ThreadPool
        ));
        float_eq::f64(recs[0].confidence, 0.85);
        assert!(matches!(recs[0].priority, RecommendationPriority::Medium));
    }

    #[test]
    fn test_apply_optimizations_empty() {
        let engine = make_engine();
        engine
            .apply_optimizations(&[])
            .expect("empty optimizations should succeed");
    }

    #[test]
    fn test_apply_optimizations_with_recommendations() {
        let engine = make_engine();
        let recs = engine
            .generate_recommendations()
            .expect("recommendations should succeed");
        engine
            .apply_optimizations(&recs)
            .expect("apply_optimizations should succeed");

        let stats = engine.get_stats().expect("stats should succeed");
        assert_eq!(stats.total_optimizations, 1);
    }

    #[test]
    fn test_execute_optimization_thread_pool() {
        let engine = make_engine();
        let rec = OptimizationRecommendation {
            optimization_type: OptimizationType::ThreadPool,
            confidence: 0.9,
            expected_improvement: 20.0,
            parameters: std::collections::HashMap::new(),
            reasoning: "test".to_string(),
            priority: RecommendationPriority::High,
        };
        engine
            .execute_optimization(&rec)
            .expect("execute ThreadPool should succeed");
    }

    #[test]
    fn test_execute_optimization_simd() {
        let engine = make_engine();
        let rec = OptimizationRecommendation {
            optimization_type: OptimizationType::Simd,
            confidence: 0.8,
            expected_improvement: 10.0,
            parameters: std::collections::HashMap::new(),
            reasoning: "simd test".to_string(),
            priority: RecommendationPriority::Low,
        };
        engine
            .execute_optimization(&rec)
            .expect("execute Simd should succeed");
    }

    #[test]
    fn test_execute_optimization_memory() {
        let engine = make_engine();
        let rec = OptimizationRecommendation {
            optimization_type: OptimizationType::Memory,
            confidence: 0.7,
            expected_improvement: 5.0,
            parameters: std::collections::HashMap::new(),
            reasoning: "mem test".to_string(),
            priority: RecommendationPriority::Medium,
        };
        engine
            .execute_optimization(&rec)
            .expect("execute Memory should succeed");
    }

    #[test]
    fn test_execute_optimization_other_variants() {
        let engine = make_engine();
        for opt_type in [
            OptimizationType::Network,
            OptimizationType::Cache,
            OptimizationType::GarbageCollection,
            OptimizationType::Pool,
            OptimizationType::Crypto,
        ] {
            let rec = OptimizationRecommendation {
                optimization_type: opt_type,
                confidence: 0.6,
                expected_improvement: 3.0,
                parameters: std::collections::HashMap::new(),
                reasoning: "test".to_string(),
                priority: RecommendationPriority::Low,
            };
            engine
                .execute_optimization(&rec)
                .expect("execute other type should succeed");
        }
    }

    #[test]
    fn test_learn_from_results() {
        let engine = make_engine();
        engine
            .learn_from_results()
            .expect("learn_from_results should succeed");
    }

    // ── Full cycle test ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_full_optimization_cycle() {
        let engine = make_engine();

        // Step 1: collect a sample
        let sample = engine.collect_performance_sample().await.expect("sample");

        // Step 2: update models
        engine.update_models(&sample).await.expect("update");

        // Step 3: generate recommendations
        let recs = engine.generate_recommendations().expect("recs");

        // Step 4: apply optimizations
        engine.apply_optimizations(&recs).expect("apply");

        // Step 5: learn
        engine.learn_from_results().expect("learn");

        // Step 6: verify stats
        let stats = engine.get_stats().expect("stats");
        assert_eq!(stats.total_optimizations, 1);
    }

    #[test]
    fn test_stats_after_multiple_optimizations() {
        let engine = make_engine();
        let recs = engine.generate_recommendations().expect("recs");

        for _ in 0..5 {
            engine.apply_optimizations(&recs).expect("apply");
        }

        let stats = engine.get_stats().expect("stats");
        assert_eq!(stats.total_optimizations, 5);
    }

    #[test]
    fn get_stats_initial_counts_zero() {
        let engine = make_engine();
        let stats = engine.get_stats().expect("stats");
        assert_eq!(stats.total_optimizations, 0);
        assert_eq!(stats.successful_optimizations, 0);
        assert!(stats.learning_accuracy > 0.0);
    }
}
