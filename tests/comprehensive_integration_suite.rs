

use beardog_adapters::*;
use beardog_types::canonical::AuditEventType;

use beardog_auth::*;
use beardog_compliance::*;
use beardog_types::config::*;
use beardog_core::*;
use beardog_errors::*;
use beardog_security::*;
use beardog_types::*;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct IntegrationTestHarness {
    pub config: OptimizedBearDogConfig,
    pub security_manager: Arc<RwLock<MemoryKeyManager>>,
    pub auth_handler: Arc<RwLock<MockAuthHandler>>,
    pub compliance_engine: Arc<RwLock<AuditEngine>>,
    pub metrics: Arc<RwLock<TestMetrics>>,
}

pub struct MockAuthHandler {
    pub authorized_users: HashMap<String, Vec<String>>,
    pub active_sessions: HashMap<String, AuthSession>,
    pub failed_attempts: u64,
    pub successful_attempts: u64,
}

#[derive(Debug, Clone)]
pub struct AuthSession {
    pub user_id: String,
    pub permissions: Vec<String>,
    pub created_at: std::time::Instant,
    pub expires_at: std::time::Instant,
}

#[derive(Debug, Default)]
pub struct TestMetrics {
    pub operations_completed: u64,
    pub operations_failed: u64,
    pub average_response_time_ms: f64,
    pub security_violations: u64,
    pub compliance_events: u64,
}

impl IntegrationTestHarness {
    pub async fn new() -> BearDogResult<Self> {
        let config = utils::create_production_config();
        let security_manager = Arc::new(RwLock::new(MemoryKeyManager::new()));
        let auth_handler = Arc::new(RwLock::new(MockAuthHandler::new()));
        let compliance_engine = Arc::new(RwLock::new(AuditEngine::new()));
        let metrics = Arc::new(RwLock::new(TestMetrics::default()));

        Ok(Self {
            config,
            security_manager,
            auth_handler,
            compliance_engine,
            metrics,
        })
    }

    pub async fn test_auth_workflow(&self, user_id: &str, requested_resource: &str) -> BearDogResult<bool> {
        let start_time = Instant::now();

        let mut auth_handler = self.auth_handler.write().await;
        let session = auth_handler.authenticate(user_id).await?;

        let authorized = auth_handler.authorize(&session, requested_resource).await?;

        drop(auth_handler); // Release the lock
        let mut compliance = self.compliance_engine.write().await;
        compliance.log_access_attempt(user_id, requested_resource, authorized).await?;

        drop(compliance); // Release the lock
        let mut metrics = self.metrics.write().await;
        let duration = start_time.elapsed();
        metrics.update_operation(authorized, duration);
        
        Ok(authorized)
    }

    pub async fn test_key_lifecycle_with_audit(&self, key_id: &str) -> BearDogResult<()> {
        let start_time = Instant::now();

        let key_data = BearDogCrypto::generate_secure_random(32)?;

        let mut security_manager = self.security_manager.write().await;
        security_manager.store_key(key_id, key_data.clone())?;

        drop(security_manager); // Release the lock
        let mut compliance = self.compliance_engine.write().await;
        let event = AuditEvent {
            id: format_args!("key_create_{}", key_id).to_string(),
            event_type: AuditEventType::Security,
            severity: AuditSeverity::Medium,
            user_id: "system".to_string(),
            resource: key_id.to_string(),
            outcome: "success".to_string(),
            description: format_args!("Cryptographic key {} created and stored", key_id).to_string(),
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("key_size".to_string(), "256".to_string());
                meta.insert("algorithm".to_string(), "AES-256".to_string());
                meta
            },
            timestamp: chrono::Utc::now(),
        };
        compliance.log_event(event).await?;

        drop(compliance); // Release the lock
        let mut security_manager = self.security_manager.write().await;
        let retrieved_key = security_manager.get_key(key_id)?
            .ok_or_else(|| BearDogError::internal("Key not found after storage".to_string(),
            ))?;
        
        assert_eq!(key_data, retrieved_key);

        let new_key_data = BearDogCrypto::generate_secure_random(32)?;
        security_manager.store_key(key_id, new_key_data.clone())?;

        let rotated_key = security_manager.get_key(key_id)?
            .ok_or_else(|| BearDogError::internal("Key not found after rotation".to_string(),
            ))?;
        
        assert_eq!(new_key_data, rotated_key);
        assert_ne!(key_data, rotated_key);

        drop(security_manager); // Release the lock
        let mut compliance = self.compliance_engine.write().await;
        let rotation_event = AuditEvent {
            id: format_args!("key_rotate_{}", key_id).to_string(),
            event_type: AuditEventType::Security,
            severity: AuditSeverity::Medium,
            user_id: "system".to_string(),
            resource: key_id.to_string(),
            outcome: "success".to_string(),
            description: format_args!("Cryptographic key {} rotated successfully", key_id).to_string(),
            metadata: HashMap::with_capacity(16),
            timestamp: chrono::Utc::now(),
        };
        compliance.log_event(rotation_event).await?;

        drop(compliance); // Release the lock
        let mut metrics = self.metrics.write().await;
        let duration = start_time.elapsed();
        metrics.update_operation(true, duration);
        
        Ok(())
    }

    pub async fn test_encrypted_communication(&self, sender: &str, receiver: &str, message: &[u8]) -> BearDogResult<Vec<u8>> {
        let start_time = Instant::now();

        let (sender_private, sender_public) = BearDogCrypto::generate_ed25519_keypair()?;
        let (receiver_private, receiver_public) = BearDogCrypto::generate_ed25519_keypair()?;

        let mut security_manager = self.security_manager.write().await;
        security_manager.store_key(&format_args!("{}_private", sender).to_string(), sender_private.clone())?;
        security_manager.store_key(&format_args!("{}_public", sender).to_string(), sender_public.clone())?;
        security_manager.store_key(&format_args!("{}_private", receiver).to_string(), receiver_private.clone())?;
        security_manager.store_key(&format_args!("{}_public", receiver).to_string(), receiver_public.clone())?;
        drop(security_manager); // Release the lock

        let encryption_key = BearDogCrypto::generate_secure_random(32)?;
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&encryption_key, message, None)?;

        let signature = BearDogCrypto::sign_ed25519(&sender_private, &ciphertext)?;

        let mut compliance = self.compliance_engine.write().await;
        let comm_event = AuditEvent {
            id: format_args!("comm_{}_{}", sender, receiver).to_string(),
            event_type: AuditEventType::Security,
            severity: AuditSeverity::Low,
            user_id: sender.to_string(),
            resource: format_args!("communication_channel_{}", receiver).to_string(),
            outcome: "encrypted".to_string(),
            description: format_args!("Encrypted communication from {} to {}", sender, receiver).to_string(),
            metadata: {
                let mut meta = HashMap::with_capacity(16);
                meta.insert("message_size".to_string(), message.len().to_string());
                meta.insert("encryption".to_string(), "AES-256-GCM".to_string());
                meta.insert("signature".to_string(), "Ed25519".to_string());
                meta
            },
            timestamp: chrono::Utc::now(),
        };
        compliance.log_event(comm_event).await?;
        drop(compliance); // Release the lock

        let signature_valid = BearDogCrypto::verify_ed25519_signature(&sender_public, &ciphertext, &signature)?;
        if !signature_valid {
            return Err(BearDogError::authentication("Invalid message signature".to_string(),
            ));
        }

        let decrypted = BearDogCrypto::decrypt_aes_gcm(&encryption_key, &ciphertext, &nonce)?;

        assert_eq!(message, decrypted);

        let mut metrics = self.metrics.write().await;
        let duration = start_time.elapsed();
        metrics.update_operation(true, duration);
        metrics.security_violations += if signature_valid { 0 } else { 1 };
        
        Ok(decrypted)
    }

    pub async fn test_concurrent_operations(&self, num_operations: usize) -> BearDogResult<TestResults> {
        let start_time = Instant::now();
        let mut handles = Vec::new();
        
        for i in 0..num_operations {
            let harness = self.clone_refs().await;
            let handle = tokio::spawn(async move {
                let user_id = format_args!("user_{}", i).to_string();
                let resource = format_args!("resource_{}", i % 10).to_string(); // 10 different resources

                let auth_result = harness.test_auth_workflow(&user_id, &resource).await;

                let key_id = format_args!("key_{}", i).to_string();
                let key_result = harness.test_key_lifecycle_with_audit(&key_id).await;
                
                (auth_result.is_ok(), key_result.is_ok())
            });
            handles.push(handle);
        }

        let mut successful_auth = 0;
        let mut successful_keys = 0;
        let mut total_operations = 0;
        
        for handle in handles {
            match handle.await {
                Ok((auth_success, key_success)) => {
                    total_operations += 1;
                    if auth_success { successful_auth += 1; }
                    if key_success { successful_keys += 1; }
                },
                Err(_) => {

                    total_operations += 1;
                }
            }
        }
        
        let duration = start_time.elapsed();
        let metrics = self.metrics.read().await;
        
        Ok(TestResults {
            total_operations,
            successful_auth_operations: successful_auth,
            successful_key_operations: successful_keys,
            total_duration: duration,
            average_response_time: duration.as_millis() as f64 / total_operations as f64,
            operations_per_second: total_operations as f64 / duration.as_secs_f64(),
            compliance_events: metrics.compliance_events,
            security_violations: metrics.security_violations,
        })
    }

    async fn clone_refs(&self) -> IntegrationTestHarness {
        IntegrationTestHarness {
            config: self.config.clone(),
            security_manager: Arc::clone(&self.security_manager),
            auth_handler: Arc::clone(&self.auth_handler),
            compliance_engine: Arc::clone(&self.compliance_engine),
            metrics: Arc::clone(&self.metrics),
        }
    }
}

impl MockAuthHandler {
    pub fn new() -> Self {
        let mut authorized_users = HashMap::with_capacity(16);
        authorized_users.insert("admin".to_string(), vec!["read".to_string(), "write".to_string(), "admin".to_string()]);
        authorized_users.insert("user".to_string(), vec!["read".to_string(), "write".to_string()]);
        authorized_users.insert("guest".to_string(), vec!["read".to_string()]);
        
        Self {
            authorized_users,
            active_sessions: HashMap::with_capacity(16),
            failed_attempts: 0,
            successful_attempts: 0,
        }
    }
    
    pub async fn authenticate(&mut self, user_id: &str) -> BearDogResult<AuthSession> {
        if let Some(permissions) = self.authorized_users.get(user_id) {
            let session = AuthSession {
                user_id: user_id.to_string(),
                permissions: permissions.clone(),
                created_at: Instant::now(),
                expires_at: Instant::now() + Duration::from_secs(3600), // 1 hour
            };
            
            self.active_sessions.insert(user_id.to_string(), session.clone());
            self.successful_attempts += 1;
            Ok(session)
        } else {
            self.failed_attempts += 1;
            Err(BearDogError::authentication(format_args!("User {) not authorized", user_id).to_string(),
            })
        }
    }
    
    pub async fn authorize(&self, session: &AuthSession, resource: &str) -> BearDogResult<bool> {

        if session.expires_at < Instant::now() {
            return Ok(false); // Session expired
        }

        let required_permission = if resource.contains("admin") {
            "admin"
        } else if resource.contains("write") || resource.starts_with("key_") {
            "write"
        } else {
            "read"
        };
        
        Ok(session.permissions.iter().any(|p| p == required_permission))
    }
}

impl TestMetrics {
    pub fn update_operation(&mut self, success: bool, duration: Duration) {
        self.operations_completed += 1;
        if !success {
            self.operations_failed += 1;
        }

        let duration_ms = duration.as_millis() as f64;
        self.average_response_time_ms = (self.average_response_time_ms * (self.operations_completed - 1) as f64 + duration_ms) / self.operations_completed as f64;
    }
}

#[derive(Debug)]
pub struct TestResults {
    pub total_operations: usize,
    pub successful_auth_operations: usize,
    pub successful_key_operations: usize,
    pub total_duration: Duration,
    pub average_response_time: f64,
    pub operations_per_second: f64,
    pub compliance_events: u64,
    pub security_violations: u64,
}

#[tokio::test]
async fn test_complete_authentication_workflow() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    let result = harness.test_auth_workflow("admin", "admin_resource").await?;
    assert!(result, "Admin should be authorized for admin resource");
    
    let result = harness.test_auth_workflow("user", "read_resource").await?;
    assert!(result, "User should be authorized for read resource");

    let result = harness.test_auth_workflow("guest", "admin_resource").await?;
    assert!(!result, "Guest should not be authorized for admin resource");

    let metrics = harness.metrics.read().await;
    assert_eq!(metrics.operations_completed, 3);
    assert!(metrics.average_response_time_ms > 0.0);
    
    println!("✅ Complete authentication workflow test passed");
    Ok(())
}

#[tokio::test]
async fn test_cryptographic_key_lifecycle() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    harness.test_key_lifecycle_with_audit("test_key_001").await?;

    let compliance = harness.compliance_engine.read().await;
    let events = compliance.search_events(Some(AuditEventType::Security), None, None).await?;
    assert!(!events.is_empty(), "Security events should be logged");

    let metrics = harness.metrics.read().await;
    assert!(metrics.operations_completed > 0);
    
    println!("✅ Cryptographic key lifecycle test passed");
    Ok(())
}

#[tokio::test]
async fn test_encrypted_communication_workflow() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;
    
    let original_message = b"This is a secret message for testing encrypted communication";
    let decrypted = harness.test_encrypted_communication("alice", "bob", original_message).await?;
    
    assert_eq!(original_message, decrypted.as_slice());

    let compliance = harness.compliance_engine.read().await;
    let events = compliance.search_events(Some(AuditEventType::Security), None, None).await?;
    assert!(events.iter().any(|e| e.description.contains("Encrypted communication")));
    
    println!("✅ Encrypted communication workflow test passed");
    Ok(())
}

#[tokio::test]
async fn test_concurrent_system_load() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    let results = harness.test_concurrent_operations(50).await?;
    
    println!("📊 Concurrent Load Test Results:");
    println!("  Total Operations: {}", results.total_operations);
    println!("  Successful Auth: {}", results.successful_auth_operations);
    println!("  Successful Keys: {}", results.successful_key_operations);
    println!("  Duration: {:?}", results.total_duration);
    println!("  Avg Response Time: {:.2}ms", results.average_response_time);
    println!("  Operations/sec: {:.2}", results.operations_per_second);
    println!("  Compliance Events: {}", results.compliance_events);
    println!("  Security Violations: {}", results.security_violations);

    assert!(results.operations_per_second > 10.0, "Should handle at least 10 ops/sec");
    assert!(results.average_response_time < 1000.0, "Average response time should be under 1 second");
    assert_eq!(results.security_violations, 0, "Should have no security violations");
    
    println!("✅ Concurrent system load test passed");
    Ok(())
}

#[tokio::test]
async fn test_system_resilience_and_recovery() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    let result = harness.test_auth_workflow("nonexistent_user", "some_resource").await;
    assert!(result.is_err(), "Should fail for nonexistent user");

    let security_manager = harness.security_manager.read().await;
    let result = security_manager.get_key("nonexistent_key");
    assert!(result.is_ok() && result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?.is_none(), "Should handle missing keys gracefully");
    drop(security_manager);

    let malformed_key = vec![1, 2, 3]; // Too short for AES-256
    let result = BearDogCrypto::encrypt_aes_gcm(&malformed_key, b"test", None);
    assert!(result.is_err(), "Should fail with malformed key");

    let result = harness.test_auth_workflow("admin", "admin_resource").await?;
    assert!(result, "System should recover and handle valid requests after errors");
    
    println!("✅ System resilience and recovery test passed");
    Ok(())
}

#[tokio::test]
async fn test_compliance_audit_trail() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    harness.test_auth_workflow("admin", "sensitive_resource").await?;
    harness.test_key_lifecycle_with_audit("audit_test_key").await?;
    harness.test_encrypted_communication("admin", "user", b"audit test message").await?;

    let compliance = harness.compliance_engine.read().await;
    let all_events = compliance.search_events(None, None, None).await?;
    
    assert!(!all_events.is_empty(), "Should have audit events");

    let security_events = compliance.search_events(Some(AuditEventType::Security), None, None).await?;
    assert!(!security_events.is_empty(), "Should have security events");

    for event in &security_events {
        assert!(!event.id.is_empty(), "Event ID should not be empty");
        assert!(!event.user_id.is_empty(), "User ID should not be empty");
        assert!(!event.description.is_empty(), "Description should not be empty");
    }
    
    println!("✅ Compliance audit trail test passed");
    println!("  Total events logged: {}", all_events.len());
    println!("  Security events: {}", security_events.len());
    
    Ok(())
}

#[tokio::test]
async fn test_configuration_integration() -> BearDogResult<()> {
    let harness = IntegrationTestHarness::new().await?;

    assert!(harness.config.database.pool.max_connections > 0);
    assert!(harness.config.memory.monitoring.enabled);
    assert!(harness.config.async_optimization.parallel_processing.enabled);
    assert!(harness.config.performance.monitoring.enabled);

    let json = serde_json::to_string(&harness.config)?;
    let deserialized: OptimizedBearDogConfig = serde_json::from_str(&json)?;
    
    assert_eq!(harness.config.database.url, deserialized.database.url);
    assert_eq!(harness.config.memory.monitoring.enabled, deserialized.memory.monitoring.enabled);
    
    println!("✅ Configuration integration test passed");
    Ok(())
} 