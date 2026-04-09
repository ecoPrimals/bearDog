// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core `CrossNodeAuthEngine` implementation

use super::types::{
    CrossNodeAuthConfig, CrossNodeAuthEngine, CrossNodeAuthorization, NodeRegistry, ProofVerifier,
    WorkflowEngine,
};
use beardog_errors::BearDogError;
use beardog_security::{
    Action, AuthorizationResult, Resource, ResourceClassification, RiskLevel, Subject,
};
use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;

impl CrossNodeAuthEngine {
    /// Creates a new instance
    pub fn new(
        node_registry: Box<dyn NodeRegistry + Send + Sync>,
        proof_verifier: Box<dyn ProofVerifier + Send + Sync>,
        config: CrossNodeAuthConfig,
    ) -> Self {
        Self {
            config,
            active_authorizations: HashMap::new(),
            spawned_beardogs: HashMap::new(),
            genetics_registry: HashMap::new(),
            node_registry,
            proof_verifier,
            workflow_engine: None,
            consensus_registry: std::collections::BTreeMap::new(),
        }
    }

    /// Creates instance with default config
    pub fn with_default_config(
        node_registry: Box<dyn NodeRegistry + Send + Sync>,
        proof_verifier: Box<dyn ProofVerifier + Send + Sync>,
    ) -> Self {
        Self::new(
            node_registry,
            proof_verifier,
            CrossNodeAuthConfig::default(),
        )
    }

    /// Sets workflow engine
    pub fn set_workflow_engine(&mut self, workflow_engine: Box<dyn WorkflowEngine + Send + Sync>) {
        self.workflow_engine = Some(workflow_engine);
    }

    /// Check if engine is initialized
    pub const fn is_initialized(&self) -> bool {
        true // Engine is initialized when created
    }

    /// Authorize access to a resource
    ///
    /// # Errors
    ///
    /// Currently always returns [`AuthorizationResult`]; the `Result` type is reserved for future
    /// registry failures.
    pub fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        _action: &Action,
        _requested_permission: &str,
    ) -> Result<AuthorizationResult, BearDogError> {
        // Get trust level from registry, defaulting to 0.0 for unknown nodes
        let trust_level = self
            .node_registry
            .get_trust_level(&subject.id)
            .unwrap_or(0.0);

        let min_trust_level = match resource.classification {
            ResourceClassification::Public => 0.0,
            ResourceClassification::Internal => 0.4,
            ResourceClassification::Confidential => 0.6,
            ResourceClassification::Secret => 0.8,
            ResourceClassification::TopSecret => 0.9,
        };

        let authorized = trust_level >= min_trust_level;
        let reason = if authorized {
            "Authorization granted based on trust level".to_string()
        } else {
            format!("Insufficient trust level: {trust_level} < {min_trust_level}")
        };

        Ok(AuthorizationResult {
            authorized,
            reason,
            risk_level: if authorized {
                RiskLevel::Low
            } else {
                RiskLevel::High
            },
            audit_id: Uuid::new_v4().to_string(),
            expires_at: Some(Utc::now() + Duration::minutes(60)),
            additional_requirements: Vec::new(),
        })
    }

    /// Get node authorizations
    pub fn get_node_authorizations(&self, node_id: &str) -> Vec<&CrossNodeAuthorization> {
        self.active_authorizations
            .values()
            .filter(|auth| auth.requester_node_id == node_id)
            .collect()
    }

    /// Get authorization metrics
    pub fn get_authorization_metrics(&self) -> HashMap<String, u64> {
        let mut metrics = HashMap::new();
        metrics.insert(
            "total_authorizations".to_string(),
            self.active_authorizations.len() as u64,
        );
        metrics.insert(
            "active_spawns".to_string(),
            self.spawned_beardogs.len() as u64,
        );
        metrics.insert(
            "registered_genetics".to_string(),
            self.genetics_registry.len() as u64,
        );

        let active_count = self
            .active_authorizations
            .values()
            .filter(|auth| auth.is_active)
            .count() as u64;
        metrics.insert("active_authorizations".to_string(), active_count);

        let now = Utc::now();
        let expired_count = self
            .active_authorizations
            .values()
            .filter(|auth| auth.expires_at < now)
            .count() as u64;
        metrics.insert("expired_authorizations".to_string(), expired_count);

        metrics
    }

    /// Cleanup expired data
    ///
    /// # Errors
    ///
    /// Currently always succeeds; the `Result` type is reserved for future persistence failures.
    pub fn cleanup_expired_data(&mut self) -> Result<(), BearDogError> {
        let now = Utc::now();
        self.active_authorizations
            .retain(|_, auth| auth.expires_at > now);

        self.spawned_beardogs
            .retain(|_, spawn| spawn.expected_lifetime.is_none_or(|exp| exp > now));

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::node_registry::InMemoryNodeRegistry;
    use crate::auth::proof_verifier::DefaultProofVerifier;
    use beardog_security::{ActionType, SubjectType};

    #[test]
    fn test_new_engine_with_config() {
        let config = CrossNodeAuthConfig::default();
        let node_registry = Box::new(InMemoryNodeRegistry::new());
        let proof_verifier = Box::new(DefaultProofVerifier::new());

        let engine = CrossNodeAuthEngine::new(node_registry, proof_verifier, config.clone());

        assert_eq!(engine.config.verification_mode, config.verification_mode);
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_with_default_config() {
        let node_registry = Box::new(InMemoryNodeRegistry::new());
        let proof_verifier = Box::new(DefaultProofVerifier::new());

        let engine = CrossNodeAuthEngine::with_default_config(node_registry, proof_verifier);

        assert_eq!(engine.config.max_proof_validity_minutes, 60);
        assert!(engine.is_initialized());
    }

    #[test]
    fn test_is_initialized_always_true() {
        let engine = CrossNodeAuthEngine::default();
        assert!(
            engine.is_initialized(),
            "Engine should be initialized upon creation"
        );
    }

    #[test]
    fn test_authorize_public_resource() {
        let engine = CrossNodeAuthEngine::default();

        let subject = Subject {
            id: "test-subject".to_string(),
            name: "Test Subject".to_string(),
            subject_type: SubjectType::User,
            roles: vec![],
            clearance_level: None,
            metadata: HashMap::new(),
        };

        let resource = Resource {
            name: "public-resource".to_string(),
            classification: ResourceClassification::Public,
            metadata: HashMap::new(),
        };

        let action = Action {
            name: "read-action".to_string(),
            action_type: ActionType::Read,
            metadata: HashMap::new(),
        };

        let result = engine.authorize(&subject, &resource, &action, "read");
        assert!(result.is_ok());
        assert!(
            result.unwrap().authorized,
            "Public resources should be accessible"
        );
    }

    #[test]
    fn test_authorize_confidential_resource_insufficient_trust() {
        let engine = CrossNodeAuthEngine::default();

        let subject = Subject {
            id: "low-trust-subject".to_string(),
            name: "Low Trust Subject".to_string(),
            subject_type: SubjectType::User,
            roles: vec![],
            clearance_level: None,
            metadata: HashMap::new(),
        };

        let resource = Resource {
            name: "confidential-resource".to_string(),
            classification: ResourceClassification::Confidential,
            metadata: HashMap::new(),
        };

        let action = Action {
            name: "read-action".to_string(),
            action_type: ActionType::Read,
            metadata: HashMap::new(),
        };

        let result = engine.authorize(&subject, &resource, &action, "read");
        assert!(result.is_ok());
        assert!(
            !result.unwrap().authorized,
            "Confidential resources require trust >= 0.6"
        );
    }

    #[test]
    fn test_get_node_authorizations_empty() {
        let engine = CrossNodeAuthEngine::default();
        let auths = engine.get_node_authorizations("nonexistent-node");

        assert!(
            auths.is_empty(),
            "Should return empty list for node with no authorizations"
        );
    }

    #[test]
    fn test_get_authorization_metrics_initial() {
        let engine = CrossNodeAuthEngine::default();
        let metrics = engine.get_authorization_metrics();

        assert_eq!(metrics.get("total_authorizations"), Some(&0));
        assert_eq!(metrics.get("active_spawns"), Some(&0));
        assert_eq!(metrics.get("registered_genetics"), Some(&0));
        assert_eq!(metrics.get("active_authorizations"), Some(&0));
        assert_eq!(metrics.get("expired_authorizations"), Some(&0));
    }

    #[test]
    fn test_cleanup_expired_data_no_data() {
        let mut engine = CrossNodeAuthEngine::default();
        let result = engine.cleanup_expired_data();

        assert!(result.is_ok(), "Cleanup should succeed even with no data");
    }

    #[test]
    fn test_cleanup_expired_data_removes_expired_authorizations() {
        let mut engine = CrossNodeAuthEngine::default();

        // Add expired authorization
        let expired_auth = CrossNodeAuthorization {
            request_id: "expired-1".to_string(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "resource-1".to_string(),
            permissions: vec![],
            conditions: vec![],
            created_at: Utc::now() - Duration::hours(2),
            expires_at: Utc::now() - Duration::hours(1),
            signature: "sig".to_string(),
            is_active: true,
        };
        engine
            .active_authorizations
            .insert("expired-1".to_string(), expired_auth);

        // Add valid authorization
        let valid_auth = CrossNodeAuthorization {
            request_id: "valid-1".to_string(),
            requester_node_id: "node-1".to_string(),
            resource_owner_node_id: "node-2".to_string(),
            resource_id: "resource-2".to_string(),
            permissions: vec![],
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
            signature: "sig".to_string(),
            is_active: true,
        };
        engine
            .active_authorizations
            .insert("valid-1".to_string(), valid_auth);

        let result = engine.cleanup_expired_data();
        assert!(result.is_ok());

        assert!(
            !engine.active_authorizations.contains_key("expired-1"),
            "Expired auth should be removed"
        );
        assert!(
            engine.active_authorizations.contains_key("valid-1"),
            "Valid auth should remain"
        );
    }

    #[test]
    fn test_set_workflow_engine() {
        use crate::auth::types::node_registry::WorkflowEngine;
        use crate::auth::types::workflow::{CrossNodeWorkflowRequest, WorkflowStatus};

        struct MockWorkflowEngine;
        impl WorkflowEngine for MockWorkflowEngine {
            fn submit_workflow(
                &mut self,
                _request: CrossNodeWorkflowRequest,
            ) -> Result<String, BearDogError> {
                Ok("workflow-id".to_string())
            }
            fn get_workflow_status(
                &self,
                _workflow_id: &str,
            ) -> Result<WorkflowStatus, BearDogError> {
                Ok(WorkflowStatus::Pending)
            }
        }

        let mut engine = CrossNodeAuthEngine::default();
        assert!(
            engine.workflow_engine.is_none(),
            "Should start without workflow engine"
        );

        engine.set_workflow_engine(Box::new(MockWorkflowEngine));
        assert!(
            engine.workflow_engine.is_some(),
            "Should have workflow engine after setting"
        );
    }
}
