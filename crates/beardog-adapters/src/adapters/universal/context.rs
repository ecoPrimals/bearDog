//! Security Context Management
//!
//! **Context-based routing and security management for ecosystem components**

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use super::traits::SecurityContext;
use crate::BearDogResult;
use beardog_errors::BearDogError;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};

/// Security Context Manager
/// 
/// Provides comprehensive context-based routing and security management for ecosystem components.
/// This system enables adaptive security policies based on user identity, device characteristics,
/// network conditions, and threat intelligence.
pub struct SecurityContextManager {
    /// Active security contexts indexed by context ID
    contexts: Arc<RwLock<HashMap<String, ActiveSecurityContext>>>,
    /// Security policies for different context types
    policies: Arc<RwLock<HashMap<String, SecurityPolicy>>>,
    /// Context validation rules
    validation_rules: Arc<RwLock<Vec<ContextValidationRule>>>,
    /// Maximum context lifetime
    max_context_lifetime: Duration,
    /// Context cleanup interval
    cleanup_interval: Duration,
}

/// Active security context with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveSecurityContext {
    context: SecurityContext,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    access_count: u64,
    risk_score: f64,
    validation_status: ContextValidationStatus,
}

/// Security policy for context types
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityPolicy {
    policy_id: String,
    context_type: String,
    required_clearance_level: u8,
    allowed_operations: Vec<String>,
    denied_operations: Vec<String>,
    risk_threshold: f64,
    requires_mfa: bool,
    max_session_duration: Duration,
    network_restrictions: Vec<String>,
}

/// Context validation rule
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ContextValidationRule {
    rule_id: String,
    rule_type: ValidationRuleType,
    condition: String,
    action: ValidationAction,
    severity: ValidationSeverity,
}

/// Types of validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValidationRuleType {
    DeviceFingerprint,
    NetworkLocation,
    UserBehavior,
    ThreatIntelligence,
    TimeBasedAccess,
    GeolocationCheck,
}

/// Action to take when validation rule triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValidationAction {
    Allow,
    Deny,
    RequireAdditionalAuth,
    LogAndContinue,
    EscalateToAdmin,
}

/// Severity of validation failures
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ValidationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Status of context validation
#[derive(Debug, Clone, Serialize, Deserialize)]
enum ContextValidationStatus {
    Valid,
    PendingValidation,
    RequiresRevalidation,
    Invalid(String),
}

impl SecurityContextManager {
    pub async fn new() -> BearDogResult<Self> {
        info!("🛡️ Initializing Security Context Manager");
        
        let manager = Self {
            contexts: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(HashMap::new())),
            validation_rules: Arc::new(RwLock::new(Vec::new())),
            max_context_lifetime: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        };
        
        // Initialize default security policies
        manager.initialize_default_policies().await?;
        
        // Initialize default validation rules
        manager.initialize_default_validation_rules().await?;
        
        // Start context cleanup task
        manager.start_cleanup_task().await;
        
        info!("✅ Security Context Manager initialized successfully");
        Ok(manager)
    }
    
    /// Create a new security context for user and device
    pub async fn create_context(&self, user_id: &str, device_id: &str) -> BearDogResult<SecurityContext> {
        debug!("🔐 Creating security context for user: {}, device: {}", user_id, device_id);
        
        // Generate unique context ID
        let context_id = format!("ctx_{}_{}_{}", user_id, device_id, uuid::Uuid::new_v4());
        
        // Analyze device and user characteristics
        let device_fingerprint = self.generate_device_fingerprint(device_id).await?;
        let user_risk_profile = self.assess_user_risk(user_id).await?;
        
        // Create base security context
        let mut context = SecurityContext::default();
        context.context_id = context_id.clone();
        context.user_id = user_id.to_string();
        context.device_id = device_id.to_string();
        context.created_at = Utc::now().to_rfc3339();
        context.security_clearance_level = self.determine_clearance_level(user_id, &user_risk_profile).await?;
        context.risk_score = user_risk_profile.base_risk_score;
        context.device_trust_level = device_fingerprint.trust_level;
        context.network_zone = self.determine_network_zone().await?;
        
        // Apply context-specific policies
        let applicable_policies = self.get_applicable_policies(&context).await?;
        context.allowed_operations = applicable_policies.iter()
            .flat_map(|p| p.allowed_operations.clone())
            .collect();
        context.denied_operations = applicable_policies.iter()
            .flat_map(|p| p.denied_operations.clone())
            .collect();
        
        // Create active context record
        let active_context = ActiveSecurityContext {
            context: context.clone(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            risk_score: user_risk_profile.base_risk_score,
            validation_status: ContextValidationStatus::Valid,
        };
        
        // Store active context
        {
            let mut contexts = self.contexts.write().await;
            contexts.insert(context_id.clone(), active_context);
        }
        
        info!("✅ Created security context: {} for user: {}", context_id, user_id);
        Ok(context)
    }
    
    /// Validate security context against current policies and rules
    pub async fn validate_context(&self, context: &SecurityContext) -> BearDogResult<bool> {
        debug!("🔍 Validating security context: {}", context.context_id);
        
        // Check if context exists and is still valid
        let mut active_context = {
            let mut contexts = self.contexts.write().await;
            match contexts.get_mut(&context.context_id) {
                Some(ctx) => {
                    ctx.last_accessed = Utc::now();
                    ctx.access_count += 1;
                    ctx.clone()
                },
                None => {
                    warn!("Context not found: {}", context.context_id);
                    return Ok(false);
                }
            }
        };
        
        // Check context expiration
        let now = Utc::now();
        let age = now.signed_duration_since(active_context.created_at);
        if age > chrono::Duration::from_std(self.max_context_lifetime)? {
            warn!("Context expired: {}", context.context_id);
            self.invalidate_context(&context.context_id).await?;
            return Ok(false);
        }
        
        // Run validation rules
        let validation_rules = self.validation_rules.read().await;
        for rule in validation_rules.iter() {
            match self.evaluate_validation_rule(rule, context, &active_context).await? {
                ValidationAction::Deny => {
                    warn!("Context validation denied by rule: {}", rule.rule_id);
                    return Ok(false);
                },
                ValidationAction::RequireAdditionalAuth => {
                    info!("Context requires additional authentication: {}", context.context_id);
                    // Mark for revalidation
                    active_context.validation_status = ContextValidationStatus::RequiresRevalidation;
                },
                ValidationAction::EscalateToAdmin => {
                    warn!("Context validation escalated to admin: {}", context.context_id);
                    // Log security event for admin review
                },
                ValidationAction::LogAndContinue => {
                    debug!("Context validation logged: rule {}", rule.rule_id);
                },
                ValidationAction::Allow => {
                    // Continue validation
                }
            }
        }
        
        // Update active context with validation results
        {
            let mut contexts = self.contexts.write().await;
            if let Some(ctx) = contexts.get_mut(&context.context_id) {
                ctx.validation_status = active_context.validation_status;
                ctx.risk_score = active_context.risk_score;
            }
        }
        
        debug!("✅ Context validation completed for: {}", context.context_id);
        Ok(true)
    }
    
    /// Get security context by ID
    pub async fn get_context(&self, context_id: &str) -> BearDogResult<Option<SecurityContext>> {
        let contexts = self.contexts.read().await;
        Ok(contexts.get(context_id).map(|ctx| ctx.context.clone()))
    }
    
    /// Invalidate security context
    pub async fn invalidate_context(&self, context_id: &str) -> BearDogResult<()> {
        info!("🚫 Invalidating security context: {}", context_id);
        let mut contexts = self.contexts.write().await;
        contexts.remove(context_id);
        Ok(())
    }
    
    /// Initialize default security policies
    async fn initialize_default_policies(&self) -> BearDogResult<()> {
        let mut policies = self.policies.write().await;
        
        // High security policy
        policies.insert("high_security".to_string(), SecurityPolicy {
            policy_id: "high_security".to_string(),
            context_type: "administrative".to_string(),
            required_clearance_level: 8,
            allowed_operations: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
            denied_operations: vec!["bulk_delete".to_string()],
            risk_threshold: 0.2,
            requires_mfa: true,
            max_session_duration: Duration::from_secs(1800), // 30 minutes
            network_restrictions: vec!["internal".to_string()],
        });
        
        // Standard security policy
        policies.insert("standard_security".to_string(), SecurityPolicy {
            policy_id: "standard_security".to_string(),
            context_type: "user".to_string(),
            required_clearance_level: 5,
            allowed_operations: vec!["read".to_string(), "write".to_string()],
            denied_operations: vec!["admin".to_string(), "delete_all".to_string()],
            risk_threshold: 0.5,
            requires_mfa: false,
            max_session_duration: Duration::from_secs(3600), // 1 hour
            network_restrictions: vec![],
        });
        
        // Guest security policy
        policies.insert("guest_security".to_string(), SecurityPolicy {
            policy_id: "guest_security".to_string(),
            context_type: "guest".to_string(),
            required_clearance_level: 1,
            allowed_operations: vec!["read".to_string()],
            denied_operations: vec!["write".to_string(), "admin".to_string(), "delete".to_string()],
            risk_threshold: 0.8,
            requires_mfa: false,
            max_session_duration: Duration::from_secs(900), // 15 minutes
            network_restrictions: vec!["public".to_string()],
        });
        
        Ok(())
    }
    
    /// Initialize default validation rules
    async fn initialize_default_validation_rules(&self) -> BearDogResult<()> {
        let mut rules = self.validation_rules.write().await;
        
        rules.push(ContextValidationRule {
            rule_id: "suspicious_device".to_string(),
            rule_type: ValidationRuleType::DeviceFingerprint,
            condition: "device_trust_level < 0.3".to_string(),
            action: ValidationAction::RequireAdditionalAuth,
            severity: ValidationSeverity::High,
        });
        
        rules.push(ContextValidationRule {
            rule_id: "high_risk_user".to_string(),
            rule_type: ValidationRuleType::UserBehavior,
            condition: "risk_score > 0.8".to_string(),
            action: ValidationAction::EscalateToAdmin,
            severity: ValidationSeverity::Critical,
        });
        
        rules.push(ContextValidationRule {
            rule_id: "external_network".to_string(),
            rule_type: ValidationRuleType::NetworkLocation,
            condition: "network_zone == 'external'".to_string(),
            action: ValidationAction::LogAndContinue,
            severity: ValidationSeverity::Medium,
        });
        
        Ok(())
    }
    
    /// Generate device fingerprint for trust assessment
    async fn generate_device_fingerprint(&self, device_id: &str) -> BearDogResult<DeviceFingerprint> {
        // In a real implementation, this would analyze device characteristics
        Ok(DeviceFingerprint {
            device_id: device_id.to_string(),
            trust_level: 0.8, // Default trust level
            characteristics: HashMap::new(),
        })
    }
    
    /// Assess user risk profile
    async fn assess_user_risk(&self, user_id: &str) -> BearDogResult<UserRiskProfile> {
        // In a real implementation, this would analyze user behavior patterns
        Ok(UserRiskProfile {
            user_id: user_id.to_string(),
            base_risk_score: 0.3, // Default risk score
            behavior_patterns: HashMap::new(),
        })
    }
    
    /// Determine security clearance level for user
    async fn determine_clearance_level(&self, _user_id: &str, risk_profile: &UserRiskProfile) -> BearDogResult<u8> {
        // Base clearance level determination on risk score
        let clearance = if risk_profile.base_risk_score < 0.2 {
            8 // High clearance
        } else if risk_profile.base_risk_score < 0.5 {
            5 // Standard clearance
        } else {
            2 // Low clearance
        };
        Ok(clearance)
    }
    
    /// Determine network zone
    async fn determine_network_zone(&self) -> BearDogResult<String> {
        // In a real implementation, this would analyze network characteristics
        Ok("internal".to_string())
    }
    
    /// Get applicable security policies for context
    async fn get_applicable_policies(&self, context: &SecurityContext) -> BearDogResult<Vec<SecurityPolicy>> {
        let policies = self.policies.read().await;
        let mut applicable = Vec::new();
        
        for policy in policies.values() {
            if context.security_clearance_level >= policy.required_clearance_level {
                applicable.push(policy.clone());
            }
        }
        
        Ok(applicable)
    }
    
    /// Evaluate validation rule against context
    async fn evaluate_validation_rule(
        &self,
        rule: &ContextValidationRule,
        context: &SecurityContext,
        active_context: &ActiveSecurityContext,
    ) -> BearDogResult<ValidationAction> {
        // Simple rule evaluation - in a real implementation, this would be more sophisticated
        match rule.rule_type {
            ValidationRuleType::DeviceFingerprint => {
                if context.device_trust_level < 0.3 {
                    Ok(rule.action.clone())
                } else {
                    Ok(ValidationAction::Allow)
                }
            },
            ValidationRuleType::UserBehavior => {
                if active_context.risk_score > 0.8 {
                    Ok(rule.action.clone())
                } else {
                    Ok(ValidationAction::Allow)
                }
            },
            ValidationRuleType::NetworkLocation => {
                if context.network_zone == "external" {
                    Ok(rule.action.clone())
                } else {
                    Ok(ValidationAction::Allow)
                }
            },
            _ => Ok(ValidationAction::Allow),
        }
    }
    
    /// Start background cleanup task
    async fn start_cleanup_task(&self) {
        let contexts = Arc::clone(&self.contexts);
        let cleanup_interval = self.cleanup_interval;
        let max_lifetime = self.max_context_lifetime;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                
                let mut contexts_guard = contexts.write().await;
                let now = Utc::now();
                
                contexts_guard.retain(|context_id, active_context| {
                    let age = now.signed_duration_since(active_context.created_at);
                    if age > chrono::Duration::from_std(max_lifetime).unwrap_or_default() {
                        debug!("🧹 Cleaning up expired context: {}", context_id);
                        false
                    } else {
                        true
                    }
                });
            }
        });
    }
}

/// Device fingerprint for trust assessment
#[derive(Debug, Clone)]
struct DeviceFingerprint {
    device_id: String,
    trust_level: f64,
    characteristics: HashMap<String, String>,
}

/// User risk profile
#[derive(Debug, Clone)]
struct UserRiskProfile {
    user_id: String,
    base_risk_score: f64,
    behavior_patterns: HashMap<String, f64>,
} 