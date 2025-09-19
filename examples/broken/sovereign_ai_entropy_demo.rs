// Sovereign AI Entropy Demonstration
//
// This example demonstrates BearDog's revolutionary approach to AI randomization
// by using human entropy for neural network initialization instead of machine-only randomness.
//
// ## Key Concepts Demonstrated:
// 
// 1. **Human-Owned AI**: Neural networks initialized with human entropy
// 2. **Entropy Hierarchy**: Tier 3 (Human) > Tier 2 (Supervised) > Tier 1 (Machine)
// 3. **Sovereignty**: Humans control their AI's foundational randomness
// 4. **Security**: Cryptographic separation between human and machine entropy

use beardog_core::ai::hybrid_intelligence::{
    neural_networks::{EntropyDistribution, WeightInitialization},
    sovereign_rng::{
        HumanEntropyWeightInitializer, NeuralNetworkEntropyIntegration, SovereignRng,
        SovereignRngConfig,
    },
};
use beardog_errors::BearDogError;
use beardog_genetics::genetics::entropy_hierarchy::EntropyHierarchyManager;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🧠 BearDog Sovereign AI Entropy Demonstration");
    info!("============================================");
    info!("Demonstrating human-controlled AI randomization using entropy hierarchy");

    // Demo 1: Traditional Machine-Only Neural Network
    demo_traditional_machine_ai()?;

    // Demo 2: Human-Supervised AI (Tier 2 Entropy)
    demo_human_supervised_ai()?;

    // Demo 3: Human-Owned AI (Tier 3 Entropy)
    demo_human_owned_ai()?;

    // Demo 4: Sovereignty Comparison
    demo_sovereignty_comparison()?;

    info!("\n🎉 Sovereign AI Entropy Demonstration Complete!");
    info!("Key Takeaway: Humans can now own and control their AI's foundational randomness");

    Ok(())
}

/// Demo 1: Traditional machine-only neural network initialization
async fn demo_traditional_machine_ai() -> Result<(), BearDogError> {
    info!("\n🔧 Demo 1: Traditional Machine-Only AI");
    info!("=====================================");
    info!("Using standard machine-generated randomness (store-bought entropy)");

    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 1, // Allow machine entropy
        cache_entropy: true,
        cache_max_age_seconds: 300,
        allow_machine_fallback: true,
        audit_entropy_usage: true,
    };

    let mut sovereign_rng = SovereignRng::new(entropy_manager, config);

    let machine_initializer = HumanEntropyWeightInitializer {
        entropy_tier: 1, // Tier 1: Store Bought Machine
        human_identity_id: "system_validation".to_string(),
        distribution: EntropyDistribution::Xavier,
        layer_shape: (128, 64), // Input layer: 128 -> 64 hidden units
    };

    let weights = sovereign_rng
        .initialize_weights(&machine_initializer)
        ?;

    info!("   ✅ Generated {}x{} weight matrix using Tier 1 (Machine) entropy", 
          weights.len(), weights[0].len());
    info!("   🔧 Use case: Validation systems, automated testing, CI/CD");
    info!("   📊 Sample weights: [{:.4}, {:.4}, {:.4}]", 
          weights[0][0], weights[0][1], weights[0][2]);

    Ok(())
}

/// Demo 2: Human-supervised AI with Tier 2 entropy
async fn demo_human_supervised_ai() -> Result<(), BearDogError> {
    info!("\n⚡ Demo 2: Human-Supervised AI (Tier 2 Entropy)");
    info!("===============================================");
    info!("Using human-validated machine entropy for business applications");

    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 2, // Require human supervision
        cache_entropy: true,
        cache_max_age_seconds: 300,
        allow_machine_fallback: false, // Strict human validation required
        audit_entropy_usage: true,
    };

    let mut sovereign_rng = SovereignRng::new(entropy_manager, config);

    let supervised_initializer = HumanEntropyWeightInitializer {
        entropy_tier: 2, // Tier 2: Human Supervised Machine
        human_identity_id: "business_analyst_alice".to_string(),
        distribution: EntropyDistribution::Normal { mean: 0.0, stddev: 0.1 },
        layer_shape: (256, 128), // Larger network for business logic
    };

    let weights = sovereign_rng
        .initialize_weights(&supervised_initializer)
        ?;

    info!("   ✅ Generated {}x{} weight matrix using Tier 2 (Human-Supervised) entropy", 
          weights.len(), weights[0].len());
    info!("   👤 Human validator: business_analyst_alice");
    info!("   🏢 Use case: Business intelligence, recommendation systems");
    info!("   📊 Sample weights: [{:.4}, {:.4}, {:.4}]", 
          weights[0][0], weights[0][1], weights[0][2]);

    Ok(())
}

/// Demo 3: Human-owned AI with Tier 3 entropy (highest quality)
async fn demo_human_owned_ai() -> Result<(), BearDogError> {
    info!("\n🔥 Demo 3: Human-Owned AI (Tier 3 Entropy)");
    info!("===========================================");
    info!("Using human-lived-experience entropy for personal AI systems");

    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 3, // Require highest quality human entropy
        cache_entropy: true,
        cache_max_age_seconds: 600, // Longer cache for expensive entropy
        allow_machine_fallback: false, // No fallback - human entropy only
        audit_entropy_usage: true,
    };

    let mut sovereign_rng = SovereignRng::new(entropy_manager, config);

    let human_owned_initializer = HumanEntropyWeightInitializer {
        entropy_tier: 3, // Tier 3: Human Lived Experience
        human_identity_id: "individual_bob_personal_ai".to_string(),
        distribution: EntropyDistribution::He, // He initialization for deep networks
        layer_shape: (512, 256), // Large personal AI network
    };

    let weights = sovereign_rng
        .initialize_weights(&human_owned_initializer)
        ?;

    info!("   ✅ Generated {}x{} weight matrix using Tier 3 (Human-Lived-Experience) entropy", 
          weights.len(), weights[0].len());
    info!("   👤 Human owner: individual_bob_personal_ai");
    info!("   🏠 Use case: Personal assistants, private AI, health AI");
    info!("   📊 Sample weights: [{:.4}, {:.4}, {:.4}]", 
          weights[0][0], weights[0][1], weights[0][2]);
    info!("   🔒 Security: Bob owns and controls his AI's foundational randomness");

    Ok(())
}

/// Demo 4: Compare sovereignty levels across entropy tiers
async fn demo_sovereignty_comparison() -> Result<(), BearDogError> {
    info!("\n👑 Demo 4: Sovereignty Comparison Across Entropy Tiers");
    info!("====================================================");

    // Create different weight initialization configurations
    let machine_init = WeightInitialization::RandomNormal { 
        mean: 0.0, 
        stddev: 0.1 
    };

    let human_supervised_init = WeightInitialization::HumanEntropyInitialization {
        required_entropy_tier: 2,
        human_identity_id: "supervisor_carol".to_string(),
        distribution: EntropyDistribution::Xavier,
        fallback_to_machine: true,
    };

    let human_owned_init = WeightInitialization::HumanEntropyInitialization {
        required_entropy_tier: 3,
        human_identity_id: "owner_david".to_string(),
        distribution: EntropyDistribution::He,
        fallback_to_machine: false,
    };

    // Analyze sovereignty levels
    info!("\n📊 Sovereignty Analysis:");
    
    info!("   🔧 Traditional Machine AI:");
    info!("      - Entropy Source: Machine RNG");
    info!("      - Human Control: None");
    info!("      - Sovereignty Level: 0% (Fully machine-controlled)");
    info!("      - Security: Reproducible, potentially predictable");
    
    info!("   ⚡ Human-Supervised AI:");
    let uses_human_2 = NeuralNetworkEntropyIntegration::uses_human_entropy(&human_supervised_init);
    info!("      - Entropy Source: Machine + Human Validation");
    info!("      - Human Control: Validation & Approval");
    info!("      - Sovereignty Level: 60% (Human-validated)");
    info!("      - Uses Human Entropy: {}", uses_human_2);
    info!("      - Security: Human-validated, audit trail");
    
    info!("   🔥 Human-Owned AI:");
    let uses_human_3 = NeuralNetworkEntropyIntegration::uses_human_entropy(&human_owned_init);
    info!("      - Entropy Source: Human Lived Experience");
    info!("      - Human Control: Complete ownership");
    info!("      - Sovereignty Level: 95% (Human-owned)");
    info!("      - Uses Human Entropy: {}", uses_human_3);
    info!("      - Security: Cryptographically tied to human identity");

    info!("\n🎯 Key Insights:");
    info!("   • Humans can now OWN their AI's randomness, not just use it");
    info!("   • Different entropy tiers for different use cases");
    info!("   • Maintains security while preserving human sovereignty");
    info!("   • Cryptographic separation between human and machine entropy");

    Ok(())
}

/// Demonstrate entropy caching and performance considerations
async fn demo_entropy_performance() -> Result<(), BearDogError> {
    info!("\n⚡ Demo 5: Entropy Performance & Caching");
    info!("=======================================");

    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig {
        min_entropy_tier: 2,
        cache_entropy: true,
        cache_max_age_seconds: 60, // Short cache for demo
        allow_machine_fallback: true,
        audit_entropy_usage: true,
    };

    let mut sovereign_rng = SovereignRng::new(entropy_manager, config);

    // Initialize multiple layers using cached entropy
    let layers = vec![
        (128, 64),   // Layer 1
        (64, 32),    // Layer 2  
        (32, 16),    // Layer 3
        (16, 1),     // Output layer
    ];

    let human_identity = "performance_test_user".to_string();

    info!("   Initializing 4-layer neural network with cached human entropy:");

    for (i, &layer_shape) in layers.iter().enumerate() {
        let start_time = std::time::Instant::now();
        
        let initializer = HumanEntropyWeightInitializer {
            entropy_tier: 2,
            human_identity_id: human_identity.clone(),
            distribution: EntropyDistribution::Xavier,
            layer_shape,
        };

        let _weights = sovereign_rng.initialize_weights(&initializer)?;
        let duration = start_time.elapsed();

        info!("      Layer {}: {}x{} weights initialized in {:?}", 
              i + 1, layer_shape.0, layer_shape.1, duration);
    }

    let stats = sovereign_rng.get_entropy_stats();
    info!("   📊 Final stats: {} cached entropy seeds", stats.cached_seeds);

    // Cleanup demonstration
    sovereign_rng.cleanup_cache();
    let stats_after = sovereign_rng.get_entropy_stats();
    info!("   🧹 After cleanup: {} cached entropy seeds", stats_after.cached_seeds);

    Ok(())
}

/// Demonstrate different distribution types with human entropy
async fn demo_entropy_distributions() -> Result<(), BearDogError> {
    info!("\n📊 Demo 6: Entropy Distribution Types");
    info!("====================================");

    let entropy_manager = EntropyHierarchyManager::default();
    let config = SovereignRngConfig::default();
    let mut sovereign_rng = SovereignRng::new(entropy_manager, config);

    let distributions = vec![
        ("Normal Distribution", EntropyDistribution::Normal { mean: 0.0, stddev: 0.1 }),
        ("Uniform Distribution", EntropyDistribution::Uniform { min: -0.1, max: 0.1 }),
        ("Xavier/Glorot", EntropyDistribution::Xavier),
        ("He Initialization", EntropyDistribution::He),
    ];

    for (name, distribution) in distributions {
        let initializer = HumanEntropyWeightInitializer {
            entropy_tier: 2,
            human_identity_id: "distribution_demo_user".to_string(),
            distribution,
            layer_shape: (10, 10), // Small matrix for demo
        };

        let weights = sovereign_rng.initialize_weights(&initializer)?;
        
        // Calculate basic statistics
        let flat_weights: Vec<f64> = weights.iter().flatten().copied().collect();
        let mean = flat_weights.iter().sum::<f64>() / flat_weights.len() as f64;
        let min = flat_weights.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max = flat_weights.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        info!("   {} - Mean: {:.4}, Min: {:.4}, Max: {:.4}", 
              name, mean, min, max);
    }

    Ok(())
} 