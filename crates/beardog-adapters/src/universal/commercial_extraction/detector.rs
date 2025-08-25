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


/// Commercial Extraction Detection Engine
///
/// Revolutionary system for distinguishing human users from commercial extraction attempts.
/// Core principle: "Open gates for humans, locked tight for commercial extraction"
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Tracing will be used when detection algorithms are fully implemented

// Import the correct UniversalRequest type that matches capability_adapter
pub use crate::adapters::universal::UniversalRequest;
/// Advanced commercial extraction detection engine
#[derive(Debug, Clone)]
pub struct CommercialExtractionDetector {
    /// Usage pattern analysis
    pub usage_patterns: HashMap<String, UsagePattern>,
    /// Entropy quality tracking
    pub entropy_tracking: HashMap<String, EntropyHistory>,
    /// Genetic key evolution engine
    pub key_evolution_engine: GeneticKeyEvolutionEngine,
}
/// Usage pattern analysis for detecting commercial vs human behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePattern {
    /// Request frequency over time windows
    pub request_frequencies: Vec<(DateTime<Utc>, u32)>,
    /// Function call patterns (commercial vs human-like)
    pub function_patterns: HashMap<String, u32>,
    /// Timing patterns (automation vs human variance)
    pub timing_variance: f64,
    /// Network behavior analysis
    pub network_patterns: NetworkBehaviorPattern,
    /// Data volume analysis
    pub data_volume_analysis: DataVolumePattern,
    /// Classification confidence
    pub classification_confidence: f64,
/// Network behavior pattern analysis
pub struct NetworkBehaviorPattern {
    /// Connection patterns (persistent vs intermittent)
    pub connection_persistence: f64,
    /// Request batching patterns
    pub batching_patterns: Vec<u32>,
    /// Geographic consistency
    pub geographic_consistency: f64,
    /// User agent consistency
    pub user_agent_patterns: Vec<String>,
/// Data volume pattern analysis
pub struct DataVolumePattern {
    /// Total data processed
    pub total_volume: u64,
    /// Processing rate (human vs machine-like)
    pub processing_rate: f64,
    /// Volume consistency (humans are inconsistent)
    pub volume_variance: f64,
    /// Bulk operation detection
    pub bulk_operation_score: f64,
/// Entropy quality history for genetic key evolution
pub struct EntropyHistory {
    /// Entropy tier progression (humans get better over time)
    pub entropy_tier_history: Vec<(DateTime<Utc>, u8)>,
    /// Quality scores over time
    pub quality_progression: Vec<(DateTime<Utc>, f64)>,
    /// Human entropy source usage
    pub human_entropy_sources: Vec<HumanEntropyUsage>,
    /// Key evolution generations
    pub key_generations: Vec<KeyGeneration>,
/// Human entropy usage tracking
pub struct HumanEntropyUsage {
    /// Source type (microphone, camera, haptic)
    pub source_type: String,
    /// Usage frequency
    pub usage_frequency: f64,
    /// Quality indicators
    pub quality_indicators: Vec<f64>,
    /// Consistency (lower = more human-like)
    pub consistency_score: f64,
/// Key generation in genetic evolution
pub struct KeyGeneration {
    /// Generation number
    pub generation: u32,
    /// Fitness score
    pub fitness_score: f64,
    /// Parent generations
    pub parent_generations: Vec<u32>,
    /// Mutation indicators
    pub mutations: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
/// Genetic key evolution engine
pub struct GeneticKeyEvolutionEngine {
    /// Active genetic lineages
    genetic_lineages: HashMap<String, Vec<KeyGeneration>>,
    /// Evolution configuration
    pub evolution_config: EvolutionConfig,
/// Evolution configuration
pub struct EvolutionConfig {
    /// Mutation rate
    pub mutation_rate: f64,
    /// Selection pressure
    pub selection_pressure: f64,
    /// Generation lifespan
    pub generation_lifespan: std::time::Duration,
    /// Fitness threshold
    pub fitness_threshold: f64,}


impl CommercialExtractionDetector {
    /// Create new commercial extraction detector}


    pub fn new() -> Self {
        Self {
            usage_patterns: HashMap::new(),
            entropy_tracking: HashMap::new(),
            key_evolution_engine: GeneticKeyEvolutionEngine::new(),
        }
    }
impl GeneticKeyEvolutionEngine {
    /// Create new genetic key evolution engine
            genetic_lineages: HashMap::new(),
            evolution_config: EvolutionConfig::default(),}


impl Default for EvolutionConfig {
    fn default() -> Self {
            mutation_rate: 0.01,
            selection_pressure: 0.8,
            generation_lifespan: std::time::Duration::from_secs(86400), // 24 hours
            fitness_threshold: 0.75,
/// Commercial vs Human classification
pub enum CommercialClassification {
    /// Confirmed human user
    Human { confidence: f64 },
    /// Suspected commercial extraction
    Commercial {
        confidence: f64,
        risk_level: ExtractionRisk,
    },
    /// Uncertain classification
    Uncertain { human_probability: f64 },
/// Access level based on classification
pub enum AccessLevel {
    Open,       // Full human access
    Restricted, // Limited access with monitoring
    Blocked,    // Commercial extraction blocked
/// Commercial extraction risk level}


pub enum ExtractionRisk {
    Low,
    Medium,
    High,
    Critical,
