

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use super::traits::*;
use beardog_errors::BearDogResult;

pub struct CapabilityRegistry {

    capabilities: Arc<RwLock<HashMap<String, Vec<Capability>>>>,

    capability_index: Arc<RwLock<HashMap<String, Vec<String>>>>,

    compatibility_matrix: Arc<RwLock<HashMap<String, Vec<String>>>>,

    stats: Arc<RwLock<RegistryStats>>,
}

#[derive(Debug, Clone, Default)]
pub struct RegistryStats {

    pub total_capabilities: u32,

    pub total_providers: u32,

    pub total_searches: u64,

    pub successful_matches: u64,

    pub last_updated: chrono::DateTime<chrono::Utc>,

pub struct CapabilitySearchCriteria {

    pub category: Option<CapabilityCategory>,

    pub min_qos_response_time_ms: Option<u64>,

    pub min_availability_percent: Option<f64>,

    pub required_attributes: HashMap<String, String>,

#[derive(Debug, Clone)]
pub struct CapabilityMatch {

    pub capability: Capability,

    pub provider_ecosystem: String,

    pub provider_instance: String,

    pub match_score: f64,

    pub compatibility_reasons: Vec<String>,}

impl CapabilityRegistry {

    pub async fn new() -> BearDogResult<Self> {
        info!("📋 Initializing Universal Capability Registry");
        Ok(Self {
            capabilities: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            capability_index: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            compatibility_matrix: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            stats: Arc::new(RwLock::new(RegistryStats::default())),
        })
    }

    pub fn placeholder() -> Self {
        Self {
        }

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

        {
            let mut caps = self.capabilities.write().await;
            caps.insert(provider_key.clone(), capabilities.clone());

            let mut index = self.capability_index.write().await;
            for capability in &capabilities {
                index
                    .entry(capability.id.clone())
                    .or_insert_with(Vec::new)
                    .push(provider_key.clone());
            }

            let mut stats = self.stats.write().await;
            stats.total_capabilities += capabilities.len() as u32;
            stats.last_updated = chrono::Utc::now();
        debug!(
            "✅ Successfully registered capabilities for {}",
        Ok(())

    pub async fn unregister_capabilities(
        info!("📋 Unregistering capabilities for {}", provider_key);

        let removed_capabilities = {
            caps.remove(&provider_key)
        };
        if let Some(capabilities) = removed_capabilities {

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

                let mut stats = self.stats.write().await;
                stats.total_capabilities = stats
                    .total_capabilities
                    .saturating_sub(capabilities.len() as u32);
                stats.last_updated = chrono::Utc::now();
            "✅ Successfully unregistered capabilities for {}",

    pub async fn search_capabilities(
        criteria: CapabilitySearchCriteria,
    ) -> BearDogResult<Vec<CapabilityMatch>> {
        debug!("🔍 Searching capabilities with criteria: {:?}", criteria);
        let mut matches = Vec::new();

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

        matches.sort_by(|a, b| {
            b.match_score
                .partial_cmp(&a.match_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

            stats.total_searches += 1;
            if !matches.is_empty() {
                stats.successful_matches += 1;
        info!("✅ Found {} capability matches", matches.len());
        Ok(matches)

    pub async fn get_provider_capabilities(
    ) -> BearDogResult<Vec<Capability>> {
        Ok(capabilities.get(&provider_key).cloned().unwrap_or_default())

    pub async fn get_capabilities_by_category(
        category: CapabilityCategory,
        let mut matching_capabilities = Vec::new();
        for provider_capabilities in capabilities.values() {
                if capability.category == category {
                    matching_capabilities.push(capability.clone());
        Ok(matching_capabilities)

    pub async fn get_stats(&self) -> RegistryStats {
        self.stats.read().await.clone()

    pub async fn find_compatible_components(
        target_capability: &str,
        min_compatibility_score: f64,
            "🧬 Finding components compatible with capability: {}",
            target_capability
        let criteria = CapabilitySearchCriteria {
            category: None,
            min_qos_response_time_ms: None,
            min_availability_percent: Some(90.0), // Minimum 90% availability for genetic spawning
            required_attributes: HashMap::with_capacity(16),
        let all_matches = self.search_capabilities(criteria).await?;

        let compatible_matches: Vec<CapabilityMatch> = all_matches
            .into_iter()
            .filter(|m| m.match_score >= min_compatibility_score)
            .collect();
            "✅ Found {} compatible components for genetic spawning",
            compatible_matches.len()
        Ok(compatible_matches)

    fn capability_matches_criteria(
        capability: &Capability,
        criteria: &CapabilitySearchCriteria,
    ) -> bool {

        if let Some(ref required_category) = criteria.category {
            if capability.category != *required_category {
                return false;

        if let Some(max_response_time) = criteria.min_qos_response_time_ms {
            if capability.qos.avg_response_time_ms > max_response_time {
        if let Some(min_availability) = criteria.min_availability_percent {
            if capability.qos.availability_percent < min_availability {

        for (key, value) in &criteria.required_attributes {
            if let Some(attr) = capability.attributes.get(key) {
                if attr.value != *value {
                    return false;
            } else {
        true

    fn calculate_match_score(
        _criteria: &CapabilitySearchCriteria,
    ) -> f64 {
        let mut score = 0.0;

        score += (100.0 - capability.qos.avg_response_time_ms as f64) / 100.0; // Lower response time = higher score
        score += capability.qos.availability_percent / 100.0; // Higher availability = higher score

        match capability.category {
            CapabilityCategory::Security => score += 2.0, // Security capabilities get bonus
            CapabilityCategory::Compute => score += 1.5,
            CapabilityCategory::Storage => score += 1.0,
            _ => {}
        score.clamp(0.0, 10.0) // Clamp between 0 and 10

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

    pub async fn add_compatibility_rule(&self, capability: &str, compatible_with: Vec<&str>) {
        let mut compatibility_matrix = self.compatibility_matrix.write().await;
        compatibility_matrix.insert(capability, compatible_with);

    pub async fn are_capabilities_compatible(
        capability_a: &str,
        capability_b: &str,
        let compatibility_matrix = self.compatibility_matrix.read().await;

        if let Some(compatible_capabilities) = compatibility_matrix.get(capability_a) {
            if compatible_capabilities.contains(&capability_b.to_string()) {
                return true;
        if let Some(compatible_capabilities) = compatibility_matrix.get(capability_b) {
            if compatible_capabilities.contains(&capability_a.to_string()) {
        false

    pub async fn initialize_default_compatibility_rules(&self) {

        compatibility_matrix.insert(
            "security_auth".to_string(),
            vec![
                "security_encryption".to_string(),
                "security_audit".to_string(),
            ],

            "storage_file".to_string(),
            vec!["storage_database".to_string(), "storage_cache".to_string()],

            "compute_cpu".to_string(),
            vec!["compute_gpu".to_string(), "compute_memory".to_string()],
