// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::types::OptimizationAction;
use std::collections::{HashMap, VecDeque};

pub struct OptimizationHistory {
    optimization_actions: VecDeque<OptimizationAction>,
    success_rates: HashMap<String, f64>,
    max_history_size: usize,
}

impl OptimizationHistory {
    /// Creates a new instance
    pub fn new(max_size: usize) -> Self {
        Self {
            optimization_actions: VecDeque::with_capacity(max_size),
            success_rates: HashMap::new(),
            max_history_size: max_size,
        }
    }

    pub fn add_action(&mut self, action: OptimizationAction) {
        // Add new action
        self.optimization_actions.push_back(action);

        // Remove old actions if we exceed max size
        if self.optimization_actions.len() > self.max_history_size {
            self.optimization_actions.pop_front();
        }

        // Update success rates
        self.update_success_rates();
    }

    /// Gets total_actions
    /// Gets total_actions
    pub fn get_total_actions(&self) -> usize {
        self.optimization_actions.len()
    }

    /// Gets successful_actions
    /// Gets successful_actions
    pub fn get_successful_actions(&self) -> usize {
        self.optimization_actions
            .iter()
            .filter(|action| action.success == Some(true))
            .count()
    }

    /// Gets average_improvement
    /// Gets average_improvement
    pub fn get_average_improvement(&self) -> f64 {
        let improvements: Vec<f64> = self
            .optimization_actions
            .iter()
            .filter_map(|action| action.actual_improvement)
            .collect();

        if improvements.is_empty() {
            0.0
        } else {
            improvements.iter().sum::<f64>() / improvements.len() as f64
        }
    }

    /// Gets success_rate
    /// Gets success_rate
    pub fn get_success_rate(&self, optimization_type: &str) -> Option<f64> {
        self.success_rates.get(optimization_type).copied()
    }

    /// Updates success_rates
    fn update_success_rates(&mut self) {
        // Clear existing rates
        self.success_rates.clear();

        // Group actions by type
        let mut type_counts: HashMap<String, (usize, usize)> = HashMap::new(); // (total, successful)

        for action in &self.optimization_actions {
            let type_name = format!("{:?}", action.action_type);
            let entry = type_counts.entry(type_name).or_insert((0, 0));
            entry.0 += 1; // Total count

            if action.success == Some(true) {
                entry.1 += 1; // Successful count
            }
        }

        // Calculate success rates
        for (type_name, (total, successful)) in type_counts {
            if total > 0 {
                let rate = successful as f64 / total as f64;
                self.success_rates.insert(type_name, rate);
            }
        }
    }

    /// Gets recent_actions
    /// Gets recent_actions
    pub fn get_recent_actions(&self, count: usize) -> Vec<&OptimizationAction> {
        self.optimization_actions.iter().rev().take(count).collect()
    }
}
