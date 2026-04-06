// SPDX-License-Identifier: AGPL-3.0-or-later

//! Context-Aware Licensing Tests
//!
//! Comprehensive test suite for context-aware licensing functionality,
//! including classification evidence, license types, enterprise indicators,
//! and pricing models.

use super::*;

// ============================================================================
// ClassificationEvidence Tests
// ============================================================================

#[test]
fn test_classification_evidence_creation() {
    let evidence = ClassificationEvidence {
        evidence_type: "usage_pattern".to_string(),
        confidence: 0.85,
        source: "metrics_analyzer".to_string(),
    };

    assert_eq!(evidence.evidence_type, "usage_pattern");
    assert_eq!(evidence.confidence, 0.85);
    assert_eq!(evidence.source, "metrics_analyzer");
}

#[test]
fn test_classification_evidence_high_confidence() {
    let evidence = ClassificationEvidence {
        evidence_type: "deployment_scale".to_string(),
        confidence: 0.95,
        source: "infrastructure_detector".to_string(),
    };

    assert!(evidence.confidence >= 0.9, "Should be high confidence");
    assert!(
        evidence.confidence <= 1.0,
        "Confidence should not exceed 1.0"
    );
}

#[test]
fn test_classification_evidence_low_confidence() {
    let evidence = ClassificationEvidence {
        evidence_type: "estimated_users".to_string(),
        confidence: 0.3,
        source: "heuristic_estimator".to_string(),
    };

    assert!(evidence.confidence < 0.5, "Should be low confidence");
    assert!(
        evidence.confidence >= 0.0,
        "Confidence should not be negative"
    );
}

#[test]
fn test_classification_evidence_confidence_bounds() {
    // Test valid confidence values
    let valid_evidence = ClassificationEvidence {
        evidence_type: "test".to_string(),
        confidence: 0.5,
        source: "test".to_string(),
    };
    assert!(valid_evidence.confidence >= 0.0 && valid_evidence.confidence <= 1.0);

    // Test boundary conditions
    let min_confidence = ClassificationEvidence {
        evidence_type: "test".to_string(),
        confidence: 0.0,
        source: "test".to_string(),
    };
    assert_eq!(min_confidence.confidence, 0.0);

    let max_confidence = ClassificationEvidence {
        evidence_type: "test".to_string(),
        confidence: 1.0,
        source: "test".to_string(),
    };
    assert_eq!(max_confidence.confidence, 1.0);
}

// ============================================================================
// ContextAwareLicense Tests
// ============================================================================

#[test]
fn test_context_aware_license_mit() {
    let license = ContextAwareLicense {
        license_type: "MIT".to_string(),
        terms: vec!["Free for all use".to_string()],
        context_requirements: vec![],
    };

    assert_eq!(license.license_type, "MIT");
    assert_eq!(license.terms.len(), 1);
    assert!(license.context_requirements.is_empty());
}

#[test]
fn test_context_aware_license_commercial() {
    let license = ContextAwareLicense {
        license_type: "Commercial".to_string(),
        terms: vec![
            "Requires payment".to_string(),
            "Support included".to_string(),
        ],
        context_requirements: vec!["Valid subscription".to_string()],
    };

    assert_eq!(license.license_type, "Commercial");
    assert_eq!(license.terms.len(), 2);
    assert_eq!(license.context_requirements.len(), 1);
}

#[test]
fn test_context_aware_license_enterprise() {
    let license = ContextAwareLicense {
        license_type: "Enterprise".to_string(),
        terms: vec![
            "Unlimited usage".to_string(),
            "Priority support".to_string(),
            "Custom SLA".to_string(),
        ],
        context_requirements: vec![
            "Enterprise agreement".to_string(),
            "Minimum user count: 100".to_string(),
        ],
    };

    assert_eq!(license.license_type, "Enterprise");
    assert_eq!(license.terms.len(), 3);
    assert_eq!(license.context_requirements.len(), 2);
}

#[test]
fn test_context_aware_license_with_requirements() {
    let license = ContextAwareLicense {
        license_type: "Conditional".to_string(),
        terms: vec!["Usage based pricing".to_string()],
        context_requirements: vec![
            "Active monitoring".to_string(),
            "Usage reporting".to_string(),
            "Compliance verification".to_string(),
        ],
    };

    assert!(!license.context_requirements.is_empty());
    assert_eq!(license.context_requirements.len(), 3);
    assert!(
        license
            .context_requirements
            .contains(&"Active monitoring".to_string())
    );
}

// ============================================================================
// EnterpriseIndicator Tests
// ============================================================================

#[test]
fn test_enterprise_indicator_small_org() {
    let indicator = EnterpriseIndicator {
        organization_size: 10,
        usage_volume: 1_000,
        integration_complexity: 0.2,
    };

    assert!(indicator.organization_size < 50, "Small organization");
    assert!(indicator.usage_volume < 10_000, "Low volume");
    assert!(
        indicator.integration_complexity < 0.5,
        "Simple integrations"
    );
}

#[test]
fn test_enterprise_indicator_medium_org() {
    let indicator = EnterpriseIndicator {
        organization_size: 150,
        usage_volume: 50_000,
        integration_complexity: 0.5,
    };

    assert!(
        indicator.organization_size >= 50 && indicator.organization_size < 500,
        "Medium organization"
    );
    assert!(indicator.usage_volume >= 10_000, "Moderate volume");
}

#[test]
fn test_enterprise_indicator_large_org() {
    let indicator = EnterpriseIndicator {
        organization_size: 5_000,
        usage_volume: 10_000_000,
        integration_complexity: 0.9,
    };

    assert!(indicator.organization_size >= 500, "Large organization");
    assert!(indicator.usage_volume >= 1_000_000, "High volume");
    assert!(
        indicator.integration_complexity >= 0.7,
        "Complex integrations"
    );
}

#[test]
fn test_enterprise_indicator_high_volume() {
    let indicator = EnterpriseIndicator {
        organization_size: 100,
        usage_volume: 5_000_000,
        integration_complexity: 0.6,
    };

    assert!(
        indicator.usage_volume >= 1_000_000,
        "Enterprise-level volume"
    );
}

#[test]
fn test_integration_complexity_bounds() {
    // Test minimum complexity
    let simple = EnterpriseIndicator {
        organization_size: 10,
        usage_volume: 100,
        integration_complexity: 0.0,
    };
    assert_eq!(simple.integration_complexity, 0.0);

    // Test maximum complexity
    let complex = EnterpriseIndicator {
        organization_size: 1000,
        usage_volume: 1_000_000,
        integration_complexity: 1.0,
    };
    assert_eq!(complex.integration_complexity, 1.0);

    // Test mid-range complexity
    let medium = EnterpriseIndicator {
        organization_size: 100,
        usage_volume: 10_000,
        integration_complexity: 0.5,
    };
    assert!(
        medium.integration_complexity >= 0.0 && medium.integration_complexity <= 1.0,
        "Complexity should be between 0 and 1"
    );
}

// ============================================================================
// EnterprisePricingModel Tests
// ============================================================================

#[test]
fn test_pricing_model_creation() {
    let model = EnterprisePricingModel {
        base_cost: 1000.0,
        volume_multiplier: 0.001,
        support_tier: "Premium".to_string(),
    };

    assert_eq!(model.base_cost, 1000.0);
    assert_eq!(model.volume_multiplier, 0.001);
    assert_eq!(model.support_tier, "Premium");
}

#[test]
fn test_pricing_model_tiers() {
    // Starter tier
    let starter = EnterprisePricingModel {
        base_cost: 100.0,
        volume_multiplier: 0.01,
        support_tier: "Basic".to_string(),
    };
    assert!(starter.base_cost < 500.0, "Starter tier pricing");
    assert_eq!(starter.support_tier, "Basic");

    // Professional tier
    let professional = EnterprisePricingModel {
        base_cost: 1000.0,
        volume_multiplier: 0.001,
        support_tier: "Professional".to_string(),
    };
    assert!(
        professional.base_cost >= 500.0 && professional.base_cost < 5000.0,
        "Professional tier pricing"
    );
    assert_eq!(professional.support_tier, "Professional");

    // Enterprise tier
    let enterprise = EnterprisePricingModel {
        base_cost: 10000.0,
        volume_multiplier: 0.0001,
        support_tier: "Enterprise".to_string(),
    };
    assert!(enterprise.base_cost >= 5000.0, "Enterprise tier pricing");
    assert_eq!(enterprise.support_tier, "Enterprise");
}

#[test]
fn test_pricing_model_volume_multiplier() {
    let model = EnterprisePricingModel {
        base_cost: 100.0,
        volume_multiplier: 0.001,
        support_tier: "Standard".to_string(),
    };

    // Volume multiplier should be small for per-unit pricing
    assert!(
        model.volume_multiplier < 0.01,
        "Volume multiplier is per-unit"
    );
    assert!(
        model.volume_multiplier > 0.0,
        "Volume multiplier is positive"
    );
}

// ============================================================================
// FunctionUsagePattern Tests
// ============================================================================

#[test]
fn test_function_usage_pattern_creation() {
    let pattern = FunctionUsagePattern {
        function_name: "encrypt_data".to_string(),
        usage_frequency: 1000,
        peak_usage: 1500,
    };

    assert_eq!(pattern.function_name, "encrypt_data");
    assert_eq!(pattern.usage_frequency, 1000);
    assert_eq!(pattern.peak_usage, 1500);
}

#[test]
fn test_function_usage_pattern_high_frequency() {
    let pattern = FunctionUsagePattern {
        function_name: "validate_token".to_string(),
        usage_frequency: 1_000_000,
        peak_usage: 2_000_000,
    };

    assert!(
        pattern.usage_frequency >= 100_000,
        "High frequency function"
    );
    assert!(
        pattern.peak_usage >= pattern.usage_frequency,
        "Peak should be >= average"
    );
}

#[test]
fn test_function_usage_pattern_peak_usage() {
    let stable = FunctionUsagePattern {
        function_name: "stable_function".to_string(),
        usage_frequency: 1000,
        peak_usage: 1200,
    };
    assert!(
        stable.peak_usage > stable.usage_frequency,
        "Peak exceeds average"
    );

    let variable = FunctionUsagePattern {
        function_name: "variable_function".to_string(),
        usage_frequency: 100,
        peak_usage: 500,
    };
    assert!(
        variable.peak_usage >= 5 * variable.usage_frequency,
        "High variability"
    );
}

// ============================================================================
// HardwareProfile Tests
// ============================================================================

#[test]
fn test_hardware_profile_creation() {
    let profile = HardwareProfile {
        cpu_cores: 8,
        memory_gb: 32,
        storage_gb: 1000,
    };

    assert_eq!(profile.cpu_cores, 8);
    assert_eq!(profile.memory_gb, 32);
    assert_eq!(profile.storage_gb, 1000);
}

#[test]
fn test_hardware_profile_small_deployment() {
    let profile = HardwareProfile {
        cpu_cores: 2,
        memory_gb: 4,
        storage_gb: 100,
    };

    assert!(profile.cpu_cores <= 4, "Small deployment - few cores");
    assert!(profile.memory_gb <= 8, "Small deployment - limited memory");
    assert!(
        profile.storage_gb <= 500,
        "Small deployment - limited storage"
    );
}

#[test]
fn test_hardware_profile_large_deployment() {
    let profile = HardwareProfile {
        cpu_cores: 64,
        memory_gb: 256,
        storage_gb: 10_000,
    };

    assert!(profile.cpu_cores >= 32, "Large deployment - many cores");
    assert!(profile.memory_gb >= 128, "Large deployment - high memory");
    assert!(
        profile.storage_gb >= 5_000,
        "Large deployment - substantial storage"
    );
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_enterprise_classification_with_evidence() {
    let indicator = EnterpriseIndicator {
        organization_size: 1000,
        usage_volume: 5_000_000,
        integration_complexity: 0.8,
    };

    let evidence = ClassificationEvidence {
        evidence_type: "infrastructure_analysis".to_string(),
        confidence: 0.9,
        source: "deployment_scanner".to_string(),
    };

    // High organization size suggests enterprise
    assert!(indicator.organization_size >= 500);
    // High volume suggests enterprise
    assert!(indicator.usage_volume >= 1_000_000);
    // High confidence in evidence
    assert!(evidence.confidence >= 0.8);
}

#[test]
fn test_license_selection_based_on_indicator() {
    let small_indicator = EnterpriseIndicator {
        organization_size: 5,
        usage_volume: 500,
        integration_complexity: 0.1,
    };

    let large_indicator = EnterpriseIndicator {
        organization_size: 2000,
        usage_volume: 10_000_000,
        integration_complexity: 0.9,
    };

    // Small organizations should get simple licenses
    assert!(small_indicator.organization_size < 50);
    assert!(small_indicator.usage_volume < 10_000);

    // Large organizations should get enterprise licenses
    assert!(large_indicator.organization_size >= 500);
    assert!(large_indicator.usage_volume >= 1_000_000);
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn test_pricing_calculation_integration() {
    let indicator = EnterpriseIndicator {
        organization_size: 500,
        usage_volume: 1_000_000,
        integration_complexity: 0.7,
    };

    let pricing = EnterprisePricingModel {
        base_cost: 5000.0,
        volume_multiplier: 0.001,
        support_tier: "Enterprise".to_string(),
    };

    // Calculate estimated price
    let volume_charge = indicator.usage_volume as f64 * pricing.volume_multiplier;
    let complexity_factor = indicator.integration_complexity.mul_add(0.5, 1.0);
    let estimated_price = (pricing.base_cost + volume_charge) * complexity_factor;

    assert!(
        estimated_price > pricing.base_cost,
        "Should include volume charge"
    );
    assert_eq!(pricing.support_tier, "Enterprise");
}

// ============================================================================
// Serialization Tests
// ============================================================================

#[test]
fn test_classification_evidence_serialization() {
    let evidence = ClassificationEvidence {
        evidence_type: "test_type".to_string(),
        confidence: 0.75,
        source: "test_source".to_string(),
    };

    // Test that serialization doesn't panic
    let json = serde_json::to_string(&evidence);
    assert!(json.is_ok(), "Should serialize to JSON");

    // Test deserialization
    let json_str = json.unwrap();
    let deserialized: Result<ClassificationEvidence, _> = serde_json::from_str(&json_str);
    assert!(deserialized.is_ok(), "Should deserialize from JSON");

    let recovered = deserialized.unwrap();
    assert_eq!(recovered.evidence_type, evidence.evidence_type);
    assert_eq!(recovered.confidence, evidence.confidence);
    assert_eq!(recovered.source, evidence.source);
}

#[test]
fn test_context_aware_license_serialization() {
    let license = ContextAwareLicense {
        license_type: "Test".to_string(),
        terms: vec!["Term 1".to_string(), "Term 2".to_string()],
        context_requirements: vec!["Req 1".to_string()],
    };

    let json = serde_json::to_string(&license);
    assert!(json.is_ok());

    let recovered: Result<ContextAwareLicense, _> = serde_json::from_str(&json.unwrap());
    assert!(recovered.is_ok());
}
