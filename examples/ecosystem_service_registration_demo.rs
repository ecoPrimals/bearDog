

use beardog_core::{
    UniversalServiceRegistry, UniversalServiceRegistration,
    AIFirstResponse, AIFirstResponseBuilder,
    ServiceCapability, ServiceMetadata
};
use beardog_types::canonical::RiskLevel;

use std::collections::HashMap;
use tokio;
use tracing::{info, warn, Level};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting BearDog Universal Service Registration Demo");

    let registration = create_beardog_registration().await?;
    print_registration_info(&registration);

    let mut registry = UniversalServiceRegistry::new("http://localhost:8080".to_string());

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

    demonstrate_ai_first_responses().await?;

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

    info!("🎉 Universal Service Registration Demo completed successfully!");
    Ok(())
}

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

async fn demonstrate_ai_first_responses() -> Result<(), Box<dyn std::error::Error>> {
    info!("🤖 === Demonstrating AI-First Response Format ===");
    
    let request_id = Uuid::new_v4();

    let success_response: AIFirstResponse<HashMap<String, String>> = AIFirstResponseBuilder::new(
        {
            let mut data = HashMap::with_capacity(16);
            data.insert("operation".to_string(), "key_generation".to_string());
            data.insert("key_id".to_string(), "generated_key_123".to_string());
            data.insert("algorithm".to_string(), "ed25519".to_string());
            data.insert("hardware_backed".to_string(), "true".to_string());
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
    info!("  Security Quality Score: {}", success_response.ai_metadata.quality_metrics.security);
    info!("  Reliability Score: {}", success_response.ai_metadata.quality_metrics.reliability);

    let request_id_2 = Uuid::new_v4();
    let suggested_action = beardog_core::SuggestedAction {
        action: "verify_attestation".to_string(),
        confidence: 0.95,
        parameters: {
            let mut params = HashMap::with_capacity(16);
            params.insert("attestation_type".to_string(), serde_json::json!("platform_specific"));
            params.insert("hardware_requirement".to_string(), serde_json::json!("strongbox_preferred"));
            params
        },
        human_approval_recommended: false,
        expected_outcome: "Hardware attestation verified with StrongBox backing".to_string(),
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
        "HSM operation completed with security recommendations".to_string(),
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
        info!("       Human Approval Required: {}", action.human_approval_recommended);
    }

    let discovery_response: AIFirstResponse<serde_json::Value> = AIFirstResponseBuilder::new(
        serde_json::json!({
            "platforms": ["android", "ios", "windows", "linux", "macos"],
            "algorithms": ["ed25519", "rsa_2048", "aes_256_gcm", "chacha20_poly1305"],
            "features": ["hardware_backed", "biometric_auth", "secure_attestation", "key_rotation"],
            "compliance": ["fips_140_2_level_3", "common_criteria_eal4"],
            "mobile_hsm": {
                "android_strongbox": true,
                "ios_secure_enclave": true,
                "biometric_integration": true
            },
            "enterprise_hsm": {
                "windows_tpm": true,
                "linux_pkcs11": true,
                "hardware_vendors": ["thales", "utimaco", "luna"]
            }
        }),
        Uuid::new_v4()
    )
    .with_confidence(1.0)
    .build();
    
    info!("🔍 HSM Capability Discovery Response:");
    info!("  Platforms Supported: {}", 
        discovery_response.data["platforms"].as_array().map(|a| a.len()).unwrap_or(0));
    info!("  Algorithms Available: {}", 
        discovery_response.data["algorithms"].as_array().map(|a| a.len()).unwrap_or(0));
    info!("  Mobile HSM Ready: {}", discovery_response.data["mobile_hsm"]["android_strongbox"]);
    info!("  Enterprise HSM Ready: {}", discovery_response.data["enterprise_hsm"]["windows_tpm"]);
    
    info!("==============================================");
    Ok(())
} 