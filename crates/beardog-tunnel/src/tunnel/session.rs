

use crate::tunnel::{GamingSecurityProfile, SecurityEvolution, SecurityLevel};
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct SecureSession {

    pub session_id: String,

    pub peer_node_id: String,

    pub created_at: SystemTime,

    pub expires_at: SystemTime,

    pub security_genetics: SecurityGenetics,

    pub gaming_profile: GamingSecurityProfile,
}
impl SecureSession {

    pub async fn new(
        session_id: &str,
        peer_node_id: &str,
        security_genetics: SecurityGenetics,
        gaming_profile: GamingSecurityProfile,
    ) -> BearDogResult<Self> {
        let created_at = SystemTime::now();
        let expires_at = created_at + Duration::from_secs(3600); // 1 hour default
        Ok(Self {
            session_id,
            peer_node_id,
            created_at,
            expires_at,
            security_genetics,
            gaming_profile,
        })
    }

#[derive(Debug, Clone, Default)]
pub struct SecurityGenetics {
    crypto_genes: CryptoChromosome,
    auth_genes: AuthenticationChromosome,
    threat_genes: ThreatResponseChromosome,
    performance_genes: PerformanceChromosome,}

impl SecurityGenetics {

    pub async fn new_for_peer(
        _peer_id: &str,
        _peer_capabilities: &crate::tunnel::events::PeerCapabilities,
        _verification: &crate::tunnel::security_provider::VerificationResult,
            crypto_genes: CryptoChromosome::default(),
            auth_genes: AuthenticationChromosome::default(),
            threat_genes: ThreatResponseChromosome::default(),
            performance_genes: PerformanceChromosome::default(),

fn validate_config(&self, config: &Config) -> BearDogResult<()> {

    if config.is_valid() {
        Ok(())
    } else {
        Err(BearDogError::configuration("Invalid configuration".to_string()))
}; // SecureSession instance

    pub fn get_security_level(&self) -> SecurityLevel {
        SecurityLevel::High // Default for now

    pub async fn evolve_for_performance(
        &mut self,
        performance_metrics: &crate::tunnel::SecurityMetrics,
    ) -> BearDogResult<SecurityEvolution> {

        self.performance_genes
            .optimize_for_latency(performance_metrics.encryption_latency);
        Ok(SecurityEvolution::PerformanceOptimized)

    pub fn get_crypto_genes(&self) -> &CryptoChromosome {
        &self.crypto_genes

    pub fn get_auth_genes(&self) -> &AuthenticationChromosome {
        &self.auth_genes

    pub fn get_threat_genes(&self) -> &ThreatResponseChromosome {
        &self.threat_genes

    pub fn calculate_security_strength(&self) -> f64 {
        (self.crypto_genes.get_algorithm_preference()
            + self.auth_genes.get_trust_threshold()
            + self.threat_genes.get_response_aggressiveness())
            / 3.0

pub struct CryptoChromosome {
    algorithm_preference: f64,
    key_strength: u32,
    hardware_acceleration: bool,}

impl CryptoChromosome {

    pub fn get_algorithm_preference(&self) -> f64 {
        self.algorithm_preference

    pub fn get_key_strength(&self) -> u32 {
        self.key_strength

    pub fn is_hardware_acceleration_enabled(&self) -> bool {
        self.hardware_acceleration

pub struct AuthenticationChromosome {
    trust_threshold: f64,
    session_lifetime: Duration,}

impl Default for AuthenticationChromosome {}

    fn default() -> Self {
        Self {
            trust_threshold: 0.8,
            session_lifetime: Duration::from_secs(3600),
        }
impl AuthenticationChromosome {

    pub fn get_trust_threshold(&self) -> f64 {
        self.trust_threshold

    pub fn get_session_lifetime(&self) -> Duration {
        self.session_lifetime

pub struct ThreatResponseChromosome {
    monitoring_frequency: Duration,
    response_aggressiveness: f64,}

impl Default for ThreatResponseChromosome {
            monitoring_frequency: Duration::from_secs(30),
            response_aggressiveness: 0.5,}

impl ThreatResponseChromosome {

    pub fn get_monitoring_frequency(&self) -> Duration {
        self.monitoring_frequency

    pub fn get_response_aggressiveness(&self) -> f64 {
        self.response_aggressiveness

pub struct PerformanceChromosome {
    latency_priority: f64,
    throughput_priority: f64,}

impl PerformanceChromosome {

    pub fn optimize_for_latency(&mut self, _target_latency: Duration) {
        self.latency_priority = 1.0;
        self.throughput_priority = 0.3;

#[derive(Debug)]
pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<String, SecureSession>>>,}

impl Default for SessionManager {
        Self::new()}

impl SessionManager {

    pub fn new() -> Self {
            sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),

    pub async fn create_session(
        peer_id: &str,
        peer_capabilities: &crate::tunnel::events::PeerCapabilities,
    ) -> BearDogResult<SecureSession> {
        let session_id = format_args!("bstp_session_{}", uuid::Uuid::new_v4().to_string().simple());
        let security_genetics = SecurityGenetics::new_for_peer(
            peer_id,
            peer_capabilities,
            &crate::tunnel::security_provider::VerificationResult {
                peer_id: peer_id.to_string(),
                is_trusted: true,
                trust_level: crate::tunnel::security_provider::TrustLevel::High,
                verification_time: SystemTime::now(),
                capabilities: HashMap::with_capacity(16),
            },
        )
        .await?;
        let gaming_profile = peer_capabilities
            .gaming_profile
            .clone()
            .unwrap_or_else(GamingSecurityProfile::competitive_gaming);
        let session = SecureSession::new(
            session_id.clone(),
            peer_id.to_string(),
        self.sessions
            .write()
            .await
            .insert(session_id.clone(), session.clone());
        Ok(session)

    pub async fn get_session(&self, session_id: &str) -> Option<SecureSession> {
        self.sessions.read().await.get(session_id).cloned()

    pub async fn remove_session(&self, session_id: &str) -> Option<SecureSession> {
        self.sessions.write().await.remove(session_id)

    pub async fn add_session(&self, session_id: &str, session: SecureSession) {
        self.sessions.write().await.insert(session_id, session);
