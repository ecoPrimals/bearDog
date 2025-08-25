//! Context-Aware Licensing Demo
//!
//! This example demonstrates how BearDog's context-aware licensing system
//! automatically detects usage patterns and adapts licensing accordingly.

use beardog_core::context_aware_licensing::{
    ClassificationEvidence, ContextAwareLicense, ContextualClassification, EnterpriseIndicator,
    EnterprisePricingModel, FunctionUsagePattern, HardwareProfile, IndividualIndicator,
    IndividualType, IntegrationComplexity, NetworkProfile, OrganizationSize, PrimaryUseCase,
    PurposeAnalysis,
};
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌟 BearDog Context-Aware Licensing System Demo");
    println!("=================================================");

    // Scenario 1: Individual Developer
    println!("\n📱 SCENARIO 1: Individual Developer");
    demonstrate_individual_detection().await;

    // Scenario 2: Small Business
    println!("\n🏢 SCENARIO 2: Small Business");
    demonstrate_small_business_detection().await;

    // Scenario 3: Enterprise
    println!("\n🏭 SCENARIO 3: Enterprise Organization");
    demonstrate_enterprise_detection().await;

    // Scenario 4: Educational Institution
    println!("\n🎓 SCENARIO 4: Educational Institution");
    demonstrate_educational_detection().await;

    // Scenario 5: Context Evolution
    println!("\n🧬 SCENARIO 5: Context Evolution (Individual → Enterprise)");
    demonstrate_context_evolution().await;

    println!("\n✅ Demo completed! BearDog keys are now context-aware and self-evolving!");

    Ok(())
}

async fn demonstrate_individual_detection() {
    println!("Context Analysis: Personal laptop, evening hours, GitHub projects...");

    let evidence = vec![
        ClassificationEvidence::InfrastructurePattern("Single MacBook Pro".to_string()),
        ClassificationEvidence::TimePattern("Evening and weekend usage".to_string()),
        ClassificationEvidence::DataVolume(0.5), // 500MB per day
        ClassificationEvidence::NetworkPattern("Home WiFi, residential IP".to_string()),
        ClassificationEvidence::FunctionUsage("2 basic functions, low frequency".to_string()),
        ClassificationEvidence::EmailDomain("gmail.com".to_string()),
    ];

    let classification = ContextualClassification::Individual {
        subtype: IndividualType::Hobbyist,
        indicators: vec![
            IndividualIndicator::SingleDeveloperEnvironment,
            IndividualIndicator::PersonalEmailDomain("gmail.com".to_string()),
            IndividualIndicator::HomeNetworkProfile,
            IndividualIndicator::LimitedInfrastructure {
                max_connections: 5,
                max_data_volume: 1000, // 1GB max
            },
        ],
    };

    println!("🎯 Detection Result: {:?}", classification);
    println!("💰 License Tier: FREE (Community) - Hobbyist individual developer");
    println!("🔑 Available Functions: All basic functions + Rust ecosystem integrations");
    println!("⏰ Renewal: Automatic genetic evolution, no expiration");
    println!("🛡️ Verification: Self-signed proofs with usage pattern evidence");
}

async fn demonstrate_small_business_detection() {
    println!("Context Analysis: Small office, business hours, team collaboration...");

    let evidence = vec![
        ClassificationEvidence::InfrastructurePattern(
            "3-server setup with load balancer".to_string(),
        ),
        ClassificationEvidence::TimePattern("9-5 business hours, 5 day pattern".to_string()),
        ClassificationEvidence::DataVolume(50.0), // 50GB per day
        ClassificationEvidence::NetworkPattern("Business ISP, static IP".to_string()),
        ClassificationEvidence::FunctionUsage("5 functions including monitoring".to_string()),
        ClassificationEvidence::EmailDomain("smallbusiness.com".to_string()),
    ];

    let classification = ContextualClassification::SmallBusiness {
        estimated_size: 8,
        indicators: vec![
            // Would have BusinessIndicators here in full implementation
        ],
    };

    println!("🎯 Detection Result: {:?}", classification);
    println!("💰 License Tier: COMMUNITY (with business features)");
    println!("🔑 Available Functions: All functions except enterprise HSM/AD");
    println!("⏰ Renewal: 12 month cycles with gentle pricing");
    println!("🛡️ Verification: Business verification with peer attestation");
}

async fn demonstrate_enterprise_detection() {
    println!("Context Analysis: Kubernetes cluster, multi-datacenter, enterprise functions...");

    let evidence = vec![
        ClassificationEvidence::InfrastructurePattern("Kubernetes cluster, 50+ nodes".to_string()),
        ClassificationEvidence::TimePattern("24/7 production workload".to_string()),
        ClassificationEvidence::DataVolume(500.0), // 500GB per day
        ClassificationEvidence::NetworkPattern("Corporate VPN, enterprise network".to_string()),
        ClassificationEvidence::FunctionUsage(
            "15+ enterprise functions including AWS KMS, AD".to_string(),
        ),
        ClassificationEvidence::EmailDomain("enterprise.corp".to_string()),
        ClassificationEvidence::Geographic("Multiple office locations".to_string()),
    ];

    let classification = ContextualClassification::Enterprise {
        estimated_size: OrganizationSize::Large,
        indicators: vec![
            EnterpriseIndicator::CorporateDomain("enterprise.corp".to_string()),
            EnterpriseIndicator::EnterpriseInfrastructure {
                kubernetes_detected: true,
                multi_datacenter: true,
                high_availability: true,
            },
            EnterpriseIndicator::EnterpriseIntegrations(vec![
                "active_directory".to_string(),
                "aws_kms_integration".to_string(),
                "prometheus_export".to_string(),
                "splunk_integration".to_string(),
            ]),
            EnterpriseIndicator::HighVolumeUsage {
                daily_transactions: 1_000_000,
                data_volume_tb: 0.5,
            },
            EnterpriseIndicator::MultiTeamCollaboration {
                team_size: 150,
                patterns: vec![
                    "DevOps".to_string(),
                    "Security".to_string(),
                    "Platform".to_string(),
                ],
            },
        ],
        pricing_model: EnterprisePricingModel::UsageBased,
    };

    println!("🎯 Detection Result: {:?}", classification);
    println!("💰 License Tier: ENTERPRISE - Usage-based pricing");
    println!("💵 Estimated Cost: $2,000/month (based on usage patterns)");
    println!(
        "🔑 Available Functions: ALL functions including HSM, cloud providers, enterprise identity"
    );
    println!("⏰ Renewal: Quarterly with usage optimization");
    println!("🛡️ Verification: Enterprise cryptographic verification chain");
    println!("🎁 Value Delivered: Funds 20 free individual developer licenses");
}

async fn demonstrate_educational_detection() {
    println!("Context Analysis: University network, .edu domain, research patterns...");

    println!("🎯 Detection Result: Educational Institution (University)");
    println!("💰 License Tier: EDUCATIONAL - Always FREE");
    println!("🔑 Available Functions: All functions for educational use");
    println!("📚 Special Features: Research collaboration tools, student access");
    println!("⏰ Renewal: Permanent with annual attestation");
    println!("🛡️ Verification: Educational domain verification + research contribution tracking");
}

async fn demonstrate_context_evolution() {
    println!("📈 EVOLUTION TIMELINE:");
    println!("Month 1: Individual hobbyist → FREE license");
    println!("Month 6: Started freelance consulting → Still FREE (individual freelancer)");
    println!("Month 12: Hired 2 employees → Detected as Small Business");
    println!("Month 18: Kubernetes deployment detected → Enterprise classification triggered");

    println!("\n🧬 GENETIC EVOLUTION PROCESS:");
    println!("1. Context change detected (individual → enterprise)");
    println!("2. Grace period initiated (30 days to transition)");
    println!("3. Key spawns evolved version with enterprise capabilities");
    println!("4. Gradual feature unlock as payment method configured");
    println!("5. Full enterprise features enabled");

    println!("\n🎯 FAIRNESS ALGORITHM:");
    println!("✅ No sudden cutoff - gradual transition");
    println!("✅ 30-day grace period for legitimate growth");
    println!("✅ Usage-based pricing from day 1 of enterprise features");
    println!("✅ Credit for previous community contributions");
    println!("✅ Option to appeal classification with evidence");

    println!("\n🔄 AUTOMATIC RENEWAL:");
    println!("- Key evolved to understand new enterprise context");
    println!("- Spawned child keys for different teams/projects");
    println!("- Each key inherits appropriate permissions");
    println!("- Genetic traits: higher security, better performance optimization");
    println!("- Self-renewal prevents any service interruption");
}

// Demonstration of creative detection methods
fn demonstrate_creative_detection_methods() {
    println!("\n🕵️ CREATIVE DETECTION METHODS:");

    println!("\n🔍 METHOD 1: Infrastructure Fingerprinting");
    println!("- Consumer laptop: Individual");
    println!("- Docker Swarm: Small Business");
    println!("- Kubernetes + Multiple DCs: Enterprise");

    println!("\n🕐 METHOD 2: Time Pattern Analysis");
    println!("- Evening/Weekend: Individual hobbyist");
    println!("- 9-5 Weekdays: Business");
    println!("- 24/7 Automated: Enterprise");

    println!("\n🌐 METHOD 3: Network Analysis");
    println!("- Residential ISP: Individual");
    println!("- Business ISP: Small Business");
    println!("- Corporate VPN + Multiple Locations: Enterprise");

    println!("\n📊 METHOD 4: Function Usage Patterns");
    println!("- Basic monitoring: Individual/Small Business");
    println!("- Active Directory + AWS KMS: Enterprise");
    println!("- 10+ external functions: Definitely Enterprise");

    println!("\n📧 METHOD 5: Email Domain Classification");
    println!("- @gmail.com: Individual");
    println!("- @smallbiz.com: Business");
    println!("- @corp.enterprise: Enterprise");
    println!("- @university.edu: Educational");

    println!("\n🎯 ANTI-GAMING MEASURES:");
    println!("- Multiple evidence sources required");
    println!("- Cryptographic proof chains");
    println!("- Peer verification networks");
    println!("- Genetic lineage tracking");
    println!("- Challenge-response verification");
    println!("- Community reputation systems");
}

/// Example of how the key makes decisions
#[allow(dead_code)]
fn example_decision_logic() -> ContextualClassification {
    // This would be the actual decision logic
    let mut confidence_score = 0.0;
    let mut evidence = Vec::new();

    // Infrastructure analysis
    let kubernetes_detected = true;
    if kubernetes_detected {
        confidence_score -= 0.6; // Strongly suggests enterprise
        evidence.push(ClassificationEvidence::InfrastructurePattern(
            "Kubernetes detected".to_string(),
        ));
    }

    // Function usage analysis
    let enterprise_functions = 8;
    if enterprise_functions > 5 {
        confidence_score -= 0.4; // Suggests enterprise
        evidence.push(ClassificationEvidence::FunctionUsage(format!(
            "{} enterprise functions in use",
            enterprise_functions
        )));
    }

    // Time pattern analysis
    let always_on = true;
    if always_on {
        confidence_score -= 0.3; // Suggests production/enterprise
        evidence.push(ClassificationEvidence::TimePattern(
            "24/7 automated usage pattern".to_string(),
        ));
    }

    // Data volume analysis
    let daily_volume_gb = 100.0;
    if daily_volume_gb > 10.0 {
        confidence_score -= 0.2; // High volume suggests business
        evidence.push(ClassificationEvidence::DataVolume(daily_volume_gb));
    }

    // Final classification based on confidence score
    if confidence_score < -0.5 {
        ContextualClassification::Enterprise {
            estimated_size: OrganizationSize::Medium,
            indicators: vec![
                // Enterprise indicators would be populated here
            ],
            pricing_model: EnterprisePricingModel::UsageBased,
        }
    } else if confidence_score > 0.3 {
        ContextualClassification::Individual {
            subtype: IndividualType::Hobbyist,
            indicators: vec![
                // Individual indicators would be populated here
            ],
        }
    } else {
        ContextualClassification::Unknown {
            candidates: vec![],      // Would contain possible classifications
            needed_evidence: vec![], // What evidence is needed
        }
    }
}
