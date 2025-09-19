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
    pub fn competitive_gaming() -> Self {
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
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates session
    /// Creates session
    pub fn create_session(
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

        let mut sessions = self.sessions.write();
        sessions.insert(session_id, session);
        Ok(())
    }

    /// Gets session
    /// Gets session
    pub fn get_session(&self, session_id: &str) -> Option<SecureSession> {
        let sessions = self.sessions.read();
        sessions.get(session_id).cloned()
    }

    /// Removes session
    /// Removes session
    pub fn remove_session(&self, session_id: &str) -> Option<SecureSession> {
        let mut sessions = self.sessions.write();
        sessions.remove(session_id)
    }

    /// Cleans up expired_sessions
    /// Cleans up expired_sessions
    pub fn cleanup_expired_sessions(&self) -> Result<usize, BearDogError> {
        let mut sessions = self.sessions.write();
        let initial_count = sessions.len();

        sessions.retain(|_, session| !session.is_expired());

        let removed_count = initial_count - sessions.len();
        Ok(removed_count)
    }

    pub fn session_count(&self) -> usize {
        let sessions = self.sessions.read();
        sessions.len()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}
