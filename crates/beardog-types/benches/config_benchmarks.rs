use beardog_types::canonical::config::UnifiedBearDogConfig;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation_default", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::default()))
    });

    c.bench_function("config_creation_new", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::new()))
    });
}

fn benchmark_config_validation(c: &mut Criterion) {
    let config = UnifiedBearDogConfig::default();

    c.bench_function("config_validation_default", |b| {
        b.iter(|| black_box(config.validate()).unwrap())
    });

    let mut production_config = UnifiedBearDogConfig::default();
    production_config.app.name = "BearDog".to_string();
    production_config.app.version = "3.0.0".to_string();
    production_config.network.port = 8080;
    production_config.database.connection_string =
        "postgresql://localhost:5432/beardog".to_string();

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

fn benchmark_config_merge(c: &mut Criterion) {
    let mut base_config = UnifiedBearDogConfig::default();
    base_config.app.name = "BearDog".to_string();

    let mut override_config = UnifiedBearDogConfig::default();
    override_config.app.version = "3.0.0".to_string();
    override_config.network.port = 8080;

    c.bench_function("config_merge", |b| {
        b.iter(|| {
            let mut config = base_config.clone();
            black_box(config.merge(override_config.clone())).unwrap()
        })
    });
}

fn benchmark_config_summary(c: &mut Criterion) {
    let config = UnifiedBearDogConfig::default();

    c.bench_function("config_summary", |b| b.iter(|| black_box(config.summary())));
}

fn benchmark_config_from_env(c: &mut Criterion) {
    // Set up environment variables
    std::env::set_var("BEARDOG_ENVIRONMENT", "production");
    std::env::set_var("BEARDOG_DEBUG", "false");
    std::env::set_var("BEARDOG_PORT", "8080");

    c.bench_function("config_from_env", |b| {
        b.iter(|| black_box(UnifiedBearDogConfig::from_env()).unwrap())
    });

    // Clean up
    std::env::remove_var("BEARDOG_ENVIRONMENT");
    std::env::remove_var("BEARDOG_DEBUG");
    std::env::remove_var("BEARDOG_PORT");
}

criterion_group!(
    config_benches,
    benchmark_config_creation,
    benchmark_config_validation,
    benchmark_config_serialization,
    benchmark_config_merge,
    benchmark_config_summary,
    benchmark_config_from_env
);

criterion_main!(config_benches);
