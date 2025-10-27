//! Comprehensive tests for AI optimization modules
//! Focus: ResourcePredictor, OptimizationHistory, and their interactions

use super::history::OptimizationHistory;
use super::predictor::ResourcePredictor;
use super::types::{OptimizationAction, OptimizationType};
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
    let mut predictor = ResourcePredictor::new(10).unwrap();
    let result = predictor.add_sample(50.0, 60.0, 10.0);
    assert!(result.is_ok());
}

#[test]
fn test_resource_predictor_multiple_samples() {
    let mut predictor = ResourcePredictor::new(5).unwrap();

    for i in 1..=10 {
        let result = predictor.add_sample(i as f64, i as f64 * 2.0, i as f64 * 0.5);
        assert!(result.is_ok());
    }
}

#[test]
fn test_predict_cpu_usage_no_data() {
    let predictor = ResourcePredictor::new(10).unwrap();
    let result = predictor.predict_cpu_usage();
    assert!(result.is_err());
}

#[test]
fn test_predict_cpu_usage_with_data() {
    let mut predictor = ResourcePredictor::new(10).unwrap();

    predictor.add_sample(50.0, 60.0, 10.0).unwrap();
    predictor.add_sample(60.0, 70.0, 12.0).unwrap();

    let prediction = predictor.predict_cpu_usage().unwrap();
    assert_eq!(prediction, 55.0); // Average of 50 and 60
}

#[test]
fn test_predict_memory_usage_no_data() {
    let predictor = ResourcePredictor::new(10).unwrap();
    let result = predictor.predict_memory_usage();
    assert!(result.is_err());
}

#[test]
fn test_predict_memory_usage_with_data() {
    let mut predictor = ResourcePredictor::new(10).unwrap();

    predictor.add_sample(50.0, 40.0, 10.0).unwrap();
    predictor.add_sample(60.0, 60.0, 12.0).unwrap();

    let prediction = predictor.predict_memory_usage().unwrap();
    assert_eq!(prediction, 50.0); // Average of 40 and 60
}

#[test]
fn test_predict_network_latency_no_data() {
    let predictor = ResourcePredictor::new(10).unwrap();
    let result = predictor.predict_network_latency();
    assert!(result.is_err());
}

#[test]
fn test_predict_network_latency_with_data() {
    let mut predictor = ResourcePredictor::new(10).unwrap();

    predictor.add_sample(50.0, 60.0, 10.0).unwrap();
    predictor.add_sample(60.0, 70.0, 20.0).unwrap();

    let prediction = predictor.predict_network_latency().unwrap();
    assert_eq!(prediction, 15.0); // Average of 10 and 20
}

#[test]
fn test_window_size_limiting() {
    let mut predictor = ResourcePredictor::new(3).unwrap();

    // Add more samples than window size
    for i in 1..=5 {
        predictor
            .add_sample(i as f64 * 10.0, i as f64 * 20.0, i as f64)
            .unwrap();
    }

    // Should only average the last 3 samples (30, 40, 50)
    let prediction = predictor.predict_cpu_usage().unwrap();
    assert_eq!(prediction, 40.0);
}

#[test]
fn test_get_trend_cpu() {
    let mut predictor = ResourcePredictor::new(20).unwrap();

    // Add increasing trend with enough samples for proper comparison
    for i in 1..=20 {
        predictor.add_sample(i as f64 * 10.0, 50.0, 10.0).unwrap();
    }

    let trend = predictor.get_trend("cpu").unwrap();
    // Trend should be positive (recent values higher than older)
    assert!(trend > 50.0); // Recent avg around 150-200, older avg around 10-100
}

#[test]
fn test_get_trend_memory() {
    let mut predictor = ResourcePredictor::new(20).unwrap();

    for i in 1..=20 {
        predictor.add_sample(50.0, i as f64 * 5.0, 10.0).unwrap();
    }

    let trend = predictor.get_trend("memory").unwrap();
    assert!(trend > 20.0);
}

#[test]
fn test_get_trend_network() {
    let mut predictor = ResourcePredictor::new(20).unwrap();

    for i in 1..=20 {
        predictor.add_sample(50.0, 60.0, i as f64 * 2.0).unwrap();
    }

    let trend = predictor.get_trend("network").unwrap();
    assert!(trend > 10.0);
}

#[test]
fn test_get_trend_unknown_type() {
    let predictor = ResourcePredictor::new(10).unwrap();
    let result = predictor.get_trend("unknown");
    assert!(result.is_err());
}

#[test]
fn test_get_trend_insufficient_data() {
    let mut predictor = ResourcePredictor::new(10).unwrap();
    predictor.add_sample(50.0, 60.0, 10.0).unwrap();

    let trend = predictor.get_trend("cpu").unwrap();
    assert_eq!(trend, 0.0); // Not enough data for trend
}

#[test]
fn test_predictor_with_large_window() {
    let predictor = ResourcePredictor::new(1000);
    assert!(predictor.is_ok());
}

// ========== OptimizationHistory Tests ==========

#[test]
fn test_optimization_history_creation() {
    let history = OptimizationHistory::new(100);
    assert_eq!(history.get_total_actions(), 0);
}

#[test]
fn test_add_optimization_action() {
    let mut history = OptimizationHistory::new(10);

    let action = OptimizationAction {
        timestamp: 1000,
        action_type: OptimizationType::ThreadPool,
        parameters: HashMap::new(),
        expected_improvement: 10.0,
        actual_improvement: Some(12.0),
        success: Some(true),
    };

    history.add_action(action);
    assert_eq!(history.get_total_actions(), 1);
}

#[test]
fn test_multiple_actions() {
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

    assert_eq!(history.get_total_actions(), 5);
}

#[test]
fn test_history_size_limiting() {
    let mut history = OptimizationHistory::new(3);

    // Add more actions than max size
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

    // Should only keep the last 3
    assert_eq!(history.get_total_actions(), 3);
}

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
    assert_eq!(history.get_average_improvement(), 0.0);
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

    assert_eq!(history.get_average_improvement(), 15.0);
}

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

    // Should only count action1
    assert_eq!(history.get_average_improvement(), 10.0);
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
    assert_eq!(rate.unwrap(), 0.75);
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
}

#[test]
fn test_mixed_optimization_types() {
    let mut history = OptimizationHistory::new(10);

    let types = [
        OptimizationType::ThreadPool,
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
    assert_eq!(history.get_total_actions(), 0);
}

#[test]
fn test_predictor_accuracy_over_time() {
    let mut predictor = ResourcePredictor::new(10).unwrap();

    // Add consistent data
    for _ in 0..5 {
        predictor.add_sample(50.0, 60.0, 10.0).unwrap();
    }

    let cpu_pred = predictor.predict_cpu_usage().unwrap();
    let mem_pred = predictor.predict_memory_usage().unwrap();
    let net_pred = predictor.predict_network_latency().unwrap();

    assert_eq!(cpu_pred, 50.0);
    assert_eq!(mem_pred, 60.0);
    assert_eq!(net_pred, 10.0);
}
