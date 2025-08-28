use beardog_errors::BearDogError;


use super::mocks::*;
use crate::auth::types::*;
use beardog_security::handlers::threat_analysis::{
    Action, ActionType, Resource, ResourceClassification, Subject, SubjectType,
};
use chrono::Utc;
use std::collections::HashMap;

#[test]
fn test_cross_node_auth_engine_creation() {
    let node_registry = Box::new(MockNodeRegistry);
    let proof_verifier = Box::new(MockProofVerifier);
    let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier);

    assert!(engine.config.proof_verification_enabled);
    assert_eq!(engine.config.max_proof_validity_minutes, 60);
    assert!(engine.active_authorizations.is_empty());
    assert!(engine.spawned_beardogs.is_empty());
}

fn test_cross_node_auth_engine_with_config() {
    let config = CrossNodeAuthConfig {
        proof_verification_enabled: true,
        max_proof_validity_minutes: 120,
        genetic_spawning_enabled: true,
        require_consensus: true,
        consensus_threshold: 0.8,
        max_spawns_per_node: 50,
        automated_approval_enabled: true,
    };
    let engine = CrossNodeAuthEngine::with_config(config, node_registry, proof_verifier);
    assert_eq!(engine.config.max_spawns_per_node, 50);
    assert!(engine.config.require_consensus);
    assert_eq!(engine.config.consensus_threshold, 0.8);
    assert!(engine.config.genetic_spawning_enabled);

#[tokio::test]
async fn test_create_authorization_trusted_node() -> Result<(), Box<dyn std::error::Error>> {
    let subject = Subject {
        id: "trusted-node".to_string(),
        name: "Trusted Service".to_string(),
        subject_type: SubjectType::Service,
        user_id: "trusted-node".to_string(),
        roles: vec!["service".to_string()],
        permissions: vec!["read".to_string()],
        trust_level: 0.8,
        clearance_level: Some(3),
        metadata: HashMap::with_capacity(16),
    let resource = Resource {
        id: "test-resource".to_string(),
        name: "Test Resource".to_string(),
        classification: ResourceClassification::Internal,
        resource_id: "test-resource".to_string(),
        resource_type: "data".to_string(),
        sensitivity_level: 0.5,
        access_patterns: vec!["read".to_string()],
    let action = Action {
        action_type: ActionType::Read,
        description: "read-data".to_string(),
        risk_level: beardog_security::types::RiskLevel::Low,
        timestamp: Utc::now(),
        context: beardog_types::canonical::SecurityContext::default(),
        risk_indicators: vec![],
    let result = engine
        .create_authorization(&subject, &resource, &action, "read")
        .await;
    assert!(result.is_ok());
    let auth_result = result.map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Authorization should succeed",
            e
        );
        beardog_errors::BearDogError::internal(format!(
            "Authorization should succeed", e
        ))
    })?;
    assert!(auth_result.allowed);

    Ok(())

async fn test_create_authorization_untrusted_node() -> Result<(), BearDogError> {
        id: "untrusted-node".to_string(),
        name: "Untrusted Service".to_string(),
        user_id: "untrusted-node".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
        trust_level: 0.2,
        clearance_level: Some(1),
        id: "secret-resource".to_string(),
        name: "Secret Resource".to_string(),
        classification: ResourceClassification::Secret,
        resource_id: "secret-resource".to_string(),
        sensitivity_level: 1.0,
        access_patterns: vec!["read".to_string(), "write".to_string()],
        action_type: ActionType::Write,
        description: "modify-data".to_string(),
        risk_level: beardog_security::types::RiskLevel::High,
        .create_authorization(&subject, &resource, &action, "write")
            "Authorization should complete",
            "Authorization should complete", e
    assert!(!auth_result.allowed); // Should be denied due to low trust

async fn test_comprehensive_authorization_workflow() -> Result<(), BearDogError> {

    let scenarios = vec![

        (
            "trusted-node",
            ResourceClassification::Public,
            ActionType::Read,
            true,
        ),
            ResourceClassification::Secret,
            "medium-node",
            ResourceClassification::Internal,
            false,
            "untrusted-node",
    ];
    for (i, (subject_id, resource_class, action_type, expected)) in
        scenarios.into_iter().enumerate()
    {
        let subject = Subject {
            id: subject_id.to_string(),
            name: format!("Test Subject {i}"),
            subject_type: SubjectType::Service,
            user_id: subject_id.to_string(),
            roles: vec!["service".to_string()],
            permissions: vec!["read".to_string()],
            trust_level: 0.5,
            clearance_level: Some(2),
            metadata: HashMap::with_capacity(16),
        };
        let resource = Resource {
            id: format!("resource-{i}"),
            name: format!("Test Resource {i}"),
            classification: resource_class.clone(),
            resource_id: format!("resource-{i}"),
            resource_type: "data".to_string(),
            sensitivity_level: 0.7,
            access_patterns: vec!["read".to_string()],
        let action = Action {
            action_type,
            description: format!("action-{i}"),
            risk_level: beardog_security::types::RiskLevel::Medium,
            timestamp: Utc::now(),
            context: beardog_types::canonical::SecurityContext::default(),
            risk_indicators: vec![],
        let result = engine
            .create_authorization(&subject, &resource, &action, "test")
            .await;
        assert!(result.is_ok(), "Authorization failed for scenario {i}");
        let auth_result = result.map_err(|e| {
            tracing::error!(
                "Operation failed ({}): {:?}",
                "Authorization should complete",
                e
            );
            beardog_errors::BearDogError::internal(format!(
                "Authorization should complete", e
            ))
        })?;
        assert_eq!(auth_result.allowed, expected,
            "Unexpected authorization result for scenario {}: subject={}, resource={:?}, expected={}, got={}", 
            i, subject_id, resource_class, expected, auth_result.allowed);
    }

async fn test_proof_verification() -> Result<(), BearDogError> {
    let authorization = CrossNodeAuthorization {
        id: "test-authorization".to_string(),
        requester_node_id: "trusted-node".to_string(),
        resource_owner_node_id: "owner-node".to_string(),
        permissions: vec![],
        conditions: vec![],
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        signature: "test-signature".to_string(),
        is_active: true,
    let operation = CrossNodeOperation {
        operation_type: OperationType::Read,
        target_resource: "test-resource".to_string(),
        parameters: HashMap::with_capacity(16),
        requester_signature: "test-signature".to_string(),

    let proof_result = engine
        .proof_verifier
        .generate_proof(&authorization, &operation);
    assert!(proof_result.is_ok());
    let proof = proof_result.map_err(|e| {
            "Proof creation should succeed",
            "Proof creation should succeed", e
    assert_eq!(proof.authorization_id, "test-authorization");
    assert!(!proof.proof_signature.is_empty());

    let verification_result = engine.proof_verifier.verify_authorization_proof(&proof);
    assert!(verification_result.is_ok());
    assert!(verification_result.map_err(|e| {
            "Verification should succeed",
            "Verification should succeed", e
    })?);

fn test_cross_node_auth_config_default() {
    let config = CrossNodeAuthConfig::default();
    assert!(config.proof_verification_enabled);
    assert_eq!(config.max_proof_validity_minutes, 60);
    assert!(config.genetic_spawning_enabled);
    assert!(!config.require_consensus);
    assert_eq!(config.consensus_threshold, 0.67);
    assert_eq!(config.max_spawns_per_node, 10);
    assert!(config.automated_approval_enabled);
