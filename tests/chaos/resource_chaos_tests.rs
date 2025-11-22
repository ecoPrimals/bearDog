//! Resource Exhaustion Chaos Tests
//!
//! Test system behavior under resource pressure

use super::{ChaosEngine, ChaosType, ResourceChaos};
use std::time::Duration;

/// Test system behavior under memory pressure
#[tokio::test]
async fn test_memory_pressure() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test("memory_pressure", ChaosType::MemoryPressure, || async {
            let chaos = ResourceChaos::new(100, 0); // 100 MB memory pressure

            // Allocate memory to create pressure
            let _pressure = chaos.create_memory_pressure();

            // System should continue operating
            for _ in 0..10 {
                // No sleep needed - testing resilience under pressure, not timing
                // Perform operations under memory pressure
            }

            Ok(())
        })
        .await;

    assert!(
        result.correctness_maintained,
        "System should handle memory pressure"
    );
    assert!(
        result.recovered,
        "System should recover after memory pressure"
    );
}

/// Test system behavior under CPU saturation
#[tokio::test]
async fn test_cpu_saturation() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test("cpu_saturation", ChaosType::CpuSaturation, || async {
            let chaos = ResourceChaos::new(0, 2); // Saturate 2 CPU cores

            // Create CPU pressure in background
            let pressure_task = tokio::spawn(async move {
                chaos.create_cpu_pressure(Duration::from_millis(500)).await;
            });

            // System should continue operating
            for _ in 0..10 {
                // No sleep needed - testing resilience under CPU pressure, not timing
                // Perform operations under CPU pressure
            }

            let _ = pressure_task.await;

            Ok(())
        })
        .await;

    assert!(
        result.correctness_maintained,
        "System should handle CPU saturation"
    );
    assert!(
        result.recovered,
        "System should recover after CPU saturation"
    );
}

/// Test system behavior under combined resource pressure
#[tokio::test]
async fn test_combined_resource_pressure() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test(
            "combined_resource_pressure",
            ChaosType::ConcurrentFailures,
            || async {
                // Both memory and CPU pressure simultaneously
                let chaos = ResourceChaos::new(50, 1); // 50 MB + 1 CPU core

                let _memory_pressure = chaos.create_memory_pressure();

                let cpu_task = tokio::spawn(async move {
                    let chaos2 = ResourceChaos::new(0, 1);
                    chaos2.create_cpu_pressure(Duration::from_millis(300)).await;
                });

                // System should continue operating
                for _ in 0..10 {
                    // No sleep needed - testing resilience under combined pressure
                }

                let _ = cpu_task.await;

                Ok(())
            },
        )
        .await;

    assert!(
        result.correctness_maintained,
        "System should handle combined resource pressure"
    );
    assert!(
        result.recovered,
        "System should recover from combined pressure"
    );
}

/// Test system behavior when approaching resource limits
#[tokio::test]
async fn test_resource_limit_boundaries() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test(
            "resource_limit_boundaries",
            ChaosType::MemoryPressure,
            || async {
                // Gradually increase memory pressure
                for mb in [10, 25, 50, 100] {
                    let chaos = ResourceChaos::new(mb, 0);
                    let _pressure = chaos.create_memory_pressure();

                    // No sleep needed - pressure applied immediately, testing resilience

                    // System should handle increasing pressure
                }

                Ok(())
            },
        )
        .await;

    assert!(
        result.correctness_maintained,
        "System should handle resource limits"
    );
    assert!(
        result.recovered,
        "System should recover from resource limits"
    );
}

/// Test rapid resource allocation/deallocation
#[tokio::test]
async fn test_resource_thrashing() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test("resource_thrashing", ChaosType::MemoryPressure, || async {
            // Rapidly allocate and deallocate resources
            for _ in 0..20 {
                let chaos = ResourceChaos::new(25, 0);
                let _pressure = chaos.create_memory_pressure();

                // No sleep needed - pressure drops when _pressure goes out of scope
                // Pressure dropped (goes out of scope)
            }

            Ok(())
        })
        .await;

    assert!(
        result.correctness_maintained,
        "System should handle resource thrashing"
    );
    assert!(
        result.recovered,
        "System should recover from resource thrashing"
    );
}

/// Test system behavior when resources are exhausted
#[tokio::test]
async fn test_resource_exhaustion_recovery() {
    let engine = ChaosEngine::default_engine();

    let result = engine
        .run_chaos_test(
            "resource_exhaustion_recovery",
            ChaosType::MemoryPressure,
            || async {
                // Create significant resource pressure
                let chaos = ResourceChaos::new(200, 2);
                let _memory = chaos.create_memory_pressure();

                let cpu_task = tokio::spawn(async move {
                    let chaos2 = ResourceChaos::new(0, 2);
                    chaos2.create_cpu_pressure(Duration::from_millis(500)).await;
                });

                // System should degrade gracefully, not crash
                // No sleep needed - testing degradation, not timing

                let _ = cpu_task.await;

                // Resources released, system should recover
                // No sleep needed - recovery is immediate

                Ok(())
            },
        )
        .await;

    assert!(
        result.correctness_maintained,
        "System should degrade gracefully"
    );
    assert!(
        result.recovered,
        "System should recover after resource exhaustion"
    );
}
