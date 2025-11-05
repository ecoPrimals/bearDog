//! Core CrossNodeAuthEngine implementation

use super::types::*;
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
    pub fn is_initialized(&self) -> bool {
        true // Engine is initialized when created
    }

    /// Authorize access to a resource
    pub fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        _action: &Action,
        _requested_permission: &str,
    ) -> Result<AuthorizationResult, BearDogError> {
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
    pub fn cleanup_expired_data(&mut self) -> Result<(), BearDogError> {
        let now = Utc::now();
        self.active_authorizations
            .retain(|_, auth| auth.expires_at > now);

        self.spawned_beardogs
            .retain(|_, spawn| spawn.expected_lifetime.map_or(true, |exp| exp > now));

        Ok(())
    }
}
