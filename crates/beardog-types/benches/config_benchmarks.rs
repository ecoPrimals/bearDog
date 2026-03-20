// SPDX-License-Identifier: AGPL-3.0-only
use beardog_types::canonical::config::UnifiedBearDogConfig;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation_default", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::default()))
    });

    c.bench_function("config_creation_load", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::load()).unwrap())
    });
}

fn benchmark_config_validation(c: &mut Criterion) {
    let config = UnifiedBearDogConfig::default();

    c.bench_function("config_validation_default", |b| {
        b.iter(|| black_box(config.validate()).unwrap())
    });

    let mut production_config = UnifiedBearDogConfig::default();
    production_config.app.app_name = "BearDog".to_string();
    production_config.app.app_description = "Production deployment".to_string();
    production_config.metadata.version.beardog_version = "3.0.0".to_string();

    c.bench_function("config_validation_production", |b| {
        b.iter(|| black_box(production_config.validate()).unwrap())
    });
}

fn benchmark_config_serialization(c: &mut Criterion) {
    let config = UnifiedBearDogConfig::default();

    c.bench_function("config_serialize_json", |b| {
        b.iter(|| black_box(serde_json::to_string(&config)).unwrap())
    });

    let json_str = serde_json::to_string(&config).unwrap();
    c.bench_function("config_deserialize_json", |b| {
        b.iter(|| black_box(serde_json::from_str::<UnifiedBearDogConfig>(&json_str)).unwrap())
    });
}

fn benchmark_config_clone(c: &mut Criterion) {
    let config = UnifiedBearDogConfig::default();

    c.bench_function("config_clone", |b| {
        b.iter(|| {
            let cloned = config.clone();
            black_box(cloned)
        })
    });
}

fn benchmark_config_migration(c: &mut Criterion) {
    c.bench_function("config_migrate_from_legacy", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::migrate_from_legacy()).unwrap())
    });
}

fn benchmark_config_from_env(c: &mut Criterion) {
    // Set up environment variables
    beardog_errors::process_env::set_var("BEARDOG_ENVIRONMENT", "production");

    c.bench_function("config_load_from_env", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::load()).unwrap())
    });

    // Clean up
    beardog_errors::process_env::remove_var("BEARDOG_ENVIRONMENT");
}

criterion_group!(
    config_benches,
    benchmark_config_creation,
    benchmark_config_validation,
    benchmark_config_serialization,
    benchmark_config_clone,
    benchmark_config_migration,
    benchmark_config_from_env
);

criterion_main!(config_benches);
