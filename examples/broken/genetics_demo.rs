use beardog_auth::auth::{BearDogGenetics, SpawnPurpose};
use beardog_errors::BearDogError;
use beardog_genetics::genetics::{GeneticSpawningEngine, SpawnRequest, SpawnResult};
use beardog_types::config::BearDogConfig;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    println!("[DNA] BearDog Genetics Engine Demo");
    println!("===============================");

    let config = BearDogConfig::default();
    let engine = GeneticSpawningEngine::new(config.genetics.clone());
    println!("[OK] Genetics Engine initialized");

    demo_basic_spawning(&engine)?;

    demo_performance_spawning(&engine)?;

    demo_security_spawning(&engine)?;

    println!("[TARGET] Genetics demo completed successfully!");
    Ok(())
}

async fn demo_basic_spawning(engine: &GeneticSpawningEngine) -> Result<(), BearDogError> {
    println!("[DNA] Basic Genetic Spawning:");

    let request = SpawnRequest {
        request_id: "demo-basic-001".to_string(SpawnPurpose::GeneralEvolution,
        requester_id: "demo_user".to_string(),
        parent_genetics_ids: vec!["genesis".to_string({}", result.request_id);
    println!("   Success: {}", result.success);
    println!("   Fitness Score: {:.2}", result.genetics.fitness_score);
    println!("   Generation: {}", result.genetics.generation);

    Ok(())
}

async fn demo_performance_spawning(engine: &GeneticSpawningEngine) -> Result<(), BearDogError> {
    println!("[LIGHTNING] Performance Optimization Spawning:");

    let request = SpawnRequest {
        request_id: "demo-perf-001".to_string(SpawnPurpose::PerformanceOptimization,
        requester_id: "demo_user".to_string(),
        parent_genetics_ids: vec!["genesis".to_string()],
    };

    let result = engine.spawn_genetics(request)?;

    println!("   Optimization Target: Performance");
    println!(
        "   Enhanced Capabilities: {}",
        result.genetics.capabilities.len({:.1}%",
        (result.genetics.fitness_score - 0.5) * 200.0
    );

    Ok(())
}

async fn demo_security_spawning(engine: &GeneticSpawningEngine) -> Result<(), BearDogError> {
    println!("[LOCK] Security Enhancement Spawning:");

    let request = SpawnRequest {
        request_id: "demo-sec-001".to_string(SpawnPurpose::SecurityOptimization,
        requester_id: "demo_user".to_string(),
        parent_genetics_ids: vec!["genesis".to_string()],
    };

    let result = engine.spawn_genetics(request)?;

    println!("   Security Enhancement: Active");
    println!(
        "   Crypto Chromosomes: {}",
        result.genetics.crypto_chromosomes.len()
    );
    println!("   Security Level: HIGH");

    Ok(())
}
