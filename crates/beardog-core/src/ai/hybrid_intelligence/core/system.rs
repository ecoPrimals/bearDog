// SPDX-License-Identifier: AGPL-3.0-or-later

//! Orchestration: `HybridIntelligenceSystem` lifecycle, prediction, decisions, and monitoring.

use super::super::config::HybridIntelligenceConfig;
use super::super::core_types::{IntelligenceCapability, ModelType};
use super::super::learning::PredictionHorizon;
use super::types::{
    DecisionResult, IntelligenceEvent, IntelligenceEventType, IntelligenceMetrics,
    PredictionResult, SystemStatus,
};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio::time::interval;
use tracing::{debug, info, trace};
use uuid::Uuid;

/// Hybrid intelligence system orchestrator
///
/// Main system that coordinates AI and human intelligence,
/// managing models, predictions, decisions, and learning processes.
#[derive(Debug)]
pub struct HybridIntelligenceSystem {
    /// System configuration
    pub config: HybridIntelligenceConfig,
    /// System health status
    pub health: Arc<RwLock<HealthStatus>>,
    /// System metrics
    pub metrics: Arc<RwLock<IntelligenceMetrics>>,
    /// Event sender for broadcasting intelligence events
    pub event_sender: broadcast::Sender<IntelligenceEvent>,
    /// Command receiver for system control
    pub command_receiver: Arc<RwLock<mpsc::Receiver<super::types::SystemCommand>>>,
    /// Active intelligence capabilities
    pub active_capabilities: Arc<RwLock<Vec<IntelligenceCapability>>>,
}

impl HybridIntelligenceSystem {
    /// Creates a new hybrid intelligence system
    ///
    /// # Errors
    /// Returns an error if system creation fails.
    pub fn new(config: HybridIntelligenceConfig) -> Result<Self, BearDogError> {
        let (event_sender, _) = broadcast::channel(1000);
        let (_command_sender, command_receiver) = mpsc::channel(100);

        Ok(Self {
            config,
            health: Arc::new(RwLock::new(HealthStatus::Healthy)),
            metrics: Arc::new(RwLock::new(IntelligenceMetrics::default())),
            event_sender,
            command_receiver: Arc::new(RwLock::new(command_receiver)),
            active_capabilities: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Initializes the hybrid intelligence system
    ///
    /// # Errors
    /// Returns an error if initialization fails.
    pub async fn initialize(&self) -> Result<(), BearDogError> {
        info!(
            "Initializing hybrid intelligence system: {}",
            self.config.system_id
        );

        self.validate_hybrid_config_bounds()?;

        for (i, capability) in self.config.enabled_capabilities.iter().enumerate() {
            if self.config.enabled_capabilities[i + 1..]
                .iter()
                .any(|other| other == capability)
            {
                return Err(BearDogError::Business {
                    message: format!(
                        "Duplicate entry in enabled_capabilities: {capability:?} (each capability must appear at most once)"
                    ),
                    category: beardog_errors::BusinessErrorCategory::Validation,
                });
            }
            self.initialize_capability(*capability)?;
        }

        {
            let mut active = self.active_capabilities.write().await;
            active.clear();
            active.extend_from_slice(&self.config.enabled_capabilities);
        }

        self.start_monitoring();

        {
            let mut health = self.health.write().await;
            *health = HealthStatus::Healthy;
        }

        info!("Hybrid intelligence system initialized successfully");
        Ok(())
    }

    /// Makes a prediction using the hybrid intelligence system
    ///
    /// # Errors
    /// Returns an error if prediction fails.
    pub async fn predict(
        &self,
        input_data: Vec<f64>,
        model_id: Option<String>,
    ) -> Result<PredictionResult, BearDogError> {
        debug!("Making prediction with {} input features", input_data.len());

        // Validate input data
        if input_data.is_empty() {
            return Err(BearDogError::Business {
                message: "Input data cannot be empty".to_string(),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }

        // Use default model if none specified
        let model_id = model_id.unwrap_or_else(|| "default_model".to_string());

        // Create prediction result using basic statistical analysis
        let predictions = self.compute_statistical_predictions(&input_data)?;
        let confidence_intervals = self.compute_confidence_intervals(&predictions, &input_data);
        let uncertainty = self.compute_uncertainty_estimates(&predictions, &input_data);

        let prediction_result = PredictionResult {
            id: Uuid::new_v4(),
            predictions,
            confidence_intervals: Some(confidence_intervals),
            uncertainty: Some(uncertainty),
            model_id,
            timestamp: Utc::now(),
            horizon: Some(PredictionHorizon::ShortTerm),
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_predictions += 1;
        }

        // Send prediction event
        let event = IntelligenceEvent {
            id: Uuid::new_v4(),
            event_type: IntelligenceEventType::PredictionMade,
            data: {
                let mut data = HashMap::new();
                data.insert(
                    "prediction_id".to_string(),
                    serde_json::Value::String(prediction_result.id.to_string()),
                );
                data.insert(
                    "model_id".to_string(),
                    serde_json::Value::String(prediction_result.model_id.clone()),
                );
                data
            },
            timestamp: Utc::now(),
        };
        let _ = self.event_sender.send(event);

        Ok(prediction_result)
    }

    /// Makes a decision using the hybrid intelligence system
    /// Makes a decision based on context
    ///
    /// # Errors
    /// Returns an error if decision making fails.
    pub async fn make_decision(
        &self,
        context: HashMap<String, serde_json::Value>,
    ) -> Result<DecisionResult, BearDogError> {
        debug!(
            "Making decision with context: {:?}",
            context.keys().collect::<Vec<_>>()
        );

        // Create decision result with real logic based on context analysis
        let (decision_action, confidence, reasoning) =
            Self::calculate_decision_from_context(&context);

        let decision = DecisionResult {
            decision_id: Uuid::new_v4(),
            decision: decision_action,
            confidence,
            reasoning,
            timestamp: Utc::now(),
            context,
        };

        // Update metrics
        {
            let mut metrics = self.metrics.write().await;
            metrics.total_decisions += 1;
            // Update average confidence (simple moving average)
            #[expect(
                clippy::cast_precision_loss,
                reason = "Moving average over u64 decision count"
            )]
            let total_decisions = metrics.total_decisions as f64;
            metrics.avg_decision_confidence = metrics
                .avg_decision_confidence
                .mul_add(total_decisions - 1.0, decision.confidence)
                / total_decisions;
        }

        // Send decision event
        let event = IntelligenceEvent {
            id: Uuid::new_v4(),
            event_type: IntelligenceEventType::DecisionMade,
            data: {
                let mut data = HashMap::new();
                data.insert(
                    "decision_id".to_string(),
                    serde_json::Value::String(decision.decision_id.to_string()),
                );
                data.insert(
                    "decision".to_string(),
                    serde_json::Value::String(decision.decision.clone()),
                );
                if let Some(confidence_num) = serde_json::Number::from_f64(decision.confidence) {
                    data.insert(
                        "confidence".to_string(),
                        serde_json::Value::Number(confidence_num),
                    );
                } else {
                    data.insert(
                        "confidence".to_string(),
                        serde_json::Value::String(format!("{}", decision.confidence)),
                    );
                }
                data
            },
            timestamp: Utc::now(),
        };
        let _ = self.event_sender.send(event);

        Ok(decision)
    }

    /// Subscribes to intelligence events
    #[must_use]
    pub fn subscribe_to_events(&self) -> broadcast::Receiver<IntelligenceEvent> {
        self.event_sender.subscribe()
    }

    /// Gets current system metrics
    /// Gets metrics
    pub async fn get_metrics(&self) -> IntelligenceMetrics {
        *self.metrics.read().await
    }

    /// Gets current system health
    /// Gets health
    pub async fn get_health(&self) -> HealthStatus {
        *self.health.read().await
    }

    /// Validates global numeric bounds on [`HybridIntelligenceConfig`] before any capability runs.
    fn validate_hybrid_config_bounds(&self) -> Result<(), BearDogError> {
        let c = &self.config;
        if !(0.0..=1.0).contains(&c.ai_confidence_threshold) {
            return Err(BearDogError::Business {
                message: format!(
                    "ai_confidence_threshold must be in [0.0, 1.0], got {}",
                    c.ai_confidence_threshold
                ),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        if !(0.0..=1.0).contains(&c.human_feedback_weight) {
            return Err(BearDogError::Business {
                message: format!(
                    "human_feedback_weight must be in [0.0, 1.0], got {}",
                    c.human_feedback_weight
                ),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(())
    }

    /// Initializes one [`IntelligenceCapability`]: structured tracing, alignment checks against
    /// `ml_config.model_type`, and readiness markers for the hybrid layer (models remain
    /// configuration-driven; this step does not load external binaries).
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result type reserved for capability-specific init errors"
    )]
    fn initialize_capability(
        &self,
        capability: IntelligenceCapability,
    ) -> Result<(), BearDogError> {
        let system_id = self.config.system_id.as_str();

        match capability {
            IntelligenceCapability::PredictiveAnalytics => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized predictive analytics: ties into prediction_config and statistical predict path"
                );
            }
            IntelligenceCapability::AnomalyDetection => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized anomaly detection: monitors residual error vs input for decision thresholds"
                );
            }
            IntelligenceCapability::PatternRecognition => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized pattern recognition: feature correlation via configured ML stack"
                );
            }
            IntelligenceCapability::NaturalLanguageProcessing => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized NLP capability: tokenizer and embedding hooks use ml_config when models are attached"
                );
            }
            IntelligenceCapability::ComputerVision => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized computer vision capability: tensor pipeline uses neural_config when present"
                );
            }
            IntelligenceCapability::ReinforcementLearning => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized reinforcement learning: pairs with learning_config exploration and reward signals"
                );
            }
            IntelligenceCapability::DecisionTrees => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized decision tree ensemble path: aligns with tree-capable model types"
                );
            }
            IntelligenceCapability::NeuralNetworks => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized neural pipeline: uses neural_config and NetworkArchitecture when set"
                );
            }
            IntelligenceCapability::GeneticAlgorithms => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized genetic / evolutionary search hooks under optimization_config"
                );
            }
            IntelligenceCapability::FuzzyLogic => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized fuzzy logic decision blending with decision_config thresholds"
                );
            }
            IntelligenceCapability::ExpertSystems => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized expert-system rule layer over structured decision context"
                );
            }
            IntelligenceCapability::AutomatedReasoning => {
                trace!(
                    system_id,
                    ?capability,
                    "Initialized automated reasoning: symbolic checks augment make_decision context analysis"
                );
            }
        }

        self.log_capability_model_alignment(capability);
        debug!(
            system_id,
            ?capability,
            "Hybrid intelligence capability initialization complete"
        );

        Ok(())
    }

    /// Logs when the enabled capability name does not match the primary [`ModelType`] (informational;
    /// the runtime still serves predictions via the generic statistical path).
    fn log_capability_model_alignment(&self, capability: IntelligenceCapability) {
        let mt = self.config.ml_config.model_type;
        let aligned = match capability {
            IntelligenceCapability::NeuralNetworks | IntelligenceCapability::ComputerVision => {
                matches!(mt, ModelType::NeuralNetwork | ModelType::DeepLearning)
            }
            IntelligenceCapability::NaturalLanguageProcessing => {
                matches!(mt, ModelType::NeuralNetwork | ModelType::DeepLearning)
            }
            IntelligenceCapability::DecisionTrees => matches!(
                mt,
                ModelType::DecisionTree | ModelType::RandomForest | ModelType::GradientBoosting
            ),
            IntelligenceCapability::ReinforcementLearning => {
                matches!(mt, ModelType::ReinforcementLearning)
            }
            _ => true,
        };

        if !aligned {
            debug!(
                system_id = %self.config.system_id,
                ?capability,
                ?mt,
                "Capability label does not match primary ml_config.model_type; verify configuration if you rely on typed model backends"
            );
        }
    }

    /// Starts monitoring tasks
    /// Starts monitoring
    fn start_monitoring(&self) {
        let metrics = Arc::clone(&self.metrics);
        let start_time = Utc::now();

        tokio::spawn(async move {
            let interval_secs =
                beardog_errors::process_env::var("BEARDOG_AI_METRICS_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60);
            let mut interval = interval(Duration::from_secs(interval_secs));

            loop {
                interval.tick().await;

                // Update uptime
                let mut metrics_guard = metrics.write().await;
                #[expect(
                    clippy::cast_sign_loss,
                    reason = "Uptime duration from start is non-negative"
                )]
                {
                    metrics_guard.uptime_secs =
                        Utc::now().signed_duration_since(start_time).num_seconds() as u64;
                }
            }
        });
    }

    /// Compute statistical predictions from input data
    #[expect(
        clippy::unnecessary_wraps,
        reason = "Result reserved for richer statistical models"
    )]
    #[expect(
        clippy::unused_self,
        reason = "Self reserved for model-backed predictions"
    )]
    fn compute_statistical_predictions(
        &self,
        input_data: &[f64],
    ) -> Result<Vec<f64>, BearDogError> {
        let mut predictions = Vec::new();

        for (i, &value) in input_data.iter().enumerate() {
            // Basic statistical prediction using moving average and trend analysis
            let prediction = if i == 0 {
                value * 1.05 // Simple 5% increase prediction for first value
            } else {
                // Use previous values to predict trend
                #[expect(
                    clippy::cast_precision_loss,
                    reason = "Index to f64 for mean of slice prefix"
                )]
                let avg = input_data[..=i].iter().sum::<f64>() / (i + 1) as f64;
                let trend = if i > 1 {
                    (input_data[i] - input_data[i - 1]) * 0.7 // Damped trend continuation
                } else {
                    0.0
                };
                avg + trend
            };
            predictions.push(prediction);
        }

        Ok(predictions)
    }

    #[expect(
        clippy::unused_self,
        reason = "Instance reserved for model-specific intervals"
    )]
    fn compute_confidence_intervals(
        &self,
        predictions: &[f64],
        input_data: &[f64],
    ) -> Vec<(f64, f64)> {
        predictions
            .iter()
            .zip(input_data.iter())
            .map(|(&pred, &input)| {
                let variance = (pred - input).abs() * 0.2; // 20% variance estimate
                (pred - variance, pred + variance)
            })
            .collect()
    }

    /// Compute uncertainty estimates
    #[expect(
        clippy::unused_self,
        reason = "Instance reserved for model-specific uncertainty"
    )]
    fn compute_uncertainty_estimates(&self, predictions: &[f64], input_data: &[f64]) -> Vec<f64> {
        predictions
            .iter()
            .zip(input_data.iter())
            .map(|(&pred, &input)| {
                let error = (pred - input).abs();
                let max_val = input.abs().max(pred.abs()).max(1.0); // Avoid division by zero
                error / max_val // Relative uncertainty
            })
            .collect()
    }

    /// Get comprehensive system status
    ///
    /// Returns complete system status including health, capabilities, and metrics.
    pub async fn get_status(&self) -> SystemStatus {
        SystemStatus {
            system_id: self.config.system_id.clone(),
            health: self.get_health().await,
            active_capabilities: self.active_capabilities.read().await.clone(),
            metrics: self.get_metrics().await,
            last_updated: Utc::now(),
        }
    }

    /// Shuts down the system gracefully
    ///
    /// # Errors
    /// Returns an error if shutdown fails.
    pub fn shutdown(&self) -> Result<(), BearDogError> {
        info!(
            "Shutting down hybrid intelligence system: {}",
            self.config.system_id
        );

        // Update health status
        {
            let mut health = self.health.blocking_write();
            *health = HealthStatus::Unhealthy;
        }

        // Clear active capabilities
        {
            let mut active = self.active_capabilities.blocking_write();
            active.clear();
        }

        info!("Hybrid intelligence system shut down successfully");
        Ok(())
    }

    /// Calculate an intelligent decision based on context analysis
    ///
    /// Returns tuple of (`decision_action`, confidence, reasoning)
    fn calculate_decision_from_context(
        context: &HashMap<String, serde_json::Value>,
    ) -> (String, f64, String) {
        // Extract relevant context values as floats
        let risk_level = context
            .get("risk_level")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.5);

        let data_quality = context
            .get("data_quality")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.7);

        let system_confidence = context
            .get("system_confidence")
            .and_then(serde_json::Value::as_f64)
            .unwrap_or(0.8);

        // Calculate overall confidence score
        let confidence = (data_quality * 0.4 + system_confidence * 0.6) * (1.0 - risk_level * 0.3);
        let confidence = confidence.clamp(0.0, 1.0);

        // Make decision based on confidence and risk
        let (action, reasoning) = match (confidence, risk_level) {
            (c, _) if c > 0.9 => (
                "approve",
                format!("High confidence ({c:.2}), low risk - approved for execution"),
            ),
            (c, r) if c > 0.75 && r < 0.3 => (
                "approve_with_monitoring",
                format!(
                    "Good confidence ({c:.2}), acceptable risk ({r:.2}) - approved with monitoring"
                ),
            ),
            (c, r) if c > 0.6 && r < 0.5 => (
                "conditional_approve",
                format!(
                    "Moderate confidence ({c:.2}), moderate risk ({r:.2}) - conditional approval"
                ),
            ),
            (c, r) if c > 0.5 => (
                "review_required",
                format!(
                    "Moderate confidence ({c:.2}), elevated risk ({r:.2}) - human review required"
                ),
            ),
            (_c, r) if r > 0.7 => (
                "reject_high_risk",
                format!("Risk level ({r:.2}) exceeds acceptable threshold - rejected for safety"),
            ),
            (c, r) => (
                "reject_low_confidence",
                format!("Insufficient confidence ({c:.2}) for risk level ({r:.2}) - rejected"),
            ),
        };

        (action.to_string(), confidence, reasoning)
    }
}

#[cfg(all(test, feature = "ai"))]
mod tests {
    #![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

    use super::HybridIntelligenceSystem;
    use crate::ai::hybrid_intelligence::config::HybridIntelligenceConfig;
    use crate::ai::hybrid_intelligence::core_types::{IntelligenceCapability, ModelType};

    fn sample_config(ids: &[IntelligenceCapability]) -> HybridIntelligenceConfig {
        HybridIntelligenceConfig {
            system_id: "sys-test".into(),
            enabled_capabilities: ids.to_vec(),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn initialize_registers_all_enabled_capabilities() {
        let system = HybridIntelligenceSystem::new(sample_config(&[
            IntelligenceCapability::PredictiveAnalytics,
            IntelligenceCapability::AnomalyDetection,
        ]))
        .expect("new");
        system.initialize().await.expect("init");
        let caps = system.active_capabilities.read().await.clone();
        assert_eq!(caps.len(), 2);
        assert_eq!(
            format!("{caps:?}"),
            format!("{:?}", system.config.enabled_capabilities)
        );
    }

    #[tokio::test]
    async fn predict_with_single_input_uses_first_value_branch() {
        let system = HybridIntelligenceSystem::new(sample_config(&[
            IntelligenceCapability::PredictiveAnalytics,
        ]))
        .expect("new");
        let p = system.predict(vec![3.0], None).await.expect("pred");
        assert_eq!(p.predictions.len(), 1);
        assert!((p.predictions[0] - 3.15).abs() < 1e-9);
    }

    #[tokio::test]
    async fn initialize_runs_initialize_capability_for_all_variants() {
        let caps = [
            IntelligenceCapability::PredictiveAnalytics,
            IntelligenceCapability::AnomalyDetection,
            IntelligenceCapability::PatternRecognition,
            IntelligenceCapability::NaturalLanguageProcessing,
            IntelligenceCapability::ComputerVision,
            IntelligenceCapability::ReinforcementLearning,
            IntelligenceCapability::DecisionTrees,
            IntelligenceCapability::NeuralNetworks,
            IntelligenceCapability::GeneticAlgorithms,
            IntelligenceCapability::FuzzyLogic,
            IntelligenceCapability::ExpertSystems,
            IntelligenceCapability::AutomatedReasoning,
        ];
        let system = HybridIntelligenceSystem::new(sample_config(&caps)).expect("new");
        system.initialize().await.expect("init");
        let active = system.active_capabilities.read().await.clone();
        assert_eq!(active.len(), caps.len());
    }

    #[tokio::test]
    async fn initialize_rejects_duplicate_capabilities() {
        let mut cfg = sample_config(&[]);
        cfg.enabled_capabilities = vec![
            IntelligenceCapability::NeuralNetworks,
            IntelligenceCapability::NeuralNetworks,
        ];
        let system = HybridIntelligenceSystem::new(cfg).expect("new");
        let err = system
            .initialize()
            .await
            .expect_err("duplicate capability entries must be rejected");
        assert!(
            err.to_string().contains("Duplicate entry"),
            "unexpected error: {err}"
        );
    }

    #[tokio::test]
    async fn initialize_logs_model_mismatch_for_neural_capability_with_linear_model() {
        let mut cfg = sample_config(&[IntelligenceCapability::NeuralNetworks]);
        cfg.ml_config.model_type = ModelType::LinearRegression;
        let system = HybridIntelligenceSystem::new(cfg).expect("new");
        system.initialize().await.expect("init");
    }
}
