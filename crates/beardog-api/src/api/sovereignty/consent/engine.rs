

use super::models::{
    ConsentAuditEvent, ConsentCondition, ConsentRecordInternal, ConsentRequestInternal,
    ConsentScopeInternal, ConsentTemplate,
};
use super::records::ConsentRecordManager;
use super::requests::ConsentRequestProcessor;
use super::templates::ConsentTemplateManager;
use super::types::{ActionType, ConsentAuditEventType, ConsentStatus, ResourceType, UrgencyLevel};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

pub struct ConsentManagementEngine {

    active_consents: Arc<RwLock<HashMap<String, ConsentRecordInternal>>>,

    pending_requests: Arc<RwLock<HashMap<String, ConsentRequestInternal>>>,

    consent_templates: Arc<RwLock<HashMap<String, ConsentTemplate>>>,

    request_processor: Arc<ConsentRequestProcessor>,

    record_manager: Arc<ConsentRecordManager>,

    template_manager: Arc<ConsentTemplateManager>,
}
impl Default for ConsentManagementEngine {}

    fn default() -> Self {
        Self::new()
    }
impl ConsentManagementEngine {

    pub fn new() -> Self {
        info!("🤝 Initializing Consent Management Engine for Individual Sovereignty");
        Self {
            active_consents: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            pending_requests: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            consent_templates: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            request_processor: Arc::new(ConsentRequestProcessor::new()),
            record_manager: Arc::new(ConsentRecordManager::new()),
            template_manager: Arc::new(ConsentTemplateManager::new()),
        }

    pub async fn initialize_default_templates(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("📋 Initializing default consent templates");

        self.template_manager.create_default_templates().await?;

        let templates = self.template_manager.get_all_templates().await?;
        {
            let mut template_map = self.consent_templates.write().await;
            for template in templates {
                template_map.insert(template.template_id.clone(), template);
            }
        info!("✅ Default consent templates initialized");
        Ok(())

    pub async fn request_consent(
        requester_id: &str,
        target_id: &str,
        scope: ConsentScopeInternal,
        justification: &str,
        urgency: UrgencyLevel,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "📝 Processing consent request from {} to {}",
            requester_id, target_id
        );
        let request_id = Uuid::new_v4().to_string();
        let request = ConsentRequestInternal {
            request_id: request_id.clone(),
            requester_id: requester_id.clone(),
            requester_display_name: format!("User {requester_id}"),
            target_id: target_id.clone(),
            requested_scope: scope,
            justification,
            urgency_level: urgency,
            created_at: Utc::now(),
            expires_at: Utc::now() + chrono::Duration::hours(24), // Default 24h expiry
            workflow_id: None,
            response_deadline: Some(Utc::now() + chrono::Duration::hours(6)),
        };

            let mut pending = self.pending_requests.write().await;
            pending.insert(request_id.clone(), request);

        self.request_processor.process_request(&request_id).await?;
        info!("✅ Consent request {} submitted successfully", request_id);
        Ok(request_id)

    pub async fn grant_consent(
        request_id: &str,
        grantor_id: &str,
        conditions: Vec<ConsentCondition>,
        expires_at: Option<DateTime<Utc>>,
        info!("✅ Granting consent for request: {}", request_id);

        let request = {
            pending.remove(request_id).ok_or("Request not found")?

        if request.target_id != grantor_id {
            return Err("Grantor does not have authority for this request".into());

        let consent_id = Uuid::new_v4().to_string();
        let consent_record = ConsentRecordInternal {
            consent_id: consent_id.clone(),
            grantor_id: grantor_id.to_string(),
            grantee_id: request.requester_id,
            consent_scope: request.requested_scope,
            granted_at: Utc::now(),
            expires_at,
            status: ConsentStatus::Granted,
            revocable: true,
            usage_count: 0,
            last_used: None,
            conditions,
            audit_trail: vec![ConsentAuditEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: ConsentAuditEventType::Granted,
                timestamp: Utc::now(),
                actor_id: grantor_id.to_string(),
                details: "Consent granted".to_string(),
                metadata: HashMap::with_capacity(16),
            }],

            let mut active = self.active_consents.write().await;
            active.insert(consent_id.clone(), consent_record);
        info!("🎉 Consent {} granted successfully", consent_id);
        Ok(consent_id)

    pub async fn deny_consent(
        reason: &str,
        info!("❌ Denying consent for request: {}", request_id);

        let mut pending = self.pending_requests.write().await;
        if let Some(_request) = pending.remove(request_id) {

            debug!(
                "Consent denied by {} for request {}: {}",
                grantor_id, request_id, reason
            );
            info!("✅ Consent request {} denied", request_id);
            Ok(())
        } else {
            Err("Request not found".into())

    pub async fn revoke_consent(
        consent_id: &str,
        info!("🚫 Revoking consent: {}", consent_id);
        let mut active = self.active_consents.write().await;
        if let Some(consent) = active.get_mut(consent_id) {

            if consent.grantor_id != grantor_id {
                return Err("Grantor does not have authority to revoke this consent".into());

            if !consent.revocable {
                return Err("This consent is not revocable".into());

            consent.status = ConsentStatus::Revoked;

            consent.audit_trail.push(ConsentAuditEvent {
                event_type: ConsentAuditEventType::Revoked,
                details: reason,
            });
            info!("✅ Consent {} revoked successfully", consent_id);
            Err("Consent not found".into())

    pub async fn check_consent(
        grantee_id: &str,
        resource_type: &ResourceType,
        action_type: &ActionType,
    ) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "🔍 Checking consent for {} to perform {:?} on {:?}",
            grantee_id, action_type, resource_type
        let active = self.active_consents.read().await;
        for (consent_id, consent) in active.iter() {

            if consent.grantee_id != grantee_id {
                continue;

            if consent.status != ConsentStatus::Granted {

            if let Some(expires_at) = consent.expires_at {
                if Utc::now() > expires_at {
                    continue;
                }

            let has_resource_access = consent
                .consent_scope
                .resource_types
                .iter()
                .any(|rta| rta.resource_type == *resource_type);
            if !has_resource_access {

            let has_action_permission = consent
                .permitted_actions
                .any(|pa| pa.action_type == *action_type);
            if has_action_permission {
                debug!("✅ Valid consent found: {}", consent_id);
                return Ok(Some(consent_id.clone()));
        debug!("❌ No valid consent found");
        Ok(None)

    pub async fn get_user_consents(&self, user_id: &str) -> Vec<ConsentRecordInternal> {
        active
            .values()
            .filter(|consent| consent.grantor_id == user_id || consent.grantee_id == user_id)
            .filter(|consent| consent.status == ConsentStatus::Granted)
            .cloned()
            .collect()

    pub async fn get_pending_requests(&self, target_id: &str) -> Vec<ConsentRequestInternal> {
        let pending = self.pending_requests.read().await;
        pending
            .filter(|request| request.target_id == target_id)

    pub async fn cleanup_expired(
    ) -> Result<(u32, u32), Box<dyn std::error::Error + Send + Sync>> {
        let now = Utc::now();
        let mut expired_consents = 0;
        let mut expired_requests = 0;

            let expired_ids: Vec<String> = active
                .filter(|(_, consent)| {
                    if let Some(expires_at) = consent.expires_at {
                        now > expires_at
                    } else {
                        false
                    }
                })
                .map(|(id, _)| id.clone())
                .collect();
            for id in expired_ids {
                if let Some(consent) = active.get_mut(&id) {
                    consent.status = ConsentStatus::Expired;
                    expired_consents += 1;

            let expired_ids: Vec<String> = pending
                .filter(|(_, request)| now > request.expires_at)
                pending.remove(&id);
                expired_requests += 1;
        if expired_consents > 0 || expired_requests > 0 {
            info!(
                "🧹 Cleanup complete: {} expired consents, {} expired requests",
                expired_consents, expired_requests
        Ok((expired_consents, expired_requests))
