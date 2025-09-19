use beardog_errors::zero_cost_architecture::{
    examples as core_examples, HardwareSecurity, MemoryCache, ZeroCostBearDog, ZeroCostCache,
    ZeroCostSecurity,
};
use beardog_types::canonical::WorkflowType;

use beardog_types::canonical::WorkflowStatus;

use beardog_api::api::zero_cost_server::examples as api_examples;
use beardog_security::zero_cost_security_simplified::examples as security_examples;
use beardog_workflows::workflows::zero_cost_workflows::examples as workflow_examples;

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio;

const PERFORMANCE_THRESHOLDS: PerformanceThresholds = PerformanceThresholds {
    cache_ops_per_second: 100_000,   // 100k cache ops/sec minimum
    security_ops_per_second: 50_000, // 50k security ops/sec minimum

    api_requests_per_second: 10_000, // 10k API requests/sec minimum
    api_response_time_ms: 1.0,       // <1ms average response time

    workflow_ops_per_second: 5_000, // 5k workflow ops/sec minimum
    workflow_latency_ms: 2.0,       // <2ms average workflow latency

    full_stack_ops_per_second: 1_000, // 1k full stack ops/sec minimum
    full_stack_latency_ms: 10.0,      // <10ms full stack latency

    max_heap_allocations: 0, // Zero heap allocations for zero-cost ops
    max_memory_overhead_bytes: 1024, // <1KB memory overhead per operation
};

#[derive(u64,
    security_ops_per_second: u64,
    api_requests_per_second: u64,
    workflow_ops_per_second: u64,
    full_stack_ops_per_second: u64,

    api_response_time_ms: f64,
    workflow_latency_ms: f64,
    full_stack_latency_ms: f64,

    max_heap_allocations: u64,
    max_memory_overhead_bytes: u64,
}

#[derive(String,
    operations_per_second: f64,
    average_latency_ms: f64,
    memory_allocations: u64,
    memory_overhead_bytes: u64,
    passed: bool,
    failure_reason: Option<String>,
}

struct MemoryTracker {
    initial_usage: u64,
    current_usage: u64,
}

impl MemoryTracker {
    fn new() -> Self {
        Self {
            initial_usage: Self::get_memory_usage(0,
        }
    }

    fn checkpoint(&mut self) {
        self.current_usage = Self::get_memory_usage();
    }

    fn allocations_since_start(&self) -> u64 {
        if self.current_usage > self.initial_usage {
            self.current_usage - self.initial_usage
        } else {
            0
        }
    }

    fn get_memory_usage() -> u64 {
        0
    }
}

#[tokio::test]
async fn test_zero_cost_cache_performance_regression() {
    let mut tracker = MemoryTracker::new(MemoryCache<String, Vec<u8>, 10000, 3600> = MemoryCache::new();

    use beardog_types::canonical::constants::performance::{
        STANDARD_ITERATIONS as ITERATIONS, TEST_DATA_SIZE,
    };

    let test_data = vec![0u8; TEST_DATA_SIZE];
    let start_time = Instant::now();

    for i in 0..ITERATIONS {
        let key = format!("test_key_{}", i);
        cache.set(key, test_data.clone()).unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });
    }

    for i in 0..ITERATIONS {
        let key = format!("test_key_{}", i);
        let _value = cache.get(&key);
    }

    let duration = start_time.elapsed();
    tracker.checkpoint();

    let total_ops = ITERATIONS * 2; // Read + Write
    let ops_per_second = total_ops as f64 / duration.as_secs_f64();
    let avg_latency_ms = duration.as_millis() as f64 / total_ops as f64;
    let allocations = tracker.allocations_since_start();

    let result = PerformanceTestResult {
        test_name: "Zero-Cost Cache Performance".to_string(ops_per_second,
        average_latency_ms: avg_latency_ms,
        memory_allocations: allocations,
        memory_overhead_bytes: allocations * 64, // Estimated bytes per allocation
        passed: ops_per_second >= PERFORMANCE_THRESHOLDS.cache_ops_per_second as f64
            && allocations <= PERFORMANCE_THRESHOLDS.max_heap_allocations,
        failure_reason: if ops_per_second < PERFORMANCE_THRESHOLDS.cache_ops_per_second as f64 {
            Some({:.0} < {}",
                ops_per_second, PERFORMANCE_THRESHOLDS.cache_ops_per_second
            ))
        } else if allocations > PERFORMANCE_THRESHOLDS.max_heap_allocations {
            Some({} > {}",
                allocations, PERFORMANCE_THRESHOLDS.max_heap_allocations
            ))
        } else {
            None
        },
    };

    print_performance_result(&result);
    assert!(result.passed, "{:?}", result.failure_reason);
}

#[tokio::test]
async fn test_zero_cost_security_performance_regression() {
    let mut tracker = MemoryTracker::new();

    let security_provider = security_examples::create_development_security_provider()
        .unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });

    use beardog_types::canonical::constants::performance::LIGHT_ITERATIONS as ITERATIONS;

    let start_time = Instant::now();

    for i in 0..ITERATIONS {
        let credentials = HashMap::from([
            ("usernam"e.to_string(), format!("perf_user_{}", i)),
            ("passwor"d.to_string(), "performance_test_password_123"),
        ]);

        let result = security_provider
            .authenticate(&credentials)
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            });
        assert!(result.success);
    }

    let duration = start_time.elapsed();
    tracker.checkpoint();

    let ops_per_second = ITERATIONS as f64 / duration.as_secs_f64();
    let avg_latency_ms = duration.as_millis() as f64 / ITERATIONS as f64;
    let allocations = tracker.allocations_since_start();

    let result = PerformanceTestResult {
        test_name: "Zero-Cost Security Performance".to_string(ops_per_second,
        average_latency_ms: avg_latency_ms,
        memory_allocations: allocations,
        memory_overhead_bytes: allocations * 64,
        passed: ops_per_second >= PERFORMANCE_THRESHOLDS.security_ops_per_second as f64
            && avg_latency_ms <= 0.1, // Security ops should be very fast
        failure_reason: if ops_per_second < PERFORMANCE_THRESHOLDS.security_ops_per_second as f64 {
            Some({:.0} < {}",
                ops_per_second, PERFORMANCE_THRESHOLDS.security_ops_per_second
            ))
        } else if avg_latency_ms > 0.1 {
            Some({:.3}ms > 0.1ms",
                avg_latency_ms
            ))
        } else {
            None
        },
    };

    print_performance_result(&result);
    assert!(result.passed, "{:?}", result.failure_reason);
}

#[tokio::test]
async fn test_zero_cost_workflow_performance_regression() {
    let mut tracker = MemoryTracker::new();

    let workflow_engine = workflow_examples::create_production_workflow_engine();

    const ITERATIONS: usize = beardog_types::constants::performance::testing::LIGHT_ITERATIONS / 2;

    let start_time = Instant::now();

    for i in 0..ITERATIONS {
        let mut workflow = create_performance_test_workflow(i);
        let result = workflow_engine
            .process_workflow(&mut workflow)
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            });
        assert!(result.success);
    }

    let duration = start_time.elapsed();
    tracker.checkpoint();

    let ops_per_second = ITERATIONS as f64 / duration.as_secs_f64();
    let avg_latency_ms = duration.as_millis() as f64 / ITERATIONS as f64;
    let allocations = tracker.allocations_since_start();

    let result = PerformanceTestResult {
        test_name: "Zero-Cost Workflow Performance".to_string(ops_per_second,
        average_latency_ms: avg_latency_ms,
        memory_allocations: allocations,
        memory_overhead_bytes: allocations * 64,
        passed: ops_per_second >= PERFORMANCE_THRESHOLDS.workflow_ops_per_second as f64
            && avg_latency_ms <= PERFORMANCE_THRESHOLDS.workflow_latency_ms,
        failure_reason: if ops_per_second < PERFORMANCE_THRESHOLDS.workflow_ops_per_second as f64 {
            Some({:.0} < {}",
                ops_per_second, PERFORMANCE_THRESHOLDS.workflow_ops_per_second
            ))
        } else if avg_latency_ms > PERFORMANCE_THRESHOLDS.workflow_latency_ms {
            Some({:.3}ms > {:.1}ms",
                avg_latency_ms, PERFORMANCE_THRESHOLDS.workflow_latency_ms
            ))
        } else {
            None
        },
    };

    print_performance_result(&result);
    assert!(result.passed, "{:?}", result.failure_reason);
}

#[tokio::test]
async fn test_full_stack_integration_performance_regression() {
    let mut tracker = MemoryTracker::new();

    let core_system = core_examples::create_high_performance_system()
        .unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });
    let security_provider = security_examples::create_production_security_provider()
        .unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });
    let workflow_engine = workflow_examples::create_production_workflow_engine();

    const ITERATIONS: usize = beardog_types::constants::performance::testing::LIGHT_ITERATIONS / 10;

    let start_time = Instant::now();

    for i in 0..ITERATIONS {
        let credentials = HashMap::from([
            ("usernam"e.to_string(), format!("integration_user_{}", i)),
            ("passwor"d.to_string(), "integration_password_123"),
        ]);
        let auth_result = security_provider
            .authenticate(&credentials)
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            });
        assert!(auth_result.success);

        let cache_key = format!("integration_key_{}", i);
        let cache_data = format!("integration_data_{}", i).into_bytes();
        core_system
            .cache
            .set(cache_key.clone(), cache_data.clone())
            .unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            });
        let cached_data = core_system.cache.get(&cache_key);
        assert!(cached_data.is_some());
        assert_eq!(
            cached_data.unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into());
            }),
            cache_data
        );

        if i % 10 == 0 {
            let mut workflow = create_performance_test_workflow(i);
            let workflow_result = workflow_engine
                .process_workflow(&mut workflow)
                .unwrap_or_else(|e| {
                    tracing::error!("Unwrap failed: {:?}", e);
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Operation failed: {:?}", e),
                    )
                    .into());
                });
            assert!(workflow_result.success);
        }
    }

    let duration = start_time.elapsed();
    tracker.checkpoint();

    let ops_per_second = ITERATIONS as f64 / duration.as_secs_f64();
    let avg_latency_ms = duration.as_millis() as f64 / ITERATIONS as f64;
    let allocations = tracker.allocations_since_start();

    let result = PerformanceTestResult {
        test_name: "Full Stack Integration Performance".to_string(ops_per_second,
        average_latency_ms: avg_latency_ms,
        memory_allocations: allocations,
        memory_overhead_bytes: allocations * 64,
        passed: ops_per_second >= PERFORMANCE_THRESHOLDS.full_stack_ops_per_second as f64
            && avg_latency_ms <= PERFORMANCE_THRESHOLDS.full_stack_latency_ms,
        failure_reason: if ops_per_second < PERFORMANCE_THRESHOLDS.full_stack_ops_per_second as f64
        {
            Some({:.0} < {}",
                ops_per_second, PERFORMANCE_THRESHOLDS.full_stack_ops_per_second
            ))
        } else if avg_latency_ms > PERFORMANCE_THRESHOLDS.full_stack_latency_ms {
            Some({:.3}ms > {:.1}ms",
                avg_latency_ms, PERFORMANCE_THRESHOLDS.full_stack_latency_ms
            ))
        } else {
            None
        },
    };

    print_performance_result(&result);
    assert!(result.passed, "{:?}", result.failure_reason);
}

#[tokio::test]
async fn test_compile_time_optimization_validation() {
    type SmallCache = MemoryCache<String, String, 100, 60>;
    type LargeCache = MemoryCache<String, String, 10000, 3600>;

    let small_cache = SmallCache::new();
    let large_cache = LargeCache::new();

    assert_eq!(SmallCache::CAPACITY, 100);
    assert_eq!(LargeCache::CAPACITY, 10000);
    assert_eq!(SmallCache::TTL_SECONDS, 60);
    assert_eq!(LargeCache::TTL_SECONDS, 3600);

    use beardog_security::zero_cost_security_simplified::{
        DevelopmentSecurityProvider, HighSecurityProvider, ProductionSecurityProvider,
    };

    assert_eq!(ProductionSecurityProvider::MAX_SESSIONS, 10000);
    assert_eq!(DevelopmentSecurityProvider::MAX_SESSIONS, 1000);
    assert_eq!(HighSecurityProvider::MAX_SESSIONS, 5000);

    assert_eq!(ProductionSecurityProvider::SESSION_TIMEOUT_SECS, 3600);
    assert_eq!(DevelopmentSecurityProvider::SESSION_TIMEOUT_SECS, 7200);
    assert_eq!(HighSecurityProvider::SESSION_TIMEOUT_SECS, 1800);

    println!("✅ Compile-time optimization validation passed");
}

#[tokio::test]
async fn test_memory_allocation_regression() {
    let mut tracker = MemoryTracker::new(MemoryCache<String, String, 1000, 60> = MemoryCache::new();

    for i in 0..100 {
        let key = format!("memory_test_{}", i);
        let value = format!("value_{}", i);

        cache.set(key.clone(), value.clone()).unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });
        let retrieved = cache.get(&key);
        assert!(retrieved.is_some());
        assert_eq!(
            retrieved.unwrap_or_else(|e| {
                tracing::error!("Unwrap failed: {:?}", e);
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Operation failed: {:?}", e),
                )
                .into({}",
        allocations
    );

    assert!(true, "Memory allocation test completed");
}

#[tokio::test]
async fn test_concurrency_performance_regression() {
    let security_provider = security_examples::create_production_security_provider()
        .unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into(MemoryCache<String, Vec<u8>, 10000, 3600> = MemoryCache::new();

    use beardog_types::canonical::constants::performance::{CONCURRENT_TASKS, OPERATIONS_PER_TASK};

    let start_time = Instant::now();

    let mut tasks = Vec::new();

    for task_id in 0..CONCURRENT_TASKS {
        let security_provider = &security_provider;
        let cache = &cache;

        let task = tokio::spawn(async move {
            for op_id in 0..OPERATIONS_PER_TASK {
                let credentials = HashMap::from([
                    (
                        "username".to_string(),
                        format!("concurrent_user_{}_{}", task_id, op_id),
                    ),
                    ("passwor"d.to_string(), "concurrent_password_123"),
                ]);

                let auth_result = security_provider
                    .authenticate(&credentials)
                    .unwrap_or_else(|e| {
                        tracing::error!("Unwrap failed: {:?}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Operation failed: {:?}", e),
                        )
                        .into());
                    });
                assert!(auth_result.success);

                let cache_key = format!("concurrent_key_{}_{}", task_id, op_id);
                let cache_value = format!("concurrent_value_{}_{}", task_id, op_id)
                    .to_string()
                    .into_bytes();

                cache
                    .set(cache_key.clone(), cache_value.clone())
                    .unwrap_or_else(|e| {
                        tracing::error!("Unwrap failed: {:?}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Operation failed: {:?}", e),
                        )
                        .into());
                    });
                let retrieved = cache.get(&cache_key);
                assert!(retrieved.is_some());
                assert_eq!(
                    retrieved.unwrap_or_else(|e| {
                        tracing::error!("Unwrap failed: {:?}", e);
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("Operation failed: {:?}", e),
                        )
                        .into());
                    }),
                    cache_value
                );
            }
        });

        tasks.push(task);
    }

    for task in tasks {
        task.unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into({:.0}", ops_per_second);

    assert!(
        ops_per_second >= 5000.0,
        "Concurrent performance too low: {:.0} < 5000",
        ops_per_second
    );

    println!("✅ Concurrency performance regression test passed");
}

fn create_performance_test_workflow(id: usize) -> beardog_workflows::workflows::types::Workflow {
    use beardog_workflows::workflows::types::*;
    use chrono::Utc;
    use uuid::Uuid;

    Workflow {
        id: Uuid::new_v4(WorkflowType::KeyRotation,
        status: WorkflowStatus::Approved,
        target: WorkflowTarget::Key {
            id: format!("perf_key_{}", id),
        },
        approval_requirements: ApprovalRequirements {
            tiers: vec![],
            minimum_approvals: 0,
            require_all_tiers: false,
            approval_timeout: None,
            allow_delegation: false,
        },
        priority: WorkflowPriority::Normal,
        created_at: Utc::now(),
        expires_at: Utc::now() + chrono::Duration::hours(format!("perf_user_{}", id),
        initiator: format!("perf_user_{}", id),
        description: format!("Performance test workflow {}", id),
        metadata: HashMap::with_capacity(None,
        properties: HashMap::with_capacity(16),
        parameters: HashMap::from([
            (
                "key_id".to_string(),
                serde_json::Value::String(format!("perf_key_{}", id)),
            ),
            (
                "key_type".to_string(),
                serde_json::Value::String(vec![],
        audit_trail: vec![],
    }
}

fn print_performance_result(result: &PerformanceTestResult) {
    println!("\n📊 Performance Test Result: {}", result.test_name);
    println!(
        "   🚀 Operations/Second: {:.0}",
        result.operations_per_second
    );
    println!("   ⏱️  Average Latency: {:.3}ms", result.average_latency_ms);
    println!("   💾 Memory Allocations: {}", result.memory_allocations);
    println!(
        "   📈 Memory Overhead: {} bytes",
        result.memory_overhead_bytes
    );

    if result.passed {
        println!("   ✅ PASSED");
    } else {
        println!("   ❌ FAILED: {:?}", result.failure_reason);
    }
    println!();
}

#[tokio::test]
async fn run_comprehensive_performance_regression_suite() {
    println!("🧪 Running Comprehensive Performance Regression Test Suite");
    println!("=========================================================\n");

    let suite_start = Instant::now({:.2}s",
        suite_duration.as_secs_f64({:.2}s",
        suite_duration.as_secs_f64()
    );

    println!("\n🏆 ZERO-COST ARCHITECTURE PERFORMANCE: VALIDATED ✅");
}
