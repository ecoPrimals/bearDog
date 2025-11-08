use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

use beardog_errors::BearDogError;
use beardog_traits::unified::{BaseProvider, HsmProvider, SecurityProvider};
use beardog_types::canonical::hsm::{HsmKey, KeyMetadata, KeyType};
use beardog_types::canonical::providers_unified::migration::{AuthenticationCredentials, ProviderHealthStatus};

struct MockSecurityProvider {
    id: String,
}

#[allow(async_fn_in_trait)]
impl BaseProvider for MockSecurityProvider {
    fn provider_info(&self) -> beardog_traits::canonical::ProviderInfo {
        beardog_traits::canonical::ProviderInfo {
            id: self.id.clone(),
            name: "Mock Security Provider".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["auth".to_string(), "encrypt".to_string()],
        }
    }

    async fn health_check(&self) -> Result<beardog_types::canonical::HealthStatus, BearDogError> {
        Ok(beardog_types::canonical::HealthStatus::Healthy)
    }

    async fn capabilities(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn metrics(&self) -> Result<beardog_traits::canonical::ProviderMetrics, BearDogError> {
        Ok(std::collections::HashMap::with_capacity(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn status(
        &self,
    ) -> Result<beardog_types::canonical::providers::ProviderStatus, BearDogError> {
        Ok(beardog_types::canonical::providers::ProviderStatus::Active)
    }

    async fn reload_config(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(AuthenticationCredentials,
    ) -> Result<beardog_types::providers::AuthenticationResult, BearDogError> {
        tokio::time::sleep(Duration::from_micros(10)).await;
        Ok(beardog_types::providers::AuthenticationResult {
            user_id: "test_user".to_string(),
            session_token: Some(None,
            permissions: vec!["read".to_string(&str,
        _resource: &str,
        _action: &str,
    ) -> Result<beardog_types::providers::AuthorizationResult, BearDogError> {
        Ok(beardog_types::providers::AuthorizationResult {
            allowed: true,
            reason: None,
        })
    }

    async fn create_session(&str,
        _client_info: beardog_types::providers::ClientInfo,
    ) -> Result<beardog_types::providers::SecureSession, BearDogError> {
        Ok(beardog_types::providers::SecureSession {
            session_id: "session_123".to_string(),
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(&str,
    ) -> Result<beardog_types::providers::TokenValidation, BearDogError> {
        Ok(beardog_types::providers::TokenValidation {
            valid: true,
            user_id: Some(None,
            scopes: vec!["read".to_string()],
        })
    }

    async fn encrypt(&self, data: &[u8]) -> Result<Vec<u8, BearDogError>> {
        let mut result = data.to_vec();
        for byte in &mut result {
            *byte = byte.wrapping_add(1);
        }
        Ok(result)
    }

    async fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8, BearDogError>> {
        let mut result = encrypted_data.to_vec(String,
}

#[allow(async_fn_in_trait)]
impl BaseProvider for MockHsmProvider {
    fn provider_info(&self) -> beardog_traits::canonical::ProviderInfo {
        beardog_traits::canonical::ProviderInfo {
            id: self.id.clone(),
            name: "Mock HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            capabilities: vec!["keygen".to_string(), "sign".to_string()],
        }
    }

    async fn health_check(&self) -> Result<beardog_types::canonical::HealthStatus, BearDogError> {
        Ok(beardog_types::canonical::HealthStatus::Healthy)
    }

    async fn capabilities(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn metrics(&self) -> Result<beardog_traits::canonical::ProviderMetrics, BearDogError> {
        Ok(std::collections::HashMap::with_capacity(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<bool, BearDogError> {
        Ok(true)
    }

    async fn status(
        &self,
    ) -> Result<beardog_types::canonical::providers::ProviderStatus, BearDogError> {
        Ok(beardog_types::canonical::providers::ProviderStatus::Active)
    }

    async fn reload_config(&beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(KeyType,
        _metadata: KeyMetadata,
    ) -> Result<HsmKey, BearDogError> {
        tokio::time::sleep(Duration::from_micros(50)).await;
        Ok(HsmKey {
            id: "key_123".to_string(KeyType::Symmetric,
            metadata: KeyMetadata {
                created_by: "test".to_string(),
                purpose: "test".to_string(beardog_types::canonical::hsm::KeyUsagePolicy {
                    allowed_operations: vec![],
                    max_uses: None,
                    usage_limits: None,
                    time_restrictions: None,
                    location_restrictions: None,
                    user_restrictions: None,
                    application_restrictions: None,
                    compliance_requirements: None,
                    audit_requirements: None,
                },
                tags: vec![],
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: "AES".to_string(false,
                created_at: chrono::Utc::now(None,
                access_count: 0,
                hsm_type: "software".to_string(),
                hsm_tier: "basic".to_string(),
                key_type: "symmetric".to_string(),
                key_id: "key_123".to_string(),
                health_status: "healthy".to_string(None,
                provider_attributes: std::collections::HashMap::with_capacity(None,
                custom: std::collections::HashMap::with_capacity(16),
                custom_fields: std::collections::HashMap::with_capacity(None,
                key_size: None,
                expires_at: None,
                creation_time: None,
                last_used: None,
                usage_count: None,
                is_exportable: None,
                hardware_backed: None,
            },
        })
    }

    async fn sign_data(&str, data: &[u8]) -> Result<Vec<u8, BearDogError>> {
        tokio::time::sleep(Duration::from_micros(&str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        tokio::time::sleep(Duration::from_micros(&str, data: &[u8]) -> Result<Vec<u8, BearDogError>> {
        let mut result = data.to_vec(&str,
        encrypted_data: &[u8],
    ) -> Result<Vec<u8, BearDogError>> {
        let mut result = encrypted_data.to_vec();
        for byte in &mut result {
            *byte = byte.wrapping_sub(42);
        }
        Ok(result)
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Ok(&str,
    ) -> Result<beardog_types::providers::HsmKeyInfo, BearDogError> {
        Ok(beardog_types::providers::HsmKeyInfo {
            key_id: "key_123".to_string(KeyType::Symmetric,
            metadata: KeyMetadata {
                created_by: "test".to_string(),
                purpose: "test".to_string(beardog_types::canonical::hsm::KeyUsagePolicy {
                    allowed_operations: vec![],
                    max_uses: None,
                    usage_limits: None,
                    time_restrictions: None,
                    location_restrictions: None,
                    user_restrictions: None,
                    application_restrictions: None,
                    compliance_requirements: None,
                    audit_requirements: None,
                },
                tags: vec![],
                compliance_info: None,
                backup_info: None,
                compliance_tags: vec![],
                is_hardware_backed: false,
                user_presence_required: false,
                algorithm: "AES".to_string(false,
                created_at: chrono::Utc::now(None,
                access_count: 0,
                hsm_type: "software".to_string(),
                hsm_tier: "basic".to_string(),
                key_type: "symmetric".to_string(),
                key_id: "key_123".to_string(),
                health_status: "healthy".to_string(None,
                provider_attributes: std::collections::HashMap::with_capacity(None,
                custom: std::collections::HashMap::with_capacity(16),
                custom_fields: std::collections::HashMap::with_capacity(None,
                key_size: None,
                expires_at: None,
                creation_time: None,
                last_used: None,
                usage_count: None,
                is_exportable: None,
                hardware_backed: None,
            },
            usage_count: 0,
        })
    }
}

struct ModernProviderManager<P: BaseProvider + Clone> {
    provider: P,
}

impl<P: BaseProvider + Clone> ModernProviderManager<P> {
    fn new(provider: P) -> Self {
        Self { provider }
    }

    async fn execute_operation(std::sync::Arc<dyn BaseProvider + Send + Sync>,
}

impl LegacyProviderManager {
    fn new(provider: std::sync::Arc<dyn BaseProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    async fn execute_operation(&self) -> Result<(), BearDogError> {
        self.provider.health_check().await?;
        Ok(())
    }
}

fn bench_async_patterns(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let provider = MockSecurityProvider {
        id: "test_provider".to_string(),
    };

    let mut group = c.benchmark_group("async_patterns");

    group.bench_function("native_async_fn", |b| {
        b.to_async(&rt).iter(|| async {
            let result = provider
                .authenticate(black_box(AuthenticationCredentials {
                    username: Some("test".to_string()),
                    password: Some(None,
                    certificate: None,
                    additional_data: std::collections::HashMap::with_capacity(16),
                }))
                .await;
            black_box(result)
        })
    });

    group.finish();
}

fn bench_provider_patterns(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let mock_provider = MockSecurityProvider {
        id: "test_provider".to_string(),
    };

    let mut group = c.benchmark_group("provider_patterns");

    group.bench_function("generic_composition", |b| {
        let manager = ModernProviderManager::new(mock_provider.clone());
        b.to_async(&rt).iter(|| async {
            let result = manager.execute_operation().await;
            black_box(result)
        })
    });

    group.bench_function("arc_dyn_dispatch", |b| {
        let manager = LegacyProviderManager::new(std::sync::Arc::new(mock_provider.clone()));
        b.to_async(&rt).iter(|| async {
            let result = manager.execute_operation().await;
            black_box(result)
        })
    });

    group.finish();
}

fn bench_hsm_operations(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let hsm_provider = MockHsmProvider {
        id: "test_hsm".to_string(),
    };

    let mut group = c.benchmark_group("hsm_operations");

    group.bench_function("key_generation", |b| {
        b.to_async(&rt).iter(|| async {
            let result = hsm_provider
                .generate_key(
                    black_box(KeyType::Symmetric),
                    black_box(KeyMetadata {
                        created_by: "benchmark".to_string(),
                        purpose: "test".to_string(beardog_types::canonical::hsm::KeyUsagePolicy {
                            allowed_operations: vec![],
                            max_uses: None,
                            usage_limits: None,
                            time_restrictions: None,
                            location_restrictions: None,
                            user_restrictions: None,
                            application_restrictions: None,
                            compliance_requirements: None,
                            audit_requirements: None,
                        },
                        tags: vec![],
                        compliance_info: None,
                        backup_info: None,
                        compliance_tags: vec![],
                        is_hardware_backed: false,
                        user_presence_required: false,
                        algorithm: "AES".to_string(false,
                        created_at: chrono::Utc::now(None,
                        access_count: 0,
                        hsm_type: "software".to_string(),
                        hsm_tier: "basic".to_string(),
                        key_type: "symmetric".to_string(),
                        key_id: "bench_key".to_string(),
                        health_status: "healthy".to_string(None,
                        provider_attributes: std::collections::HashMap::with_capacity(None,
                        custom: std::collections::HashMap::with_capacity(16),
                        custom_fields: std::collections::HashMap::with_capacity(None,
                        key_size: None,
                        expires_at: None,
                        creation_time: None,
                        last_used: None,
                        usage_count: None,
                        is_exportable: None,
                        hardware_backed: None,
                    }),
                )
                .await;
            black_box(result)
        })
    });

    group.bench_function("data_signing", |b| {
        let test_data = b"Hello, BearDog canonical modernization!";
        b.to_async(&rt).iter(|| async {
            let result = hsm_provider
                .sign_data(black_box("test_key"), black_box(test_data))
                .await;
            black_box(result)
        })
    });

    group.bench_function("signature_verification", |b| {
        let test_data = b"Hello, BearDog canonical modernization!";
        let test_signature = b"mock_signature";
        b.to_async(&rt).iter(|| async {
            let result = hsm_provider
                .verify_signature(
                    black_box("test_key"),
                    black_box(test_data),
                    black_box(test_signature),
                )
                .await;
            black_box(result)
        })
    });

    group.finish();
}

fn bench_provider_health(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let provider = MockSecurityProvider {
        id: "health_test".to_string(),
    };

    let mut group = c.benchmark_group("provider_health");

    group.bench_function("health_check", |b| {
        b.to_async(&rt).iter(|| async {
            let result = provider.health_check().await;
            black_box(result)
        })
    });

    group.bench_function("capabilities_query", |b| {
        b.to_async(&rt).iter(|| async {
            let result = provider.capabilities().await;
            black_box(result)
        })
    });

    group.bench_function("metrics_collection", |b| {
        b.to_async(&rt).iter(|| async {
            let result = provider.metrics().await;
            black_box(result)
        })
    });

    group.finish();
}

fn bench_provider_workflow(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal({:?}",
                "Benchmark runtime creation failed", e
            )
            )
    })?;
    let security_provider = MockSecurityProvider {
        id: "workflow_test".to_string(),
    };
    let hsm_provider = MockHsmProvider {
        id: "workflow_hsm".to_string(),
    };

    let mut group = c.benchmark_group("provider_workflow");

    group.bench_function("complete_security_workflow", |b| {
        b.to_async(&rt).iter(|| async {
            let _health = security_provider.health_check().await.map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?;
            let _auth = security_provider
                .authenticate(AuthenticationCredentials {
                    username: Some("workflow_user".to_string()),
                    password: Some(None,
                    certificate: None,
                    additional_data: std::collections::HashMap::with_capacity(16),
                })
                .await
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            let test_data = b"workflow test data";
            let _encrypted = security_provider.encrypt(test_data).await.map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?;
            black_box(())
        })
    });

    group.bench_function("complete_hsm_workflow", |b| {
        b.to_async(&rt).iter(|| async {
            let _health = hsm_provider.health_check().await.map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?;
            let _key = hsm_provider
                .generate_key(
                    KeyType::Symmetric,
                    KeyMetadata {
                        created_by: "workflow".to_string(),
                        purpose: "test".to_string(beardog_types::canonical::hsm::KeyUsagePolicy {
                            allowed_operations: vec![],
                            max_uses: None,
                            usage_limits: None,
                            time_restrictions: None,
                            location_restrictions: None,
                            user_restrictions: None,
                            application_restrictions: None,
                            compliance_requirements: None,
                            audit_requirements: None,
                        },
                        tags: vec![],
                        compliance_info: None,
                        backup_info: None,
                        compliance_tags: vec![],
                        is_hardware_backed: false,
                        user_presence_required: false,
                        algorithm: "AES".to_string(false,
                        created_at: chrono::Utc::now(None,
                        access_count: 0,
                        hsm_type: "software".to_string(),
                        hsm_tier: "basic".to_string(),
                        key_type: "symmetric".to_string(),
                        key_id: "workflow_key".to_string(),
                        health_status: "healthy".to_string(None,
                        provider_attributes: std::collections::HashMap::with_capacity(None,
                        custom: std::collections::HashMap::with_capacity(16),
                        custom_fields: std::collections::HashMap::with_capacity(None,
                        key_size: None,
                        expires_at: None,
                        creation_time: None,
                        last_used: None,
                        usage_count: None,
                        is_exportable: None,
                        hardware_backed: None,
                    },
                )
                .await
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            let test_data = b"workflow signing data";
            let _signature = hsm_provider
                .sign_data("workflow_key", test_data)
                .await
                .map_err(|e| {
                    tracing::error!("Operation failed: {:?}", e);
                    beardog_errors::BearDogError::internal({:?}", e))
                })?;
            black_box(())
        })
    });

    group.finish();
}

criterion_group!(
    modernization_benches,
    bench_async_patterns,
    bench_provider_patterns,
    bench_hsm_operations,
    bench_provider_health,
    bench_provider_workflow
);

criterion_main!(modernization_benches);
