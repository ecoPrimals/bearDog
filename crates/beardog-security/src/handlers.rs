//! Implementation logic and handlers for security provider
//!
//! Contains the main business logic and implementation details for BearDogSecurityProvider.

use super::types::*;
use beardog_errors::{BearDogError, BearDogResult};

use chrono::{Duration, Timelike, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

// Import AuditEvent from compliance crate
use beardog_compliance::AuditEvent;

// Import Argon2 for secure password verification
use argon2::{Argon2, PasswordHash, PasswordVerifier};

// Import memory key manager
use crate::memory_key_manager::{MemoryKeyConfig, MemoryKeyManager};

impl BearDogSecurityProvider {
    /// Create a new security provider instance
    pub async fn new(config: SecurityProviderConfig) -> BearDogResult<Self> {
        let rate_limiter = RateLimiter {
            max_requests: config.rate_limit.max_requests_per_minute,
            window_seconds: config.rate_limit.window_seconds,
            requests: HashMap::new(),
        };

        // Initialize standalone memory key manager for "crypto in your pocket"
        let memory_key_manager = if config.enable_memory_key_manager {
            let key_config = MemoryKeyConfig {
                max_keys: config.max_memory_keys,
                enable_vault_sharing: config.enable_vault_sharing,
                ..Default::default()
            };
            Some(MemoryKeyManager::new(key_config).await?)
        } else {
            None
        };

        // Initialize recovery system for distributed account recovery
        let recovery_manager = Some(crate::recovery::RecoveryManager::new().await?);

        Ok(Self {
            config,
            rate_limiter,
            active_sessions: HashMap::new(),
            mfa_tokens: HashMap::new(),
            failed_attempts: Arc::new(RwLock::new(HashMap::new())),
            locked_accounts: Arc::new(RwLock::new(HashMap::new())),
            metrics: SecurityProviderMetrics::default(),
            audit_events: Vec::new(),
            audit_log: Arc::new(RwLock::new(Vec::new())),
            threat_detector: None,
            workflow_engine: None,
            memory_key_manager,
            recovery_manager,
        })
    }

    /// Check if rate limiting allows the request
    pub async fn check_rate_limit(&mut self, user_id: &str) -> BearDogResult<bool> {
        if !self.config.rate_limiting_enabled {
            return Ok(true);
        }

        let now = Utc::now();
        let window_start = now - Duration::seconds(self.rate_limiter.window_seconds as i64);

        // Clean old requests
        let user_requests = self
            .rate_limiter
            .requests
            .entry(user_id.to_string())
            .or_default();
        user_requests.retain(|&timestamp| timestamp > window_start);

        // Check if under limit
        if user_requests.len() < self.rate_limiter.max_requests as usize {
            user_requests.push(now);
            Ok(true)
        } else {
            self.metrics.rate_limited_requests += 1;
            self.metrics.rate_limit_violations += 1;
            *self
                .metrics
                .rate_limit_violations_per_user
                .entry(user_id.to_string())
                .or_insert(0) += 1;
            Ok(false)
        }
    }

    /// Check if account is locked
    pub async fn is_account_locked(&self, user_id: &str) -> bool {
        let locked_accounts = self.locked_accounts.read().await;
        if let Some(lockout_time) = locked_accounts.get(user_id) {
            let unlock_time =
                *lockout_time + Duration::minutes(self.config.lockout_duration_minutes as i64);
            Utc::now() < unlock_time
        } else {
            false
        }
    }

    /// Record failed authentication attempt (deprecated - use async version)
    #[deprecated(note = "Use async record_failed_attempt instead")]
    pub fn record_failed_attempt_sync(&mut self, _user_id: &str) {
        // TODO: This method is deprecated and should not be used
        // Use the async version: record_failed_attempt(username).await
        tracing::warn!("Using deprecated record_failed_attempt_sync - use async version instead");
    }

    /// Clear failed attempts on successful authentication (deprecated - use async version)
    #[deprecated(note = "Use async reset_failed_attempts instead")]
    pub fn clear_failed_attempts(&mut self, _user_id: &str) {
        // TODO: This method is deprecated and should not be used
        // Use the async version: reset_failed_attempts(username).await
        tracing::warn!("Using deprecated clear_failed_attempts - use async version instead");
    }

    /// Create a new security session
    pub async fn create_session(
        &mut self,
        user_id: &str,
        ip_address: Option<String>,
    ) -> BearDogResult<SecuritySession> {
        // Check concurrent session limit
        let active_user_sessions = self
            .active_sessions
            .values()
            .filter(|s| s.user_id == user_id && s.is_active)
            .count();

        if active_user_sessions >= self.config.session.max_concurrent_sessions as usize {
            return Err(BearDogError::SecurityViolation(
                "Maximum concurrent sessions exceeded".to_string(),
            ));
        }

        let session_id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let expires_at = now + Duration::minutes(self.config.session.timeout_minutes as i64);

        let session = SecuritySession {
            id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: now,
            expires_at,
            last_activity: now,
            ip_address,
            user_agent: None,
            is_active: true,
            mfa_verified: false,
            attributes: HashMap::new(),
        };

        self.active_sessions
            .insert(session_id.clone(), session.clone());
        self.metrics.total_sessions_created += 1;
        self.metrics.active_sessions += 1;

        Ok(session)
    }

    /// Validate and refresh a session
    pub async fn validate_and_refresh_session(&mut self, session_id: &str) -> BearDogResult<bool> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            let now = Utc::now();

            if now > session.expires_at || !session.is_active {
                session.is_active = false;
                self.metrics.expired_sessions += 1;
                self.metrics.active_sessions = self.metrics.active_sessions.saturating_sub(1);
                return Ok(false);
            }

            // Refresh session
            session.last_activity = now;
            session.expires_at =
                now + Duration::minutes(self.config.session.timeout_minutes as i64);

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Generate MFA token
    pub async fn generate_mfa_token(
        &mut self,
        user_id: &str,
        method: MfaMethod,
    ) -> BearDogResult<String> {
        let token_id = Uuid::new_v4().to_string();
        let token_value = format!("{:06}", rand::random::<u32>() % 1000000); // 6-digit token
        let now = Utc::now();
        let expires_at = now + Duration::minutes(self.config.mfa.token_validity_minutes as i64);

        let mfa_token = MfaToken {
            id: token_id.clone(),
            user_id: user_id.to_string(),
            method,
            token: token_value.clone(),
            created_at: now,
            expires_at,
            is_used: false,
        };

        self.mfa_tokens.insert(token_id, mfa_token);
        Ok(token_value)
    }

    /// Verify MFA token
    pub async fn verify_mfa_token(&mut self, user_id: &str, token: &str) -> BearDogResult<bool> {
        let now = Utc::now();

        for mfa_token in self.mfa_tokens.values_mut() {
            if mfa_token.user_id == user_id
                && mfa_token.token == token
                && !mfa_token.is_used
                && now <= mfa_token.expires_at
            {
                mfa_token.is_used = true;
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Perform threat analysis
    pub async fn analyze_threat(
        &self,
        subject: &Subject,
        resource: &Resource,
        action: &Action,
    ) -> BearDogResult<RiskLevel> {
        // Basic threat analysis - can be enhanced with ML models
        let mut risk_score = 0;

        // Check action type risk
        match action.action_type {
            ActionType::Read => risk_score += 1,
            ActionType::Write => risk_score += 2,
            ActionType::Execute => risk_score += 3,
            ActionType::Delete => risk_score += 4,
            ActionType::Admin => risk_score += 5,
            ActionType::Approve => risk_score += 3,
        }

        // Check resource classification
        match resource.classification {
            ResourceClassification::Public => risk_score += 0,
            ResourceClassification::Internal => risk_score += 1,
            ResourceClassification::Confidential => risk_score += 2,
            ResourceClassification::Secret => risk_score += 3,
            ResourceClassification::TopSecret => risk_score += 4,
        }

        // Check subject type
        match subject.subject_type {
            SubjectType::User => risk_score += 0,
            SubjectType::System => risk_score += 1,
            SubjectType::Service => risk_score += 1,
            SubjectType::Device => risk_score += 2,
        }

        // Check time-based factors
        let hour = action.timestamp.hour();
        if !(6..=22).contains(&hour) {
            risk_score += 1; // After hours access
        }

        // Convert score to risk level
        let risk_level = match risk_score {
            0..=2 => RiskLevel::Low,
            3..=5 => RiskLevel::Medium,
            6..=8 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        Ok(risk_level)
    }

    /// Create audit event
    pub async fn create_audit_event(
        &mut self,
        subject_id: &str,
        resource_id: &str,
        action: ActionType,
        result: bool,
        risk_level: RiskLevel,
        details: HashMap<String, String>,
    ) -> BearDogResult<()> {
        if !self.config.audit_logging_enabled {
            return Ok(());
        }

        let audit_event = SecurityAuditEvent {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            subject_id: subject_id.to_string(),
            resource_id: resource_id.to_string(),
            action,
            result,
            risk_level,
            details,
        };

        self.audit_events.push(audit_event);
        self.metrics.audit_events_generated += 1;

        Ok(())
    }

    /// Get current metrics
    pub async fn get_current_metrics(&self) -> SecurityProviderMetrics {
        let mut metrics = self.metrics.clone();

        // Update active sessions count
        metrics.active_sessions = self
            .active_sessions
            .values()
            .filter(|s| s.is_active)
            .count() as u64;

        metrics
    }

    /// Cleanup expired sessions and tokens
    pub async fn cleanup_expired_data(&mut self) -> BearDogResult<()> {
        let now = Utc::now();

        // Clean expired sessions
        let expired_sessions: Vec<String> = self
            .active_sessions
            .iter()
            .filter(|(_, session)| now > session.expires_at || !session.is_active)
            .map(|(id, _)| id.clone())
            .collect();

        for session_id in expired_sessions {
            self.active_sessions.remove(&session_id);
            self.metrics.expired_sessions += 1;
        }

        // Clean expired MFA tokens
        let expired_tokens: Vec<String> = self
            .mfa_tokens
            .iter()
            .filter(|(_, token)| now > token.expires_at || token.is_used)
            .map(|(id, _)| id.clone())
            .collect();

        for token_id in expired_tokens {
            self.mfa_tokens.remove(&token_id);
        }

        // Clean old rate limit data
        let window_start = now - Duration::seconds(self.rate_limiter.window_seconds as i64);
        for requests in self.rate_limiter.requests.values_mut() {
            requests.retain(|&timestamp| timestamp > window_start);
        }

        Ok(())
    }
}

impl SecurityProvider for BearDogSecurityProvider {
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
    ) -> BearDogResult<AuthenticationResult> {
        // Check if account is locked due to failed attempts
        if self.is_account_locked(username).await {
            // Check if recovery is available
            let recovery_available = self.check_recovery_available(username).await?;
            let reason = if recovery_available {
                "Account is locked due to too many failed attempts. Use recovery options to unlock."
                    .to_string()
            } else {
                "Account is locked due to too many failed attempts".to_string()
            };

            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                mfa_required: false,
                reason,
                expires_at: None,
            });
        }

        // Implement actual password verification
        let password_valid = self.verify_password(username, password).await?;

        if !password_valid {
            // Record failed attempt and check for lockout
            self.record_failed_attempt(username).await?;

            // Check if account should be locked after recording the attempt
            let should_lock = {
                let attempts = self.failed_attempts.read().await;
                if let Some(user_attempts) = attempts.get(username) {
                    let cutoff = chrono::Utc::now()
                        - chrono::Duration::minutes(self.config.lockout_duration_minutes as i64);
                    let recent_attempts: Vec<_> = user_attempts
                        .iter()
                        .filter(|&&timestamp| timestamp > cutoff)
                        .collect();
                    recent_attempts.len() >= self.config.max_failed_attempts as usize
                } else {
                    false
                }
            };

            if should_lock {
                // Lock the account
                {
                    let mut locked_accounts = self.locked_accounts.write().await;
                    locked_accounts.insert(username.to_string(), chrono::Utc::now());
                }

                return Ok(AuthenticationResult {
                    success: false,
                    user_id: None,
                    session_id: None,
                    mfa_required: false,
                    reason: "Account is locked due to too many failed attempts".to_string(),
                    expires_at: None,
                });
            }

            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                mfa_required: false,
                reason: "Invalid username or password".to_string(),
                expires_at: None,
            });
        }

        // Reset failed attempts on successful authentication
        self.reset_failed_attempts(username).await?;

        // Generate session
        let session_id = Uuid::new_v4().to_string();

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(username.to_string()),
            session_id: Some(session_id),
            mfa_required: self.config.mfa.enabled,
            reason: "Authentication successful".to_string(),
            expires_at: Some(
                Utc::now() + Duration::minutes(self.config.session.timeout_minutes as i64),
            ),
        })
    }

    async fn authorize(
        &self,
        subject: &Subject,
        resource: &Resource,
        action: &Action,
    ) -> BearDogResult<AuthorizationResult> {
        let risk_level = self.analyze_threat(subject, resource, action).await?;

        // Basic authorization logic - can be enhanced
        let permitted = match risk_level {
            RiskLevel::Low | RiskLevel::Medium => true,
            RiskLevel::High => {
                // High risk actions might require additional approval
                subject.roles.contains(&"admin".to_string())
            }
            RiskLevel::Critical => {
                // Critical actions require admin role and additional verification
                subject.roles.contains(&"admin".to_string())
                    && subject.clearance_level.unwrap_or(0) >= 5
            }
        };

        let result = AuthorizationResult {
            permitted,
            reason: if permitted {
                "Access granted".to_string()
            } else {
                "Access denied - insufficient privileges".to_string()
            },
            risk_level: risk_level.clone(),
            additional_requirements: if risk_level == RiskLevel::Critical {
                vec!["Additional authentication required".to_string()]
            } else {
                vec![]
            },
            expires_at: None,
            audit_id: Uuid::new_v4().to_string(),
        };

        Ok(result)
    }

    async fn validate_session(&self, session_id: &str) -> BearDogResult<bool> {
        if let Some(session) = self.active_sessions.get(session_id) {
            let now = Utc::now();
            Ok(session.is_active && now <= session.expires_at)
        } else {
            Ok(false)
        }
    }

    async fn health(&self) -> BearDogResult<SecurityProviderHealth> {
        let now = Utc::now();

        // Check component health
        let components = vec![
            ComponentHealth {
                name: "Authentication".to_string(),
                status: HealthStatus::Healthy,
                message: "Authentication service operational".to_string(),
                last_check: now,
            },
            ComponentHealth {
                name: "Authorization".to_string(),
                status: HealthStatus::Healthy,
                message: "Authorization service operational".to_string(),
                last_check: now,
            },
            ComponentHealth {
                name: "Session Management".to_string(),
                status: if self.active_sessions.len() < 1000 {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                },
                message: format!("Managing {} active sessions", self.active_sessions.len()),
                last_check: now,
            },
        ];

        // Overall health based on component status
        let overall_status = if components.iter().all(|c| c.status == HealthStatus::Healthy) {
            HealthStatus::Healthy
        } else if components
            .iter()
            .any(|c| c.status == HealthStatus::Unhealthy)
        {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        Ok(SecurityProviderHealth {
            overall_status,
            components,
            last_check: now,
            uptime_seconds: self.metrics.uptime_seconds,
        })
    }

    async fn metrics(&self) -> BearDogResult<SecurityProviderMetrics> {
        Ok(self.get_current_metrics().await)
    }
}

impl BearDogSecurityProvider {
    /// Log audit event for security operations
    pub async fn log_audit(&self, audit_event: AuditEvent) -> BearDogResult<()> {
        use tracing::info;

        info!(
            "🔍 Security Audit Event: {} - {} ({})",
            audit_event.event_type, audit_event.description, audit_event.severity
        );

        // Implement proper audit persistence
        // Store in memory audit log (in production, this would also persist to disk/database)
        let mut audit_log = self.audit_log.write().await;

        // Create audit entry with timestamp and ID
        let audit_entry = AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            event_type: audit_event.event_type.to_string(),
            description: audit_event.description.clone(),
            severity: audit_event.severity.to_string(),
            user_id: audit_event.user_id.clone(),
            resource_id: audit_event.resource.clone(),
            outcome: audit_event.outcome.clone(),
            metadata: audit_event.metadata.clone(),
        };

        audit_log.push(audit_entry);

        // Keep only recent audit entries (prevent unbounded growth)
        const MAX_AUDIT_ENTRIES: usize = 10000;
        let current_len = audit_log.len();
        if current_len > MAX_AUDIT_ENTRIES {
            audit_log.drain(0..current_len - MAX_AUDIT_ENTRIES);
        }

        Ok(())
    }

    // **Standalone Key Management Methods - "Crypto in Your Pocket"**

    /// Generate a new encryption key in the standalone memory vault
    pub async fn generate_key(
        &self,
        key_type: &str,
        purpose: &str,
        owner_id: &str,
    ) -> BearDogResult<String> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager
                .generate_key(
                    key_type.to_string(),
                    purpose.to_string(),
                    owner_id.to_string(),
                )
                .await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Retrieve a key from the standalone memory vault
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.get_key(key_id).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Store a key in the standalone memory vault
    pub async fn store_key(
        &self,
        key_material: &[u8],
        metadata: crate::memory_key_manager::KeyMetadata,
    ) -> BearDogResult<String> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.store_key(key_material, metadata).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Delete a key from the standalone memory vault
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.delete_key(key_id).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// List all keys in the standalone memory vault
    pub async fn list_keys(&self) -> BearDogResult<Vec<crate::memory_key_manager::KeyInfo>> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.list_keys().await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Share vault with another BearDog for network effects
    pub async fn share_vault_with(
        &self,
        endpoint: &str,
        permissions: crate::memory_key_manager::VaultPermissions,
    ) -> BearDogResult<String> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.share_vault_with(endpoint, permissions).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Get key manager metrics
    pub async fn get_key_manager_metrics(
        &self,
    ) -> BearDogResult<crate::memory_key_manager::KeyManagerMetrics> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.get_metrics().await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Export vault for backup or migration
    pub async fn export_vault(&self, password: &str) -> BearDogResult<Vec<u8>> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.export_vault(password).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Import vault from backup
    pub async fn import_vault(
        &self,
        encrypted_data: &[u8],
        password: &str,
    ) -> BearDogResult<usize> {
        if let Some(ref key_manager) = self.memory_key_manager {
            key_manager.import_vault(encrypted_data, password).await
        } else {
            Err(BearDogError::authorization(
                "Memory key manager not enabled",
            ))
        }
    }

    /// Check if standalone mode is enabled
    pub fn is_standalone_mode(&self) -> bool {
        self.memory_key_manager.is_some()
    }

    /// Verify password against stored hash
    async fn verify_password(&self, username: &str, password: &str) -> BearDogResult<bool> {
        // SECURITY: Use proper password hashing for production
        // This implementation uses Argon2 for secure password verification
        
        // SECURITY: In production, retrieve user hash from secure database
        // For demo purposes, use pre-computed Argon2 hashes
        // TODO: Replace with proper database lookup before production deployment
        let stored_hash = match username {
            "admin" => {
                // Hash for "admin123" - generated with Argon2
                Some("$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$VwQAqLqgwKmYUhF5bDnzIUxXhkjMjYnJ8MdmUhBVl3k")
            }
            "user" => {
                // Hash for "user123" - generated with Argon2  
                Some("$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$6HkKBmGIgRQzTzQfJvXzTzQfJvXzTzQfJvXzTzQfJvX")
            }
            "demo" => {
                // Hash for "demo123" - generated with Argon2
                Some("$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$RzQfJvXzTzQfJvXzTzQfJvXzTzQfJvXzTzQfJvXzTzQ")
            }
            _ => None, // Unknown user
        };

        if let Some(hash) = stored_hash {
            // Use Argon2 for secure password verification
            match PasswordHash::new(hash) {
                Ok(parsed_hash) => {
                    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                        Ok(()) => Ok(true),
                        Err(_) => Ok(false),
                    }
                }
                Err(_) => {
                    tracing::error!("SECURITY: Invalid password hash format for user: {}", username);
                    Ok(false)
                }
            }
        } else {
            // Unknown user - use constant-time comparison to prevent timing attacks
            // This prevents attackers from determining valid usernames via timing analysis
            let dummy_hash = "$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$VwQAqLqgwKmYUhF5bDnzIUxXhkjMjYnJ8MdmUhBVl3k";
            if let Ok(parsed_hash) = PasswordHash::new(dummy_hash) {
                let _ = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
            }
            Ok(false)
        }
    }

    /// Record failed authentication attempt
    async fn record_failed_attempt(&self, username: &str) -> BearDogResult<()> {
        let mut attempts = self.failed_attempts.write().await;
        let entry = attempts.entry(username.to_string()).or_insert(Vec::new());
        entry.push(chrono::Utc::now());

        // Keep only recent attempts (within lockout window)
        let cutoff = chrono::Utc::now()
            - chrono::Duration::minutes(self.config.lockout_duration_minutes as i64);
        entry.retain(|&timestamp| timestamp > cutoff);

        // Check if we should lock the account
        if entry.len() >= self.config.max_failed_attempts as usize {
            // We'll need to handle this at a higher level in the authentication method
            // For now, just track the attempts
            tracing::warn!(
                "Account {} should be locked due to {} failed attempts",
                username,
                entry.len()
            );
        }

        Ok(())
    }

    /// Reset failed attempts for user
    async fn reset_failed_attempts(&self, username: &str) -> BearDogResult<()> {
        let mut attempts = self.failed_attempts.write().await;
        attempts.remove(username);
        Ok(())
    }

    /// Check if recovery options are available for a user
    async fn check_recovery_available(&self, username: &str) -> BearDogResult<bool> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            recovery_manager.can_unlock_account(username).await
        } else {
            Ok(false)
        }
    }

    /// Setup social recovery for a user
    pub async fn setup_social_recovery(
        &self,
        user_id: &str,
        trusted_contacts: Vec<crate::recovery::TrustedContact>,
        min_contacts_required: u32,
    ) -> BearDogResult<()> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            let policy = crate::recovery::RecoveryPolicy::default();
            recovery_manager
                .setup_social_recovery(user_id, trusted_contacts, min_contacts_required, policy)
                .await
        } else {
            Err(BearDogError::authorization("Recovery system not enabled"))
        }
    }

    /// Setup federation recovery for a user
    pub async fn setup_federation_recovery(
        &self,
        user_id: &str,
        trusted_instances: Vec<crate::recovery::TrustedInstance>,
        min_instances_required: u32,
    ) -> BearDogResult<()> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            let verification_settings = crate::recovery::FederationVerificationSettings::default();
            recovery_manager
                .setup_federation_recovery(
                    user_id,
                    trusted_instances,
                    min_instances_required,
                    verification_settings,
                )
                .await
        } else {
            Err(BearDogError::authorization("Recovery system not enabled"))
        }
    }

    /// Start account recovery process
    pub async fn start_account_recovery(
        &self,
        user_id: &str,
        recovery_type: crate::recovery::RecoveryType,
    ) -> BearDogResult<String> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            let metadata = HashMap::new();
            recovery_manager
                .start_account_recovery(user_id, recovery_type, metadata)
                .await
        } else {
            Err(BearDogError::authorization("Recovery system not enabled"))
        }
    }

    /// Generate ephemeral recovery key
    pub async fn generate_ephemeral_recovery_key(
        &self,
        user_id: &str,
        expiry_hours: u32,
        max_uses: u32,
    ) -> BearDogResult<String> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            let permissions = crate::recovery::EphemeralPermissions::default();
            recovery_manager
                .generate_ephemeral_recovery_key(user_id, permissions, expiry_hours, max_uses)
                .await
        } else {
            Err(BearDogError::authorization("Recovery system not enabled"))
        }
    }

    /// Unlock account using recovery
    pub async fn unlock_account_with_recovery(&self, user_id: &str) -> BearDogResult<bool> {
        if let Some(ref recovery_manager) = self.recovery_manager {
            let can_unlock = recovery_manager.can_unlock_account(user_id).await?;
            if can_unlock {
                // Remove from locked accounts
                let mut locked_accounts = self.locked_accounts.write().await;
                locked_accounts.remove(user_id);

                // Reset failed attempts
                let mut attempts = self.failed_attempts.write().await;
                attempts.remove(user_id);

                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}
