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


/// Universal Capability Registry
///
/// **Comprehensive capability discovery and matching system**

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use super::traits::*;
use beardog_errors::BearDogResult;
/// Central registry for discovering and matching capabilities across the entire ecosystem.
/// Provides intelligent capability matching for genetic spawning and service discovery.
pub struct CapabilityRegistry {
    /// Indexed capabilities by ecosystem and instance
    capabilities: Arc<RwLock<HashMap<String, Vec<Capability>>>>,
    /// Capability index for fast searching
    capability_index: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Component compatibility matrix
    compatibility_matrix: Arc<RwLock<HashMap<String, Vec<String>>>>,
    /// Registry statistics
    stats: Arc<RwLock<RegistryStats>>,
}
/// Registry statistics and metrics
#[derive(Debug, Clone, Default)]
pub struct RegistryStats {
    /// Total number of capabilities registered
    pub total_capabilities: u32,
    /// Total number of providers registered
    pub total_providers: u32,
    /// Total number of searches performed
    pub total_searches: u64,
    /// Number of successful matches found
    pub successful_matches: u64,
    /// Timestamp of last registry update
    pub last_updated: chrono::DateTime<chrono::Utc>,
/// Capability search criteria
pub struct CapabilitySearchCriteria {
    /// Required capability category
    pub category: Option<CapabilityCategory>,
    /// Minimum QoS response time in milliseconds
    pub min_qos_response_time_ms: Option<u64>,
    /// Minimum availability percentage required
    pub min_availability_percent: Option<f64>,
    /// Required attributes that must be present
    pub required_attributes: HashMap<String, String>,
/// Capability match result
#[derive(Debug, Clone)]
pub struct CapabilityMatch {
    /// The matched capability
    pub capability: Capability,
    /// Ecosystem ID of the provider
    pub provider_ecosystem: String,
    /// Instance ID of the provider
    pub provider_instance: String,
    /// Match score indicating compatibility (0.0 to 10.0)
    pub match_score: f64,
    /// Reasons why this capability is compatible
    pub compatibility_reasons: Vec<String>,}


impl CapabilityRegistry {
    /// Create a new capability registry
    pub async fn new() -> BearDogResult<Self> {
        info!("📋 Initializing Universal Capability Registry");
        Ok(Self {
            capabilities: Arc::new(RwLock::new(HashMap::new())),
            capability_index: Arc::new(RwLock::new(HashMap::new())),
            compatibility_matrix: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        })
    }
    /// Create a placeholder capability registry for testing
    pub fn placeholder() -> Self {
        Self {
        }
    /// Register capabilities for an ecosystem component
    pub async fn register_capabilities(
        &self,
        ecosystem_id: &str,
        instance_id: &str,
        capabilities: Vec<Capability>,
    ) -> BearDogResult<()> {
        let provider_key = format!("{ecosystem_id}:{instance_id}");
        info!(
            "📋 Registering {} capabilities for {}",
            capabilities.len(),
            provider_key
        );
        // Store capabilities
        {
            let mut caps = self.capabilities.write().await;
            caps.insert(provider_key.clone(), capabilities.clone());
        // Update capability index
            let mut index = self.capability_index.write().await;
            for capability in &capabilities {
                index
                    .entry(capability.id.clone())
                    .or_insert_with(Vec::new)
                    .push(provider_key.clone());
            }
        // Update statistics
            let mut stats = self.stats.write().await;
            stats.total_capabilities += capabilities.len() as u32;
            stats.last_updated = chrono::Utc::now();
        debug!(
            "✅ Successfully registered capabilities for {}",
        Ok(())
    /// Unregister capabilities for an ecosystem component
    pub async fn unregister_capabilities(
        info!("📋 Unregistering capabilities for {}", provider_key);
        // Remove from capabilities
        let removed_capabilities = {
            caps.remove(&provider_key)
        };
        if let Some(capabilities) = removed_capabilities {
            // Update capability index
            {
                let mut index = self.capability_index.write().await;
                for capability in &capabilities {
                    if let Some(providers) = index.get_mut(&capability.id) {
                        providers.retain(|p| p != &provider_key);
                        if providers.is_empty() {
                            index.remove(&capability.id);
                        }
                    }
                }
            // Update statistics
                let mut stats = self.stats.write().await;
                stats.total_capabilities = stats
                    .total_capabilities
                    .saturating_sub(capabilities.len() as u32);
                stats.last_updated = chrono::Utc::now();
            "✅ Successfully unregistered capabilities for {}",
    /// Search for capabilities matching criteria
    pub async fn search_capabilities(
        criteria: CapabilitySearchCriteria,
    ) -> BearDogResult<Vec<CapabilityMatch>> {
        debug!("🔍 Searching capabilities with criteria: {:?}", criteria);
        let mut matches = Vec::new();
        // Get all capabilities
        let capabilities = self.capabilities.read().await;
        for (provider_key, provider_capabilities) in capabilities.iter() {
            let parts: Vec<&str> = provider_key.split(':').collect();
            if parts.len() != 2 {
                continue;
            let (ecosystem_id, instance_id) = (parts[0], parts[1]);
            for capability in provider_capabilities {
                if self.capability_matches_criteria(capability, &criteria) {
                    let match_score = self.calculate_match_score(capability, &criteria);
                    let compatibility_reasons =
                        self.get_compatibility_reasons(capability, &criteria);
                    matches.push(CapabilityMatch {
                        capability: capability.clone(),
                        provider_ecosystem: ecosystem_id.to_string(),
                        provider_instance: instance_id.to_string(),
                        match_score,
                        compatibility_reasons,
                    });
        // Sort by match score (highest first)
        matches.sort_by(|a, b| {
            b.match_score
                .partial_cmp(&a.match_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        // Update search statistics
            stats.total_searches += 1;
            if !matches.is_empty() {
                stats.successful_matches += 1;
        info!("✅ Found {} capability matches", matches.len());
        Ok(matches)
    /// Get all capabilities for a specific provider
    pub async fn get_provider_capabilities(
    ) -> BearDogResult<Vec<Capability>> {
        Ok(capabilities.get(&provider_key).cloned().unwrap_or_default())
    /// Get all capabilities by category}


    pub async fn get_capabilities_by_category(
        category: CapabilityCategory,
        let mut matching_capabilities = Vec::new();
        for provider_capabilities in capabilities.values() {
                if capability.category == category {
                    matching_capabilities.push(capability.clone());
        Ok(matching_capabilities)
    /// Get registry statistics
    pub async fn get_stats(&self) -> RegistryStats {
        self.stats.read().await.clone()
    /// Find compatible components for genetic spawning}


    pub async fn find_compatible_components(
        target_capability: &str,
        min_compatibility_score: f64,
            "🧬 Finding components compatible with capability: {}",
            target_capability
        let criteria = CapabilitySearchCriteria {
            category: None,
            min_qos_response_time_ms: None,
            min_availability_percent: Some(90.0), // Minimum 90% availability for genetic spawning
            required_attributes: HashMap::new(),
        let all_matches = self.search_capabilities(criteria).await?;
        // Filter by compatibility score
        let compatible_matches: Vec<CapabilityMatch> = all_matches
            .into_iter()
            .filter(|m| m.match_score >= min_compatibility_score)
            .collect();
            "✅ Found {} compatible components for genetic spawning",
            compatible_matches.len()
        Ok(compatible_matches)
    /// Check if a capability matches the search criteria
    fn capability_matches_criteria(
        capability: &Capability,
        criteria: &CapabilitySearchCriteria,
    ) -> bool {
        // Check category
        if let Some(ref required_category) = criteria.category {
            if capability.category != *required_category {
                return false;
        // Check QoS requirements
        if let Some(max_response_time) = criteria.min_qos_response_time_ms {
            if capability.qos.avg_response_time_ms > max_response_time {
        if let Some(min_availability) = criteria.min_availability_percent {
            if capability.qos.availability_percent < min_availability {
        // Check required attributes
        for (key, value) in &criteria.required_attributes {
            if let Some(attr) = capability.attributes.get(key) {
                if attr.value != *value {
                    return false;
            } else {
        true
    /// Calculate match score for a capability
    fn calculate_match_score(
        _criteria: &CapabilitySearchCriteria,
    ) -> f64 {
        let mut score = 0.0;
        // Base score from QoS metrics
        score += (100.0 - capability.qos.avg_response_time_ms as f64) / 100.0; // Lower response time = higher score
        score += capability.qos.availability_percent / 100.0; // Higher availability = higher score
        // Bonus for certain capabilities
        match capability.category {
            CapabilityCategory::Security => score += 2.0, // Security capabilities get bonus
            CapabilityCategory::Compute => score += 1.5,
            CapabilityCategory::Storage => score += 1.0,
            _ => {}
        score.clamp(0.0, 10.0) // Clamp between 0 and 10
    /// Get compatibility reasons for a capability match
    fn get_compatibility_reasons(
    ) -> Vec<String> {
        let mut reasons = Vec::new();
        if let Some(ref category) = criteria.category {
            if capability.category == *category {
                reasons.push(format!("Matches required category: {category:?}"));
            if capability.qos.availability_percent >= min_availability {
                reasons.push(format!(
                    "Meets availability requirement: {:.1}% >= {:.1}%",
                    capability.qos.availability_percent, min_availability
                ));
            if capability.qos.avg_response_time_ms <= max_response_time {
                    "Meets response time requirement: {}ms <= {}ms",
                    capability.qos.avg_response_time_ms, max_response_time
        if reasons.is_empty() {
            reasons.push("General compatibility".to_string());
        reasons
    /// Add compatibility rule to the matrix
    pub async fn add_compatibility_rule(&self, capability: String, compatible_with: Vec<String>) {
        let mut compatibility_matrix = self.compatibility_matrix.write().await;
        compatibility_matrix.insert(capability, compatible_with);
    /// Check if two capabilities are compatible using the compatibility matrix}


    pub async fn are_capabilities_compatible(
        capability_a: &str,
        capability_b: &str,
        let compatibility_matrix = self.compatibility_matrix.read().await;
        // Check bidirectional compatibility
        if let Some(compatible_capabilities) = compatibility_matrix.get(capability_a) {
            if compatible_capabilities.contains(&capability_b.to_string()) {
                return true;
        if let Some(compatible_capabilities) = compatibility_matrix.get(capability_b) {
            if compatible_capabilities.contains(&capability_a.to_string()) {
        false
    /// Initialize default compatibility rules
    pub async fn initialize_default_compatibility_rules(&self) {
        // Security capabilities are compatible with each other
        compatibility_matrix.insert(
            "security_auth".to_string(),
            vec![
                "security_encryption".to_string(),
                "security_audit".to_string(),
            ],
        // Storage capabilities compatibility
            "storage_file".to_string(),
            vec!["storage_database".to_string(), "storage_cache".to_string()],
        // Compute capabilities compatibility
            "compute_cpu".to_string(),
            vec!["compute_gpu".to_string(), "compute_memory".to_string()],
