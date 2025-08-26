

use beardog_errors::{BearDogError, BearDogResult};
use std::sync::atomic::{AtomicPtr, AtomicUsize, AtomicU64, Ordering};
use std::ptr;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use tracing::{debug, info};

pub struct LockFreeHashMap<K, V> 
where
    K: Hash + Eq + Clone,
    V: Clone,
{

    buckets: Vec<AtomicPtr<HashEntry<K, V>>>,

    size: AtomicUsize,

    capacity: usize,

    stats: LockFreeStats,
}

#[repr(align(64))] // Cache line alignment
struct HashEntry<K, V> {
    key: K,
    value: V,
    next: AtomicPtr<HashEntry<K, V>>,
    hash: u64,
}

#[derive(Debug, Default)]
pub struct LockFreeStats {

    pub insertions: AtomicU64,

    pub lookups: AtomicU64,

    pub cache_hits: AtomicU64,

    pub cache_misses: AtomicU64,

    pub collisions: AtomicU64,
}

impl<K, V> LockFreeHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{

    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two(); // Ensure power of 2 for fast modulo
        let mut buckets = Vec::with_capacity(capacity);
        
        for _ in 0..capacity {
            buckets.push(AtomicPtr::new(ptr::null_mut()));
        }
        
        info!("🔓 Lock-free HashMap created: {} buckets", capacity);
        
        Self {
            buckets,
            size: AtomicUsize::new(0),
            capacity,
            stats: LockFreeStats::default(),
        }
    }

    pub fn insert(&self, key: K, value: V) -> BearDogResult<Option<V>> {
        let hash = self.hash_key(&key);
        let bucket_index = (hash as usize) & (self.capacity - 1); // Fast modulo for power of 2
        
        let new_entry = Box::into_raw(Box::new(HashEntry {
            key: key.clone(),
            value: value.clone(),
            next: AtomicPtr::new(ptr::null_mut()),
            hash,
        }));
        
        loop {
            let bucket = &self.buckets[bucket_index];
            let current_head = bucket.load(Ordering::Acquire);

            if let Some(_existing_value) = self.find_in_chain(current_head, &key, hash) {

                unsafe {
                    (*new_entry).value = value.clone();

                    let mut current = current_head;
                    while !current.is_null() {
                        if (*current).key == key && (*current).hash == hash {

                            let old_value = (*current).value.clone();
                            (*current).value = value;

                            let _ = Box::from_raw(new_entry);
                            
                            self.stats.insertions.fetch_add(1, Ordering::Relaxed);
                            return Ok(Some(old_value));
                        }
                        current = (*current).next.load(Ordering::Acquire);
                    }
                }
            }

            unsafe {
                (*new_entry).next.store(current_head, Ordering::Relaxed);
            }

            match bucket.compare_exchange_weak(
                current_head,
                new_entry,
                Ordering::Release,
                Ordering::Relaxed
            ) {
                Ok(_) => {
                    self.size.fetch_add(1, Ordering::Relaxed);
                    self.stats.insertions.fetch_add(1, Ordering::Relaxed);
                    debug!("🔓 Inserted key into lock-free map");
                    return Ok(None);
                }
                Err(_) => {

                    continue;
                }
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let hash = self.hash_key(key);
        let bucket_index = (hash as usize) & (self.capacity - 1);
        
        let bucket = &self.buckets[bucket_index];
        let head = bucket.load(Ordering::Acquire);
        
        self.stats.lookups.fetch_add(1, Ordering::Relaxed);
        
        if let Some(value) = self.find_in_chain(head, key, hash) {
            self.stats.cache_hits.fetch_add(1, Ordering::Relaxed);
            Some(value)
        } else {
            self.stats.cache_misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_stats(&self) -> LockFreeMapStats {
        let insertions = self.stats.insertions.load(Ordering::Relaxed);
        let lookups = self.stats.lookups.load(Ordering::Relaxed);
        let cache_hits = self.stats.cache_hits.load(Ordering::Relaxed);
        let _cache_misses = self.stats.cache_misses.load(Ordering::Relaxed);
        
        let hit_rate = if lookups > 0 {
            (cache_hits as f64 / lookups as f64) * 100.0
        } else {
            0.0
        };
        
        LockFreeMapStats {
            size: self.len(),
            capacity: self.capacity,
            insertions,
            lookups,
            hit_rate_percent: hit_rate,
            load_factor: self.len() as f64 / self.capacity as f64,
        }
    }

    fn hash_key(&self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    fn find_in_chain(&self, mut head: *mut HashEntry<K, V>, key: &K, hash: u64) -> Option<V> {
        unsafe {
            while !head.is_null() {
                if (*head).hash == hash && (*head).key == *key {
                    return Some((*head).value.clone());
                }
                head = (*head).next.load(Ordering::Acquire);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct LockFreeMapStats {
    pub size: usize,
    pub capacity: usize,
    pub insertions: u64,
    pub lookups: u64,
    pub hit_rate_percent: f64,
    pub load_factor: f64,
}

pub struct LockFreeQueue<T> {

    head: AtomicPtr<QueueNode<T>>,

    tail: AtomicPtr<QueueNode<T>>,

    size: AtomicUsize,
}

#[repr(align(64))] // Cache line alignment
struct QueueNode<T> {
    data: Option<T>,
    next: AtomicPtr<QueueNode<T>>,
}

impl<T> LockFreeQueue<T> {

    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(QueueNode {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        
        info!("🔓 Lock-free Queue created");
        
        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
            size: AtomicUsize::new(0),
        }
    }

    pub fn enqueue(&self, item: T) {
        let new_node = Box::into_raw(Box::new(QueueNode {
            data: Some(item),
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            
            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {

                    if unsafe { (*tail).next.compare_exchange_weak(
                        next,
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed
                    ).is_ok() } {

                        let _ = self.tail.compare_exchange_weak(
                            tail,
                            new_node,
                            Ordering::Release,
                            Ordering::Relaxed
                        );
                        break;
                    }
                } else {

                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                }
            }
        }
        
        self.size.fetch_add(1, Ordering::Relaxed);
        debug!("🔓 Enqueued item to lock-free queue");
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };
            
            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if next.is_null() {

                        return None;
                    }

                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    );
                } else {

                    let data = unsafe { (*next).data.take() };

                    if self.head.compare_exchange_weak(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed
                    ).is_ok() {

                        unsafe {
                            let _ = Box::from_raw(head); // Free old head
                        }
                        
                        self.size.fetch_sub(1, Ordering::Relaxed);
                        debug!("🔓 Dequeued item from lock-free queue");
                        return data;
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Default for LockFreeQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct LockFreeCounter {
    value: AtomicU64,
    max_value: u64,
}

impl LockFreeCounter {

    pub fn new(max_value: u64) -> Self {
        Self {
            value: AtomicU64::new(0),
            max_value,
        }
    }

    pub fn increment(&self) -> BearDogResult<u64> {
        loop {
            let current = self.value.load(Ordering::Relaxed);
            if current >= self.max_value {
                return Err(BearDogError::system("Counter overflow"));
            }
            
            match self.value.compare_exchange_weak(
                current,
                current + 1,
                Ordering::Relaxed,
                Ordering::Relaxed
            ) {
                Ok(_) => return Ok(current + 1),
                Err(_) => continue,
            }
        }
    }

    pub fn decrement(&self) -> BearDogResult<u64> {
        loop {
            let current = self.value.load(Ordering::Relaxed);
            if current == 0 {
                return Err(BearDogError::business("Counter underflow"));
            }
            
            match self.value.compare_exchange_weak(
                current,
                current - 1,
                Ordering::Relaxed,
                Ordering::Relaxed
            ) {
                Ok(_) => return Ok(current - 1),
                Err(_) => continue,
            }
        }
    }

    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn reset(&self) -> u64 {
        self.value.swap(0, Ordering::Relaxed)
    }
}

impl<K, V> Drop for LockFreeHashMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn drop(&mut self) {

        for bucket in &self.buckets {
            let mut current = bucket.load(Ordering::Relaxed);
            while !current.is_null() {
                unsafe {
                    let next = (*current).next.load(Ordering::Relaxed);
                    let _ = Box::from_raw(current);
                    current = next;
                }
            }
        }
    }
}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {

        while self.dequeue().is_some() {}

        let head = self.head.load(Ordering::Relaxed);
        if !head.is_null() {
            unsafe {
                let _ = Box::from_raw(head);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::sync::Arc;
    
    #[test]
    fn test_lock_free_hashmap() -> Result<(), Box<dyn std::error::Error>> {
        let map = LockFreeHashMap::new(16);

        assert_eq!(map.insert("key1".to_string(), "value1".to_string()).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?, None);
        assert_eq!(map.len(), 1);

        assert_eq!(map.get(&"key1".to_string()), Some("value1".to_string()));
        assert_eq!(map.get(&"nonexistent".to_string()), None);

        assert_eq!(map.insert("key1".to_string(), "value2".to_string()).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?, Some("value1".to_string()));
        assert_eq!(map.get(&"key1".to_string()), Some("value2".to_string()));
    }
    
    #[test]
    fn test_lock_free_queue() {
        let queue = LockFreeQueue::new();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(42);
        queue.enqueue(84);
        assert_eq!(queue.len(), 2);
        
        assert_eq!(queue.dequeue(), Some(42));
        assert_eq!(queue.dequeue(), Some(84));
        assert_eq!(queue.dequeue(), None);
        assert!(queue.is_empty());
        Ok(())
    }
    
    #[test]
    fn test_lock_free_counter() -> Result<(), Box<dyn std::error::Error>> {
        let counter = LockFreeCounter::new(100);
        
        assert_eq!(counter.get(), 0);
        assert_eq!(counter.increment().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?, 1);
        assert_eq!(counter.increment().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?, 2);
        assert_eq!(counter.decrement().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?, 1);
        assert_eq!(counter.get(), 1);
        Ok(())
    }
    
    #[test]
    fn test_concurrent_hashmap() -> Result<(), Box<dyn std::error::Error>> {
        let map = Arc::new(LockFreeHashMap::new(64));
        let mut handles = vec![];

        for i in 0..4 {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    let key = format_args!("thread{}_key{}", i, j).to_string();
                    let value = format_args!("value{}", j).to_string();
                    map_clone.insert(key.clone(), value.clone()).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
                    assert_eq!(map_clone.get(&key), Some(value));
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        }
        
        assert_eq!(map.len(), 400);
        let stats = map.get_stats();
        assert_eq!(stats.insertions, 400);
        Ok(())
    }
} 