// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::tunnel::events::types::SecurityLevel;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GamingSecurityProfile {
    /// The latency priority value
    pub latency_priority: f64,
    /// The security level value
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecureSession {
    pub session_id: String,
    pub peer_node_id: String,
    /// The created at value
    pub created_at: SystemTime,
    /// The expires at value
    pub expires_at: SystemTime,
    /// The security genetics value
    pub security_genetics: SecurityGenetics,
    /// The gaming profile value
    pub gaming_profile: GamingSecurityProfile,
}

impl SecureSession {
    /// New operation.
    /// Creates a new instance
    pub fn new(
        session_id: &str,
        peer_node_id: &str,
        security_genetics: SecurityGenetics,
        gaming_profile: GamingSecurityProfile,
    ) -> Result<Self, BearDogError> {
        let created_at = SystemTime::now();
        let expires_at = created_at + Duration::from_secs(3600); // 1 hour default
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
    /// Checks if expired
    #[must_use]
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.expires_at
    }

    pub fn extend_session(&mut self, duration: Duration) {
        self.expires_at += duration;
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityGenetics {
    /// The entropy level value
    pub entropy_level: f64,
    /// The mutation rate value
    pub mutation_rate: f64,
    /// The adaptive threshold value
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

#[derive(Debug)]
pub struct SessionManager {
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
    /// Creates session
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
    /// Gets session
    pub async fn get_session(&self, session_id: &str) -> Option<SecureSession> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Removes session
    /// Removes session
    pub async fn remove_session(&self, session_id: &str) -> Option<SecureSession> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id)
    }

    /// Cleans up `expired_sessions`
    /// Cleans up `expired_sessions`
    pub async fn cleanup_expired_sessions(&self) -> Result<usize, BearDogError> {
        let mut sessions = self.sessions.write().await;
        let initial_count = sessions.len();

        sessions.retain(|_, session| !session.is_expired());

        let removed_count = initial_count - sessions.len();
        Ok(removed_count)
    }

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
        let session = session.unwrap();
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
        .unwrap();

        // Should not be expired initially
        assert!(!session.is_expired());

        // Extend session
        session.extend_session(Duration::from_secs(7200));
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
            .unwrap();

        let session = manager.get_session(session_id).await;
        assert!(session.is_some());

        let session = session.unwrap();
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
            .unwrap();

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
                    format!("session-{}", i),
                    format!("peer-{}", i),
                    SecurityGenetics::default(),
                    GamingSecurityProfile::competitive_gaming(),
                )
                .await
                .unwrap();
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
                    format!("session-{}", i),
                    format!("peer-{}", i),
                    SecurityGenetics::default(),
                    GamingSecurityProfile::competitive_gaming(),
                )
                .await
            });
            handles.push(handle);
        }

        // Wait for all to complete
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }

        let count = manager.session_count().await;
        assert_eq!(count, 10);
    }
}
