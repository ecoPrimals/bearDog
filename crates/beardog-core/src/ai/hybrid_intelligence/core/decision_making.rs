// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Decision Making and Confidence Assessment
//!
//! This module handles decision making, confidence assessment, and feedback
//! processing for the hybrid intelligence system.

use super::super::config::HybridIntelligenceConfig;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::time::{Duration, SystemTime};
use uuid::Uuid;
use beardog_errors::BearDogError;

/// Decision confidence levels
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum DecisionConfidence {
    /// Low confidence, requires human review
    Low = 1,
    /// Medium confidence, human review recommended
    Medium = 2,
    /// High confidence, can proceed autonomously
    High = 3,
}

impl DecisionConfidence {
    /// Convert DecisionConfidence to numerical value for calculations
    #[must_use]
    pub fn as_f64(&self) -> f64 {
        match self {
            DecisionConfidence::Low => 0.3,
            DecisionConfidence::Medium => 0.6,
            DecisionConfidence::High => 0.9,
        }
    }
}

/// AI model types supported by the hybrid intelligence system
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AIModelType {
    /// Neural network models
    NeuralNetwork,
    /// Decision tree models
    DecisionTree,
    /// Ensemble models combining multiple approaches
    Ensemble,
    /// Rule-based expert systems
    RuleBased,
    /// Fuzzy logic systems
    FuzzyLogic,
    /// Genetic algorithms
    GeneticAlgorithm,
}

/// Learning feedback types from human operators
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LearningFeedback {
    /// Positive feedback - AI decision was correct
    Positive,
    /// Negative feedback - AI decision was incorrect
    Negative,
    /// Neutral feedback - AI decision was acceptable but not optimal
    Neutral,
    /// Corrective feedback with suggested improvement
    Corrective,
}

/// Decision context for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    /// Unique identifier for this decision request
    pub id: String,
    
    /// Input data for the decision
    pub input_data: serde_json::Value,
    
    /// Decision type or category
    pub decision_type: String,
    
    /// Priority level (1-10, 10 being highest)
    pub priority: u8,
    
    /// Timeout for decision processing
    pub timeout: Duration,
    
    /// Required confidence level
    pub required_confidence: DecisionConfidence,
    
    /// Additional context metadata
    pub metadata: BTreeMap<String, String>,
}

/// Result of a decision process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionResult {
    /// Decision identifier
    pub id: String,
    
    /// The actual decision or recommendation
    pub decision: serde_json::Value,
    
    /// Confidence level of the decision
    pub confidence: DecisionConfidence,
    
    /// Confidence score (0.0 to 1.0)
    pub confidence_score: f64,
    
    /// AI model used for the decision
    pub model_used: AIModelType,
    
    /// Processing time taken
    pub processing_time: Duration,
    
    /// Explanation or reasoning for the decision
    pub explanation: Option<String>,
    
    /// Alternative options considered
    pub alternatives: Vec<AlternativeOption>,
    
    /// Timestamp when decision was made
    pub timestamp: SystemTime,
}

/// Alternative decision option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeOption {
    /// The alternative decision
    pub decision: serde_json::Value,
    
    /// Confidence score for this alternative
    pub confidence_score: f64,
    
    /// Brief explanation
    pub explanation: Option<String>,
}

/// Confidence assessment metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceMetrics {
    /// Overall confidence distribution
    pub confidence_distribution: BTreeMap<String, u32>,
    
    /// Average confidence score
    pub average_confidence: f64,
    
    /// Confidence trend over time
    pub confidence_trend: Vec<ConfidenceTrendPoint>,
    
    /// Model accuracy by confidence level
    pub accuracy_by_confidence: BTreeMap<String, f64>,
}

/// Point in confidence trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfidenceTrendPoint {
    /// Timestamp
    pub timestamp: SystemTime,
    
    /// Confidence score at this point
    pub confidence: f64,
    
    /// Number of decisions at this point
    pub decision_count: u32,
}

/// Decision metrics for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMetrics {
    /// Total decisions made
    pub total_decisions: u64,
    
    /// Decisions by confidence level
    pub decisions_by_confidence: BTreeMap<String, u32>,
    
    /// Average processing time
    pub avg_processing_time: Duration,
    
    /// Success rate by confidence level
    pub success_rate_by_confidence: BTreeMap<String, f64>,
    
    /// Feedback statistics
    pub feedback_stats: FeedbackStatistics,
}

/// Feedback statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackStatistics {
    /// Total feedback received
    pub total_feedback: u32,
    
    /// Positive feedback count
    pub positive_feedback: u32,
    
    /// Negative feedback count
    pub negative_feedback: u32,
    
    /// Neutral feedback count
    pub neutral_feedback: u32,
    
    /// Corrective feedback count
    pub corrective_feedback: u32,
}

/// **DECISION ENGINE** - Core decision making system
pub struct DecisionEngine {
    /// Configuration
    config: HybridIntelligenceConfig,
    
    /// Decision history
    decision_history: Vec<DecisionResult>,
    
    /// Feedback history
    feedback_history: HashMap<String, LearningFeedback>,
    
    /// Confidence metrics
    confidence_metrics: ConfidenceMetrics,
    
    /// Decision metrics
    metrics: DecisionMetrics,
}

impl DecisionEngine {
    #[must_use]
    /// Create a new decision engine
    pub fn new(config: HybridIntelligenceConfig) -> Self {
        Self {
            config,
            decision_history: Vec::new(),
            feedback_history: HashMap::new(),
            confidence_metrics: ConfidenceMetrics::default(),
            metrics: DecisionMetrics::default(),
        }
    }
    
    /// Evaluate decision with zero-cost error handling
    pub async fn evaluate(&mut self, context: &DecisionContext) -> Result<DecisionResult, BearDogError> {
        let start_time = SystemTime::now();
        
        // Simulate decision processing
        let decision = self.process_decision(context).await?;
        
        let processing_time = start_time.elapsed().unwrap_or(Duration::from_millis(0));
        
        let result = DecisionResult {
            id: context.id.clone(),
            decision,
            confidence: self.assess_confidence(context).await,
            confidence_score: self.calculate_confidence_score(context).await,
            model_used: self.select_model(context).await,
            processing_time,
            explanation: self.generate_explanation(context).await,
            alternatives: self.generate_alternatives(context).await,
            timestamp: SystemTime::now(),
        };
        
        // Store decision for future reference
        self.decision_history.push(result.clone());
        
        // Update metrics
        self.update_metrics(&result).await;
        
        Ok(result)
    }
    
    /// Process feedback with zero-cost error handling  
    pub async fn process_feedback(&mut self, decision_id: &str, feedback: LearningFeedback) -> Result<(), BearDogError> {
        // Store feedback
        self.feedback_history.insert(decision_id.to_string(), feedback);
        
        // Update feedback statistics
        match feedback {
            LearningFeedback::Positive => self.metrics.feedback_stats.positive_feedback += 1,
            LearningFeedback::Negative => self.metrics.feedback_stats.negative_feedback += 1,
            LearningFeedback::Neutral => self.metrics.feedback_stats.neutral_feedback += 1,
            LearningFeedback::Corrective => self.metrics.feedback_stats.corrective_feedback += 1,
        }
        
        self.metrics.feedback_stats.total_feedback += 1;
        
        // Update confidence metrics based on feedback
        self.update_confidence_metrics(decision_id, feedback).await;
        
        Ok(())
    }
    
    /// Get metrics with zero-cost error handling
    pub async fn get_metrics(&self) -> Result<DecisionMetrics, BearDogError> {
        Ok(self.metrics.clone())
    }
    
    /// Get confidence metrics with zero-cost error handling
    pub async fn get_confidence_metrics(&self) -> Result<ConfidenceMetrics, BearDogError> {
        Ok(self.confidence_metrics.clone())
    }
    
    /// Process decision with zero-cost error handling
    async fn process_decision(&self, context: &DecisionContext) -> Result<serde_json::Value, BearDogError> {
        // Simulate decision processing based on context
        let decision = match context.decision_type.as_str() {
            "classification" => serde_json::json!({"class": "positive", "score": 0.85}),
            "recommendation" => serde_json::json!({"recommendation": "approve", "reasoning": "meets criteria"}),
            "prediction" => serde_json::json!({"prediction": 42.5, "confidence_interval": [40.0, 45.0]}),
            _ => serde_json::json!({"result": "processed", "status": "success"}),
        };
        
        Ok(decision)
    }
    
    /// Assess confidence level for a decision
    async fn assess_confidence(&self, context: &DecisionContext) -> DecisionConfidence {
        // Simple heuristic based on data quality and complexity
        let data_quality_score = self.assess_data_quality(&context.input_data).await;
        let complexity_score = self.assess_complexity(context).await;
        
        let combined_score = (data_quality_score + complexity_score) / 2.0;
        
        if combined_score >= 0.8 {
            DecisionConfidence::High
        } else if combined_score >= 0.6 {
            DecisionConfidence::Medium
        } else {
            DecisionConfidence::Low
        }
    }
    
    /// Calculate numeric confidence score
    async fn calculate_confidence_score(&self, context: &DecisionContext) -> f64 {
        // Simulate confidence calculation
        let base_confidence = 0.7;
        let priority_boost = (context.priority as f64) / 100.0;
        let historical_accuracy = self.get_historical_accuracy(&context.decision_type).await;
        
        (base_confidence + priority_boost + historical_accuracy) / 3.0
    }
    
    /// Select appropriate AI model for the decision
    async fn select_model(&self, context: &DecisionContext) -> AIModelType {
        match context.decision_type.as_str() {
            "classification" => AIModelType::NeuralNetwork,
            "recommendation" => AIModelType::Ensemble,
            "prediction" => AIModelType::NeuralNetwork,
            "rule_based" => AIModelType::RuleBased,
            _ => AIModelType::DecisionTree,
        }
    }
    
    /// Generate explanation for the decision
    async fn generate_explanation(&self, context: &DecisionContext) -> Option<String> {
        Some(format!(
            "Decision made for {} with priority {} based on input analysis",
            context.decision_type, context.priority
        ))
    }
    
    /// Generate alternative options
    async fn generate_alternatives(&self, _context: &DecisionContext) -> Vec<AlternativeOption> {
        // Simulate generation of alternatives
        vec![
            AlternativeOption {
                decision: serde_json::json!({"alternative": "option_a"}),
                confidence_score: 0.6,
                explanation: Some("Conservative approach".to_string()),
            },
            AlternativeOption {
                decision: serde_json::json!({"alternative": "option_b"}),
                confidence_score: 0.4,
                explanation: Some("Aggressive approach".to_string()),
            },
        ]
    }
    
    /// Update decision metrics
    async fn update_metrics(&mut self, result: &DecisionResult) {
        self.metrics.total_decisions += 1;
        
        let confidence_key = format!("{:?}", result.confidence);
        *self.metrics.decisions_by_confidence.entry(confidence_key).or_insert(0) += 1;
        
        // Update average processing time
        let total_time = self.metrics.avg_processing_time.as_millis() as u64 * (self.metrics.total_decisions - 1);
        let new_time = result.processing_time.as_millis() as u64;
        self.metrics.avg_processing_time = Duration::from_millis((total_time + new_time) / self.metrics.total_decisions);
    }
    
    /// Update confidence metrics based on feedback
    async fn update_confidence_metrics(&mut self, decision_id: &str, feedback: LearningFeedback) {
        // Find the decision and update confidence metrics
        if let Some(decision) = self.decision_history.iter().find(|d| d.id == decision_id) {
            let confidence_key = format!("{:?}", decision.confidence);
            
            // Update accuracy based on feedback
            let current_accuracy = self.metrics.success_rate_by_confidence
                .get(&confidence_key)
                .copied()
                .unwrap_or(0.5);
            
            let feedback_score = match feedback {
                LearningFeedback::Positive => 1.0,
                LearningFeedback::Corrective => 0.5,
                LearningFeedback::Neutral => 0.5,
                LearningFeedback::Negative => 0.0,
            };
            
            let updated_accuracy = (current_accuracy * 0.9) + (feedback_score * 0.1);
            self.metrics.success_rate_by_confidence.insert(confidence_key, updated_accuracy);
        }
    }
    
    /// Assess data quality from a JSON payload.
    ///
    /// Scores based on structural richness: penalises null/empty payloads, rewards
    /// object depth and field count.  Returns a value in `0.0..=1.0`.
    async fn assess_data_quality(&self, data: &serde_json::Value) -> f64 {
        match data {
            serde_json::Value::Null => 0.0,
            serde_json::Value::Bool(_) | serde_json::Value::Number(_) | serde_json::Value::String(s) if s.is_empty() => 0.3,
            serde_json::Value::String(_) => 0.6,
            serde_json::Value::Array(arr) => {
                if arr.is_empty() { 0.3 } else { (0.5 + 0.05 * arr.len() as f64).min(1.0) }
            }
            serde_json::Value::Object(map) => {
                if map.is_empty() { 0.3 } else { (0.5 + 0.05 * map.len() as f64).min(1.0) }
            }
        }
    }

    /// Assess decision complexity from context.
    ///
    /// Uses the number of constraints and the decision type to estimate complexity
    /// on a `0.0..=1.0` scale.
    async fn assess_complexity(&self, context: &DecisionContext) -> f64 {
        let base = match context.decision_type.as_str() {
            "security" | "cryptographic" => 0.8,
            "operational" | "deployment" => 0.6,
            _ => 0.5,
        };
        let constraint_factor = (context.constraints.len() as f64 * 0.05).min(0.3);
        (base + constraint_factor).min(1.0)
    }

    /// Look up historical accuracy for a given decision type.
    ///
    /// Falls back to the engine's overall success rate when no per-type data exists.
    async fn get_historical_accuracy(&self, decision_type: &str) -> f64 {
        let key = format!("historical_{decision_type}");
        self.metrics
            .success_rate_by_confidence
            .get(&key)
            .copied()
            .unwrap_or(self.metrics.average_confidence)
    }
}

impl Default for DecisionEngine {
    fn default() -> Self {
        Self::new(HybridIntelligenceConfig::default())
    }
}

impl Default for ConfidenceMetrics {
    fn default() -> Self {
        Self {
            confidence_distribution: BTreeMap::new(),
            average_confidence: 0.5,
            confidence_trend: Vec::new(),
            accuracy_by_confidence: BTreeMap::new(),
        }
    }
}

impl Default for DecisionMetrics {
    fn default() -> Self {
        Self {
            total_decisions: 0,
            decisions_by_confidence: BTreeMap::new(),
            avg_processing_time: Duration::from_millis(0),
            success_rate_by_confidence: BTreeMap::new(),
            feedback_stats: FeedbackStatistics::default(),
        }
    }
}

impl Default for FeedbackStatistics {
    fn default() -> Self {
        Self {
            total_feedback: 0,
            positive_feedback: 0,
            negative_feedback: 0,
            neutral_feedback: 0,
            corrective_feedback: 0,
        }
    }
}

#[must_use]
impl DecisionContext {
    /// Create a new decision context
    pub fn new(decision_type: String, input_data: serde_json::Value) -> Self {
        use beardog_config::domains::timeouts_new::TimeoutConfig;
        
        let timeout_config = TimeoutConfig::from_env();
        
        Self {
            id: Uuid::new_v4().to_string(),
            input_data,
            decision_type,
            priority: 5,
            timeout: timeout_config.ai_decision_duration(),
            required_confidence: DecisionConfidence::Medium,
            metadata: BTreeMap::new(),
        }
    }
}

impl From<u8> for DecisionConfidence {
    fn from(value: u8) -> Self {
        match value {
            1 => DecisionConfidence::Low,
            2 => DecisionConfidence::Medium,
            3 => DecisionConfidence::High,
            _ => DecisionConfidence::Medium,
        }
    }
} 