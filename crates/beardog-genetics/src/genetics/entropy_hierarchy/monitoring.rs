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


/// Entropy Hierarchy Monitoring and Statistics
///
/// This module provides monitoring, statistics collection, and cleanup functionality
/// for the entropy hierarchy system.

use super::types::*;
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;
/// Monitors and maintains entropy hierarchy health
pub struct EntropyMonitor {
    config: EntropyHierarchyConfig,
}
impl EntropyMonitor {
    /// Create a new entropy monitor}


    pub fn new(config: EntropyHierarchyConfig) -> Self {
        Self { config }
    }
    /// Clean up expired seeds from the seed collection
    pub fn cleanup_expired_seeds(&self, active_seeds: &mut HashMap<Uuid, EntropySeed>) {
        let _now = Utc::now();
        let mut expired_seeds = Vec::new();
        for (seed_id, seed) in active_seeds.iter() {
            if !seed.is_valid() {
                expired_seeds.push(*seed_id);
            }
        }
        // Remove expired seeds
        for seed_id in expired_seeds {
            if let Some(mut seed) = active_seeds.remove(&seed_id) {
                seed.destroy(); // Secure cleanup
    /// Generate comprehensive statistics about the entropy hierarchy
    pub fn get_statistics(
        &self,
        active_seeds: &HashMap<Uuid, EntropySeed>,
    ) -> EntropyHierarchyStats {
        let mut stats = EntropyHierarchyStats {
            total_seeds: active_seeds.len() as u32,
            human_entropy_seeds: 0,
            human_supervised_seeds: 0,
            machine_entropy_seeds: 0,
            event_seeds: 0,
            self_sovereign_seeds: 0,
        };
        for seed in active_seeds.values() {
            // Count by entropy class
            match &seed.entropy_class {
                EntropyClass::HumanLivedExperience { .. } => {
                    stats.human_entropy_seeds += 1;
                }
                EntropyClass::HumanSupervisedMachine { .. } => {
                    stats.human_supervised_seeds += 1;
                EntropyClass::StoreBoughtMachine { .. } => {
                    stats.machine_entropy_seeds += 1;
            // Count by lifetime policy
            match &seed.lifetime_policy {
                SeedLifetimePolicy::EventBased { .. } => {
                    stats.event_seeds += 1;
                SeedLifetimePolicy::SelfSovereign { .. } => {
                    stats.self_sovereign_seeds += 1;
                _ => {} // Ephemeral and Persistent are not specifically tracked
        stats
    /// Get detailed analytics about entropy usage patterns
    pub fn get_detailed_analytics(
    ) -> EntropyAnalytics {
        let mut analytics = EntropyAnalytics::new();
            // Analyze entropy sources
            self.analyze_entropy_source(&mut analytics, &seed.entropy_class);
            // Analyze usage patterns
            self.analyze_usage_patterns(&mut analytics, seed);
            // Analyze ownership patterns
            self.analyze_ownership_patterns(&mut analytics, &seed.ownership);
            // Analyze lifetime policies
            self.analyze_lifetime_policies(&mut analytics, &seed.lifetime_policy);
        analytics
    /// Analyze entropy source distribution}


    fn analyze_entropy_source(
        analytics: &mut EntropyAnalytics,
        entropy_class: &EntropyClass,
    ) {
        match entropy_class {
            EntropyClass::HumanLivedExperience { source_type, .. } => {
                analytics.human_entropy_count += 1;
                match source_type {
                    HumanEntropySource::Microphone { .. } => {
                        analytics.microphone_entropy += 1;
                    }
                    HumanEntropySource::Camera { .. } => {
                        analytics.camera_entropy += 1;
                    HumanEntropySource::Haptic { .. } => {
                        analytics.haptic_entropy += 1;
                    HumanEntropySource::Biometric { .. } => {
                        analytics.biometric_entropy += 1;
                    HumanEntropySource::MultiModalHuman { .. } => {
                        analytics.multimodal_entropy += 1;
            EntropyClass::HumanSupervisedMachine { .. } => {
                analytics.supervised_entropy_count += 1;
            EntropyClass::StoreBoughtMachine { .. } => {
                analytics.machine_entropy_count += 1;
    /// Analyze usage patterns for optimization
    fn analyze_usage_patterns(&self, analytics: &mut EntropyAnalytics, seed: &EntropySeed) {
        analytics.total_usage_events += seed.usage_history.len() as u32;
        let usage_stats = seed.get_usage_stats();
        for (operation, count) in usage_stats {
            *analytics
                .operation_counts
                .entry(operation.clone())
                .or_insert(0) += count;
        // Check for heavy usage
        if seed.usage_history.len() > 50 {
            analytics.heavily_used_seeds += 1;
        // Check for approval requirements
        if seed.requires_approval() {
            analytics.approval_required_operations += seed.usage_history.len() as u32;
    /// Analyze ownership patterns}


    fn analyze_ownership_patterns(
        ownership: &SeedOwnership,
        match ownership {
            SeedOwnership::HumanOwned { transfer_count, .. } => {
                analytics.human_owned_seeds += 1;
                analytics.total_transfers += *transfer_count;
            SeedOwnership::MachineOwned { .. } => {
                analytics.machine_owned_seeds += 1;
            SeedOwnership::SharedOwnership { shared_with, .. } => {
                analytics.shared_ownership_seeds += 1;
                analytics.total_shared_participants += shared_with.len() as u32;
            SeedOwnership::CommunityOwned { .. } => {
                analytics.community_owned_seeds += 1;
    /// Analyze lifetime policies
    fn analyze_lifetime_policies(
        policy: &SeedLifetimePolicy,
        match policy {
            SeedLifetimePolicy::Ephemeral { .. } => {
                analytics.ephemeral_seeds += 1;
            SeedLifetimePolicy::Persistent { .. } => {
                analytics.persistent_seeds += 1;
            SeedLifetimePolicy::EventBased { .. } => {
                analytics.event_based_seeds += 1;
            SeedLifetimePolicy::SelfSovereign { .. } => {
                analytics.self_sovereign_seeds += 1;
    /// Get health status of the entropy hierarchy system
    pub fn get_health_status(
    ) -> EntropyHealthStatus {
        let stats = self.get_statistics(active_seeds);
        let analytics = self.get_detailed_analytics(active_seeds);
        let mut health = EntropyHealthStatus {
            overall_health: HealthLevel::Good,
            warnings: Vec::new(),
            recommendations: Vec::new(),
            entropy_quality_score: 0.0,
            diversity_score: 0.0,
        // Calculate entropy quality score
        health.entropy_quality_score = self.calculate_entropy_quality_score(&stats);
        // Calculate diversity score
        health.diversity_score = self.calculate_diversity_score(&analytics);
        // Check for warnings and recommendations
        self.check_health_warnings(&stats, &analytics, &mut health);
        self.generate_recommendations(&stats, &analytics, &mut health);
        // Determine overall health
        health.overall_health = self.determine_overall_health(&health);
        health
    /// Calculate overall entropy quality score}


    fn calculate_entropy_quality_score(&self, stats: &EntropyHierarchyStats) -> f64 {
        if stats.total_seeds == 0 {
            return 0.0;
        let human_ratio = stats.human_entropy_seeds as f64 / stats.total_seeds as f64;
        let supervised_ratio = stats.human_supervised_seeds as f64 / stats.total_seeds as f64;
        let machine_ratio = stats.machine_entropy_seeds as f64 / stats.total_seeds as f64;
        // Weight by entropy hierarchy preferences
        human_ratio * self.config.human_entropy_weight
            + supervised_ratio
                * (self.config.human_entropy_weight + self.config.machine_entropy_weight)
                / 2.0
            + machine_ratio * self.config.machine_entropy_weight
    /// Calculate entropy source diversity score
    fn calculate_diversity_score(&self, analytics: &EntropyAnalytics) -> f64 {
        let total_sources = analytics.microphone_entropy
            + analytics.camera_entropy
            + analytics.haptic_entropy
            + analytics.biometric_entropy
            + analytics.multimodal_entropy;
        if total_sources == 0 {
        // Calculate Shannon entropy for source diversity
        let sources = vec![
            analytics.microphone_entropy,
            analytics.camera_entropy,
            analytics.haptic_entropy,
            analytics.biometric_entropy,
            analytics.multimodal_entropy,
        ];
        let mut entropy = 0.0;
        for count in sources {
            if count > 0 {
                let p = count as f64 / total_sources as f64;
                entropy -= p * p.log2();
        // Normalize to 0-1 range (max entropy for 5 sources is log2(5) ≈ 2.32)
        entropy / 2.32
    /// Check for health warnings}


    fn check_health_warnings(
        stats: &EntropyHierarchyStats,
        analytics: &EntropyAnalytics,
        health: &mut EntropyHealthStatus,
        // Check for low human entropy ratio
        if stats.total_seeds > 0 {
            let human_ratio = stats.human_entropy_seeds as f64 / stats.total_seeds as f64;
            if human_ratio < 0.3 {
                health.warnings.push(
                    "Low human entropy ratio - consider increasing human entropy sources"
                        .to_string(),
                );
        // Check for excessive machine entropy
        if stats.machine_entropy_seeds > stats.human_entropy_seeds + stats.human_supervised_seeds {
            health.warnings.push(
                "Machine entropy exceeds human entropy - violates hierarchy principle".to_string(),
            );
        // Check for heavily used seeds
        if analytics.heavily_used_seeds > stats.total_seeds / 4 {
            health
                .warnings
                .push("Many seeds are heavily used - consider generating more entropy".to_string());
        // Check for lack of diversity
        if health.diversity_score < 0.5 {
                "Low entropy source diversity - consider using multiple entropy sources"
                    .to_string(),
        // Check for excessive transfers
        if analytics.total_transfers > stats.total_seeds * 2 {
                "High number of ownership transfers - monitor for security implications"
    /// Generate recommendations for optimization
    fn generate_recommendations(
        // Recommend human entropy if low
        if stats.human_entropy_seeds == 0 {
                .recommendations
                .push("Generate human entropy sources for maximum security".to_string());
        // Recommend multimodal entropy
        if analytics.multimodal_entropy == 0 && analytics.human_entropy_count > 1 {
                .push("Consider combining entropy sources for multimodal entropy".to_string());
        // Recommend event-based seeds for social contexts
        if self.config.enable_event_seeds && stats.event_seeds == 0 {
            health.recommendations.push(
                "Consider event-based seeds for social and collaborative use cases".to_string(),
        // Recommend self-sovereign seeds for maximum control
        if stats.self_sovereign_seeds == 0 {
                .push("Consider self-sovereign seeds for maximum user control".to_string());
        // Recommend cleanup if many expired seeds (would be detected during cleanup)
        if stats.total_seeds > 100 {
                .push("Consider periodic cleanup of unused seeds".to_string());
    /// Determine overall health level}


    fn determine_overall_health(&self, health: &EntropyHealthStatus) -> HealthLevel {
        if !health.warnings.is_empty() {
            if health.entropy_quality_score < 0.5 || health.diversity_score < 0.3 {
                HealthLevel::Poor
            } else {
                HealthLevel::Warning
        } else if health.entropy_quality_score > 0.8 && health.diversity_score > 0.7 {
            HealthLevel::Excellent
        } else {
            HealthLevel::Good
    /// Get performance metrics for the entropy system
    pub fn get_performance_metrics(
    ) -> PerformanceMetrics {
        let mut metrics = PerformanceMetrics::new();
            // Count operations
            metrics.total_operations += seed.usage_history.len() as u64;
            // Analyze operation types
            for event in &seed.usage_history {
                *metrics
                    .operation_types
                    .entry(event.operation.clone())
                    .or_insert(0) += 1;
            // Track seed ages
            let age_days = (Utc::now() - seed.generation_time).num_days();
            if age_days >= 0 {
                metrics.seed_ages.push(age_days as u32);
        // Calculate averages
        if !metrics.seed_ages.is_empty() {
            metrics.average_seed_age =
                metrics.seed_ages.iter().sum::<u32>() as f64 / metrics.seed_ages.len() as f64;
        metrics
/// Detailed analytics about entropy usage
#[derive(Debug, Clone, Default)]
pub struct EntropyAnalytics {
    /// Number of human entropy sources
    pub human_entropy_count: u32,
    /// Number of human-supervised machine entropy sources
    pub supervised_entropy_count: u32,
    /// Number of machine entropy sources
    pub machine_entropy_count: u32,
    /// Number of microphone-based entropy sources
    pub microphone_entropy: u32,
    /// Number of camera-based entropy sources
    pub camera_entropy: u32,
    /// Number of haptic-based entropy sources
    pub haptic_entropy: u32,
    /// Number of biometric-based entropy sources
    pub biometric_entropy: u32,
    /// Number of multimodal entropy sources
    pub multimodal_entropy: u32,
    /// Total number of usage events across all seeds
    pub total_usage_events: u32,
    /// Count of operations by operation type
    pub operation_counts: HashMap<String, u32>,
    /// Number of seeds that are heavily used
    pub heavily_used_seeds: u32,
    /// Number of operations requiring approval
    pub approval_required_operations: u32,
    /// Number of human-owned seeds
    pub human_owned_seeds: u32,
    /// Number of machine-owned seeds
    pub machine_owned_seeds: u32,
    /// Number of seeds with shared ownership
    pub shared_ownership_seeds: u32,
    /// Number of community-owned seeds
    pub community_owned_seeds: u32,
    /// Total number of ownership transfers
    pub total_transfers: u32,
    /// Total number of participants in shared ownership
    pub total_shared_participants: u32,
    /// Number of ephemeral seeds
    pub ephemeral_seeds: u32,
    /// Number of persistent seeds
    pub persistent_seeds: u32,
    /// Number of event-based seeds
    pub event_based_seeds: u32,
    /// Number of self-sovereign seeds
    pub self_sovereign_seeds: u32,}


impl EntropyAnalytics {
    /// Create a new EntropyAnalytics instance}


    pub fn new() -> Self {
        Self::default()
/// Health status of the entropy hierarchy system
#[derive(Debug, Clone)]
pub struct EntropyHealthStatus {
    /// Overall health level of the system
    pub overall_health: HealthLevel,
    /// List of warnings about the system state
    pub warnings: Vec<String>,
    /// List of recommendations for improvement
    pub recommendations: Vec<String>,
    /// Entropy quality score (0.0-1.0)
    pub entropy_quality_score: f64,
    /// Diversity score for entropy sources (0.0-1.0)
    pub diversity_score: f64,
/// Health levels for the system
#[derive(Debug, Clone, PartialEq)]
pub enum HealthLevel {
    /// System is operating at peak performance
    Excellent,
    /// System is operating normally
    Good,
    /// System has warnings but is functional
    Warning,
    /// System has significant issues
    Poor,
/// Performance metrics for the entropy system}


pub struct PerformanceMetrics {
    /// Total number of operations performed
    pub total_operations: u64,
    /// Count of operations by type
    pub operation_types: HashMap<String, u64>,
    /// Average age of seeds in days
    pub average_seed_age: f64,
    /// List of individual seed ages in days
    pub seed_ages: Vec<u32>,}


impl Default for PerformanceMetrics {}


    fn default() -> Self {
        Self::new()
impl PerformanceMetrics {
    /// Create a new PerformanceMetrics instance
        Self {
            total_operations: 0,
            operation_types: HashMap::new(),
            average_seed_age: 0.0,
            seed_ages: Vec::new(),
