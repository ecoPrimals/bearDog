// SPDX-License-Identifier: AGPL-3.0-or-later

//! Secure Session Management for BearDog Tunnels
//!
//! This module provides secure session lifecycle management including:
//! - Session creation and validation
//! - Expiration tracking and cleanup
//! - Security genetics for adaptive threat response
//! - Gaming-optimized security profiles
//!
//! # Thread Safety
//! All session operations are thread-safe via `RwLock` synchronization.

use crate::tunnel::events::types::SecurityLevel;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

const DEFAULT_SESSION_LIFETIME_SECS: u64 = 3600;
#[cfg_attr(
    not(test),
    allow(dead_code, reason = "used only in session-lifetime tests")
)]
const DEFAULT_EXTENDED_SESSION_LIFETIME_SECS: u64 = 7200;

/// Gaming-optimized security profile
///
/// Balances latency requirements with security needs for gaming applications.
/// Profiles are tuned for specific use cases like competitive gaming where
/// low latency is critical.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GamingSecurityProfile {
    /// Latency priority (0.0-1.0, higher = more priority)
    pub latency_priority: f64,
    /// Security level configuration
    pub security_level: SecurityLevel,
}

impl GamingSecurityProfile {
    /// Competitive Gaming operation.
    #[must_use]
    pub const fn competitive_gaming() -> Self {
        Self {
            latency_priority: 0.9,
            security_level: SecurityLevel {
                level: 3,
                authentication_strength: 85,
                threat_detection_accuracy: 0.95,
                performance_overhead: 0.15,
            },
        }
    }
}

/// Secure tunnel session
///
/// Represents an active secure tunnel session with peer authentication,
/// expiration tracking, and security genetics for adaptive protection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecureSession {
    /// Unique session identifier
    pub session_id: String,
    /// Remote peer node identifier
    pub peer_node_id: String,
    /// Timestamp when session was created
    pub created_at: SystemTime,
    /// Timestamp when session expires
    pub expires_at: SystemTime,
    /// Security genetics configuration for adaptive protection
    pub security_genetics: SecurityGenetics,
    /// Gaming-specific security profile
    pub gaming_profile: GamingSecurityProfile,
}

impl SecureSession {
    /// Creates a new secure session
    ///
    /// # Errors
    ///
    /// Returns an error if the session cannot be constructed.
    pub fn new(
        session_id: &str,
        peer_node_id: &str,
        security_genetics: SecurityGenetics,
        gaming_profile: GamingSecurityProfile,
    ) -> Result<Self, BearDogError> {
        let created_at = SystemTime::now();
        let expires_at = created_at + Duration::from_secs(DEFAULT_SESSION_LIFETIME_SECS);
        Ok(Self {
            session_id: session_id.to_string(),
            peer_node_id: peer_node_id.to_string(),
            created_at,
            expires_at,
            security_genetics,
            gaming_profile,
        })
    }

    /// Checks if expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    /// Extends the session lifetime by the specified duration
    ///
    /// # Arguments
    /// * `duration` - Time to add to the current expiration
    ///
    /// # Example
    /// ```ignore
    /// session.extend_session(Duration::from_secs(DEFAULT_SESSION_LIFETIME_SECS));
    /// ```
    pub fn extend_session(&mut self, duration: Duration) {
        self.expires_at += duration;
    }

    /// Gets remaining session time
    #[must_use]
    pub fn remaining_time(&self) -> Option<Duration> {
        self.expires_at.duration_since(SystemTime::now()).ok()
    }
}

/// Security genetics configuration
///
/// Provides adaptive security parameters that can evolve based on threat landscape.
/// Uses genetic algorithm principles for self-tuning security responses.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityGenetics {
    /// Entropy level for randomization (0.0-1.0)
    pub entropy_level: f64,
    /// Rate of security parameter mutation (0.0-1.0)
    pub mutation_rate: f64,
    /// Threshold for adaptive security adjustments (0.0-1.0)
    pub adaptive_threshold: f64,
}

impl Default for SecurityGenetics {
    fn default() -> Self {
        Self {
            entropy_level: 0.8,
            mutation_rate: 0.05,
            adaptive_threshold: 0.7,
        }
    }
}

/// Session manager for secure tunnel sessions
///
/// Manages the lifecycle of secure sessions including creation, validation,
/// key rotation, and cleanup. Thread-safe via `RwLock`.
#[derive(Debug)]
pub struct SessionManager {
    /// Active sessions indexed by session ID
    sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
}

impl SessionManager {
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates session
    ///
    /// # Errors
    ///
    /// Returns an error if the session cannot be created or the lock cannot be acquired.
    pub async fn create_session(
        &self,
        session_id: String,
        peer_node_id: String,
        security_genetics: SecurityGenetics,
        gaming_profile: GamingSecurityProfile,
    ) -> Result<(), BearDogError> {
        let session = SecureSession::new(
            &session_id,
            &peer_node_id,
            security_genetics,
            gaming_profile,
        )?;

        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, session);
        Ok(())
    }

    /// Gets session
    pub async fn get_session(&self, session_id: &str) -> Option<SecureSession> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Removes session
    pub async fn remove_session(&self, session_id: &str) -> Option<SecureSession> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id)
    }

    /// Cleans up `expired_sessions`
    ///
    /// # Errors
    ///
    /// Returns an error if the session map lock cannot be acquired.
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, BearDogError> {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();

        sessions.retain(|_, session| !session.is_expired());

        let removed_count = initial_count - sessions.len();
        Ok(removed_count)
    }

    /// Returns the total number of active sessions
    ///
    /// # Returns
    /// The count of currently active sessions in the manager
    pub async fn session_count(&self) -> usize {
        let sessions = self.sessions.read().await;
        sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_session_creation() {
        let session = SecureSession::new(
            "test-session-123",
            "peer-node-456",
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        );

        assert!(session.is_ok());
        let session = session.expect("SecureSession::new should succeed with valid inputs");
        assert_eq!(session.session_id, "test-session-123");
        assert_eq!(session.peer_node_id, "peer-node-456");
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_expiration() {
        let mut session = SecureSession::new(
            "test-session",
            "peer-node",
            SecurityGenetics::default(),
            GamingSecurityProfile::competitive_gaming(),
        )
        .expect("SecureSession::new should succeed with valid inputs");

        // Should not be expired initially
        assert!(!session.is_expired());

        // Extend session
        session.extend_session(Duration::from_secs(DEFAULT_EXTENDED_SESSION_LIFETIME_SECS));
        assert!(!session.is_expired());
    }

    #[test]
    fn test_security_genetics_default() {
        let genetics = SecurityGenetics::default();
        assert_eq!(genetics.entropy_level, 0.8);
        assert_eq!(genetics.mutation_rate, 0.05);
        assert_eq!(genetics.adaptive_threshold, 0.7);
    }

    #[test]
    fn test_gaming_security_profile_competitive() {
        let profile = GamingSecurityProfile::competitive_gaming();
        assert_eq!(profile.latency_priority, 0.9);
        assert_eq!(profile.security_level.level, 3);
        assert_eq!(profile.security_level.authentication_strength, 85);
    }

    #[tokio::test]
    async fn test_session_manager_create_session() {
        let manager = SessionManager::new();

        let result = manager
            .create_session(
                "session-123".to_string(),
                "peer-123".to_string(),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await;

        assert!(result.is_ok());
        let count = manager.session_count().await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_session_manager_get_session() {
        let manager = SessionManager::new();

        let session_id = "session-456";
        manager
            .create_session(
                session_id.to_string(),
                "peer-456".to_string(),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .expect("create_session should succeed");

        let session = manager.get_session(session_id).await;
        assert!(session.is_some());

        let session = session.expect("session should exist after create_session");
        assert_eq!(session.session_id, session_id);
        assert_eq!(session.peer_node_id, "peer-456");
    }

    #[tokio::test]
    async fn test_session_manager_remove_session() {
        let manager = SessionManager::new();

        let session_id = "session-789";
        manager
            .create_session(
                session_id.to_string(),
                "peer-789".to_string(),
                SecurityGenetics::default(),
                GamingSecurityProfile::competitive_gaming(),
            )
            .await
            .expect("create_session should succeed");

        let removed = manager.remove_session(session_id).await;
        assert!(removed.is_some());

        let session = manager.get_session(session_id).await;
        assert!(session.is_none());

        let count = manager.session_count().await;
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_session_manager_cleanup_expired() {
        let manager = SessionManager::new();

        // Create sessions
        for i in 0..5 {
            manager
                .create_session(
                    format!("session-{i}"),
                    format!("peer-{i}"),
                    SecurityGenetics::default(),
                    GamingSecurityProfile::competitive_gaming(),
                )
                .await
                .expect("create_session should succeed");
        }

        let result = manager.cleanup_expired_sessions().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_session_manager_concurrent_access() {
        let manager = Arc::new(SessionManager::new());
        let mut handles = vec![];

        // Create multiple sessions concurrently
        for i in 0..10 {
            let mgr = Arc::clone(&manager);
            let handle = tokio::spawn(async move {
                mgr.create_session(
                    format!("session-{i}"),
                    format!("peer-{i}"),
                    SecurityGenetics::default(),
                    GamingSecurityProfile::competitive_gaming(),
                )
                .await
            });
            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            let join_result = handle
                .await
                .expect("concurrent create_session task should complete");
            assert!(join_result.is_ok());
        }

        let count = manager.session_count().await;
        assert_eq!(count, 10);
    }

    #[tokio::test]
    async fn test_session_remaining_time() {
        let genetics = SecurityGenetics::default();
        let profile = GamingSecurityProfile::competitive_gaming();

        let session = SecureSession::new("test-session", "peer-123", genetics, profile)
            .expect("SecureSession::new should succeed with valid inputs");

        // Should have remaining time
        let remaining = session.remaining_time();
        assert!(remaining.is_some());

        // Should be close to 1 hour (3600 seconds)
        let duration = remaining.expect("non-expired session should have remaining time");
        assert!(
            duration.as_secs() > DEFAULT_SESSION_LIFETIME_SECS.saturating_sub(10)
                && duration.as_secs() <= DEFAULT_SESSION_LIFETIME_SECS
        );
    }

    #[tokio::test]
    async fn test_session_extension() {
        let genetics = SecurityGenetics::default();
        let profile = GamingSecurityProfile::competitive_gaming();

        let mut session = SecureSession::new("test-session", "peer-123", genetics, profile)
            .expect("SecureSession::new should succeed with valid inputs");

        let initial_remaining = session
            .remaining_time()
            .expect("non-expired session should have remaining time");

        // Extend by 1 hour
        session.extend_session(Duration::from_secs(DEFAULT_SESSION_LIFETIME_SECS));

        let after_extension = session
            .remaining_time()
            .expect("extended session should have remaining time");

        // Should have approximately 1 more hour
        assert!(after_extension > initial_remaining);
        assert!(after_extension.as_secs() > DEFAULT_SESSION_LIFETIME_SECS * 2 - 200);
    }

    #[tokio::test]
    async fn test_expired_session_no_remaining_time() {
        let genetics = SecurityGenetics::default();
        let profile = GamingSecurityProfile::competitive_gaming();

        let mut session = SecureSession::new("test-session", "peer-123", genetics, profile)
            .expect("SecureSession::new should succeed with valid inputs");

        // Set expiration to the past
        session.expires_at = SystemTime::now() - Duration::from_secs(60);

        // Should have no remaining time
        assert!(session.remaining_time().is_none());
        assert!(session.is_expired());
    }

    #[tokio::test]
    async fn test_security_genetics_default_values() {
        let genetics = SecurityGenetics::default();

        assert_eq!(genetics.entropy_level, 0.8);
        assert_eq!(genetics.mutation_rate, 0.05);
        assert_eq!(genetics.adaptive_threshold, 0.7);
    }

    #[tokio::test]
    async fn test_competitive_gaming_profile() {
        let profile = GamingSecurityProfile::competitive_gaming();

        assert_eq!(profile.latency_priority, 0.9);
        assert_eq!(profile.security_level.level, 3);
        assert_eq!(profile.security_level.authentication_strength, 85);
        assert_eq!(profile.security_level.threat_detection_accuracy, 0.95);
        assert_eq!(profile.security_level.performance_overhead, 0.15);
    }
}
