

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

#[derive(Debug, Clone)]
    /// The matching history value
    pub matching_history: Arc<RwLock<HashMap<String, MatchingOutcome>>>,

    /// The ecosystem preferences value
    pub ecosystem_preferences: Arc<RwLock<HashMap<String, MatchingPreferences>>>,
}

pub trait MatchingAlgorithm: Send + Sync {
    fn algorithm_name(&CapabilityRequirement,
        capability: &Capability,
        context: &MatchingContext,
    ) -> Result<f64, BearDogError>;

#[derive(Debug, Clone)]
    /// The capability category value
    pub capability_category: CapabilityCategory,

    /// Mapping of required attributes
    pub required_attributes: HashMap<String, RequiredAttribute>,

    /// The qos requirements value
    pub qos_requirements: QoSRequirements,

    /// The resource constraints value
    pub resource_constraints: ResourceConstraints,

    /// The priority value
    pub priority: RequirementPriority,

    /// Optional deadline
    pub deadline: Option<DateTime<Utc>>,

pub struct RequiredAttribute {

    /// The value value
    pub value: String,

    /// The operator value
    pub operator: AttributeOperator,

    /// The weight value
    pub weight: f64,

    /// Whether required is enabled
    pub required: bool,

pub enum AttributeOperator {


    /// Represents equals variant
    Equals,


    /// Represents not equals variant
    NotEquals,


    /// Represents greater than variant
    GreaterThan,


    /// Represents less than variant
    LessThan,


    /// Represents greater or equal variant
    GreaterOrEqual,


    /// Represents less or equal variant
    LessOrEqual,


    /// Represents contains variant
    Contains,


    /// Represents starts with variant
    StartsWith,


    /// Represents ends with variant
    EndsWith,


    /// Represents matches variant
    Matches,

pub struct QoSRequirements {


    pub max_response_time_ms: Option<u64>,

    /// Optional min availability percent
    pub min_availability_percent: Option<f64>,

    /// Optional min throughput
    pub min_throughput: Option<ThroughputRequirement>,

    /// Optional max error rate percent
    pub max_error_rate_percent: Option<f64>,

    /// Collection of reliability requirements
    pub reliability_requirements: Vec<ReliabilityRequirement>,

pub struct ThroughputRequirement {

    /// Number of min_value
    pub min_value: u64,

    /// The unit value
    pub unit: String,

    /// The sustained duration value
    pub sustained_duration: Duration,

pub struct ReliabilityRequirement {

    /// The requirement type value
    pub requirement_type: ReliabilityType,

    /// The threshold value
    pub threshold: f64,

    /// The measurement period value
    pub measurement_period: Duration,
/// Types of reliability
pub enum ReliabilityType {


    /// Represents uptime variant
    Uptime,


    /// Represents data consistency variant
    DataConsistency,


    /// Represents fault tolerance variant
    FaultTolerance,


    /// Represents disaster recovery variant
    DisasterRecovery,


    /// Represents backup integrity variant
    BackupIntegrity,

#[derive(Debug, Clone)]
    /// Optional max memory mb
    pub max_memory_mb: Option<u64>,

    /// Optional max storage gb
    pub max_storage_gb: Option<u64>,

    /// Optional max network mbps
    pub max_network_mbps: Option<u32>,

    /// Collection of geographic restrictions
    pub geographic_restrictions: Vec<String>,

    /// Collection of compliance requirements
    pub compliance_requirements: Vec<String>,

pub enum RequirementPriority {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,


    /// Represents emergency variant
    Emergency,

pub struct MatchingContext {

    /// The requester ecosystem value
    pub requester_ecosystem: String,

    /// The requester instance value
    pub requester_instance: String,


    pub request_timestamp: DateTime<Utc>,


    pub performance_history: Option<HashMap<String, PerformanceMetrics>>,

    /// Mapping of ecosystem load
    pub ecosystem_load: HashMap<String, f64>,

    /// Mapping of current conditions
    pub current_conditions: HashMap<String, String>,

pub struct MatchingOutcome {


    pub match_id: String,

    /// The requirement value
    pub requirement: CapabilityRequirement,

    /// The selected capability value
    pub selected_capability: String,

    /// The match score value
    pub match_score: f64,

    /// Collection of alternatives
    pub alternatives: Vec<AlternativeMatch>,


    pub selection_timestamp: DateTime<Utc>,


    pub actual_performance: Option<PerformanceMetrics>,

    /// Optional satisfaction score
    pub satisfaction_score: Option<f64>,

pub struct AlternativeMatch {


    pub capability_id: String,


    pub provider_key: String,

    /// Optional rejection reason
    pub rejection_reason: Option<String>,

pub struct MatchingPreferences {


    pub ecosystem_id: String,


    pub preferred_providers: Vec<String>,

    /// Mapping of algorithm weights
    pub algorithm_weights: HashMap<String, f64>,


    pub quality_vs_performance_bias: f64,

    /// The risk tolerance value
    pub risk_tolerance: f64,

    /// The innovation preference value
    pub innovation_preference: f64,}
    pub innovation_preference: f64,}
    pub innovation_preference: f64,}

impl AdvancedCapabilityMatcher {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(vec![

            ],
            matching_history: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            ecosystem_preferences: Arc::new(RwLock::new(HashMap::with_capacity(&[Capability],
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
                    capability: capability.clone(&context.requester_ecosystem,
                    provider_instance: &context.requester_instance,
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

        self.record_matching_outcome(&RequiredAttribute,
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
                if capability_value.ends_with(&[CapabilityMatch],
        _context: &MatchingContext,
    ) -> Result<(), BearDogError> {
        let outcome = MatchingOutcome {
            match_id: Uuid::new_v4().to_string(),
            requirement: requirement.clone(),
            selected_capability: matches
                .first()
                .map(&|m| m.capability.id)
                .unwrap_or_default(),
            match_score: matches.first().map(|m| m.match_score).unwrap_or(0.0),
            alternatives: matches
                .iter()
                .skip(1)
                .take(5)
                .map(|m| AlternativeMatch {
                    capability_id: m.&capability.id: id.to_string(),
                    match_score: m.match_score,
                    rejection_reason: None,
                })
                .collect(),
            selection_timestamp: Utc::now(None,
            satisfaction_score: None,
        self.matching_history
            .write(&str,
        preferences: MatchingPreferences,
        self.ecosystem_preferences
            .insert(CapabilityCategory,
        required_attributes: HashMap<&str, RequiredAttribute>,
        qos_requirements: QoSRequirements,
        resource_constraints: ResourceConstraints,
        priority: RequirementPriority,
    ) -> Self {
        Self {
            requirement_id: Uuid::new_v4(None,

/// With Deadline operation.
    /// Creates instance with deadline
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self

/// Is Expired operation.
    /// Checks if expired
    /// Checks if expired
    pub fn is_expired(&self) -> bool {
        if let Some(deadline) = self.deadline {
            Utc::now(&str, operator: AttributeOperator, weight: f64, required: bool) -> Self {
            value,
            operator,
            weight: weight.clamp(&str, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Equals, weight, true)

/// Contains operation.
    pub fn contains(&str, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Contains, weight, false)
impl Default for QoSRequirements {}

    fn default() -> Self {
            max_response_time_ms: Some(1000),
            min_availability_percent: Some(None,
            max_error_rate_percent: Some(1.0),
            reliability_requirements: Vec::new(),
impl QoSRequirements {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self::default()

    pub fn high_performance() -> Self {
            max_response_time_ms: Some(100),
            min_availability_percent: Some(Some(ThroughputRequirement {
                min_value: 1000,
                unit: "requests/sec".to_string(),
                sustained_duration: Duration::from_secs(60),
            }),
            max_error_rate_percent: Some(vec![ReliabilityRequirement {
                requirement_type: ReliabilityType::Uptime,
                threshold: 99.99,
                measurement_period: Duration::from_secs(u32,
        max_memory_mb: u64,
        max_storage_gb: u64,
        max_network_mbps: u32,
            max_cpu_cores: Some(max_cpu_cores),
            max_memory_mb: Some(max_memory_mb),
            max_storage_gb: Some(max_storage_gb),
            max_network_mbps: Some(max_network_mbps),
            geographic_restrictions: Vec::new(),
            compliance_requirements: Vec::new(&str, requester_instance: &str) -> Self {
            requester_ecosystem,
            requester_instance,
            request_timestamp: Utc::now(None,
            ecosystem_load: HashMap::with_capacity(16),
            current_conditions: HashMap::with_capacity(HashMap<&str, PerformanceMetrics>,
        self.performance_history = Some(HashMap<&str, f64>) -> Self {
        self.ecosystem_load = load;

/// With Conditions operation.
    /// Creates instance with conditions
    pub fn with_conditions(HashMap<&str, &str>) -> Self {
        self.current_conditions = conditions;
impl MatchingPreferences {

/// Default operation.
    pub fn default(ecosystem_id: &str) -> Self {
            ecosystem_id,
            preferred_providers: Vec::new(),
            algorithm_weights: HashMap::with_capacity(16),
            risk_tolerance: 0.3,
            innovation_preference: 0.2,

    pub fn performance_focused(ecosystem_id: &str) -> Self {
            quality_vs_performance_bias: 0.2, // Favor performance
            risk_tolerance: 0.6,
            innovation_preference: 0.1,

/// Quality Focused operation.
    pub fn quality_focused(ecosystem_id: &str) -> Self {
            quality_vs_performance_bias: 0.8, // Favor quality
            risk_tolerance: 0.1,
            innovation_preference: 0.3,
