//! Session Management Module
//!
//! Handles user session creation, validation, and management.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Create a new session for a user
    pub async fn create_session(
        &mut self,
        user_id: &str,
        user_info: UserInfo,
        session_config: SessionConfig,
    ) -> BearDogResult<SessionToken> {
        // Check if account is locked
        if self.is_account_locked(user_id).await {
            return Err(BearDogError::Authentication {
                message: "Account is locked".to_string(),
            });
        }

        // Check rate limiting
        if !self.check_rate_limit(user_id).await? {
            return Err(BearDogError::RateLimited {
                message: "Too many session creation attempts".to_string(),
                retry_after: Some(60),
            });
        }

        // Generate session token
        let session_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::seconds(session_config.max_age_seconds as i64);
        
        let session = Session {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            user_info,
            created_at: Utc::now(),
            expires_at,
            is_active: true,
            last_activity: Utc::now(),
            permissions: vec![], // TODO: Implement permission system
            metadata: HashMap::new(),
        };

        // Store session
        self.session_store.insert(session_id.clone(), session);

        // Update metrics
        self.metrics.active_sessions += 1;
        self.metrics.total_sessions_created += 1;

        // Create audit event
        self.create_audit_event(AuditEvent::SessionCreated {
            session_id: session_id.clone(),
            user_id: user_id.to_string(),
            created_at: Utc::now(),
            expires_at,
        }).await?;

        Ok(SessionToken {
            session_id,
            expires_at,
            token_type: "Bearer".to_string(),
        })
    }

    /// Validate and optionally refresh a session
    pub async fn validate_and_refresh_session(&mut self, session_id: &str) -> BearDogResult<bool> {
        if let Some(session) = self.session_store.get_mut(session_id) {
            // Check if session is expired
            if Utc::now() > session.expires_at || !session.is_active {
                self.session_store.remove(session_id);
                self.metrics.active_sessions = self.metrics.active_sessions.saturating_sub(1);
                return Ok(false);
            }

            // Update last activity
            session.last_activity = Utc::now();
            
            // Optionally extend session (sliding window)
            if self.config.session_config.sliding_window {
                session.expires_at = Utc::now() + Duration::seconds(
                    self.config.session_config.max_age_seconds as i64
                );
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Revoke a specific session
    pub async fn revoke_session(&mut self, session_id: &str) -> BearDogResult<()> {
        if let Some(mut session) = self.session_store.remove(session_id) {
            session.is_active = false;
            self.metrics.active_sessions = self.metrics.active_sessions.saturating_sub(1);

            // Create audit event
            self.create_audit_event(AuditEvent::SessionRevoked {
                session_id: session_id.to_string(),
                user_id: session.user_id,
                revoked_at: Utc::now(),
            }).await?;
        }
        Ok(())
    }

    /// Revoke all sessions for a user
    pub async fn revoke_user_sessions(&mut self, user_id: &str) -> BearDogResult<u32> {
        let mut revoked_count = 0;
        let sessions_to_revoke: Vec<_> = self.session_store
            .iter()
            .filter_map(|(session_id, session)| {
                if session.user_id == user_id && session.is_active {
                    Some(session_id.clone())
                } else {
                    None
                }
            })
            .collect();

        for session_id in sessions_to_revoke {
            self.revoke_session(&session_id).await?;
            revoked_count += 1;
        }

        Ok(revoked_count)
    }

    /// Get active sessions for a user
    pub async fn get_user_sessions(&self, user_id: &str) -> Vec<SessionInfo> {
        self.session_store
            .values()
            .filter(|session| session.user_id == user_id && session.is_active)
            .map(|session| SessionInfo {
                session_id: session.session_id.clone(),
                created_at: session.created_at,
                expires_at: session.expires_at,
                last_activity: session.last_activity,
                is_active: session.is_active,
            })
            .collect()
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired_sessions(&mut self) -> BearDogResult<u32> {
        let now = Utc::now();
        let initial_count = self.session_store.len();
        
        self.session_store.retain(|_, session| {
            session.is_active && now <= session.expires_at
        });
        
        let removed_count = initial_count - self.session_store.len();
        self.metrics.active_sessions = self.session_store.len() as u64;
        
        Ok(removed_count as u32)
    }
}

/// Session information for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
    pub last_activity: chrono::DateTime<Utc>,
    pub is_active: bool,
} 