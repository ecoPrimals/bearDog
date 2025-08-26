

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use crate::adapters::universal::UniversalRequest;

#[derive(Debug, Clone)]
pub struct CommercialExtractionDetector {

    pub usage_patterns: HashMap<String, UsagePattern>,

    pub entropy_tracking: HashMap<String, EntropyHistory>,

    pub key_evolution_engine: GeneticKeyEvolutionEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsagePattern {

    pub request_frequencies: Vec<(DateTime<Utc>, u32)>,

    pub function_patterns: HashMap<String, u32>,

    pub timing_variance: f64,

    pub network_patterns: NetworkBehaviorPattern,

    pub data_volume_analysis: DataVolumePattern,

    pub classification_confidence: f64,

pub struct NetworkBehaviorPattern {

    pub connection_persistence: f64,

    pub batching_patterns: Vec<u32>,

    pub geographic_consistency: f64,

    pub user_agent_patterns: Vec<String>,

pub struct DataVolumePattern {

    pub total_volume: u64,

    pub processing_rate: f64,

    pub volume_variance: f64,

    pub bulk_operation_score: f64,

pub struct EntropyHistory {

    pub entropy_tier_history: Vec<(DateTime<Utc>, u8)>,

    pub quality_progression: Vec<(DateTime<Utc>, f64)>,

    pub human_entropy_sources: Vec<HumanEntropyUsage>,

    pub key_generations: Vec<KeyGeneration>,

pub struct HumanEntropyUsage {

    pub source_type: String,

    pub usage_frequency: f64,

    pub quality_indicators: Vec<f64>,

    pub consistency_score: f64,

pub struct KeyGeneration {

    pub generation: u32,

    pub fitness_score: f64,

    pub parent_generations: Vec<u32>,

    pub mutations: Vec<String>,

    pub created_at: DateTime<Utc>,

pub struct GeneticKeyEvolutionEngine {

    genetic_lineages: HashMap<String, Vec<KeyGeneration>>,

    pub evolution_config: EvolutionConfig,

pub struct EvolutionConfig {

    pub mutation_rate: f64,

    pub selection_pressure: f64,

    pub generation_lifespan: std::time::Duration,

    pub fitness_threshold: f64,}

impl CommercialExtractionDetector {

    pub fn new() -> Self {
        Self {
            usage_patterns: HashMap::with_capacity(16),
            entropy_tracking: HashMap::with_capacity(16),
            key_evolution_engine: GeneticKeyEvolutionEngine::new(),
        }
    }
impl GeneticKeyEvolutionEngine {

            genetic_lineages: HashMap::with_capacity(16),
            evolution_config: EvolutionConfig::default(),}

impl Default for EvolutionConfig {
    fn default() -> Self {
            mutation_rate: 0.01,
            selection_pressure: 0.8,
            generation_lifespan: std::time::Duration::from_secs(86400), // 24 hours
            fitness_threshold: 0.75,

pub enum CommercialClassification {

    Human { confidence: f64 },

    Commercial {
        confidence: f64,
        risk_level: ExtractionRisk,
    },

    Uncertain { human_probability: f64 },

pub enum AccessLevel {
    Open,       // Full human access
    Restricted, // Limited access with monitoring
    Blocked,    // Commercial extraction blocked

pub enum ExtractionRisk {
    Low,
    Medium,
    High,
    Critical,
