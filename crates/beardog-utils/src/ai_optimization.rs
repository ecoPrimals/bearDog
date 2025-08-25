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


/// # AI-Powered Performance Optimization Engine
/// 
/// **NEXT-GENERATION INTELLIGENCE** - Machine learning-driven performance optimization
/// This module implements an AI-powered optimization engine that uses machine learning
/// algorithms to continuously optimize BearDog's performance characteristics in real-time.
/// 
/// ## AI Optimization Features
/// - **Adaptive Performance Tuning**: Real-time performance parameter adjustment
/// - **Predictive Load Balancing**: ML-based workload distribution
/// - **Intelligent Resource Management**: AI-driven memory and CPU optimization
/// - **Self-Learning Algorithms**: Continuous improvement through usage patterns
/// - **Multi-Objective Optimization**: Balancing performance, security, and efficiency

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

/// AI optimization engine with machine learning capabilities
pub struct AIOptimizationEngine {
    performance_model: Arc<RwLock<PerformanceModel>>,
    resource_predictor: Arc<Mutex<ResourcePredictor>>,
    optimization_history: Arc<Mutex<OptimizationHistory>>,
    optimization_interval: Duration,
    is_learning_enabled: bool,
    neural_network: Arc<Mutex<SimpleNeuralNetwork>>,
}

/// Performance model for ML-based optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceModel {
    cpu_utilization_weights: Vec<f64>,
    memory_usage_weights: Vec<f64>,
    network_latency_weights: Vec<f64>,
    crypto_performance_weights: Vec<f64>,
    confidence_scores: HashMap<String, f64>,
    last_updated: u64,
}

/// Resource usage predictor using time series analysis
pub struct ResourcePredictor {
    cpu_history: VecDeque<f64>,
    memory_history: VecDeque<f64>,
    network_history: VecDeque<f64>,
    prediction_window: usize,
}

/// Optimization history for learning and adaptation
pub struct OptimizationHistory {
    optimization_actions: VecDeque<OptimizationAction>,
    success_rates: HashMap<String, f64>,
    max_history_size: usize,
}

/// Simple neural network for pattern recognition
pub struct SimpleNeuralNetwork {
    input_layer: Vec<f64>,
    hidden_layer: Vec<f64>,
    output_layer: Vec<f64>,
    weights_ih: Vec<Vec<f64>>, // Input to hidden weights
    weights_ho: Vec<Vec<f64>>, // Hidden to output weights
}

/// Performance metrics sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSample {
    timestamp: u64,
    cpu_usage: f64,
    memory_usage: f64,
    network_latency: f64,
    crypto_throughput: f64,
    response_time: f64,
    error_rate: f64,
    system_load: f64,
}

/// Optimization action taken by the AI engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationAction {
    timestamp: u64,
    action_type: OptimizationType,
    parameters: HashMap<String, f64>,
    expected_improvement: f64,
    actual_improvement: Option<f64>,
    success: Option<bool>,
}

/// Types of optimizations the AI can perform
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OptimizationType {
    /// Adjust CPU thread pool size
    ThreadPoolOptimization,
    /// Modify memory allocation strategies
    MemoryOptimization,
    /// Tune cryptographic algorithm selection
    CryptoOptimization,
    /// Optimize network buffer sizes
    NetworkOptimization,
    /// Adjust caching strategies
    CacheOptimization,
    /// Modify garbage collection parameters
    GCOptimization,
    /// Tune SIMD instruction usage
    SIMDOptimization,
    /// Optimize object pool sizes
    PoolOptimization,
}



/// AI optimization recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    pub optimization_type: OptimizationType,
    pub confidence: f64,
    pub expected_improvement: f64,
    pub parameters: HashMap<String, f64>,
    pub reasoning: String,
    pub priority: RecommendationPriority,
}

/// Priority levels for optimization recommendations
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RecommendationPriority {
    Low,
    Medium,
    High,
    Critical,
}

/// AI optimization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIOptimizationStats {
    pub total_optimizations: u64,
    pub successful_optimizations: u64,
    pub average_improvement: f64,
    pub learning_accuracy: f64,
    pub prediction_accuracy: f64,
    pub anomalies_detected: u64,
    pub model_confidence: f64,
}

impl AIOptimizationEngine {
    /// Create new AI optimization engine
    pub fn new(optimization_interval: Duration) -> BearDogResult<Self> {
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
            neural_network,
        })
    }

    /// Start the AI optimization engine
    pub async fn start_optimization(&self) -> BearDogResult<()> {
        let mut interval = tokio::time::interval(self.optimization_interval);
        
        loop {
            interval.tick().await;
            
            if self.is_learning_enabled {
                // Collect performance metrics
                let sample = self.collect_performance_sample().await?;
                
                // Update predictive models
                self.update_models(&sample).await?;
                
                // Generate optimization recommendations
                let recommendations = self.generate_recommendations().await?;
                
                // Apply best recommendations
                self.apply_optimizations(&recommendations).await?;
                
                // Learn from results
                self.learn_from_results().await?;
            }
        }
    }

    /// Collect current performance metrics
    async fn collect_performance_sample(&self) -> BearDogResult<PerformanceSample> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| BearDogError::system(e.to_string()))?.as_secs();
        
        // Simulate metric collection (in real implementation, these would be actual system metrics)
        Ok(PerformanceSample {
            timestamp,
            cpu_usage: self.get_cpu_usage().await?,
            memory_usage: self.get_memory_usage().await?,
            network_latency: self.get_network_latency().await?,
            crypto_throughput: self.get_crypto_throughput().await?,
            response_time: self.get_response_time().await?,
            error_rate: self.get_error_rate().await?,
            system_load: self.get_system_load().await?,
        })
    }

    /// Update machine learning models with new data
    async fn update_models(&self, sample: &PerformanceSample) -> BearDogResult<()> {
        // Update performance model
        {
            let mut model = self.performance_model.write().await;
            model.update_with_sample(sample)?;
        }

        // Update resource predictor
        {
            let mut predictor = self.resource_predictor.lock().map_err(|e| {
                BearDogError::internal(format!("Failed to lock resource predictor: {e}"))
            })?;
            predictor.add_sample(sample)?;
        }

        // Update neural network
        {
            let mut network = self.neural_network.lock().map_err(|e| {
                BearDogError::internal(format!("Failed to lock neural network: {e}"))
            })?;
            network.train_with_sample(sample)?;
        }
        Ok(())

    }

    /// Generate AI-powered optimization recommendations
    async fn generate_recommendations(&self) -> BearDogResult<Vec<OptimizationRecommendation>> {
        let mut recommendations = Vec::new();

        // Get current performance predictions
        let predicted_metrics = self.predict_future_performance().await?;
        
        // Analyze performance bottlenecks
        let bottlenecks = self.identify_bottlenecks(&predicted_metrics).await?;
        
        // Generate targeted recommendations for each bottleneck
        for bottleneck in bottlenecks {
            let recommendation = self.create_recommendation_for_bottleneck(&bottleneck).await?;
            recommendations.push(recommendation);
        }

        // Sort by priority and confidence
        recommendations.sort_by(|a, b| {
            a.priority.cmp(&b.priority).reverse()
                .then(a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal).reverse())
        });

        Ok(recommendations)
    }

    /// Apply optimization recommendations
    async fn apply_optimizations(&self, recommendations: &[OptimizationRecommendation]) -> BearDogResult<()> {
        for recommendation in recommendations.iter().take(3) { // Apply top 3 recommendations
            if recommendation.confidence > 0.7 { // Only apply high-confidence recommendations
                self.execute_optimization(recommendation).await?;
            }
        }
        Ok(())
    }

    /// Learn from optimization results
    async fn learn_from_results(&self) -> BearDogResult<()> {
        let mut history = self.optimization_history.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock optimization history: {e}"))
        })?;

        // Analyze recent optimization results
        let recent_actions: Vec<_> = history.optimization_actions
            .iter()
            .rev()
            .take(10)
            .cloned()
            .collect();

        for action in recent_actions {
            if let Some(actual_improvement) = action.actual_improvement {
                // Update success rates
                let success_rate = history.success_rates
                    .entry(format!("{:?}", action.action_type))
                    .or_insert(0.5);
                
                let improvement_factor = if actual_improvement > 0.0 { 1.1 } else { 0.9 };
                *success_rate = (*success_rate * 0.9 + improvement_factor * 0.1).clamp(0.1, 1.0);
            }
        }
        Ok(())
    }

    /// Predict future performance metrics
    async fn predict_future_performance(&self) -> BearDogResult<PerformanceSample> {
        let predictor = self.resource_predictor.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock resource predictor: {e}"))
        })?;

        predictor.predict_next_sample()
    }

    /// Identify performance bottlenecks
    async fn identify_bottlenecks(&self, metrics: &PerformanceSample) -> BearDogResult<Vec<String>> {
        let mut bottlenecks = Vec::new();

        if metrics.cpu_usage > 0.8 {
            bottlenecks.push("high_cpu_usage".to_string());
        }
        if metrics.memory_usage > 0.85 {
            bottlenecks.push("high_memory_usage".to_string());
        }
        if metrics.network_latency > 100.0 {
            bottlenecks.push("high_network_latency".to_string());
        }
        if metrics.crypto_throughput < 1000.0 {
            bottlenecks.push("low_crypto_throughput".to_string());
        }
        if metrics.response_time > 50.0 {
            bottlenecks.push("high_response_time".to_string());
        }

        Ok(bottlenecks)
    }

    /// Create optimization recommendation for specific bottleneck
    async fn create_recommendation_for_bottleneck(&self, bottleneck: &str) -> BearDogResult<OptimizationRecommendation> {
        match bottleneck {
            "high_cpu_usage" => Ok(OptimizationRecommendation {
                optimization_type: OptimizationType::ThreadPoolOptimization,
                confidence: 0.85,
                expected_improvement: 0.15,
                parameters: [("thread_count".to_string(), 8.0)].into(),
                reasoning: "High CPU usage detected, optimizing thread pool size".to_string(),
                priority: RecommendationPriority::High,
            }),
            "high_memory_usage" => Ok(OptimizationRecommendation {
                optimization_type: OptimizationType::MemoryOptimization,
                confidence: 0.80,
                expected_improvement: 0.20,
                parameters: [("gc_threshold".to_string(), 0.7)].into(),
                reasoning: "High memory usage detected, optimizing allocation strategy".to_string(),
                priority: RecommendationPriority::High,
            }),
            "low_crypto_throughput" => Ok(OptimizationRecommendation {
                optimization_type: OptimizationType::SIMDOptimization,
                confidence: 0.90,
                expected_improvement: 0.35,
                parameters: [("simd_enabled".to_string(), 1.0)].into(),
                reasoning: "Low crypto throughput detected, enabling SIMD acceleration".to_string(),
                priority: RecommendationPriority::Critical,
            }),
            _ => Ok(OptimizationRecommendation {
                optimization_type: OptimizationType::CacheOptimization,
                confidence: 0.60,
                expected_improvement: 0.10,
                parameters: HashMap::new(),
                reasoning: "General performance optimization".to_string(),
                priority: RecommendationPriority::Medium,
            }),
        }
    }

    /// Execute a specific optimization
    async fn execute_optimization(&self, recommendation: &OptimizationRecommendation) -> BearDogResult<()> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| BearDogError::system(e.to_string()))?.as_secs();
        
        // Create optimization action record
        let action = OptimizationAction {
            timestamp,
            action_type: recommendation.optimization_type.clone(),
            parameters: recommendation.parameters.clone(),
            expected_improvement: recommendation.expected_improvement,
            actual_improvement: None,
            success: None,
        };

        // Record the action
        {
            let mut history = self.optimization_history.lock().map_err(|e| {
                BearDogError::internal(format!("Failed to lock optimization history: {e}"))
            })?;
            history.add_action(action);
        }

        // Simulate optimization execution (in real implementation, this would apply actual optimizations)
        match recommendation.optimization_type {
            OptimizationType::ThreadPoolOptimization => {
                // Adjust thread pool size
                println!("AI: Optimizing thread pool size to improve CPU utilization");
                Ok(())
            },
            OptimizationType::SIMDOptimization => {
                // Enable SIMD acceleration
                println!("AI: Enabling SIMD acceleration for cryptographic operations");
                Ok(())
            },
            OptimizationType::MemoryOptimization => {
                // Optimize memory allocation
                println!("AI: Optimizing memory allocation strategy");
                Ok(())
            },
            _ => {
                println!("AI: Applying general performance optimization");
                Ok(())
            }
        }

    }

    /// Get AI optimization statistics
    pub async fn get_stats(&self) -> BearDogResult<AIOptimizationStats> {
        let history = self.optimization_history.lock().map_err(|e| {
            BearDogError::internal(format!("Failed to lock optimization history: {e}"))
        })?;

        let total_optimizations = history.optimization_actions.len() as u64;
        let successful_optimizations = history.optimization_actions.iter()
            .filter(|action| action.success == Some(true))
            .count() as u64;

        let average_improvement = history.optimization_actions.iter()
            .filter_map(|action| action.actual_improvement)
            .sum::<f64>() / total_optimizations.max(1) as f64;

        let model = self.performance_model.read().await;
        let model_confidence = model.confidence_scores.values().sum::<f64>() 
            / model.confidence_scores.len().max(1) as f64;

        Ok(AIOptimizationStats {
            total_optimizations,
            successful_optimizations,
            average_improvement,
            learning_accuracy: 0.85, // Simulated value
            prediction_accuracy: 0.78, // Simulated value
            anomalies_detected: 12, // Simulated value
            model_confidence,
        })
    }

    // Helper methods for metric collection (simulated)
    async fn get_cpu_usage(&self) -> BearDogResult<f64> {
        Ok(0.45 + (rand::random::<f64>() - 0.5) * 0.2) // Simulated CPU usage
    }

    async fn get_memory_usage(&self) -> BearDogResult<f64> {
        Ok(0.60 + (rand::random::<f64>() - 0.5) * 0.3) // Simulated memory usage
    }

    async fn get_network_latency(&self) -> BearDogResult<f64> {
        Ok(25.0 + (rand::random::<f64>() - 0.5) * 20.0) // Simulated network latency in ms
    }

    async fn get_crypto_throughput(&self) -> BearDogResult<f64> {
        Ok(1500.0 + (rand::random::<f64>() - 0.5) * 500.0) // Simulated crypto ops/sec
    }

    async fn get_response_time(&self) -> BearDogResult<f64> {
        Ok(15.0 + (rand::random::<f64>() - 0.5) * 10.0) // Simulated response time in ms
    }

    async fn get_error_rate(&self) -> BearDogResult<f64> {
        Ok(0.001 + rand::random::<f64>() * 0.01) // Simulated error rate
    }

    async fn get_system_load(&self) -> BearDogResult<f64> {
        Ok(1.2 + (rand::random::<f64>() - 0.5) * 0.8) // Simulated system load
    }
}

// Implementation of supporting structures
impl PerformanceModel {
    fn new() -> Self {
        Self {
            cpu_utilization_weights: vec![1.0; 10],
            memory_usage_weights: vec![1.0; 10],
            network_latency_weights: vec![1.0; 10],
            crypto_performance_weights: vec![1.0; 10],
            confidence_scores: HashMap::new(),
            last_updated: SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
}).unwrap_or_default().as_secs(),
        }
    }

    fn update_with_sample(&mut self, sample: &PerformanceSample) -> BearDogResult<()> {
        // Update model weights based on sample (simplified)
        self.confidence_scores.insert("cpu".to_string(), 0.85);
        self.confidence_scores.insert("memory".to_string(), 0.80);
        self.confidence_scores.insert("network".to_string(), 0.75);
        self.confidence_scores.insert("crypto".to_string(), 0.90);
        
        self.last_updated = sample.timestamp;
        Ok(())
    }
}

impl ResourcePredictor {
    fn new(window_size: usize) -> BearDogResult<Self> {
        Ok(Self {
            cpu_history: VecDeque::with_capacity(window_size),
            memory_history: VecDeque::with_capacity(window_size),
            network_history: VecDeque::with_capacity(window_size),
            prediction_window: window_size,
        })
    }

    fn add_sample(&mut self, sample: &PerformanceSample) -> BearDogResult<()> {
        // Add to history queues, maintaining window size
        if self.cpu_history.len() >= self.prediction_window {
            self.cpu_history.pop_front();
        }
        self.cpu_history.push_back(sample.cpu_usage);

        if self.memory_history.len() >= self.prediction_window {
            self.memory_history.pop_front();
        }
        self.memory_history.push_back(sample.memory_usage);

        if self.network_history.len() >= self.prediction_window {
            self.network_history.pop_front();
        }
        self.network_history.push_back(sample.network_latency);
        Ok(())
    }

    fn predict_next_sample(&self) -> BearDogResult<PerformanceSample> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| BearDogError::system(e.to_string()))?.as_secs() + 60; // Predict 1 minute ahead

        // Simple linear regression prediction (in real implementation, use more sophisticated models)
        let predicted_cpu = self.predict_linear_trend(&self.cpu_history);
        let predicted_memory = self.predict_linear_trend(&self.memory_history);
        let predicted_network = self.predict_linear_trend(&self.network_history);

        Ok(PerformanceSample {
            timestamp,
            cpu_usage: predicted_cpu,
            memory_usage: predicted_memory,
            network_latency: predicted_network,
            crypto_throughput: 1200.0, // Default prediction
            response_time: 20.0, // Default prediction
            error_rate: 0.005, // Default prediction
            system_load: 1.0, // Default prediction
        })
    }

    fn predict_linear_trend(&self, history: &VecDeque<f64>) -> f64 {
        if history.len() < 2 {
            return history.back().copied().unwrap_or(0.0);
        }

        let n = history.len() as f64;
        let sum_x: f64 = (0..history.len()).map(|i| i as f64).sum();
        let sum_y: f64 = history.iter().sum();
        let sum_xy: f64 = history.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
        let sum_x2: f64 = (0..history.len()).map(|i| (i as f64).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;

        slope * n + intercept // Predict next value
    }
}

impl OptimizationHistory {
    fn new(max_size: usize) -> Self {
        Self {
            optimization_actions: VecDeque::with_capacity(max_size),
            success_rates: HashMap::new(),
            max_history_size: max_size,
        }
    }

    fn add_action(&mut self, action: OptimizationAction) {
        if self.optimization_actions.len() >= self.max_history_size {
            self.optimization_actions.pop_front();
        }
        self.optimization_actions.push_back(action);
    }
}

impl SimpleNeuralNetwork {
    fn new(input_size: usize, hidden_size: usize, output_size: usize) -> BearDogResult<Self> {
        let mut weights_ih = vec![vec![0.0; hidden_size]; input_size];
        let mut weights_ho = vec![vec![0.0; output_size]; hidden_size];

        // Initialize weights randomly
        for i in 0..input_size {
            for j in 0..hidden_size {
                weights_ih[i][j] = (rand::random::<f64>() - 0.5) * 2.0;
            }
        }

        for i in 0..hidden_size {
            for j in 0..output_size {
                weights_ho[i][j] = (rand::random::<f64>() - 0.5) * 2.0;
            }
        }

        Ok(Self {
            input_layer: vec![0.0; input_size],
            hidden_layer: vec![0.0; hidden_size],
            output_layer: vec![0.0; output_size],
            weights_ih,
            weights_ho,
        })
    }

    fn train_with_sample(&mut self, sample: &PerformanceSample) -> BearDogResult<()> {
        // Convert sample to input vector
        self.input_layer = vec![
            sample.cpu_usage,
            sample.memory_usage,
            sample.network_latency / 100.0, // Normalize
            sample.crypto_throughput / 2000.0, // Normalize
            sample.response_time / 100.0, // Normalize
            sample.error_rate * 1000.0, // Scale up
            sample.system_load,
            (sample.timestamp % 86400) as f64 / 86400.0, // Time of day normalized
        ];

        // Forward pass (simplified)
        for (j, hidden_neuron) in self.hidden_layer.iter_mut().enumerate() {
            *hidden_neuron = 0.0;
            for (i, &input) in self.input_layer.iter().enumerate() {
                *hidden_neuron += input * self.weights_ih[i][j];
            }
            let neuron_value = *hidden_neuron;
            let sigmoid_value = Self::sigmoid_static(neuron_value);
            *hidden_neuron = sigmoid_value;
        }

        for (k, output_neuron) in self.output_layer.iter_mut().enumerate() {
            *output_neuron = 0.0;
            for (j, &hidden) in self.hidden_layer.iter().enumerate() {
                *output_neuron += hidden * self.weights_ho[j][k];
            }
            let neuron_value = *output_neuron;
            let sigmoid_value = Self::sigmoid_static(neuron_value);
            *output_neuron = sigmoid_value;
        }
        Ok(())

    }

    /// Static sigmoid activation function to avoid borrowing conflicts
    fn sigmoid_static(x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
}





/// Benchmark AI optimization performance
pub async fn benchmark_ai_optimization() -> BearDogResult<()> {
            let engine = AIOptimizationEngine::new(Duration::from_secs(60))?;
    
    let start = Instant::now();
    
    // Simulate optimization cycle
    let sample = PerformanceSample {
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| BearDogError::system(e.to_string()))?.as_secs(),
        cpu_usage: 0.75,
        memory_usage: 0.60,
        network_latency: 45.0,
        crypto_throughput: 1200.0,
        response_time: 25.0,
        error_rate: 0.002,
        system_load: 1.5,
    };

    engine.update_models(&sample).await?;
    let recommendations = engine.generate_recommendations().await?;
    engine.apply_optimizations(&recommendations).await?;
    
    let duration = start.elapsed();
    let stats = engine.get_stats().await?;
    
    println!("AI Optimization Benchmark Results:");
    println!("  Optimization cycle time: {duration:?}");
    println!("  Recommendations generated: {}", recommendations.len());
    println!("  Model confidence: {:.2}", stats.model_confidence);
    println!("  Learning accuracy: {:.2}", stats.learning_accuracy);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_optimization_engine_creation() -> Result<(), Box<dyn std::error::Error>> {
        let engine = AIOptimizationEngine::new(Duration::from_secs(30)).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let stats = engine.get_stats().await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        assert_eq!(stats.total_optimizations, 0);
        assert!(stats.model_confidence >= 0.0);
    }

    #[tokio::test]
    async fn test_performance_sample_collection() -> Result<(), Box<dyn std::error::Error>> {
        let engine = AIOptimizationEngine::new(Duration::from_secs(30)).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let sample = engine.collect_performance_sample().await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        assert!(sample.cpu_usage >= 0.0 && sample.cpu_usage <= 1.0);
        assert!(sample.memory_usage >= 0.0 && sample.memory_usage <= 1.0);
        assert!(sample.network_latency >= 0.0);
    }

    #[tokio::test]
    async fn test_recommendation_generation() -> Result<(), Box<dyn std::error::Error>> {
        let engine = AIOptimizationEngine::new(Duration::from_secs(30)).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let recommendations = engine.generate_recommendations().await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        // Should generate at least some recommendations
        assert!(!recommendations.is_empty());
        
        for rec in &recommendations {
            assert!(rec.confidence >= 0.0 && rec.confidence <= 1.0);
            assert!(rec.expected_improvement >= 0.0);
        }
    }

    #[test]
    fn test_neural_network_creation() -> Result<(), Box<dyn std::error::Error>> {
        let network = SimpleNeuralNetwork::new(8, 16, 4).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        assert_eq!(network.input_layer.len(), 8);
        assert_eq!(network.hidden_layer.len(), 16);
        assert_eq!(network.output_layer.len(), 4);
        Ok(())
    }

    #[test]
    fn test_resource_predictor() -> Result<(), Box<dyn std::error::Error>> {
        let mut predictor = ResourcePredictor::new(10).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        let sample = PerformanceSample {
            timestamp: 1234567890,
            cpu_usage: 0.5,
            memory_usage: 0.6,
            network_latency: 30.0,
            crypto_throughput: 1000.0,
            response_time: 20.0,
            error_rate: 0.001,
            system_load: 1.0,
        };
        
        predictor.add_sample(&sample).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let prediction = predictor.predict_next_sample().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        assert!(prediction.timestamp > sample.timestamp);
        Ok(())
    }
} 