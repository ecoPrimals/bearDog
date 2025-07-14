//! Genetic capability management for spawning and evolution
//!
//! This module provides genetic capability tracking, trait inheritance,
//! mutation mechanisms, and genetic spawning for capability evolution.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::super::traits::*;
use crate::BearDogResult;

/// Genetic capability profile for spawning integration
#[derive(Debug, Clone)]
pub struct GeneticCapabilityProfile {
    /// Unique genetic identifier
    pub genetic_id: String,
    /// Capabilities that served as parents for this genetic profile
    pub parent_capabilities: Vec<String>,
    /// Traits inherited from parent capabilities
    pub inherited_traits: Vec<CapabilityTrait>,
    /// New capabilities that evolved from this profile
    pub evolved_capabilities: Vec<String>,
    /// Fitness score indicating adaptation success
    pub fitness_score: f64,
    /// Generation number in evolutionary chain
    pub generation: u32,
    /// History of mutations applied to this profile
    pub mutation_history: Vec<CapabilityMutation>,
}

/// Trait characteristics for genetic capabilities
#[derive(Debug, Clone)]
pub struct CapabilityTrait {
    /// Unique trait identifier
    pub trait_id: String,
    /// Category of the trait
    pub trait_type: CapabilityTraitType,
    /// Level of trait expression (0.0 to 1.0)
    pub expression_level: f64,
    /// Dominance level when competing with other traits
    pub dominance: f64,
    /// Likelihood of being passed to offspring
    pub heritability: f64,
}

/// Categories of capability traits
#[derive(Debug, Clone)]
pub enum CapabilityTraitType {
    /// Performance-related traits
    Performance,
    /// Reliability and stability traits
    Reliability,
    /// Scalability and capacity traits
    Scalability,
    /// Security and protection traits
    Security,
    /// Efficiency and optimization traits
    Efficiency,
    /// Adaptability and flexibility traits
    Adaptability,
    /// Compatibility and integration traits
    Compatibility,
    /// Innovation and novelty traits
    Innovation,
}

/// Mutation event in genetic capability evolution
#[derive(Debug, Clone)]
pub struct CapabilityMutation {
    /// Unique mutation identifier
    pub mutation_id: String,
    /// Type of mutation applied
    pub mutation_type: MutationType,
    /// Capabilities affected by this mutation
    pub affected_capabilities: Vec<String>,
    /// Strength of the mutation effect (0.0 to 1.0)
    pub mutation_strength: f64,
    /// When the mutation occurred
    pub timestamp: DateTime<Utc>,
    /// What triggered this mutation
    pub trigger: MutationTrigger,
}

/// Types of genetic mutations
#[derive(Debug, Clone)]
pub enum MutationType {
    /// Enhancement of existing traits
    Enhancement,
    /// Specialization for specific use cases
    Specialization,
    /// Hybridization combining multiple traits
    Hybridization,
    /// Adaptation to environmental changes
    Adaptation,
    /// Optimization for better performance
    Optimization,
}

/// Triggers for genetic mutations
#[derive(Debug, Clone)]
pub enum MutationTrigger {
    /// Changes in operating environment
    EnvironmentalPressure,
    /// Combining traits from multiple sources
    CrossBreeding,
    /// Optimization for better performance
    PerformanceOptimization,
    /// Response to security requirements
    SecurityRequirement,
    /// Adaptation to user demand patterns
    UserDemand,
}

impl GeneticCapabilityProfile {
    /// Create a new genetic capability profile
    pub fn new(parent_capabilities: Vec<String>, inherited_traits: Vec<CapabilityTrait>) -> Self {
        Self {
            genetic_id: Uuid::new_v4().to_string(),
            parent_capabilities,
            inherited_traits,
            evolved_capabilities: Vec::new(),
            fitness_score: 0.5, // Start with neutral fitness
            generation: 1,
            mutation_history: Vec::new(),
        }
    }

    /// Apply a mutation to this genetic profile
    pub fn apply_mutation(&mut self, mutation: CapabilityMutation) {
        // Update fitness score based on mutation
        match mutation.mutation_type {
            MutationType::Enhancement => {
                self.fitness_score += mutation.mutation_strength * 0.2;
            },
            MutationType::Specialization => {
                self.fitness_score += mutation.mutation_strength * 0.15;
            },
            MutationType::Hybridization => {
                self.fitness_score += mutation.mutation_strength * 0.25;
            },
            MutationType::Adaptation => {
                self.fitness_score += mutation.mutation_strength * 0.3;
            },
            MutationType::Optimization => {
                self.fitness_score += mutation.mutation_strength * 0.1;
            },
        }

        // Clamp fitness score between 0.0 and 1.0
        self.fitness_score = self.fitness_score.max(0.0).min(1.0);

        // Record mutation in history
        self.mutation_history.push(mutation);
    }

    /// Calculate dominance score for trait inheritance
    pub fn calculate_dominance(&self, trait_type: &CapabilityTraitType) -> f64 {
        let matching_traits: Vec<_> = self.inherited_traits
            .iter()
            .filter(|t| std::mem::discriminant(&t.trait_type) == std::mem::discriminant(trait_type))
            .collect();
        
        if matching_traits.is_empty() {
            return 0.0;
        }
        
        matching_traits
            .iter()
            .map(|t| t.dominance * t.expression_level)
            .sum::<f64>()
            / matching_traits.len() as f64
    }
}

impl CapabilityTrait {
    /// Create a new capability trait
    pub fn new(
        trait_type: CapabilityTraitType,
        expression_level: f64,
        dominance: f64,
        heritability: f64,
    ) -> Self {
        Self {
            trait_id: Uuid::new_v4().to_string(),
            trait_type,
            expression_level: expression_level.max(0.0).min(1.0),
            dominance: dominance.max(0.0).min(1.0),
            heritability: heritability.max(0.0).min(1.0),
        }
    }

    /// Create a performance trait
    pub fn performance(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Performance,
            expression_level,
            0.8, // High dominance for performance
            0.9, // High heritability
        )
    }

    /// Create a security trait
    pub fn security(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Security,
            expression_level,
            0.9, // Very high dominance for security
            0.85, // High heritability
        )
    }

    /// Create a reliability trait
    pub fn reliability(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Reliability,
            expression_level,
            0.85, // High dominance for reliability
            0.8, // High heritability
        )
    }

    /// Create a scalability trait
    pub fn scalability(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Scalability,
            expression_level,
            0.7, // Medium-high dominance
            0.75, // Medium-high heritability
        )
    }

    /// Create an efficiency trait
    pub fn efficiency(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Efficiency,
            expression_level,
            0.6, // Medium dominance
            0.7, // Medium heritability
        )
    }

    /// Create an adaptability trait
    pub fn adaptability(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Adaptability,
            expression_level,
            0.75, // Medium-high dominance
            0.8, // High heritability
        )
    }

    /// Create a compatibility trait
    pub fn compatibility(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Compatibility,
            expression_level,
            0.65, // Medium dominance
            0.75, // Medium-high heritability
        )
    }

    /// Create an innovation trait
    pub fn innovation(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Innovation,
            expression_level,
            0.5, // Lower dominance - innovation is often recessive
            0.6, // Medium heritability
        )
    }
}

impl CapabilityMutation {
    /// Create a new capability mutation
    pub fn new(
        mutation_type: MutationType,
        affected_capabilities: Vec<String>,
        mutation_strength: f64,
        trigger: MutationTrigger,
    ) -> Self {
        Self {
            mutation_id: Uuid::new_v4().to_string(),
            mutation_type,
            affected_capabilities,
            mutation_strength: mutation_strength.max(0.0).min(1.0),
            timestamp: Utc::now(),
            trigger,
        }
    }

    /// Create an enhancement mutation
    pub fn enhancement(capabilities: Vec<String>, strength: f64) -> Self {
        Self::new(
            MutationType::Enhancement,
            capabilities,
            strength,
            MutationTrigger::PerformanceOptimization,
        )
    }

    /// Create a specialization mutation
    pub fn specialization(capabilities: Vec<String>, strength: f64) -> Self {
        Self::new(
            MutationType::Specialization,
            capabilities,
            strength,
            MutationTrigger::UserDemand,
        )
    }

    /// Create a hybridization mutation
    pub fn hybridization(capabilities: Vec<String>, strength: f64) -> Self {
        Self::new(
            MutationType::Hybridization,
            capabilities,
            strength,
            MutationTrigger::CrossBreeding,
        )
    }

    /// Create an adaptation mutation
    pub fn adaptation(capabilities: Vec<String>, strength: f64) -> Self {
        Self::new(
            MutationType::Adaptation,
            capabilities,
            strength,
            MutationTrigger::EnvironmentalPressure,
        )
    }

    /// Create an optimization mutation
    pub fn optimization(capabilities: Vec<String>, strength: f64) -> Self {
        Self::new(
            MutationType::Optimization,
            capabilities,
            strength,
            MutationTrigger::PerformanceOptimization,
        )
    }
} 