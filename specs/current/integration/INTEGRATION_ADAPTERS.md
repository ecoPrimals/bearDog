# BearDog Integration Adapters Specification

**Version:** 1.0  
**Date:** January 2025  
**Status:** SPECIFICATION  
**Priority:** HIGH  

## 🎯 **Overview**

BearDog's integration adapters enable seamless connectivity with external systems:
- **NestGate ZFS integration** via KeyManager trait
- **SongBird orchestration** via SecurityProvider trait  
- **Enterprise SIEM/SOAR** integration
- **Cloud provider** integrations (AWS, Azure, GCP)
- **Legacy system** adapters
- **Third-party security tools** integration

## 🔌 **Integration Architecture**

### **Core Integration Engine**
```rust
pub struct IntegrationEngine {
    config: Arc<IntegrationConfig>,
    adapters: HashMap<String, Box<dyn SystemAdapter>>,
    message_router: Arc<MessageRouter>,
    transformation_engine: Arc<TransformationEngine>,
    health_monitor: Arc<IntegrationHealthMonitor>,
}

impl IntegrationEngine {
    pub async fn register_adapter<T: SystemAdapter + 'static>(&mut self, adapter: T) -> Result<()> {
        let adapter_name = adapter.name().to_string();
        
        // Initialize adapter
        adapter.initialize(&self.config).await?;
        
        // Register with health monitoring
        self.health_monitor.register_adapter(&adapter_name).await?;
        
        // Store adapter
        self.adapters.insert(adapter_name, Box::new(adapter));
        
        Ok(())
    }
    
    pub async fn route_message(&self, message: IntegrationMessage) -> Result<IntegrationResponse> {
        // Determine target adapter
        let adapter_name = self.message_router.determine_target(&message)?;
        
        let adapter = self.adapters.get(&adapter_name)
            .ok_or_else(|| BearDogError::AdapterNotFound(adapter_name.clone()))?;
        
        // Transform message to adapter format
        let transformed_message = self.transformation_engine
            .transform_for_adapter(&message, &adapter_name)
            .await?;
        
        // Send to adapter
        let response = adapter.handle_message(transformed_message).await?;
        
        // Transform response back to standard format
        let transformed_response = self.transformation_engine
            .transform_from_adapter(&response, &adapter_name)
            .await?;
        
        Ok(transformed_response)
    }
}

#[async_trait]
pub trait SystemAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn supported_operations(&self) -> Vec<OperationType>;
    
    async fn initialize(&self, config: &IntegrationConfig) -> Result<()>;
    async fn health_check(&self) -> Result<AdapterHealth>;
    async fn handle_message(&self, message: TransformedMessage) -> Result<AdapterResponse>;
    async fn shutdown(&self) -> Result<()>;
}
```

## 🔑 **NestGate Integration Adapter**

### **ZFS KeyManager Implementation**
```rust
pub struct NestGateAdapter {
    name: String,
    beardog_core: Arc<BearDogCore>,
    nestgate_config: NestGateConfig,
    key_mapping: Arc<RwLock<HashMap<String, String>>>, // NestGate ID -> BearDog ID
}

impl NestGateAdapter {
    pub async fn new(beardog_core: Arc<BearDogCore>, config: NestGateConfig) -> Result<Self> {
        Ok(Self {
            name: "nestgate-zfs".to_string(),
            beardog_core,
            nestgate_config: config,
            key_mapping: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait]
impl nestgate_zfs::KeyManager for NestGateAdapter {
    async fn generate_master_key(&self, owner_id: &str) -> Result<nestgate_zfs::MasterKey> {
        // Generate key using BearDog's advanced key management
        let beardog_request = crate::GenerateKeyRequest {
            key_type: KeyType::Master,
            owner_id: owner_id.to_string(),
            algorithm: self.nestgate_config.default_algorithm.clone(),
            purpose: KeyPurpose::FileSystemEncryption,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("source".to_string(), json!("nestgate-zfs"));
                meta.insert("owner_id".to_string(), json!(owner_id));
                meta
            },
        };
        
        let beardog_key = self.beardog_core.key_manager
            .generate_master_key(beardog_request)
            .await?;
        
        // Store mapping
        let nestgate_key_id = format!("nestgate-{}", uuid::Uuid::new_v4());
        self.key_mapping.write().await.insert(
            nestgate_key_id.clone(),
            beardog_key.id.clone()
        );
        
        // Convert to NestGate format
        Ok(nestgate_zfs::MasterKey {
            id: nestgate_key_id,
            owner_id: owner_id.to_string(),
            algorithm: beardog_key.algorithm.to_string(),
            created_at: beardog_key.created_at,
            key_material: beardog_key.export_for_nestgate()?,
        })
    }
    
    async fn wrap_key(&self, key: &[u8], master_key_id: &str) -> Result<nestgate_zfs::WrappedKey> {
        // Map NestGate key ID to BearDog key ID
        let beardog_key_id = self.key_mapping.read().await
            .get(master_key_id)
            .cloned()
            .ok_or_else(|| BearDogError::KeyNotFound(master_key_id.to_string()))?;
        
        // Use BearDog's key wrapping with compliance tracking
        let wrap_request = KeyWrapRequest {
            key_to_wrap: key.to_vec(),
            wrapping_key_id: beardog_key_id,
            algorithm: self.nestgate_config.wrap_algorithm.clone(),
            context: "nestgate-zfs-backup".to_string(),
        };
        
        let wrapped_result = self.beardog_core.key_manager
            .wrap_key(wrap_request)
            .await?;
        
        // Convert to NestGate format
        Ok(nestgate_zfs::WrappedKey {
            wrapped_data: wrapped_result.wrapped_key,
            wrapping_key_id: master_key_id.to_string(),
            algorithm: wrapped_result.algorithm,
            metadata: wrapped_result.metadata,
        })
    }
    
    async fn unwrap_key(&self, wrapped_key: &nestgate_zfs::WrappedKey, master_key_id: &str) -> Result<Vec<u8>> {
        // Owner-only decryption with audit trail
        let beardog_key_id = self.key_mapping.read().await
            .get(master_key_id)
            .cloned()
            .ok_or_else(|| BearDogError::KeyNotFound(master_key_id.to_string()))?;
        
        let unwrap_request = KeyUnwrapRequest {
            wrapped_key: wrapped_key.wrapped_data.clone(),
            unwrapping_key_id: beardog_key_id,
            algorithm: wrapped_key.algorithm.clone(),
            context: "nestgate-zfs-restore".to_string(),
        };
        
        let unwrapped_key = self.beardog_core.key_manager
            .unwrap_key(unwrap_request)
            .await?;
        
        Ok(unwrapped_key)
    }
    
    async fn rotate_keys(&self, owner_id: &str) -> Result<nestgate_zfs::KeyRotationResult> {
        // Initiate multi-party approval workflow for key rotation
        let workflow_request = WorkflowRequest {
            workflow_type: WorkflowType::KeyRotation,
            initiator: "nestgate-system".to_string(),
            target: WorkflowTarget::User { user_id: owner_id.to_string() },
            parameters: {
                let mut params = HashMap::new();
                params.insert("owner_id".to_string(), json!(owner_id));
                params.insert("source".to_string(), json!("nestgate-zfs"));
                params
            },
            approval_requirements: None, // Use default requirements
            metadata: HashMap::new(),
        };
        
        let workflow_response = self.beardog_core.workflow_engine
            .initiate_workflow(workflow_request)
            .await?;
        
        Ok(nestgate_zfs::KeyRotationResult {
            workflow_id: workflow_response.workflow_id,
            status: "pending_approval".to_string(),
            estimated_completion: workflow_response.estimated_completion,
        })
    }
}

#[async_trait]
impl SystemAdapter for NestGateAdapter {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { "1.0.0" }
    fn supported_operations(&self) -> Vec<OperationType> {
        vec![
            OperationType::KeyGeneration,
            OperationType::KeyWrapping,
            OperationType::KeyRotation,
            OperationType::KeyBackup,
        ]
    }
    
    async fn initialize(&self, config: &IntegrationConfig) -> Result<()> {
        // Sync existing NestGate keys with BearDog
        self.sync_existing_keys().await?;
        Ok(())
    }
    
    async fn health_check(&self) -> Result<AdapterHealth> {
        // Check BearDog connectivity and key mapping consistency
        let beardog_health = self.beardog_core.health_check().await?;
        let mapping_count = self.key_mapping.read().await.len();
        
        Ok(AdapterHealth {
            status: if beardog_health.overall_status == ServiceStatus::Healthy {
                AdapterStatus::Healthy
            } else {
                AdapterStatus::Degraded
            },
            last_check: Utc::now(),
            details: format!("BearDog: {:?}, Mappings: {}", beardog_health.overall_status, mapping_count),
        })
    }
    
    async fn handle_message(&self, message: TransformedMessage) -> Result<AdapterResponse> {
        match message.operation {
            "generate_key" => {
                let owner_id = message.parameters.get("owner_id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::MissingParameter("owner_id".to_string()))?;
                
                let key = self.generate_master_key(owner_id).await?;
                
                Ok(AdapterResponse {
                    success: true,
                    data: serde_json::to_value(&key)?,
                    message: "Key generated successfully".to_string(),
                })
            }
            _ => Err(BearDogError::UnsupportedOperation(message.operation)),
        }
    }
    
    async fn shutdown(&self) -> Result<()> {
        // Cleanup resources
        Ok(())
    }
}
```

## 🎼 **SongBird Integration Adapter**

### **SecurityProvider Implementation**
```rust
pub struct SongBirdAdapter {
    name: String,
    beardog_core: Arc<BearDogCore>,
    songbird_config: SongBirdConfig,
    hook_registry: Arc<RwLock<HashMap<String, Box<dyn songbird::EventHook>>>>,
}

#[async_trait]
impl songbird_orchestrator::SecurityProvider for SongBirdAdapter {
    async fn authorize(&self, subject: &songbird::Subject, resource: &songbird::Resource, action: &songbird::Action) -> Result<bool> {
        // Convert SongBird entities to BearDog format
        let beardog_subject = self.convert_subject(subject)?;
        let beardog_resource = self.convert_resource(resource)?;
        let beardog_action = self.convert_action(action)?;
        
        // Use BearDog's advanced authorization engine
        let auth_request = AuthorizationRequest {
            subject: beardog_subject,
            resource: beardog_resource,
            action: beardog_action,
            context: self.build_context(subject, resource, action)?,
            timestamp: Utc::now(),
        };
        
        let auth_response = self.beardog_core.security_provider
            .authorize(auth_request)
            .await?;
        
        // Log to BearDog's comprehensive audit system
        self.log_authorization_decision(&auth_response).await?;
        
        Ok(auth_response.permitted)
    }
    
    async fn log_audit(&self, event: songbird::AuditEvent) -> Result<()> {
        // Convert SongBird audit event to BearDog's comprehensive format
        let beardog_event = SecurityAuditEvent {
            event_id: event.id,
            timestamp: event.timestamp,
            event_type: self.convert_event_type(&event.event_type)?,
            subject: self.convert_audit_subject(&event.subject)?,
            resource: self.convert_audit_resource(&event.resource)?,
            action: self.convert_audit_action(&event.action)?,
            decision: AuthorizationDecision {
                allowed: event.allowed,
                reason: event.reason,
                policies_applied: event.policies_applied,
                conditions: Vec::new(),
                context: event.context,
                decision_time: event.timestamp,
                confidence_score: 1.0,
            },
            threat_assessment: None, // Will be added by BearDog if needed
            policy_decision: None,
            processing_time_ms: 0,
            metadata: event.metadata,
        };
        
        // Send to BearDog's audit engine for comprehensive processing
        self.beardog_core.audit_engine
            .log_security_event(&beardog_event)
            .await?;
        
        Ok(())
    }
}

// SongBird Hook Implementation for Real-time Security Monitoring
pub struct BearDogSecurityHook {
    name: String,
    beardog_core: Arc<BearDogCore>,
    threat_detector: Arc<ThreatDetectionEngine>,
}

#[async_trait]
impl songbird::EventHook for BearDogSecurityHook {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { "1.0.0" }
    fn priority(&self) -> u32 { 100 } // High priority for security
    
    async fn handle_event(&self, event: &songbird::OrchestratorEvent) -> Result<songbird::HookResult> {
        match event {
            songbird::OrchestratorEvent::RequestReceived { service_id, request, timestamp } => {
                // Real-time threat analysis
                let security_event = self.convert_to_security_event(service_id, request, *timestamp)?;
                let threat_analysis = self.threat_detector.analyze_security_event(security_event).await?;
                
                if threat_analysis.threat_level >= ThreatLevel::High {
                    // Block request and trigger incident response
                    return Ok(songbird::HookResult {
                        success: true,
                        continue_chain: false,
                        allow_operation: false,
                        modifications: None,
                        log_messages: vec![format!("High threat detected: {}", threat_analysis.risk_score)],
                        execution_time_ms: 0,
                        error: None,
                    });
                }
                
                Ok(songbird::HookResult::allow_continue())
            }
            
            songbird::OrchestratorEvent::ServiceRegistering { service_info, timestamp } => {
                // Security posture assessment for new services
                let assessment = self.assess_service_security_posture(service_info).await?;
                
                if !assessment.meets_security_requirements {
                    return Ok(songbird::HookResult {
                        success: true,
                        continue_chain: false,
                        allow_operation: false,
                        modifications: None,
                        log_messages: vec!["Service security posture assessment failed".to_string()],
                        execution_time_ms: 0,
                        error: Some(assessment.failure_reason),
                    });
                }
                
                Ok(songbird::HookResult::allow_continue())
            }
            
            _ => Ok(songbird::HookResult::allow_continue())
        }
    }
}
```

## 🏢 **Enterprise SIEM Integration**

### **Splunk Adapter**
```rust
pub struct SplunkAdapter {
    name: String,
    splunk_client: Arc<SplunkClient>,
    event_transformer: Arc<SplunkEventTransformer>,
    batch_processor: Arc<BatchProcessor>,
}

#[async_trait]
impl SystemAdapter for SplunkAdapter {
    fn name(&self) -> &str { &self.name }
    
    async fn handle_message(&self, message: TransformedMessage) -> Result<AdapterResponse> {
        match message.operation.as_str() {
            "send_security_event" => {
                let event = self.event_transformer.transform_to_splunk(&message)?;
                
                // Send to Splunk HEC (HTTP Event Collector)
                self.splunk_client.send_event(event).await?;
                
                Ok(AdapterResponse {
                    success: true,
                    data: json!({"indexed": true}),
                    message: "Event sent to Splunk".to_string(),
                })
            }
            
            "query_events" => {
                let query = message.parameters.get("query")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::MissingParameter("query".to_string()))?;
                
                let results = self.splunk_client.search(query).await?;
                
                Ok(AdapterResponse {
                    success: true,
                    data: serde_json::to_value(&results)?,
                    message: format!("Found {} results", results.len()),
                })
            }
            
            _ => Err(BearDogError::UnsupportedOperation(message.operation)),
        }
    }
}

pub struct SplunkEventTransformer;

impl SplunkEventTransformer {
    pub fn transform_to_splunk(&self, message: &TransformedMessage) -> Result<SplunkEvent> {
        Ok(SplunkEvent {
            time: Utc::now().timestamp(),
            host: "beardog-security-manager".to_string(),
            source: "beardog".to_string(),
            sourcetype: "beardog:security".to_string(),
            index: "security".to_string(),
            event: message.parameters.clone(),
        })
    }
}
```

## ☁️ **Cloud Provider Integrations**

### **AWS Integration Adapter**
```rust
pub struct AwsAdapter {
    name: String,
    kms_client: Arc<aws_sdk_kms::Client>,
    s3_client: Arc<aws_sdk_s3::Client>,
    secrets_client: Arc<aws_sdk_secretsmanager::Client>,
    cloudtrail_client: Arc<aws_sdk_cloudtrail::Client>,
}

#[async_trait]
impl SystemAdapter for AwsAdapter {
    fn name(&self) -> &str { &self.name }
    
    async fn handle_message(&self, message: TransformedMessage) -> Result<AdapterResponse> {
        match message.operation.as_str() {
            "generate_kms_key" => {
                let key_spec = message.parameters.get("key_spec")
                    .and_then(|v| v.as_str())
                    .unwrap_or("SYMMETRIC_DEFAULT");
                
                let create_key_output = self.kms_client
                    .create_key()
                    .key_usage(aws_sdk_kms::types::KeyUsageType::EncryptDecrypt)
                    .key_spec(key_spec.parse().unwrap_or(aws_sdk_kms::types::KeySpec::SymmetricDefault))
                    .send()
                    .await?;
                
                Ok(AdapterResponse {
                    success: true,
                    data: json!({
                        "key_id": create_key_output.key_metadata.unwrap().key_id,
                        "arn": create_key_output.key_metadata.unwrap().arn,
                    }),
                    message: "KMS key created successfully".to_string(),
                })
            }
            
            "backup_to_s3" => {
                let bucket = message.parameters.get("bucket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::MissingParameter("bucket".to_string()))?;
                
                let key = message.parameters.get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::MissingParameter("key".to_string()))?;
                
                let data = message.parameters.get("data")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| BearDogError::MissingParameter("data".to_string()))?;
                
                self.s3_client
                    .put_object()
                    .bucket(bucket)
                    .key(key)
                    .body(aws_sdk_s3::primitives::ByteStream::from(data.as_bytes().to_vec()))
                    .server_side_encryption(aws_sdk_s3::types::ServerSideEncryption::AwsKms)
                    .send()
                    .await?;
                
                Ok(AdapterResponse {
                    success: true,
                    data: json!({"uploaded": true, "bucket": bucket, "key": key}),
                    message: "Data backed up to S3".to_string(),
                })
            }
            
            _ => Err(BearDogError::UnsupportedOperation(message.operation)),
        }
    }
}
```

## ⚙️ **Configuration**

### **Integration Configuration**
```toml
[integrations]
# General integration settings
enabled_adapters = ["nestgate", "songbird", "splunk", "aws"]
message_timeout_seconds = 30
retry_attempts = 3
health_check_interval_seconds = 60

[integrations.nestgate]
# NestGate ZFS integration
enabled = true
default_algorithm = "aes-256-gcm"
wrap_algorithm = "aes-kw"
sync_existing_keys = true
key_rotation_policy = "manual"

[integrations.songbird]
# SongBird orchestrator integration
enabled = true
hook_priority = 100
enable_threat_detection = true
enable_security_posture_assessment = true
real_time_monitoring = true

[integrations.splunk]
# Splunk SIEM integration
enabled = true
hec_endpoint = "https://splunk.internal.com:8088/services/collector"
hec_token_env_var = "SPLUNK_HEC_TOKEN"
index = "security"
sourcetype = "beardog:security"
batch_size = 100
batch_timeout_seconds = 30

[integrations.aws]
# AWS integration
enabled = true
region = "us-east-1"
assume_role_arn = "arn:aws:iam::123456789012:role/BearDogIntegrationRole"

[integrations.aws.kms]
# AWS KMS integration
default_key_spec = "SYMMETRIC_DEFAULT"
key_usage = "ENCRYPT_DECRYPT"
enable_multi_region = true

[integrations.aws.s3]
# AWS S3 backup integration
backup_bucket = "beardog-backups"
encryption = "aws:kms"
versioning_enabled = true

[integrations.transformation]
# Message transformation settings
enable_schema_validation = true
schema_registry_url = "https://schema-registry.internal.com"
enable_field_mapping = true
custom_transformations_path = "./transformations"
```

---

**Summary**: This comprehensive integration adapter system ensures BearDog can seamlessly connect with NestGate, SongBird, and all major enterprise security tools while maintaining security, compliance, and performance standards. 