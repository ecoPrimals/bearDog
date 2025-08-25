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


//! # BearDog Universal Service Registration Demo
//!
//! This example demonstrates BearDog's Universal Service Registration
//! and ecosystem integration capabilities following the Universal Primal Architecture.

use beardog_core::{
    BearDogCore, EcoPrimal, PrimalConfig, UniversalServiceRegistry,
    AIFirstResponse, AIFirstResponseBuilder, PrimalRequest, PrimalResponse,
    UniversalServiceRegistration, ServiceCapability, ServiceMetadata
};
use beardog_types::canonical::RiskLevel;

use std::collections::HashMap;
use tokio;
use tracing::{info, warn, Level};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting BearDog Universal Service Registration Demo");

    // Step 1: Create BearDog Universal Service Registration
    let registration = create_beardog_registration().await?;
    print_registration_info(&registration);

    // Step 2: Create Universal Service Registry client
    let mut registry = UniversalServiceRegistry::new("http://localhost:8080".to_string());
    
    // Step 3: Register BearDog with the ecosystem
    match registry.register_service(registration.clone()).await {
        Ok(response) => {
            info!("✅ Registration successful!");
            info!("Registration ID: {}", response.registration_id);
            info!("Status: {:?}", response.status);
            info!("Message: {}", response.message);
        },
        Err(e) => {
            warn!("⚠️ Registration failed, using local fallback: {}", e);
        }
    }

    // Step 4: Create BearDog Core instance and initialize as EcoPrimal
    let beardog_core = BearDogCore::new();
    let primal_config = PrimalConfig::default();
    
    info!("🌱 Initializing BearDog as EcoPrimal");
    if let Err(e) = EcoPrimal::initialize(&beardog_core, &primal_config).await {
        warn!("EcoPrimal initialization failed: {:?}", e);
    } else {
        info!("✅ BearDog EcoPrimal initialized successfully");
    }

    // Step 5: Demonstrate EcoPrimal capabilities
    demonstrate_ecoprimal_capabilities(&beardog_core).await?;

    // Step 6: Demonstrate AI-First responses
    demonstrate_ai_first_responses().await?;

    // Step 7: Send heartbeat to registry
    if registry.is_registered() {
        match registry.send_heartbeat().await {
            Ok(response) => {
                info!("💓 Heartbeat sent successfully");
                info!("Status: {:?}", response.status);
                info!("Next heartbeat: {}", response.next_heartbeat);
            },
            Err(e) => {
                warn!("Heartbeat failed: {}", e);
            }
        }
    }

    // Step 8: Demonstrate graceful shutdown
    info!("🛑 Initiating graceful shutdown");
    if let Err(e) = EcoPrimal::shutdown(&beardog_core).await {
        warn!("Shutdown error: {:?}", e);
    } else {
        info!("✅ BearDog shutdown completed gracefully");
    }

    info!("🎉 Universal Service Registration Demo completed successfully!");
    Ok(())
}

/// Create BearDog's universal service registration
async fn create_beardog_registration() -> Result<UniversalServiceRegistration, Box<dyn std::error::Error>> {
    info!("📝 Creating BearDog Universal Service Registration");
    
    let registration = UniversalServiceRegistry::create_beardog_registration()?;
    
    info!("✅ Universal Service Registration created");
    info!("Service ID: {}", registration.service_id);
    info!("Service Name: {}", registration.metadata.name);
    info!("Version: {}", registration.metadata.version);
    info!("Capabilities: {} items", registration.capabilities.len());
    info!("Endpoints: {} items", registration.endpoints.len());
    
    Ok(registration)
}

/// Print detailed registration information
fn print_registration_info(registration: &UniversalServiceRegistration) {
    info!("📊 === BearDog Service Registration Details ===");
    info!("🔍 Service ID: {}", registration.service_id);
    info!("📦 Name: {}", registration.metadata.name);
    info!("🏷️ Version: {}", registration.metadata.version);
    info!("📝 Description: {}", registration.metadata.description);
    info!("🏢 Organization: {:?}", registration.metadata.maintainer.organization);
    
    info!("🎯 Capabilities:");
    for (i, capability) in registration.capabilities.iter().enumerate() {
        match capability {
            ServiceCapability::Security { functions, compliance, trust_levels } => {
                info!("  {}. Security Capability:", i + 1);
                info!("     Functions: {}", functions.join(", "));
                info!("     Compliance: {}", compliance.join(", "));
                info!("     Trust Levels: {}", trust_levels.join(", "));
            },
            ServiceCapability::ArtificialIntelligence { models, tasks, interfaces } => {
                info!("  {}. AI Capability:", i + 1);
                info!("     Models: {}", models.join(", "));
                info!("     Tasks: {}", tasks.join(", "));
                info!("     Interfaces: {}", interfaces.join(", "));
            },
            ServiceCapability::Custom { domain, capability, parameters } => {
                info!("  {}. Custom Capability:", i + 1);
                info!("     Domain: {}", domain);
                info!("     Capability: {}", capability);
                info!("     Parameters: {} items", parameters.len());
            },
        }
    }
    
    info!("🌐 Endpoints:");
    for (i, endpoint) in registration.endpoints.iter().enumerate() {
        info!("  {}. {}: {}", i + 1, endpoint.name, endpoint.url);
        info!("     Protocol: {}, AI-Optimized: {}", endpoint.protocol, endpoint.ai_optimized);
    }
    
    info!("💻 Resource Requirements:");
    info!("  CPU Cores: {}", registration.resources.cpu_cores);
    info!("  Memory: {} MB", registration.resources.memory_bytes / 1024 / 1024);
    info!("  Storage: {} MB", registration.resources.storage_bytes.unwrap_or(0) / 1024 / 1024);
    info!("  Specialized Hardware: {}", registration.resources.specialized_hardware.join(", "));
    
    info!("⚙️ Integration Preferences:");
    info!("  Local Deployment: {}", registration.integration.prefers_local_deployment);
    info!("  Horizontal Scaling: {}", registration.integration.supports_horizontal_scaling);
    info!("  Load Balancing: {}", registration.integration.supports_load_balancing);
    
    info!("🏆 Priority: {}", registration.priority);
    info!("📅 Registration Time: {}", registration.registration_timestamp);
    info!("================================================");
}

/// Demonstrate EcoPrimal capabilities
async fn demonstrate_ecoprimal_capabilities(beardog_core: &BearDogCore) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 === Demonstrating EcoPrimal Capabilities ===");
    
    // Get metadata
    let metadata = EcoPrimal::metadata(beardog_core);
    info!("📊 Primal Type: {:?}", metadata.primal_type);
    info!("📦 Name: {}", metadata.name);
    info!("🏷️ Version: {}", metadata.version);
    
    // Get capabilities
    let capabilities = EcoPrimal::capabilities(beardog_core);
    info!("🎯 Capabilities: {} items", capabilities.len());
    for (i, capability) in capabilities.iter().enumerate() {
        info!("  {}. {:?}", i + 1, capability);
    }
    
    // Perform health check
    let health = EcoPrimal::health_check(beardog_core).await;
    info!("🏥 Health Status: {:?}", health.status);
    info!("🧩 Components: {} items", health.components.len());
    for component in &health.components {
        info!("  - {}: {:?}", component.name, component.status);
    }
    
    // Demonstrate request handling
    let request = PrimalRequest {
        request_id: Uuid::new_v4(),
        method: "hsm.discover_capabilities".to_string(),
        parameters: HashMap::new(),
        metadata: beardog_core::RequestMetadata {
            source_primal: beardog_core::PrimalType::ToadStool,
            priority: beardog_core::RequestPriority::Normal,
            timeout: None,
            auth_context: None,
        },
        timestamp: chrono::Utc::now(),
    };
    
    info!("📨 Sending test request: {}", request.method);
    match EcoPrimal::handle_request(beardog_core, request).await {
        Ok(response) => {
            info!("✅ Request handled successfully");
            info!("📊 Success: {}", response.success);
            info!("⏱️ Timestamp: {}", response.timestamp);
        },
        Err(e) => {
            warn!("❌ Request failed: {:?}", e);
        }
    }
    
    info!("==============================================");
    Ok(())
}

/// Demonstrate AI-First response format
async fn demonstrate_ai_first_responses() -> Result<(), Box<dyn std::error::Error>> {
    info!("🤖 === Demonstrating AI-First Response Format ===");
    
    let request_id = Uuid::new_v4();
    
    // Example 1: Successful response
    let success_response: AIFirstResponse<HashMap<String, String>> = AIFirstResponseBuilder::new(
        {
            let mut data = HashMap::new();
            data.insert("operation".to_string(), "key_generation".to_string());
            data.insert("key_id".to_string(), "generated_key_123".to_string());
            data.insert("algorithm".to_string(), "ed25519".to_string());
            data
        },
        request_id
    )
    .with_confidence(0.98)
    .build();
    
    info!("✅ Success Response Example:");
    info!("  Request ID: {}", success_response.request_id);
    info!("  Success: {}", success_response.success);
    info!("  Confidence: {}", success_response.confidence_score);
    info!("  Processing Time: {}ms", success_response.processing_time_ms);
    info!("  Quality Score: {}", success_response.ai_metadata.quality_metrics.security);
    
    // Example 2: Response with suggested actions
    let request_id_2 = Uuid::new_v4();
    let suggested_action = beardog_core::SuggestedAction {
        action: "verify_attestation".to_string(),
        confidence: 0.95,
        parameters: {
            let mut params = HashMap::new();
            params.insert("attestation_type".to_string(), serde_json::json!("platform_specific"));
            params
        },
        human_approval_recommended: false,
        expected_outcome: "Hardware attestation verified".to_string(),
        risk_assessment: beardog_core::RiskAssessment {
            risk_level: beardog_core::RiskLevel::Low,
            risk_factors: vec!["Hardware dependency".to_string()],
            mitigation_strategies: vec!["Fallback to software attestation".to_string()],
            impact_assessment: beardog_core::ImpactAssessment {
                security_impact: beardog_core::ImpactLevel::Minimal,
                performance_impact: beardog_core::ImpactLevel::None,
                user_experience_impact: beardog_core::ImpactLevel::None,
                system_stability_impact: beardog_core::ImpactLevel::None,
            },
        },
    };
    
    let response_with_actions: AIFirstResponse<String> = AIFirstResponseBuilder::new(
        "Operation completed with recommendations".to_string(),
        request_id_2
    )
    .with_confidence(0.87)
    .with_suggested_action(suggested_action)
    .build();
    
    info!("🎯 Response with AI Suggestions:");
    info!("  Request ID: {}", response_with_actions.request_id);
    info!("  Confidence: {}", response_with_actions.confidence_score);
    info!("  Suggested Actions: {} items", response_with_actions.suggested_actions.len());
    for (i, action) in response_with_actions.suggested_actions.iter().enumerate() {
        info!("    {}. {} (confidence: {:.2})", i + 1, action.action, action.confidence);
        info!("       Expected: {}", action.expected_outcome);
        info!("       Risk Level: {:?}", action.risk_assessment.risk_level);
    }
    
    info!("==============================================");
    Ok(())
}

impl BearDogCore {
    /// Create a new BearDog core instance for demo purposes
    pub fn new() -> Self {
        // This is a simplified constructor for demo purposes
        // In real implementation, this would initialize all components
        BearDogCore
    }
}

// Placeholder struct for demo - in real implementation this would be the actual BearDogCore
struct BearDogCore; 