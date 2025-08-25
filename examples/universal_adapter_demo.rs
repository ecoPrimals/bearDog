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


//! Universal Adapter Architecture Demo
//!
//! This example demonstrates how BearDog integrates with the ecoPrimals ecosystem
//! using universal, domain-agnostic patterns rather than BearDog-centric ones.
//!
//! The key insight is that BearDog implements the universal `PrimalProvider` trait,
//! just like all other ecosystem components, enabling true interoperability.

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
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("🌐 Universal Adapter Architecture Demo");
    println!("=====================================\n");

    // Initialize BearDog core
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    // Create universal ecosystem manager
    let mut manager = UniversalEcosystemManager::new(core.clone()).await?;

    // Create BearDog PrimalProvider
    let beardog_provider = BearDogPrimalProvider::new(core.clone(), "main".to_string());

    // Create provider configuration
    let provider_config = ProviderConfig {
        provider_config: HashMap::from([
            ("security_level".to_string(), json!("high")),
            ("enable_audit".to_string(), json!(true)),
            ("compliance_standards".to_string(), json!(["SOC2", "GDPR"])),
        ]),
        ecosystem_config: HashMap::from([
            ("ecosystem_version".to_string(), json!("1.0.0")),
            ("discovery_enabled".to_string(), json!(true)),
        ]),
        network_config: beardog::adapters::universal::NetworkConfig::default(),
        monitoring_config: beardog::adapters::universal::MonitoringConfig::default(),
    };

    // Register BearDog with the universal ecosystem
    println!("🔌 Registering BearDog as universal PrimalProvider...");
    let registration = manager
        .register_provider(beardog_provider, provider_config)
        .await?;
    println!(
        "✅ Registration successful: {}",
        registration.registration_id
    );

    // Show BearDog's universal capabilities
    println!("\n🎯 BearDog's Universal Capabilities:");
    println!("====================================");
    let capabilities = manager.get_all_capabilities().await;
    for capability in &capabilities {
        println!("• {} - {}", capability.id, capability.name);
        println!("  Category: {:?}", capability.category);
        println!(
            "  QoS: {}ms avg, {:.2}% availability",
            capability.qos.avg_response_time_ms, capability.qos.availability_percent
        );
        println!();
    }

    // Show security capabilities specifically
    println!("🔒 Security Capabilities:");
    println!("========================");
    let security_capabilities = manager
        .get_capabilities_by_category(beardog::adapters::universal::CapabilityCategory::Security)
        .await;
    for capability in &security_capabilities {
        println!("• {} - {}", capability.id, capability.name);
        if let Some(algorithms) = capability.attributes.get("algorithms") {
            println!("  Algorithms: {}", algorithms.value);
        }
        if let Some(throughput) = &capability.qos.throughput {
            println!("  Throughput: {} {}", throughput.value, throughput.unit);
        }
        println!();
    }

    // Demonstrate universal request handling
    println!("🔄 Universal Request Handling:");
    println!("==============================");

    // Test encryption request
    let encrypt_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_ENCRYPT.to_string(),
        payload: json!({
            "data": "Hello, Universal Ecosystem!",
            "algorithm": "AES-256-GCM"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Normal,
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
            metadata: HashMap::new(),
        },
    };

    println!("📤 Sending encryption request...");
    let encrypt_response = manager.route_request(encrypt_request).await?;
    println!("📥 Encryption response: {}", encrypt_response.success);
    if let Some(encrypted_data) = encrypt_response.payload.get("encrypted_data") {
        println!("   Encrypted data: {}", encrypted_data);
    }

    // Test authentication request
    let auth_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_AUTHENTICATE.to_string(),
        payload: json!({
            "username": "demo-user",
            "password": "demo-password",
            "method": "password"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::High,
        metadata: HashMap::new(),
        context: RequestContext {
            user_id: None,
            session_id: None,
            transaction_id: Some("auth-transaction".to_string()),
            source_ecosystem: "demo".to_string(),
            target_ecosystem: Some("beardog".to_string()),
            metadata: HashMap::new(),
        },
    };

    println!("📤 Sending authentication request...");
    let auth_response = manager.route_request(auth_request).await?;
    println!("📥 Authentication response: {}", auth_response.success);
    if let Some(token) = auth_response.payload.get("token") {
        println!("   Token: {}", token);
    }

    // Test authorization request
    let authz_request = ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_types::SECURITY_AUTHORIZE.to_string(),
        payload: json!({
            "resource": "/api/secure-data",
            "action": "read",
            "subject": "demo-user"
        }),
        timestamp: chrono::Utc::now(),
        priority: RequestPriority::Critical,
        metadata: HashMap::new(),
        context: RequestContext {
            user_id: Some("demo-user".to_string()),
            session_id: Some("demo-session".to_string()),
            transaction_id: Some("authz-transaction".to_string()),
            source_ecosystem: "demo".to_string(),
            target_ecosystem: Some("beardog".to_string()),
            metadata: HashMap::new(),
        },
    };

    println!("📤 Sending authorization request...");
    let authz_response = manager.route_request(authz_request).await?;
    println!("📥 Authorization response: {}", authz_response.success);
    if let Some(authorized) = authz_response.payload.get("authorized") {
        println!("   Authorized: {}", authorized);
    }

    // Show ecosystem health
    println!("\n🏥 Ecosystem Health Report:");
    println!("===========================");
    let health_report = manager.health_check_all().await?;
    println!("Overall Health: {:?}", health_report.overall_health);
    println!("Total Providers: {}", health_report.total_providers);
    println!("Healthy Providers: {}", health_report.healthy_providers);
    println!("Degraded Providers: {}", health_report.degraded_providers);
    println!("Unhealthy Providers: {}", health_report.unhealthy_providers);

    for report in &health_report.provider_reports {
        println!("• {}: {:?}", report.provider_key, report.health_status);
    }

    // Show manager status
    println!("\n📊 Manager Status:");
    println!("==================");
    let status = manager.get_status().await;
    println!("Total Providers: {}", status.total_providers);
    println!("Active Providers: {}", status.active_providers);
    println!("Total Capabilities: {}", status.total_capabilities);
    println!("Active Requests: {}", status.active_requests);

    // Show all registered providers
    println!("\n🔍 Registered Providers:");
    println!("========================");
    let providers = manager.get_providers().await;
    for provider in &providers {
        println!(
            "• {} ({}) - {}",
            provider.ecosystem_id, provider.instance_id, provider.service_name
        );
        println!("  Version: {}", provider.service_version);
        println!("  Capabilities: {}", provider.capabilities.len());
        println!("  Dependencies: {}", provider.dependencies.len());
        println!("  Health: {:?}", provider.health_status);
        println!("  Endpoints: {}", provider.endpoints.primary);
        println!();
    }

    // Demonstrate that this is truly universal
    println!("🌟 Universal Architecture Benefits:");
    println!("===================================");
    println!("✅ Domain-agnostic: Works for any ecosystem component");
    println!("✅ Interoperable: Standard interfaces across all components");
    println!("✅ Scalable: Easy to add new ecosystem components");
    println!("✅ Observable: Unified monitoring and health checking");
    println!("✅ Discoverable: Automatic capability advertisement");
    println!("✅ Composable: Components can depend on each other's capabilities");
    println!("✅ Testable: Mock providers for testing");
    println!("✅ Configurable: Flexible configuration management");

    // Show how other ecosystems would integrate
    println!("\n🔮 Future Ecosystem Integration:");
    println!("================================");
    println!("This same pattern works for:");
    println!("• ToadStool (Compute Provider)");
    println!("  - Capabilities: compute.execute, compute.orchestrate");
    println!("  - Requests: compute.*, orchestration.*");
    println!("• SongBird (Communication Provider)");
    println!("  - Capabilities: communication.discovery, communication.routing");
    println!("  - Requests: discovery.*, routing.*");
    println!("• NestGate (Storage Provider)");
    println!("  - Capabilities: storage.persist, storage.backup");
    println!("  - Requests: storage.*, backup.*");
    println!("• Squirrel (AI Provider)");
    println!("  - Capabilities: ai.inference, ai.training");
    println!("  - Requests: ai.*, ml.*");

    println!("\n🎯 Key Insight:");
    println!("===============");
    println!("BearDog is NOT building BearDog-centric adapters.");
    println!("BearDog is IMPLEMENTING universal patterns established by SongBird.");
    println!("This makes BearDog a true ecosystem citizen, not a special case.");

    // Clean shutdown
    println!("\n🛑 Shutting down ecosystem manager...");
    manager.shutdown().await?;
    println!("✅ Shutdown complete");

    Ok(())
}

/// Example of how ToadStool might implement PrimalProvider
/// (This is just a mock to show the pattern)
#[allow(dead_code)]
struct MockToadStoolProvider {
    instance_id: String,
}

#[allow(dead_code)]
impl MockToadStoolProvider {
    fn new(instance_id: String) -> Self {
        Self { instance_id }
    }
}

// This would be the actual implementation in ToadStool's codebase
#[allow(dead_code)]
#[async_trait::async_trait]
impl PrimalProvider for MockToadStoolProvider {
    fn ecosystem_id(&self) -> &str {
        "toadstool"
    }

    fn instance_id(&self) -> &str {
        &self.instance_id
    }

    fn service_name(&self) -> &str {
        "ToadStool Compute Provider"
    }

    fn service_version(&self) -> &str {
        "1.0.0"
    }

    fn capabilities(&self) -> Vec<beardog::adapters::universal::Capability> {
        vec![beardog::adapters::universal::Capability {
            id: "compute.execute".to_string(),
            name: "Compute Execution".to_string(),
            description: "Execute compute workloads on universal platforms".to_string(),
            category: beardog::adapters::universal::CapabilityCategory::Compute,
            attributes: HashMap::from([(
                "platforms".to_string(),
                beardog::adapters::universal::CapabilityAttribute {
                    value: "8bit,16bit,32bit,64bit,gpu,quantum".to_string(),
                    data_type: beardog::adapters::universal::AttributeDataType::Array,
                    required: true,
                    description: Some("Supported compute platforms".to_string()),
                },
            )]),
            qos: beardog::adapters::universal::QualityOfService {
                avg_response_time_ms: 50,
                availability_percent: 99.9,
                throughput: Some(beardog::adapters::universal::ThroughputMetric {
                    value: 1000,
                    unit: "jobs/sec".to_string(),
                }),
                scalability: beardog::adapters::universal::ScalabilityInfo {
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
            min_version: Some("1.0.0".to_string()),
            optional: false,
            attributes: HashMap::from([(
                "purpose".to_string(),
                "Secure compute execution".to_string(),
            )]),
        }]
    }

    fn endpoints(&self) -> beardog::adapters::universal::ServiceEndpoints {
        beardog::adapters::universal::ServiceEndpoints {
            primary: "http://localhost:8080".to_string(),
            health: "http://localhost:8080/health".to_string(),
            metrics: Some("http://localhost:8080/metrics".to_string()),
            admin: Some("http://localhost:8080/admin".to_string()),
            events: None,
            custom: HashMap::new(),
        }
    }

    async fn health_check(&self) -> beardog::adapters::universal::HealthStatus {
        beardog::adapters::universal::HealthStatus::Healthy
    }

    async fn handle_request(
        &self,
        request: beardog::adapters::universal::ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {
        // Mock implementation
        Ok(ServiceResponse {
            request_id: request.request_id,
            success: true,
            payload: json!({
                "result": "Mock ToadStool response",
                "request_type": request.request_type
            }),
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
            error: None,
        })
    }

    async fn register_with_ecosystem(
        &self,
    ) -> BearDogResult<beardog::adapters::universal::EcosystemRegistration> {
        Ok(beardog::adapters::universal::EcosystemRegistration {
            registration_id: Uuid::new_v4(),
            ecosystem_id: self.ecosystem_id().to_string(),
            instance_id: self.instance_id().to_string(),
            endpoints: self.endpoints(),
            capabilities: self.capabilities(),
            registration_time: chrono::Utc::now(),
            status: beardog::adapters::universal::RegistrationStatus::Active,
        })
    }

    async fn initialize(
        &mut self,
        _config: beardog::adapters::universal::ProviderConfig,
    ) -> BearDogResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> BearDogResult<()> {
        Ok(())
    }

    fn can_handle_request(&self, request: &beardog::adapters::universal::ServiceRequest) -> bool {
        request.request_type.starts_with("compute.")
    }

    fn metadata(&self) -> beardog::adapters::universal::ProviderMetadata {
        beardog::adapters::universal::ProviderMetadata {
            name: "ToadStool Compute Provider".to_string(),
            version: "1.0.0".to_string(),
            description: "Universal compute provider for the ecoPrimals ecosystem".to_string(),
            author: "ecoPrimals Compute Team".to_string(),
            website: Some("https://github.com/ecoprimal/toadstool".to_string()),
            license: "MIT".to_string(),
            tags: vec!["compute".to_string(), "orchestration".to_string()],
            custom: HashMap::new(),
        }
    }
}
