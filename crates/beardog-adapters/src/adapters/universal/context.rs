

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use super::traits::SecurityContext;
use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug};
use chrono::{DateTime, Utc};

pub struct SecurityContextManager {

    contexts: Arc<RwLock<HashMap<String, ActiveSecurityContext>>>,

    policies: Arc<RwLock<HashMap<String, SecurityPolicy>>>,

    validation_rules: Arc<RwLock<Vec<ContextValidationRule>>>,

    max_context_lifetime: Duration,

    cleanup_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveSecurityContext {
    context: SecurityContext,
    created_at: DateTime<Utc>,
    last_accessed: DateTime<Utc>,
    access_count: u64,
    risk_score: f64,
    validation_status: ContextValidationStatus,

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

struct ContextValidationRule {
    rule_id: String,
    rule_type: ValidationRuleType,
    condition: String,
    action: ValidationAction,
    severity: ValidationSeverity,

enum ValidationRuleType {
    DeviceFingerprint,
    NetworkLocation,
    UserBehavior,
    ThreatIntelligence,
    TimeBasedAccess,
    GeolocationCheck,

enum ValidationAction {
    Allow,
    Deny,
    RequireAdditionalAuth,
    LogAndContinue,
    EscalateToAdmin,

enum ValidationSeverity {
    Low,
    Medium,
    High,
    Critical,

enum ContextValidationStatus {
    Valid,
    PendingValidation,
    RequiresRevalidation,
    Invalid(String),
impl SecurityContextManager {
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🛡️ Initializing Security Context Manager");
        
        let manager = Self {
            contexts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            policies: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            validation_rules: Arc::new(RwLock::new(Vec::new())),
            max_context_lifetime: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(300), // 5 minutes
        };

        manager.initialize_default_policies().await?;

        manager.initialize_default_validation_rules().await?;

        manager.start_cleanup_task().await;
        info!("✅ Security Context Manager initialized successfully");
        Ok(manager)
    }

    pub async fn create_context(&self, user_id: &str, device_id: &str) -> Result<SecurityContext, BearDogError> {
        debug!("🔐 Creating security context for user: {}, device: {}", user_id, device_id);

        let context_id = format_args!("ctx_{}_{}_{}", user_id, device_id, uuid::Uuid::new_v4().to_string());

        let device_fingerprint = self.generate_device_fingerprint(device_id).await?;
        let user_risk_profile = self.assess_user_risk(user_id).await?;

        let mut context = SecurityContext::default();
        context.context_id = context_id.clone();
        context.user_id = user_id.to_string();
        context.device_id = device_id.to_string();
        context.created_at = Utc::now().to_rfc3339();
        context.security_clearance_level = self.determine_clearance_level(user_id, &user_risk_profile).await?;
        context.risk_score = user_risk_profile.base_risk_score;
        context.device_trust_level = device_fingerprint.trust_level;
        context.network_zone = self.determine_network_zone().await?;

        let applicable_policies = self.get_applicable_policies(&context).await?;
        context.allowed_operations = applicable_policies.iter()
            .flat_map(|p| p.allowed_operations.clone())
            .collect();
        context.denied_operations = applicable_policies.iter()
            .flat_map(|p| p.denied_operations.clone())

        let active_context = ActiveSecurityContext {
            context: context.clone(),
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            risk_score: user_risk_profile.base_risk_score,
            validation_status: ContextValidationStatus::Valid,

        {
            let mut contexts = self.contexts.write().await;
            contexts.insert(context_id.clone(), active_context);
        }
        info!("✅ Created security context: {} for user: {}", context_id, user_id);
        Ok(context)

    pub async fn validate_context(&self, context: &SecurityContext) -> Result<bool, BearDogError> {
        debug!("🔍 Validating security context: {}", context.context_id);

        let mut active_context = {
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

        let now = Utc::now();
        let age = now.signed_duration_since(active_context.created_at);
        if age > chrono::Duration::from_std(self.max_context_lifetime)? {
            warn!("Context expired: {}", context.context_id);
            self.invalidate_context(&context.context_id).await?;
            return Ok(false);

        let validation_rules = self.validation_rules.read().await;
        for rule in validation_rules.iter() {
            match self.evaluate_validation_rule(rule, context, &active_context).await? {
                ValidationAction::Deny => {
                    warn!("Context validation denied by rule: {}", rule.rule_id);
                ValidationAction::RequireAdditionalAuth => {
                    info!("Context requires additional authentication: {}", context.context_id);

                    active_context.validation_status = ContextValidationStatus::RequiresRevalidation;
                ValidationAction::EscalateToAdmin => {
                    warn!("Context validation escalated to admin: {}", context.context_id);

                ValidationAction::LogAndContinue => {
                    debug!("Context validation logged: rule {}", rule.rule_id);
                ValidationAction::Allow => {

            if let Some(ctx) = contexts.get_mut(&context.context_id) {
                ctx.validation_status = active_context.validation_status;
                ctx.risk_score = active_context.risk_score;
        debug!("✅ Context validation completed for: {}", context.context_id);
        Ok(true)

    pub async fn get_context(&self, context_id: &str) -> Result<Option<SecurityContext>, BearDogError>> {
        let contexts = self.contexts.read().await;
        Ok(contexts.get(context_id).map(|ctx| ctx.context.clone()))

    pub async fn invalidate_context(&self, context_id: &str) -> Result<(), BearDogError> {
        info!("🚫 Invalidating security context: {}", context_id);
        let mut contexts = self.contexts.write().await;
        contexts.remove(context_id);
        Ok(())

    async fn initialize_default_policies(&self) -> Result<(), BearDogError> {
        let mut policies = self.policies.write().await;

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

        policies.insert("guest_security".to_string(), SecurityPolicy {
            policy_id: "guest_security".to_string(),
            context_type: "guest".to_string(),
            required_clearance_level: 1,
            allowed_operations: vec!["read".to_string()],
            denied_operations: vec!["write".to_string(), "admin".to_string(), "delete".to_string()],
            risk_threshold: 0.8,
            max_session_duration: Duration::from_secs(900), // 15 minutes
            network_restrictions: vec!["public".to_string()],

    async fn initialize_default_validation_rules(&self) -> Result<(), BearDogError> {
        let mut rules = self.validation_rules.write().await;
        rules.push(ContextValidationRule {
            rule_id: "suspicious_device".to_string(),
            rule_type: ValidationRuleType::DeviceFingerprint,
            condition: "device_trust_level < 0.3".to_string(),
            action: ValidationAction::RequireAdditionalAuth,
            severity: ValidationSeverity::High,
            rule_id: "high_risk_user".to_string(),
            rule_type: ValidationRuleType::UserBehavior,
            condition: "risk_score > 0.8".to_string(),
            action: ValidationAction::EscalateToAdmin,
            severity: ValidationSeverity::Critical,
            rule_id: "external_network".to_string(),
            rule_type: ValidationRuleType::NetworkLocation,
            condition: "network_zone == 'external'".to_string(),
            action: ValidationAction::LogAndContinue,
            severity: ValidationSeverity::Medium,

    async fn generate_device_fingerprint(&self, device_id: &str) -> Result<DeviceFingerprint, BearDogError> {

        Ok(DeviceFingerprint {
            device_id: device_id.to_string(),
            trust_level: 0.8, // Default trust level
            characteristics: HashMap::with_capacity(16),
        })

    async fn assess_user_risk(&self, user_id: &str) -> Result<UserRiskProfile, BearDogError> {

        Ok(UserRiskProfile {
            user_id: user_id.to_string(),
            base_risk_score: 0.3, // Default risk score
            behavior_patterns: HashMap::with_capacity(16),

    async fn determine_clearance_level(&self, _user_id: &str, risk_profile: &UserRiskProfile) -> Result<u8, BearDogError> {

        let clearance = if risk_profile.base_risk_score < 0.2 {
            8 // High clearance
        } else if risk_profile.base_risk_score < 0.5 {
            5 // Standard clearance
        } else {
            2 // Low clearance
        Ok(clearance)

    async fn determine_network_zone(&self) -> Result<String, BearDogError> {

        Ok("internal".to_string())

    async fn get_applicable_policies(&self, context: &SecurityContext) -> Result<Vec<SecurityPolicy>, BearDogError>> {
        let policies = self.policies.read().await;
        let mut applicable = Vec::new();
        for policy in policies.values() {
            if context.security_clearance_level >= policy.required_clearance_level {
                applicable.push(policy.clone());
        Ok(applicable)

    async fn evaluate_validation_rule(
        &self,
        rule: &ContextValidationRule,
        context: &SecurityContext,
        active_context: &ActiveSecurityContext,
    ) -> Result<ValidationAction, BearDogError> {

        match rule.rule_type {
            ValidationRuleType::DeviceFingerprint => {
                if context.device_trust_level < 0.3 {
                    Ok(rule.action.clone())
                } else {
                    Ok(ValidationAction::Allow)
            },
            ValidationRuleType::UserBehavior => {
                if active_context.risk_score > 0.8 {
            ValidationRuleType::NetworkLocation => {
                if context.network_zone == "external" {
            _ => Ok(ValidationAction::Allow),

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

#[derive(Debug, Clone)]
struct DeviceFingerprint {
    device_id: String,
    trust_level: f64,
    characteristics: HashMap<String, String>,

struct UserRiskProfile {
    user_id: String,
    base_risk_score: f64,
    behavior_patterns: HashMap<String, f64>,
} 
