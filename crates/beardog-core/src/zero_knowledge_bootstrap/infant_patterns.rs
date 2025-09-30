//! Infant Discovery Patterns for Zero-Knowledge Bootstrap
//!
//! This module implements "infant" discovery patterns - starting from minimal
//! knowledge and gradually learning about the ecosystem through observation
//! and interaction, similar to how an infant learns about the world.

use beardog_errors::BearDogResult;
use beardog_types::capabilities::CapabilityType as Capability;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Infant discovery pattern engine
#[derive(Debug)]
pub struct InfantPatternEngine {
    /// Discovered patterns
    patterns: Arc<RwLock<HashMap<String, DiscoveryPattern>>>,
    /// Learning state
    learning_state: Arc<RwLock<LearningState>>,
    /// Configuration
    config: InfantPatternConfig,
}

/// A discovered pattern in the ecosystem
#[derive(Debug, Clone)]
pub struct DiscoveryPattern {
    /// Pattern identifier
    pub pattern_id: String,
    /// Pattern type
    pub pattern_type: PatternType,
    /// Observed frequency
    pub frequency: f64,
    /// Confidence in this pattern (0.0 to 1.0)
    pub confidence: f64,
    /// When this pattern was first observed
    pub first_observed: Instant,
    /// Last time this pattern was observed
    pub last_observed: Instant,
    /// Pattern metadata
    pub metadata: HashMap<String, String>,
}

/// Types of patterns that can be discovered
#[derive(Debug, Clone, PartialEq)]
pub enum PatternType {
    /// Service endpoint pattern
    ServiceEndpoint,
    /// Capability availability pattern
    CapabilityAvailability,
    /// Communication protocol pattern
    CommunicationProtocol,
    /// Response time pattern
    ResponseTime,
    /// Error recovery pattern
    ErrorRecovery,
    /// Load balancing pattern
    LoadBalancing,
}

/// Learning state of the infant pattern engine
#[derive(Debug, Clone)]
pub struct LearningState {
    /// Total observations made
    pub total_observations: u64,
    /// Successful pattern matches
    pub successful_matches: u64,
    /// Failed pattern attempts
    pub failed_attempts: u64,
    /// Learning confidence (0.0 to 1.0)
    pub learning_confidence: f64,
    /// Current learning phase
    pub learning_phase: LearningPhase,
}

/// Phases of learning
#[derive(Debug, Clone, PartialEq)]
pub enum LearningPhase {
    /// Initial observation phase
    Observation,
    /// Pattern recognition phase
    PatternRecognition,
    /// Hypothesis formation phase
    HypothesisFormation,
    /// Pattern validation phase
    Validation,
    /// Mature operation phase
    MatureOperation,
}

/// Configuration for infant pattern discovery
#[derive(Debug, Clone)]
pub struct InfantPatternConfig {
    /// Minimum observations before forming a pattern
    pub min_observations: u64,
    /// Confidence threshold for pattern acceptance
    pub confidence_threshold: f64,
    /// Maximum age for patterns before re-evaluation
    pub pattern_max_age: Duration,
    /// Learning rate for confidence updates
    pub learning_rate: f64,
}

impl Default for InfantPatternConfig {
    fn default() -> Self {
        Self {
            min_observations: 5,
            confidence_threshold: 0.7,
            pattern_max_age: Duration::from_secs(3600), // 1 hour
            learning_rate: 0.1,
        }
    }
}

impl InfantPatternEngine {
    /// Create a new infant pattern engine
    pub fn new(config: InfantPatternConfig) -> Self {
        info!("👶 Initializing InfantPatternEngine - Starting with zero knowledge");
        Self {
            patterns: Arc::new(RwLock::new(HashMap::new())),
            learning_state: Arc::new(RwLock::new(LearningState {
                total_observations: 0,
                successful_matches: 0,
                failed_attempts: 0,
                learning_confidence: 0.0,
                learning_phase: LearningPhase::Observation,
            })),
            config,
        }
    }

    /// Observe an ecosystem interaction
    pub async fn observe_interaction(
        &self,
        interaction_type: &str,
        context: HashMap<String, String>,
        success: bool,
    ) -> BearDogResult<()> {
        debug!(
            "👁️ Observing interaction: {} (success: {})",
            interaction_type, success
        );

        let mut learning_state = self.learning_state.write().await;
        learning_state.total_observations += 1;

        if success {
            learning_state.successful_matches += 1;
        } else {
            learning_state.failed_attempts += 1;
        }

        // Update learning confidence
        let success_rate =
            learning_state.successful_matches as f64 / learning_state.total_observations as f64;
        learning_state.learning_confidence = success_rate;

        // Update learning phase based on observations
        self.update_learning_phase(&mut learning_state).await;

        // Try to identify patterns
        self.identify_patterns(interaction_type, context, success)
            .await?;

        Ok(())
    }

    /// Identify patterns from observations
    async fn identify_patterns(
        &self,
        interaction_type: &str,
        context: HashMap<String, String>,
        success: bool,
    ) -> BearDogResult<()> {
        let pattern_id = format!(
            "{}_{}",
            interaction_type,
            context.get("service_id").unwrap_or(&"unknown".to_string())
        );

        let mut patterns = self.patterns.write().await;

        match patterns.get_mut(&pattern_id) {
            Some(pattern) => {
                // Update existing pattern
                pattern.last_observed = Instant::now();
                pattern.frequency += if success {
                    self.config.learning_rate
                } else {
                    -self.config.learning_rate * 0.5
                };
                pattern.frequency = pattern.frequency.clamp(0.0, 1.0);

                // Update confidence based on recent success
                if success {
                    pattern.confidence += self.config.learning_rate * (1.0 - pattern.confidence);
                } else {
                    pattern.confidence -= self.config.learning_rate * pattern.confidence;
                }
                pattern.confidence = pattern.confidence.clamp(0.0, 1.0);

                debug!(
                    "🔄 Updated pattern {}: confidence={:.2}, frequency={:.2}",
                    pattern_id, pattern.confidence, pattern.frequency
                );
            }
            None => {
                // Create new pattern
                let pattern_type = self.classify_pattern_type(interaction_type);
                let pattern = DiscoveryPattern {
                    pattern_id: pattern_id.clone(),
                    pattern_type,
                    frequency: if success { 0.5 } else { 0.1 },
                    confidence: if success { 0.6 } else { 0.2 },
                    first_observed: Instant::now(),
                    last_observed: Instant::now(),
                    metadata: context,
                };

                patterns.insert(pattern_id.clone(), pattern);
                info!("🆕 Discovered new pattern: {}", pattern_id);
            }
        }

        Ok(())
    }

    /// Classify the type of pattern based on interaction type
    fn classify_pattern_type(&self, interaction_type: &str) -> PatternType {
        match interaction_type {
            s if s.contains("endpoint") => PatternType::ServiceEndpoint,
            s if s.contains("capability") => PatternType::CapabilityAvailability,
            s if s.contains("protocol") => PatternType::CommunicationProtocol,
            s if s.contains("response_time") => PatternType::ResponseTime,
            s if s.contains("error") => PatternType::ErrorRecovery,
            s if s.contains("load") => PatternType::LoadBalancing,
            _ => PatternType::ServiceEndpoint, // Default
        }
    }

    /// Update learning phase based on current state
    async fn update_learning_phase(&self, learning_state: &mut LearningState) {
        let new_phase = match learning_state.learning_phase {
            LearningPhase::Observation
                if learning_state.total_observations
                    >= (beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE
                        as u64) =>
            {
                LearningPhase::PatternRecognition
            }
            LearningPhase::PatternRecognition if learning_state.total_observations >= 25 => {
                LearningPhase::HypothesisFormation
            }
            LearningPhase::HypothesisFormation if learning_state.learning_confidence >= 0.7 => {
                LearningPhase::Validation
            }
            LearningPhase::Validation if learning_state.learning_confidence >= 0.8 => {
                LearningPhase::MatureOperation
            }
            _ => learning_state.learning_phase.clone(),
        };

        if new_phase != learning_state.learning_phase {
            info!(
                "🎓 Learning phase transition: {:?} -> {:?}",
                learning_state.learning_phase, new_phase
            );
            learning_state.learning_phase = new_phase;
        }
    }

    /// Get patterns by type
    pub async fn get_patterns_by_type(&self, pattern_type: PatternType) -> Vec<DiscoveryPattern> {
        let patterns = self.patterns.read().await;
        patterns
            .values()
            .filter(|p| {
                p.pattern_type == pattern_type && p.confidence >= self.config.confidence_threshold
            })
            .cloned()
            .collect()
    }

    /// Predict best service for a capability based on learned patterns
    pub async fn predict_best_service(&self, capability: &Capability) -> Option<String> {
        let patterns = self.patterns.read().await;

        let mut best_service = None;
        let mut best_score = 0.0;

        for pattern in patterns.values() {
            if pattern.pattern_type == PatternType::CapabilityAvailability {
                if let Some(service_id) = pattern.metadata.get("service_id") {
                    let score = pattern.confidence * pattern.frequency;
                    if score > best_score {
                        best_score = score;
                        best_service = Some(service_id.clone());
                    }
                }
            }
        }

        if let Some(service) = &best_service {
            debug!(
                "🎯 Predicted best service for {:?}: {} (score: {:.2})",
                capability, service, best_score
            );
        }

        best_service
    }

    /// Get current learning state
    pub async fn get_learning_state(&self) -> LearningState {
        self.learning_state.read().await.clone()
    }

    /// Clean up old patterns
    pub async fn cleanup_old_patterns(&self) -> BearDogResult<u64> {
        let mut patterns = self.patterns.write().await;
        let now = Instant::now();
        let mut removed_count = 0;

        patterns.retain(|_id, pattern| {
            let age = now.duration_since(pattern.last_observed);
            let should_keep = age < self.config.pattern_max_age;

            if !should_keep {
                removed_count += 1;
            }

            should_keep
        });

        if removed_count > 0 {
            info!("🧹 Cleaned up {} old patterns", removed_count);
        }

        Ok(removed_count)
    }

    /// Get pattern statistics
    pub async fn get_pattern_stats(&self) -> HashMap<String, u64> {
        let patterns = self.patterns.read().await;
        let learning_state = self.learning_state.read().await;

        let mut stats = HashMap::new();
        stats.insert("total_patterns".to_string(), patterns.len() as u64);
        stats.insert(
            "total_observations".to_string(),
            learning_state.total_observations,
        );
        stats.insert(
            "successful_matches".to_string(),
            learning_state.successful_matches,
        );
        stats.insert(
            "failed_attempts".to_string(),
            learning_state.failed_attempts,
        );

        // Count patterns by type
        for pattern in patterns.values() {
            let type_key = format!("{:?}_patterns", pattern.pattern_type);
            *stats.entry(type_key).or_insert(0) += 1;
        }

        stats
    }
}
