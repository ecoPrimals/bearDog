//! Standalone Security Mode
//!
//! Conservative security policies when federation is unavailable.
//! Implements the "fail closed" philosophy - deny when uncertain.
//!
//! ## Architecture
//!
//! When BearDog cannot reach federation/quorum:
//! 1. Use more conservative threat thresholds
//! 2. Apply stricter rate limiting
//! 3. Deny operations requiring federation consensus
//! 4. Audit log all decisions for later reconciliation
//!
//! ## Philosophy
//!
//! **"Better to deny a legitimate request than approve a malicious one"**
//!
//! In standalone mode, BearDog errs on the side of caution, implementing
//! more restrictive policies to maintain security when unable to achieve
//! distributed consensus with other security nodes.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::security::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Standalone security mode with conservative policies
pub struct StandaloneSecurityMode {
    /// Conservative security policy
    policy: ConservativeSecurityPolicy,
    
    /// Local threat database
    threat_db: Arc<RwLock<LocalThreatDatabase>>,
    
    /// Audit logger for reconciliation
    audit_log: Arc<AuditLogger>,
    
    /// Rate limiter (stricter than network mode)
    rate_limiter: Arc<RwLock<StandaloneRateLimiter>>,
    
    /// Standalone mode start time
    standalone_since: DateTime<Utc>,
}

impl StandaloneSecurityMode {
    /// Create new standalone security mode
    pub fn new() -> Self {
        info!("🏛️ Initializing standalone security mode");
        info!("   Conservative policies enabled");
        info!("   Operating in sovereign mode - no federation");
        
        Self {
            policy: ConservativeSecurityPolicy::default(),
            threat_db: Arc::new(RwLock::new(LocalThreatDatabase::new())),
            audit_log: Arc::new(AuditLogger::new()),
            rate_limiter: Arc::new(RwLock::new(StandaloneRateLimiter::default())),
            standalone_since: Utc::now(),
        }
    }
    
    /// Evaluate security request in standalone mode
    pub async fn evaluate_security_request(
        &self,
        request: &SecurityRequest,
    ) -> BearDogResult<SecurityDecision> {
        info!("🔒 Evaluating security request in standalone mode");
        info!("   User: {}", request.user_id);
        info!("   Operation: {}", request.operation);
        
        // Check if operation requires federation
        if self.policy.requires_federation(&request.operation) {
            warn!("⚠️ Operation requires federation but operating standalone");
            self.audit_log.record_federation_required(request).await;
            
            return Ok(SecurityDecision::Deny {
                reason: format!(
                    "Operation '{}' requires federation consensus (standalone mode)",
                    request.operation
                ),
                retry_after: Some(Duration::from_secs(
                    std::env::var("BEARDOG_FEDERATION_RETRY_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(60)
                )),
                security_level: SecurityLevel::Critical,
            });
        }
        
        // Check rate limits (stricter in standalone)
        if !self.rate_limiter.write().await.allow_request(&request.user_id).await? {
            self.audit_log.record_rate_limit(request).await;
            
            return Ok(SecurityDecision::Deny {
                reason: "Rate limit exceeded (standalone conservative policy)".to_string(),
                retry_after: Some(Duration::from_secs(
                    std::env::var("BEARDOG_RATE_LIMIT_RETRY_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(300)
                )),
                security_level: SecurityLevel::Moderate,
            });
        }
        
        // Check threat database
        let threat_score = self.threat_db.read().await
            .check_threat(request)
            .await?;
        
        info!("   Threat score: {:.2}", threat_score);
        
        // Conservative threshold (lower than network mode)
        if threat_score > self.policy.threat_threshold {
            warn!("⚠️ Threat score exceeds conservative threshold");
            self.audit_log.record_denial(request, "standalone_conservative", threat_score).await;
            
            return Ok(SecurityDecision::Deny {
                reason: format!(
                    "Standalone mode: conservative policy (threat score: {:.2} > threshold: {:.2})",
                    threat_score,
                    self.policy.threat_threshold
                ),
                retry_after: Some(Duration::from_secs(
                    std::env::var("BEARDOG_SECURITY_THREAT_RETRY_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(60)
                )),
                security_level: SecurityLevel::High,
            });
        }
        
        // Allow with lower confidence (standalone has less context)
        info!("✅ Request approved in standalone mode (conservative)");
        self.audit_log.record_standalone_approval(request, threat_score).await;
        
        Ok(SecurityDecision::Allow {
            confidence: 0.6, // Lower confidence in standalone
            security_level: SecurityLevel::Moderate,
            restrictions: vec![
                "standalone_mode".to_string(),
                "conservative_policy".to_string(),
            ],
        })
    }
    
    /// Reconcile decisions when federation returns
    pub async fn reconcile_with_federation(
        &self,
        federation_coordinator: &dyn SecurityQuorumCoordinator,
    ) -> BearDogResult<ReconciliationReport> {
        info!("🔄 Reconciling standalone decisions with federation");
        
        let standalone_decisions = self.audit_log.get_standalone_decisions().await?;
        
        let mut report = ReconciliationReport::default();
        
        for decision in standalone_decisions {
            // Re-evaluate with federation consensus
            match federation_coordinator.evaluate_historical_decision(&decision).await {
                Ok(federation_decision) if federation_decision.agrees_with(&decision) => {
                    report.confirmed += 1;
                    info!("✅ Federation confirms standalone decision: {}", decision.operation_id);
                }
                Ok(federation_decision) => {
                    report.disagreed += 1;
                    warn!(
                        "⚠️ Federation disagrees with standalone decision: {}",
                        decision.operation_id
                    );
                    warn!("   Standalone: {:?}", decision.decision);
                    warn!("   Federation: {:?}", federation_decision);
                }
                Err(e) => {
                    report.errors += 1;
                    warn!("❌ Reconciliation error for {}: {}", decision.operation_id, e);
                }
            }
        }
        
        info!("🔄 Reconciliation complete:");
        info!("   Confirmed: {}", report.confirmed);
        info!("   Disagreed: {}", report.disagreed);
        info!("   Errors: {}", report.errors);
        
        Ok(report)
    }
    
    /// Get standalone mode duration
    pub fn standalone_duration(&self) -> Duration {
        Utc::now()
            .signed_duration_since(self.standalone_since)
            .to_std()
            .unwrap_or(Duration::from_secs(0))
    }
}

impl Default for StandaloneSecurityMode {
    fn default() -> Self {
        Self::new()
    }
}

/// Conservative security policy for standalone operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConservativeSecurityPolicy {
    /// Threat score threshold (lower = more strict)
    pub threat_threshold: f32,
    
    /// Rate limit (requests per minute)
    pub rate_limit: u32,
    
    /// Allowed operations in standalone mode
    pub allowed_operations: Vec<String>,
    
    /// Operations requiring federation
    pub require_federation: Vec<String>,
}

impl ConservativeSecurityPolicy {
    /// Check if operation requires federation
    pub fn requires_federation(&self, operation: &str) -> bool {
        self.require_federation.iter().any(|op| operation.contains(op))
    }
}

impl Default for ConservativeSecurityPolicy {
    fn default() -> Self {
        Self {
            threat_threshold: 0.3, // More conservative (vs 0.5 in network mode)
            rate_limit: 10,        // Stricter rate limit (vs 30 in network mode)
            allowed_operations: vec![
                "read".to_string(),
                "basic_auth".to_string(),
                "health_check".to_string(),
            ],
            require_federation: vec![
                "admin_operation".to_string(),
                "sensitive_data_access".to_string(),
                "configuration_change".to_string(),
                "key_rotation".to_string(),
                "permission_grant".to_string(),
            ],
        }
    }
}

/// Local threat database for offline operation
pub struct LocalThreatDatabase {
    known_threats: HashMap<String, ThreatEntry>,
    suspicious_patterns: Vec<String>,
    last_updated: DateTime<Utc>,
}

impl LocalThreatDatabase {
    pub fn new() -> Self {
        let mut db = Self {
            known_threats: HashMap::new(),
            suspicious_patterns: Vec::new(),
            last_updated: Utc::now(),
        };
        
        // Initialize with common attack patterns
        db.suspicious_patterns = vec![
            "admin".to_string(),
            "../".to_string(),
            "<script>".to_string(),
            "DROP TABLE".to_string(),
            "'; --".to_string(),
            "UNION SELECT".to_string(),
            "eval(".to_string(),
            "exec(".to_string(),
        ];
        
        db
    }
    
    pub async fn check_threat(&self, request: &SecurityRequest) -> BearDogResult<f32> {
        let mut threat_score = 0.0;
        
        // IP-based checks
        if let Some(threat) = self.known_threats.get(&request.source_ip) {
            threat_score += threat.severity_score;
        }
        
        // Pattern-based checks
        if self.matches_attack_pattern(request) {
            threat_score += 0.5;
        }
        
        // User ID checks
        if request.user_id.contains("admin") || request.user_id.contains("root") {
            threat_score += 0.2;
        }
        
        // Operation type checks
        if request.operation.contains("DELETE") || request.operation.contains("DROP") {
            threat_score += 0.3;
        }
        
        Ok(threat_score.min(1.0))
    }
    
    fn matches_attack_pattern(&self, request: &SecurityRequest) -> bool {
        self.suspicious_patterns.iter().any(|pattern| {
            request.request_path.contains(pattern) ||
            request.user_agent.to_lowercase().contains(&pattern.to_lowercase())
        })
    }
    
    pub fn add_threat(&mut self, ip: String, entry: ThreatEntry) {
        self.known_threats.insert(ip, entry);
    }
}

#[derive(Debug, Clone)]
pub struct ThreatEntry {
    pub severity_score: f32,
    pub detected_at: DateTime<Utc>,
    pub attack_type: String,
}

/// Audit logger for standalone decisions
pub struct AuditLogger {
    decisions: Arc<RwLock<Vec<AuditEntry>>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            decisions: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    pub async fn record_denial(
        &self,
        request: &SecurityRequest,
        reason: &str,
        threat_score: f32,
    ) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            operation_id: request.operation_id.clone(),
            user_id: request.user_id.clone(),
            operation: request.operation.clone(),
            decision: "DENY".to_string(),
            reason: reason.to_string(),
            threat_score: Some(threat_score),
        };
        
        self.decisions.write().await.push(entry);
    }
    
    pub async fn record_standalone_approval(
        &self,
        request: &SecurityRequest,
        threat_score: f32,
    ) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            operation_id: request.operation_id.clone(),
            user_id: request.user_id.clone(),
            operation: request.operation.clone(),
            decision: "ALLOW".to_string(),
            reason: "standalone_approved".to_string(),
            threat_score: Some(threat_score),
        };
        
        self.decisions.write().await.push(entry);
    }
    
    pub async fn record_federation_required(&self, request: &SecurityRequest) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            operation_id: request.operation_id.clone(),
            user_id: request.user_id.clone(),
            operation: request.operation.clone(),
            decision: "DENY".to_string(),
            reason: "federation_required".to_string(),
            threat_score: None,
        };
        
        self.decisions.write().await.push(entry);
    }
    
    pub async fn record_rate_limit(&self, request: &SecurityRequest) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            operation_id: request.operation_id.clone(),
            user_id: request.user_id.clone(),
            operation: request.operation.clone(),
            decision: "DENY".to_string(),
            reason: "rate_limit_exceeded".to_string(),
            threat_score: None,
        };
        
        self.decisions.write().await.push(entry);
    }
    
    pub async fn get_standalone_decisions(&self) -> BearDogResult<Vec<AuditEntry>> {
        Ok(self.decisions.read().await.clone())
    }
}

#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub operation_id: String,
    pub user_id: String,
    pub operation: String,
    pub decision: String,
    pub reason: String,
    pub threat_score: Option<f32>,
}

impl AuditEntry {
    pub fn agrees_with(&self, other: &Self) -> bool {
        self.decision == other.decision
    }
}

/// Rate limiter for standalone mode
pub struct StandaloneRateLimiter {
    requests: HashMap<String, Vec<DateTime<Utc>>>,
    window: Duration,
    max_requests: usize,
}

impl StandaloneRateLimiter {
    pub async fn allow_request(&mut self, user_id: &str) -> BearDogResult<bool> {
        let now = Utc::now();
        
        // Clean old requests outside window
        if let Some(user_requests) = self.requests.get_mut(user_id) {
            user_requests.retain(|&timestamp| {
                now.signed_duration_since(timestamp).to_std()
                    .map(|d| d < self.window)
                    .unwrap_or(false)
            });
        }
        
        // Get current count
        let count = self.requests
            .get(user_id)
            .map(|v| v.len())
            .unwrap_or(0);
        
        if count >= self.max_requests {
            warn!("⚠️ Rate limit exceeded for user: {} ({}/{})", user_id, count, self.max_requests);
            return Ok(false);
        }
        
        // Record new request
        self.requests
            .entry(user_id.to_string())
            .or_insert_with(Vec::new)
            .push(now);
        
        Ok(true)
    }
}

impl Default for StandaloneRateLimiter {
    fn default() -> Self {
        Self {
            requests: HashMap::new(),
            window: Duration::from_secs(
                std::env::var("BEARDOG_STANDALONE_RATE_LIMIT_WINDOW_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
            max_requests: std::env::var("BEARDOG_STANDALONE_RATE_LIMIT_MAX_REQUESTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }
}

/// Reconciliation report
#[derive(Debug, Default)]
pub struct ReconciliationReport {
    pub confirmed: usize,
    pub disagreed: usize,
    pub errors: usize,
}

/// Security quorum coordinator trait
#[async_trait::async_trait]
pub trait SecurityQuorumCoordinator: Send + Sync {
    async fn evaluate_historical_decision(
        &self,
        decision: &AuditEntry,
    ) -> BearDogResult<AuditEntry>;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_standalone_security_conservative_policy() {
        let standalone = StandaloneSecurityMode::new();
        
        // Normal request should pass (but with lower confidence)
        let normal_request = SecurityRequest {
            operation_id: "test_001".to_string(),
            user_id: "test_user".to_string(),
            operation: "read".to_string(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            source_ip: "192.168.1.100".to_string(),
            request_path: "/api/data".to_string(),
            user_agent: "test-client/1.0".to_string(),
        };
        
        let decision = standalone.evaluate_security_request(&normal_request).await?;
        assert!(matches!(decision, SecurityDecision::Allow { confidence, .. } if confidence < 0.7));
    }
    
    #[tokio::test]
    async fn test_standalone_security_denies_suspicious() {
        let standalone = StandaloneSecurityMode::new();
        
        // Suspicious request should be denied
        let suspicious_request = SecurityRequest {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            operation_id: "test_002".to_string(),
            user_id: "admin'; DROP TABLE users; --".to_string(),
            operation: "admin_operation".to_string(),
            source_ip: "192.168.1.100".to_string(),
            request_path: "/api/admin".to_string(),
            user_agent: "test-client/1.0".to_string(),
        };
        
        let decision = standalone.evaluate_security_request(&suspicious_request).await?;
        assert!(matches!(decision, SecurityDecision::Deny { .. }));
    }
    
    #[tokio::test]
    async fn test_federation_required_operations() {
        let standalone = StandaloneSecurityMode::new();
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: security
         // TEST_PRIORITY: normal
        
        // Operation requiring federation
        let admin_request = SecurityRequest {
            operation_id: "test_003".to_string(),
            user_id: "admin".to_string(),
            operation: "configuration_change".to_string(),
            source_ip: "192.168.1.100".to_string(),
            request_path: "/api/config".to_string(),
            user_agent: "test-client/1.0".to_string(),
        };
        
        let decision = standalone.evaluate_security_request(&admin_request).await?;
        assert!(matches!(decision, SecurityDecision::Deny { .. }));
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_rate_limiting() {
        let standalone = StandaloneSecurityMode::new();
        
        let request = SecurityRequest {
            operation_id: "test_004".to_string(),
            user_id: "test_user".to_string(),
            operation: "read".to_string(),
            source_ip: "192.168.1.100".to_string(),
            request_path: "/api/data".to_string(),
            user_agent: "test-client/1.0".to_string(),
        };
        
        // First 10 should succeed
        for _ in 0..10 {
            let decision = standalone.evaluate_security_request(&request).await?;
            assert!(matches!(decision, SecurityDecision::Allow { .. }));
        }
        
        // 11th should be rate limited
        let decision = standalone.evaluate_security_request(&request).await?;
        assert!(matches!(decision, SecurityDecision::Deny { reason, .. } if reason.contains("Rate limit")));
    }
}

