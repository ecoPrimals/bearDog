// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Consent Management Models
///
/// **EXTRACTED FROM LARGE FILE** - Internal data structures (~120 lines)
/// This module contains all the internal data structures and models used
/// by the consent management system.

use super::types::{
    ActionType, ConditionType, ConsentAuditEventType, ConsentStatus, DelegationType, PrivacyLevel,
    ResourceType, TemplateCategory, UrgencyLevel,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Internal consent record data structure
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
/// Internal consent request data structure
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
/// Internal consent scope definition
pub struct ConsentScopeInternal {
    pub resource_types: Vec<ResourceTypeAccess>,
    pub permitted_actions: Vec<PermittedAction>,
    pub usage_limits: HashMap<String, UsageLimit>,
    pub privacy_level: PrivacyLevel,
    pub purpose: String,
    pub data_retention_period: Option<chrono::Duration>,
    pub third_party_sharing: bool,
    pub geographic_restrictions: Vec<String>,
/// Resource type with specific access permissions
pub struct ResourceTypeAccess {
    pub resource_type: ResourceType,
    pub access_level: AccessLevel,
    pub specific_fields: Option<Vec<String>>,
    pub excluded_fields: Option<Vec<String>>,
/// Permitted action with constraints
pub struct PermittedAction {
    pub action_type: ActionType,
    pub constraints: Vec<ActionConstraint>,
    pub requires_confirmation: bool,
/// Usage limits for consent
pub struct UsageLimit {
    pub limit_type: String,
    pub max_count: Option<u64>,
    pub time_window: Option<chrono::Duration>,
    pub current_usage: u64,
    pub reset_schedule: Option<String>,
/// Consent condition
pub struct ConsentCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub active: bool,
/// Consent audit event
pub struct ConsentAuditEvent {
    pub event_id: String,
    pub event_type: ConsentAuditEventType,
    pub timestamp: DateTime<Utc>,
    pub actor_id: String,
    pub details: String,
    pub metadata: HashMap<String, String>,
/// Consent template structure
pub struct ConsentTemplate {
    pub template_id: String,
    pub name: String,
    pub category: TemplateCategory,
    pub default_scope: ConsentScopeInternal,
    pub default_conditions: Vec<ConsentCondition>,
    pub customizable_fields: Vec<String>,
    pub version: String,
/// Access level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessLevel {
    None,
    ReadOnly,
    Limited,
    Full,
    Administrative,
/// Action constraints}


pub struct ActionConstraint {
    pub constraint_type: String,
    pub enforced: bool,
/// Consent delegation record
pub struct ConsentDelegation {
    pub delegation_id: String,
    pub delegator_id: String,
    pub delegate_id: String,
    pub delegation_type: DelegationType,
    pub scope: ConsentScopeInternal,
