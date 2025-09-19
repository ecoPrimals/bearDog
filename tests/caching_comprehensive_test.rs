use beardog_errors::BearDogError;
use beardog_utils::caching::*;
use std::time::Duration;
use tokio::time::sleep;

#[cfg(test)]
mod caching_tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_manager_initialization() {
        let config = CacheConfig::default();
        let manager = AdvancedCacheManager::new(config);
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_multi_tier_caching() {
        let config = CacheConfig::default();
        let manager = AdvancedCacheManager::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        let key = "test_key";
        let value = b"test_value".to_vec();

        manager
            .put(key, value.clone())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        let retrieved = manager
            .get(key)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    fn test_l1_cache_performance() {
        let config = L1CacheConfig::default();
        let mut l1_cache = L1Cache::new(config);

        let start = std::time::Instant::now();

        for i in 0..1000 {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i).into_bytes();
            l1_cache
                .put(&key, value)
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        }

        let duration = start.elapsed();
        assert!(duration.as_millis() < 100); // Should be very fast
    }

    #[tokio::test]
    async fn test_cache_eviction_strategies() {
        let mut config = CacheConfig::default();
        config.eviction_strategy = EvictionStrategy::LRU;
        config.max_size = 2; // Small cache to trigger eviction

        let manager = AdvancedCacheManager::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        manager
            .put("key1", b"value1".to_vec())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        manager
            .put("key2", b"value2".to_vec())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        manager
            .put("key3", b"value3".to_vec())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        let result = manager
            .get("key1")
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        assert_eq!(result, None);

        assert!(manager
            .get("key2")
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
            .is_some());
        assert!(manager
            .get("key3")
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
            .is_some());
    }

    #[tokio::test]
    async fn test_cache_metrics() {
        let config = CacheConfig::default();
        let manager = AdvancedCacheManager::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        manager
            .put("key1", b"value1".to_vec())
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        let _ = manager
            .get("key1")
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?; // Hit
        let _ = manager
            .get("key2")
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?; // Miss

        let metrics = manager.get_metrics();
        assert!(metrics.hits > 0);
        assert!(metrics.misses > 0);
        assert!(metrics.hit_rate() > 0.0);
    }

    #[test]
    fn test_memory_pool_allocation() {
        let manager = MemoryManager::new();

        let block1 = manager.allocate(1024);
        let block2 = manager.allocate(2048);

        assert!(block1.is_ok());
        assert!(block2.is_ok());

        let stats = manager.get_stats();
        assert_eq!(stats.active_allocations, 2);
        assert!(stats.total_allocated >= 3072);
    }

    #[tokio::test]
    async fn test_cache_warming_strategy() {
        let mut config = CacheConfig::default();
        config.warming_strategy = WarmingStrategy::Proactive;

        let manager = AdvancedCacheManager::new(config)
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

        let warm_keys = vec!["warm1", "warm2", "warm3"];
        for key in &warm_keys {
            manager
                .put(key, format!("value_{}", key).into_bytes())
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
        }

        for key in &warm_keys {
            let result = manager
                .get(key)
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
            assert!(result.is_some());
        }
    }
}
