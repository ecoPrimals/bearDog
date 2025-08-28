

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;
use super::super::registry::CapabilityMatch;
use super::super::traits::*;
use super::monitoring::PerformanceMetrics;
use beardog_errors::BearDogError;

#[derive(Debug)]
pub enum MatchingAlgorithmType {
    Semantic(SemanticMatchingAlgorithm),
    Fuzzy(FuzzyMatchingAlgorithm),
    Exact(ExactMatchingAlgorithm),
    Weighted(WeightedMatchingAlgorithm),
}

pub struct AdvancedCapabilityMatcher {

    pub matching_algorithms: Vec<MatchingAlgorithmType>,

    pub matching_history: Arc<RwLock<HashMap<String, MatchingOutcome>>>,

    pub ecosystem_preferences: Arc<RwLock<HashMap<String, MatchingPreferences>>>,
}

pub trait MatchingAlgorithm: Send + Sync {
    fn algorithm_name(&self) -> &str;
    fn calculate_match_score(
        &self,
        requirement: &CapabilityRequirement,
        capability: &Capability,
        context: &MatchingContext,
    ) -> Result<f64, BearDogError>;

#[derive(Debug, Clone)]
pub struct CapabilityRequirement {

    pub requirement_id: String,

    pub capability_category: CapabilityCategory,

    pub required_attributes: HashMap<String, RequiredAttribute>,

    pub qos_requirements: QoSRequirements,

    pub resource_constraints: ResourceConstraints,

    pub priority: RequirementPriority,

    pub deadline: Option<DateTime<Utc>>,

pub struct RequiredAttribute {

    pub value: String,

    pub operator: AttributeOperator,

    pub weight: f64,

    pub required: bool,

pub enum AttributeOperator {

    Equals,

    NotEquals,

    GreaterThan,

    LessThan,

    GreaterOrEqual,

    LessOrEqual,

    Contains,

    StartsWith,

    EndsWith,

    Matches,

pub struct QoSRequirements {

    pub max_response_time_ms: Option<u64>,

    pub min_availability_percent: Option<f64>,

    pub min_throughput: Option<ThroughputRequirement>,

    pub max_error_rate_percent: Option<f64>,

    pub reliability_requirements: Vec<ReliabilityRequirement>,

pub struct ThroughputRequirement {

    pub min_value: u64,

    pub unit: String,

    pub sustained_duration: Duration,

pub struct ReliabilityRequirement {

    pub requirement_type: ReliabilityType,

    pub threshold: f64,

    pub measurement_period: Duration,

pub enum ReliabilityType {

    Uptime,

    DataConsistency,

    FaultTolerance,

    DisasterRecovery,

    BackupIntegrity,

#[derive(Debug, Clone, Default)]}

pub struct ResourceConstraints {

    pub max_cpu_cores: Option<u32>,

    pub max_memory_mb: Option<u64>,

    pub max_storage_gb: Option<u64>,

    pub max_network_mbps: Option<u32>,

    pub geographic_restrictions: Vec<String>,

    pub compliance_requirements: Vec<String>,

pub enum RequirementPriority {

    Low,

    Medium,

    High,

    Critical,

    Emergency,

pub struct MatchingContext {

    pub requester_ecosystem: String,

    pub requester_instance: String,

    pub request_timestamp: DateTime<Utc>,

    pub performance_history: Option<HashMap<String, PerformanceMetrics>>,

    pub ecosystem_load: HashMap<String, f64>,

    pub current_conditions: HashMap<String, String>,

pub struct MatchingOutcome {

    pub match_id: String,

    pub requirement: CapabilityRequirement,

    pub selected_capability: String,

    pub match_score: f64,

    pub alternatives: Vec<AlternativeMatch>,

    pub selection_timestamp: DateTime<Utc>,

    pub actual_performance: Option<PerformanceMetrics>,

    pub satisfaction_score: Option<f64>,

pub struct AlternativeMatch {

    pub capability_id: String,

    pub provider_key: String,

    pub rejection_reason: Option<String>,

pub struct MatchingPreferences {

    pub ecosystem_id: String,

    pub preferred_providers: Vec<String>,

    pub algorithm_weights: HashMap<String, f64>,

    pub quality_vs_performance_bias: f64,

    pub risk_tolerance: f64,

    pub innovation_preference: f64,}

impl AdvancedCapabilityMatcher {

    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            matching_algorithms: vec![

            ],
            matching_history: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            ecosystem_preferences: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

    pub async fn find_matches(
        available_capabilities: &[Capability],
    ) -> Result<Vec<CapabilityMatch>, BearDogError>> {
        info!(
            "🔍 Finding matches for requirement: {}",
            requirement.requirement_id
        );
        let mut matches = Vec::new();

        for capability in available_capabilities {
            let mut total_score = 0.0;
            let mut algorithm_scores = Vec::new();

            for algorithm in &self.matching_algorithms {
                let score = algorithm.calculate_match_score(requirement, capability, context)?;
                algorithm_scores.push((algorithm.algorithm_name().to_string(), score));
                total_score += score;
            }

            let average_score = if self.matching_algorithms.is_empty() {
                self.calculate_basic_match_score(requirement, capability)
            } else {
                total_score / self.matching_algorithms.len() as f64
            };

            if average_score > 0.3 {
                matches.push(CapabilityMatch {
                    capability: capability.clone(),
                    provider_ecosystem: context.requester_ecosystem.clone(),
                    provider_instance: context.requester_instance.clone(),
                    match_score: average_score,
                    compatibility_reasons: algorithm_scores
                        .iter()
                        .map(|(alg, score)| format!("{alg}: {score:.2}"))
                        .collect(),
                });
        }

        matches.sort_by(|a, b| {
            b.match_score
                .partial_cmp(&a.match_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self.record_matching_outcome(requirement, &matches, context)
            .await?;
        info!("✅ Found {} capability matches", matches.len());
        Ok(matches)

    fn calculate_basic_match_score(
    ) -> f64 {

        let category_match = if capability.category == requirement.capability_category {
            1.0
        } else {
            0.0
        };

        let attribute_score = self.calculate_attribute_match_score(requirement, capability);

        (category_match * 0.4) + (attribute_score * 0.6)

    fn calculate_attribute_match_score(
        if requirement.required_attributes.is_empty() {
            return 1.0;
        let mut total_score = 0.0;
        let mut total_weight = 0.0;
        for (attr_name, required_attr) in &requirement.required_attributes {
            if let Some(capability_attribute) = capability.attributes.get(attr_name) {
                let match_score =
                    self.evaluate_attribute_match(required_attr, &capability_attribute.value);
                total_score += match_score * required_attr.weight;
                total_weight += required_attr.weight;
            } else if required_attr.required {

                return 0.0;
        if total_weight > 0.0 {
            total_score / total_weight

    fn evaluate_attribute_match(
        required_attr: &RequiredAttribute,
        capability_value: &str,
        match required_attr.operator {
            AttributeOperator::Equals => {
                if capability_value == required_attr.value {
                    1.0
                } else {
                    0.0
                }
            AttributeOperator::NotEquals => {
                if capability_value != required_attr.value {
            AttributeOperator::Contains => {
                if capability_value.contains(&required_attr.value) {
            AttributeOperator::StartsWith => {
                if capability_value.starts_with(&required_attr.value) {
            AttributeOperator::EndsWith => {
                if capability_value.ends_with(&required_attr.value) {
            _ => {

                0.5

    async fn record_matching_outcome(
        matches: &[CapabilityMatch],
        _context: &MatchingContext,
    ) -> Result<(), BearDogError> {
        let outcome = MatchingOutcome {
            match_id: Uuid::new_v4().to_string(),
            requirement: requirement.clone(),
            selected_capability: matches
                .first()
                .map(|m| m.capability.id.clone())
                .unwrap_or_default(),
            match_score: matches.first().map(|m| m.match_score).unwrap_or(0.0),
            alternatives: matches
                .iter()
                .skip(1)
                .take(5)
                .map(|m| AlternativeMatch {
                    capability_id: m.capability.id.clone(),
                    provider_key: format_args!("{}:{}", m.provider_ecosystem, m.provider_instance).to_string(),
                    match_score: m.match_score,
                    rejection_reason: None,
                })
                .collect(),
            selection_timestamp: Utc::now(),
            actual_performance: None,
            satisfaction_score: None,
        self.matching_history
            .write()
            .await
            .insert(outcome.match_id.clone(), outcome);
        Ok(())

    pub async fn get_matching_history(&self) -> Result<Vec<MatchingOutcome>, BearDogError>> {
        let history = self.matching_history.read().await;
        Ok(history.values().cloned().collect())

    pub async fn update_preferences(
        ecosystem_id: &str,
        preferences: MatchingPreferences,
        self.ecosystem_preferences
            .insert(ecosystem_id.to_string(), preferences);

    pub async fn get_preferences(
    ) -> Result<Option<MatchingPreferences>, BearDogError>> {
        let preferences = self.ecosystem_preferences.read().await;
        Ok(preferences.get(ecosystem_id).cloned())

impl CapabilityRequirement {

    pub fn new(
        capability_category: CapabilityCategory,
        required_attributes: HashMap<&str, RequiredAttribute>,
        qos_requirements: QoSRequirements,
        resource_constraints: ResourceConstraints,
        priority: RequirementPriority,
    ) -> Self {
        Self {
            requirement_id: Uuid::new_v4().to_string(),
            capability_category,
            required_attributes,
            qos_requirements,
            resource_constraints,
            priority,
            deadline: None,

    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self

    pub fn is_expired(&self) -> bool {
        if let Some(deadline) = self.deadline {
            Utc::now() > deadline
            false
impl RequiredAttribute {

    pub fn new(value: &str, operator: AttributeOperator, weight: f64, required: bool) -> Self {
            value,
            operator,
            weight: weight.clamp(0.0, 1.0),
            required,

    pub fn equals(value: &str, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Equals, weight, true)

    pub fn contains(value: &str, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Contains, weight, false)
impl Default for QoSRequirements {}

    fn default() -> Self {
            max_response_time_ms: Some(1000),
            min_availability_percent: Some(99.0),
            min_throughput: None,
            max_error_rate_percent: Some(1.0),
            reliability_requirements: Vec::new(),
impl QoSRequirements {

    pub fn new() -> Self {
        Self::default()

    pub fn high_performance() -> Self {
            max_response_time_ms: Some(100),
            min_availability_percent: Some(99.9),
            min_throughput: Some(ThroughputRequirement {
                min_value: 1000,
                unit: "requests/sec".to_string(),
                sustained_duration: Duration::from_secs(60),
            }),
            max_error_rate_percent: Some(0.1),
            reliability_requirements: vec![ReliabilityRequirement {
                requirement_type: ReliabilityType::Uptime,
                threshold: 99.99,
                measurement_period: Duration::from_secs(3600),
            }],
impl ResourceConstraints {

    pub fn strict(
        max_cpu_cores: u32,
        max_memory_mb: u64,
        max_storage_gb: u64,
        max_network_mbps: u32,
            max_cpu_cores: Some(max_cpu_cores),
            max_memory_mb: Some(max_memory_mb),
            max_storage_gb: Some(max_storage_gb),
            max_network_mbps: Some(max_network_mbps),
            geographic_restrictions: Vec::new(),
            compliance_requirements: Vec::new(),
impl MatchingContext {

    pub fn new(requester_ecosystem: &str, requester_instance: &str) -> Self {
            requester_ecosystem,
            requester_instance,
            request_timestamp: Utc::now(),
            performance_history: None,
            ecosystem_load: HashMap::with_capacity(16),
            current_conditions: HashMap::with_capacity(16),

    pub fn with_performance_history(
        mut self,
        history: HashMap<&str, PerformanceMetrics>,
        self.performance_history = Some(history);

    pub fn with_ecosystem_load(mut self, load: HashMap<&str, f64>) -> Self {
        self.ecosystem_load = load;

    pub fn with_conditions(mut self, conditions: HashMap<&str, &str>) -> Self {
        self.current_conditions = conditions;
impl MatchingPreferences {

    pub fn default(ecosystem_id: &str) -> Self {
            ecosystem_id,
            preferred_providers: Vec::new(),
            algorithm_weights: HashMap::with_capacity(16),
            quality_vs_performance_bias: 0.5,
            risk_tolerance: 0.3,
            innovation_preference: 0.2,

    pub fn performance_focused(ecosystem_id: &str) -> Self {
            quality_vs_performance_bias: 0.2, // Favor performance
            risk_tolerance: 0.6,
            innovation_preference: 0.1,

    pub fn quality_focused(ecosystem_id: &str) -> Self {
            quality_vs_performance_bias: 0.8, // Favor quality
            risk_tolerance: 0.1,
            innovation_preference: 0.3,
