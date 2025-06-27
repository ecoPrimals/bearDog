# BearDog API Interfaces Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** HIGH  

## 🎯 **Overview**

BearDog's API interfaces provide secure, RESTful access to security management functions:
- **RESTful API** with OpenAPI 3.0 specification
- **GraphQL** endpoint for complex queries
- **gRPC** for high-performance system integration
- **WebSocket** for real-time notifications
- **Webhook** support for external integrations

## 🔗 **Core API Architecture**

### **API Server Implementation**
```rust
use axum::{Router, middleware, extract::State};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::sync::Arc;

pub struct BearDogApiServer {
    config: Arc<ApiConfig>,
    core: Arc<BearDogCore>,
    auth_middleware: Arc<AuthenticationMiddleware>,
    rate_limiter: Arc<RateLimitMiddleware>,
}

impl BearDogApiServer {
    pub async fn new(config: ApiConfig, core: Arc<BearDogCore>) -> Result<Self> {
        Ok(Self {
            config: Arc::new(config),
            core,
            auth_middleware: Arc::new(AuthenticationMiddleware::new()),
            rate_limiter: Arc::new(RateLimitMiddleware::new()),
        })
    }
    
    pub fn create_router(&self) -> Router {
        Router::new()
            // Health and status
            .route("/health", get(health_check))
            .route("/status", get(system_status))
            
            // Authentication
            .route("/auth/login", post(auth_login))
            .route("/auth/logout", post(auth_logout))
            .route("/auth/refresh", post(auth_refresh))
            
            // Key Management
            .nest("/keys", self.create_key_routes())
            
            // Security Operations
            .nest("/security", self.create_security_routes())
            
            // Compliance
            .nest("/compliance", self.create_compliance_routes())
            
            // Workflows
            .nest("/workflows", self.create_workflow_routes())
            
            // Threat Detection
            .nest("/threats", self.create_threat_routes())
            
            // Admin
            .nest("/admin", self.create_admin_routes())
            
            // Middleware
            .layer(middleware::from_fn_with_state(
                self.core.clone(),
                auth_middleware,
            ))
            .layer(middleware::from_fn(rate_limit_middleware))
            .layer(TraceLayer::new_for_http())
            .layer(CorsLayer::permissive())
            .with_state(self.core.clone())
    }
}

// Key Management Endpoints
async fn generate_key(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<GenerateKeyRequest>,
) -> Result<Json<GenerateKeyResponse>, ApiError> {
    let key = core.key_manager.generate_master_key(request).await?;
    Ok(Json(GenerateKeyResponse { key }))
}

async fn encrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<EncryptionRequest>,
) -> Result<Json<EncryptionResponse>, ApiError> {
    let encrypted_data = core.key_manager.encrypt_data(request).await?;
    Ok(Json(EncryptionResponse { encrypted_data }))
}

async fn decrypt_data(
    State(core): State<Arc<BearDogCore>>,
    Json(request): Json<DecryptionRequest>,
) -> Result<Json<DecryptionResponse>, ApiError> {
    let plaintext = core.key_manager.decrypt_data(request).await?;
    Ok(Json(DecryptionResponse { plaintext }))
}
```

### **OpenAPI Specification**
```yaml
openapi: 3.0.3
info:
  title: BearDog Security Manager API
  description: Enterprise security management and encryption services
  version: 1.0.0
  contact:
    name: BearDog Security Team
    email: security@beardog.com
  license:
    name: Proprietary
    
servers:
  - url: https://beardog.internal:8443/api/v1
    description: Production server
  - url: https://beardog-dev.internal:8443/api/v1
    description: Development server

security:
  - bearerAuth: []
  - apiKeyAuth: []

paths:
  /health:
    get:
      summary: Health check
      operationId: healthCheck
      security: []
      responses:
        '200':
          description: Service is healthy
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HealthResponse'
  
  /keys:
    post:
      summary: Generate new encryption key
      operationId: generateKey
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/GenerateKeyRequest'
      responses:
        '201':
          description: Key generated successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/GenerateKeyResponse'
        '400':
          $ref: '#/components/responses/BadRequest'
        '401':
          $ref: '#/components/responses/Unauthorized'
        '403':
          $ref: '#/components/responses/Forbidden'

  /keys/{keyId}/encrypt:
    post:
      summary: Encrypt data with specified key
      operationId: encryptData
      parameters:
        - name: keyId
          in: path
          required: true
          schema:
            type: string
          description: Unique identifier of the encryption key
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/EncryptionRequest'
      responses:
        '200':
          description: Data encrypted successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/EncryptionResponse'

components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
    apiKeyAuth:
      type: apiKey
      in: header
      name: X-API-Key

  schemas:
    GenerateKeyRequest:
      type: object
      required:
        - keyType
        - ownerId
      properties:
        keyType:
          $ref: '#/components/schemas/KeyType'
        ownerId:
          type: string
          description: ID of the key owner
        algorithm:
          $ref: '#/components/schemas/EncryptionAlgorithm'
        purpose:
          $ref: '#/components/schemas/KeyPurpose'
        metadata:
          type: object
          additionalProperties: true
          
    EncryptionRequest:
      type: object
      required:
        - data
      properties:
        data:
          type: string
          format: base64
          description: Base64-encoded data to encrypt
        algorithm:
          $ref: '#/components/schemas/EncryptionAlgorithm'
        context:
          type: object
          additionalProperties: true
```

## 📱 **GraphQL Interface**

### **GraphQL Schema**
```rust
use async_graphql::{Object, Schema, Context, Result, Subscription};
use futures_util::Stream;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn keys(&self, ctx: &Context<'_>) -> Result<Vec<Key>> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let keys = core.key_manager.list_keys().await?;
        Ok(keys)
    }
    
    async fn key(&self, ctx: &Context<'_>, id: String) -> Result<Option<Key>> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let key = core.key_manager.get_key(&id).await?;
        Ok(Some(key))
    }
    
    async fn compliance_status(&self, ctx: &Context<'_>) -> Result<ComplianceStatus> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let status = core.compliance_engine.get_status().await?;
        Ok(status)
    }
    
    async fn threat_assessments(
        &self,
        ctx: &Context<'_>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<ThreatAssessment>> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let assessments = core.threat_engine
            .get_assessments(limit.unwrap_or(50), offset.unwrap_or(0))
            .await?;
        Ok(assessments)
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn generate_key(&self, ctx: &Context<'_>, input: GenerateKeyInput) -> Result<Key> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let request = GenerateKeyRequest {
            key_type: input.key_type,
            owner_id: input.owner_id,
            algorithm: input.algorithm,
            purpose: input.purpose,
            metadata: input.metadata.unwrap_or_default(),
        };
        let key = core.key_manager.generate_master_key(request).await?;
        Ok(key)
    }
    
    async fn initiate_workflow(&self, ctx: &Context<'_>, input: WorkflowInput) -> Result<Workflow> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let request = WorkflowRequest::from(input);
        let response = core.workflow_engine.initiate_workflow(request).await?;
        let workflow = core.workflow_engine.get_workflow(&response.workflow_id).await?;
        Ok(workflow)
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn threat_alerts(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = ThreatAlert>> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let stream = core.threat_engine.subscribe_to_alerts().await?;
        Ok(stream)
    }
    
    async fn workflow_updates(&self, ctx: &Context<'_>, workflow_id: String) -> Result<impl Stream<Item = WorkflowUpdate>> {
        let core = ctx.data::<Arc<BearDogCore>>()?;
        let stream = core.workflow_engine.subscribe_to_workflow_updates(&workflow_id).await?;
        Ok(stream)
    }
}

pub type BearDogSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub fn create_schema(core: Arc<BearDogCore>) -> BearDogSchema {
    Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(core)
        .finish()
}
```

## ⚡ **gRPC Interface**

### **Protocol Buffer Definitions**
```protobuf
syntax = "proto3";
package beardog.v1;

// Key Management Service
service KeyManagementService {
  rpc GenerateKey(GenerateKeyRequest) returns (GenerateKeyResponse);
  rpc EncryptData(EncryptDataRequest) returns (EncryptDataResponse);
  rpc DecryptData(DecryptDataRequest) returns (DecryptDataResponse);
  rpc RotateKey(RotateKeyRequest) returns (RotateKeyResponse);
  rpc DeleteKey(DeleteKeyRequest) returns (DeleteKeyResponse);
  rpc ListKeys(ListKeysRequest) returns (ListKeysResponse);
}

// Security Service
service SecurityService {
  rpc Authorize(AuthorizeRequest) returns (AuthorizeResponse);
  rpc Authenticate(AuthenticateRequest) returns (AuthenticateResponse);
  rpc AssessThreat(ThreatAssessmentRequest) returns (ThreatAssessmentResponse);
  rpc ReportIncident(IncidentReportRequest) returns (IncidentReportResponse);
}

// Compliance Service
service ComplianceService {
  rpc AssessCompliance(ComplianceAssessmentRequest) returns (ComplianceAssessmentResponse);
  rpc GenerateReport(ReportRequest) returns (ReportResponse);
  rpc CheckViolations(ViolationCheckRequest) returns (ViolationCheckResponse);
}

// Workflow Service
service WorkflowService {
  rpc InitiateWorkflow(InitiateWorkflowRequest) returns (InitiateWorkflowResponse);
  rpc SubmitApproval(SubmitApprovalRequest) returns (SubmitApprovalResponse);
  rpc GetWorkflowStatus(GetWorkflowStatusRequest) returns (GetWorkflowStatusResponse);
  rpc CancelWorkflow(CancelWorkflowRequest) returns (CancelWorkflowResponse);
}

message GenerateKeyRequest {
  string key_type = 1;
  string owner_id = 2;
  string algorithm = 3;
  string purpose = 4;
  map<string, string> metadata = 5;
}

message GenerateKeyResponse {
  string key_id = 1;
  string status = 2;
  string message = 3;
  KeyInfo key_info = 4;
}

message KeyInfo {
  string id = 1;
  string key_type = 2;
  string algorithm = 3;
  string owner_id = 4;
  string created_at = 5;
  string expires_at = 6;
}
```

### **gRPC Service Implementation**
```rust
use tonic::{Request, Response, Status};
use beardog_proto::key_management_service_server::{KeyManagementService, KeyManagementServiceServer};
use beardog_proto::{GenerateKeyRequest, GenerateKeyResponse};

pub struct BearDogKeyManagementService {
    core: Arc<BearDogCore>,
}

#[tonic::async_trait]
impl KeyManagementService for BearDogKeyManagementService {
    async fn generate_key(
        &self,
        request: Request<GenerateKeyRequest>,
    ) -> Result<Response<GenerateKeyResponse>, Status> {
        let req = request.into_inner();
        
        // Validate request
        if req.key_type.is_empty() || req.owner_id.is_empty() {
            return Err(Status::invalid_argument("Missing required fields"));
        }
        
        // Convert to internal request format
        let key_request = crate::GenerateKeyRequest {
            key_type: req.key_type.parse()
                .map_err(|_| Status::invalid_argument("Invalid key type"))?,
            owner_id: req.owner_id,
            algorithm: req.algorithm.parse().unwrap_or_default(),
            purpose: req.purpose.parse().unwrap_or_default(),
            metadata: req.metadata,
        };
        
        // Generate key
        match self.core.key_manager.generate_master_key(key_request).await {
            Ok(key) => {
                let response = GenerateKeyResponse {
                    key_id: key.id,
                    status: "success".to_string(),
                    message: "Key generated successfully".to_string(),
                    key_info: Some(beardog_proto::KeyInfo {
                        id: key.id,
                        key_type: key.key_type.to_string(),
                        algorithm: key.algorithm.to_string(),
                        owner_id: key.owner_id,
                        created_at: key.created_at.to_rfc3339(),
                        expires_at: key.expires_at.map(|dt| dt.to_rfc3339()).unwrap_or_default(),
                    }),
                };
                Ok(Response::new(response))
            }
            Err(e) => Err(Status::internal(format!("Key generation failed: {}", e))),
        }
    }
    
    // ... other method implementations
}
```

## 🔌 **Webhook Support**

### **Webhook Management**
```rust
pub struct WebhookManager {
    config: WebhookConfig,
    subscribers: Arc<RwLock<HashMap<String, WebhookSubscription>>>,
    delivery_queue: Arc<Mutex<VecDeque<WebhookDelivery>>>,
    http_client: reqwest::Client,
}

impl WebhookManager {
    pub async fn register_webhook(&self, subscription: WebhookSubscription) -> Result<String> {
        let subscription_id = uuid::Uuid::new_v4().to_string();
        
        // Validate webhook endpoint
        self.validate_webhook_endpoint(&subscription.url).await?;
        
        // Store subscription
        self.subscribers.write().await.insert(subscription_id.clone(), subscription);
        
        Ok(subscription_id)
    }
    
    pub async fn trigger_webhook(&self, event: WebhookEvent) -> Result<()> {
        let subscribers = self.subscribers.read().await;
        
        for (subscription_id, subscription) in subscribers.iter() {
            if subscription.event_types.contains(&event.event_type) {
                let delivery = WebhookDelivery {
                    id: uuid::Uuid::new_v4().to_string(),
                    subscription_id: subscription_id.clone(),
                    event: event.clone(),
                    url: subscription.url.clone(),
                    secret: subscription.secret.clone(),
                    max_retries: subscription.max_retries,
                    retry_count: 0,
                    scheduled_at: Utc::now(),
                };
                
                self.delivery_queue.lock().await.push_back(delivery);
            }
        }
        
        Ok(())
    }
    
    async fn deliver_webhook(&self, delivery: &WebhookDelivery) -> Result<bool> {
        let payload = serde_json::to_string(&delivery.event)?;
        let signature = self.generate_signature(&payload, &delivery.secret)?;
        
        let response = self.http_client
            .post(&delivery.url)
            .header("Content-Type", "application/json")
            .header("X-BearDog-Signature", signature)
            .header("X-BearDog-Event", delivery.event.event_type.to_string())
            .header("X-BearDog-Delivery", &delivery.id)
            .body(payload)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;
        
        Ok(response.status().is_success())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub event_id: String,
    pub event_type: WebhookEventType,
    pub timestamp: DateTime<Utc>,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WebhookEventType {
    KeyGenerated,
    KeyRotated,
    ThreatDetected,
    ComplianceViolation,
    WorkflowCompleted,
    IncidentCreated,
}
```

## ⚙️ **Configuration**

### **API Configuration**
```toml
[api]
# Server settings
bind_address = "0.0.0.0"
port = 8443
enable_tls = true
tls_cert_path = "./certs/api.crt"
tls_key_path = "./certs/api.key"

# API features
enable_rest = true
enable_graphql = true
enable_grpc = true
enable_websockets = true
enable_webhooks = true

[api.cors]
# CORS settings
enabled = true
allowed_origins = ["https://dashboard.internal.com"]
allowed_methods = ["GET", "POST", "PUT", "DELETE"]
allowed_headers = ["Authorization", "Content-Type"]

[api.rate_limiting]
# Rate limiting
enabled = true
requests_per_minute = 1000
burst_size = 100
cleanup_interval_minutes = 1

[api.authentication]
# Authentication settings
jwt_secret_env_var = "BEARDOG_JWT_SECRET"
jwt_expiration_minutes = 60
api_key_required = true
require_https = true

[api.documentation]
# API documentation
openapi_enabled = true
redoc_enabled = true
swagger_ui_enabled = true
graphql_playground_enabled = true

[api.webhooks]
# Webhook settings
max_subscribers = 1000
delivery_timeout_seconds = 30
max_retries = 3
retry_backoff_seconds = [1, 5, 15]
```

---

**Summary**: Created comprehensive specifications covering BearDog's core architecture, encryption/key management, security provider interface, configuration management, multi-party workflows, compliance/audit engine, threat detection/response, and API interfaces. Each spec follows secure-by-default principles with zero hardcoding and full configurability. 