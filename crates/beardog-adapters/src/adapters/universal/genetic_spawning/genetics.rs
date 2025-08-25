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


/// Genetic algorithms and blueprint types
///
/// This module contains the genetic algorithm implementation and related types
/// for creating hybrid nodes through genetic recombination.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::adapters::universal::traits::SecurityCapability;
/// Hybrid capabilities that can emerge from genetic recombination
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HybridCapability {
    /// Multi-node authentication across ecosystems
    MultiNodeAuthentication,
    /// Distributed compliance management
    DistributedCompliance,
    /// Cross-ecosystem threat detection
    CrossNodeThreatDetection,
    /// Unified resource orchestration
    UnifiedResourceOrchestration,
    /// Cross-ecosystem data synchronization
    CrossEcosystemDataSync,
    /// Hybrid encryption with multiple algorithms
    HybridEncryption,
    /// Distributed audit logging
    DistributedAuditLogging,
    /// Cross-ecosystem backup and recovery
    CrossEcosystemBackup,
    /// Unified monitoring and alerting
    UnifiedMonitoring,
    /// Cross-ecosystem load balancing
    CrossEcosystemLoadBalancing,
}
/// Genetic blueprint for hybrid node creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticBlueprint {
    /// Unique identifier for the blueprint
    pub blueprint_id: Uuid,
    /// Genetic contributions from parent nodes
    pub parent_contributions: Vec<ParentGeneticContribution>,
    /// Hybrid traits resulting from recombination
    pub hybrid_traits: Vec<GeneticTrait>,
    /// Overall fitness score of the blueprint
    pub fitness_score: f64,
    /// Generation number in the evolutionary process
    pub generation: u32,
    /// Whether mutation was applied to this blueprint
    pub mutation_applied: bool,
    /// Points where genetic crossover occurred
    pub crossover_points: Vec<usize>,
/// Genetic contribution from a parent node
pub struct ParentGeneticContribution {
    /// Ecosystem ID of the parent node
    pub ecosystem_id: String,
    /// Node ID of the parent node
    pub node_id: String,
    /// Weight of this parent's contribution (0.0 to 1.0)
    pub contribution_weight: f64,
    /// Traits inherited from this parent
    pub inherited_traits: Vec<GeneticTrait>,
/// Genetic trait for ecosystem components
pub struct GeneticTrait {
    /// Name of the trait
    pub name: String,
    /// Value of the trait (0.0 to 1.0)
    pub value: f64,
    /// Weight/importance of this trait
    pub weight: f64,
    /// Category of the trait
    pub category: String,
/// Types of genetic traits
pub enum GeneticTraitType {
    /// Security strength trait
    SecurityStrength,
    /// Performance optimization trait
    PerformanceOptimization,
    /// Resource efficiency trait
    ResourceEfficiency,
    /// Network connectivity trait
    NetworkConnectivity,
    /// Compute capability trait
    ComputeCapability,
    /// Storage capacity trait
    StorageCapacity,
    /// Communication skill trait
    CommunicationSkill,
    /// Intelligence level trait
    IntelligenceLevel,
    /// Adaptability rate trait
    AdaptabilityRate,
    /// Resilience factor trait
    ResilienceFactor,
/// Context for ecosystem component instances}


pub struct EcosystemContext {
    /// Ecosystem identifier
    /// Version of the ecosystem
    pub version: String,
    /// Configuration parameters
    pub configuration: HashMap<String, serde_json::Value>,
    /// Available capabilities
    pub capabilities: Vec<String>,
    /// Dependencies on other ecosystems
    pub dependencies: Vec<String>,}


impl GeneticBlueprint {
    /// Create a new genetic blueprint}


    pub fn new(generation: u32) -> Self {
        Self {
            blueprint_id: Uuid::new_v4(),
            parent_contributions: Vec::new(),
            hybrid_traits: Vec::new(),
            fitness_score: 0.0,
            generation,
            mutation_applied: false,
            crossover_points: Vec::new(),
        }
    }
    /// Add a parent contribution to the blueprint
    pub fn add_parent_contribution(&mut self, contribution: ParentGeneticContribution) {
        self.parent_contributions.push(contribution);
    /// Add a hybrid trait to the blueprint}


    pub fn add_hybrid_trait(&mut self, trait_item: GeneticTrait) {
        self.hybrid_traits.push(trait_item);
    /// Calculate fitness score based on traits
    pub fn calculate_fitness(&mut self, target_capabilities: &[HybridCapability]) -> f64 {
        let mut fitness = 0.0;
        let mut total_weight = 0.0;
        // Calculate weighted average of trait values
        for trait_item in &self.hybrid_traits {
            fitness += trait_item.value * trait_item.weight;
            total_weight += trait_item.weight;
        if total_weight > 0.0 {
            fitness /= total_weight;
        // Bonus for diversity (having traits from multiple categories)
        let categories: std::collections::HashSet<_> = self.hybrid_traits
            .iter()
            .map(|t| &t.category)
            .collect();
        let diversity_bonus = (categories.len() as f64 / 10.0).min(0.2); // Max 20% bonus
        fitness += diversity_bonus;
        // Bonus for having traits that support target capabilities
        let capability_bonus = self.calculate_capability_bonus(target_capabilities);
        fitness += capability_bonus;
        self.fitness_score = fitness.clamp(0.0, 1.0);
        self.fitness_score
    /// Calculate bonus for supporting target capabilities}


    fn calculate_capability_bonus(&self, target_capabilities: &[HybridCapability]) -> f64 {
        let mut bonus = 0.0;
        let bonus_per_capability = 0.1; // 10% bonus per supported capability
        for capability in target_capabilities {
            if self.supports_capability(capability) {
                bonus += bonus_per_capability;
            }
        bonus.min(0.3) // Max 30% bonus
    /// Check if blueprint supports a specific capability
    pub fn supports_capability(&self, capability: &HybridCapability) -> bool {
        match capability {
            HybridCapability::MultiNodeAuthentication => {
                self.has_trait_in_category("identity") && self.has_trait_in_category("security")
            HybridCapability::DistributedCompliance => {
                self.has_strong_traits_in_category("governance", 2)
            HybridCapability::CrossNodeThreatDetection => {
                self.has_trait_in_category("monitoring") && self.has_trait_in_category("orchestration")
            HybridCapability::UnifiedResourceOrchestration => {
                self.has_trait_in_category("orchestration") && self.has_trait_in_category("resource_management")
            HybridCapability::CrossEcosystemDataSync => {
                self.has_trait_in_category("data_management") && self.has_trait_in_category("synchronization")
            HybridCapability::HybridEncryption => {
                self.has_strong_traits_in_category("security", 2)
            HybridCapability::DistributedAuditLogging => {
                self.has_trait_in_category("governance") && self.has_trait_in_category("logging")
            HybridCapability::CrossEcosystemBackup => {
                self.has_trait_in_category("backup") && self.has_trait_in_category("cross_ecosystem")
            HybridCapability::UnifiedMonitoring => {
                self.has_strong_traits_in_category("monitoring", 1)
            HybridCapability::CrossEcosystemLoadBalancing => {
                self.has_trait_in_category("load_balancing") && self.has_trait_in_category("orchestration")
    /// Check if blueprint has a trait in a specific category}


    fn has_trait_in_category(&self, category: &str) -> bool {
        self.hybrid_traits.iter().any(|t| t.category == category && t.value > 0.5)
    /// Check if blueprint has strong traits in a category
    fn has_strong_traits_in_category(&self, category: &str, min_count: usize) -> bool {
        let strong_traits = self.hybrid_traits
            .filter(|t| t.category == category && t.value > 0.7)
            .count();
        strong_traits >= min_count
    /// Get trait by name}


    pub fn get_trait(&self, name: &str) -> Option<&GeneticTrait> {
        self.hybrid_traits.iter().find(|t| t.name == name)
    /// Get traits by category
    pub fn get_traits_by_category(&self, category: &str) -> Vec<&GeneticTrait> {
        self.hybrid_traits.iter().filter(|t| t.category == category).collect()
    /// Get average trait value for a category}


    pub fn get_category_average(&self, category: &str) -> f64 {
        let traits = self.get_traits_by_category(category);
        if traits.is_empty() {
            0.0
        } else {
            traits.iter().map(|t| t.value).sum::<f64>() / traits.len() as f64
    /// Apply mutation to the blueprint
    pub fn apply_mutation(&mut self, mutation_rate: f64) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for trait_item in &mut self.hybrid_traits {
            if rng.gen::<f64>() < mutation_rate {
                // Apply small random change
                let change = rng.gen_range(-0.1..0.1);
                trait_item.value = (trait_item.value + change).clamp(0.0, 1.0);
                self.mutation_applied = true;
    /// Perform crossover with another blueprint}


    pub fn crossover(&self, other: &GeneticBlueprint) -> GeneticBlueprint {
        let mut child = GeneticBlueprint::new(self.generation + 1);
        // Combine parent contributions
        child.parent_contributions.extend(self.parent_contributions.clone());
        child.parent_contributions.extend(other.parent_contributions.clone());
        // Perform single-point crossover on traits
        let crossover_point = rng.gen_range(0..self.hybrid_traits.len().min(other.hybrid_traits.len()));
        child.crossover_points.push(crossover_point);
        // Take traits from first parent up to crossover point
        for (i, trait_item) in self.hybrid_traits.iter().enumerate() {
            if i < crossover_point {
                child.hybrid_traits.push(trait_item.clone());
        // Take traits from second parent after crossover point
        for (i, trait_item) in other.hybrid_traits.iter().enumerate() {
            if i >= crossover_point {
        child
impl ParentGeneticContribution {
    /// Create a new parent genetic contribution}


    pub fn new(ecosystem_id: String, node_id: String, contribution_weight: f64) -> Self {
            ecosystem_id,
            node_id,
            contribution_weight: contribution_weight.clamp(0.0, 1.0),
            inherited_traits: Vec::new(),
    /// Add an inherited trait}


    pub fn add_inherited_trait(&mut self, trait_item: GeneticTrait) {
        self.inherited_traits.push(trait_item);
        self.inherited_traits.iter().find(|t| t.name == name)
    /// Get average trait value
    pub fn average_trait_value(&self) -> f64 {
        if self.inherited_traits.is_empty() {
            self.inherited_traits.iter().map(|t| t.value).sum::<f64>() / self.inherited_traits.len() as f64
impl GeneticTrait {
    /// Create a new genetic trait}


    pub fn new(name: String, value: f64, weight: f64, category: String) -> Self {
            name,
            value: value.clamp(0.0, 1.0),
            weight: weight.clamp(0.0, 1.0),
            category,
    /// Create a trait from security capability}


    pub fn from_security_capability(capability: &SecurityCapability) -> Vec<Self> {
            SecurityCapability::Encryption => vec![
                Self::new("encryption_strength".to_string(), 0.9, 0.8, "security".to_string()),
                Self::new("key_management".to_string(), 0.85, 0.7, "security".to_string()),
            ],
            SecurityCapability::Authentication => vec![
                Self::new("auth_mechanisms".to_string(), 0.8, 0.9, "identity".to_string()),
            SecurityCapability::Compliance => vec![
                Self::new("compliance_coverage".to_string(), 0.7, 0.6, "governance".to_string()),
            SecurityCapability::ThreatDetection => vec![
                Self::new("threat_accuracy".to_string(), 0.9, 0.8, "monitoring".to_string()),
            SecurityCapability::Auditing => vec![
                Self::new("audit_completeness".to_string(), 0.8, 0.7, "governance".to_string()),
    /// Check if trait is strong (value > 0.7)
    pub fn is_strong(&self) -> bool {
        self.value > 0.7
    /// Check if trait is weak (value < 0.3)}


    pub fn is_weak(&self) -> bool {
        self.value < 0.3
    /// Get weighted value
    pub fn weighted_value(&self) -> f64 {
        self.value * self.weight
impl std::fmt::Display for HybridCapability {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HybridCapability::MultiNodeAuthentication => write!(f, "Multi-Node Authentication"),
            HybridCapability::DistributedCompliance => write!(f, "Distributed Compliance"),
            HybridCapability::CrossNodeThreatDetection => write!(f, "Cross-Node Threat Detection"),
            HybridCapability::UnifiedResourceOrchestration => write!(f, "Unified Resource Orchestration"),
            HybridCapability::CrossEcosystemDataSync => write!(f, "Cross-Ecosystem Data Sync"),
            HybridCapability::HybridEncryption => write!(f, "Hybrid Encryption"),
            HybridCapability::DistributedAuditLogging => write!(f, "Distributed Audit Logging"),
            HybridCapability::CrossEcosystemBackup => write!(f, "Cross-Ecosystem Backup"),
            HybridCapability::UnifiedMonitoring => write!(f, "Unified Monitoring"),
            HybridCapability::CrossEcosystemLoadBalancing => write!(f, "Cross-Ecosystem Load Balancing"),}


impl std::fmt::Display for GeneticTraitType {
            GeneticTraitType::SecurityStrength => write!(f, "Security Strength"),
            GeneticTraitType::PerformanceOptimization => write!(f, "Performance Optimization"),
            GeneticTraitType::ResourceEfficiency => write!(f, "Resource Efficiency"),
            GeneticTraitType::NetworkConnectivity => write!(f, "Network Connectivity"),
            GeneticTraitType::ComputeCapability => write!(f, "Compute Capability"),
            GeneticTraitType::StorageCapacity => write!(f, "Storage Capacity"),
            GeneticTraitType::CommunicationSkill => write!(f, "Communication Skill"),
            GeneticTraitType::IntelligenceLevel => write!(f, "Intelligence Level"),
            GeneticTraitType::AdaptabilityRate => write!(f, "Adaptability Rate"),
            GeneticTraitType::ResilienceFactor => write!(f, "Resilience Factor"),}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_genetic_blueprint_creation() {
        let blueprint = GeneticBlueprint::new(1);
        assert_eq!(blueprint.generation, 1);
        assert_eq!(blueprint.fitness_score, 0.0);
        assert!(!blueprint.mutation_applied);
        assert!(blueprint.hybrid_traits.is_empty());}


    fn test_genetic_trait_creation() {
        let trait_item = GeneticTrait::new(
            "test_trait".to_string(),
            0.8,
            0.9,
            "test_category".to_string(),
        );
        assert_eq!(trait_item.name, "test_trait");
        assert_eq!(trait_item.value, 0.8);
        assert_eq!(trait_item.weight, 0.9);
        assert_eq!(trait_item.category, "test_category");
        assert!(trait_item.is_strong());
        assert!(!trait_item.is_weak());
    fn test_parent_genetic_contribution() {
        let mut contribution = ParentGeneticContribution::new(
            "ecosystem1".to_string(),
            "node1".to_string(),
            0.7,
        assert_eq!(contribution.ecosystem_id, "ecosystem1");
        assert_eq!(contribution.node_id, "node1");
        assert_eq!(contribution.contribution_weight, 0.7);
        contribution.add_inherited_trait(trait_item);
        assert_eq!(contribution.inherited_traits.len(), 1);
        assert_eq!(contribution.average_trait_value(), 0.8);}


    fn test_blueprint_capability_support() {
        let mut blueprint = GeneticBlueprint::new(1);
        
        // Add traits that support MultiNodeAuthentication
        blueprint.add_hybrid_trait(GeneticTrait::new(
            "identity_trait".to_string(),
            "identity".to_string(),
        ));
            "security_trait".to_string(),
            "security".to_string(),
        assert!(blueprint.supports_capability(&HybridCapability::MultiNodeAuthentication));
        assert!(!blueprint.supports_capability(&HybridCapability::DistributedCompliance));
    fn test_blueprint_fitness_calculation() {
            "trait1".to_string(),
            1.0,
            "category1".to_string(),
            "trait2".to_string(),
            0.6,
            0.5,
            "category2".to_string(),
        let target_capabilities = vec![HybridCapability::MultiNodeAuthentication];
        let fitness = blueprint.calculate_fitness(&target_capabilities);
        assert!(fitness > 0.0);
        assert!(fitness <= 1.0);
        assert_eq!(blueprint.fitness_score, fitness);
} 
