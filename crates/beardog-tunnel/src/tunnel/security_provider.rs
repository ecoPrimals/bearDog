

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::{
    events::*, BStpConfig, BStpKeyManager, EncryptedPacket, GamingCryptoEngine,
    GamingSecurityProfile, GeneticSecurityHealing, LatencyMonitor, NetworkSecurityEvent,
    SecureSession, SecurityGenetics, SecurityResponse,
};
use beardog_errors::BearDogError;
use beardog_genetics::genetics::DefaultBearDogGeneticsEngine;
use beardog_security::encryption::EncryptionEngine;
use beardog_threat::threat::ThreatDetectionEngine;
use beardog_auth::auth::CrossNodeAuthEngine;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::debug;

use beardog_traits::unified::SecurityProvider;

pub use beardog_traits::unified::PlatformProvider as BStpSecurityProvider;

pub struct BStpSecurityManager {

    crypto_engine: Arc<GamingCryptoEngine>,

    healing_engine: Arc<RwLock<GeneticSecurityHealing>>,

    auth_engine: Arc<CrossNodeAuthEngine>,

    threat_engine: Arc<ThreatDetectionEngine>,

    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,

    session_monitors: Arc<RwLock<HashMap<String, SessionMonitor>>>,}

impl BStpSecurityManager {

/// New operation.
    /// Creates a new instance
    pub async fn new(Arc<EncryptionEngine>,
        genetics: Arc<DefaultBearDogGeneticsEngine>,
        key_manager: Arc<BStpKeyManager>,
        config: BStpConfig,
        auth_engine: Arc<CrossNodeAuthEngine>,
        threat_engine: Arc<ThreatDetectionEngine>,
    ) -> Result<Self, BearDogError> {
        let crypto_engine = Arc::new(
            GamingCryptoEngine::new(
                encryption,
                genetics.clone(),
                key_manager.clone(),
                config.clone(),
            )
            ?,
        );
        let healing_engine = Arc::new(&RwLock::new(
            GeneticSecurityHealing::new(genetics)?,
        ));
        Ok(Self {
            crypto_engine,
            healing_engine,
            auth_engine,
            threat_engine,
            active_sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            session_monitors: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }


    fn generate_session_id() -> String {
        format!("bstp_session_{}", uuid::Uuid::new_v4().simple())
    }

    /// Creates session_monitor
    fn create_session_monitor(&self, session_id: &str) -> SessionMonitor {
        SessionMonitor {
            session_id: session_id.to_string(),
            created_at: SystemTime::now(),
            latency_monitor: LatencyMonitor::new(0,
            decryption_count: 0,
        }
    }

/// Get Healing Engine operation.
    /// Gets healing_engine
    /// Gets healing_engine
    pub fn get_healing_engine(&self) -> &Arc<RwLock<GeneticSecurityHealing>> {
        &self.healing_engine
    }

/// Get Auth Engine operation.
    /// Gets auth_engine
    /// Gets auth_engine
    pub fn get_auth_engine(&self) -> &Arc<CrossNodeAuthEngine> {
        &self.auth_engine
    }

/// Get Threat Engine operation.
    /// Gets threat_engine
    /// Gets threat_engine
    pub fn get_threat_engine(&self) -> &Arc<ThreatDetectionEngine> {
        &self.threat_engine
    }

///
/// # Errors
/// Returns an error if the operation fails.
    pub fn perform_security_healing(&self, session_id: &str) -> Result<(), BearDogError> {
        let healing_engine = self.healing_engine.read({}", session_id);
        Ok(&str,
        peer_capabilities: &[&str],
    ) -> Result<SecureSession, BearDogError> {
        let session_id = Self::generate_session_id();
        let start_time = Instant::now();

        let security_genetics = SecurityGenetics::new_for_peer(
            peer_id,
            peer_capabilities,
            &VerificationResult {
                peer_id: peer_id.to_string(),
                verification_time: SystemTime::now(),
                capabilities: HashMap::with_capacity(16),
            },
        )
        ?;

        let session = SecureSession::new(
            session_id.clone(),
            peer_id.to_string(),
            security_genetics,
            peer_capabilities
                .gaming_profile
                .clone()
                .unwrap_or_else(GamingSecurityProfile::competitive_gaming),
        );

        {
            let mut sessions = self.active_sessions.write({}, age: {} seconds",
                monitor.get_session_id(),
                monitor.get_age_seconds()
            );
            monitors.insert(session_id.clone(), monitor);
        }
        let setup_duration = start_time.elapsed();
        tracing::info!(
            "Created BSTP secure session {} for peer {} in {}μs",
            session_id: session_id.to_string(&str,
        data: &[u8],
    ) -> Result<EncryptedPacket, BearDogError> {

        let session = {
            let sessions = self.active_sessions.read();
            sessions
                .get(session_id)
                .cloned()
                .ok_or(BearDogError::SessionNotFound)?
        };

        let encrypted_packet = self
            .crypto_engine
            .ultra_fast_encrypt({}, created at: {:?}",
                    monitor.get_session_id(&str,
        encrypted_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {

        let session = {
            let sessions = self.active_sessions.read();
            sessions
                .get(session_id)
                .cloned()
                .ok_or(BearDogError::SessionNotFound)?
        };

        let decrypted_data = self
            .crypto_engine
            .ultra_fast_decrypt({}, created at: {:?}",
                    monitor.get_session_id(NetworkSecurityEvent,
    ) -> Result<SecurityResponse, BearDogError> {
        match event {
            NetworkSecurityEvent::PeerDiscovered {
                peer_id,
                peer_capabilities,
                trust_indicators: _,
            } => {

                let session = self
                    .create_secure_session(&peer_id, &peer_capabilities)
                    ?;
                Ok(SecurityResponse::SessionCreated {
                    session_id: &session.session_id.to_string(),
                    peer_id,
                    security_level: session.security_genetics.get_security_level(),
                })
            }
            NetworkSecurityEvent::PeerDisconnected {
                peer_id,
                reason: _,
                was_planned: _,
            } => {
                let session_id = {
                    let sessions = self.active_sessions.read();
                    sessions
                        .iter()
                        .find(|(_, session)| session.peer_node_id == peer_id)
                        .map(|(id, _)| id.clone())
                };
                if let Some(session_id) = session_id {
                    self.terminate_session(&session_id)?;
                }
                Ok(SecurityResponse::SessionTerminated { peer_id })
            }
            _ => Ok(SecurityResponse::SecurityAdapted {
                reason: "Network event handled".to_string(),
            }),
        }
    }


    fn terminate_session(&self, session_id: &str) -> Result<(), BearDogError> {
        let mut sessions = self.active_sessions.write();
        let mut monitors = self.session_monitors.write();
        sessions.remove(session_id);
        monitors.remove(session_id);
        tracing::info!("Terminated BSTP secure session {}", session_id);
        Ok(String,
    /// Whether is_trusted is enabled
    pub is_trusted: bool,
    /// The trust level value
    pub trust_level: TrustLevel,
    pub verification_time: SystemTime,
    /// Mapping of capabilities
    pub capabilities: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    created_at: SystemTime,
    latency_monitor: LatencyMonitor,
    encryption_count: u64,
    decryption_count: u64,
}

impl SessionMonitor {
/// Get Session Id operation.
    /// Gets session_id
    /// Gets session_id
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

/// Get Created At operation.
    /// Gets created_at
    /// Gets created_at
    pub fn get_created_at(&self) -> SystemTime {
        self.created_at
    }

/// Get Age Seconds operation.
    /// Gets age_seconds
    /// Gets age_seconds
    pub fn get_age_seconds(&self) -> u64 {
        self.created_at.elapsed().map_or(0, |d| d.as_secs())
    }
}
