// SPDX-License-Identifier: AGPL-3.0-only

// Trust Evolution Dynamics - Dynamic Trust Building and Healing
//
// This module implements TrustEvolution patterns that replace binary trusted/untrusted
// states with dynamic trust building, healing, and evolution based on interaction
// patterns and ecosystem behavior.

use crate::constants::time;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, SystemTime};

use super::InteractionResult;

/// Dynamic trust evolution states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustEvolution {
    /// Actively building trust through positive interactions
    Building {
        current_level: f64,
        growth_rate: f64,
        building_factors: Vec<TrustBuildingFactor>,
        milestone_progress: f64,
    },
    
    /// Trust is flourishing with consistent positive patterns
    Flourishing {
        current_level: f64,
        stability_score: f64,
        flourishing_indicators: Vec<FlourishingIndicator>,
        sustained_duration: Duration,
    },
    
    /// Questioning trust due to concerning patterns
    Questioning {
        current_level: f64,
        concern_factors: Vec<TrustConcern>,
        questioning_duration: Duration,
        resolution_path: Vec<String>,
    },
    
    /// Actively healing trust after conflicts or issues
    Healing {
        current_level: f64,
        healing_progress: f64,
        healing_actions: Vec<HealingAction>,
        recovery_timeline: Duration,
    },
    
    /// Transforming trust patterns through significant change
    Transforming {
        current_level: f64,
        transformation_type: TransformationType,
        progress_indicators: Vec<String>,
        expected_outcome: TrustLevel,
    },
}

/// Trust level classifications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustLevel {
    /// Exceptional trust (0.9-1.0)
    Exceptional,
    /// High trust (0.7-0.9)
    High,
    /// Moderate trust (0.5-0.7)
    Moderate,
    /// Limited trust (0.3-0.5)
    Limited,
    /// Minimal trust (0.1-0.3)
    Minimal,
    /// No trust (0.0-0.1)
    None,
}

/// Factors that contribute to trust building
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustBuildingFactor {
    ConsistentPositiveInteractions,
    ReliableCommitmentFulfillment,
    TransparentCommunication,
    ProactiveSupport,
    ConflictResolution,
    KnowledgeSharing,
    CommunityContribution,
}

/// Indicators of flourishing trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlourishingIndicator {
    MutualSupport,
    CollaborativeInnovation,
    ConflictPrevention,
    KnowledgeExchange,
    ResourceSharing,
    CommunityLeadership,
}

/// Concerns that cause trust questioning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustConcern {
    InconsistentBehavior,
    UnmetCommitments,
    CommunicationBreakdown,
    ResourceMisuse,
    PolicyViolations,
    CommunityDisruption,
}

/// Actions taken to heal trust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HealingAction {
    Acknowledgment(String),
    Apology(String),
    Restitution(String),
    ProcessImprovement(String),
    CommunityService(String),
    MentoredLearning(String),
}

/// Types of trust transformation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransformationType {
    RecoveryFromConflict,
    EscalationToStewardship,
    TransitionToCollaboration,
    EvolutionToMentorship,
    AdaptationToChange,
}

/// Trust transition record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustTransition {
    pub from_state: TrustEvolution,
    pub to_state: TrustEvolution,
    pub trigger_event: String,
    pub timestamp: SystemTime,
    pub impact_assessment: f64,
}

/// Trust metrics and analytics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMetrics {
    pub current_level: f64,
    pub trend: TrustTrend,
    pub stability: f64,
    pub interaction_history: VecDeque<TrustInteractionRecord>,
    pub milestone_achievements: Vec<TrustMilestone>,
}

/// Trust trend analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrustTrend {
    StronglyIncreasing,
    Increasing,
    Stable,
    Decreasing,
    StronglyDecreasing,
    Volatile,
}

/// Record of trust-affecting interactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustInteractionRecord {
    pub interaction_type: String,
    pub trust_impact: f64,
    pub timestamp: SystemTime,
    pub context: String,
}

/// Trust milestone achievements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustMilestone {
    pub milestone_name: String,
    pub achieved_at: SystemTime,
    pub trust_level_at_achievement: f64,
    pub significance: String,
}

impl TrustEvolution {
    /// Get the current trust level
    pub fn current_level(&self) -> f64 {
        match self {
            TrustEvolution::Building { current_level, .. } => *current_level,
            TrustEvolution::Flourishing { current_level, .. } => *current_level,
            TrustEvolution::Questioning { current_level, .. } => *current_level,
            TrustEvolution::Healing { current_level, .. } => *current_level,
            TrustEvolution::Transforming { current_level, .. } => *current_level,
        }
    }
    
    /// Evolve trust based on an interaction result
    pub fn evolve_from_interaction(&mut self, interaction: &InteractionResult) -> Result<()> {
        let impact = self.calculate_interaction_impact(interaction);
        
        match self {
            TrustEvolution::Building { current_level, growth_rate, .. } => {
                *current_level = (*current_level + impact * *growth_rate).clamp(0.0, 1.0);
                
                // Transition to Flourishing if consistently high
                if *current_level > 0.8 {
                    *self = TrustEvolution::Flourishing {
                        current_level: *current_level,
                        stability_score: 0.8,
                        flourishing_indicators: vec![FlourishingIndicator::MutualSupport],
                        sustained_duration: Duration::from_secs(0),
                    };
                }
            }
            
            TrustEvolution::Flourishing { current_level, .. } => {
                *current_level = (*current_level + impact * 0.1).clamp(0.0, 1.0);
                
                // Transition to Questioning if negative impact
                if impact < -0.2 {
                    *self = TrustEvolution::Questioning {
                        current_level: *current_level,
                        concern_factors: vec![TrustConcern::InconsistentBehavior],
                        questioning_duration: Duration::from_secs(0),
                        resolution_path: vec!["Address concerns".to_string()],
                    };
                }
            }
            
            TrustEvolution::Questioning { current_level, .. } => {
                *current_level = (*current_level + impact * 0.5).clamp(0.0, 1.0);
                
                // Transition to Healing if positive steps taken
                if impact > 0.1 {
                    *self = TrustEvolution::Healing {
                        current_level: *current_level,
                        healing_progress: 0.2,
                        healing_actions: vec![HealingAction::Acknowledgment("Addressing concerns".to_string())],
                        recovery_timeline: Duration::from_secs(time::SECONDS_PER_DAY * 7), // 1 week
                    };
                }
            }
            
            TrustEvolution::Healing { current_level, healing_progress, .. } => {
                *current_level = (*current_level + impact * 0.8).clamp(0.0, 1.0);
                *healing_progress += impact.abs() * 0.3;
                
                // Transition back to Building if healing progresses well
                if *healing_progress > 0.8 {
                    *self = TrustEvolution::Building {
                        current_level: *current_level,
                        growth_rate: 0.1,
                        building_factors: vec![TrustBuildingFactor::ConflictResolution],
                        milestone_progress: 0.0,
                    };
                }
            }
            
            TrustEvolution::Transforming { current_level, .. } => {
                *current_level = (*current_level + impact * 0.3).clamp(0.0, 1.0);
                // Transformation logic would be more complex in practice
            }
        }
        
        Ok(())
    }
    
    /// Calculate the impact of an interaction on trust
    fn calculate_interaction_impact(&self, interaction: &InteractionResult) -> f64 {
        match interaction {
            InteractionResult::Positive { impact, .. } => *impact,
            InteractionResult::Neutral { .. } => 0.0,
            InteractionResult::Concerning { severity, .. } => -severity * 0.5,
            InteractionResult::Harmful { damage, .. } => -damage,
        }
    }
    
    /// Get the trust level classification
    pub fn trust_level_classification(&self) -> TrustLevel {
        let level = self.current_level();
        match level {
            l if l >= 0.9 => TrustLevel::Exceptional,
            l if l >= 0.7 => TrustLevel::High,
            l if l >= 0.5 => TrustLevel::Moderate,
            l if l >= 0.3 => TrustLevel::Limited,
            l if l >= 0.1 => TrustLevel::Minimal,
            _ => TrustLevel::None,
        }
    }
    
    /// Check if trust allows a specific action
    pub fn allows_action(&self, required_trust_level: f64) -> bool {
        self.current_level() >= required_trust_level
    }
    
    /// Get recommendations for trust improvement
    pub fn improvement_recommendations(&self) -> Vec<String> {
        match self {
            TrustEvolution::Building { .. } => vec![
                "Continue consistent positive interactions".to_string(),
                "Focus on reliability and commitment fulfillment".to_string(),
            ],
            TrustEvolution::Flourishing { .. } => vec![
                "Maintain current excellence".to_string(),
                "Consider mentoring others".to_string(),
            ],
            TrustEvolution::Questioning { resolution_path, .. } => resolution_path.clone(),
            TrustEvolution::Healing { healing_actions, .. } => {
                healing_actions.iter().map(|action| format!("Complete: {:?}", action)).collect()
            }
            TrustEvolution::Transforming { progress_indicators, .. } => progress_indicators.clone(),
        }
    }
}

impl TrustMetrics {
    /// Create new trust metrics
    pub fn new(initial_level: f64) -> Self {
        Self {
            current_level: initial_level,
            trend: TrustTrend::Stable,
            stability: 0.5,
            interaction_history: VecDeque::with_capacity(beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE),
            milestone_achievements: Vec::new(),
        }
    }
    
    /// Record a trust interaction
    pub fn record_interaction(
        &mut self,
        interaction_type: String,
        trust_impact: f64,
        context: String,
    ) {
        let record = TrustInteractionRecord {
            interaction_type,
            trust_impact,
            timestamp: SystemTime::now(),
            context,
        };
        
        self.interaction_history.push_back(record);
        
        // Keep only recent interactions
        if self.interaction_history.len() > beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u32 {
            self.interaction_history.pop_front();
        }
        
        // Update current level and trend
        self.current_level = (self.current_level + trust_impact).clamp(0.0, 1.0);
        self.update_trend();
    }
    
    /// Update trust trend based on recent interactions
    fn update_trend(&mut self) {
        if self.interaction_history.len() < 5 {
            self.trend = TrustTrend::Stable;
            return;
        }
        
        let recent: Vec<f64> = self.interaction_history
            .iter()
            .rev()
            .take(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE)
            .map(|r| r.trust_impact)
            .collect();
        
        let avg_impact: f64 = recent.iter().sum::<f64>() / recent.len() as f64;
        let variance: f64 = recent.iter()
            .map(|&x| (x - avg_impact).powi(2))
            .sum::<f64>() / recent.len() as f64;
        
        self.trend = match (avg_impact, variance) {
            (_avg, var) if var > 0.1 => TrustTrend::Volatile,
            (avg, _) if avg > 0.1 => TrustTrend::StronglyIncreasing,
            (avg, _) if avg > 0.05 => TrustTrend::Increasing,
            (avg, _) if avg < -0.1 => TrustTrend::StronglyDecreasing,
            (avg, _) if avg < -0.05 => TrustTrend::Decreasing,
            _ => TrustTrend::Stable,
        };
        
        self.stability = 1.0 - variance.min(1.0);
    }
    
    /// Add a trust milestone
    pub fn add_milestone(&mut self, milestone: TrustMilestone) {
        self.milestone_achievements.push(milestone);
    }
} 