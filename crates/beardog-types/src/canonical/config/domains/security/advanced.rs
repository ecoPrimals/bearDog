//! Advanced Security Features Configuration
//!
//! This module provides genetic security, ecosystem membership, and trust computation
//! configuration structures for the BearDog security system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Genetic security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticSecurityConfiguration {
    /// Enable genetic security algorithms
    pub enable_genetic_security: bool,
    /// Genetic algorithm parameters
    pub genetic_parameters: HashMap<String, f64>,
    /// Evolution strategies
    pub evolution_strategies: Vec<String>,
    /// Fitness evaluation criteria
    pub fitness_criteria: Vec<String>,
}

/// Ecosystem membership configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemMembershipConfiguration {
    /// Enable ecosystem membership model
    pub enabled: bool,
    /// Membership levels and permissions
    pub membership_levels: HashMap<String, Vec<String>>,
    /// Membership evolution rules
    pub evolution_rules: Vec<MembershipEvolutionRule>,
    /// Integration with genetics system
    pub genetics_integration: GeneticsIntegrationConfiguration,
}

/// Membership evolution rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembershipEvolutionRule {
    /// Rule name
    pub name: String,
    /// Trigger conditions
    pub conditions: Vec<String>,
    /// Target membership level
    pub target_level: String,
    /// Evolution probability
    pub probability: f64,
}

/// Genetics integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticsIntegrationConfiguration {
    /// Enable genetics integration
    pub enabled: bool,
    /// Genetic factors for security decisions
    pub genetic_factors: Vec<String>,
    /// Integration strength (0.0 to 1.0)
    pub integration_strength: f64,
    /// Genetic validation timeout
    pub validation_timeout_seconds: u64,
}

/// Trust computation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustComputationConfiguration {
    /// Enable trust computation
    pub enabled: bool,
    /// Trust computation algorithm
    pub algorithm: String,
    /// Trust factors and weights
    pub trust_factors: HashMap<String, f64>,
    /// Trust decay parameters
    pub trust_decay: TrustDecayConfiguration,
    /// Evaluation configuration
    pub evaluation: EvaluationConfiguration,
}

/// Trust decay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustDecayConfiguration {
    /// Enable trust decay
    pub enabled: bool,
    /// Decay rate (0.0 to 1.0)
    pub decay_rate: f64,
    /// Decay interval in seconds
    pub decay_interval_seconds: u64,
    /// Minimum trust threshold
    pub minimum_trust: f64,
}

/// Evaluation configuration for trust and security decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationConfiguration {
    /// Evaluation algorithm
    pub algorithm: String,
    /// Evaluation criteria
    pub criteria: Vec<String>,
    /// Evaluation weights
    pub weights: HashMap<String, f64>,
    /// Evaluation timeout
    pub timeout_seconds: u64,
}
