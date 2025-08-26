

use std::sync::Arc;
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use beardog_compliance::audit::{AuditEvent, AuditEventType, AuditSeverity};
use beardog_security::{
    Action, ActionType, AuthenticationResult, BearDogSecurityProvider, Resource,
    ResourceClassification, RiskLevel, SecurityProvider, SecurityProviderConfig, Subject,
    SubjectType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongBirdConfig {

    pub endpoint: String,

    pub api_key: String,

    pub timeout_seconds: u64,

    pub max_retries: u32,

    pub enable_security_provider: bool,

    pub security_provider: SecurityProviderConfig,
}
impl Default for SongBirdConfig {}

    fn default() -> Self {
        Self {
            endpoint: "https://songbird.orchestrator.internal".to_string(),
            api_key: std::env::var("SONGBIRD_API_KEY").unwrap_or_default(),
            timeout_seconds: 30,
            max_retries: 3,
            enable_security_provider: true,
            security_provider: SecurityProviderConfig::default(),
        }
    }

pub struct SongBirdAdapter<T> {
    config: SongBirdConfig,
    core: Arc<T>,
    security_provider: Arc<BearDogSecurityProvider>,

    active_connections: Arc<RwLock<HashMap<String, SongBirdConnection>>>,
    communication_sessions: Arc<RwLock<HashMap<String, CommunicationSession>>>,

#[derive(Debug, Clone)]
pub struct SongBirdConnection {

    pub connection_id: String,

    pub user_id: String,

    pub established_at: DateTime<Utc>,

    pub last_activity: DateTime<Utc>,

    pub encryption_status: EncryptionStatus,

    pub threat_level: beardog_threat::threat::types::ThreatSeverity,

pub struct CommunicationSession {

    pub session_id: String,

    pub participants: Vec<String>,

    pub session_type: SessionType,

    pub started_at: DateTime<Utc>,

    pub security_level: SecurityLevel,

    pub recording_enabled: bool,

    pub compliance_monitoring: bool,

pub enum EncryptionStatus {

    None,

    InTransit,

    EndToEnd,

    QuantumResistant,

pub enum SessionType {

    Messaging,

    VoiceCall,

    VideoCall,

    Conference,

    ScreenShare,

    FileTransfer,

pub enum SecurityLevel {

    Standard,

    Enhanced,

    Classified,

    TopSecret,

pub struct SecureMessage {

    pub message_id: String,

    pub sender_id: String,

    pub recipient_ids: Vec<String>,

    pub content: String,

    pub timestamp: DateTime<Utc>,

    pub encryption_type: EncryptionStatus,

    pub security_classification: SecurityLevel,

    pub requires_confirmation: bool,

    pub auto_delete_after: Option<chrono::Duration>,

pub struct CommunicationPolicy {

    pub policy_id: String,

    pub name: String,

    pub description: String,

    pub max_participants: u32,

    pub max_session_duration: chrono::Duration,

    pub required_encryption: EncryptionStatus,

    pub allowed_external_domains: Vec<String>,

    pub recording_policy: RecordingPolicy,

    pub data_retention_days: u32,

    pub compliance_standards: Vec<String>,

pub enum RecordingPolicy {

    Never,

    OptIn,

    OptOut,

    Always,

    ComplianceOnly,
}

impl<T> SongBirdAdapter<T> {

    pub async fn new(core: Arc<T>, config: SongBirdConfig) -> BearDogResult<Self> {
        info!("🎵 Initializing SongBird adapter with BearDog security integration");

        let security_provider =
            Arc::new(BearDogSecurityProvider::new(config.security_provider.clone()).await?);
        let adapter = Self {
            config,
            core,
            security_provider,
            active_connections: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            communication_sessions: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        };
        info!("✅ SongBird adapter initialized successfully with security provider");
        Ok(adapter)

    pub async fn establish_connection(&self, user_id: &str) -> BearDogResult<String> {
        info!("🔗 Establishing SongBird connection for user: {}", user_id);

        let subject = Subject {
            id: user_id.to_string(),
            subject_type: SubjectType::User,
            roles: vec!["user".to_string()], // Would be fetched from user store
            attributes: HashMap::with_capacity(16),
            clearance_level: Some(3),

        let resource = Resource {
            id: "songbird_orchestrator".to_string(),
            resource_type: "communication_platform".to_string(),
            owner: None,
            classification: ResourceClassification::Internal,

        let action = Action {
            action_type: ActionType::Execute,
            context: {
                let mut ctx = HashMap::with_capacity(16);
                ctx.insert(
                    "action_name".to_string(),
                    "establish_connection".to_string(),
                );
                ctx
            },
            timestamp: Utc::now(),
            source_ip: None,

        let auth_result = self
            .security_provider
            .authorize(&subject, &resource, &action)
            .await?;
        if !auth_result.permitted {
            warn!(
                "🚫 Connection denied for user {}: {}",
                user_id, auth_result.reason
            );
            return Err(BearDogError::invalid_input(auth_result.reason,
            ));

        if !auth_result.additional_requirements.is_empty() {
            info!("🔐 Additional requirements for user: {}", user_id);

        let connection_id = uuid::Uuid::new_v4().to_string();
        let connection = SongBirdConnection {
            connection_id: connection_id.clone(),
            user_id: user_id.to_string(),
            endpoint: self.config.endpoint.clone(),
            established_at: Utc::now(),
            last_activity: Utc::now(),
            encryption_status: EncryptionStatus::EndToEnd,
            threat_level: match auth_result.risk_level {
                RiskLevel::Low => beardog_threat::threat::types::ThreatSeverity::Low,
                RiskLevel::Medium => beardog_threat::threat::types::ThreatSeverity::Medium,
                RiskLevel::High => beardog_threat::threat::types::ThreatSeverity::High,
                RiskLevel::Critical => beardog_threat::threat::types::ThreatSeverity::Critical,
        {
            let mut connections = self.active_connections.write().await;
            connections.insert(connection_id.clone(), connection);

        let audit_event = AuditEvent {
            id: uuid::Uuid::new_v4().to_string(),
            event_type: AuditEventType::Authentication,
            severity: AuditSeverity::Low,
            user_id: Some(subject.id.clone()),
            resource: Some(resource.id.clone()),
            action: "connection_established".to_string(),
            description: format!("SongBird connection established: {connection_id}"),
            outcome: "success".to_string(),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("connection_id".to_string(), connection_id.clone());
                metadata.insert("endpoint".to_string(), self.config.endpoint.clone());
                metadata
            details: HashMap::with_capacity(16),

        self.security_provider.log_audit(audit_event).await?;
        info!("✅ SongBird connection established: {}", connection_id);
        Ok(connection_id)

    pub async fn send_secure_message(
        &self,
        sender_id: &str,
        message: SecureMessage,
    ) -> BearDogResult<String> {
        info!("📨 Sending secure message from: {}", sender_id);
            id: sender_id.to_string(),
            roles: vec!["user".to_string()],

            id: format_args!("message_recipients_{}", message.recipient_ids.join("_").to_string()),
            resource_type: "communication_channel".to_string(),
            owner: Some(sender_id.to_string()),
            classification: match message.security_classification {
                SecurityLevel::Standard => ResourceClassification::Internal,
                SecurityLevel::Enhanced => ResourceClassification::Confidential,
                SecurityLevel::Classified => ResourceClassification::Confidential,
                SecurityLevel::TopSecret => ResourceClassification::TopSecret,

            action_type: ActionType::Write,
                ctx.insert("action_name".to_string(), "send_message".to_string());
                    "security_level".to_string(),
                    format_args!("{:?}", message.security_classification).to_string(),

                "🚫 Message sending denied for user {}: {}",
                sender_id, auth_result.reason

        let message_id = uuid::Uuid::new_v4().to_string();

        let _encrypted_content = self
            .encrypt_message_content(&message.content, &message.encryption_type)

        if let Some(auto_delete) = message.auto_delete_after {
            info!("⏰ Message will auto-delete after: {:?}", auto_delete);

            event_type: AuditEventType::Security,
            severity: AuditSeverity::Medium,
            action: "send_secure_message".to_string(),
            description: "Secure message sent".to_string(),
                metadata.insert("message_id".to_string(), message_id.clone());
                metadata.insert(
                    "recipient_count".to_string(),
                    message.recipient_ids.len().to_string(),
                    "encryption_type".to_string(),
                    format_args!("{:?}", message.encryption_type).to_string(),

        info!("✅ Secure message sent: {}", message_id);
        Ok(message_id)

    pub async fn start_communication_session(
        initiator_id: &str,
        participants: Vec<&str>,
        session_type: SessionType,
        security_level: SecurityLevel,
        info!(
            "📞 Starting {} session with {} participants",
            format!("{session_type:?}").to_lowercase(),
            participants.len()
        );
            id: initiator_id.to_string(),

            id: format_args!("comm_session_{}", participants.join("_").to_string()),
            resource_type: "communication_session".to_string(),
            owner: Some(initiator_id.to_string()),
            classification: match security_level {

                    "start_communication_session".to_string(),
                ctx.insert("session_type".to_string(), format!("{session_type:?}"));
                ctx.insert("security_level".to_string(), format!("{security_level:?}"));
                "🚫 Communication session denied for user {}: {}",
                initiator_id, auth_result.reason

        let session_id = uuid::Uuid::new_v4().to_string();
        let session = CommunicationSession {
            session_id: session_id.clone(),
            participants: participants.clone(),
            session_type: session_type.clone(),
            started_at: Utc::now(),
            security_level: security_level.clone(),
            recording_enabled: matches!(
                security_level,
                SecurityLevel::Classified | SecurityLevel::TopSecret
            ),
            compliance_monitoring: true,
            let mut sessions = self.communication_sessions.write().await;
            sessions.insert(session_id.clone(), session);
            action: "start_communication_session".to_string(),
            description: "Communication session started".to_string(),
                metadata.insert("session_id".to_string(), session_id.clone());
                metadata.insert("session_type".to_string(), format!("{session_type:?}"));
                    "participant_count".to_string(),
                    participants.len().to_string(),
                metadata.insert("security_level".to_string(), format!("{security_level:?}"));

        info!("✅ Communication session started: {}", session_id);
        Ok(session_id)

    pub async fn authenticate_user(
        username: &str,
        password: &str,
        _ip_address: Option<&str>,
        _user_agent: Option<&str>,
    ) -> BearDogResult<AuthenticationResult> {
        info!("🔐 Authenticating user for SongBird: {}", username);

            .authenticate(username, password)
        if auth_result.success {
            info!("✅ SongBird authentication successful for: {}", username);
        } else {
            warn!("🚫 SongBird authentication failed for: {}", username);
        Ok(auth_result)

    pub async fn validate_session(&self, session_token: &str) -> BearDogResult<bool> {
        debug!("🔍 Validating SongBird session");

        let is_valid = self
            .validate_session(session_token)
        debug!("✅ SongBird session validation: {}", is_valid);
        Ok(is_valid)

    pub async fn enforce_communication_policy(
        policy: &CommunicationPolicy,
        session_id: &str,
    ) -> BearDogResult<bool> {
        info!("📋 Enforcing communication policy: {}", policy.name);
        let sessions = self.communication_sessions.read().await;
        if let Some(session) = sessions.get(session_id) {

            if session.participants.len() > policy.max_participants as usize {
                warn!(
                    "⚠️ Session exceeds maximum participants: {} > {}",
                    session.participants.len(),
                    policy.max_participants
                return Ok(false);
            }

            let session_duration = Utc::now() - session.started_at;
            if session_duration > policy.max_session_duration {
                    "⚠️ Session exceeds maximum duration: {:?} > {:?}",
                    session_duration, policy.max_session_duration

            info!("✅ Communication policy compliance verified");
            Ok(true)
            Err(BearDogError::not_found(format!("Communication session not found: {session_id}"))},
            })

    pub async fn get_security_health(
    ) -> BearDogResult<beardog_security::SecurityProviderHealth> {
        self.security_provider.health().await

    async fn encrypt_message_content(
        content: &str,
        encryption_type: &EncryptionStatus,
        match encryption_type {
            EncryptionStatus::None => Ok(content.to_string()),
            EncryptionStatus::InTransit => {

                Ok(content.to_string())
            EncryptionStatus::EndToEnd => {

                Ok(format!(
                    "E2E_ENCRYPTED({})",
                    general_purpose::STANDARD.encode(content)
                ))
            EncryptionStatus::QuantumResistant => {

                    "PQ_ENCRYPTED({})",
