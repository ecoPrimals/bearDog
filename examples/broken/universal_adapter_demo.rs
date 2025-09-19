use beardog_errors::BearDogError;

use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use beardog::{
    adapters::universal::{
        request_types, BearDogPrimalProvider, PrimalProvider, ProviderConfig, RequestContext,
        RequestPriority, ServiceRequest, ServiceResponse, UniversalEcosystemManager,
    },
    BearDogConfig, BearDogCore, BearDogResult,
};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::fmt::init();

    println!("🌐 Universal Adapter Architecture Demo");
    println!("=====================================");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config)?);

    let mut manager = UniversalEcosystemManager::new(core.clone())?;

    let beardog_provider = BearDogPrimalProvider::new(core.clone(), "main".to_string());

    let provider_config = ProviderConfig {
        provider_config: HashMap::from([
            ("security_level".to_string(), json!("high".to_string())),
            ("enable_audit".to_string(), json!(true)),
            ("compliance_standards".to_string(), json!(["SOC2", "GDPR"])),
        ]),
        ecosystem_config: HashMap::from([
            ("ecosystem_version".to_string(), json!("1.0.0")),
            ("discovery_enabled".to_string(), json!(true)),
        ]),
        network_config: beardog::adapters::universal::NetworkConfig::default(),
        monitoring_config: beardog::adapters::universal::MonitoringConfig::default({}",
        registration.registration_id
    );

    println!("[TARGET] BearDog's Universal Capabilities:");
    println!("====================================");
    let capabilities = manager.get_all_capabilities({:?}", capability.category);
        println!(
            "  QoS: {}ms avg, {:.2}% availability",
            capability.qos.avg_response_time_ms, capability.qos.availability_percent
        );
        println!();
    }

    println!("[LOCK] Security Capabilities:");
    println!("========================");
    let security_capabilities = manager
        .get_capabilities_by_category(beardog::adapters::universal::CapabilityCategory::Security)
        ;
    for capability in &security_capabilities {
        println!("- {} - {}", capability.id, capability.name);
        if let Some({}", algorithms.value);
        }
        if let Some({} {}", throughput.value, throughput.unit);
        }
        println!();
    }

    println!("[CYCLE] Universal Request Handling:");
    println!("==============================");

    let encrypt_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_ENCRYPT.to_string(json!({
            "data": "Hello, Universal Ecosystem!",
            "algorithm": "AES-256-GCM"
        }),
        timestamp: chrono::Utc::now(RequestPriority::Normal,
        metadata: HashMap::from([
            ("client".to_string(), "demo-client".to_string()),
            ("version".to_string(), "1.0.0".to_string()),
        ]),
        context: RequestContext {
            user_id: Some("demo-user".to_string()),
            session_id: Some("demo-session".to_string()),
            transaction_id: Some("demo-transaction".to_string()),
            source_ecosystem: "demo".to_string(),
            target_ecosystem: Some("beardog".to_string()),
            metadata: HashMap::with_capacity({}", encrypt_response.success);
    if let Some({}", encrypted_data);
    }

    let auth_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_AUTHENTICATE.to_string(json!({
            "username": "demo-user",
            "password": "demo-password",
            "method": "password"
        }),
        timestamp: chrono::Utc::now(RequestPriority::High,
        metadata: HashMap::with_capacity(RequestContext {
            user_id: None,
            session_id: None,
            transaction_id: Some("auth-transaction".to_string()),
            source_ecosystem: "demo".to_string(),
            target_ecosystem: Some("beardog".to_string()),
            metadata: HashMap::with_capacity({}", auth_response.success);
    if let Some({}", token);
    }

    let authz_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_AUTHORIZE.to_string(json!({
            "resource": "/api/secure-data",
            "action": "read",
            "subject": "demo-user"
        }),
        timestamp: chrono::Utc::now(RequestPriority::Critical,
        metadata: HashMap::with_capacity(16),
        context: RequestContext {
            user_id: Some("demo-user".to_string()),
            session_id: Some("demo-session".to_string()),
            transaction_id: Some("authz-transaction".to_string()),
            source_ecosystem: "demo".to_string(),
            target_ecosystem: Some("beardog".to_string()),
            metadata: HashMap::with_capacity({}", authz_response.success);
    if let Some({}", authorized);
    }

    println!("🏥 Ecosystem Health Report:");
    println!("===========================");
    let health_report = manager.health_check_all({:?}", health_report.overall_health);
    println!("Total Providers: {}", health_report.total_providers);
    println!("Healthy Providers: {}", health_report.healthy_providers);
    println!("Degraded Providers: {}", health_report.degraded_providers);
    println!("Unhealthy Providers: {}", health_report.unhealthy_providers);

    for report in &health_report.provider_reports {
        println!("- {}: {:?}", report.provider_key, report.health_status);
    }

    println!("[CHART] Manager Status:");
    println!("==================");
    let status = manager.get_status({}", status.total_providers);
    println!("Active Providers: {}", status.active_providers);
    println!("Total Capabilities: {}", status.total_capabilities);
    println!("Active Requests: {}", status.active_requests);

    println!("[SEARCH] Registered Providers:");
    println!("========================");
    let providers = manager.get_providers({}", provider.service_version);
        println!("  Capabilities: {}", provider.capabilities.len({}", provider.dependencies.len({:?}", provider.health_status);
        println!("  Endpoints: {}", provider.endpoints.primary);
        println!();
    }

    println!("🌟 Universal Architecture Benefits:");
    println!("===================================");
    println!("[OK] Domain-agnostic: Works for any ecosystem component");
    println!("[OK] Interoperable: Standard interfaces across all components");
    println!("[OK] Scalable: Easy to add new ecosystem components");
    println!("[OK] Observable: Unified monitoring and health checking");
    println!("[OK] Discoverable: Automatic capability advertisement");
    println!("[OK] Composable: Components can depend on each other's capabilities");
    println!("[OK] Testable: Mock providers for testing");
    println!("[OK] Configurable: Flexible configuration management");

    println!("🔮 Future Ecosystem Integration:");
    println!("================================");
    println!("This same pattern works for:");
    println!("- ComputeService (Compute Provider)");
    println!("  - Capabilities: compute.execute, compute.orchestrate");
    println!("  - Requests: compute.*, orchestration.*");
    println!("- MeshService (Communication Provider)");
    println!("  - Capabilities: communication.discovery, communication.routing");
    println!("  - Requests: discovery.*, routing.*");
    println!("- StorageService (Storage Provider)");
    println!("  - Capabilities: storage.persist, storage.backup");
    println!("  - Requests: storage.*, backup.*");
    println!("- AutomationService (AI Provider)");
    println!("  - Capabilities: ai.inference, ai.training");
    println!("  - Requests: ai.*, ml.*");

    println!("[TARGET] Key Insight:");
    println!("===============");
    println!("BearDog is NOT building BearDog-centric adapters.");
    println!("BearDog is IMPLEMENTING universal patterns established by MeshService.");
    println!("This makes BearDog a true ecosystem citizen, not a special case.");

    println!("🛑 Shutting down ecosystem manager...");
    manager.shutdown(&AdapterRequest,
    ) -> impl std::future::Future<Output = Result<AdapterResponse, BearDogError>> + Send;
    fn health_check(
        &self,
    ) -> impl std::future::Future<Output = Result<HealthStatus, BearDogError>> + Send;
}

struct MockComputeServiceProvider {
    instance_id: String,
}

impl MockComputeServiceProvider {
    fn new(instance_id: &str) -> Self {
        Self { instance_id }
    }
}

impl PrimalProvider for MockComputeServiceProvider {
    fn ecosystem_id(&self) -> &str {
        "compute-service"
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn service_name(&self) -> &str {
        "ComputeService Compute Provider"
    }

    fn service_version(&self) -> &str {
        "1.0.0"
    }

    fn capabilities(&self) -> Vec<beardog::adapters::universal::Capability> {
        vec![beardog::adapters::universal::Capability {
            id: "compute.execute".to_string(),
            name: "Compute Execution".to_string(),
            description: "Execute compute workloads on universal platforms".to_string(beardog::adapters::universal::CapabilityCategory::Compute,
            attributes: HashMap::from([(
                "platforms".to_string(),
                beardog::adapters::universal::CapabilityAttribute {
                    value: "8bit,16bit,32bit,64bit,gpu,quantum".to_string(beardog::adapters::universal::AttributeDataType::Array,
                    required: true,
                    description: Some(beardog::adapters::universal::QualityOfService {
                avg_response_time_ms: 50,
                availability_percent: 99.9,
                throughput: Some(beardog::adapters::universal::ThroughputMetric {
                    value: 1000,
                    unit: "jobs/sec".to_string(beardog::adapters::universal::ScalabilityInfo {
                    min_instances: 1,
                    max_instances: 1000,
                    auto_scaling: true,
                },
            },
            resource_requirements: beardog::adapters::universal::ResourceRequirements::default(),
        }]
    }

    fn dependencies(&self) -> Vec<beardog::adapters::universal::Dependency> {
        vec![beardog::adapters::universal::Dependency {
            id: "security-provider".to_string(),
            required_capability: "security.authorize".to_string(),
            min_version: Some(false,
            attributes: HashMap::from([("purpose".to_string(), "Secure compute execution")]),
        }]
    }

    fn endpoints(&self) -> beardog::adapters::universal::ServiceEndpoints {
        beardog::adapters::universal::ServiceEndpoints {
            primary: adapter
                .discover_capability_endpoint(required_capability)
                ?
                .to_string(),
            health: "http://localhost:8080/health".to_string(),
            metrics: Some("http://localhost:8080/metrics".to_string()),
            admin: Some("http://localhost:8080/admin".to_string(None,
            custom: HashMap::with_capacity(16),
        }
    }

    async fn health_check(&self) -> beardog::adapters::universal::HealthStatus {
        beardog::adapters::universal::HealthStatus::Healthy
    }

    async fn handle_request(beardog::adapters::universal::ServiceRequest,
    ) -> Result<ServiceResponse, BearDogError> {
        Ok(ServiceResponse {
            request_id: request.request_id.clone(true,
            payload: json!({
                "result": "Mock ComputeService response",
                "request_type": request.request_type
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::with_capacity(None,
        })
    }

    async fn register_with_ecosystem(
        &self,
    ) -> Result<beardog::adapters::universal::EcosystemRegistration, BearDogError> {
        Ok(beardog::adapters::universal::EcosystemRegistration {
            registration_id: Uuid::new_v4(),
            ecosystem_id: self.ecosystem_id().to_string(),
            instance_id: self.instance_id().to_string(),
            endpoints: self.endpoints(),
            capabilities: self.capabilities(),
            registration_time: chrono::Utc::now(beardog::adapters::universal::RegistrationStatus::Active,
        })
    }

    async fn initialize(beardog::adapters::universal::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), BearDogError> {
        Ok(())
    }

    fn can_handle_request(&self, request: &beardog::adapters::universal::ServiceRequest) -> bool {
        request.request_type.starts_with("compute.")
    }

    fn metadata(&self) -> beardog::adapters::universal::ProviderMetadata {
        beardog::adapters::universal::ProviderMetadata {
            name: "ComputeService Compute Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal compute provider for the ecoPrimals ecosystem".to_string(),
            author: "ecoPrimals Compute Team".to_string(),
            website: Some("https://github.com/ecoprimal/toadstool".to_string()),
            license: "MIT".to_string(),
            tags: vec!["compute".to_string(), "orchestration".to_string()],
            custom: HashMap::with_capacity(16),
        }
    }
}
