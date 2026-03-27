// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for AI optimization modules
//! Focus: ResourcePredictor, OptimizationHistory, and their interactions

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use super::history::OptimizationHistory;
use super::predictor::ResourcePredictor;
use super::types::{OptimizationAction, OptimizationType};
use crate::float_eq;
use std::collections::HashMap;

// ========== ResourcePredictor Tests ==========

#[test]
fn test_resource_predictor_creation() {
    let predictor = ResourcePredictor::new(100);
    assert!(predictor.is_ok());
}

#[test]
fn test_resource_predictor_zero_window_size() {
    let predictor = ResourcePredictor::new(0);
    assert!(predictor.is_err());
}

#[test]
fn test_resource_predictor_add_sample() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    let result = predictor.add_sample(50.0, 60.0, 10.0);
    assert!(result.is_ok());
}

#[test]
fn test_resource_predictor_multiple_samples() {
    let mut predictor = ResourcePredictor::new(5).expect("ResourcePredictor::new");

    for i in 1..=10 {
        let result = predictor.add_sample(f64::from(i), f64::from(i) * 2.0, f64::from(i) * 0.5);
        assert!(result.is_ok());
    }
}

#[test]
fn test_predict_cpu_usage_no_data() {
    let predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    let result = predictor.predict_cpu_usage();
    assert!(result.is_err());
}

#[test]
fn test_predict_cpu_usage_with_data() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");

    predictor.add_sample(50.0, 60.0, 10.0).expect("add_sample");
    predictor.add_sample(60.0, 70.0, 12.0).expect("add_sample");

    let prediction = predictor.predict_cpu_usage().expect("predict_cpu_usage");
    float_eq::f64(prediction, 55.0); // Average of 50 and 60
}

#[test]
fn test_predict_memory_usage_no_data() {
    let predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    let result = predictor.predict_memory_usage();
    assert!(result.is_err());
}

#[test]
fn test_predict_memory_usage_with_data() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");

    predictor.add_sample(50.0, 40.0, 10.0).expect("add_sample");
    predictor.add_sample(60.0, 60.0, 12.0).expect("add_sample");

    let prediction = predictor
        .predict_memory_usage()
        .expect("predict_memory_usage");
    float_eq::f64(prediction, 50.0); // Average of 40 and 60
}

#[test]
fn test_predict_network_latency_no_data() {
    let predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    let result = predictor.predict_network_latency();
    assert!(result.is_err());
}

#[test]
fn test_predict_network_latency_with_data() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");

    predictor.add_sample(50.0, 60.0, 10.0).expect("add_sample");
    predictor.add_sample(60.0, 70.0, 20.0).expect("add_sample");

    let prediction = predictor
        .predict_network_latency()
        .expect("predict_network_latency");
    float_eq::f64(prediction, 15.0); // Average of 10 and 20
}

#[test]
fn test_window_size_limiting() {
    let mut predictor = ResourcePredictor::new(3).expect("ResourcePredictor::new");

    // Add more samples than window size
    for i in 1..=5 {
        predictor
            .add_sample(f64::from(i) * 10.0, f64::from(i) * 20.0, f64::from(i))
            .expect("add_sample");
    }

    // Should only average the last 3 samples (30, 40, 50)
    let prediction = predictor.predict_cpu_usage().expect("predict_cpu_usage");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    float_eq::f64(prediction, 40.0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_trend_cpu() {
    let mut predictor = ResourcePredictor::new(20).expect("ResourcePredictor::new");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Add increasing trend with enough samples for proper comparison
    for i in 1..=20 {
        predictor
            .add_sample(f64::from(i) * 10.0, 50.0, 10.0)
            .expect("add_sample");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    let trend = predictor.get_trend("cpu").expect("get_trend cpu");
    // Trend should be positive (recent values higher than older)
    assert!(trend > 50.0); // Recent avg around 150-200, older avg around 10-100
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_trend_memory() {
    let mut predictor = ResourcePredictor::new(20).expect("ResourcePredictor::new");

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for i in 1..=20 {
        predictor
            .add_sample(50.0, f64::from(i) * 5.0, 10.0)
            .expect("add_sample");
    }

    let trend = predictor.get_trend("memory").expect("get_trend memory");
    assert!(trend > 20.0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_trend_network() {
    let mut predictor = ResourcePredictor::new(20).expect("ResourcePredictor::new");

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    for i in 1..=20 {
        predictor
            .add_sample(50.0, 60.0, f64::from(i) * 2.0)
            .expect("add_sample");
    }

    let trend = predictor.get_trend("network").expect("get_trend network");
    assert!(trend > 10.0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_trend_unknown_type() {
    let predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    let result = predictor.get_trend("unknown");
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert!(result.is_err());
}

#[test]
fn test_get_trend_insufficient_data() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");
    predictor.add_sample(50.0, 60.0, 10.0).expect("add_sample");

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let trend = predictor.get_trend("cpu").expect("get_trend cpu");
    float_eq::f64(trend, 0.0); // Not enough data for trend
}

#[test]
fn test_predictor_with_large_window() {
    let predictor = ResourcePredictor::new(1000);
    assert!(predictor.is_ok());
}

// ========== OptimizationHistory Tests ==========

#[test]
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
fn test_optimization_history_creation() {
    let history = OptimizationHistory::new(100);
    assert_eq!(history.get_total_actions(), 0);
}

#[test]
fn test_add_optimization_action() {
    let mut history = OptimizationHistory::new(10);

    let action = OptimizationAction {
        timestamp: 1000,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(12.0),
        success: Some(true),
    };

    history.add_action(action);
    assert_eq!(history.get_total_actions(), 1);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_multiple_actions() {
    let mut history = OptimizationHistory::new(10);

    for i in 0..5 {
        let action = OptimizationAction {
            timestamp: 1000 + i,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            action_type: OptimizationType::ThreadPool,
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            success: Some(true),
        };
        history.add_action(action);
    }

    assert_eq!(history.get_total_actions(), 5);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_history_size_limiting() {
    let mut history = OptimizationHistory::new(3);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal

    // Add more actions than max size
    for i in 0..5 {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        let action = OptimizationAction {
            timestamp: 1000 + i,
            action_type: OptimizationType::ThreadPool,
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            success: Some(true),
        };
        history.add_action(action);
    }

    // Should only keep the last 3
    assert_eq!(history.get_total_actions(), 3);
}
// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal

#[test]
fn test_get_successful_actions() {
    let mut history = OptimizationHistory::new(10);

    // Add successful action
    let success_action = OptimizationAction {
        timestamp: 1000,
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(12.0),
        success: Some(true),
    };
    history.add_action(success_action);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Add failed action
    let failed_action = OptimizationAction {
        timestamp: 1001,
        action_type: OptimizationType::Simd,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(2.0),
        success: Some(false),
    };
    history.add_action(failed_action);

    assert_eq!(history.get_successful_actions(), 1);
    assert_eq!(history.get_total_actions(), 2);
}

#[test]
fn test_get_average_improvement_no_data() {
    let history = OptimizationHistory::new(10);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    float_eq::f64(history.get_average_improvement(), 0.0);
}

#[test]
fn test_get_average_improvement_with_data() {
    let mut history = OptimizationHistory::new(10);

    let action1 = OptimizationAction {
        timestamp: 1000,
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(10.0),
        success: Some(true),
    };
    history.add_action(action1);

    let action2 = OptimizationAction {
        timestamp: 1001,
        action_type: OptimizationType::Simd,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(20.0),
        success: Some(true),
    };
    history.add_action(action2);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    float_eq::f64(history.get_average_improvement(), 15.0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_get_average_improvement_with_none_values() {
    let mut history = OptimizationHistory::new(10);

    let action1 = OptimizationAction {
        timestamp: 1000,
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(10.0),
        success: Some(true),
    };
    history.add_action(action1);

    let action2 = OptimizationAction {
        timestamp: 1001,
        action_type: OptimizationType::Simd,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: None, // Not measured yet
        success: None,
    };
    history.add_action(action2);

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    // Should only count action1
    float_eq::f64(history.get_average_improvement(), 10.0);
}

#[test]
fn test_get_recent_actions() {
    let mut history = OptimizationHistory::new(10);

    for i in 0..5 {
        let action = OptimizationAction {
            timestamp: 1000 + i,
            action_type: OptimizationType::ThreadPool,
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            success: Some(true),
        };
        history.add_action(action);
    }

    let recent = history.get_recent_actions(3);
    assert_eq!(recent.len(), 3);

    // Should be in reverse order (most recent first)
    assert_eq!(recent[0].timestamp, 1004);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(recent[1].timestamp, 1003);
    assert_eq!(recent[2].timestamp, 1002);
}

#[test]
fn test_get_recent_actions_more_than_available() {
    let mut history = OptimizationHistory::new(10);

    for i in 0..3 {
        let action = OptimizationAction {
            timestamp: 1000 + i,
            action_type: OptimizationType::ThreadPool,
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            success: Some(true),
        };
        history.add_action(action);
    }

    let recent = history.get_recent_actions(10);
    assert_eq!(recent.len(), 3); // Only 3 available
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_get_success_rate() {
    let mut history = OptimizationHistory::new(10);

    // Add 3 successful ThreadPool optimizations
    for i in 0..3 {
        let action = OptimizationAction {
            timestamp: 1000 + i,
            action_type: OptimizationType::ThreadPool,
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            success: Some(true),
        };
        history.add_action(action);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
    }

    // Add 1 failed ThreadPool optimization
    let failed_action = OptimizationAction {
        timestamp: 1003,
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(2.0),
        success: Some(false),
    };
    history.add_action(failed_action);

    // Success rate should be 3/4 = 0.75
    let rate = history.get_success_rate("ThreadPool");
    assert!(rate.is_some());
    float_eq::f64(rate.expect("ThreadPool success rate"), 0.75);
}

#[test]
fn test_get_success_rate_unknown_type() {
    let history = OptimizationHistory::new(10);
    let rate = history.get_success_rate("UnknownType");
    assert!(rate.is_none());
}

#[test]
fn test_get_recent_actions_empty() {
    let history = OptimizationHistory::new(10);
    let recent = history.get_recent_actions(5);
    assert_eq!(recent.len(), 0);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
}

#[test]
fn test_mixed_optimization_types() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    let mut history = OptimizationHistory::new(10);

    let types = [
        OptimizationType::ThreadPool,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        OptimizationType::Simd,
        OptimizationType::Memory,
        OptimizationType::Cache,
    ];

    for (i, opt_type) in types.iter().enumerate() {
        let action = OptimizationAction {
            timestamp: 1000 + i as u64,
            action_type: opt_type.clone(),
            parameters: HashMap::new(),
            expected_improvement: 10.0,
            actual_improvement: Some(12.0),
            success: Some(true),
        };
        history.add_action(action);
    }

    assert_eq!(history.get_total_actions(), 4);
    assert_eq!(history.get_successful_actions(), 4);
}

#[test]
fn test_optimization_history_with_large_capacity() {
    let history = OptimizationHistory::new(10000);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    assert_eq!(history.get_total_actions(), 0);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: core
// TEST_PRIORITY: normal
#[test]
fn test_predictor_accuracy_over_time() {
    let mut predictor = ResourcePredictor::new(10).expect("ResourcePredictor::new");

    // Add consistent data
    for _ in 0..5 {
        predictor.add_sample(50.0, 60.0, 10.0).expect("add_sample");
    }

    let cpu_pred = predictor.predict_cpu_usage().expect("predict_cpu_usage");
    let mem_pred = predictor
        .predict_memory_usage()
        .expect("predict_memory_usage");
    let net_pred = predictor
        .predict_network_latency()
        .expect("predict_network_latency");

    float_eq::f64(cpu_pred, 50.0);
    float_eq::f64(mem_pred, 60.0);
    float_eq::f64(net_pred, 10.0);
}
