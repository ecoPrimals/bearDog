// SPDX-License-Identifier: AGPL-3.0-or-later
//! Workflow engine, retry, and adapter chain configuration tests.

#![cfg(test)]

use std::time::Duration;

use crate::canonical::config::domains::adapter::chain::{
    ChainConfig, RetryConfig as AdapterRetryConfig, StepConfig,
};
use crate::canonical::config::domains::workflow::engine::{
    QueueConfig, TimeoutConfig, WorkflowEngineConfig,
};
use crate::canonical::config::domains::workflow::retry::RetryConfig as WorkflowRetryConfig;
use crate::canonical::traits::RetryStrategy;
use crate::canonical::traits::TimeoutPolicy;

use super::common::assert_serde_json_roundtrip;

#[test]
#[expect(
    clippy::cast_possible_truncation,
    reason = "max_attempts fits u32 for RetryStrategy::max_attempts contract in test"
)]
fn workflow_retry_config_default_trait_and_serde() {
    let c = WorkflowRetryConfig::default();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize workflow retry");
    let rt: WorkflowRetryConfig = serde_json::from_str(&json).expect("deserialize workflow retry");
    assert_eq!(c.max_attempts, rt.max_attempts);

    assert_eq!(RetryStrategy::max_attempts(&c), c.max_attempts as u32);
    let d0 = RetryStrategy::delay_for_attempt(&c, 0);
    assert!(d0 > Duration::ZERO || c.max_delay.is_zero());
    assert!(RetryStrategy::should_retry_error(
        &c,
        &std::io::Error::other("x")
    ));
    assert!(!RetryStrategy::is_limit_reached(&c, 0));
    let total = RetryStrategy::total_delay(&c, 2);
    assert!(total >= Duration::from_millis(0));
}

#[test]
fn workflow_engine_config_queue_timeout_serde_and_timeout_policy() {
    let cfg = WorkflowEngineConfig {
        engine_type: std::sync::Arc::from("custom"),
        worker_pool_size: 4,
        queue: QueueConfig {
            queue_type: std::sync::Arc::from("memory"),
            capacity: 10,
            message_ttl: Duration::from_secs(60),
            dead_letter_queue: Some("dlq".to_string()),
        },
        timeouts: TimeoutConfig {
            default: Duration::from_secs(20),
            maximum: Duration::from_secs(200),
            connection: Duration::from_secs(5),
            read: Duration::from_secs(15),
        },
    };
    assert_serde_json_roundtrip(&cfg);
    let json = serde_json::to_string(&cfg).expect("serialize workflow engine");
    let rt: WorkflowEngineConfig =
        serde_json::from_str(&json).expect("deserialize workflow engine");
    assert_eq!(cfg.worker_pool_size, rt.worker_pool_size);

    let t = &cfg.timeouts;
    assert_eq!(t.connection_timeout(), t.connection);
    assert_eq!(t.operation_timeout("read"), t.read);
    assert_eq!(t.operation_timeout("connect"), t.connection);
    assert_eq!(t.operation_timeout("other"), t.default);
    assert!(t.should_timeout(Duration::from_secs(100), "read"));
    assert_eq!(t.global_timeout(), Some(t.maximum));
    assert_eq!(t.read_timeout(), t.read);
    assert_eq!(t.write_timeout(), t.default);
    assert!(t.idle_timeout().is_none());
    assert_eq!(
        t.remaining_time(Duration::from_secs(1), "read"),
        t.read.saturating_sub(Duration::from_secs(1))
    );
    t.validate().expect("valid timeouts");

    assert!(t.is_production_ready());
    let mut bad = cfg.clone();
    bad.timeouts.connection = Duration::ZERO;
    assert!(!bad.timeouts.is_production_ready());
}

#[test]
fn adapter_chain_config_and_retry_strategy() {
    let c = ChainConfig::with_defaults();
    assert_serde_json_roundtrip(&c);
    let json = serde_json::to_string(&c).expect("serialize chain");
    let rt: ChainConfig = serde_json::from_str(&json).expect("deserialize chain");
    assert_eq!(c.max_chain_length, rt.max_chain_length);

    let r = AdapterRetryConfig::default();
    assert_eq!(RetryStrategy::max_attempts(&r), r.max_attempts);
    let d = RetryStrategy::delay_for_attempt(&r, 1);
    assert!(d <= r.max_delay || r.max_delay.is_zero());

    let mut linear = r.clone();
    linear.exponential_backoff = false;
    let d0 = RetryStrategy::delay_for_attempt(&linear, 0);
    assert_eq!(d0, linear.initial_delay.min(linear.max_delay));

    assert!(RetryStrategy::is_limit_reached(&r, r.max_attempts));
    assert!(RetryStrategy::total_delay(&r, 2) >= Duration::ZERO);
}

#[test]
fn step_config_default_serde() {
    let s = StepConfig::default();
    assert_serde_json_roundtrip(&s);
}

#[test]
fn chain_config_default_matches_with_defaults() {
    let a = ChainConfig::default();
    let b = ChainConfig::with_defaults();
    assert_eq!(a.max_chain_length, b.max_chain_length);
}

#[test]
fn workflow_engine_config_from_source_uses_engine_type() {
    use crate::canonical::config::source::EnvConfigSource;
    let src = EnvConfigSource::new();
    let w = WorkflowEngineConfig::from_source(&src);
    assert!(!w.engine_type.as_ref().is_empty());
}

#[test]
fn timeout_config_validate_errors_on_zero_connection() {
    let t = TimeoutConfig {
        default: Duration::from_secs(10),
        maximum: Duration::from_secs(100),
        connection: Duration::ZERO,
        read: Duration::from_secs(5),
    };
    t.validate().expect_err("zero connection timeout");
}

#[test]
fn timeout_config_validate_errors_when_max_lt_default() {
    let t = TimeoutConfig {
        default: Duration::from_secs(50),
        maximum: Duration::from_secs(10),
        connection: Duration::from_secs(5),
        read: Duration::from_secs(5),
    };
    t.validate().expect_err("max < default");
}

#[test]
fn queue_config_default_serde() {
    let q = QueueConfig::default();
    assert_serde_json_roundtrip(&q);
}

#[test]
fn workflow_retry_total_delay_increases_with_attempts() {
    let c = WorkflowRetryConfig {
        max_attempts: 3,
        initial_delay: Duration::from_millis(10),
        backoff_multiplier: 2.0,
        max_delay: Duration::from_secs(60),
    };
    let t0 = RetryStrategy::total_delay(&c, 0);
    let t2 = RetryStrategy::total_delay(&c, 2);
    assert!(t2 >= t0);
}

#[test]
fn adapter_retry_limit_reached_edge() {
    let r = AdapterRetryConfig {
        max_attempts: 1,
        initial_delay: Duration::from_millis(1),
        max_delay: Duration::from_secs(1),
        backoff_multiplier: 2.0,
        exponential_backoff: true,
        jitter_factor: 0.0,
    };
    assert!(RetryStrategy::is_limit_reached(&r, 1));
}
