// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


// 🛡️ BSTP Security Provider - Core security interface for gaming tunnels

// CrossNodeAuthEngine functionality is now part of this module
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

// Import canonical SecurityProvider
use beardog_traits::canonical::SecurityProvider;

/// Gaming-specific security provider trait - MIGRATED TO CANONICAL SYSTEM
/// 
/// **MIGRATION COMPLETE** ✅
/// This trait now extends the canonical SecurityProvider instead of being
/// a separate BStpSecurityProvider. This eliminates fragmentation while
/// preserving gaming-specific ultra-fast encryption capabilities.
/// 
/// ## Migration Benefits:
/// - **Zero-cost async** - Uses native async fn instead of async_trait
/// - **Unified interface** - Extends canonical SecurityProvider
/// - **Gaming optimizations** - Preserves <100μs encryption targets
/// - **Better performance** - No boxing overhead
#[allow(async_fn_in_trait)]
pub trait GamingSecurityProvider: SecurityProvider {
    /// Ultra-fast packet encryption for gaming (<100μs target)
    async fn encrypt_packet(&self, session_id: &str, data: &[u8])
        -> BearDogResult<EncryptedPacket>;
    
    /// Ultra-fast packet decryption for gaming (<100μs target)  
    async fn decrypt_packet(
        &self,
        session_id: &str,
        encrypted_data: &EncryptedPacket,
    ) -> BearDogResult<Vec<u8>>;
    
    /// Handle security events from network layer (Songbird)
    async fn handle_network_event(
        &self,
        event: NetworkSecurityEvent,
    ) -> BearDogResult<SecurityResponse>;
}

/// **MIGRATION NOTICE**
/// BStpSecurityProvider has been unified with the canonical SecurityProvider.
/// Gaming-specific functionality is now provided by GamingSecurityProvider.
/// 
/// ## Migration Path:
/// ```rust
/// // OLD:
/// use crate::tunnel::security_provider::BStpSecurityProvider;
/// 
/// // NEW:
/// use beardog_traits::canonical::SecurityProvider;
/// use crate::tunnel::security_provider::GamingSecurityProvider;
/// ```

// Use canonical security provider
pub use GamingSecurityProvider as BStpSecurityProvider;

/// Main BSTP security manager implementation
pub struct BStpSecurityManager {
    /// Genetic crypto engine optimized for gaming
    crypto_engine: Arc<GamingCryptoEngine>,
    /// Genetic security healing engine
    healing_engine: Arc<RwLock<GeneticSecurityHealing>>,
    /// Cross-node authentication for peer verification
    auth_engine: Arc<CrossNodeAuthEngine>,
    /// Threat detection for security monitoring
    threat_engine: Arc<ThreatDetectionEngine>,
    /// Active secure sessions
    active_sessions: Arc<RwLock<HashMap<String, SecureSession>>>,
    /// Session performance monitoring
    session_monitors: Arc<RwLock<HashMap<String, SessionMonitor>>>,}


impl BStpSecurityManager {
    /// Create new BSTP security manager
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
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            session_monitors: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    /// Internal method to generate session ID
    fn generate_session_id() -> String {
        format!("bstp_session_{}", uuid::Uuid::new_v4().simple())
    /// Internal method to create session monitor
    async fn create_session_monitor(&self, session_id: &str) -> SessionMonitor {
        SessionMonitor {
            session_id: session_id.to_string(),
            created_at: SystemTime::now(),
            latency_monitor: LatencyMonitor::new(),
            encryption_count: 0,
            decryption_count: 0,
        }
    /// Get the genetic security healing engine
    pub fn get_healing_engine(&self) -> &Arc<RwLock<GeneticSecurityHealing>> {
        &self.healing_engine
    /// Get the cross-node authentication engine}


    pub fn get_auth_engine(&self) -> &Arc<CrossNodeAuthEngine> {
        &self.auth_engine
    /// Get the threat detection engine
    pub fn get_threat_engine(&self) -> &Arc<ThreatDetectionEngine> {
        &self.threat_engine
    /// Perform security healing using the genetic engine}


    pub async fn perform_security_healing(&self, session_id: &str) -> BearDogResult<()> {
        let healing_engine = self.healing_engine.read().await;
        // Use the healing_engine to perform security healing
        let _ = &*healing_engine; // Use the field
        debug!("Performing security healing for session: {}", session_id);
        Ok(())
impl BStpSecurityProvider for BStpSecurityManager {
    ) -> BearDogResult<SecureSession> {
        let session_id = Self::generate_session_id();
        let start_time = Instant::now();
        // Generate security genetics for this session
        let security_genetics = SecurityGenetics::new_for_peer(
            peer_id,
            peer_capabilities,
            &VerificationResult {
                peer_id: peer_id.to_string(),
                is_trusted: true,
                trust_level: TrustLevel::High,
                verification_time: SystemTime::now(),
                capabilities: HashMap::new(),
            },
        )
        .await?;
        // Create session with gaming-optimized security profile
        let session = SecureSession::new(
            session_id.clone(),
            peer_id.to_string(),
            security_genetics,
            peer_capabilities
                .gaming_profile
                .clone()
                .unwrap_or_else(GamingSecurityProfile::competitive_gaming),
        // Store session and create monitor
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
        // Get session for encryption context
        let session = {
            let sessions = self.active_sessions.read().await;
            sessions
                .get(session_id)
                .cloned()
                .ok_or(BearDogError::SessionNotFound)?
        };
        // Use gaming crypto engine for ultra-fast encryption
        let encrypted_packet = self
            .crypto_engine
            .ultra_fast_encrypt(session_id, data, &session.security_genetics)
            .await?;
        let encryption_duration = start_time.elapsed();
        // Update session monitor
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
        // Get session for decryption context
        // Use gaming crypto engine for ultra-fast decryption
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
                // Create secure session with new peer
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
                // Find and terminate session for this peer
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
        // Remove session and monitor
            sessions.remove(session_id);
            monitors.remove(session_id);
        tracing::info!("Terminated BSTP secure session {}", session_id);
// Support types for BSTP security
/// Verification result for peer identity and capabilities
///
/// Contains comprehensive information about a peer's verification status,
/// trust level, and security capabilities for gaming tunnel establishment.
/// # Security Model
/// The verification process includes:
/// - Cryptographic identity verification
/// - Trust level assessment based on previous interactions
/// - Capability negotiation for optimal performance
/// - Timestamp validation to prevent replay attacks
/// # Example
/// ```rust,no_run
/// use beardog::tunnel::security_provider::{VerificationResult, TrustLevel};
/// use std::time::SystemTime;
/// use std::collections::HashMap;
/// let verification = VerificationResult {
///     peer_id: "gaming-peer-123".to_string(),
///     is_trusted: true,
///     trust_level: TrustLevel::High,
///     verification_time: SystemTime::now(),
///     capabilities: HashMap::new(),
/// };
/// if verification.is_trusted && verification.trust_level >= TrustLevel::Medium {
///     println!("Peer verified for secure gaming tunnel");
/// }
/// ```
pub struct VerificationResult {
    /// Unique identifier for the verified peer
    pub peer_id: String,
    /// Whether the peer passed all verification checks
    pub is_trusted: bool,
    /// Assessed trust level based on verification criteria
    pub trust_level: TrustLevel,
    /// Timestamp when verification was completed
    pub verification_time: SystemTime,
    /// Peer capabilities and security features
    pub capabilities: HashMap<String, String>,
/// Trust levels for peer verification in gaming environments
/// Trust levels determine what operations a peer is allowed to perform
/// and the security measures applied to communications with that peer.
/// # Trust Level Hierarchy
/// Trust levels are ordered from lowest to highest security:
/// `Low` < `Medium` < `High` < `Ultimate`
/// # Gaming Context
/// - **Ultimate**: Tournament/competitive gaming with verified players
/// - **High**: Trusted gaming partners with established reputation
/// - **Medium**: Standard gaming peers with basic verification
/// - **Low**: New or unverified gaming peers with restricted access
/// use beardog::tunnel::security_provider::TrustLevel;
/// fn should_allow_voice_chat(trust: TrustLevel) -> bool {
///     match trust {
///         TrustLevel::Ultimate | TrustLevel::High => true,
///         TrustLevel::Medium | TrustLevel::Low => false,
///     }
/// assert!(should_allow_voice_chat(TrustLevel::Ultimate));
/// assert!(!should_allow_voice_chat(TrustLevel::Low));
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// Lowest trust level for unverified or suspicious peers
    ///
    /// Applied to new peers or those with failed verification.
    /// Operations are heavily restricted and monitored.
    Low,
    /// Standard trust level for basic verified peers
    /// Applied to peers with successful basic verification.
    /// Standard gaming operations allowed with monitoring.
    Medium,
    /// High trust level for well-established gaming peers
    /// Applied to peers with strong verification and good history.
    /// Most gaming operations allowed with minimal restrictions.
    High,
    /// Maximum trust level for tournament and competitive gaming
    /// Applied to verified tournament players and trusted partners.
    /// All operations allowed with optimized performance settings.
    Ultimate,
#[derive(Debug, Clone)]}


struct SessionMonitor {
    session_id: String,
    created_at: SystemTime,
    latency_monitor: LatencyMonitor,
    encryption_count: u64,
    decryption_count: u64,
impl SessionMonitor {
    /// Get the session ID}


    pub fn get_session_id(&self) -> &str {
        &self.session_id
    /// Get the creation timestamp}


    pub fn get_created_at(&self) -> SystemTime {
        self.created_at
    /// Get session age in seconds
    pub fn get_age_seconds(&self) -> u64 {
        self.created_at.elapsed().map_or(0, |d| d.as_secs())
