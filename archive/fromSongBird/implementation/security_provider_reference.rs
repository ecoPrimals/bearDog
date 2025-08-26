

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

use songbird_orchestrator::security::{
    SecurityProvider, Subject, Resource, Action, AuditEvent,
    AuthenticationProvider, Credentials, AuthenticationResult, SessionInfo, AuthToken,
};
use songbird_orchestrator::errors::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogSecurityConfig {

    pub endpoint: String,

    pub api_key: String,

    pub timeout_seconds: u64,

    pub max_retries: u32,

    pub audit_batch_size: usize,

    pub audit_flush_interval_seconds: u64,

    pub enable_audit_encryption: bool,

    pub cache_size: usize,
    pub cache_ttl_seconds: u64,
}

impl Default for BearDogSecurityConfig {
    fn default() -> Self {
        Self {
            endpoint: "https://beardog.security.internal".to_string(),
            api_key: std::env::var("BEARDOG_API_KEY").unwrap_or_default(),
            timeout_seconds: 30,
            max_retries: 3,
            audit_batch_size: 100,
            audit_flush_interval_seconds: 60,
            enable_audit_encryption: true,
            cache_size: 1000,
            cache_ttl_seconds: 3600,
        }
    }
}

#[async_trait]
pub trait BearDogClient: Send + Sync {

    async fn check_authorization(
        &self,
        subject: &BearDogSubject,
        resource: &BearDogResource,
        action: &BearDogAction,
    ) -> Result<AuthorizationDecision>;

    async fn send_audit_batch(&self, events: Vec<BearDogAuditEvent>) -> Result<()>;

    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<BearDogAuthResult>;

    async fn validate_token(&self, token: &str) -> Result<BearDogSessionInfo>;

    async fn health_check(&self) -> Result<bool>;
}

pub struct BearDogHttpClient {
    config: BearDogSecurityConfig,
    http_client: reqwest::Client, // or hyper client
}

impl BearDogHttpClient {
    pub fn new(config: BearDogSecurityConfig) -> Self {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Failed to create HTTP client", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Failed to create HTTP client", e).to_string())
})?;

        Self { config, http_client }
    }

    async fn make_request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        payload: &T,
    ) -> Result<R> {
        let url = format_args!("{}{}", self.config.endpoint, path).to_string();
        
        let response = self.http_client
            .post(&url)
            .header("Authorization", format_args!("Bearer {}", self.config.api_key).to_string())
            .header("Content-Type", "application/json")
            .json(payload)
            .send()
            .await
            .map_err(|e| format_args!("HTTP request failed: {}", e).to_string())?;

        if response.status().is_success() {
            response.json::<R>().await
                .map_err(|e| format_args!("Failed to parse response: {}", e).to_string().into())
        } else {
            Err(format_args!("BearDog API error: {}", response.status().to_string()).into())
        }
    }
}

#[async_trait]
impl BearDogClient for BearDogHttpClient {
    async fn check_authorization(
        &self,
        subject: &BearDogSubject,
        resource: &BearDogResource,
        action: &BearDogAction,
    ) -> Result<AuthorizationDecision> {
        let request = AuthorizationRequest {
            subject: subject.clone(),
            resource: resource.clone(),
            action: action.clone(),
            context: HashMap::with_capacity(16),
            timestamp: Utc::now(),
        };

        self.make_request("/api/v1/authorize", &request).await
    }

    async fn send_audit_batch(&self, events: Vec<BearDogAuditEvent>) -> Result<()> {
        let request = AuditBatchRequest {
            events,
            timestamp: Utc::now(),
        };

        self.make_request::<_, serde_json::Value>("/api/v1/audit/batch", &request).await?;
        Ok(())
    }

    async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<BearDogAuthResult> {
        let request = AuthenticationRequest {
            username: username.to_string(),
            password: password.to_string(),
            timestamp: Utc::now(),
        };

        self.make_request("/api/v1/auth", &request).await
    }

    async fn validate_token(&self, token: &str) -> Result<BearDogSessionInfo> {
        let request = TokenValidationRequest {
            token: token.to_string(),
            timestamp: Utc::now(),
        };

        self.make_request("/api/v1/auth/validate", &request).await
    }

    async fn health_check(&self) -> Result<bool> {
        let response = self.http_client
            .get(&format_args!("{}/api/v1/health", self.config.endpoint).to_string())
            .header("Authorization", format_args!("Bearer {}", self.config.api_key).to_string())
            .send()
            .await
            .map_err(|e| format_args!("Health check failed: {}", e).to_string())?;

        Ok(response.status().is_success())
    }
}

pub struct BearDogSecurityProvider {
    config: BearDogSecurityConfig,
    client: Arc<dyn BearDogClient>,
    audit_queue: Arc<Mutex<VecDeque<AuditEvent>>>,
    auth_cache: Arc<Mutex<HashMap<String, CachedAuthDecision>>>,
}

impl BearDogSecurityProvider {
    pub fn new(config: BearDogSecurityConfig, client: Arc<dyn BearDogClient>) -> Self {
        Self {
            config,
            client,
            audit_queue: Arc::new(Mutex::new(VecDeque::new())),
            auth_cache: Arc::new(Mutex::new(HashMap::with_capacity(16))),
        }
    }

    fn map_subject(&self, subject: &Subject) -> BearDogSubject {
        BearDogSubject {
            id: subject.id.clone(),
            subject_type: match subject.subject_type {
                songbird_orchestrator::security::SubjectType::User => BearDogSubjectType::User,
                songbird_orchestrator::security::SubjectType::Service => BearDogSubjectType::Service,
                songbird_orchestrator::security::SubjectType::System => BearDogSubjectType::System,
            },
            attributes: subject.attributes.clone(),
        }
    }

    fn map_resource(&self, resource: &Resource) -> BearDogResource {
        BearDogResource {
            id: resource.id.clone(),
            resource_type: resource.resource_type.clone(),
            attributes: resource.attributes.clone(),
        }
    }

    fn map_action(&self, action: &Action) -> BearDogAction {
        BearDogAction {
            name: action.name.clone(),
            attributes: action.attributes.clone(),
        }
    }

    async fn check_auth_cache(
        &self,
        cache_key: &str,
    ) -> Option<bool> {
        let cache = self.auth_cache.lock().await;
        if let Some(cached) = cache.get(cache_key) {
            if cached.expires_at > Utc::now() {
                return Some(cached.decision);
            }
        }
        None
    }

    async fn cache_auth_decision(
        &self,
        cache_key: &str,
        decision: bool,
    ) {
        let mut cache = self.auth_cache.lock().await;
        let expires_at = Utc::now() + chrono::Duration::seconds(self.config.cache_ttl_seconds as i64);
        
        cache.insert(cache_key, CachedAuthDecision {
            decision,
            expires_at,
        });

        if cache.len() > self.config.cache_size {
            let now = Utc::now();
            cache.retain(|_, v| v.expires_at > now);
        }
    }

    fn generate_cache_key(&self, subject: &Subject, resource: &Resource, action: &Action) -> String {
        format_args!("{}:{}:{}:{}", 
            subject.subject_type as u8, 
            subject.id, 
            resource.id, 
            action.name
        ).to_string()
    }

    async fn maybe_flush_audit_queue(&self) -> Result<()> {
        let mut queue = self.audit_queue.lock().await;
        
        if queue.len() >= self.config.audit_batch_size {
            let events: Vec<_> = queue.drain(..).map(|e| self.map_audit_event(e)).collect();
            drop(queue); // Release lock before network call
            
            self.client.send_audit_batch(events).await?;
        }
        
        Ok(())
    }

    fn map_audit_event(&self, event: AuditEvent) -> BearDogAuditEvent {
        BearDogAuditEvent {
            event_type: format_args!("{:?}", event.event_type).to_string(),
            user_id: event.user_id,
            timestamp: event.timestamp,
            details: event.details,
            success: event.success,
            ip_address: event.ip_address,
            user_agent: event.user_agent,
            source: "songbird-orchestrator".to_string(),
        }
    }
}

#[async_trait]
impl SecurityProvider for BearDogSecurityProvider {
    async fn authorize(&self, subject: &Subject, resource: &Resource, action: &Action) -> Result<bool> {

        let cache_key = self.generate_cache_key(subject, resource, action);
        if let Some(cached_decision) = self.check_auth_cache(&cache_key).await {
            return Ok(cached_decision);
        }

        let beardog_subject = self.map_subject(subject);
        let beardog_resource = self.map_resource(resource);
        let beardog_action = self.map_action(action);

        let decision = self.client
            .check_authorization(&beardog_subject, &beardog_resource, &beardog_action)
            .await?;

        self.cache_auth_decision(cache_key, decision.allowed).await;

        let audit_event = AuditEvent {
            event_type: songbird_orchestrator::security::AuthEventType::AccessGranted,
            user_id: subject.id.clone(),
            timestamp: Utc::now(),
            details: {
                let mut details = HashMap::with_capacity(16);
                details.insert("resource".to_string(), serde_json::to_value(&resource.id)?);
                details.insert("action".to_string(), serde_json::to_value(&action.name)?);
                details.insert("decision".to_string(), serde_json::to_value(decision.allowed)?);
                details.insert("reason".to_string(), serde_json::to_value(&decision.reason)?);
                details
            },
            success: true,
            ip_address: None,
            user_agent: None,
        };

        self.log_audit(audit_event).await?;

        Ok(decision.allowed)
    }

    async fn log_audit(&self, event: AuditEvent) -> Result<()> {

        let mut queue = self.audit_queue.lock().await;
        queue.push_back(event);
        drop(queue);

        self.maybe_flush_audit_queue().await?;

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogSubject {
    pub id: String,
    pub subject_type: BearDogSubjectType,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BearDogSubjectType {
    User,
    Service,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogResource {
    pub id: String,
    pub resource_type: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogAction {
    pub name: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub subject: BearDogSubject,
    pub resource: BearDogResource,
    pub action: BearDogAction,
    pub context: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationDecision {
    pub allowed: bool,
    pub reason: String,
    pub policy_used: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogAuditEvent {
    pub event_type: String,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
    pub details: HashMap<String, serde_json::Value>,
    pub success: bool,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditBatchRequest {
    pub events: Vec<BearDogAuditEvent>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogAuthResult {
    pub success: bool,
    pub user_info: Option<BearDogUserInfo>,
    pub session_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogUserInfo {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub roles: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearDogSessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub roles: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone)]
struct CachedAuthDecision {
    decision: bool,
    expires_at: DateTime<Utc>,
}

pub async fn setup_beardog_integration() -> Result<BearDogSecurityProvider> {

    let config = BearDogSecurityConfig::default();

    let client = Arc::new(BearDogHttpClient::new(config.clone()));

    if !client.health_check().await? {
        return Err("BearDog health check failed".into());
    }

    let provider = BearDogSecurityProvider::new(config, client);
    
    Ok(provider)
}

pub async fn integrate_with_songbird() -> Result<()> {

    let beardog_provider = setup_beardog_integration().await?;

    let mut orchestrator = songbird_orchestrator::Orchestrator::builder()
        .with_security_provider(Arc::new(beardog_provider))
        .build()?;

    orchestrator.start().await?;
    
    Ok(())
} 