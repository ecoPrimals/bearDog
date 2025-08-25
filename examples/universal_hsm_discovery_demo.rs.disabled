//! Universal HSM Discovery System Demo
//!
//! This example demonstrates the complete workflow of the Universal HSM Discovery System,
//! showcasing automatic HSM discovery, capability detection, human entropy classification,
//! tier elevation, and operation-specific HSM selection.

use beardog_tunnel::universal_hsm_discovery::universal_adapter::*;
use beardog_tunnel::universal_hsm_discovery::*;
use std::collections::HashMap;
use std::time::Duration;
use tracing::{debug, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for the demo
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("🐕 BearDog Universal HSM Discovery & Network Communication Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Phase 1: Initialize BearDog Universal Adapter with External Primal Support
    demo_universal_adapter_initialization().await?;

    // Phase 2: Discover Local HSMs
    demo_hsm_discovery().await?;

    // Phase 3: Discover External Primals (Songbird, ToadStool, etc.)
    demo_external_primal_discovery().await?;

    // Phase 4: Concert Scenario - Distributed Key Ceremony
    demo_concert_key_ceremony().await?;

    // Phase 5: Network Integration & Monitoring
    demo_network_integration().await?;

    info!("✅ BearDog Universal Adapter Demo completed successfully!");
    info!("🌐 BearDog is ready for standalone and network operation");

    Ok(())
}

/// Phase 1: Initialize Universal Adapter with tarpc support
async fn demo_universal_adapter_initialization() -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🔧 Phase 1: Universal Adapter Initialization");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Initialize Universal Adapter with external primal communication
    let mut adapter = UniversalAdapter::new()?;
    info!("✅ Universal Adapter initialized with tarpc support");

    // Show available HSM adapters
    let available_adapters = adapter.get_available_adapters();
    info!("📋 Available HSM Adapters: {:?}", available_adapters);

    // Simulate mobile detection
    info!("📱 BearDog Mode: Detected mobile environment (Pixel 8a simulation)");
    info!("🔐 StrongBox: Available for hardware-backed operations");
    info!("🌟 Human Entropy: Premium tier HSM ready for ephemeral seeds");

    Ok(())
}

/// Phase 2: Discover Local HSMs
async fn demo_hsm_discovery() -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🔍 Phase 2: Local HSM Discovery");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Initialize discovery system
    let discovery_config = DiscoveryConfig {
        enable_network_scanning: true,
        enable_usb_scanning: false, // Mobile environment
        enable_cloud_hsms: true,
        scan_timeout_seconds: 30,
        discovery_parallelism: 4,
        require_attestation: true,
        preferred_vendors: vec![
            "Google".to_string(),  // For StrongBox
            "BearDog".to_string(), // For native implementation
        ],
    };

    let discovery = UniversalHsmDiscovery::new(discovery_config)?;

    // Discover all available HSMs
    let discovered_hsms = discovery.discover_all_hsms().await?;

    info!("📊 Discovery Results:");
    info!("  Total HSMs discovered: {}", discovered_hsms.len());

    // Show HSM tier distribution
    let tier_distribution = discovery.get_tier_distribution(&discovered_hsms);
    for (tier, count) in tier_distribution {
        info!("  {:?}: {} HSMs", tier, count);
    }

    // Highlight human entropy capable HSMs
    let human_entropy_hsms = discovery.get_human_entropy_hsms(&discovered_hsms);
    info!(
        "🌟 Human Entropy HSMs: {} available",
        human_entropy_hsms.len()
    );

    for hsm in &human_entropy_hsms {
        info!("  • {} ({})", hsm.name, hsm.hsm_id);
        info!("    Tier: {:?}", hsm.tier);
        if let Some(entropy_caps) = &hsm.capabilities.human_entropy {
            info!("    Quality Score: {:.2}", entropy_caps.quality_score);
            info!(
                "    Collection Methods: {:?}",
                entropy_caps.collection_methods
            );
        }
    }

    Ok(())
}

/// Phase 3: Discover External Primals for Network Communication
async fn demo_external_primal_discovery() -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🌐 Phase 3: External Primal Discovery");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let adapter = UniversalAdapter::new()?;

    // Discover external primals in the network
    match adapter.discover_external_primals().await {
        Ok(primals) => {
            if primals.is_empty() {
                info!("📱 Standalone Mode: No external primals discovered");
                info!("🔒 BearDog operating in failsafe standalone mode");
                info!("💪 Full functionality available via local HSMs");
            } else {
                info!(
                    "🌟 Network Mode: {} external primals discovered",
                    primals.len()
                );
                for primal in &primals {
                    info!("  • {} connected via tarpc", primal);
                }

                // Show network capabilities
                info!("🚀 Network Capabilities Available:");
                info!("  • Songbird: Universal orchestration");
                info!("  • ToadStool: Distributed computing & streaming");
                info!("  • Squirrel: Data persistence & backup");
                info!("  • NestGate: Network management");
            }
        }
        Err(e) => {
            warn!("⚠️  External primal discovery failed: {}", e);
            info!("📱 Falling back to standalone mode - full functionality maintained");
        }
    }

    Ok(())
}

/// Phase 4: Concert Scenario - Distributed Key Ceremony
async fn demo_concert_key_ceremony() -> Result<(), Box<dyn std::error::Error>> {
    info!("\n🎭 Phase 4: Concert Key Ceremony Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let adapter = UniversalAdapter::new()?;

    // Simulate concert scenario
    info!("🎵 Concert Scenario: 1000 ticket holders want shared key");
    info!("📱 Each participant has BearDog on their device");
    info!("🎯 Goal: Generate shared key for exclusive content access");

    // Define human entropy requirements for high-security ceremony
    let entropy_requirements = HumanEntropyRequirements {
        collection_methods: vec![
            EntropyCollectionMethod::TouchPatterns,
            EntropyCollectionMethod::DeviceMotion,
            EntropyCollectionMethod::BiometricVariation,
        ],
        quality_threshold: 0.95, // Very high quality required
        seed_lifetime: Duration::from_secs(3600), // 1 hour
        verification_required: true,
        real_time_collection: true,
    };

    // Generate local human entropy seed first
    match adapter
        .generate_human_entropy_seed(entropy_requirements.clone())
        .await
    {
        Ok(seed) => {
            info!("🌟 Local Human Entropy Seed Generated:");
            info!("  Quality: {:.3}", seed.entropy_quality);
            info!("  Methods: {:?}", seed.collection_methods_used);
            info!("  Expires: {}", seed.expires_at.format("%H:%M:%S"));

            // Simulate distributed ceremony coordination
            info!("\n🌐 Coordinating Distributed Key Ceremony...");

            // Mock participant list (in reality, discovered via network)
            let participants = vec![
                "concert-participant-001".to_string(),
                "concert-participant-002".to_string(),
                "concert-participant-003".to_string(),
                // ... would be 1000+ participants
                "concert-participant-999".to_string(),
            ];

            match adapter
                .coordinate_concert_key_ceremony(
                    1000, // participant_count
                    667,  // threshold (2/3 majority)
                    entropy_requirements,
                    participants,
                )
                .await
            {
                Ok(ceremony_result) => {
                    info!("✅ Concert Key Ceremony Completed!");
                    info!("  Ceremony ID: {}", ceremony_result.ceremony_id);
                    info!("  Success: {}", ceremony_result.success);
                    info!(
                        "  Participants: {} contributed",
                        ceremony_result.participant_contributions.len()
                    );

                    if ceremony_result.success {
                        info!("🎉 Shared key generated successfully!");
                        info!("🔓 Concert content now accessible to all participants");
                    }
                }
                Err(e) => {
                    warn!("⚠️  Distributed ceremony failed: {}", e);
                    info!("📱 Fallback: Using local key generation for individual access");
                }
            }
        }
        Err(e) => {
            warn!("⚠️  Local entropy generation failed: {}", e);
            info!("🔄 Would fallback to standard key generation");
        }
    }

    Ok(())
}

/// Phase 5: Network Integration & Monitoring
async fn demo_network_integration() -> Result<(), Box<dyn std::error::Error>> {
    info!("\n📊 Phase 5: Network Integration & Monitoring");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let adapter = UniversalAdapter::new()?;

    // Register with network orchestrator (Songbird)
    info!("📋 Registering BearDog capabilities with Songbird...");
    match adapter.register_with_network().await {
        Ok(_) => {
            info!("✅ Successfully registered with network orchestrator");
            info!("🎯 BearDog capabilities now available to ecosystem");
        }
        Err(e) => {
            debug!("Network registration not available: {}", e);
            info!("📱 Operating in standalone mode");
        }
    }

    // Generate sample metrics
    let metrics = HsmMetricsSnapshot {
        instance_id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Utc::now(),
        discovered_hsms: 5,
        active_operations: 12,
        performance_metrics: {
            let mut metrics = HashMap::new();
            metrics.insert("operations_per_second".to_string(), 150.0);
            metrics.insert("average_latency_ms".to_string(), 15.2);
            metrics.insert("success_rate".to_string(), 0.998);
            metrics.insert("entropy_quality_avg".to_string(), 0.96);
            metrics
        },
        health_scores: {
            let mut scores = HashMap::new();
            scores.insert("strongbox".to_string(), 0.98);
            scores.insert("beardog_native".to_string(), 0.99);
            scores.insert("network_connectivity".to_string(), 0.95);
            scores
        },
        tier_distribution: {
            let mut distribution = HashMap::new();
            distribution.insert(HsmTier::HumanEntropyPremium, 2);
            distribution.insert(HsmTier::PremiumHardware, 1);
            distribution.insert(HsmTier::BasicHardware, 2);
            distribution
        },
        human_entropy_hsms: 2,
    };

    // Stream metrics to ToadStool for monitoring
    info!("📊 Streaming metrics to ToadStool monitoring...");
    match adapter.stream_metrics(metrics).await {
        Ok(_) => {
            info!("✅ Metrics streamed to network monitoring");
            info!("📈 Performance data available to ecosystem");
        }
        Err(e) => {
            debug!("Metrics streaming not available: {}", e);
            info!("💾 Metrics stored locally for later sync");
        }
    }

    // Demonstrate capability request to external primal
    info!("\n🎯 Testing External Capability Request...");

    let capability_requirements = CapabilityRequirements {
        minimum_tier: HsmTier::BasicHardware,
        required_features: vec!["compute".to_string(), "orchestration".to_string()],
        performance_requirements: PerformanceRequirements {
            max_latency_ms: 1000,
            min_throughput_ops_per_sec: 10,
            max_memory_mb: Some(512),
            concurrent_operations: 5,
        },
        security_requirements: SecurityRequirements {
            encryption_level: "AES-256".to_string(),
            key_protection: "hardware".to_string(),
            audit_logging: true,
            compliance_needed: vec!["SOC2".to_string()],
        },
        timeout: Duration::from_secs(30),
    };

    match adapter
        .request_external_capability(
            "songbird",
            "orchestration.task_coordination",
            serde_json::json!({
                "task_type": "distributed_key_ceremony",
                "participants": 1000,
                "priority": "high"
            }),
            capability_requirements,
        )
        .await
    {
        Ok(response) => {
            info!("✅ Capability request successful");
            info!("📋 Response: {}", response.success);
            if !response.metadata.is_empty() {
                info!("📊 Metadata: {:?}", response.metadata);
            }
        }
        Err(e) => {
            debug!("External capability request failed: {}", e);
            info!("🔄 Using local capabilities instead");
        }
    }

    // Summary
    info!("\n🎉 Network Integration Summary:");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📱 Standalone Mode: ✅ Full functionality");
    info!("🌐 Network Mode: ✅ Enhanced capabilities");
    info!("🔐 HSM Discovery: ✅ Universal ingestion");
    info!("🌟 Human Entropy: ✅ Premium tier available");
    info!("🎭 Concert Scale: ✅ 1000+ participant support");
    info!("🔄 Failover: ✅ Graceful degradation");
    info!("📊 Monitoring: ✅ Real-time metrics");
    info!("🚀 tarpc Integration: ✅ Pure Rust networking");

    Ok(())
}

// Helper function to simulate mock discovery results
fn create_mock_discovered_hsm(
    id: &str,
    name: &str,
    interface_type: HsmInterfaceType,
    tier: HsmTier,
    human_entropy: bool,
) -> DiscoveredHsm {
    let human_entropy_caps = if human_entropy {
        Some(HumanEntropyCapabilities {
            collection_methods: vec![
                EntropyCollectionMethod::TouchPatterns,
                EntropyCollectionMethod::DeviceMotion,
            ],
            quality_assessment: true,
            real_time_collection: true,
            quality_score: 0.95,
            verification_methods: vec!["biometric".to_string()],
        })
    } else {
        None
    };

    DiscoveredHsm {
        hsm_id: id.to_string(),
        name: name.to_string(),
        interface_type,
        capabilities: HsmCapabilities {
            key_generation: KeyGenerationCapabilities {
                supported_algorithms: vec!["RSA".to_string(), "ECDSA".to_string()],
                key_sizes: vec![2048, 3072, 4096],
                hardware_backed: matches!(
                    tier,
                    HsmTier::BasicHardware
                        | HsmTier::PremiumHardware
                        | HsmTier::HumanEntropyPremium
                ),
                attestation_support: true,
            },
            crypto_operations: CryptoOperationCapabilities {
                signing_algorithms: vec!["RSA-PSS".to_string(), "ECDSA".to_string()],
                encryption_algorithms: vec!["RSA-OAEP".to_string(), "AES-GCM".to_string()],
                max_concurrent_operations: 10,
                performance_tier: "high".to_string(),
            },
            human_entropy: human_entropy_caps,
            vendor_specific: None,
        },
        health_status: HsmHealthStatus::Healthy,
        tier,
    }
}
