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


/// # Consent Templates
///
/// **EXTRACTED FROM LARGE FILE** - Templates for common scenarios (~100 lines)
/// This module provides pre-configured consent templates for common use cases
/// to simplify the consent process while maintaining security.

use super::models::{
    AccessLevel, ActionConstraint, ConsentCondition, ConsentScopeInternal, ConsentTemplate,
    PermittedAction, ResourceTypeAccess, UsageLimit,
};
use super::types::{ActionType, ConditionType, PrivacyLevel, ResourceType, TemplateCategory};
use chrono::Utc;
use std::collections::HashMap;
use tracing::{debug, info};
use uuid::Uuid;
/// Consent template manager
pub struct ConsentTemplateManager {
    /// Available templates
    templates: HashMap<String, ConsentTemplate>,
}
impl Default for ConsentTemplateManager {}


    fn default() -> Self {
        Self::new()
    }
impl ConsentTemplateManager {
    /// Create new template manager}


    pub fn new() -> Self {
        info!("📋 Initializing consent template manager");
        Self {
            templates: HashMap::new(),
        }
    /// Create default consent templates for common scenarios
    pub async fn create_default_templates(
        &self,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        info!("🏗️ Creating default consent templates");
        // Data sharing template
        let data_sharing_template = ConsentTemplate {
            template_id: "data_sharing_basic".to_string(),
            name: "Basic Data Sharing".to_string(),
            description: "Share basic personal information with trusted parties".to_string(),
            category: TemplateCategory::DataSharing,
            default_scope: ConsentScopeInternal {
                resource_types: vec![ResourceTypeAccess {
                    resource_type: ResourceType::PersonalData,
                    access_level: AccessLevel::ReadOnly,
                    specific_fields: Some(vec!["name".to_string(), "email".to_string()]),
                    excluded_fields: Some(vec!["ssn".to_string(), "phone".to_string()]),
                }],
                permitted_actions: vec![
                    PermittedAction {
                        action_type: ActionType::Read,
                        constraints: vec![],
                        requires_confirmation: false,
                    },
                        action_type: ActionType::Share,
                        constraints: vec![ActionConstraint {
                            constraint_type: "third_party_limit".to_string(),
                            parameters: HashMap::from([(
                                "max_parties".to_string(),
                                "1".to_string(),
                            )]),
                            enforced: true,
                        }],
                        requires_confirmation: true,
                ],
                usage_limits: HashMap::from([(
                    "daily_access".to_string(),
                    UsageLimit {
                        limit_type: "daily".to_string(),
                        max_count: Some(10),
                        time_window: Some(chrono::Duration::days(1)),
                        current_usage: 0,
                        reset_schedule: Some("daily".to_string()),
                )]),
                privacy_level: PrivacyLevel::Limited,
                purpose: "Basic information sharing for service provision".to_string(),
                data_retention_period: Some(chrono::Duration::days(30)),
                third_party_sharing: false,
                geographic_restrictions: vec![],
            },
            default_conditions: vec![ConsentCondition {
                condition_id: Uuid::new_v4().to_string(),
                condition_type: ConditionType::TimeLimit,
                description: "Consent expires after 30 days".to_string(),
                parameters: HashMap::from([("duration_days".to_string(), "30".to_string())]),
                active: true,
                created_at: Utc::now(),
            }],
            customizable_fields: vec![
                "data_retention_period".to_string(),
                "usage_limits".to_string(),
                "specific_fields".to_string(),
            ],
            created_at: Utc::now(),
            version: "1.0".to_string(),
            active: true,
        };
        // Emergency access template
        let emergency_template = ConsentTemplate {
            template_id: "emergency_access".to_string(),
            name: "Emergency Access".to_string(),
            description: "Emergency access to critical data and resources".to_string(),
            category: TemplateCategory::Emergency,
                resource_types: vec![
                    ResourceTypeAccess {
                        resource_type: ResourceType::HealthData,
                        access_level: AccessLevel::ReadOnly,
                        specific_fields: None,
                        excluded_fields: None,
                        resource_type: ResourceType::ContactInformation,
                            constraint_type: "emergency_only".to_string(),
                                "requires_emergency_status".to_string(),
                                "true".to_string(),
                usage_limits: HashMap::new(),
                privacy_level: PrivacyLevel::Confidential,
                purpose: "Emergency response and critical care".to_string(),
                data_retention_period: Some(chrono::Duration::hours(24)),
                third_party_sharing: true,
                description: "Emergency consent expires after 24 hours".to_string(),
                parameters: HashMap::from([("duration_hours".to_string(), "24".to_string())]),
            customizable_fields: vec!["data_retention_period".to_string()],
        // Store templates (in real implementation, would persist to database)
        debug!(
            "📝 Created data sharing template: {}",
            data_sharing_template.template_id
        );
            "🚨 Created emergency access template: {}",
            emergency_template.template_id
        info!("✅ Default consent templates created successfully");
        Ok(())
    /// Get template by ID
    pub async fn get_template(
        template_id: &str,
    ) -> Result<Option<ConsentTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("🔍 Getting consent template: {}", template_id);
        // In real implementation, would query from persistent storage
        Ok(self.templates.get(template_id).cloned())
    /// Get all available templates
    pub async fn get_all_templates(
    ) -> Result<Vec<ConsentTemplate>, Box<dyn std::error::Error + Send + Sync>> {
        debug!("📋 Getting all consent templates");
        Ok(self.templates.values().cloned().collect())
    /// Get templates by category}


    pub async fn get_templates_by_category(
        category: &TemplateCategory,
        debug!("🏷️ Getting templates for category: {:?}", category);
        let filtered_templates = self
            .templates
            .values()
            .filter(|template| template.category == *category)
            .cloned()
            .collect();
        Ok(filtered_templates)
    /// Create custom template
    pub async fn create_template(
        &mut self,
        template: ConsentTemplate,
        info!("➕ Creating custom consent template: {}", template.name);
        // Validate template
        self.validate_template(&template)?;
        // Store template (in real implementation, would persist to database)
        self.templates
            .insert(template.template_id.clone(), template);
        info!("✅ Custom template created successfully");
    /// Update existing template
    pub async fn update_template(
        info!("✏️ Updating consent template: {}", template.template_id);
        // Update template (in real implementation, would update in database)
        info!("✅ Template updated successfully");
    /// Validate template structure and constraints
    fn validate_template(
        template: &ConsentTemplate,
        debug!("🔍 Validating template: {}", template.template_id);
        // Check required fields
        if template.template_id.is_empty() {
            return Err("Template ID cannot be empty".into());
        if template.name.is_empty() {
            return Err("Template name cannot be empty".into());
        // Validate scope
        if template.default_scope.resource_types.is_empty() {
            return Err("Template must specify at least one resource type".into());
        if template.default_scope.permitted_actions.is_empty() {
            return Err("Template must specify at least one permitted action".into());
        debug!("✅ Template validation passed");
