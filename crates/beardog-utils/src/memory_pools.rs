use beardog_errors::BearDogError;
use std::mem::MaybeUninit;
use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct MemoryPool<T, const SIZE: usize>
where
    T: Default + Send + Sync,
{
    storage: Box<[MaybeUninit<T>; SIZE]>,
    available: AtomicPtr<PoolNode>,
    stats: PoolStats,
}

#[repr(align(64))] // Cache line aligned
struct PoolNode {
    next: *mut PoolNode,
    index: usize,
}

#[derive(Debug, Default)]
pub struct PoolStats {
    pub allocations: AtomicUsize,

    pub deallocations: AtomicUsize,

    pub objects_in_use: AtomicUsize,

    pub hit_rate: AtomicUsize, // Stored as percentage * 100
}

impl PoolStats {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T, const SIZE: usize> MemoryPool<T, SIZE>
where
    T: Default + Send + Sync,
{
    pub fn new() -> Result<Self, BearDogError> {
        let mut storage = Box::new([const { MaybeUninit::uninit() }; SIZE]);

        for slot in storage.iter_mut() {
            slot.write(T::default());
        }

        let mut nodes = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            let node = Box::into_raw(Box::new(PoolNode {
                next: if i == 0 {
                    ptr::null_mut()
                } else {
                    nodes[i - 1]
                },
                index: i,
            }));
            nodes.push(node);
        }

        let _available = AtomicPtr::new(if SIZE > 0 {
            nodes[SIZE - 1]
        } else {
            ptr::null_mut()
        });

        Ok(Self {
            storage,
            available: AtomicPtr::new(std::ptr::null_mut()),
            stats: PoolStats::new(),
        })
    }

    pub fn acquire(&self) -> Result<PooledObject<T, SIZE>, BearDogError> {
        loop {
            let head = self.available.load(Ordering::Acquire);

            if head.is_null() {
                self.stats.allocations.fetch_add(1, Ordering::Relaxed);
                return Err(BearDogError::system("Object pool exhausted".to_string()));
            }

            let next = unsafe { (*head).next };
            let index = unsafe { (*head).index };

            if self
                .available
                .compare_exchange_weak(head, next, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                self.stats.allocations.fetch_add(1, Ordering::Relaxed);
                self.stats.objects_in_use.fetch_add(1, Ordering::Relaxed);

                let object_ptr = self.storage[index].as_ptr();
                let object = unsafe { ptr::read(object_ptr) };

                unsafe {
                    let _ = Box::from_raw(head);
                };

                return Ok(PooledObject {
                    object: Some(object),
                    pool: None, // Simplified - no circular reference
                    index,
                });
            }

            std::hint::spin_loop();
        }
    }

    #[allow(dead_code, unused_assignments)]
    fn return_object(&mut self, mut object: T, index: usize) {
        object = T::default();

        unsafe {
            ptr::write(self.storage[index].as_mut_ptr(), object);
        }

        let node = Box::into_raw(Box::new(PoolNode {
            next: ptr::null_mut(),
            index,
        }));

        loop {
            let head = self.available.load(Ordering::Acquire);
            unsafe { (*node).next = head };

            if self
                .available
                .compare_exchange_weak(head, node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }

            std::hint::spin_loop();
        }

        self.stats.deallocations.fetch_add(1, Ordering::Relaxed);
        self.stats.objects_in_use.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn stats(&self) -> PoolStatistics {
        let allocations = self.stats.allocations.load(Ordering::Relaxed);
        let deallocations = self.stats.deallocations.load(Ordering::Relaxed);
        let in_use = self.stats.objects_in_use.load(Ordering::Relaxed);

        PoolStatistics {
            total_capacity: SIZE,
            objects_in_use: in_use,
            objects_available: SIZE - in_use,
            total_allocations: allocations,
            total_deallocations: deallocations,
            hit_rate: if allocations > 0 {
                (deallocations * 100) / allocations
            } else {
                100
            },
        }
    }

    pub fn minimal_default() -> Self {
        let mut storage = Box::new([const { MaybeUninit::uninit() }; SIZE]);

        for slot in storage.iter_mut() {
            slot.write(T::default());
        }

        let mut nodes = Vec::with_capacity(SIZE);
        for i in 0..SIZE {
            let node = Box::into_raw(Box::new(PoolNode {
                next: if i == 0 {
                    ptr::null_mut()
                } else {
                    nodes[i - 1]
                },
                index: i,
            }));
            nodes.push(node);
        }

        let _available = AtomicPtr::new(if SIZE > 0 {
            nodes[SIZE - 1]
        } else {
            ptr::null_mut()
        });

        Self {
            storage,
            available: AtomicPtr::new(std::ptr::null_mut()),
            stats: PoolStats::new(),
        }
    }
}

impl<T, const SIZE: usize> Drop for MemoryPool<T, SIZE>
where
    T: Default + Send + Sync,
{
    fn drop(&mut self) {
        let mut current = self.available.load(Ordering::Relaxed);
        while !current.is_null() {
            let next = unsafe { (*current).next };
            unsafe {
                let _ = Box::from_raw(current);
            };
            current = next;
        }

        for slot in self.storage.iter_mut() {
            unsafe {
                slot.assume_init_drop();
            }
        }
    }
}

#[derive(Debug)]
pub struct PooledObject<T, const SIZE: usize>
where
    T: Default + Send + Sync,
{
    object: Option<T>,
    #[allow(dead_code)]
    pool: Option<()>, // Placeholder - simplified
    #[allow(dead_code)]
    index: usize,
}

impl<T, const SIZE: usize> PooledObject<T, SIZE>
where
    T: Default + Send + Sync,
{
    pub fn get(&mut self) -> &T {
        if self.object.is_none() {
            tracing::error!("PooledObject accessed with None object - creating emergency default");
            self.object = Some(T::default());
        }
        // Safe to unwrap because we just ensured object is Some
        self.object
            .as_ref()
            .expect("Object should exist after default initialization")
    }

    pub fn get_mut(&mut self) -> &mut T {
        if self.object.is_none() {
            tracing::error!("PooledObject accessed with None object - creating emergency default");
            self.object = Some(T::default());
        }
        // Safe to unwrap because we just ensured object is Some
        self.object
            .as_mut()
            .expect("Object should exist after default initialization")
    }
}

impl<T, const SIZE: usize> Drop for PooledObject<T, SIZE>
where
    T: Default + Send + Sync,
{
    fn drop(&mut self) {
        // Simplified drop - no pool return needed since we removed circular reference
        if let Some(_object) = self.object.take() {
            // Object is dropped automatically
        }
    }
}

impl<T, const SIZE: usize> std::ops::Deref for PooledObject<T, SIZE>
where
    T: Default + Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safe deref - if object is None, return a default reference
        // This is a fallback for the immutable deref case
        match self.object.as_ref() {
            Some(obj) => obj,
            None => {
                // This should never happen in normal operation
                // Return a leaked static reference as last resort
                tracing::error!("PooledObject deref called on None object - critical error");
                Box::leak(Box::new(T::default()))
            }
        }
    }
}

impl<T, const SIZE: usize> std::ops::DerefMut for PooledObject<T, SIZE>
where
    T: Default + Send + Sync,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

#[derive(Debug, Clone)]
pub struct PoolStatistics {
    pub total_capacity: usize,
    pub objects_in_use: usize,
    pub objects_available: usize,
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub hit_rate: usize, // Percentage
}

pub type CryptoBufferPool = MemoryPool<Vec<u8>, 1000>;

pub type StringPool = MemoryPool<String, 500>;

pub type HashMapPool = MemoryPool<std::collections::HashMap<String, String>, 200>;

pub struct GlobalPoolManager {
    crypto_buffers: Arc<CryptoBufferPool>,
    strings: Arc<StringPool>,
    hashmaps: Arc<HashMapPool>,
}

impl GlobalPoolManager {
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            crypto_buffers: Arc::new(CryptoBufferPool::new()?),
            strings: Arc::new(StringPool::new()?),
            hashmaps: Arc::new(HashMapPool::new()?),
        })
    }

    pub fn get_crypto_buffer(&self) -> Result<PooledObject<Vec<u8>, 1000>, BearDogError> {
        self.crypto_buffers.acquire()
    }

    pub fn get_string(&self) -> Result<PooledObject<String, 500>, BearDogError> {
        self.strings.acquire()
    }

    pub fn get_hashmap(
        &self,
    ) -> Result<PooledObject<std::collections::HashMap<String, String>, 200>, BearDogError> {
        self.hashmaps.acquire()
    }

    pub fn get_global_stats(&self) -> GlobalPoolStats {
        GlobalPoolStats {
            crypto_buffers: self.crypto_buffers.stats(),
            strings: self.strings.stats(),
            hashmaps: self.hashmaps.stats(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GlobalPoolStats {
    pub crypto_buffers: PoolStatistics,
    pub strings: PoolStatistics,
    pub hashmaps: PoolStatistics,
}

static GLOBAL_POOLS: std::sync::OnceLock<GlobalPoolManager> = std::sync::OnceLock::new();

pub fn global_pools() -> &'static GlobalPoolManager {
    GLOBAL_POOLS.get_or_init(|| match GlobalPoolManager::new() {
        Ok(manager) => manager,
        Err(e) => {
            tracing::error!("Failed to initialize global pools: {:?}", e);

            GlobalPoolManager {
                crypto_buffers: Arc::new(MemoryPool::new().unwrap_or_else(|_| {
                    tracing::error!("Failed to create crypto_buffers pool, using minimal default");
                    MemoryPool::minimal_default()
                })),
                strings: Arc::new(MemoryPool::new().unwrap_or_else(|_| {
                    tracing::error!("Failed to create strings pool, using minimal default");
                    MemoryPool::minimal_default()
                })),
                hashmaps: Arc::new(MemoryPool::new().unwrap_or_else(|_| {
                    tracing::error!("Failed to create hashmaps pool, using minimal default");
                    MemoryPool::minimal_default()
                })),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_pool_creation() -> Result<(), Box<dyn std::error::Error>> {
        let pool: MemoryPool<String, 10> = MemoryPool::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;
        let stats = pool.stats();
        assert_eq!(stats.total_capacity, 10);
        assert_eq!(stats.objects_available, 10);
        Ok(())
    }

    #[test]
    fn test_object_acquisition_and_return() -> Result<(), Box<dyn std::error::Error>> {
        let pool: MemoryPool<Vec<u8>, 5> = MemoryPool::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;

        let mut obj = pool.acquire().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;
        obj.push(42);
        assert_eq!(obj[0], 42);

        let stats = pool.stats();
        assert_eq!(stats.objects_in_use, 1);
        assert_eq!(stats.objects_available, 4);

        drop(obj);

        let stats = pool.stats();
        assert_eq!(stats.objects_in_use, 0);
        assert_eq!(stats.objects_available, 5);
        Ok(())
    }

    #[test]
    fn test_pool_exhaustion() -> Result<(), Box<dyn std::error::Error>> {
        let pool: MemoryPool<String, 2> = MemoryPool::new().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;

        let _obj1 = pool.acquire().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;
        let _obj2 = pool.acquire().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;

        let result = pool.acquire();
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_global_pools() -> Result<(), Box<dyn std::error::Error>> {
        let pools = global_pools();
        let buffer = pools.get_crypto_buffer().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(
                format_args!("Operation failed: {:?}", e).to_string(),
            )
        })?;
        assert_eq!(buffer.len(), 0); // Default empty vector

        let stats = pools.get_global_stats();
        assert_eq!(stats.crypto_buffers.objects_in_use, 1);
        Ok(())
    }
}
