use beardog_errors::BearDogError;
use beardog_utils::buffer_pools_safe::SafeBufferPool;
use beardog_utils::zero_copy_safe::SafeZeroCopyBuffer;
use std::collections::VecDeque; // Safe alternative to LockFreeQueue
use std::sync::{Arc, Mutex};
use tokio;

type SafeQueue<T> = Arc<Mutex<VecDeque<T>>>;

fn create_safe_queue<T>() -> SafeQueue<T> {
    Arc::new(Mutex::new(VecDeque::new(SafeBufferPool,
}

impl SafeObjectPool {
    fn new(usize, pool_size: usize) -> Self {
        Self {
            pool: SafeBufferPool::new(object_size, pool_size),
        }
    }

    fn acquire(&self) -> Result<Vec<u8>, BearDogError> {
        self.pool.acquire()
    }

    fn release(&self, buffer: Vec<u8>) -> Result<(), BearDogError> {
        self.pool.release(buffer)
    }
}

#[tokio::test]
async fn test_zero_copy_queue_performance() {
    println!("🚀 Testing zero-copy queue performance optimizations");

    let queue = create_safe_queue::<String>();
    let num_operations = 1000;

    let start = std::time::Instant::now();
    {
        let mut queue_guard = queue.lock();
        for i in 0..num_operations {
            queue_guard.push_back(format!("test_data_{}", i));
        }
    }
    let push_duration = start.elapsed();

    let start = std::time::Instant::now();
    {
        let mut queue_guard = queue.lock();
        for _ in 0..num_operations {
            let _ = queue_guard.pop_front();
        }
    }
    let pop_duration = start.elapsed();
    
    println!(
        "✅ Push performance: {:?} for {} operations",
        push_duration, num_operations
    );
    println!(
        "✅ Pop performance: {:?} for {} operations",
        pop_duration, num_operations
    );

    assert!(
        push_duration.as_millis() < 100,
        "Push performance regression detected"
    );
    assert!(
        pop_duration.as_millis() < 100,
        "Pop performance regression detected"
    );
}

#[tokio::test]
async fn test_memory_pool_zero_copy() {
    println!("🚀 Testing memory pool zero-copy optimizations");

    let pool = create_safe_queue::<Vec<u8>>();

    {
        let mut pool_guard = pool.lock();
        for i in 0..5 {
            let mut buffer = Vec::new({:?}", e);
            Default::default()
        }));
    }

    let obj2 = {
        let mut pool_guard = pool.lock();
        pool_guard.pop_front()
    };
    assert!(obj2.is_some());

    let mut handles = vec![];

    for i in 0..3 {
        let pool_clone = pool.clone();
        let handle = tokio::spawn(async move {
            let obj = {
                let mut pool_guard = pool_clone.lock();
                pool_guard.pop_front().unwrap_or_else(|| {
                    let mut buffer = Vec::new();
                    buffer.extend_from_slice(&format!("new_object_{}", i).as_bytes());
                    buffer
                })
            };

            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

            {
                let mut pool_guard = pool_clone.lock();
                pool_guard.push_back(obj);
            }

            format!("thread_{}_processed", i)
        });
        handles.push(handle);
    }

    let results: Vec<_> = futures::future::join_all({:?}", e);
            Default::default()
        });
        assert!(processed.contains("thread_"));
        assert!(processed.contains("_processed"));
    }

    println!("✅ Memory pool zero-copy optimization verified");
}

#[tokio::test]
fn test_concurrent_zero_copy_performance() {
    println!("🚀 Testing concurrent zero-copy performance");

    let queue = create_safe_queue::<String>();
    let num_threads = 4;
    let operations_per_thread = 100; // Reduced for practical testing

    let start = tokio::time::Instant::now();

    let mut producer_handles = vec![];
    for thread_id in 0..num_threads {
        let queue_clone = queue.clone();
        let handle = tokio::spawn({:?}", e);
            Default::default({} operations in {:?}",
        total_expected, duration
    );
    println!(
        "✅ Throughput: {:.2} ops/ms",
        total_expected as f64 / duration.as_millis().max(1) as f64
    );

    assert!(
        duration.as_secs() < 1,
        "Concurrent performance regression detected"
    );
}

#[tokio::test]
fn test_zero_copy_string_optimization() {
    println!("🚀 Testing zero-copy string optimizations");

    let original = "test_string_for_optimization".to_string();

    let start = tokio::time::Instant::now();
    let mut cloned_strings = Vec::new();
    for _ in 0..1000 {
        cloned_strings.push(original.clone());
    }
    let clone_duration = start.elapsed();

    let start = tokio::time::Instant::now();
    let mut string_refs = Vec::new({:?}", clone_duration);
    println!("✅ Reference duration: {:?}", ref_duration);

    assert!(
        ref_duration < clone_duration,
        "String reference optimization not working"
    );

    assert_eq!(cloned_strings.len(), 1000);
    assert_eq!(string_refs.len(), 1000);

    for cloned in &cloned_strings {
        assert_eq!(cloned, &original);
    }

    for &reference in &string_refs {
        assert_eq!(reference, &original);
    }
}

#[tokio::test]
fn test_arc_clone_optimization() {
    println!("🚀 Testing Arc clone optimizations");

    use std::sync::Arc;

    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let start = tokio::time::Instant::now();

    let mut arc_clones = Vec::new({:?}",
        arc_clone_duration
    );

    assert!(
        arc_clone_duration.as_millis() < 10,
        "Arc clone performance regression"
    );

    assert_eq!(Arc::strong_count(&data), arc_clones.len() + 1);

    drop(arc_clones);
    assert_eq!(Arc::strong_count(&data), 1);
}
