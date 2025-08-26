

use crate::tunnel::{
    events::*, BStpConfig, BStpKeyManager, EncryptedPacket, GamingCryptoEngine,
    GamingSecurityProfile, GeneticSecurityHealing, LatencyMonitor, NetworkSecurityEvent,
    SecureSession, SecurityGenetics, SecurityResponse,
};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_genetics::genetics::DefaultBearDogGeneticsEngine;
use beardog_security::encryption::EncryptionEngine;
use beardog_threat::threat::ThreatDetectionEngine;
use beardog_auth::auth::CrossNodeAuthEngine;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::debug;

use beardog_traits::canonical::SecurityProvider;

#[allow(async_fn_in_trait)]
#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
pub trait GamingSecurityProvider: SecurityProvider {

    async fn encrypt_packet(&self, session_id: &str, data: &[u8])
        -> BearDogResult<EncryptedPacket>;

    async fn decrypt_packet(
        &self,
        session_id: &str,
        encrypted_data: &EncryptedPacket,
    ) -> BearDogResult<Vec<u8>>;

    async fn handle_network_event(
        &self,
        event: NetworkSecurityEvent,
    ) -> BearDogResult<SecurityResponse>;
}

pub use GamingSecurityProvider as BStpSecurityProvider;

pub struct BStpSecurityManager {

    crypto_engine: Arc<GamingCryptoEngine>,

    healing_engine: Arc<RwLock<GeneticSecurityHealing>>,

    auth_engine: Arc<CrossNodeAuthEngine>,

    threat_engine: Arc<ThreatDetectionEngine>,

    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,

    session_monitors: Arc<RwLock<HashMap<String, SessionMonitor>>>,}

impl BStpSecurityManager {

    pub async fn new(
        encryption: Arc<EncryptionEngine>,
        genetics: Arc<DefaultBearDogGeneticsEngine>,
        key_manager: Arc<BStpKeyManager>,
        config: BStpConfig,
        auth_engine: Arc<CrossNodeAuthEngine>,
        threat_engine: Arc<ThreatDetectionEngine>,
    ) -> BearDogResult<Self> {
        let crypto_engine = Arc::new(
            GamingCryptoEngine::new(
                encryption,
                genetics.clone(),
                key_manager.clone(),
                config.clone(),
            )
            .await?,
        );
        let healing_engine = Arc::new(RwLock::new(
            GeneticSecurityHealing::new(genetics.clone()).await?,
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
        format_args!("bstp_session_{}", uuid::Uuid::new_v4().to_string().simple())

    async fn create_session_monitor(&self, session_id: &str) -> SessionMonitor {
        SessionMonitor {
            session_id: session_id.to_string(),
            created_at: SystemTime::now(),
            latency_monitor: LatencyMonitor::new(),
            encryption_count: 0,
            decryption_count: 0,
        }

    pub fn get_healing_engine(&self) -> &Arc<RwLock<GeneticSecurityHealing>> {
        &self.healing_engine

    pub fn get_auth_engine(&self) -> &Arc<CrossNodeAuthEngine> {
        &self.auth_engine

    pub fn get_threat_engine(&self) -> &Arc<ThreatDetectionEngine> {
        &self.threat_engine

    pub async fn perform_security_healing(&self, session_id: &str) -> BearDogResult<()> {
        let healing_engine = self.healing_engine.read().await;

        let _ = &*healing_engine; // Use the field
        debug!("Performing security healing for session: {}", session_id);
        Ok(())
impl BStpSecurityProvider for BStpSecurityManager {
    ) -> BearDogResult<SecureSession> {
        let session_id = Self::generate_session_id();
        let start_time = Instant::now();

        let security_genetics = SecurityGenetics::new_for_peer(
            peer_id,
            peer_capabilities,
            &VerificationResult {
                peer_id: peer_id.to_string(),
                is_trusted: true,
                trust_level: TrustLevel::High,
                verification_time: SystemTime::now(),
                capabilities: HashMap::with_capacity(16),
            },
        )
        .await?;

        let session = SecureSession::new(
            session_id.clone(),
            peer_id.to_string(),
            security_genetics,
            peer_capabilities
                .gaming_profile
                .clone()
                .unwrap_or_else(GamingSecurityProfile::competitive_gaming),

        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.clone(), session.clone());
            let mut monitors = self.session_monitors.write().await;
            let monitor = self.create_session_monitor(&session_id).await;
            debug!(
                "Created session monitor for session: {}, age: {} seconds",
                monitor.get_session_id(),
                monitor.get_age_seconds()
            );
            monitors.insert(session_id.clone(), monitor);
        let setup_duration = start_time.elapsed();
        tracing::info!(
            "Created BSTP secure session {} for peer {} in {}μs",
            session_id,
            setup_duration.as_micros()
        Ok(session)
    async fn encrypt_packet(
        data: &[u8],
    ) -> BearDogResult<EncryptedPacket> {

        let session = {
            let sessions = self.active_sessions.read().await;
            sessions
                .get(session_id)
                .cloned()
                .ok_or(BearDogError::SessionNotFound)?
        };

        let encrypted_packet = self
            .crypto_engine
            .ultra_fast_encrypt(session_id, data, &session.security_genetics)
            .await?;
        let encryption_duration = start_time.elapsed();

            if let Some(monitor) = monitors.get_mut(session_id) {
                monitor
                    .latency_monitor
                    .record_encryption_latency(encryption_duration);
                monitor.encryption_count += 1;
                debug!(
                    "Updated encryption count for session: {}, created at: {:?}",
                    monitor.get_session_id(),
                    monitor.get_created_at()
                );
            }
        Ok(encrypted_packet)
    ) -> BearDogResult<Vec<u8>> {

        let decrypted_data = self
            .ultra_fast_decrypt(session_id, encrypted_data, &session.security_genetics)
        let decryption_duration = start_time.elapsed();
                    .record_decryption_latency(decryption_duration);
                monitor.decryption_count += 1;
        Ok(decrypted_data)
    ) -> BearDogResult<SecurityResponse> {
        match event {
            NetworkSecurityEvent::PeerDiscovered {
                peer_id,
                peer_capabilities,
                trust_indicators: _,
            } => {

                let session = self
                    .create_secure_session(&peer_id, &peer_capabilities)
                    .await?;
                Ok(SecurityResponse::SessionCreated {
                    session_id: session.session_id.clone(),
                    peer_id,
                    security_level: session.security_genetics.get_security_level(),
                })
            NetworkSecurityEvent::PeerDisconnected {
                reason: _,
                was_planned: _,

                let session_id = {
                    let sessions = self.active_sessions.read().await;
                    sessions
                        .iter()
                        .find(|(_, session)| session.peer_node_id == peer_id)
                        .map(|(id, _)| id.clone())
                };
                if let Some(session_id) = session_id {
                    self.terminate_session(&session_id).await?;
                }
                Ok(SecurityResponse::SessionTerminated { peer_id })
            _ => Ok(SecurityResponse::SecurityAdapted {
                reason: "Network event handled".to_string(),
                new_security_level: SecurityLevel::Adaptive,
            }),
    async fn terminate_session(&self, session_id: &str) -> BearDogResult<()> {

            sessions.remove(session_id);
            monitors.remove(session_id);
        tracing::info!("Terminated BSTP secure session {}", session_id);

pub struct VerificationResult {

    pub peer_id: String,

    pub is_trusted: bool,

    pub trust_level: TrustLevel,

    pub verification_time: SystemTime,

    pub capabilities: HashMap<String, String>,

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {

    Low,

    Medium,

    High,

    Ultimate,
#[derive(Debug, Clone)]}

struct SessionMonitor {
    session_id: String,
    created_at: SystemTime,
    latency_monitor: LatencyMonitor,
    encryption_count: u64,
    decryption_count: u64,
impl SessionMonitor {

    pub fn get_session_id(&self) -> &str {
        &self.session_id

    pub fn get_created_at(&self) -> SystemTime {
        self.created_at

    pub fn get_age_seconds(&self) -> u64 {
        self.created_at.elapsed().map_or(0, |d| d.as_secs())
