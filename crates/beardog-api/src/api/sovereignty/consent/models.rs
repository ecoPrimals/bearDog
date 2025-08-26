

use super::types::{
    ActionType, ConditionType, ConsentAuditEventType, ConsentStatus, DelegationType, PrivacyLevel,
    ResourceType, TemplateCategory, UrgencyLevel,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecordInternal {
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

pub struct ConsentRequestInternal {
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

pub struct ConsentScopeInternal {
    pub resource_types: Vec<ResourceTypeAccess>,
    pub permitted_actions: Vec<PermittedAction>,
    pub usage_limits: HashMap<String, UsageLimit>,
    pub privacy_level: PrivacyLevel,
    pub purpose: String,
    pub data_retention_period: Option<chrono::Duration>,
    pub third_party_sharing: bool,
    pub geographic_restrictions: Vec<String>,

pub struct ResourceTypeAccess {
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,
    pub specific_fields: Option<Vec<String>>,
    pub excluded_fields: Option<Vec<String>>,

pub struct PermittedAction {
    pub action_type: ActionType,
    pub constraints: Vec<ActionConstraint>,
    pub requires_confirmation: bool,

pub struct UsageLimit {
    pub limit_type: String,
    pub max_count: Option<u64>,
    pub time_window: Option<chrono::Duration>,
    pub current_usage: u64,
    pub reset_schedule: Option<String>,

pub struct ConsentCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub active: bool,

pub struct ConsentAuditEvent {
    pub event_id: String,
    pub event_type: ConsentAuditEventType,
    pub timestamp: DateTime<Utc>,
    pub actor_id: String,
    pub details: String,
    pub metadata: HashMap<String, String>,

pub struct ConsentTemplate {
    pub template_id: String,
    pub name: String,
    pub category: TemplateCategory,
    pub default_scope: ConsentScopeInternal,
    pub default_conditions: Vec<ConsentCondition>,
    pub customizable_fields: Vec<String>,
    pub version: String,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessLevel {
    None,
    ReadOnly,
    Limited,
    Full,
    Administrative,

pub struct ActionConstraint {
    pub constraint_type: String,
    pub enforced: bool,

pub struct ConsentDelegation {
    pub delegation_id: String,
    pub delegator_id: String,
    pub delegate_id: String,
    pub delegation_type: DelegationType,
    pub scope: ConsentScopeInternal,
