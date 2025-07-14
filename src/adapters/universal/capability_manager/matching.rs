//! Advanced capability matching algorithms and requirements
//!
//! This module provides sophisticated capability matching capabilities,
//! including multiple matching algorithms, QoS requirements, resource
//! constraints, and matching context management.

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::super::registry::CapabilityMatch;
use super::super::traits::*;
use super::monitoring::PerformanceMetrics;
use crate::BearDogResult;

/// Advanced capability matcher with pluggable algorithms
pub struct AdvancedCapabilityMatcher {
    /// Matching algorithms
    pub matching_algorithms: Vec<Box<dyn MatchingAlgorithm + Send + Sync>>,

    /// Historical matching data for learning
    pub matching_history: Arc<RwLock<HashMap<String, MatchingOutcome>>>,

    /// Matching preferences by ecosystem
    pub ecosystem_preferences: Arc<RwLock<HashMap<String, MatchingPreferences>>>,
}

/// Algorithm interface for capability matching
pub trait MatchingAlgorithm: Send + Sync {
    fn algorithm_name(&self) -> &str;
    fn calculate_match_score(
        &self,
        requirement: &CapabilityRequirement,
        capability: &Capability,
        context: &MatchingContext,
    ) -> BearDogResult<f64>;
}

/// Comprehensive capability requirement specification
#[derive(Debug, Clone)]
pub struct CapabilityRequirement {
    /// Unique requirement identifier
    pub requirement_id: String,
    /// Category of capability being requested
    pub capability_category: CapabilityCategory,
    /// Required attributes and their constraints
    pub required_attributes: HashMap<String, RequiredAttribute>,
    /// Quality of service requirements
    pub qos_requirements: QoSRequirements,
    /// Resource constraints
    pub resource_constraints: ResourceConstraints,
    /// Priority level of this requirement
    pub priority: RequirementPriority,
    /// Deadline for capability provision
    pub deadline: Option<DateTime<Utc>>,
}

/// Required attribute specification
#[derive(Debug, Clone)]
pub struct RequiredAttribute {
    /// Expected value for the attribute
    pub value: String,
    /// Comparison operator
    pub operator: AttributeOperator,
    /// Weight of this attribute in matching (0.0 to 1.0)
    pub weight: f64,
    /// Whether this attribute is required or optional
    pub required: bool,
}

/// Attribute comparison operators
#[derive(Debug, Clone)]
pub enum AttributeOperator {
    /// Exact string equality
    Equals,
    /// String inequality
    NotEquals,
    /// Numeric greater than comparison
    GreaterThan,
    /// Numeric less than comparison
    LessThan,
    /// Numeric greater than or equal comparison
    GreaterOrEqual,
    /// Numeric less than or equal comparison
    LessOrEqual,
    /// String contains substring
    Contains,
    /// String starts with prefix
    StartsWith,
    /// String ends with suffix
    EndsWith,
    /// Regular expression pattern matching
    Matches,
}

/// Quality of service requirements
#[derive(Debug, Clone)]
pub struct QoSRequirements {
    /// Maximum acceptable response time
    pub max_response_time_ms: Option<u64>,
    /// Minimum availability percentage
    pub min_availability_percent: Option<f64>,
    /// Minimum throughput requirements
    pub min_throughput: Option<ThroughputRequirement>,
    /// Maximum acceptable error rate
    pub max_error_rate_percent: Option<f64>,
    /// Reliability requirements
    pub reliability_requirements: Vec<ReliabilityRequirement>,
}

/// Throughput requirement specification
#[derive(Debug, Clone)]
pub struct ThroughputRequirement {
    /// Minimum throughput value
    pub min_value: u64,
    /// Unit of measurement (e.g., "requests/sec", "MB/sec")
    pub unit: String,
    /// Duration over which throughput must be sustained
    pub sustained_duration: Duration,
}

/// Reliability requirement specification
#[derive(Debug, Clone)]
pub struct ReliabilityRequirement {
    /// Type of reliability being measured
    pub requirement_type: ReliabilityType,
    /// Threshold value that must be met
    pub threshold: f64,
    /// Period over which reliability is measured
    pub measurement_period: Duration,
}

/// Types of reliability measurements
#[derive(Debug, Clone)]
pub enum ReliabilityType {
    /// System uptime requirements
    Uptime,
    /// Data consistency requirements
    DataConsistency,
    /// Fault tolerance requirements
    FaultTolerance,
    /// Disaster recovery requirements
    DisasterRecovery,
    /// Backup integrity requirements
    BackupIntegrity,
}

/// Resource constraints for capability provision
#[derive(Debug, Clone)]
pub struct ResourceConstraints {
    /// Maximum CPU cores allowed
    pub max_cpu_cores: Option<u32>,
    /// Maximum memory in megabytes
    pub max_memory_mb: Option<u64>,
    /// Maximum storage in gigabytes
    pub max_storage_gb: Option<u64>,
    /// Maximum network bandwidth in megabits per second
    pub max_network_mbps: Option<u32>,
    /// Geographic restrictions
    pub geographic_restrictions: Vec<String>,
    /// Compliance requirements
    pub compliance_requirements: Vec<String>,
}

/// Priority levels for requirements
#[derive(Debug, Clone)]
pub enum RequirementPriority {
    /// Low priority - can be delayed
    Low,
    /// Medium priority - normal processing
    Medium,
    /// High priority - expedited processing
    High,
    /// Critical priority - immediate attention required
    Critical,
    /// Emergency priority - highest priority processing
    Emergency,
}

/// Context for capability matching
#[derive(Debug, Clone)]
pub struct MatchingContext {
    /// Requesting ecosystem identifier
    pub requester_ecosystem: String,
    /// Requesting instance identifier
    pub requester_instance: String,
    /// Timestamp of the request
    pub request_timestamp: DateTime<Utc>,
    /// Historical performance data
    pub performance_history: Option<HashMap<String, PerformanceMetrics>>,
    /// Current ecosystem load metrics
    pub ecosystem_load: HashMap<String, f64>,
    /// Current environmental conditions
    pub current_conditions: HashMap<String, String>,
}

/// Outcome of a matching operation
#[derive(Debug, Clone)]
pub struct MatchingOutcome {
    /// Unique identifier for this match
    pub match_id: String,
    /// Original requirement that was matched
    pub requirement: CapabilityRequirement,
    /// ID of the selected capability
    pub selected_capability: String,
    /// Match score of the selected capability
    pub match_score: f64,
    /// Alternative matches that were considered
    pub alternatives: Vec<AlternativeMatch>,
    /// When the selection was made
    pub selection_timestamp: DateTime<Utc>,
    /// Actual performance after selection (if available)
    pub actual_performance: Option<PerformanceMetrics>,
    /// Satisfaction score (0.0 to 1.0)
    pub satisfaction_score: Option<f64>,
}

/// Alternative match that was considered but not selected
#[derive(Debug, Clone)]
pub struct AlternativeMatch {
    /// Capability ID
    pub capability_id: String,
    /// Provider key
    pub provider_key: String,
    /// Match score
    pub match_score: f64,
    /// Reason for rejection
    pub rejection_reason: Option<String>,
}

/// Matching preferences for an ecosystem
#[derive(Debug, Clone)]
pub struct MatchingPreferences {
    /// Ecosystem identifier
    pub ecosystem_id: String,
    /// Preferred provider ecosystems
    pub preferred_providers: Vec<String>,
    /// Weights for different matching algorithms
    pub algorithm_weights: HashMap<String, f64>,
    /// Bias between quality and performance (0.0 = performance, 1.0 = quality)
    pub quality_vs_performance_bias: f64,
    /// Risk tolerance level (0.0 = risk-averse, 1.0 = risk-accepting)
    pub risk_tolerance: f64,
    /// Preference for innovative solutions (0.0 = conservative, 1.0 = innovative)
    pub innovation_preference: f64,
}

impl AdvancedCapabilityMatcher {
    /// Create a new advanced capability matcher
    pub async fn new() -> BearDogResult<Self> {
        Ok(Self {
            matching_algorithms: vec![
                // Box::new(SemanticMatchingAlgorithm::new()),
                // Box::new(PerformanceBasedMatcher::new()),
                // Box::new(MLDrivenMatcher::new()),
                // More algorithms would be added here
            ],
            matching_history: Arc::new(RwLock::new(HashMap::new())),
            ecosystem_preferences: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Find the best capability matches for a requirement
    pub async fn find_matches(
        &self,
        requirement: &CapabilityRequirement,
        available_capabilities: &[Capability],
        context: &MatchingContext,
    ) -> BearDogResult<Vec<CapabilityMatch>> {
        info!("🔍 Finding matches for requirement: {}", requirement.requirement_id);

        let mut matches = Vec::new();

        // Evaluate each capability against the requirement
        for capability in available_capabilities {
            let mut total_score = 0.0;
            let mut algorithm_scores = Vec::new();

            // Run each matching algorithm
            for algorithm in &self.matching_algorithms {
                let score = algorithm.calculate_match_score(requirement, capability, context)?;
                algorithm_scores.push((algorithm.algorithm_name().to_string(), score));
                total_score += score;
            }

            // Calculate average score
            let average_score = if self.matching_algorithms.is_empty() {
                self.calculate_basic_match_score(requirement, capability)
            } else {
                total_score / self.matching_algorithms.len() as f64
            };

            // Apply minimum threshold
            if average_score > 0.3 {
                matches.push(CapabilityMatch {
                    capability: capability.clone(),
                    provider_ecosystem: context.requester_ecosystem.clone(),
                    provider_instance: context.requester_instance.clone(),
                    match_score: average_score,
                    compatibility_reasons: algorithm_scores
                        .iter()
                        .map(|(alg, score)| format!("{}: {:.2}", alg, score))
                        .collect(),
                });
            }
        }

        // Sort matches by score
        matches.sort_by(|a, b| {
            b.match_score
                .partial_cmp(&a.match_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Record matching outcome
        self.record_matching_outcome(requirement, &matches, context).await?;

        info!("✅ Found {} capability matches", matches.len());
        Ok(matches)
    }

    /// Calculate basic match score when no algorithms are available
    fn calculate_basic_match_score(
        &self,
        requirement: &CapabilityRequirement,
        capability: &Capability,
    ) -> f64 {
        // Basic category matching
        let category_match = if capability.category == requirement.capability_category {
            1.0
        } else {
            0.0
        };

        // Basic attribute matching
        let attribute_score = self.calculate_attribute_match_score(requirement, capability);

        // Simple weighted average
        (category_match * 0.4) + (attribute_score * 0.6)
    }

    /// Calculate attribute match score
    fn calculate_attribute_match_score(
        &self,
        requirement: &CapabilityRequirement,
        capability: &Capability,
    ) -> f64 {
        if requirement.required_attributes.is_empty() {
            return 1.0;
        }

        let mut total_score = 0.0;
        let mut total_weight = 0.0;

        for (attr_name, required_attr) in &requirement.required_attributes {
            if let Some(capability_attribute) = capability.attributes.get(attr_name) {
                let match_score = self.evaluate_attribute_match(required_attr, &capability_attribute.value);
                total_score += match_score * required_attr.weight;
                total_weight += required_attr.weight;
            } else if required_attr.required {
                // Required attribute is missing
                return 0.0;
            }
        }

        if total_weight > 0.0 {
            total_score / total_weight
        } else {
            1.0
        }
    }

    /// Evaluate attribute match based on operator
    fn evaluate_attribute_match(&self, required_attr: &RequiredAttribute, capability_value: &str) -> f64 {
        match required_attr.operator {
            AttributeOperator::Equals => {
                if capability_value == required_attr.value {
                    1.0
                } else {
                    0.0
                }
            },
            AttributeOperator::NotEquals => {
                if capability_value != required_attr.value {
                    1.0
                } else {
                    0.0
                }
            },
            AttributeOperator::Contains => {
                if capability_value.contains(&required_attr.value) {
                    1.0
                } else {
                    0.0
                }
            },
            AttributeOperator::StartsWith => {
                if capability_value.starts_with(&required_attr.value) {
                    1.0
                } else {
                    0.0
                }
            },
            AttributeOperator::EndsWith => {
                if capability_value.ends_with(&required_attr.value) {
                    1.0
                } else {
                    0.0
                }
            },
            _ => {
                // For numeric comparisons, we'd need to parse the values
                // For now, return a default match
                0.5
            }
        }
    }

    /// Record matching outcome for learning
    async fn record_matching_outcome(
        &self,
        requirement: &CapabilityRequirement,
        matches: &[CapabilityMatch],
        context: &MatchingContext,
    ) -> BearDogResult<()> {
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
                    provider_key: format!("{}:{}", m.provider_ecosystem, m.provider_instance),
                    match_score: m.match_score,
                    rejection_reason: None,
                })
                .collect(),
            selection_timestamp: Utc::now(),
            actual_performance: None,
            satisfaction_score: None,
        };

        self.matching_history
            .write()
            .await
            .insert(outcome.match_id.clone(), outcome);

        Ok(())
    }

    /// Get matching history
    pub async fn get_matching_history(&self) -> BearDogResult<Vec<MatchingOutcome>> {
        let history = self.matching_history.read().await;
        Ok(history.values().cloned().collect())
    }

    /// Update ecosystem preferences
    pub async fn update_preferences(
        &self,
        ecosystem_id: &str,
        preferences: MatchingPreferences,
    ) -> BearDogResult<()> {
        self.ecosystem_preferences
            .write()
            .await
            .insert(ecosystem_id.to_string(), preferences);
        Ok(())
    }

    /// Get ecosystem preferences
    pub async fn get_preferences(&self, ecosystem_id: &str) -> BearDogResult<Option<MatchingPreferences>> {
        let preferences = self.ecosystem_preferences.read().await;
        Ok(preferences.get(ecosystem_id).cloned())
    }
}

// Implementation helpers for various types
impl CapabilityRequirement {
    /// Create a new capability requirement
    pub fn new(
        capability_category: CapabilityCategory,
        required_attributes: HashMap<String, RequiredAttribute>,
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
        }
    }

    /// Set a deadline for this requirement
    pub fn with_deadline(mut self, deadline: DateTime<Utc>) -> Self {
        self.deadline = Some(deadline);
        self
    }

    /// Check if this requirement has expired
    pub fn is_expired(&self) -> bool {
        if let Some(deadline) = self.deadline {
            Utc::now() > deadline
        } else {
            false
        }
    }
}

impl RequiredAttribute {
    /// Create a new required attribute
    pub fn new(value: String, operator: AttributeOperator, weight: f64, required: bool) -> Self {
        Self {
            value,
            operator,
            weight: weight.max(0.0).min(1.0),
            required,
        }
    }

    /// Create an equals attribute
    pub fn equals(value: String, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Equals, weight, true)
    }

    /// Create a contains attribute
    pub fn contains(value: String, weight: f64) -> Self {
        Self::new(value, AttributeOperator::Contains, weight, false)
    }
}

impl QoSRequirements {
    /// Create default QoS requirements
    pub fn default() -> Self {
        Self {
            max_response_time_ms: Some(1000),
            min_availability_percent: Some(99.0),
            min_throughput: None,
            max_error_rate_percent: Some(1.0),
            reliability_requirements: Vec::new(),
        }
    }

    /// Create high-performance QoS requirements
    pub fn high_performance() -> Self {
        Self {
            max_response_time_ms: Some(100),
            min_availability_percent: Some(99.9),
            min_throughput: Some(ThroughputRequirement {
                min_value: 1000,
                unit: "requests/sec".to_string(),
                sustained_duration: Duration::from_secs(60),
            }),
            max_error_rate_percent: Some(0.1),
            reliability_requirements: vec![
                ReliabilityRequirement {
                    requirement_type: ReliabilityType::Uptime,
                    threshold: 99.99,
                    measurement_period: Duration::from_secs(3600),
                },
            ],
        }
    }
}

impl ResourceConstraints {
    /// Create default resource constraints
    pub fn default() -> Self {
        Self {
            max_cpu_cores: None,
            max_memory_mb: None,
            max_storage_gb: None,
            max_network_mbps: None,
            geographic_restrictions: Vec::new(),
            compliance_requirements: Vec::new(),
        }
    }

    /// Create strict resource constraints
    pub fn strict(
        max_cpu_cores: u32,
        max_memory_mb: u64,
        max_storage_gb: u64,
        max_network_mbps: u32,
    ) -> Self {
        Self {
            max_cpu_cores: Some(max_cpu_cores),
            max_memory_mb: Some(max_memory_mb),
            max_storage_gb: Some(max_storage_gb),
            max_network_mbps: Some(max_network_mbps),
            geographic_restrictions: Vec::new(),
            compliance_requirements: Vec::new(),
        }
    }
}

impl MatchingContext {
    /// Create a new matching context
    pub fn new(requester_ecosystem: String, requester_instance: String) -> Self {
        Self {
            requester_ecosystem,
            requester_instance,
            request_timestamp: Utc::now(),
            performance_history: None,
            ecosystem_load: HashMap::new(),
            current_conditions: HashMap::new(),
        }
    }

    /// Add performance history
    pub fn with_performance_history(mut self, history: HashMap<String, PerformanceMetrics>) -> Self {
        self.performance_history = Some(history);
        self
    }

    /// Add ecosystem load information
    pub fn with_ecosystem_load(mut self, load: HashMap<String, f64>) -> Self {
        self.ecosystem_load = load;
        self
    }

    /// Add current conditions
    pub fn with_conditions(mut self, conditions: HashMap<String, String>) -> Self {
        self.current_conditions = conditions;
        self
    }
}

impl MatchingPreferences {
    /// Create default matching preferences
    pub fn default(ecosystem_id: String) -> Self {
        Self {
            ecosystem_id,
            preferred_providers: Vec::new(),
            algorithm_weights: HashMap::new(),
            quality_vs_performance_bias: 0.5,
            risk_tolerance: 0.3,
            innovation_preference: 0.2,
        }
    }

    /// Create performance-focused preferences
    pub fn performance_focused(ecosystem_id: String) -> Self {
        Self {
            ecosystem_id,
            preferred_providers: Vec::new(),
            algorithm_weights: HashMap::new(),
            quality_vs_performance_bias: 0.2, // Favor performance
            risk_tolerance: 0.6,
            innovation_preference: 0.1,
        }
    }

    /// Create quality-focused preferences
    pub fn quality_focused(ecosystem_id: String) -> Self {
        Self {
            ecosystem_id,
            preferred_providers: Vec::new(),
            algorithm_weights: HashMap::new(),
            quality_vs_performance_bias: 0.8, // Favor quality
            risk_tolerance: 0.1,
            innovation_preference: 0.3,
        }
    }
} 