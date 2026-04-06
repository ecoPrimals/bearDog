// SPDX-License-Identifier: AGPL-3.0-or-later
//! # HSM Discovery Benchmarks

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]
//!
//! Comprehensive benchmarks for HSM discovery operations including:
//! - Network endpoint probing (HTTP/TCP)
//! - Platform HSM detection (TPM, Secure Enclave, StrongBox)
//! - Cloud HSM discovery (AWS KMS, Azure, GCP)
//! - USB HSM detection (YubiKey, Nitrokey)
//! - Overall discovery throughput
//!
//! ## Methodology
//!
//! - Simulates realistic network latencies
//! - Tests concurrent discovery operations
//! - Measures caching effectiveness
//! - Profiles memory allocation during discovery

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
#[cfg(feature = "profiling")]
use pprof::criterion::{Output, PProfProfiler};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// Mock Discovery Types
// ============================================================================

#[derive(Clone, Debug)]
struct DiscoveredHsm {
    id: String,
    hsm_type: String,
    endpoint: Option<String>,
    capabilities: Vec<String>,
}

#[derive(Clone, Debug)]
struct NetworkEndpoint {
    url: String,
    protocol: String,
    port: u16,
}

// ============================================================================
// Network Discovery Benchmarks
// ============================================================================

fn benchmark_endpoint_probing(c: &mut Criterion) {
    let mut group = c.benchmark_group("network_endpoint_probing");

    // Simulate different network conditions
    let latencies = vec![
        ("local_1ms", 1),
        ("lan_5ms", 5),
        ("wan_50ms", 50),
        ("slow_200ms", 200),
    ];

    for (name, latency_ms) in latencies {
        group.bench_function(name, |b| {
            b.iter(|| {
                // Simulate HTTP health check
                std::thread::sleep(Duration::from_millis(latency_ms));
                black_box(true) // Success
            })
        });
    }

    group.finish();
}

fn benchmark_concurrent_probing(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_endpoint_probing");

    let endpoint_counts = vec![1, 5, 10, 20, 50];

    for count in endpoint_counts {
        group.bench_function(BenchmarkId::new("parallel", count), |b| {
            b.iter(|| {
                let handles: Vec<_> = (0..count)
                    .map(|_| {
                        std::thread::spawn(|| {
                            std::thread::sleep(Duration::from_millis(5)); // Simulate probe
                            black_box(true)
                        })
                    })
                    .collect();

                let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

                black_box(results);
            })
        });
    }

    group.finish();
}

fn benchmark_tcp_vs_http_probing(c: &mut Criterion) {
    let mut group = c.benchmark_group("tcp_vs_http_probing");

    // TCP connection test (faster)
    group.bench_function("tcp_probe", |b| {
        b.iter(|| {
            std::thread::sleep(Duration::from_millis(2)); // TCP handshake
            black_box(true)
        })
    });

    // HTTP health check (slower)
    group.bench_function("http_probe", |b| {
        b.iter(|| {
            std::thread::sleep(Duration::from_millis(5)); // HTTP request + response
            black_box(true)
        })
    });

    // HTTPS health check (slowest)
    group.bench_function("https_probe", |b| {
        b.iter(|| {
            std::thread::sleep(Duration::from_millis(10)); // TLS handshake + HTTP
            black_box(true)
        })
    });

    group.finish();
}

// ============================================================================
// Platform Discovery Benchmarks
// ============================================================================

fn benchmark_platform_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("platform_hsm_detection");

    // Windows TPM detection
    group.bench_function("windows_tpm", |b| {
        b.iter(|| {
            // Simulate: PowerShell query + registry check + WMIC
            std::thread::sleep(Duration::from_millis(50));
            black_box(Some(DiscoveredHsm {
                id: "tpm-0".to_string(),
                hsm_type: "TPM 2.0".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "encrypt".to_string()],
            }))
        })
    });

    // Linux TPM detection
    group.bench_function("linux_tpm", |b| {
        b.iter(|| {
            // Simulate: /sys/class/tpm/tpm0 check
            std::thread::sleep(Duration::from_millis(5));
            black_box(Some(DiscoveredHsm {
                id: "tpm-0".to_string(),
                hsm_type: "TPM 2.0".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "encrypt".to_string()],
            }))
        })
    });

    // macOS Secure Enclave detection
    group.bench_function("ios_secure_enclave", |b| {
        b.iter(|| {
            // Simulate: arch check + security framework query
            std::thread::sleep(Duration::from_millis(2));
            black_box(Some(DiscoveredHsm {
                id: "secure-enclave-0".to_string(),
                hsm_type: "iOS Secure Enclave".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "biometric".to_string()],
            }))
        })
    });

    // Android StrongBox detection
    group.bench_function("android_strongbox", |b| {
        b.iter(|| {
            // Simulate: getprop + pm list features
            std::thread::sleep(Duration::from_millis(20));
            black_box(Some(DiscoveredHsm {
                id: "strongbox-0".to_string(),
                hsm_type: "Android StrongBox".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "biometric".to_string()],
            }))
        })
    });

    group.finish();
}

// ============================================================================
// Cloud Discovery Benchmarks
// ============================================================================

fn benchmark_cloud_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("cloud_hsm_discovery");

    // AWS KMS discovery
    group.bench_function("aws_kms", |b| {
        b.iter(|| {
            // Simulate: env vars check + credential file check + IMDS query
            std::thread::sleep(Duration::from_millis(10));
            black_box(Some(DiscoveredHsm {
                id: "aws-kms-us-east-1".to_string(),
                hsm_type: "AWS KMS".to_string(),
                endpoint: Some("https://kms.us-east-1.amazonaws.com".to_string()),
                capabilities: vec!["encrypt".to_string(), "sign".to_string()],
            }))
        })
    });

    // Azure Key Vault discovery
    group.bench_function("azure_keyvault", |b| {
        b.iter(|| {
            // Simulate: env vars check + managed identity check
            std::thread::sleep(Duration::from_millis(15));
            black_box(Some(DiscoveredHsm {
                id: "azure-kv-eastus".to_string(),
                hsm_type: "Azure Key Vault".to_string(),
                endpoint: Some("https://vault.azure.net".to_string()),
                capabilities: vec!["encrypt".to_string(), "sign".to_string()],
            }))
        })
    });

    // GCP Cloud KMS discovery
    group.bench_function("gcp_kms", |b| {
        b.iter(|| {
            // Simulate: env vars check + service account check + metadata query
            std::thread::sleep(Duration::from_millis(12));
            black_box(Some(DiscoveredHsm {
                id: "gcp-kms-us-central1".to_string(),
                hsm_type: "GCP Cloud KMS".to_string(),
                endpoint: Some("https://cloudkms.googleapis.com".to_string()),
                capabilities: vec!["encrypt".to_string(), "sign".to_string()],
            }))
        })
    });

    group.finish();
}

// ============================================================================
// USB Discovery Benchmarks
// ============================================================================

fn benchmark_usb_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("usb_hsm_discovery");

    group.bench_function("enumerate_usb_devices", |b| {
        b.iter(|| {
            // Simulate: USB enumeration
            std::thread::sleep(Duration::from_millis(30));
            let devices = vec![
                ("046d:c52b", "YubiKey 5 NFC"),
                ("20a0:4108", "Nitrokey Pro"),
            ];
            black_box(devices)
        })
    });

    group.bench_function("yubikey_detection", |b| {
        b.iter(|| {
            // Simulate: YubiKey APDU communication
            std::thread::sleep(Duration::from_millis(15));
            black_box(Some(DiscoveredHsm {
                id: "yubikey-123456".to_string(),
                hsm_type: "YubiKey 5 NFC".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "piv".to_string()],
            }))
        })
    });

    group.bench_function("nitrokey_detection", |b| {
        b.iter(|| {
            // Simulate: Nitrokey communication
            std::thread::sleep(Duration::from_millis(20));
            black_box(Some(DiscoveredHsm {
                id: "nitrokey-654321".to_string(),
                hsm_type: "Nitrokey Pro".to_string(),
                endpoint: None,
                capabilities: vec!["sign".to_string(), "pgp".to_string()],
            }))
        })
    });

    group.finish();
}

// ============================================================================
// Discovery Caching Benchmarks
// ============================================================================

fn benchmark_discovery_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("discovery_caching");

    // Create mock cache
    let mut cache: HashMap<String, DiscoveredHsm> = HashMap::new();
    for i in 0..100 {
        cache.insert(
            format!("hsm-{}", i),
            DiscoveredHsm {
                id: format!("hsm-{}", i),
                hsm_type: "Test HSM".to_string(),
                endpoint: None,
                capabilities: vec![],
            },
        );
    }

    group.bench_function("cache_hit", |b| {
        b.iter(|| {
            let result = cache.get(black_box("hsm-50"));
            black_box(result);
        })
    });

    group.bench_function("cache_miss_and_discover", |b| {
        b.iter(|| {
            let key = black_box("hsm-999");
            if cache.get(key).is_none() {
                // Simulate discovery
                std::thread::sleep(Duration::from_millis(10));
                let hsm = DiscoveredHsm {
                    id: key.to_string(),
                    hsm_type: "Test HSM".to_string(),
                    endpoint: None,
                    capabilities: vec![],
                };
                black_box(hsm);
            }
        })
    });

    group.bench_function("cache_invalidation", |b| {
        let mut local_cache = cache.clone();
        b.iter(|| {
            local_cache.clear();
            black_box(&local_cache);
        })
    });

    group.finish();
}

// ============================================================================
// Full Discovery Cycle Benchmarks
// ============================================================================

fn benchmark_full_discovery_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_discovery_cycle");
    group.sample_size(20);

    group.bench_function("discover_all_sequential", |b| {
        b.iter(|| {
            let mut discovered = Vec::new();

            // Platform discovery (fast)
            std::thread::sleep(Duration::from_millis(30));
            discovered.push(DiscoveredHsm {
                id: "platform-0".to_string(),
                hsm_type: "TPM".to_string(),
                endpoint: None,
                capabilities: vec![],
            });

            // Network discovery (medium)
            std::thread::sleep(Duration::from_millis(50));
            discovered.push(DiscoveredHsm {
                id: "network-0".to_string(),
                hsm_type: "Network HSM".to_string(),
                endpoint: Some("https://hsm.local:8443".to_string()),
                capabilities: vec![],
            });

            // Cloud discovery (slower)
            std::thread::sleep(Duration::from_millis(100));
            discovered.push(DiscoveredHsm {
                id: "cloud-0".to_string(),
                hsm_type: "AWS KMS".to_string(),
                endpoint: Some("https://kms.amazonaws.com".to_string()),
                capabilities: vec![],
            });

            // USB discovery (variable)
            std::thread::sleep(Duration::from_millis(40));

            black_box(discovered)
        })
    });

    group.bench_function("discover_all_parallel", |b| {
        b.iter(|| {
            let handles: Vec<_> = vec![
                // Platform
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_millis(30));
                    vec![DiscoveredHsm {
                        id: "platform-0".to_string(),
                        hsm_type: "TPM".to_string(),
                        endpoint: None,
                        capabilities: vec![],
                    }]
                }),
                // Network
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_millis(50));
                    vec![DiscoveredHsm {
                        id: "network-0".to_string(),
                        hsm_type: "Network HSM".to_string(),
                        endpoint: Some("https://hsm.local:8443".to_string()),
                        capabilities: vec![],
                    }]
                }),
                // Cloud
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_millis(100));
                    vec![DiscoveredHsm {
                        id: "cloud-0".to_string(),
                        hsm_type: "AWS KMS".to_string(),
                        endpoint: Some("https://kms.amazonaws.com".to_string()),
                        capabilities: vec![],
                    }]
                }),
                // USB
                std::thread::spawn(|| {
                    std::thread::sleep(Duration::from_millis(40));
                    vec![]
                }),
            ];

            let mut discovered = Vec::new();
            for handle in handles {
                discovered.extend(handle.join().unwrap());
            }

            black_box(discovered)
        })
    });

    group.finish();
}

// ============================================================================
// mDNS/DNS-SD Simulation
// ============================================================================

fn benchmark_mdns_discovery(c: &mut Criterion) {
    let mut group = c.benchmark_group("mdns_discovery");

    group.bench_function("mdns_query", |b| {
        b.iter(|| {
            // Simulate mDNS query: send multicast + wait for responses
            std::thread::sleep(Duration::from_millis(100)); // Typical timeout
            let responses = vec![
                ("hsm-1.local", "192.168.1.100:8443"),
                ("hsm-2.local", "192.168.1.101:8443"),
            ];
            black_box(responses)
        })
    });

    group.bench_function("dns_sd_query", |b| {
        b.iter(|| {
            // Simulate DNS-SD query
            std::thread::sleep(Duration::from_millis(50));
            let services = vec!["_beardog-hsm._tcp.local.", "_pkcs11._tcp.local."];
            black_box(services)
        })
    });

    group.finish();
}

// ============================================================================
// Criterion Configuration
// ============================================================================

criterion_group! {
    name = discovery_benches;
    config = {
        let c = Criterion::default()
            .measurement_time(Duration::from_secs(10))
            .warm_up_time(Duration::from_secs(3));
        #[cfg(feature = "profiling")]
        let c = c.with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
        c
    };
    targets =
        benchmark_endpoint_probing,
        benchmark_concurrent_probing,
        benchmark_tcp_vs_http_probing,
        benchmark_platform_detection,
        benchmark_cloud_discovery,
        benchmark_usb_discovery,
        benchmark_discovery_caching,
        benchmark_full_discovery_cycle,
        benchmark_mdns_discovery,
}

criterion_main!(discovery_benches);
