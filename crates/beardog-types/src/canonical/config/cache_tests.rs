// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

#[test]
fn test_cache_config_default() {
    let config = CanonicalCacheConfig::default();
    assert!(!config.enabled);
    assert_eq!(config.max_size_mb, 0);
    assert!(matches!(config.eviction_policy, EvictionPolicy::Lru));
}

#[test]
fn test_cache_config_enabled() {
    let config = CanonicalCacheConfig {
        enabled: true,
        max_size_mb: 512,
        ttl: Duration::from_secs(3600),
        eviction_policy: EvictionPolicy::Lru,
        ..Default::default()
    };
    assert!(config.enabled);
    assert_eq!(config.max_size_mb, 512);
    assert_eq!(config.ttl, Duration::from_secs(3600));
}

#[test]
fn test_cache_config_production() {
    let config = CanonicalCacheConfig {
        enabled: true,
        max_size_mb: 4096, // 4GB for production
        ttl: Duration::from_secs(1800),
        eviction_policy: EvictionPolicy::Lru,
        ..Default::default()
    };
    assert_eq!(config.max_size_mb, 4096);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[test]
fn test_eviction_policy_lru() {
    let policy = EvictionPolicy::Lru;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(matches!(policy, EvictionPolicy::Lru));
}

#[test]
fn test_eviction_policy_lfu() {
    let policy = EvictionPolicy::Lfu;
    assert!(matches!(policy, EvictionPolicy::Lfu));
}

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
fn test_eviction_policy_fifo() {
    let policy = EvictionPolicy::Fifo;
    assert!(matches!(policy, EvictionPolicy::Fifo));
}

#[test]
fn test_eviction_policy_random() {
    let policy = EvictionPolicy::Random;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(matches!(policy, EvictionPolicy::Random));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_eviction_policy_default() {
    let policy = EvictionPolicy::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert!(matches!(policy, EvictionPolicy::Lru));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_cache_ttl_short() {
    let config = CanonicalCacheConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        ttl: Duration::from_secs(300), // 5 minutes
        ..Default::default()
    };
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(config.ttl, Duration::from_secs(300));
}

#[test]
fn test_cache_ttl_long() {
    let config = CanonicalCacheConfig {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        ttl: Duration::from_secs(7200), // 2 hours
        ..Default::default()
    };
    assert_eq!(config.ttl, Duration::from_secs(7200));
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_type_alias() {
    let _config: CacheConfig = CanonicalCacheConfig::default();
}

#[test]
fn test_l1_cache_config() {
    let config = CanonicalCacheConfig::high_performance_l1();
    assert!(config.enabled);
    assert!(config.tier.is_some());
    if let Some(CacheTier::L1 {
        max_entries,
        track_access,
        ..
    }) = config.tier
    {
        assert_eq!(max_entries, 50_000);
        assert!(track_access);
    } else {
        panic!("Expected L1 tier");
    }
}

#[test]
fn test_l2_cache_config() {
    let config = CanonicalCacheConfig::high_performance_l2();
    assert!(config.enabled);
    assert!(config.tier.is_some());
    if let Some(CacheTier::L2 {
        max_size_mb,
        enable_compression,
    }) = config.tier
    {
        assert_eq!(max_size_mb, 500);
        assert!(!enable_compression); // Disabled for performance
    } else {
        panic!("Expected L2 tier");
    }
}

#[test]
fn test_l3_cache_config() {
    let config = CanonicalCacheConfig::high_performance_l3();
    assert!(config.enabled);
    assert!(config.tier.is_some());
    if let Some(CacheTier::L3 {
        max_size_gb,
        enable_persistence,
        ..
    }) = config.tier
    {
        assert_eq!(max_size_gb, 5);
        assert!(enable_persistence);
    } else {
        panic!("Expected L3 tier");
    }
}

#[test]
fn test_memory_optimized_preset() {
    let config = CanonicalCacheConfig::memory_optimized();
    assert!(config.enabled);
    assert!(config.enable_compression);
    assert!(!config.enable_statistics);
    assert_eq!(config.max_size_mb, 256);
}

#[test]
fn test_warming_strategy() {
    let config = CanonicalCacheConfig {
        enabled: true,
        max_size_mb: 1024,
        ttl: Duration::from_secs(3600),
        eviction_policy: EvictionPolicy::Lru,
        warming_strategy: Some(WarmingStrategy::Predictive),
        ..Default::default()
    };
    assert_eq!(config.warming_strategy, Some(WarmingStrategy::Predictive));
}

#[test]
fn test_type_aliases() {
    // Test all type aliases compile
    let _: CacheConfig = CanonicalCacheConfig::default();
    let _: L1CacheConfig = CanonicalCacheConfig::default();
    let _: L2CacheConfig = CanonicalCacheConfig::default();
    let _: L3CacheConfig = CanonicalCacheConfig::default();
    let _: DiscoveryCacheConfig = CanonicalCacheConfig::default();
    let _: NetworkCacheConfig = CanonicalCacheConfig::default();
    let _: AdapterCacheConfig = CanonicalCacheConfig::default();
    let _: StorageCacheConfig = CanonicalCacheConfig::default();
    let _: PerformanceCacheConfig = CanonicalCacheConfig::default();
}
