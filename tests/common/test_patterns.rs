

use beardog_errors::{BearDogError, BearDogResult};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

pub async fn execute_test_with_context<F, Fut, T>(
    test_name: &str,
    test_fn: F,
) -> BearDogResult<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = BearDogResult<T>>,
{
    info!("🧪 Starting test: {}", test_name);
    let start_time = Instant::now();
    
    match test_fn().await {
        Ok(result) => {
            let duration = start_time.elapsed();
            info!("✅ Test passed: {} (took {:?})", test_name, duration);
            Ok(result)
        }
        Err(e) => {
            let duration = start_time.elapsed();
            error!("❌ Test failed: {} after {:?}: {}", test_name, duration, e);
            Err(e)
        }
    }
}

pub fn create_adapter_safely<T, E>(
    adapter_result: Result<T, E>,
    adapter_name: &str,
) -> BearDogResult<T>
where
    E: std::fmt::Debug,
{
    adapter_result.map_err(|e| {
        error!("Failed to create {} adapter: {:?}", adapter_name, e);
        BearDogError::Initialization {
            message: format_args!("Failed to create {} adapter: {:?}", adapter_name, e).to_string(),
        }
    })
}

pub async fn test_hsm_operation<F, Fut, T>(
    operation_name: &str,
    platform: &str,
    operation: F,
) -> BearDogResult<T>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = BearDogResult<T>>,
{
    info!("🔐 Testing {} on platform: {}", operation_name, platform);
    
    let result = operation().await;
    
    match &result {
        Ok(_) => debug!("{} test completed successfully for {}", operation_name, platform),
        Err(e) => warn!("{} test failed for {}: {}", operation_name, platform, e),
    }
    
    result
}

pub async fn test_across_platforms<F, Fut>(
    platforms: &[&str],
    test_name: &str,
    test_fn: F,
) -> BearDogResult<()>
where
    F: Fn(&str) -> Fut,
    Fut: std::future::Future<Output = BearDogResult<()>>,
{
    for platform in platforms {
        test_hsm_operation(test_name, platform, || test_fn(platform)).await?;
    }
    Ok(())
}

pub async fn setup_test_harness(test_name: &str) -> BearDogResult<TestHarnessContext> {
    info!("⚙️ Setting up test harness for: {}", test_name);
    
    let config = beardog::config::BearDogConfig::default();
    let core = std::sync::Arc::new(beardog::core::BearDogCore::new(config).await?);
    
    Ok(TestHarnessContext {
        test_name: test_name.to_string(),
        core,
        start_time: Instant::now(),
    })
}

pub struct TestHarnessContext {
    pub test_name: String,
    pub core: std::sync::Arc<beardog::core::BearDogCore>,
    pub start_time: Instant,
}

impl TestHarnessContext {
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub async fn cleanup(self) -> BearDogResult<()> {
        debug!("🧹 Cleaning up test harness for: {} (ran for {:?})", 
               self.test_name, self.elapsed());

        Ok(())
    }
} 