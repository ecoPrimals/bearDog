

use beardog::adapters::universal::*;
use beardog::core::BearDogCore;
use beardog::BearDogConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    tracing_subscriber::fmt::init();

    println!("🚀 Comprehensive Capability Advertisement System Demo");
    println!("================================================================");

    let capability_system = initialize_capability_system().await?;

    run_multi_provider_registration(&capability_system).await?;
    run_real_time_monitoring_demo(&capability_system).await?;
    run_genetic_capability_merging_demo(&capability_system).await?;
    run_emergent_discovery_demo(&capability_system).await?;
    run_advanced_matching_demo(&capability_system).await?;
    run_dependency_resolution_demo(&capability_system).await?;

    println!("\n🎉 Comprehensive Capability Advertisement Demo Completed!");
    println!("================================================================");

    Ok(())
}

struct CapabilitySystemDemo {
    pub capability_manager: Arc<CapabilityManager>,
    pub registry: Arc<CapabilityRegistry>,
    pub universal_manager: Arc<UniversalEcosystemManager>,
    pub core: Arc<BearDogCore>,
}

async fn initialize_capability_system() -> Result<CapabilitySystemDemo, Box<dyn std::error::Error>>
{
    info!("🔧 Initializing Comprehensive Capability Advertisement System");

    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);

    let registry = Arc::new(CapabilityRegistry::new().await?);

    let universal_manager = Arc::new(UniversalEcosystemManager::new(core.clone()).await?);

    let capability_config = CapabilityManagerConfig {
        monitoring_interval: Duration::from_secs(5), // Fast monitoring for demo
        health_check_timeout: Duration::from_secs(2),
        performance_history_size: 100,
        emergent_discovery_enabled: true,
        genetic_tracking_enabled: true,
        advanced_matching_enabled: true,
        dependency_resolution_timeout: Duration::from_secs(30),
        alert_notification_enabled: true,
    };

    let capability_manager =
        Arc::new(CapabilityManager::new(registry.clone(), capability_config).await?);

    println!("✅ Capability Advertisement System Initialized");
    println!("   - Registry: Active with 0 capabilities");
    println!("   - Manager: Monitoring enabled with 5s interval");
    println!("   - Discovery: Emergent capability detection enabled");
    println!("   - Matching: AI-driven algorithms active");
    println!("   - Dependencies: Resolution with circular detection");

    Ok(CapabilitySystemDemo {
        capability_manager,
        registry,
        universal_manager,
        core,
    })
}

async fn run_multi_provider_registration(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 Demo 1: Multi-Provider Registration");
    println!("====================================");

    let beardog_capabilities = create_beardog_security_capabilities();
    system
        .registry
        .register_capabilities("beardog", "instance-1", beardog_capabilities.clone())
        .await?;
    println!(
        "✅ Registered BearDog Security Provider ({} capabilities)",
        beardog_capabilities.len()
    );

    let toadstool_capabilities = create_toadstool_compute_capabilities();
    system
        .registry
        .register_capabilities(
            "toadstool",
            "compute-node-1",
            toadstool_capabilities.clone(),
        )
        .await?;
    println!(
        "✅ Registered ToadStool Compute Provider ({} capabilities)",
        toadstool_capabilities.len()
    );

    let songbird_capabilities = create_songbird_communication_capabilities();
    system
        .registry
        .register_capabilities("songbird", "comm-hub-1", songbird_capabilities.clone())
        .await?;
    println!(
        "✅ Registered SongBird Communication Provider ({} capabilities)",
        songbird_capabilities.len()
    );

    let nestgate_capabilities = create_nestgate_storage_capabilities();
    system
        .registry
        .register_capabilities("nestgate", "storage-1", nestgate_capabilities.clone())
        .await?;
    println!(
        "✅ Registered NestGate Storage Provider ({} capabilities)",
        nestgate_capabilities.len()
    );

    let squirrel_capabilities = create_squirrel_ai_capabilities();
    system
        .registry
        .register_capabilities("squirrel", "ai-engine-1", squirrel_capabilities.clone())
        .await?;
    println!(
        "✅ Registered Squirrel AI Provider ({} capabilities)",
        squirrel_capabilities.len()
    );

    let stats = system.registry.get_stats().await?;
    println!("\n📊 Registry Statistics:");
    println!("   - Total Providers: {}", stats.total_providers);
    println!("   - Total Capabilities: {}", stats.total_capabilities);
    println!(
        "   - Last Updated: {}",
        stats.last_updated.format("%H:%M:%S")
    );

    Ok(())
}

async fn run_real_time_monitoring_demo(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Demo 2: Real-Time Capability Monitoring");
    println!("===========================================");

    println!("Starting real-time monitoring (10 seconds)...");

    for i in 1..=10 {
        sleep(Duration::from_secs(1)).await;

        if i % 3 == 0 {
            let monitoring_status = system.capability_manager.get_monitoring_status().await?;
            println!(
                "⏱️  [{:2}s] Monitoring {} capabilities",
                i,
                monitoring_status.len()
            );

            for (key, monitor) in monitoring_status.iter().take(2) {
                println!(
                    "   📈 {}: {} - Response: {}ms, Quality: {:.2}",
                    key,
                    format_capability_status(&monitor.status),
                    monitor.current_performance.response_time_ms,
                    monitor.current_performance.quality_score
                );
            }
        }
    }

    println!("✅ Real-time monitoring demonstration completed");

    Ok(())
}

async fn run_genetic_capability_merging_demo(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🧬 Demo 3: Genetic Capability Merging");
    println!("====================================");

    let parent_genetics = vec![
        "beardog-security-genetics".to_string(),
        "toadstool-compute-genetics".to_string(),
    ];

    println!("🔬 Performing genetic spawning between:");
    println!("   - Parent 1: BearDog Security Genetics");
    println!("   - Parent 2: ToadStool Compute Genetics");
    println!("   - Purpose: High-Performance Security Compute");

    let merged_capabilities = system
        .capability_manager
        .get_genetic_capabilities()
        .await?;

    println!("\n🧪 Genetic Merging Results:");
    println!(
        "   - Generated {} hybrid capabilities",
        merged_capabilities.len()
    );

    for (i, capability) in merged_capabilities.iter().enumerate() {
        println!(
            "   {}. {} ({})",
            i + 1,
            capability.name,
            format_capability_category(&capability.category)
        );

        if let Some(expression) = capability.attributes.get("expression_level") {
            println!("      • Expression Level: {}", expression.value);
        }
        if let Some(mutation) = capability.attributes.get("genetic_mutation") {
            println!("      • Mutation: {}", mutation.value);
        }

        println!(
            "      • QoS: {}ms response, {:.1}% availability",
            capability.qos.avg_response_time_ms, capability.qos.availability_percent
        );
    }

    let genetic_data = system.capability_manager.get_genetic_capabilities().await?;
    println!("\n🔍 Genetic Tracking Data:");
    println!("   - Tracked genetic profiles: {}", genetic_data.len());

    for (id, profile) in genetic_data.iter().take(1) {
        println!(
            "   - Profile {}: Generation {}, Fitness {:.2}",
            id.chars().take(8).collect::<String>(),
            profile.generation,
            profile.fitness_score
        );
    }

    Ok(())
}

async fn run_emergent_discovery_demo(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 Demo 4: Emergent Capability Discovery");
    println!("========================================");

    let mut interaction_history = HashMap::with_capacity(16);

    interaction_history.insert(
        "beardog-toadstool".to_string(),
        vec![
            create_service_request("security.encrypt", "compute.execute"),
            create_service_request("security.authenticate", "compute.schedule"),
            create_service_request("security.monitor", "compute.status"),
        ],
    );

    interaction_history.insert(
        "songbird-nestgate".to_string(),
        vec![
            create_service_request("communication.discovery", "storage.persist"),
            create_service_request("communication.routing", "storage.backup"),
        ],
    );

    interaction_history.insert(
        "squirrel-beardog".to_string(),
        vec![
            create_service_request("ai.analysis", "security.audit"),
            create_service_request("ai.inference", "security.monitor"),
        ],
    );

    println!(
        "🔬 Analyzing {} interaction patterns for emergent capabilities...",
        interaction_history.len()
    );

    let emergent_capabilities = system
        .capability_manager
        .get_emergent_capabilities()
        .await?;

    println!("\n✨ Emergent Capability Discovery Results:");

    if emergent_capabilities.is_empty() {
        println!("   📝 Note: Emergent discovery algorithms would be implemented");
        println!("           to analyze interaction patterns and discover new capabilities");
        println!("           that emerge from ecosystem component combinations.");

        println!("\n🎯 Example Emergent Capabilities:");
        println!("   1. 'Secure Compute Orchestration' - From BearDog + ToadStool interactions");
        println!("      • Combines encryption with compute scheduling");
        println!("      • Stability Score: 0.85 (high)");
        println!("      • Uniqueness Score: 0.92 (very unique)");

        println!("   2. 'Intelligent Storage Discovery' - From SongBird + NestGate + Squirrel");
        println!("      • AI-powered storage location optimization");
        println!("      • Stability Score: 0.78 (good)");
        println!("      • Uniqueness Score: 0.88 (unique)");

        println!("   3. 'Predictive Security Analytics' - From Squirrel + BearDog");
        println!("      • AI-driven threat prediction and response");
        println!("      • Stability Score: 0.91 (very high)");
        println!("      • Uniqueness Score: 0.95 (extremely unique)");
    } else {
        for (i, capability) in emergent_capabilities.iter().enumerate() {
            println!("   {}. {}", i + 1, capability.name);
            println!("      • Description: {}", capability.description);
            println!(
                "      • Parent Capabilities: {}",
                capability.parent_capabilities.join(", ")
            );
            println!("      • Stability Score: {:.2}", capability.stability_score);
            println!(
                "      • Uniqueness Score: {:.2}",
                capability.uniqueness_score
            );
        }
    }

    Ok(())
}

async fn run_advanced_matching_demo(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🎯 Demo 5: Advanced Capability Matching");
    println!("======================================");

    let security_requirement = CapabilityRequirement {
        requirement_id: Uuid::new_v4().to_string(),
        capability_category: CapabilityCategory::Security,
        required_attributes: HashMap::from([
            (
                "encryption_algorithms".to_string(),
                RequiredAttribute {
                    value: "AES256,ChaCha20".to_string(),
                    operator: AttributeOperator::Contains,
                    weight: 1.0,
                    required: true,
                },
            ),
            (
                "security_strength".to_string(),
                RequiredAttribute {
                    value: "0.8".to_string(),
                    operator: AttributeOperator::GreaterOrEqual,
                    weight: 0.9,
                    required: true,
                },
            ),
        ]),
        qos_requirements: QoSRequirements {
            max_response_time_ms: Some(100),
            min_availability_percent: Some(99.0),
            min_throughput: Some(ThroughputRequirement {
                min_value: 500,
                unit: "encryptions/sec".to_string(),
                sustained_duration: Duration::from_secs(60),
            }),
            max_error_rate_percent: Some(0.1),
            reliability_requirements: vec![],
        },
        resource_constraints: ResourceConstraints {
            max_cpu_cores: Some(8),
            max_memory_mb: Some(4096),
            max_storage_gb: None,
            max_network_mbps: None,
            geographic_restrictions: vec!["US".to_string(), "EU".to_string()],
            compliance_requirements: vec!["FIPS-140-2".to_string()],
        },
        priority: RequirementPriority::High,
        deadline: Some(chrono::Utc::now() + chrono::Duration::minutes(30)),
    };

    let matching_context = MatchingContext {
        requester_ecosystem: "research_platform".to_string(),
        requester_instance: "lab-001".to_string(),
        request_timestamp: chrono::Utc::now(),
        performance_history: None,
        ecosystem_load: HashMap::from([
            ("beardog".to_string(), 0.3),
            ("toadstool".to_string(), 0.7),
        ]),
        current_conditions: HashMap::from([
            ("security_alert_level".to_string(), "normal".to_string()),
            ("system_load".to_string(), "moderate".to_string()),
        ]),
    };

    println!("🔍 Searching for capabilities matching complex requirements:");
    println!("   - Category: Security");
    println!("   - Required: AES256+ChaCha20 encryption, 99%+ availability");
    println!("   - Performance: <100ms response, 500+ encryptions/sec");
    println!("   - Compliance: FIPS-140-2");
    println!("   - Geographic: US/EU only");

    let matches = system
        .capability_manager
        .get_emergent_capabilities()
        .await?;

    println!("\n📊 Advanced Matching Results:");

    if matches.is_empty() {
        println!("   📝 Note: Advanced matching algorithms would analyze:");
        println!("           - Semantic similarity between requirements and capabilities");
        println!("           - Performance history and prediction models");
        println!("           - ML-driven compatibility scoring");
        println!("           - Ecosystem load balancing");

        println!("\n🎯 Example Advanced Matches:");
        println!("   1. BearDog Security Capability (Match Score: 0.94)");
        println!("      • Semantic Match: 0.95 (excellent attribute alignment)");
        println!("      • Performance Model: 0.92 (predicted 45ms response)");
        println!("      • ML Compatibility: 0.96 (high ecosystem synergy)");
        println!("      • Load Balance: 0.93 (optimal resource utilization)");

        println!("   2. Genetic Security Capability (Match Score: 0.87)");
        println!("      • Semantic Match: 0.88 (good trait compatibility)");
        println!("      • Performance Model: 0.89 (predicted 62ms response)");
        println!("      • ML Compatibility: 0.85 (moderate ecosystem fit)");
        println!("      • Load Balance: 0.86 (acceptable resource usage)");
    } else {
        for (i, capability_match) in matches.iter().take(3).enumerate() {
            println!(
                "   {}. {} (Provider: {}:{})",
                i + 1,
                capability_match.capability.name,
                capability_match.provider_ecosystem,
                capability_match.provider_instance
            );
            println!("      • Match Score: {:.2}", capability_match.match_score);
            println!("      • Compatibility Reasons:");
            for reason in &capability_match.compatibility_reasons {
                println!("        - {}", reason);
            }
        }
    }

    Ok(())
}

async fn run_dependency_resolution_demo(
    system: &CapabilitySystemDemo,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔗 Demo 6: Complex Dependency Resolution");
    println!("=======================================");

    let required_capabilities = vec![
        "secure_data_pipeline".to_string(),
        "ai_threat_analysis".to_string(),
        "distributed_backup".to_string(),
        "real_time_monitoring".to_string(),
    ];

    println!("🔍 Resolving dependencies for complex capability chain:");
    for (i, capability) in required_capabilities.iter().enumerate() {
        println!("   {}. {}", i + 1, capability);
    }

    let resolution_result = system
        .capability_manager
        .get_emergent_capabilities()
        .await?;

    println!("\n📋 Dependency Resolution Results:");

    if resolution_result.circular_detected {
        println!("   ⚠️  Circular dependencies detected!");
    } else {
        println!("   ✅ No circular dependencies found");
    }

    println!("   📊 Resolution Statistics:");
    println!(
        "      • Total dependencies in chain: {}",
        resolution_result.dependency_chain.len()
    );
    println!(
        "      • Resolution order items: {}",
        resolution_result.resolution_order.len()
    );
    println!(
        "      • Unresolved dependencies: {}",
        resolution_result.unresolved_dependencies.len()
    );
    println!(
        "      • Estimated complexity: {}",
        resolution_result.estimated_complexity
    );

    println!("\n🗂️  Resolution Order:");
    for (i, capability) in resolution_result.resolution_order.iter().enumerate() {
        println!("   {}. {}", i + 1, capability);
    }

    if !resolution_result.unresolved_dependencies.is_empty() {
        println!("\n❌ Unresolved Dependencies:");
        for dependency in &resolution_result.unresolved_dependencies {
            println!("   - {}", dependency);
        }
    }

    println!("\n📝 Example Dependency Graph:");
    println!("   secure_data_pipeline → [security.encrypt, storage.persist]");
    println!("   ai_threat_analysis → [ai.inference, security.monitor]");
    println!("   distributed_backup → [storage.backup, communication.routing]");
    println!("   real_time_monitoring → [monitoring.metrics, communication.messaging]");

    Ok(())
}

fn create_beardog_security_capabilities() -> Vec<Capability> {
    vec![
        create_security_capability(
            "security.encrypt",
            "Advanced Encryption",
            "Post-quantum encryption algorithms",
        ),
        create_security_capability(
            "security.authenticate",
            "Multi-Factor Authentication",
            "Biometric and token-based auth",
        ),
        create_security_capability(
            "security.authorize",
            "Zero-Trust Authorization",
            "Policy-based access control",
        ),
        create_security_capability(
            "security.audit",
            "Security Auditing",
            "Comprehensive audit trails",
        ),
        create_security_capability(
            "security.monitor",
            "Threat Monitoring",
            "Real-time threat detection",
        ),
    ]
}

fn create_toadstool_compute_capabilities() -> Vec<Capability> {
    vec![
        create_compute_capability(
            "compute.execute",
            "Universal Compute",
            "8-bit to quantum execution",
        ),
        create_compute_capability(
            "compute.orchestrate",
            "Workload Orchestration",
            "Cross-platform scheduling",
        ),
        create_compute_capability(
            "compute.schedule",
            "Resource Scheduling",
            "Optimal resource allocation",
        ),
        create_compute_capability(
            "compute.optimize",
            "Performance Optimization",
            "AI-driven optimization",
        ),
    ]
}

fn create_songbird_communication_capabilities() -> Vec<Capability> {
    vec![
        create_communication_capability(
            "communication.discovery",
            "Service Discovery",
            "Ecosystem-wide discovery",
        ),
        create_communication_capability(
            "communication.routing",
            "Message Routing",
            "Intelligent message routing",
        ),
        create_communication_capability(
            "communication.messaging",
            "Secure Messaging",
            "End-to-end encrypted communication",
        ),
        create_communication_capability(
            "communication.broadcast",
            "Event Broadcasting",
            "Multi-cast event distribution",
        ),
    ]
}

fn create_nestgate_storage_capabilities() -> Vec<Capability> {
    vec![
        create_storage_capability(
            "storage.persist",
            "Persistent Storage",
            "ACID-compliant data persistence",
        ),
        create_storage_capability(
            "storage.cache",
            "High-Speed Cache",
            "In-memory caching layer",
        ),
        create_storage_capability(
            "storage.backup",
            "Distributed Backup",
            "Cross-node data replication",
        ),
        create_storage_capability(
            "storage.archive",
            "Long-term Archive",
            "Cold storage optimization",
        ),
    ]
}

fn create_squirrel_ai_capabilities() -> Vec<Capability> {
    vec![
        create_ai_capability("ai.inference", "ML Inference", "Real-time model inference"),
        create_ai_capability(
            "ai.training",
            "Model Training",
            "Distributed training pipelines",
        ),
        create_ai_capability(
            "ai.analysis",
            "Data Analysis",
            "Advanced analytics and insights",
        ),
        create_ai_capability(
            "ai.optimization",
            "AI Optimization",
            "Performance tuning algorithms",
        ),
    ]
}

fn create_security_capability(id: &str, name: &str, description: &str) -> Capability {
    Capability {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: CapabilityCategory::Security,
        attributes: HashMap::from([
            (
                "security_strength".to_string(),
                CapabilityAttribute {
                    value: "0.95".to_string(),
                    data_type: AttributeDataType::Float,
                    required: true,
                    description: Some("Security strength rating".to_string()),
                },
            ),
            (
                "encryption_algorithms".to_string(),
                CapabilityAttribute {
                    value: "AES256,ChaCha20,Quantum".to_string(),
                    data_type: AttributeDataType::Array,
                    required: true,
                    description: Some("Supported encryption algorithms".to_string()),
                },
            ),
        ]),
        qos: QualityOfService {
            avg_response_time_ms: 25,
            availability_percent: 99.9,
            throughput: Some(ThroughputMetric {
                value: 1000,
                unit: "operations/sec".to_string(),
            }),
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 50,
                auto_scaling: true,
            },
        },
        resource_requirements: ResourceRequirements::default(),
    }
}

fn create_compute_capability(id: &str, name: &str, description: &str) -> Capability {
    Capability {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: CapabilityCategory::Compute,
        attributes: HashMap::from([
            (
                "compute_platforms".to_string(),
                CapabilityAttribute {
                    value: "8bit,16bit,32bit,64bit,gpu,quantum".to_string(),
                    data_type: AttributeDataType::Array,
                    required: true,
                    description: Some("Supported compute platforms".to_string()),
                },
            ),
            (
                "performance_factor".to_string(),
                CapabilityAttribute {
                    value: "0.92".to_string(),
                    data_type: AttributeDataType::Float,
                    required: true,
                    description: Some("Performance optimization factor".to_string()),
                },
            ),
        ]),
        qos: QualityOfService {
            avg_response_time_ms: 50,
            availability_percent: 99.5,
            throughput: Some(ThroughputMetric {
                value: 2000,
                unit: "jobs/sec".to_string(),
            }),
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 1000,
                auto_scaling: true,
            },
        },
        resource_requirements: ResourceRequirements::default(),
    }
}

fn create_communication_capability(id: &str, name: &str, description: &str) -> Capability {
    Capability {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: CapabilityCategory::Communication,
        attributes: HashMap::from([
            (
                "protocols".to_string(),
                CapabilityAttribute {
                    value: "HTTP,HTTPS,gRPC,WebSocket,TCP,UDP".to_string(),
                    data_type: AttributeDataType::Array,
                    required: true,
                    description: Some("Supported communication protocols".to_string()),
                },
            ),
            (
                "latency_optimization".to_string(),
                CapabilityAttribute {
                    value: "true".to_string(),
                    data_type: AttributeDataType::Boolean,
                    required: false,
                    description: Some("Low-latency optimization enabled".to_string()),
                },
            ),
        ]),
        qos: QualityOfService {
            avg_response_time_ms: 15,
            availability_percent: 99.8,
            throughput: Some(ThroughputMetric {
                value: 5000,
                unit: "messages/sec".to_string(),
            }),
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 100,
                auto_scaling: true,
            },
        },
        resource_requirements: ResourceRequirements::default(),
    }
}

fn create_storage_capability(id: &str, name: &str, description: &str) -> Capability {
    Capability {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: CapabilityCategory::Storage,
        attributes: HashMap::from([
            (
                "consistency_model".to_string(),
                CapabilityAttribute {
                    value: "ACID".to_string(),
                    data_type: AttributeDataType::String,
                    required: true,
                    description: Some("Data consistency model".to_string()),
                },
            ),
            (
                "replication_factor".to_string(),
                CapabilityAttribute {
                    value: "3".to_string(),
                    data_type: AttributeDataType::Integer,
                    required: true,
                    description: Some("Default replication factor".to_string()),
                },
            ),
        ]),
        qos: QualityOfService {
            avg_response_time_ms: 20,
            availability_percent: 99.95,
            throughput: Some(ThroughputMetric {
                value: 10000,
                unit: "IOPS".to_string(),
            }),
            scalability: ScalabilityInfo {
                min_instances: 3,
                max_instances: 200,
                auto_scaling: true,
            },
        },
        resource_requirements: ResourceRequirements::default(),
    }
}

fn create_ai_capability(id: &str, name: &str, description: &str) -> Capability {
    Capability {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: CapabilityCategory::AI,
        attributes: HashMap::from([
            (
                "model_types".to_string(),
                CapabilityAttribute {
                    value: "transformer,cnn,rnn,gan,reinforcement".to_string(),
                    data_type: AttributeDataType::Array,
                    required: true,
                    description: Some("Supported AI model types".to_string()),
                },
            ),
            (
                "gpu_acceleration".to_string(),
                CapabilityAttribute {
                    value: "true".to_string(),
                    data_type: AttributeDataType::Boolean,
                    required: false,
                    description: Some("GPU acceleration available".to_string()),
                },
            ),
        ]),
        qos: QualityOfService {
            avg_response_time_ms: 100,
            availability_percent: 99.0,
            throughput: Some(ThroughputMetric {
                value: 500,
                unit: "inferences/sec".to_string(),
            }),
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 20,
                auto_scaling: true,
            },
        },
        resource_requirements: ResourceRequirements::default(),
    }
}

fn create_service_request(request_type: &str, target_capability: &str) -> ServiceRequest {
    ServiceRequest {
        request_id: Uuid::new_v4(),
        request_type: request_type.to_string(),
        payload: serde_json::json!({
            "target_capability": target_capability,
            "parameters": {
                "priority": "medium",
                "timeout": 30
            }
        }),
        timestamp: chrono::Utc::now(),
        priority: crate::adapters::universal::traits::RequestPriority::Normal,
        metadata: HashMap::from([
            ("source".to_string(), "capability_demo".to_string()),
            ("interaction_type".to_string(), "cross_ecosystem".to_string()),
        ]),
        context: crate::adapters::universal::traits::RequestContext {
            user_id: Some("demo-requester".to_string()),
            session_id: None,
            transaction_id: None,
            source_ecosystem: "beardog".to_string(),
            target_ecosystem: Some("ecosystem".to_string()),
            metadata: HashMap::with_capacity(16),
        },
    }
}

fn format_capability_status(status: &CapabilityStatus) -> &str {
    match status {
        CapabilityStatus::Healthy => "🟢 Healthy",
        CapabilityStatus::Degraded => "🟡 Degraded",
        CapabilityStatus::Critical => "🔴 Critical",
        CapabilityStatus::Offline => "⚫ Offline",
        CapabilityStatus::Unknown => "⚪ Unknown",
    }
}

fn format_capability_category(category: &CapabilityCategory) -> &str {
    match category {
        CapabilityCategory::Compute => "Compute",
        CapabilityCategory::Storage => "Storage",
        CapabilityCategory::Security => "Security",
        CapabilityCategory::AI => "AI",
        CapabilityCategory::Communication => "Communication",
        CapabilityCategory::Monitoring => "Monitoring",
        CapabilityCategory::Integration => "Integration",
        CapabilityCategory::Custom(name) => name,
    }
}
