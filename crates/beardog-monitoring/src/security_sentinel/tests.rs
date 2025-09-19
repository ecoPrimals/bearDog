

use super::*;
use beardog_errors::BearDogError;
use std::sync::Arc;

#[tokio::test]
fn test_performance_sentinel_initialization(1000,
        max_error_rate: 0.05,
        max_memory_mb: 512.0,
        max_cpu_percent: 80.0,
        min_success_rate: 95.0,
    };
    let alert_manager = Arc::new(AlertManager::new());
    let _performance_sentinel = PerformanceSentinel::new(thresholds, alert_manager)?;

    Ok(())
}
fn test_security_sentinel_comprehensive() -> Result<(), BearDogError> {
    use super::super::SecuritySentinel;
    let _sentinel = SecuritySentinel::new();

