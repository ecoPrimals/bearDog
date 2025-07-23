//! Consent Management for Individual Sovereignty
//!
//! **Handles explicit consent mechanisms for all peer-to-peer interactions**
//!
//! This module ensures that every interaction, data sharing, and resource access
//! happens only with explicit, informed consent from individuals. Your consent
//! is sacred and cannot be bypassed or assumed.
//!
//! ## Core Consent Principles
//! - **Explicit Consent**: Every action requires clear, specific approval
//! - **Informed Consent**: You understand exactly what you're agreeing to
//! - **Granular Control**: Consent can be specific to exact operations and data
//! - **Revocable Always**: You can withdraw consent at any time
//! - **Audit Trail**: Complete history of all consent decisions
//!
//! ## Consent Features
//! - Fine-grained permission management
//! - Time-limited consent with automatic expiration
//! - Conditional consent based on context
//! - Multi-party consent workflows
//! - Privacy-preserving consent verification
//! - Consent delegation for trusted parties

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

use super::models::{ConsentRecord, ConsentScope, ConsentStatus, PrivacyLevel};

/// Consent management engine for individual sovereignty
pub struct ConsentManagementEngine {
    /// Active consent records (granted)
    active_consents: Arc<RwLock<HashMap<String, ConsentRecordInternal>>>,
    /// Consent requests (pending approval)
    pending_requests: Arc<RwLock<HashMap<String, ConsentRequestInternal>>>,
    /// Consent templates for common scenarios
    consent_templates: Arc<RwLock<HashMap<String, ConsentTemplate>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentRecordInternal {
    pub consent_id: String,
    pub grantor_id: String,
    pub grantee_id: String,
    pub consent_scope: ConsentScopeInternal,
    pub granted_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: ConsentStatus,
    pub revocable: bool,
    pub usage_count: u64,
    pub last_used: Option<DateTime<Utc>>,
    pub conditions: Vec<ConsentCondition>,
    pub audit_trail: Vec<ConsentAuditEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentRequestInternal {
    pub request_id: String,
    pub requester_id: String,
    pub requester_display_name: String,
    pub target_id: String,
    pub requested_scope: ConsentScopeInternal,
    pub justification: String,
    pub urgency_level: UrgencyLevel,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub workflow_id: Option<String>,
    pub response_deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentScopeInternal {
    pub resource_types: Vec<ResourceTypeAccess>,
    pub permitted_actions: Vec<PermittedAction>,
    pub usage_limits: HashMap<String, UsageLimit>,
    pub privacy_level: PrivacyLevel,
    pub geographic_restrictions: Option<Vec<String>>,
    pub temporal_restrictions: Option<TemporalRestriction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentTemplate {
    pub template_id: String,
    pub template_name: String,
    pub template_type: TemplateType,
    pub default_scope: ConsentScopeInternal,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub usage_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentDelegation {
    pub delegation_id: String,
    pub delegator_id: String,
    pub delegate_id: String,
    pub delegate_display_name: String,
    pub delegated_authority: DelegatedAuthority,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub conditions: Vec<DelegationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub condition_data: HashMap<String, String>,
    pub evaluation_result: Option<bool>,
    pub last_evaluated: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConsentAuditEvent {
    pub event_id: String,
    pub event_type: ConsentEventType,
    pub timestamp: DateTime<Utc>,
    pub actor_id: String,
    pub details: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ResourceTypeAccess {
    pub resource_type: String,
    pub access_level: AccessLevel,
    pub specific_resources: Option<Vec<String>>,
    pub exclusions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PermittedAction {
    pub action: String,
    pub parameters: HashMap<String, String>,
    pub frequency_limit: Option<FrequencyLimit>,
    pub requires_notification: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UsageLimit {
    pub limit_type: LimitType,
    pub limit_value: u64,
    pub time_window: Option<Duration>,
    pub current_usage: u64,
    pub reset_schedule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TemporalRestriction {
    pub allowed_hours: Option<Vec<u8>>,
    pub allowed_days: Option<Vec<u8>>,
    pub timezone: String,
    pub blackout_periods: Option<Vec<(DateTime<Utc>, DateTime<Utc>)>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DelegatedAuthority {
    pub authority_type: AuthorityType,
    pub scope_limitations: Option<ConsentScopeInternal>,
    pub max_consent_duration: Option<Duration>,
    pub notification_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DelegationCondition {
    pub condition_type: DelegationConditionType,
    pub condition_value: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UrgencyLevel {
    Low,
    Normal,
    High,
    Urgent,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum TemplateType {
    ResourceSharing,
    DataAccess,
    IdentityVerification,
    RecoveryAssistance,
    Emergency,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum AuthorityType {
    ResourceSharing,
    DataAccess,
    IdentityManagement,
    EmergencyResponse,
    Limited,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    TimeWindow,
    GeographicLocation,
    NetworkCondition,
    UserPresence,
    ResourceAvailability,
    TrustScore,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum ConsentEventType {
    Granted,
    Revoked,
    Modified,
    Used,
    Expired,
    ConditionViolation,
    DelegationCreated,
    DelegationRevoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum AccessLevel {
    Read,
    Write,
    Execute,
    Admin,
    Full,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum LimitType {
    Count,
    Size,
    Duration,
    Bandwidth,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum FrequencyLimit {
    Minute(u32),
    Hour(u32),
    Day(u32),
    Week(u32),
    Month(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum DelegationConditionType {
    MaxDuration,
    SpecificResources,
    EmergencyOnly,
    CustomCondition,
}

impl Default for ConsentManagementEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for consent requests
#[derive(Debug, Clone)]
pub struct ConsentRequestParams {
    pub requester_id: String,
    pub requester_display_name: String,
    pub target_id: String,
    pub requested_scope: ConsentScope,
    pub justification: String,
    pub urgency: UrgencyLevel,
    pub response_deadline_hours: Option<i64>,
}

impl ConsentManagementEngine {
    /// Create new consent management engine
    pub fn new() -> Self {
        info!("⚖️ Initializing consent management system for individual sovereignty");

        Self {
            active_consents: Arc::new(RwLock::new(HashMap::new())),
            pending_requests: Arc::new(RwLock::new(HashMap::new())),
            consent_templates: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initialize default consent templates
    pub async fn initialize_default_templates(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("📝 Initializing default consent templates");

        let templates = vec![
            self.create_resource_sharing_template(),
            self.create_data_access_template(),
            self.create_identity_verification_template(),
            self.create_emergency_template(),
        ];

        let mut template_storage = self.consent_templates.write().await;
        for template in templates {
            template_storage.insert(template.template_id.clone(), template);
        }

        info!("✅ Default consent templates initialized");
        Ok(())
    }

    /// Request consent from an individual
    pub async fn request_consent(
        &self,
        params: ConsentRequestParams,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!(
            "📤 Requesting consent from {} by {}",
            params.target_id, params.requester_display_name
        );

        let request_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::hours(72); // Default 72 hour expiration
        let response_deadline = params
            .response_deadline_hours
            .map(|h| now + Duration::hours(h));

        // Convert to internal scope format
        let requested_scope_internal = ConsentScopeInternal {
            resource_types: Vec::new(), // Would be populated from requested_scope
            permitted_actions: Vec::new(),
            usage_limits: HashMap::new(),
            privacy_level: params.requested_scope.privacy_level,
            geographic_restrictions: None,
            temporal_restrictions: None,
        };

        // Create consent workflow
        let workflow_id = self
            .create_consent_workflow(
                &request_id,
                &params.requester_id,
                &params.target_id,
                &params.justification,
                params.urgency.clone(),
            )
            .await?;

        let consent_request = ConsentRequestInternal {
            request_id: request_id.clone(),
            requester_id: params.requester_id,
            requester_display_name: params.requester_display_name,
            target_id: params.target_id,
            requested_scope: requested_scope_internal,
            justification: params.justification,
            urgency_level: params.urgency,
            created_at: now,
            expires_at,
            workflow_id: Some(workflow_id),
            response_deadline,
        };

        // Store the request
        {
            let mut requests = self.pending_requests.write().await;
            requests.insert(request_id.clone(), consent_request);
        }

        info!(
            "✅ Consent request created and workflow initiated: {}",
            request_id
        );
        Ok(request_id)
    }

    /// Grant consent to a request
    pub async fn grant_consent(
        &self,
        request_id: &str,
        grantor_id: &str,
        custom_conditions: Option<Vec<ConsentCondition>>,
        custom_expiration: Option<DateTime<Utc>>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        info!("✅ Granting consent for request: {}", request_id);

        // Find the consent request
        let request = {
            let requests = self.pending_requests.read().await;
            requests
                .get(request_id)
                .cloned()
                .ok_or("Consent request not found")?
        };

        // Check if request is still valid
        if Utc::now() > request.expires_at {
            return Err("Consent request has expired".into());
        }

        // Create consent record
        let consent_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = custom_expiration.or(Some(now + Duration::days(30))); // Default 30 day consent

        let consent_record = ConsentRecordInternal {
            consent_id: consent_id.clone(),
            grantor_id: grantor_id.to_string(),
            grantee_id: request.requester_id.clone(),
            consent_scope: request.requested_scope.clone(),
            granted_at: now,
            expires_at,
            status: ConsentStatus::Active,
            revocable: true,
            usage_count: 0,
            last_used: None,
            conditions: custom_conditions.unwrap_or_default(),
            audit_trail: vec![ConsentAuditEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: ConsentEventType::Granted,
                timestamp: now,
                actor_id: grantor_id.to_string(),
                details: "Consent granted".to_string(),
                metadata: HashMap::new(),
            }],
        };

        // Store the consent
        {
            let mut consents = self.active_consents.write().await;
            consents.insert(consent_id.clone(), consent_record);
        }

        // Remove the pending request
        {
            let mut requests = self.pending_requests.write().await;
            requests.remove(request_id);
        }

        // Notify the workflow system
        if let Some(workflow_id) = &request.workflow_id {
            self.notify_consent_decision(workflow_id, true).await?;
        }

        info!("🎉 Consent granted successfully: {}", consent_id);
        Ok(consent_id)
    }

    /// Revoke previously granted consent
    pub async fn revoke_consent(
        &self,
        consent_id: &str,
        revoker_id: &str,
        reason: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🚫 Revoking consent: {}", consent_id);

        let mut consents = self.active_consents.write().await;
        if let Some(consent) = consents.get_mut(consent_id) {
            // Verify the revoker has authority
            if consent.grantor_id != revoker_id {
                return Err("Only the grantor can revoke consent".into());
            }

            // Check if consent is revocable
            if !consent.revocable {
                return Err("This consent is not revocable".into());
            }

            // Update consent status
            consent.status = ConsentStatus::Revoked;

            // Add audit event
            consent.audit_trail.push(ConsentAuditEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: ConsentEventType::Revoked,
                timestamp: Utc::now(),
                actor_id: revoker_id.to_string(),
                details: reason.unwrap_or_else(|| "Consent revoked by grantor".to_string()),
                metadata: HashMap::new(),
            });

            info!("✅ Consent revoked successfully: {}", consent_id);
            Ok(())
        } else {
            Err("Consent record not found".into())
        }
    }

    /// Check if an action is permitted under existing consent
    pub async fn check_consent_permission(
        &self,
        grantee_id: &str,
        grantor_id: &str,
        action: &str,
        resource: &str,
        context: &HashMap<String, String>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "🔍 Checking consent permission for action: {} on resource: {}",
            action, resource
        );

        let consents = self.active_consents.read().await;

        // Find applicable consent
        for consent in consents.values() {
            if consent.grantor_id == grantor_id
                && consent.grantee_id == grantee_id
                && consent.status == ConsentStatus::Active
            {
                // Check if consent has expired
                if let Some(expires_at) = consent.expires_at {
                    if Utc::now() > expires_at {
                        continue;
                    }
                }

                // Check if action is permitted
                let action_permitted = consent
                    .consent_scope
                    .permitted_actions
                    .iter()
                    .any(|permitted| permitted.action == action);

                if !action_permitted {
                    continue;
                }

                // Check resource access
                let resource_permitted =
                    consent
                        .consent_scope
                        .resource_types
                        .iter()
                        .any(|resource_access| {
                            resource_access.resource_type == resource
                                || resource_access
                                    .specific_resources
                                    .as_ref()
                                    .map(|resources| resources.contains(&resource.to_string()))
                                    .unwrap_or(false)
                        });

                if !resource_permitted {
                    continue;
                }

                // Check conditions
                let conditions_met = self
                    .evaluate_consent_conditions(&consent.conditions, context)
                    .await?;
                if !conditions_met {
                    continue;
                }

                // Check usage limits
                let usage_allowed = self
                    .check_usage_limits(&consent.consent_scope.usage_limits, action)
                    .await?;
                if !usage_allowed {
                    continue;
                }

                debug!("✅ Consent permission granted for action: {}", action);
                return Ok(true);
            }
        }

        debug!("❌ No valid consent found for action: {}", action);
        Ok(false)
    }

    /// List active consents granted by the user
    pub async fn list_granted_consents(
        &self,
        grantor_id: &str,
    ) -> Result<Vec<ConsentRecord>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Listing consents granted by: {}", grantor_id);

        let consents = self.active_consents.read().await;
        let mut result = Vec::new();

        for consent in consents.values() {
            if consent.grantor_id == grantor_id && consent.status == ConsentStatus::Active {
                // Don't include expired consents
                if let Some(expires_at) = consent.expires_at {
                    if Utc::now() > expires_at {
                        continue;
                    }
                }

                result.push(ConsentRecord {
                    consent_id: consent.consent_id.clone(),
                    grantor_id: consent.grantor_id.clone(),
                    grantee_id: consent.grantee_id.clone(),
                    consent_scope: ConsentScope {
                        resource_types: Vec::new(), // Would be converted from internal format
                        permitted_actions: Vec::new(),
                        usage_limits: HashMap::new(),
                        privacy_level: consent.consent_scope.privacy_level.clone(),
                    },
                    granted_at: consent.granted_at.to_rfc3339(),
                    expires_at: consent.expires_at.map(|dt| dt.to_rfc3339()),
                    status: consent.status.clone(),
                    revocable: consent.revocable,
                });
            }
        }

        Ok(result)
    }

    /// List pending consent requests
    pub async fn list_pending_consent_requests(
        &self,
        target_id: &str,
    ) -> Result<Vec<ConsentRecord>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📥 Listing pending consent requests for: {}", target_id);

        let requests = self.pending_requests.read().await;
        let mut result = Vec::new();

        for request in requests.values() {
            if request.target_id == target_id && Utc::now() <= request.expires_at {
                result.push(ConsentRecord {
                    consent_id: request.request_id.clone(),
                    grantor_id: request.target_id.clone(),
                    grantee_id: request.requester_id.clone(),
                    consent_scope: ConsentScope {
                        resource_types: Vec::new(),
                        permitted_actions: Vec::new(),
                        usage_limits: HashMap::new(),
                        privacy_level: request.requested_scope.privacy_level.clone(),
                    },
                    granted_at: request.created_at.to_rfc3339(),
                    expires_at: Some(request.expires_at.to_rfc3339()),
                    status: ConsentStatus::Pending,
                    revocable: true,
                });
            }
        }

        Ok(result)
    }

    /// Record consent usage for audit trail
    pub async fn record_consent_usage(
        &self,
        consent_id: &str,
        action: &str,
        details: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!("📊 Recording consent usage: {}", consent_id);

        let mut consents = self.active_consents.write().await;
        if let Some(consent) = consents.get_mut(consent_id) {
            consent.usage_count += 1;
            consent.last_used = Some(Utc::now());

            // Add audit event
            consent.audit_trail.push(ConsentAuditEvent {
                event_id: Uuid::new_v4().to_string(),
                event_type: ConsentEventType::Used,
                timestamp: Utc::now(),
                actor_id: consent.grantee_id.clone(),
                details: format!("Action: {action} - {details}"),
                metadata: HashMap::new(),
            });
        }

        Ok(())
    }

    // Private helper methods

    /// Create consent workflow
    async fn create_consent_workflow(
        &self,
        request_id: &str,
        requester_id: &str,
        target_id: &str,
        justification: &str,
        urgency: UrgencyLevel,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        debug!("⚖️ Creating consent workflow for resource sharing");

        // Simplified workflow creation - just return a UUID for now
        let workflow_id = Uuid::new_v4().to_string();

        info!(
            "📋 Created consent workflow {} for request {}",
            workflow_id, request_id
        );
        info!(
            "   Requester: {}, Target: {}, Urgency: {:?}",
            requester_id, target_id, urgency
        );
        info!("   Justification: {}", justification);

        Ok(workflow_id)
    }

    /// Notify workflow system of consent decision  
    async fn notify_consent_decision(
        &self,
        workflow_id: &str,
        approved: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        debug!(
            "📤 Consent decision processed for workflow: {} (approved: {})",
            workflow_id, approved
        );
        Ok(())
    }

    /// Evaluate consent conditions
    async fn evaluate_consent_conditions(
        &self,
        conditions: &[ConsentCondition],
        context: &HashMap<String, String>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        for condition in conditions {
            let met = match condition.condition_type {
                ConditionType::TimeWindow => {
                    self.evaluate_time_window_condition(&condition.condition_data)
                        .await?
                }
                ConditionType::GeographicLocation => {
                    self.evaluate_geographic_condition(&condition.condition_data, context)
                        .await?
                }
                ConditionType::TrustScore => {
                    self.evaluate_trust_score_condition(&condition.condition_data, context)
                        .await?
                }
                _ => true, // Other conditions assumed to be met for now
            };

            if !met {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check usage limits
    async fn check_usage_limits(
        &self,
        limits: &HashMap<String, UsageLimit>,
        action: &str,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(limit) = limits.get(action) {
            return Ok(limit.current_usage < limit.limit_value);
        }
        Ok(true) // No limit defined
    }

    /// Evaluate time window condition
    async fn evaluate_time_window_condition(
        &self,
        _condition_data: &HashMap<String, String>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would check time-based conditions
        Ok(true)
    }

    /// Evaluate geographic condition
    async fn evaluate_geographic_condition(
        &self,
        _condition_data: &HashMap<String, String>,
        _context: &HashMap<String, String>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would check geographic conditions
        Ok(true)
    }

    /// Evaluate trust score condition
    async fn evaluate_trust_score_condition(
        &self,
        _condition_data: &HashMap<String, String>,
        _context: &HashMap<String, String>,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would check trust scores
        Ok(true)
    }

    /// Create default consent templates
    fn create_resource_sharing_template(&self) -> ConsentTemplate {
        ConsentTemplate {
            template_id: Uuid::new_v4().to_string(),
            template_name: "Resource Sharing".to_string(),
            template_type: TemplateType::ResourceSharing,
            default_scope: ConsentScopeInternal {
                resource_types: vec![ResourceTypeAccess {
                    resource_type: "compute".to_string(),
                    access_level: AccessLevel::Execute,
                    specific_resources: None,
                    exclusions: None,
                }],
                permitted_actions: vec![PermittedAction {
                    action: "use_compute".to_string(),
                    parameters: HashMap::new(),
                    frequency_limit: Some(FrequencyLimit::Day(10)),
                    requires_notification: false,
                }],
                usage_limits: HashMap::from([(
                    "compute_hours".to_string(),
                    UsageLimit {
                        limit_type: LimitType::Duration,
                        limit_value: 8, // 8 hours per day
                        time_window: Some(Duration::hours(24)),
                        current_usage: 0,
                        reset_schedule: Some("daily".to_string()),
                    },
                )]),
                privacy_level: PrivacyLevel::Anonymous,
                geographic_restrictions: None,
                temporal_restrictions: None,
            },
            description: "Standard template for peer-to-peer resource sharing".to_string(),
            created_at: Utc::now(),
            usage_count: 0,
        }
    }

    fn create_data_access_template(&self) -> ConsentTemplate {
        ConsentTemplate {
            template_id: Uuid::new_v4().to_string(),
            template_name: "Data Access".to_string(),
            template_type: TemplateType::DataAccess,
            default_scope: ConsentScopeInternal {
                resource_types: vec![ResourceTypeAccess {
                    resource_type: "user_data".to_string(),
                    access_level: AccessLevel::Read,
                    specific_resources: None,
                    exclusions: Some(vec!["sensitive".to_string(), "private".to_string()]),
                }],
                permitted_actions: vec![PermittedAction {
                    action: "read_data".to_string(),
                    parameters: HashMap::new(),
                    frequency_limit: Some(FrequencyLimit::Hour(5)),
                    requires_notification: true,
                }],
                usage_limits: HashMap::new(),
                privacy_level: PrivacyLevel::Anonymous,
                geographic_restrictions: None,
                temporal_restrictions: None,
            },
            description: "Template for limited data access with privacy protection".to_string(),
            created_at: Utc::now(),
            usage_count: 0,
        }
    }

    fn create_identity_verification_template(&self) -> ConsentTemplate {
        ConsentTemplate {
            template_id: Uuid::new_v4().to_string(),
            template_name: "Identity Verification".to_string(),
            template_type: TemplateType::IdentityVerification,
            default_scope: ConsentScopeInternal {
                resource_types: vec![ResourceTypeAccess {
                    resource_type: "identity".to_string(),
                    access_level: AccessLevel::Read,
                    specific_resources: Some(vec![
                        "public_key".to_string(),
                        "attestations".to_string(),
                    ]),
                    exclusions: Some(vec!["private_key".to_string(), "personal_data".to_string()]),
                }],
                permitted_actions: vec![PermittedAction {
                    action: "verify_identity".to_string(),
                    parameters: HashMap::new(),
                    frequency_limit: Some(FrequencyLimit::Day(3)),
                    requires_notification: true,
                }],
                usage_limits: HashMap::new(),
                privacy_level: PrivacyLevel::Anonymous,
                geographic_restrictions: None,
                temporal_restrictions: None,
            },
            description: "Template for identity verification with minimal data exposure"
                .to_string(),
            created_at: Utc::now(),
            usage_count: 0,
        }
    }

    fn create_emergency_template(&self) -> ConsentTemplate {
        ConsentTemplate {
            template_id: Uuid::new_v4().to_string(),
            template_name: "Emergency Access".to_string(),
            template_type: TemplateType::Emergency,
            default_scope: ConsentScopeInternal {
                resource_types: vec![ResourceTypeAccess {
                    resource_type: "emergency_data".to_string(),
                    access_level: AccessLevel::Read,
                    specific_resources: None,
                    exclusions: None,
                }],
                permitted_actions: vec![PermittedAction {
                    action: "emergency_access".to_string(),
                    parameters: HashMap::new(),
                    frequency_limit: None, // No limits in emergency
                    requires_notification: true,
                }],
                usage_limits: HashMap::new(),
                privacy_level: PrivacyLevel::Anonymous,
                geographic_restrictions: None,
                temporal_restrictions: Some(TemporalRestriction {
                    allowed_hours: None, // 24/7 access
                    allowed_days: None,  // All days
                    timezone: "UTC".to_string(),
                    blackout_periods: None,
                }),
            },
            description: "Emergency access template for critical situations".to_string(),
            created_at: Utc::now(),
            usage_count: 0,
        }
    }
}
