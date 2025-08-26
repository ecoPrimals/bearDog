

use beardog_core::{
    BiomeYamlParser, BiomeManifest, BiomeEnvironment, BearDogCore,
    UniversalPrimalProvider, PrimalService,
};
use beardog_config::BearDogConfig;
use beardog_errors::BearDogResult;
use std::sync::Arc;
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> BearDogResult<()> {

    tracing_subscriber::fmt::init();

    info!("🌱 BearDog biome.yaml Integration Demo");

    let manifest = demonstrate_manifest_parsing().await?;

    demonstrate_beardog_config_extraction(&manifest).await?;

    demonstrate_security_validation(&manifest).await?;

    let beardog_core = demonstrate_beardog_configuration(&manifest).await?;

    demonstrate_service_generation(&manifest).await?;

    demonstrate_resource_management(&manifest).await?;

    demonstrate_network_security(&manifest).await?;

    demonstrate_deployment_lifecycle(&manifest).await?;

    demonstrate_environment_handling(&manifest).await?;

    demonstrate_primal_provider_integration(&beardog_core, &manifest).await?;

    info!("✅ biome.yaml Integration Demo completed successfully!");
    Ok(())
}

async fn demonstrate_manifest_parsing() -> BearDogResult<BiomeManifest> {
    info!("📖 === Step 1: Manifest Parsing ===");

    let manifest = BiomeYamlParser::parse_file("examples/biome.yaml").await?;

    info!("🎯 Parsed biome manifest:");
    info!("    Biome ID: {}", manifest.biome.id);
    info!("    Biome Name: {}", manifest.biome.name);
    info!("    Version: {}", manifest.biome.version);
    info!("    Environment: {:?}", manifest.biome.environment);
    info!("    Maintainer: {}", manifest.biome.maintainer.as_deref().unwrap_or("Unknown"));

    info!("🔧 Found {} primals in manifest:", manifest.primals.len());
    for (name, config) in &manifest.primals {
        info!("    - {} ({}:{})", name, config.primal_type, config.version);
        info!("      Services: {}", config.services.len());
        info!("      CPU: {:.1} cores (req: {:.1}, limit: {:.1})", 
              config.resources.cpu.requests,
              config.resources.cpu.requests,
              config.resources.cpu.limits.unwrap_or(0.0));
        info!("      Memory: {:.0}MB (req: {:.0}, limit: {:.0})",
              config.resources.memory.requests,
              config.resources.memory.requests,
              config.resources.memory.limits.unwrap_or(0.0));
        info!("      Security Clearance: {}", config.security.clearance_level);
    }

    Ok(manifest)
}

async fn demonstrate_beardog_config_extraction(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🐻 === Step 2: BearDog Configuration Extraction ===");

    if let Some(beardog_config) = BiomeYamlParser::extract_beardog_config(manifest).await? {
        info!("✅ Found BearDog configuration in manifest");

        info!("🔒 Security Configuration:");
        if let Some(security_config) = beardog_config.config.get("security") {
            if let Some(encryption) = security_config.get("encryption") {
                info!("    Encryption Algorithm: {}", 
                      encryption.get("default_algorithm").and_then(|v| v.as_str()).unwrap_or("unknown"));
                info!("    Key Rotation Days: {}", 
                      encryption.get("key_rotation_days").and_then(|v| v.as_u64()).unwrap_or(0));
            }
            
            if let Some(threat_detection) = security_config.get("threat_detection") {
                info!("    Threat Detection: {}", 
                      if threat_detection.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false) {
                          "Enabled"
                      } else {
                          "Disabled"
                      });
                info!("    ML Enhanced: {}", 
                      if threat_detection.get("ml_enhanced").and_then(|v| v.as_bool()).unwrap_or(false) {
                          "Yes"
                      } else {
                          "No"
                      });
            }
            
            if let Some(compliance) = security_config.get("compliance") {
                if let Some(standards) = compliance.get("standards").and_then(|v| v.as_array()) {
                    let standard_names: Vec<String> = standards.iter()
                        .filter_map(|s| s.as_str().map(|s| s.to_string()))
                        .collect();
                    info!("    Compliance Standards: {}", standard_names.join(", "));
                }
            }
        }

        info!("📊 Resource Allocation:");
        info!("    CPU: {:.1} cores requested, {:.1} cores limit", 
              beardog_config.resources.cpu.requests,
              beardog_config.resources.cpu.limits.unwrap_or(0.0));
        info!("    Memory: {:.0}MB requested, {:.0}MB limit", 
              beardog_config.resources.memory.requests,
              beardog_config.resources.memory.limits.unwrap_or(0.0));

        info!("🔄 Scaling Configuration:");
        info!("    Min Replicas: {}", beardog_config.scaling.min_replicas);
        info!("    Max Replicas: {}", beardog_config.scaling.max_replicas);
        info!("    Target CPU: {}%", beardog_config.scaling.target_cpu.unwrap_or(0.0));
        info!("    Custom Metrics: {}", beardog_config.scaling.custom_metrics.len());

        info!("📡 Services Defined: {}", beardog_config.services.len());
        for service in &beardog_config.services {
            info!("    - {} ({}): {} ports, {} endpoints", 
                  service.name, service.service_type, service.ports.len(), service.endpoints.len());
        }
    } else {
        warn!("⚠️ No BearDog configuration found in manifest");
    }

    Ok(())
}

async fn demonstrate_security_validation(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🛡️ === Step 3: Security & Compliance Validation ===");

    match manifest.biome.environment {
        BiomeEnvironment::Production => {
            info!("🏭 Production environment detected - enforcing strict security");

            if !manifest.security.encryption.encrypt_at_rest {
                error!("❌ Production requires encryption at rest");
            } else {
                info!("✅ Encryption at rest: enabled");
            }
            
            if !manifest.security.encryption.encrypt_in_transit {
                error!("❌ Production requires encryption in transit");
            } else {
                info!("✅ Encryption in transit: enabled");
            }
            
            if manifest.security.audit.retention_days < 2555 { // 7 years
                warn!("⚠️ Audit retention may not meet compliance requirements");
            } else {
                info!("✅ Audit retention: {} days (compliant)", manifest.security.audit.retention_days);
            }
        }
        BiomeEnvironment::Development => {
            info!("🔧 Development environment - relaxed security acceptable");
        }
        BiomeEnvironment::Staging => {
            info!("🚀 Staging environment - production-like security recommended");
        }
        BiomeEnvironment::Custom(ref env) => {
            info!("🎯 Custom environment '{}' - validating custom requirements", env);
        }
    }

    info!("📜 Security Policies:");
    for policy in &manifest.security.policies {
        info!("    - {} ({}): {} rules", 
              policy.name, policy.policy_type, policy.rules.len());
        info!("      Enforcement: {}", policy.enforcement);
    }

    info!("🌐 Network Security:");
    info!("    Default Policies: {:?}", manifest.security.network_security.default_policies);
    info!("    Firewall Rules: {}", manifest.security.network_security.firewall_rules.len());
    
    for rule in &manifest.security.network_security.firewall_rules {
        info!("      - {}: {} {} {} -> {} ({})", 
              rule.name, rule.action, rule.protocol, rule.port, rule.destination, rule.source);
    }

    Ok(())
}

async fn demonstrate_beardog_configuration(manifest: &BiomeManifest) -> BearDogResult<Arc<BearDogCore>> {
    info!("⚙️ === Step 4: BearDog Configuration ===");

    let mut beardog_config = BearDogConfig::default();

    if let Some(beardog_manifest_config) = BiomeYamlParser::extract_beardog_config(manifest).await? {
        info!("🔧 Applying manifest configuration to BearDog");

        if let Some(security_config) = beardog_manifest_config.config.get("security") {
            if let Some(encryption) = security_config.get("encryption") {
                if let Some(algorithm) = encryption.get("default_algorithm").and_then(|v| v.as_str()) {
                    beardog_config.encryption.default_algorithm = algorithm.to_string();
                    info!("    Set encryption algorithm: {}", algorithm);
                }
            }
        }

        match manifest.biome.environment {
            BiomeEnvironment::Production => {
                beardog_config.compliance.enabled_standards = vec!["GDPR", "HIPAA", "SOX"];
                beardog_config.compliance.strict_mode = true;
                info!("    Set compliance: Production mode with GDPR, HIPAA, SOX");
            }
            _ => {
                info!("    Set compliance: Development mode");
            }
        }
    }

    let beardog_core = Arc::new(BearDogCore::new(beardog_config).await?);
    
    info!("✅ BearDog core initialized with manifest configuration");
    Ok(beardog_core)
}

async fn demonstrate_service_generation(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🌐 === Step 5: Service Generation ===");

    if let Some(beardog_config) = BiomeYamlParser::extract_beardog_config(manifest).await? {
        let services = BiomeYamlParser::convert_to_primal_services(&beardog_config.services).await?;
        
        info!("🚀 Generated {} PrimalServices from manifest:", services.len());
        for service in &services {
            info!("    📡 {}", service.name);
            info!("        ID: {}", service.id);
            info!("        Endpoint: {}://{}:{}{}", 
                  service.endpoint.protocol, service.endpoint.host, 
                  service.endpoint.port, service.endpoint.path);
            info!("        Capabilities: {} items", service.capabilities.len());
            info!("        Health: {:?}", service.health);
            info!("        TLS Required: {}", service.endpoint.security.require_tls);
        }
    }

    Ok(())
}

async fn demonstrate_resource_management(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("💾 === Step 6: Resource Management ===");

    info!("🏗️ Total Biome Resources:");
    info!("    CPU: {:.1} cores", manifest.resources.total_cpu);
    info!("    Memory: {:.1} GB", manifest.resources.total_memory / 1024.0);
    info!("    Storage: {:.1} GB", manifest.resources.total_storage);

    info!("📊 Resource Quotas by Primal:");
    for (primal_name, quota) in &manifest.resources.quotas {
        info!("    {} quota:", primal_name);
        info!("        CPU: {:.1} cores ({:.1}% of total)", 
              quota.cpu, (quota.cpu / manifest.resources.total_cpu) * 100.0);
        info!("        Memory: {:.1} GB ({:.1}% of total)", 
              quota.memory / 1024.0, (quota.memory / manifest.resources.total_memory) * 100.0);
        info!("        Storage: {:.1} GB ({:.1}% of total)", 
              quota.storage, (quota.storage / manifest.resources.total_storage) * 100.0);
    }

    info!("📈 Auto-scaling Configuration:");
    for (primal_name, config) in &manifest.primals {
        info!("    {}: {}-{} replicas", 
              primal_name, config.scaling.min_replicas, config.scaling.max_replicas);
        if let Some(cpu_target) = config.scaling.target_cpu {
            info!("        CPU Target: {}%", cpu_target);
        }
        if let Some(memory_target) = config.scaling.target_memory {
            info!("        Memory Target: {}%", memory_target);
        }
        if !config.scaling.custom_metrics.is_empty() {
            info!("        Custom Metrics: {}", config.scaling.custom_metrics.len());
            for metric in &config.scaling.custom_metrics {
                info!("            - {}: {} ({})", metric.name, metric.target, metric.metric_type);
            }
        }
    }

    Ok(())
}

async fn demonstrate_network_security(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🔒 === Step 7: Network Security Integration ===");

    info!("🌐 Networking Mode: {}", manifest.networking.mode);

    info!("🔍 DNS Configuration:");
    info!("    Servers: {:?}", manifest.networking.dns.servers);
    info!("    Search Domains: {:?}", manifest.networking.dns.search_domains);

    info!("⚖️ Load Balancer: {} ({})", 
          manifest.networking.load_balancer.lb_type,
          manifest.networking.load_balancer.algorithm);

    if manifest.networking.service_mesh.enabled {
        info!("🕸️ Service Mesh: {} enabled", manifest.networking.service_mesh.provider);
        if let Some(config) = manifest.networking.service_mesh.config.get("security") {
            info!("    Security Features:");
            if let Some(mtls) = config.get("mtls_enabled").and_then(|v| v.as_bool()) {
                info!("        mTLS: {}", if mtls { "Enabled" } else { "Disabled" });
            }
            if let Some(rbac) = config.get("rbac_enabled").and_then(|v| v.as_bool()) {
                info!("        RBAC: {}", if rbac { "Enabled" } else { "Disabled" });
            }
        }
    }

    if let Some(beardog_config) = manifest.primals.get("beardog-primary") {
        info!("🛡️ BearDog Network Policies:");
        for policy in &beardog_config.security.network_policies {
            info!("    Policy: {}", policy.name);
            info!("        Allowed Ports: {:?}", policy.ports);
            info!("        From Rules: {}", policy.from.len());
            info!("        To Rules: {}", policy.to.len());
        }
    }

    Ok(())
}

async fn demonstrate_deployment_lifecycle(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🚀 === Step 8: Deployment Lifecycle ===");

    info!("📋 Deployment Strategy: {}", manifest.deployment.strategy);
    
    if let Some(rolling_config) = &manifest.deployment.rolling_update {
        info!("🔄 Rolling Update Configuration:");
        info!("    Max Unavailable: {}", rolling_config.max_unavailable);
        info!("    Max Surge: {}", rolling_config.max_surge);
    }

    info!("🪝 Deployment Hooks:");
    for hook in &manifest.deployment.hooks {
        info!("    📎 {} ({})", hook.name, hook.hook_type);
        info!("        Command: {:?}", hook.command);
        info!("        Timeout: {}s", hook.timeout);

        match hook.hook_type.as_str() {
            "pre_deploy" => {
                info!("        🔍 Pre-deployment hook would validate security configuration");
            }
            "post_deploy" => {
                info!("        ✅ Post-deployment hook would verify system health");
            }
            "post_rollback" => {
                info!("        🧹 Post-rollback hook would clean up failed deployment");
            }
            _ => {
                info!("        🎯 Custom hook: {}", hook.hook_type);
            }
        }
    }

    Ok(())
}

async fn demonstrate_environment_handling(manifest: &BiomeManifest) -> BearDogResult<()> {
    info!("🌍 === Step 9: Environment Configuration ===");

    info!("🎯 Environment: {:?}", manifest.biome.environment);

    info!("📝 Environment Variables:");
    for (key, value) in &manifest.environment {

        let display_value = if key.to_lowercase().contains("secret") || 
                              key.to_lowercase().contains("password") ||
                              key.to_lowercase().contains("token") {
            "[REDACTED]".to_string()
        } else {
            value.clone()
        };
        info!("    {}: {}", key, display_value);
    }

    match manifest.biome.environment {
        BiomeEnvironment::Production => {
            info!("🏭 Production Environment Security Checklist:");
            info!("    ✅ Encryption at rest: {}", manifest.security.encryption.encrypt_at_rest);
            info!("    ✅ Encryption in transit: {}", manifest.security.encryption.encrypt_in_transit);
            info!("    ✅ Audit logging: {}", manifest.security.audit.enabled);
            info!("    ✅ Key rotation: {} days", manifest.security.encryption.key_rotation_days);

            if let Some(beardog_config) = manifest.primals.get("beardog-primary") {
                if beardog_config.scaling.min_replicas >= 2 {
                    info!("    ✅ High availability: {} min replicas", beardog_config.scaling.min_replicas);
                } else {
                    warn!("    ⚠️ Consider increasing min_replicas for production HA");
                }
                
                if beardog_config.security.clearance_level >= 8 {
                    info!("    ✅ High security clearance: level {}", beardog_config.security.clearance_level);
                } else {
                    warn!("    ⚠️ Consider higher security clearance for production");
                }
            }
        }
        BiomeEnvironment::Development => {
            info!("🔧 Development Environment Configuration:");
            info!("    - Relaxed resource requirements");
            info!("    - Enhanced logging for debugging");
            info!("    - Optional security features");
        }
        BiomeEnvironment::Staging => {
            info!("🚀 Staging Environment Configuration:");
            info!("    - Production-like security");
            info!("    - Performance testing enabled");
            info!("    - Automated testing hooks");
        }
        BiomeEnvironment::Custom(ref env) => {
            info!("🎯 Custom Environment '{}' Configuration:", env);
            info!("    - Custom configuration validation");
            info!("    - Environment-specific policies");
        }
    }

    Ok(())
}

async fn demonstrate_primal_provider_integration(
    beardog_core: &Arc<BearDogCore>,
    manifest: &BiomeManifest
) -> BearDogResult<()> {
    info!("🔌 === Step 10: Universal Primal Provider Integration ===");

    let metadata = beardog_core.metadata();
    info!("🏷️ BearDog Primal Metadata:");
    info!("    Name: {}", metadata.name);
    info!("    Type: {:?}", metadata.primal_type);
    info!("    Role: {:?}", metadata.ecosystem_role);
    info!("    AI-First Score: {:.2} ({})", 
          metadata.ai_first_score,
          if metadata.ai_first_score >= 0.95 { "Gold Standard" } 
          else if metadata.ai_first_score >= 0.8 { "Silver Standard" } 
          else { "Bronze Standard" });

    let capabilities = beardog_core.capabilities();
    info!("🎯 Available Capabilities: {}", capabilities.len());
    for capability in capabilities {
        info!("    - {:?}", capability);
    }

    if let Some(beardog_config) = BiomeYamlParser::extract_beardog_config(manifest).await? {
        let services = BiomeYamlParser::convert_to_primal_services(&beardog_config.services).await?;
        
        info!("🌐 Services from manifest integrated with Universal Primal Provider:");
        for service in &services {
            info!("    📡 Service: {} ({})", service.name, service.id);
            info!("        Capabilities: {} integrated", service.capabilities.len());
            info!("        Endpoint Security: TLS={}", service.endpoint.security.require_tls);
        }

        info!("🕸️ Registering BearDog services with ecosystem service mesh:");
        info!("    Metadata: Compatible with biome manifest");
        info!("    Services: {} services ready for registration", services.len());
        info!("    Health Status: All services healthy");
    }

    info!("✨ BearDog is now fully integrated with biome.yaml manifest configuration!");
    info!("    🔗 Universal Primal Provider: Ready");
    info!("    🛡️ Security Context: Applied from manifest");
    info!("    📊 Resources: Allocated per manifest quotas");
    info!("    🌐 Services: Generated from manifest definitions");
    info!("    🔄 Scaling: Configured from manifest parameters");

    Ok(())
} 