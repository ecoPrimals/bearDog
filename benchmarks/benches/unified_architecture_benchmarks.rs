use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;

use beardog_traits::canonical::{PlatformProvider, UniversalProvider};
use beardog_types::config::{ProcessorType, UnifiedProcessorConfig};

struct UnifiedProviderImpl {
    config: UnifiedProcessorConfig,
}

impl UniversalProvider for UnifiedProviderImpl {
    type Config = UnifiedProcessorConfig;
    type Error = std::io::Error;

    fn process(&self, _data: &[u8]) -> Result<Vec<u8>, Self::Error> {

        Ok(vec![42; 1024])
    }

    fn configure(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = config;
        Ok(u64,
    max_ops: usize,
}

impl LegacyProviderImpl {
    fn process(&self, _data: &[u8]) -> Result<Vec<u8>, std::io::Error> {

        Ok(vec![42; 1024])
    }

    fn set_timeout(&mut self, timeout: u64) {
        self.timeout = timeout;
    }

    fn set_max_ops(&mut self, max_ops: usize) {
        self.max_ops = max_ops;
    }
}

fn benchmark_provider_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group(UnifiedProcessorConfig {
            processor_type: ProcessorType::System,
            core: Default::default(),
            security: Default::default(300,
        max_ops: 100,
    };

    let test_data = vec![1u8; 1024];

    group.bench_function("unified_provider", |b| {
        b.iter(|| {
            unified_provider
                .process(black_box(&test_data))
                .map_err(|e| BearDogError::system({:?}", e)))?
        })
    });

    group.bench_function("legacy_provider", |b| {
        b.iter(|| {
            legacy_provider
                .process(black_box(&test_data))
                .map_err(|e| BearDogError::system({:?}", e)))?
        })
    });

    group.finish();
}

fn benchmark_config_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group(ProcessorType::Security,
                core: Default::default(),
                security: Default::default(ProcessorType::System,
        core: Default::default(),
        security: Default::default(),
    };

    group.bench_function("unified_config_serialization", |b| {
        b.iter(|| {
            serde_json::to_string(black_box(&unified_config))
                .map_err(|e| BearDogError::system({:?}", e)))?
        })
    });

    group.finish();
}

fn benchmark_error_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_performance");

    group.bench_function("unified_error_creation", |b| {
        b.iter(|| {
            use beardog_errors::BearDogError;
            black_box(BearDogError::security("Test security error"))
        })
    });

    group.bench_function("legacy_error_creation", |b| {
        b.iter(|| {
            black_box(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "Test security error",
            ))
        })
    });

    group.finish();
}

fn benchmark_trait_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group(UnifiedProcessorConfig {
            processor_type: ProcessorType::System,
            core: Default::default(),
            security: Default::default(),
        },
    };

    let test_data = vec![1u8; 1024];

    group.bench_function("static_dispatch", |b| {
        b.iter(|| {
            unified_provider
                .process(black_box(&test_data))
                .map_err(|e| BearDogError::system({:?}", e)))?
        })
    });

    let provider: Box<
        dyn UniversalProvider<Config = UnifiedProcessorConfig, Error = std::io::Error>,
    > = Box::new(unified_provider);

    group.bench_function("dynamic_dispatch", |b| {
        b.iter(|| {
            provider
                .process(black_box(&test_data))
                .map_err(|e| BearDogError::system({:?}", e)))?
        })
    });

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    group.bench_function("unified_config_memory", |b| {
        b.iter(|| {
            let configs: Vec<UnifiedProcessorConfig> = (0..1000)
                .map(ProcessorType::System,
                    core: Default::default(),
                    security: Default::default(Vec<(String, u64, usize, bool, bool, u64, usize, u32)> = (0..1000)
                .map(|_| {
                    (
                        "system".to_string(),
                        300u64,
                        100usize,
                        true,
                        true,
                        60u64,
                        50usize,
                        3u32,
                    )
                })
                .collect();
            black_box(configs)
        })
    });

    group.finish();
}

fn benchmark_compilation_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("compilation_impact");
    group.measurement_time(Duration::from_secs(UnifiedProcessorConfig {
                        processor_type: if i % 2 == 0 {
                            ProcessorType::Security
                        } else {
                            ProcessorType::System
                        },
                        core: Default::default(),
                        security: Default::default(),
                    },
                };
                black_box(provider);
            }
        })
    });

    group.finish();
}

criterion_group!(
    unified_benchmarks,
    benchmark_provider_performance,
    benchmark_config_performance,
    benchmark_error_performance,
    benchmark_trait_dispatch,
    benchmark_memory_usage,
    benchmark_compilation_impact
);

criterion_main!(unified_benchmarks);
