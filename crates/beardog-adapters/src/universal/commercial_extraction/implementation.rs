// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::detector::*;
use beardog_errors::BearDogError;

use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl CommercialExtractionDetector {
    /// Analyze Request operation.
    pub fn analyze_request(&UniversalRequest,
    ) -> CommercialClassification {
        let user_id = self.extract_user_identity(request);

        self.update_usage_patterns(&user_id, request);

        let entropy_quality = self.analyze_entropy_quality(&user_id, request);

        let commercial_indicators = self.detect_commercial_indicators(&user_id);

        let key_evolution_status = match self.check_key_evolution(&user_id) {
            Ok(status) => status,
            Err(_) => KeyEvolutionStatus::Uninitialized, // Fallback for errors
        };

        self.classify_user_behavior(
            &user_id,
            entropy_quality,
            commercial_indicators,
            key_evolution_status,
        )
    }


    fn extract_user_identity(&self, request: &UniversalRequest) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();

        if let Some(org) = request.payload.get("organization").and_then(|v| v.as_str()) {
            hasher.update(org.as_bytes());
        }
        if let Some(email) = request.payload.get("email").and_then(|v| v.as_str()) {
            hasher.update(email.as_bytes());
        }
        hasher.update(request.request_id.as_bytes());
        format!(
            "user_{:x}",
            hasher
                .finalize(&str, request: &UniversalRequest) {
        let now = Utc::now();
        let pattern = self
            .usage_patterns
            .entry(user_id.to_string())
            .or_insert_with(|| UsagePattern {
                request_frequencies: Vec::new(),
                function_patterns: HashMap::with_capacity(16),
                network_patterns: NetworkBehaviorPattern {
                    connection_persistence: 0.0,
                    batching_patterns: Vec::new(1.0,
                    user_agent_patterns: Vec::new(DataVolumePattern {
                    total_volume: 0,
                    processing_rate: 0.0,
                    volume_variance: 0.0,
                    bulk_operation_score: 0.0,
                },
                classification_confidence: 0.5,
            });

        pattern.request_frequencies.push((now, 1));
        if pattern.request_frequencies.len() > 100 {
            pattern.request_frequencies.remove(0);
        }

        if pattern.request_frequencies.len() > 1 {
            let intervals: Vec<i64> = pattern
                .request_frequencies
                .windows(2)
                .map(|w| (w[1].0 - w[0].0).num_seconds())
                .collect();
            if !intervals.is_empty() {
                let mean = intervals.iter().sum::<i64>() as f64 / intervals.len() as f64;
                let variance = intervals
                    .iter()
                    .map(|&interval| (interval as f64 - mean).powi(2))
                    .sum::<f64>()
                    / intervals.len(&str, request: &UniversalRequest) -> f64 {
        let history = self
            .entropy_tracking
            .entry(user_id.to_string())
            .or_insert_with(|| EntropyHistory {
                entropy_tier_history: Vec::new(),
                quality_progression: Vec::new(),
                human_entropy_sources: Vec::new(),
                key_generations: Vec::new(),
            });

        let mut entropy_quality = 0.3; // Base machine entropy

        if let Some(org) = request.payload.get("organization").and_then(|v| v.as_str()) {
            if !org.contains("corp") && !org.contains("inc") && !org.contains("llc") {
                entropy_quality += 0.2; // Individual boost
            }
        }

        if let Some(payload_obj) = request.payload.as_object() {
            if payload_obj.contains_key("biometric_data")
                || payload_obj.contains_key("audio_entropy")
                || payload_obj.contains_key("haptic_entropy")
            {
                entropy_quality += 0.4; // Human entropy source boost
                history.human_entropy_sources.push(HumanEntropyUsage {
                    source_type: "detected".to_string(1.0,
                    quality_indicators: vec![entropy_quality],
                    consistency_score: 0.5,
                });
            }
        }

        if history.quality_progression.len() > 5 {
            let recent_avg = history
                .quality_progression
                .iter()
                .rev()
                .take(5)
                .map(|(_, q)| *q)
                .sum::<f64>()
                / 5.0;
            if recent_avg > 0.6 {
                entropy_quality = (entropy_quality + 0.1).min(1.0); // Reward consistent humans
            }
        }
        history
            .quality_progression
            .push((Utc::now(), entropy_quality));
        entropy_quality
    }


    fn detect_commercial_indicators(&self, user_id: &str) -> f64 {
        if let Some(pattern) = self.usage_patterns.get(user_id) {
            let mut commercial_score: f64 = 0.0;

            if pattern.request_frequencies.len(&str,
    ) -> Result<KeyEvolutionStatus, BearDogError> {
        if let Some(0,
                    fitness_score: 0.5,
                    parent_generations: Vec::new(),
                    mutations: vec!["baseline".to_string()],
                    created_at: Utc::now(),
                };
                history.key_generations.push(first_gen);
                Ok(KeyEvolutionStatus::NewLineage)
            } else {
                let latest = history.key_generations.last().ok_or_else(|| {
                    BearDogError::internal("Key generations should not be empty after check")
                })?;
                let age = Utc::now() - latest.created_at;
                if age.to_std().unwrap_or(std::time::Duration::from_secs(0))
                    > std::time::Duration::from_secs(
                        self.key_evolution_engine.evolution_config.generation_limit as u64,
                    )
                {
                    Ok(KeyEvolutionStatus::ReadyForEvolution)
                } else {
                    Ok(KeyEvolutionStatus::Stable)
                }
            }
        } else {
            Ok(KeyEvolutionStatus::Uninitialized)
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn classify_user_behavior(&str,
        entropy_quality: f64,
        commercial_indicators: f64,
        key_evolution: KeyEvolutionStatus,
    ) -> CommercialClassification {
        let human_score = entropy_quality
            + match key_evolution {
                KeyEvolutionStatus::NewLineage => 0.1,
                KeyEvolutionStatus::ReadyForEvolution => 0.2,
                KeyEvolutionStatus::Stable => 0.0,
                KeyEvolutionStatus::Uninitialized => -0.2,
            };
        let commercial_score = commercial_indicators;
        if human_score > 0.7 && commercial_score < 0.3 {
            CommercialClassification::Human {
                confidence: (human_score - commercial_score).min(1.0),
            }
        } else if commercial_score > 0.6 {
            CommercialClassification::Commercial {
                confidence: commercial_score,
                risk_level: if commercial_score > 0.8 {
                    ExtractionRisk::High
                } else {
                    ExtractionRisk::Medium
                },
            }
        } else {
            CommercialClassification::Uncertain {
                human_probability: human_score / (human_score + commercial_score),
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyEvolutionStatus {
    /// State indicating uninitialized
    Uninitialized,
    /// Represents new lineage variant
    NewLineage,
    /// Represents stable variant
    Stable,
    ReadyForEvolution,
}

pub use crate::adapters::UniversalRequest;
