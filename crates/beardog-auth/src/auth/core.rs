

use chrono::{Duration, Utc};
use std::collections::HashMap;
use uuid::Uuid;
use beardog_errors::BearDogResult;
use beardog_security::{
    Action, AuthorizationResult, Resource, ResourceClassification, RiskLevel, Subject,
};
use super::types::*;

impl CrossNodeAuthEngine {

    pub fn new(
        node_registry: Box<dyn NodeRegistry + Send + Sync>,
        proof_verifier: Box<dyn ProofVerifier + Send + Sync>,
    ) -> Self {
        Self {
            config: CrossNodeAuthConfig::default(),
            active_authorizations: HashMap::with_capacity(16),
            spawned_beardogs: HashMap::with_capacity(16),
            genetics_registry: HashMap::with_capacity(16),
            node_registry,
            proof_verifier,
            workflow_engine: None,
        }
    }

    pub fn with_config(
        config: CrossNodeAuthConfig,
            config,

    pub fn set_workflow_engine(&mut self, workflow_engine: Box<dyn WorkflowEngine + Send + Sync>) {
        self.workflow_engine = Some(workflow_engine);

    pub async fn create_authorization(
        &self,
        subject: &Subject,
        resource: &Resource,
        _action: &Action,
        requested_permission: &str,
    ) -> BearDogResult<AuthorizationResult> {

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

        if trust_level < min_trust_level {
            return Ok(AuthorizationResult {
                permitted: false,
                authorized: false,
                reason: format!("Insufficient trust level: {trust_level} < {min_trust_level}"),
                additional_requirements: Vec::new(),
                risk_level: RiskLevel::High,
                audit_id: Uuid::new_v4().to_string(),
                expires_at: Some(chrono::Utc::now() + chrono::Duration::minutes(60)),
            });

        if self.config.require_consensus {

            return Box::pin(self.create_consensus_authorization(
                &subject.id,
                &resource.id,
                requested_permission,
            ))
            .await;

        let auth_id = Uuid::new_v4().to_string();
        let authorization = CrossNodeAuthorization {
            id: auth_id.clone(),
            requester_node_id: subject.id.clone(),
            resource_owner_node_id: self.get_resource_owner(&resource.id)?,
            resource_id: resource.id.clone(),
            permissions: self.parse_permissions(&requested_permission)?,
            conditions: vec![],
            created_at: Utc::now(),
            expires_at: Utc::now()
                + Duration::minutes(self.config.max_proof_validity_minutes as i64),
            signature: self.generate_session_signature(&session_data).await?,
            is_active: true,
        Ok(AuthorizationResult {
            permitted: true,
            authorized: true,
            reason: "Direct authorization granted".to_string(),
            additional_requirements: Vec::new(),
            risk_level: RiskLevel::Low,
            audit_id: auth_id,
            expires_at: Some(authorization.expires_at),
        })

    pub fn get_node_authorizations(&self, node_id: &str) -> Vec<&CrossNodeAuthorization> {
        self.active_authorizations
            .values()
            .filter(|auth| auth.requester_node_id == node_id)
            .collect()

    pub fn get_authorization_metrics(&self) -> HashMap<String, u64> {
        let mut metrics = HashMap::with_capacity(16);
        metrics.insert(
            "total_authorizations".to_string(),
            self.active_authorizations.len() as u64,
        );
            "active_spawns".to_string(),
            self.spawned_beardogs.len() as u64,
            "registered_genetics".to_string(),
            self.genetics_registry.len() as u64,

        let active_count = self
            .active_authorizations
            .filter(|auth| auth.is_active)
            .count() as u64;
        metrics.insert("active_authorizations".to_string(), active_count);

        let now = Utc::now();
        let expired_count = self
            .filter(|auth| auth.expires_at < now)
        metrics.insert("expired_authorizations".to_string(), expired_count);
        metrics

    pub async fn cleanup_expired_data(&mut self) -> BearDogResult<()> {

            .retain(|_, auth| auth.expires_at > now);

        self.spawned_beardogs
            .retain(|_, spawn| spawn.expected_lifetime.map_or(true, |exp| exp > now));
        Ok(())

    async fn generate_session_signature(&self, session_data: &str) -> BearDogResult<String> {
        use beardog_security::crypto_utils::BearDogCrypto;

        let (private_key, _public_key) = BearDogCrypto::generate_ed25519_keypair()?;

        let signature = BearDogCrypto::sign_ed25519(&private_key, session_data.as_bytes())?;

        Ok(hex::encode(signature))

    fn get_resource_owner(&self, resource_id: &str) -> BearDogResult<String> {

        if resource_id.starts_with("system_") {
            Ok("system_node".to_string())
        } else if resource_id.starts_with("user_") {
            Ok("user_node".to_string())
        } else {
            Ok("default_node".to_string())

    fn parse_permissions(&self, permission_str: &str) -> BearDogResult<Vec<ResourcePermission>> {
        match permission_str.to_lowercase().as_str() {
            "read" => Ok(vec![ResourcePermission::Read]),
            "write" => Ok(vec![ResourcePermission::Write]),
            "execute" => Ok(vec![ResourcePermission::Execute]),
            "admin" => Ok(vec![
                ResourcePermission::Read,
                ResourcePermission::Write,
                ResourcePermission::Execute,
            ]),
            _ => Ok(vec![ResourcePermission::Read]), // Default to read-only
}
