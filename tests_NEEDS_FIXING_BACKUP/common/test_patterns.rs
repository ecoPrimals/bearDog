use beardog_errors::BearDogError;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};

pub async fn execute_test_with_context<F, Fut, T>(
    test_name: &str,
    test_fn: F,
) -> Result<T, BearDogError>
where
    F: FnOnce(std::future::Future<Output = Result<T, BearDogError>>,
{
    info!("🧪 Starting test: {}", test_name);
    let start_time = Instant::now();

    match test_fn() {
        Ok(result) => {
            let duration = start_time.elapsed();
            info!("✅ Test passed: {} (took {:?})", test_name, duration);
            Ok({} after {:?}: {}", test_name, duration, e);
            Err(Result<T, E>,
    adapter_name: &str,
) -> Result<T, BearDogError>
where
    E: std::fmt::Debug,
{
    adapter_result.map_err({:?}", adapter_name, e);
        BearDogError::Initialization {
            message: format!("Failed to create {} adapter: {:?}", adapter_name, e),
        }
    })
}

pub async fn test_hsm_operation<F, Fut, T>(
    operation_name: &str,
    platform: &str,
    operation: F,
) -> Result<T, BearDogError>
where
    F: FnOnce(std::future::Future<Output = Result<T, BearDogError>>,
{
    info!("🔐 Testing {} on platform: {}", operation_name, platform);

    let result = operation({}", operation_name, platform, e),
    }

    result
}

pub async fn test_across_platforms<F, Fut>(
    platforms: &[&str],
    test_name: &str,
    test_fn: F,
) -> Result<(), BearDogError>
where
    F: Fn(&str) -> Fut,
    Fut: std::future::Future<Output = Result<(), BearDogError>>,
{
    for platform in platforms {
        test_hsm_operation(test_name, platform, || test_fn(platform))?;
    }
    Ok(())
}

pub async fn setup_test_harness(test_name: &str) -> Result<TestHarnessContext, BearDogError> {
    info!("⚙️ Setting up test harness for: {}", test_name);

    let config = beardog::config::BearDogConfig::default();
    let core = std::sync::Arc::new(beardog::core::BearDogCore::new(config)?);

    Ok(TestHarnessContext {
        test_name: test_name.to_string(),
        core,
        start_time: Instant::now(String,
    pub core: std::sync::Arc<beardog::core::BearDogCore>,
    pub start_time: Instant,
}

impl TestHarnessContext {
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub async fn cleanup(self) -> Result<(), BearDogError> {
        debug!(
            "🧹 Cleaning up test harness for: {} (ran for {:?})",
            self.test_name,
            self.elapsed()
        );

        Ok(())
    }
}
