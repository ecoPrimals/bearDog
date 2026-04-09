// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Context-Aware Licensing Module
//!
//! Provides intelligent licensing and usage classification based on deployment
//! context, enabling fair pricing that adapts to actual usage patterns.
//!
//! ## Overview
//!
//! This module implements context-aware licensing that:
//! - Detects enterprise vs. small-scale usage automatically
//! - Adjusts pricing based on actual usage patterns
//! - Collects evidence for usage classification
//! - Provides fair, transparent pricing models
//!
//! ## Key Components
//!
//! - `ContextAwareLicense` - License configuration
//! - `EnterpriseIndicator` - Metrics for detecting enterprise usage
//! - `EnterprisePricingModel` - Pricing calculation
//! - `ClassificationEvidence` - Usage pattern evidence
//! - `FunctionUsagePattern` - Function-level usage tracking
//! - `HardwareProfile` - Hardware resource tracking
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_core::context_aware_licensing::{EnterpriseIndicator, EnterprisePricingModel};
//!
//! # fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Detect enterprise usage
//! let indicator = EnterpriseIndicator {
//!     organization_size: 500,
//!     usage_volume: 1_000_000,
//!     integration_complexity: 0.8,
//! };
//!
//! // Calculate appropriate pricing
//! let pricing = EnterprisePricingModel::calculate(&indicator)?;
//! # Ok(())
//! # }
//! ```

use serde::{Deserialize, Serialize};

/// Evidence used for context classification and pricing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationEvidence {
    /// Type of evidence collected (e.g., "`usage_pattern`", "`deployment_scale`")
    pub evidence_type: String,
    /// Confidence level of the evidence (0.0 to 1.0)
    pub confidence: f64,
    /// Source that provided this evidence
    /// The source value
    pub source: String,
}

/// Context-aware license configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAwareLicense {
    /// Type of license (e.g., "MIT", "Commercial", "Enterprise")
    /// The license type value
    pub license_type: String,
    /// License terms and conditions
    /// Collection of terms
    pub terms: Vec<String>,
    /// Collection of context requirements
    pub context_requirements: Vec<String>,
}

/// Enterprise usage indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseIndicator {
    /// Size of the organization (number of employees/users)
    /// Number of `organization_size`
    pub organization_size: usize,
    /// Volume of usage (requests, operations, etc.)
    /// Number of `usage_volume`
    pub usage_volume: u64,
    /// Complexity score of integrations (0.0 to 1.0)
    /// The integration complexity value
    pub integration_complexity: f64,
}

#[expect(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    dead_code,
    reason = "large integration test modules: noisy style lints and synthetic helpers"
)]
#[cfg(test)]
#[path = "context_aware_licensing_tests.rs"]
mod context_aware_licensing_tests;

/// Enterprise pricing model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterprisePricingModel {
    /// The base cost value
    pub base_cost: f64,
    /// Multiplier based on usage volume
    /// The volume multiplier value
    pub volume_multiplier: f64,
    /// Support tier level (e.g., "Basic", "Premium", "Enterprise")
    /// The support tier value
    pub support_tier: String,
}

/// Function usage patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionUsagePattern {
    /// Name of the function being tracked
    /// Name of the function
    pub function_name: String,
    /// Frequency of function usage
    /// Number of `usage_frequency`
    pub usage_frequency: u64,
    /// Peak usage count during high-load periods
    /// Number of `peak_usage`
    pub peak_usage: u64,
}

/// Hardware resource profile for usage classification
///
/// Tracks hardware resources to help determine deployment scale and
/// appropriate license tier (e.g., individual developer vs. data center).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Memory capacity in gigabytes
    pub memory_gb: u32,
    /// Storage capacity in gigabytes
    /// Number of `storage_gb`
    pub storage_gb: u64,
}

/// Individual user indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualIndicator {
    /// Type of user (e.g., "developer", "student", "hobbyist")
    /// The user type value
    pub user_type: String,
    /// Level of usage (e.g., "light", "moderate", "heavy")
    /// The usage level value
    pub usage_level: String,
    /// Whether `commercial_use` is enabled
    pub commercial_use: bool,
}

/// Integration complexity metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationComplexity {
    /// Number of API endpoints being used
    /// Number of `api_endpoints`
    pub api_endpoints: u32,
    /// Number of custom integrations implemented
    /// Number of `custom_integrations`
    pub custom_integrations: u32,
    /// Overall complexity score (0.0 to 1.0)
    /// The complexity score value
    pub complexity_score: f64,
}

/// Network profile metrics for deployment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProfile {
    /// Available bandwidth in megabits per second
    pub bandwidth_mbps: u32,
    /// Number of concurrent network connections
    /// Number of connection
    pub connection_count: u32,
    /// Geographic distribution of network nodes
    /// Collection of geographic distribution
    pub geographic_distribution: Vec<String>,
}

/// Organization size metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationSize {
    /// Total number of employees in the organization
    /// Number of employee
    pub employee_count: u32,
    /// Annual revenue in US dollars
    /// Number of `revenue_usd`
    pub revenue_usd: u64,
    /// Market segment classification (e.g., "SMB", "Enterprise", "Fortune500")
    /// The market segment value
    pub market_segment: String,
}

/// Primary use case classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryUseCase {
    /// Description of the primary use case
    /// The use case value
    pub use_case: String,
    /// Whether this use case is business-critical
    /// Whether `business_critical` is enabled
    pub business_critical: bool,
    /// Whether compliance requirements apply to this use case
    /// Whether `compliance_required` is enabled
    pub compliance_required: bool,
}

/// Purpose analysis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurposeAnalysis {
    /// Whether `commercial_purpose` is enabled
    pub commercial_purpose: bool,
    /// Whether `research_purpose` is enabled
    pub research_purpose: bool,
    /// Whether `educational_purpose` is enabled
    pub educational_purpose: bool,
}

/// Enterprise tax calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnterpriseTax {
    /// The tax rate value
    pub tax_rate: f64,
    /// Calculated tax amount in currency units
    /// The tax amount value
    pub tax_amount: f64,
    /// Tax jurisdiction (country, state, or region)
    /// The jurisdiction value
    pub jurisdiction: String,
}

/// Entropy benefit calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyBenefit {
    /// Entropy quality score (0.0 to 1.0)
    /// The entropy score value
    pub entropy_score: f64,
    /// The benefit multiplier value
    pub benefit_multiplier: f64,
    /// Number of `disitems_percentage`
    pub discount_percentage: f64,
}

/// Entropy-based pricing multipliers for fair cost calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyMultipliers {
    /// Quality-based pricing multiplier
    /// The quality multiplier value
    pub quality_multiplier: f64,
    /// Volume-based pricing multiplier
    /// The volume multiplier value
    pub volume_multiplier: f64,
    /// Diversity-based pricing multiplier
    /// The diversity multiplier value
    pub diversity_multiplier: f64,
}

/// Profile of entropy generation for benefit calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyProfile {
    /// Quality score of entropy generated (0.0 to 1.0)
    /// The entropy quality value
    pub entropy_quality: f64,
    /// Total volume of entropy generated
    /// Number of `entropy_volume`
    pub entropy_volume: u64,
    /// Sources of entropy generation
    /// Collection of entropy sources
    pub entropy_sources: Vec<String>,
}

/// Hyperscale usage indicators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperscaleIndicator {
    /// Number of requests processed
    /// Number of `request_volume`
    pub request_volume: u64,
    /// Amount of data processed in gigabytes
    /// Number of `data_processed_gb`
    pub data_processed_gb: u64,
    /// Whether the usage spans multiple global regions
    /// Whether `global_distribution` is enabled
    pub global_distribution: bool,
}

/// Organization scale metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationScale {
    /// The scale tier value
    pub scale_tier: String,
    /// The growth rate value
    pub growth_rate: f64,
    /// Number of `infrastructure_size`
    pub infrastructure_size: u32,
}

/// Pricing recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRecommendation {
    /// The recommended tier value
    pub recommended_tier: String,
    /// The monthly cost value
    pub monthly_cost: f64,
    /// The justification value
    pub justification: String,
}

/// Pricing tier definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingTier {
    /// Name of the tier
    pub tier_name: String,
    /// The base price value
    pub base_price: f64,
    /// Collection of features
    pub features: Vec<String>,
}

/// Progressive multipliers that scale with usage volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressiveMultipliers {
    /// Collection of volume tiers
    pub volume_tiers: Vec<f64>,
    /// Collection of usage multipliers
    pub usage_multipliers: Vec<f64>,
    /// The enterprise multiplier value
    pub enterprise_multiplier: f64,
}

/// Evidence of organizational scale for pricing decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleEvidence {
    /// Collection of scale indicators
    pub scale_indicators: Vec<String>,
    /// Confidence score of scale assessment (0.0 to 1.0)
    pub confidence_score: f64,
    /// Sources providing evidence
    pub evidence_sources: Vec<String>,
}

/// Usage-based pricing model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageBasedPricing {
    /// The per request cost value
    pub per_request_cost: f64,
    /// Number of `volume_disitemss`
    pub volume_discounts: Vec<f64>,
    /// The minimum monthly value
    pub minimum_monthly: f64,
}

/// Volume pricing tier configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumePricingTier {
    /// Number of `volume_threshold`
    pub volume_threshold: u64,
    /// The price per unit value
    pub price_per_unit: f64,
    /// Number of `disitems_percentage`
    pub discount_percentage: f64,
}
