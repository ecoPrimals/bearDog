

use std::collections::HashMap;
use std::time::Duration;

pub use beardog_errors::BearDogResult;

pub type TestResult<T = ()> = BearDogResult<T>;

pub type AssertionResult<T = ()> = BearDogResult<T>;

pub type ProviderMetrics = HashMap<String, f64>;

pub type SystemMetrics = HashMap<String, f64>;

pub type BenchmarkMetrics = HashMap<String, Duration>;

pub type CacheResult<T> = BearDogResult<T>;

pub type CacheStats = HashMap<String, u64>;

pub type MemoryCache<K, V> = HashMap<K, V>;

pub type ConfigResult<T> = BearDogResult<T>;

pub type ConfigMap = HashMap<String, String>;

pub type SettingsMap = HashMap<String, String>;

pub type NetworkResult<T> = BearDogResult<T>;

pub type ConnectionPool<T> = Vec<T>;

pub type EndpointMap = HashMap<String, String>;

pub type SecurityResult<T> = BearDogResult<T>;

pub type CryptoResult<T> = BearDogResult<T>;

pub type KeyStore = HashMap<String, Vec<u8>>;

pub type WorkflowResult<T> = BearDogResult<T>;

pub type WorkflowRegistry = HashMap<String, String>;

pub type ProcessResult<T> = BearDogResult<T>;

pub type HsmResult<T> = BearDogResult<T>;

pub type KeyResult<T> = BearDogResult<T>;

pub type HsmRegistry = HashMap<String, String>;

pub type MigrationResult<T> = BearDogResult<T>;

pub type RefactorResult<T> = BearDogResult<T>;

pub fn validate_type_aliases() -> BearDogResult<()> {

    let _test_result: BearDogResult<()> = Ok(());
    let _test_result2: TestResult<()> = Ok(());
    let _assertion_result: AssertionResult<()> = Ok(());

    let _provider_metrics: ProviderMetrics = HashMap::with_capacity(16);
    let _system_metrics: SystemMetrics = HashMap::with_capacity(16);

    let _cache_stats: CacheStats = HashMap::with_capacity(16);
    let _memory_cache: MemoryCache<String, String> = HashMap::with_capacity(16);

    let _config_map: ConfigMap = HashMap::with_capacity(16);
    let _settings_map: SettingsMap = HashMap::with_capacity(16);

    let _endpoint_map: EndpointMap = HashMap::with_capacity(16);

    let _key_store: KeyStore = HashMap::with_capacity(16);

    let _workflow_registry: WorkflowRegistry = HashMap::with_capacity(16);

    let _hsm_registry: HsmRegistry = HashMap::with_capacity(16);
    
    Ok(())
}

pub fn get_alias_info() -> Vec<(&'static str, &'static str)> {
    vec![
        ("BearDogResult<T>", "Primary result type for all BearDog operations"),
        ("TestResult<T>", "Specialized result type for testing operations"),
        ("AssertionResult<T>", "Result type for test assertions and validations"),
        ("ProviderMetrics", "Performance and operational metrics collection"),
        ("SystemMetrics", "System-wide performance metrics"),
        ("BenchmarkMetrics", "Performance benchmarking data"),
        ("CacheResult<T>", "Cache operation results"),
        ("CacheStats", "Cache performance statistics"),
        ("MemoryCache<K,V>", "Generic in-memory cache"),
        ("ConfigResult<T>", "Configuration operation results"),
        ("ConfigMap", "Configuration key-value storage"),
        ("SettingsMap", "Application settings storage"),
        ("NetworkResult<T>", "Network operation results"),
        ("ConnectionPool<T>", "Network connection pool"),
        ("EndpointMap", "Service endpoint registry"),
        ("SecurityResult<T>", "Security operation results"),
        ("CryptoResult<T>", "Cryptographic operation results"),
        ("KeyStore", "Cryptographic key storage"),
        ("WorkflowResult<T>", "Workflow operation results"),
        ("WorkflowRegistry", "Active workflow registry"),
        ("ProcessResult<T>", "Process execution results"),
        ("HsmResult<T>", "HSM operation results"),
        ("KeyResult<T>", "Key management operation results"),
        ("HsmRegistry", "HSM provider registry"),
        ("MigrationResult<T>", "Code migration operation results"),
        ("RefactorResult<T>", "Code refactoring operation results"),
    ]
} 