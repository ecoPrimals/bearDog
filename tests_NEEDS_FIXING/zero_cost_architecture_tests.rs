// Tests for zero-cost architecture patterns and compile-time optimizations
//
// Verifies that zero-cost abstractions work correctly and provide
// compile-time guarantees without runtime overhead.

use beardog_core::zero_cost_architecture::{
    ZeroCostConfig, ZeroCostSystem, OptimizationLevel, PerformanceProfile
};

#[tokio::test]
async fn test_zero_cost_config_const_generics() {
    // Test compile-time configuration with const generics
    const CACHE_SIZE: usize = 1024;
    const MAX_CONNECTIONS: usize = 100;
    
    let config = ZeroCostConfig::<CACHE_SIZE, MAX_CONNECTIONS>::new();
    
    // These should be compile-time constants
    assert_eq!(config.cache_size(), CACHE_SIZE);
    assert_eq!(config.max_connections(), MAX_CONNECTIONS);
}

#[tokio::test]
async fn test_different_configurations() {
    // Test that different const generic parameters create different types
    let small_config = ZeroCostConfig::<512, 50>::new();
    let large_config = ZeroCostConfig::<2048, 200>::new();
    
    assert_eq!(small_config.cache_size(), 512);
    assert_eq!(small_config.max_connections(), 50);
    
    assert_eq!(large_config.cache_size(), 2048);
    assert_eq!(large_config.max_connections(), 200);
}

#[tokio::test]
async fn test_zero_cost_system_creation() {
    let config = ZeroCostConfig::<1024, 100>::new();
    let system = ZeroCostSystem::new(config);
    
    assert!(system.is_optimized());
    assert_eq!(system.cache_capacity(), 1024);
}

#[tokio::test]
async fn test_optimization_levels() {
    let levels = vec![
        OptimizationLevel::Development,
        OptimizationLevel::Production,
        OptimizationLevel::Maximum,
    ];
    
    for level in levels {
        let system = ZeroCostSystem::with_optimization(level);
        assert!(system.optimization_level() == level);
    }
}

#[tokio::test]
async fn test_performance_profiles() {
    let profiles = vec![
        PerformanceProfile::Balanced,
        PerformanceProfile::Throughput,
        PerformanceProfile::Latency,
        PerformanceProfile::Memory,
    ];
    
    for profile in profiles {
        let system = ZeroCostSystem::with_profile(profile);
        assert!(system.performance_profile() == profile);
    }
}

#[tokio::test]
async fn test_compile_time_assertions() {
    // These should be evaluated at compile time
    const CONFIG: ZeroCostConfig<2000, 200> = ZeroCostConfig::new();
    
    // Compile-time assertions
    static_assertions::const_assert_eq!(CONFIG.cache_size(), 2000);
    static_assertions::const_assert_eq!(CONFIG.max_connections(), 200);
}

#[tokio::test]
async fn test_zero_runtime_overhead() {
    let config = ZeroCostConfig::<1024, 100>::new();
    let system = ZeroCostSystem::new(config);
    
    // Measure operation that should have zero runtime cost
    let start = std::time::Instant::now();
    
    for _ in 0..1000 {
        let _ = system.cache_capacity(); // Should be compile-time constant
        let _ = system.is_optimized(); // Should be compile-time constant
    }
    
    let duration = start.elapsed();
    
    // These operations should be extremely fast (sub-microsecond)
    assert!(duration.as_nanos() < 100_000); // Less than 100 microseconds for 1000 ops
}

#[tokio::test]
async fn test_type_safety() {
    // Different configurations should be different types
    let config1 = ZeroCostConfig::<1024, 100>::new();
    let config2 = ZeroCostConfig::<2048, 200>::new();
    
    let system1 = ZeroCostSystem::new(config1);
    let system2 = ZeroCostSystem::new(config2);
    
    // Type system ensures these are different types
    assert_ne!(
        std::any::TypeId::of::<ZeroCostSystem<1024, 100>>(),
        std::any::TypeId::of::<ZeroCostSystem<2048, 200>>()
    );
}

#[tokio::test]
async fn test_memory_efficiency() {
    let config = ZeroCostConfig::<1024, 100>::new();
    let system = ZeroCostSystem::new(config);
    
    // Zero-cost abstractions should not increase memory usage
    let size = std::mem::size_of_val(&system);
    let config_size = std::mem::size_of_val(&config);
    
    // System should not be significantly larger than its components
    assert!(size <= config_size + 64); // Allow small overhead for metadata
} 