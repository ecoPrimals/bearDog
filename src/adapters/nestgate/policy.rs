//! Universal Policy Engine for NestGate
//!
//! Policy engine implementation for access control, rule evaluation, and policy management
//! that can be used by any ecosystem component.

use chrono::{Datelike, Timelike};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

use super::types::*;

/// Universal policy engine
pub struct PolicyEngine {
    /// Policy configuration
    config: PolicyConfig,
    /// Active policies
    policies: Arc<RwLock<HashMap<String, AccessPolicy>>>,
    /// Policy evaluation cache
    evaluation_cache: Arc<RwLock<HashMap<String, PolicyEvaluationResult>>>,
    /// Policy statistics
    statistics: Arc<RwLock<PolicyStatistics>>,
}

/// Policy evaluation result (cached)
#[derive(Debug, Clone)]
pub struct PolicyEvaluationResult {
    /// Evaluation result
    pub result: PolicyCheckResult,
    /// Evaluation timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Cache expiry
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Policy engine statistics
#[derive(Debug, Default)]
pub struct PolicyStatistics {
    /// Total policy evaluations
    pub total_evaluations: u64,
    /// Policy decisions by result
    pub decisions: HashMap<String, u64>,
    /// Policy usage by ID
    pub policy_usage: HashMap<String, u64>,
    /// Average evaluation time in microseconds
    pub avg_evaluation_time: u64,
}

impl PolicyEngine {
    /// Create new policy engine
    pub async fn new(config: PolicyConfig) -> NestGateResult<Self> {
        info!(
            "Creating policy engine with {} enabled policies",
            config.enabled_policies.len()
        );

        let engine = Self {
            config,
            policies: Arc::new(RwLock::new(HashMap::new())),
            evaluation_cache: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(PolicyStatistics::default())),
        };

        // Load default policies
        engine.load_default_policies().await?;

        info!("Policy engine created successfully");
        Ok(engine)
    }

    /// Load default policies
    async fn load_default_policies(&self) -> NestGateResult<()> {
        debug!("Loading default policies");

        let default_policies = vec![
            self.create_default_read_write_policy().await?,
            self.create_system_protection_policy().await?,
            self.create_admin_policy().await?,
            self.create_time_based_policy().await?,
        ];

        let mut policies = self.policies.write().await;
        for policy in default_policies {
            policies.insert(policy.id.clone(), policy);
        }

        info!("Default policies loaded successfully");
        Ok(())
    }

    /// Create default read-write policy
    async fn create_default_read_write_policy(&self) -> NestGateResult<AccessPolicy> {
        Ok(AccessPolicy {
            id: "default_read_write".to_string(),
            name: "Default Read-Write Access".to_string(),
            description: "Default policy allowing read-write access to user data".to_string(),
            rules: vec![PolicyRule {
                id: "allow_user_data".to_string(),
                subject: "*".to_string(),
                resource: "/data/*".to_string(),
                operations: vec![
                    FileOperation::Read,
                    FileOperation::Write,
                    FileOperation::Copy,
                    FileOperation::CreateDirectory,
                    FileOperation::ListDirectory,
                    FileOperation::GetAttributes,
                ],
                access_level: AccessLevel::ReadWrite,
                time_restrictions: None,
                conditions: vec![],
            }],
            enabled: true,
            priority: 100,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        })
    }

    /// Create system protection policy
    async fn create_system_protection_policy(&self) -> NestGateResult<AccessPolicy> {
        Ok(AccessPolicy {
            id: "protect_system_files".to_string(),
            name: "Protect System Files".to_string(),
            description: "Policy protecting system files and directories".to_string(),
            rules: vec![
                PolicyRule {
                    id: "deny_system_access".to_string(),
                    subject: "*".to_string(),
                    resource: "/etc/*".to_string(),
                    operations: vec![
                        FileOperation::Write,
                        FileOperation::Delete,
                        FileOperation::Move,
                    ],
                    access_level: AccessLevel::None,
                    time_restrictions: None,
                    conditions: vec![],
                },
                PolicyRule {
                    id: "deny_root_access".to_string(),
                    subject: "*".to_string(),
                    resource: "/root/*".to_string(),
                    operations: vec![
                        FileOperation::Read,
                        FileOperation::Write,
                        FileOperation::Delete,
                        FileOperation::Move,
                        FileOperation::Copy,
                    ],
                    access_level: AccessLevel::None,
                    time_restrictions: None,
                    conditions: vec![],
                },
            ],
            enabled: true,
            priority: 200,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        })
    }

    /// Create admin policy
    async fn create_admin_policy(&self) -> NestGateResult<AccessPolicy> {
        Ok(AccessPolicy {
            id: "admin_access".to_string(),
            name: "Administrator Access".to_string(),
            description: "Policy granting full access to administrators".to_string(),
            rules: vec![PolicyRule {
                id: "admin_full_access".to_string(),
                subject: "admin:*".to_string(),
                resource: "*".to_string(),
                operations: vec![
                    FileOperation::Read,
                    FileOperation::Write,
                    FileOperation::Copy,
                    FileOperation::Move,
                    FileOperation::Delete,
                    FileOperation::CreateDirectory,
                    FileOperation::ListDirectory,
                    FileOperation::Compress,
                    FileOperation::Decompress,
                    FileOperation::Encrypt,
                    FileOperation::Decrypt,
                    FileOperation::GetAttributes,
                    FileOperation::SetAttributes,
                ],
                access_level: AccessLevel::Admin,
                time_restrictions: None,
                conditions: vec![],
            }],
            enabled: true,
            priority: 300,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        })
    }

    /// Create time-based policy
    async fn create_time_based_policy(&self) -> NestGateResult<AccessPolicy> {
        Ok(AccessPolicy {
            id: "business_hours_access".to_string(),
            name: "Business Hours Access".to_string(),
            description: "Policy restricting access to business hours".to_string(),
            rules: vec![PolicyRule {
                id: "business_hours_only".to_string(),
                subject: "employee:*".to_string(),
                resource: "/business/*".to_string(),
                operations: vec![
                    FileOperation::Read,
                    FileOperation::Write,
                    FileOperation::Copy,
                ],
                access_level: AccessLevel::ReadWrite,
                time_restrictions: Some(TimeRestriction {
                    start_hour: 9,
                    end_hour: 17,
                    allowed_days: vec![1, 2, 3, 4, 5], // Monday to Friday
                    timezone: "UTC".to_string(),
                }),
                conditions: vec![],
            }],
            enabled: true,
            priority: 150,
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
        })
    }

    /// Check access permissions
    pub async fn check_access(
        &self,
        user_id: &str,
        resource: &str,
        operation: &FileOperation,
    ) -> NestGateResult<PolicyCheckResult> {
        let start_time = std::time::Instant::now();

        debug!(
            "Checking access for user: {}, resource: {}, operation: {:?}",
            user_id, resource, operation
        );

        // Check evaluation cache first
        let cache_key = format!("{user_id}:{resource}:{operation:?}");
        if let Some(cached_result) = self.get_cached_evaluation(&cache_key).await? {
            debug!("Using cached policy evaluation result");
            return Ok(cached_result.result);
        }

        // Get enabled policies sorted by priority
        let policies = self.get_enabled_policies_by_priority().await?;

        // Evaluate policies
        let mut final_result = PolicyCheckResult {
            allowed: false,
            reason: "No matching policy found, default deny".to_string(),
            access_level: AccessLevel::None,
            policy_id: None,
            rule_id: None,
        };

        for policy in policies {
            if let Some(rule_result) = self
                .evaluate_policy(&policy, user_id, resource, operation)
                .await?
            {
                final_result = rule_result;
                break; // First matching policy wins (highest priority)
            }
        }

        // Cache the result
        self.cache_evaluation_result(&cache_key, &final_result)
            .await?;

        // Update statistics
        self.update_statistics(&final_result, start_time.elapsed())
            .await?;

        debug!(
            "Access check completed: allowed={}, reason={}",
            final_result.allowed, final_result.reason
        );
        Ok(final_result)
    }

    /// Get cached evaluation result
    async fn get_cached_evaluation(
        &self,
        cache_key: &str,
    ) -> NestGateResult<Option<PolicyEvaluationResult>> {
        let cache = self.evaluation_cache.read().await;
        if let Some(cached) = cache.get(cache_key) {
            if chrono::Utc::now() < cached.expires_at {
                return Ok(Some(cached.clone()));
            }
        }
        Ok(None)
    }

    /// Cache evaluation result
    async fn cache_evaluation_result(
        &self,
        cache_key: &str,
        result: &PolicyCheckResult,
    ) -> NestGateResult<()> {
        let cached_result = PolicyEvaluationResult {
            result: result.clone(),
            timestamp: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(300), // 5 minutes cache
        };

        self.evaluation_cache
            .write()
            .await
            .insert(cache_key.to_string(), cached_result);
        Ok(())
    }

    /// Get enabled policies sorted by priority
    async fn get_enabled_policies_by_priority(&self) -> NestGateResult<Vec<AccessPolicy>> {
        let policies = self.policies.read().await;
        let mut enabled_policies: Vec<AccessPolicy> =
            policies.values().filter(|p| p.enabled).cloned().collect();

        // Sort by priority (higher priority first)
        enabled_policies.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(enabled_policies)
    }

    /// Evaluate policy against request
    async fn evaluate_policy(
        &self,
        policy: &AccessPolicy,
        user_id: &str,
        resource: &str,
        operation: &FileOperation,
    ) -> NestGateResult<Option<PolicyCheckResult>> {
        for rule in &policy.rules {
            if self
                .evaluate_rule(rule, user_id, resource, operation)
                .await?
            {
                return Ok(Some(PolicyCheckResult {
                    allowed: rule.access_level != AccessLevel::None,
                    reason: if rule.access_level != AccessLevel::None {
                        format!("Allowed by policy: {} (rule: {})", policy.name, rule.id)
                    } else {
                        format!("Denied by policy: {} (rule: {})", policy.name, rule.id)
                    },
                    access_level: rule.access_level.clone(),
                    policy_id: Some(policy.id.clone()),
                    rule_id: Some(rule.id.clone()),
                }));
            }
        }
        Ok(None)
    }

    /// Evaluate rule against request
    async fn evaluate_rule(
        &self,
        rule: &PolicyRule,
        user_id: &str,
        resource: &str,
        operation: &FileOperation,
    ) -> NestGateResult<bool> {
        // Check subject pattern
        if !self.matches_pattern(&rule.subject, user_id) {
            return Ok(false);
        }

        // Check resource pattern
        if !self.matches_pattern(&rule.resource, resource) {
            return Ok(false);
        }

        // Check operation
        if !rule.operations.contains(operation) {
            return Ok(false);
        }

        // Check time restrictions
        if let Some(time_restriction) = &rule.time_restrictions {
            if !self.check_time_restriction(time_restriction).await? {
                return Ok(false);
            }
        }

        // Check conditions
        for condition in &rule.conditions {
            if !self
                .evaluate_condition(condition, user_id, resource)
                .await?
            {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Check if pattern matches target
    fn matches_pattern(&self, pattern: &str, target: &str) -> bool {
        if pattern == "*" {
            return true;
        }

        if let Some(prefix) = pattern.strip_suffix('*') {
            target.starts_with(prefix)
        } else {
            pattern == target
        }
    }

    /// Check time-based restrictions
    async fn check_time_restriction(&self, restriction: &TimeRestriction) -> NestGateResult<bool> {
        let now = chrono::Utc::now();
        let hour = now.hour() as u8;
        let weekday = now.weekday().num_days_from_monday() as u8;

        // Check time window
        if hour < restriction.start_hour || hour > restriction.end_hour {
            return Ok(false);
        }

        // Check allowed days
        if !restriction.allowed_days.contains(&weekday) {
            return Ok(false);
        }

        Ok(true)
    }

    /// Evaluate policy condition
    async fn evaluate_condition(
        &self,
        condition: &PolicyCondition,
        user_id: &str,
        resource: &str,
    ) -> NestGateResult<bool> {
        match &condition.condition_type {
            ConditionType::IpAddress => {
                // Simulate IP address check
                Ok(true)
            }
            ConditionType::UserAgent => {
                // Simulate user agent check
                Ok(true)
            }
            ConditionType::FileSize => {
                // Simulate file size check
                Ok(true)
            }
            ConditionType::FileType => {
                // Simulate file type check
                Ok(true)
            }
            ConditionType::Custom(custom_type) => {
                // Handle custom condition types
                match custom_type.as_str() {
                    "user_group" => Ok(user_id.contains("admin")),
                    "resource_type" => Ok(resource.contains("secure")),
                    _ => Ok(false),
                }
            }
        }
    }

    /// Update statistics
    async fn update_statistics(
        &self,
        result: &PolicyCheckResult,
        evaluation_time: std::time::Duration,
    ) -> NestGateResult<()> {
        let mut stats = self.statistics.write().await;

        stats.total_evaluations += 1;

        // Update decision counts
        let decision_key = if result.allowed { "allowed" } else { "denied" };
        *stats.decisions.entry(decision_key.to_string()).or_insert(0) += 1;

        // Update policy usage
        if let Some(policy_id) = &result.policy_id {
            *stats.policy_usage.entry(policy_id.clone()).or_insert(0) += 1;
        }

        // Update average evaluation time
        let eval_time_micros = evaluation_time.as_micros() as u64;
        stats.avg_evaluation_time = if stats.total_evaluations == 1 {
            eval_time_micros
        } else {
            ((stats.avg_evaluation_time * (stats.total_evaluations - 1)) + eval_time_micros)
                / stats.total_evaluations
        };

        Ok(())
    }

    /// Get all policies
    pub async fn get_all_policies(&self) -> NestGateResult<Vec<AccessPolicy>> {
        let policies = self.policies.read().await;
        Ok(policies.values().cloned().collect())
    }

    /// Add policy
    pub async fn add_policy(&self, policy: AccessPolicy) -> NestGateResult<()> {
        info!("Adding policy: {} ({})", policy.name, policy.id);

        // Clear cache when policies change
        self.evaluation_cache.write().await.clear();

        self.policies
            .write()
            .await
            .insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Remove policy
    pub async fn remove_policy(&self, policy_id: &str) -> NestGateResult<bool> {
        info!("Removing policy: {}", policy_id);

        // Clear cache when policies change
        self.evaluation_cache.write().await.clear();

        Ok(self.policies.write().await.remove(policy_id).is_some())
    }

    /// Update policy
    pub async fn update_policy(&self, policy: AccessPolicy) -> NestGateResult<()> {
        info!("Updating policy: {} ({})", policy.name, policy.id);

        // Clear cache when policies change
        self.evaluation_cache.write().await.clear();

        self.policies
            .write()
            .await
            .insert(policy.id.clone(), policy);
        Ok(())
    }

    /// Enable policy
    pub async fn enable_policy(&self, policy_id: &str) -> NestGateResult<bool> {
        info!("Enabling policy: {}", policy_id);

        // Clear cache when policies change
        self.evaluation_cache.write().await.clear();

        if let Some(policy) = self.policies.write().await.get_mut(policy_id) {
            policy.enabled = true;
            policy.modified_at = chrono::Utc::now();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Disable policy
    pub async fn disable_policy(&self, policy_id: &str) -> NestGateResult<bool> {
        info!("Disabling policy: {}", policy_id);

        // Clear cache when policies change
        self.evaluation_cache.write().await.clear();

        if let Some(policy) = self.policies.write().await.get_mut(policy_id) {
            policy.enabled = false;
            policy.modified_at = chrono::Utc::now();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get policy statistics
    pub async fn get_statistics(&self) -> NestGateResult<PolicyStatistics> {
        let stats = self.statistics.read().await;
        Ok(PolicyStatistics {
            total_evaluations: stats.total_evaluations,
            decisions: stats.decisions.clone(),
            policy_usage: stats.policy_usage.clone(),
            avg_evaluation_time: stats.avg_evaluation_time,
        })
    }

    /// Clear evaluation cache
    pub async fn clear_cache(&self) -> NestGateResult<()> {
        info!("Clearing policy evaluation cache");
        self.evaluation_cache.write().await.clear();
        Ok(())
    }

    /// Health check
    pub async fn health_check(&self) -> NestGateResult<HealthStatus> {
        debug!("Performing policy engine health check");

        let policies = self.policies.read().await;
        let enabled_count = policies.values().filter(|p| p.enabled).count();

        let healthy = !policies.is_empty() && enabled_count > 0;
        let message = if healthy {
            format!(
                "Policy engine healthy with {enabled_count} enabled policies"
            )
        } else {
            "Policy engine unhealthy - no enabled policies".to_string()
        };

        Ok(HealthStatus {
            healthy,
            message,
            components: HashMap::new(),
            last_check: chrono::Utc::now(),
        })
    }

    /// Cleanup expired cache entries
    pub async fn cleanup_cache(&self) -> NestGateResult<()> {
        let mut cache = self.evaluation_cache.write().await;
        let now = chrono::Utc::now();

        cache.retain(|_, entry| entry.expires_at > now);

        debug!("Cleaned up expired cache entries");
        Ok(())
    }
}
