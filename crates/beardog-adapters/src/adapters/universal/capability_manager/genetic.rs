

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticCapabilityProfile {

    pub genetic_id: String,

    pub parent_capabilities: Vec<String>,

    pub inherited_traits: Vec<CapabilityTrait>,

    pub evolved_capabilities: Vec<String>,

    pub fitness_score: f64,

    pub generation: u32,

    pub mutation_history: Vec<CapabilityMutation>,
}

pub struct CapabilityTrait {

    pub trait_id: String,

    pub trait_type: CapabilityTraitType,

    pub expression_level: f64,

    pub dominance: f64,

    pub heritability: f64,

pub enum CapabilityTraitType {

    Performance,

    Reliability,

    Scalability,

    Security,

    Efficiency,

    Adaptability,

    Compatibility,

    Innovation,

pub struct CapabilityMutation {

    pub mutation_id: String,

    pub mutation_type: MutationType,

    pub affected_capabilities: Vec<String>,

    pub mutation_strength: f64,

    pub timestamp: DateTime<Utc>,

    pub trigger: MutationTrigger,

pub enum MutationType {

    Enhancement,

    Specialization,

    Hybridization,

    Adaptation,

    Optimization,

pub enum MutationTrigger {

    EnvironmentalPressure,

    CrossBreeding,
    PerformanceOptimization,

    SecurityRequirement,

    UserDemand,}

impl GeneticCapabilityProfile {

    pub fn new(parent_capabilities: Vec<&str>, inherited_traits: Vec<CapabilityTrait>) -> Self {
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

    pub fn apply_mutation(&mut self, mutation: CapabilityMutation) {

        match mutation.mutation_type {
            MutationType::Enhancement => {
                self.fitness_score += mutation.mutation_strength * 0.2;
            }
            MutationType::Specialization => {
                self.fitness_score += mutation.mutation_strength * 0.15;
            MutationType::Hybridization => {
                self.fitness_score += mutation.mutation_strength * 0.25;
            MutationType::Adaptation => {
                self.fitness_score += mutation.mutation_strength * 0.3;
            MutationType::Optimization => {
                self.fitness_score += mutation.mutation_strength * 0.1;

        self.fitness_score = self.fitness_score.clamp(0.0, 1.0);

        self.mutation_history.push(mutation);

    pub fn calculate_dominance(&self, trait_type: &CapabilityTraitType) -> f64 {
        let matching_traits: Vec<_> = self
            .inherited_traits
            .iter()
            .filter(|t| std::mem::discriminant(&t.trait_type) == std::mem::discriminant(trait_type))
            .collect();
        if matching_traits.is_empty() {
            return 0.0;
        matching_traits
            .map(|t| t.dominance * t.expression_level)
            .sum::<f64>()
            / matching_traits.len() as f64
impl CapabilityTrait {

    pub fn new(
        trait_type: CapabilityTraitType,
        expression_level: f64,
        dominance: f64,
        heritability: f64,
    ) -> Self {
            trait_id: Uuid::new_v4().to_string(),
            trait_type,
            expression_level: expression_level.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
            heritability: heritability.clamp(0.0, 1.0),

    pub fn performance(expression_level: f64) -> Self {
        Self::new(
            CapabilityTraitType::Performance,
            expression_level,
            0.8, // High dominance for performance
            0.9, // High heritability
        )

    pub fn security(expression_level: f64) -> Self {
            CapabilityTraitType::Security,
            0.9,  // Very high dominance for security
            0.85, // High heritability

    pub fn reliability(expression_level: f64) -> Self {
            CapabilityTraitType::Reliability,
            0.85, // High dominance for reliability
            0.8,  // High heritability

    pub fn scalability(expression_level: f64) -> Self {
            CapabilityTraitType::Scalability,
            0.7,  // Medium-high dominance
            0.75, // Medium-high heritability

    pub fn efficiency(expression_level: f64) -> Self {
            CapabilityTraitType::Efficiency,
            0.6, // Medium dominance
            0.7, // Medium heritability

    pub fn adaptability(expression_level: f64) -> Self {
            CapabilityTraitType::Adaptability,
            0.75, // Medium-high dominance

    pub fn compatibility(expression_level: f64) -> Self {
            CapabilityTraitType::Compatibility,
            0.65, // Medium dominance

    pub fn innovation(expression_level: f64) -> Self {
            CapabilityTraitType::Innovation,
            0.5, // Lower dominance - innovation is often recessive
            0.6, // Medium heritability
impl CapabilityMutation {

        mutation_type: MutationType,
        affected_capabilities: Vec<String>,
        mutation_strength: f64,
        trigger: MutationTrigger,
            mutation_id: Uuid::new_v4().to_string(),
            mutation_type,
            affected_capabilities,
            mutation_strength: mutation_strength.clamp(0.0, 1.0),
            timestamp: Utc::now(),
            trigger,

    pub fn enhancement(capabilities: Vec<&str>, strength: f64) -> Self {
            MutationType::Enhancement,
            capabilities,
            strength,
            MutationTrigger::PerformanceOptimization,

    pub fn specialization(capabilities: Vec<&str>, strength: f64) -> Self {
            MutationType::Specialization,
            MutationTrigger::UserDemand,

    pub fn hybridization(capabilities: Vec<&str>, strength: f64) -> Self {
            MutationType::Hybridization,
            MutationTrigger::CrossBreeding,

    pub fn adaptation(capabilities: Vec<&str>, strength: f64) -> Self {
            MutationType::Adaptation,
            MutationTrigger::EnvironmentalPressure,

    pub fn optimization(capabilities: Vec<&str>, strength: f64) -> Self {
            MutationType::Optimization,
