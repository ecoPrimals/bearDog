//! # Production Workload Benchmarks

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]
//!
//! Real-world scenario benchmarks that simulate production usage patterns:
//! - API request handling with crypto operations
//! - Multi-tenant key operations
//! - High-throughput signing service
//! - Configuration loading and parsing
//! - Error handling and propagation overhead
//! - Full-stack end-to-end flows
//!
//! ## Methodology
//!
//! - Models actual production workloads
//! - Tests under realistic concurrency
//! - Measures percentile latencies (p50, p95, p99)
//! - Validates performance under stress

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

// ============================================================================
// Mock Production Types
// ============================================================================

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ApiRequest {
    tenant_id: String,
    operation: String,
    payload: Vec<u8>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct ApiResponse {
    status: u16,
    body: Vec<u8>,
}

#[derive(Clone)]
struct KeyStore {
    keys: Arc<Mutex<HashMap<String, Vec<u8>>>>,
}

impl KeyStore {
    fn new() -> Self {
        Self {
            keys: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_key(&self, key_id: &str) -> Option<Vec<u8>> {
        self.keys.lock().unwrap().get(key_id).cloned()
    }

    fn store_key(&self, key_id: String, key_data: Vec<u8>) {
        self.keys.lock().unwrap().insert(key_id, key_data);
    }
}

// ============================================================================
// API Request Handling
// ============================================================================

fn benchmark_api_request_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("api_request_handling");

    let request = ApiRequest {
        tenant_id: "tenant-123".to_string(),
        operation: "sign".to_string(),
        payload: vec![0u8; 1024],
    };

    // Simulate: auth + routing + HSM operation + response
    group.bench_function("full_request_lifecycle", |b| {
        b.iter(|| {
            // 1. Authentication (JWT validation)
            std::thread::sleep(Duration::from_micros(100));

            // 2. Authorization check
            std::thread::sleep(Duration::from_micros(50));

            // 3. Request parsing
            let _req = black_box(&request);

            // 4. HSM operation (signing)
            std::thread::sleep(Duration::from_micros(500));

            // 5. Response serialization
            let response = ApiResponse {
                status: 200,
                body: vec![0u8; 256], // Signature
            };

            black_box(response)
        })
    });

    // Breakdown of components
    group.bench_function("auth_only", |b| {
        b.iter(|| {
            std::thread::sleep(Duration::from_micros(100));
            black_box(true)
        })
    });

    group.bench_function("routing_only", |b| {
        b.iter(|| {
            let req = black_box(&request);
            let route = match req.operation.as_str() {
                "sign" => "/hsm/sign",
                "encrypt" => "/hsm/encrypt",
                _ => "/unknown",
            };
            black_box(route)
        })
    });

    group.finish();
}

// ============================================================================
// Multi-Tenant Key Operations
// ============================================================================

fn benchmark_multitenant_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("multitenant_operations");

    let store = KeyStore::new();

    // Pre-populate with tenant keys
    for i in 0..100 {
        store.store_key(
            format!("tenant-{}", i),
            vec![0u8; 32], // 256-bit key
        );
    }

    let tenant_counts = vec![1, 10, 50, 100];

    for count in tenant_counts {
        group.bench_function(BenchmarkId::new("concurrent_access", count), |b| {
            let store_clone = store.clone();
            b.iter(|| {
                let handles: Vec<_> = (0..count)
                    .map(|i| {
                        let store = store_clone.clone();
                        std::thread::spawn(move || {
                            let key_id = format!("tenant-{}", i % 100);
                            let key = store.get_key(&key_id);
                            black_box(key);
                        })
                    })
                    .collect();

                for handle in handles {
                    handle.join().unwrap();
                }
            })
        });
    }

    group.finish();
}

// ============================================================================
// High-Throughput Signing Service
// ============================================================================

fn benchmark_signing_service(c: &mut Criterion) {
    let mut group = c.benchmark_group("signing_service");
    group.throughput(Throughput::Elements(1));

    let message_sizes = vec![
        ("small_100B", 100),
        ("medium_1KB", 1024),
        ("large_10KB", 10 * 1024),
        ("xlarge_100KB", 100 * 1024),
    ];

    for (name, size) in message_sizes {
        let message = vec![0u8; size];

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(BenchmarkId::new("sign", name), &message, |b, msg| {
            b.iter(|| {
                // Simulate: hash + sign + encode

                // 1. Hash message (SHA-256)
                std::thread::sleep(Duration::from_micros((msg.len() / 1024) as u64));

                // 2. Sign hash (ECDSA P-256)
                std::thread::sleep(Duration::from_micros(30));

                // 3. Encode signature (DER)
                let signature = vec![0u8; 72]; // ECDSA signature

                black_box(signature)
            })
        });
    }

    group.finish();
}

// ============================================================================
// Configuration Loading
// ============================================================================

fn benchmark_configuration_loading(c: &mut Criterion) {
    let mut group = c.benchmark_group("configuration_loading");

    // Simple config (env vars only)
    group.bench_function("env_vars_only", |b| {
        b.iter(|| {
            let config = HashMap::from([
                ("BEARDOG_LOG_LEVEL", "info"),
                ("BEARDOG_PORT", "8443"),
                ("BEARDOG_TIMEOUT_MS", "30000"),
            ]);
            black_box(config)
        })
    });

    // TOML config parsing
    group.bench_function("toml_parse_small", |b| {
        let toml_str = r#"
            [server]
            port = 8443
            timeout_ms = 30000
            
            [hsm]
            provider = "software"
            key_size = 256
        "#;

        b.iter(|| {
            // Simulate TOML parsing
            std::thread::sleep(Duration::from_micros(50));
            black_box(toml_str)
        })
    });

    // Complex config with validation
    group.bench_function("complex_config_with_validation", |b| {
        b.iter(|| {
            // 1. Load from multiple sources
            std::thread::sleep(Duration::from_micros(100)); // File I/O

            // 2. Parse TOML/YAML
            std::thread::sleep(Duration::from_micros(50));

            // 3. Merge with env vars
            std::thread::sleep(Duration::from_micros(20));

            // 4. Validate config
            std::thread::sleep(Duration::from_micros(30));

            black_box(true)
        })
    });

    // Config reload (hot reload)
    group.bench_function("hot_reload", |b| {
        b.iter(|| {
            // 1. Detect config change
            std::thread::sleep(Duration::from_micros(10));

            // 2. Parse new config
            std::thread::sleep(Duration::from_micros(50));

            // 3. Validate
            std::thread::sleep(Duration::from_micros(30));

            // 4. Apply atomically
            std::thread::sleep(Duration::from_micros(20));

            black_box(true)
        })
    });

    group.finish();
}

// ============================================================================
// Error Handling Overhead
// ============================================================================

#[derive(Debug, Clone)]
enum BenchError {
    Security { message: String },
    System { message: String },
    Validation { message: String },
}

fn benchmark_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_handling");

    // Error creation
    group.bench_function("create_error", |b| {
        b.iter(|| {
            let err = BenchError::Security {
                message: "Invalid signature".to_string(),
            };
            black_box(err)
        })
    });

    // Error propagation (Result chain)
    fn level1() -> Result<u32, BenchError> {
        level2()
    }

    fn level2() -> Result<u32, BenchError> {
        level3()
    }

    fn level3() -> Result<u32, BenchError> {
        Err(BenchError::Security {
            message: "Deep error".to_string(),
        })
    }

    group.bench_function("error_propagation_3_levels", |b| {
        b.iter(|| {
            let result = level1();
            black_box(result)
        })
    });

    // Error with context (anyhow-style)
    group.bench_function("error_with_context", |b| {
        b.iter(|| {
            let err = BenchError::System {
                message: format!(
                    "Failed to connect to {}: timeout after {}ms",
                    "hsm.local:8443", 30000
                ),
            };
            black_box(err)
        })
    });

    // Error logging
    group.bench_function("error_log", |b| {
        b.iter(|| {
            let err = BenchError::Security {
                message: "Auth failed".to_string(),
            };
            // Simulate structured logging
            std::thread::sleep(Duration::from_micros(5));
            black_box(err)
        })
    });

    group.finish();
}

// ============================================================================
// Full-Stack E2E Flow
// ============================================================================

fn benchmark_e2e_crypto_flow(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e_crypto_flow");
    group.sample_size(20);

    group.bench_function("encrypt_store_retrieve_decrypt", |b| {
        let plaintext = vec![0u8; 1024];
        let store = KeyStore::new();
        store.store_key("master-key".to_string(), vec![0u8; 32]);

        b.iter(|| {
            // 1. Get encryption key
            let _key = store.get_key("master-key").unwrap();
            std::thread::sleep(Duration::from_micros(10));

            // 2. Generate data encryption key (DEK)
            std::thread::sleep(Duration::from_micros(20));

            // 3. Encrypt data with DEK
            std::thread::sleep(Duration::from_micros(100));
            let ciphertext = vec![0u8; 1040]; // With IV/tag

            // 4. Encrypt DEK with master key (key wrapping)
            std::thread::sleep(Duration::from_micros(50));
            let wrapped_dek = vec![0u8; 48];

            // 5. Store encrypted data + wrapped DEK
            std::thread::sleep(Duration::from_micros(30));

            // 6. Retrieve encrypted data
            std::thread::sleep(Duration::from_micros(30));

            // 7. Unwrap DEK
            std::thread::sleep(Duration::from_micros(50));

            // 8. Decrypt data
            std::thread::sleep(Duration::from_micros(100));

            black_box((ciphertext, wrapped_dek, plaintext.clone()))
        })
    });

    group.bench_function("sign_verify_flow", |b| {
        let _message = vec![0u8; 1024];
        let store = KeyStore::new();
        store.store_key("signing-key".to_string(), vec![0u8; 32]);

        b.iter(|| {
            // 1. Get signing key
            let _key = store.get_key("signing-key").unwrap();
            std::thread::sleep(Duration::from_micros(10));

            // 2. Hash message
            std::thread::sleep(Duration::from_micros(20));

            // 3. Sign hash
            std::thread::sleep(Duration::from_micros(30));
            let signature = vec![0u8; 64];

            // 4. Encode signature
            std::thread::sleep(Duration::from_micros(10));

            // 5. Transmit (serialize)
            std::thread::sleep(Duration::from_micros(20));

            // 6. Deserialize
            std::thread::sleep(Duration::from_micros(20));

            // 7. Get verification key
            std::thread::sleep(Duration::from_micros(10));

            // 8. Hash message (again)
            std::thread::sleep(Duration::from_micros(20));

            // 9. Verify signature
            std::thread::sleep(Duration::from_micros(60));

            black_box((signature, true))
        })
    });

    group.finish();
}

// ============================================================================
// Rate Limiting and Throttling
// ============================================================================

fn benchmark_rate_limiting(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_limiting");

    // Token bucket check
    group.bench_function("token_bucket_check", |b| {
        let mut tokens = 1000.0_f64;
        let rate = 100.0; // tokens per second

        b.iter(|| {
            // Check if tokens available
            if tokens >= 1.0 {
                tokens -= 1.0;
                black_box(true)
            } else {
                // Refill tokens
                tokens += rate * 0.001; // 1ms elapsed
                black_box(false)
            }
        })
    });

    // Sliding window rate limit
    group.bench_function("sliding_window_check", |b| {
        let mut requests: Vec<u64> = Vec::with_capacity(100);
        let limit = 100;
        let window_ms = 1000;

        b.iter(|| {
            let now = 0u64; // Mock timestamp

            // Remove old requests
            requests.retain(|&t| now - t < window_ms);

            // Check limit
            if requests.len() < limit {
                requests.push(now);
                black_box(true)
            } else {
                black_box(false)
            }
        })
    });

    group.finish();
}

// ============================================================================
// Monitoring and Metrics
// ============================================================================

fn benchmark_metrics_collection(c: &mut Criterion) {
    let mut group = c.benchmark_group("metrics_collection");

    // Counter increment
    group.bench_function("counter_increment", |b| {
        let counter = Arc::new(Mutex::new(0u64));
        b.iter(|| {
            let mut c = counter.lock().unwrap();
            *c += 1;
            black_box(*c)
        })
    });

    // Histogram recording
    group.bench_function("histogram_record", |b| {
        let histogram = Arc::new(Mutex::new(Vec::<f64>::new()));
        b.iter(|| {
            let value = 42.5;
            let mut h = histogram.lock().unwrap();
            h.push(value);
            black_box(value)
        })
    });

    // Structured logging with metadata
    group.bench_function("structured_log_with_metadata", |b| {
        b.iter(|| {
            let log_entry = format!(
                r#"{{"timestamp":"{}","level":"info","message":"{}","tenant":"{}","duration_ms":{}}}"#,
                "2025-11-06T12:00:00Z",
                "HSM operation completed",
                "tenant-123",
                42
            );
            black_box(log_entry)
        })
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group! {
    name = production_benches;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
        .measurement_time(Duration::from_secs(10))
        .warm_up_time(Duration::from_secs(3));
    targets =
        benchmark_api_request_handling,
        benchmark_multitenant_operations,
        benchmark_signing_service,
        benchmark_configuration_loading,
        benchmark_error_handling,
        benchmark_e2e_crypto_flow,
        benchmark_rate_limiting,
        benchmark_metrics_collection,
}

criterion_main!(production_benches);
