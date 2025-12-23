// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use std::collections::VecDeque;

pub struct ResourcePredictor {
    cpu_history: VecDeque<f64>,
    memory_history: VecDeque<f64>,
    network_history: VecDeque<f64>,
    prediction_window: usize,
}

impl ResourcePredictor {
    /// Creates a new instance
    pub fn new(window_size: usize) -> Result<Self, BearDogError> {
        if window_size == 0 {
            return Err(BearDogError::invalid_input(
                "Window size must be greater than 0",
            ));
        }

        Ok(Self {
            cpu_history: VecDeque::with_capacity(window_size),
            memory_history: VecDeque::with_capacity(window_size),
            network_history: VecDeque::with_capacity(window_size),
            prediction_window: window_size,
        })
    }

    pub fn add_sample(&mut self, cpu: f64, memory: f64, network: f64) -> Result<(), BearDogError> {
        // Add new samples
        self.cpu_history.push_back(cpu);
        self.memory_history.push_back(memory);
        self.network_history.push_back(network);

        // Remove old samples if we exceed window size
        if self.cpu_history.len() > self.prediction_window {
            self.cpu_history.pop_front();
        }
        if self.memory_history.len() > self.prediction_window {
            self.memory_history.pop_front();
        }
        if self.network_history.len() > self.prediction_window {
            self.network_history.pop_front();
        }

        Ok(())
    }

    pub fn predict_cpu_usage(&self) -> Result<f64, BearDogError> {
        if self.cpu_history.is_empty() {
            return Err(BearDogError::invalid_input("No CPU history available"));
        }

        // Simple moving average prediction
        let sum: f64 = self.cpu_history.iter().sum();
        Ok(sum / self.cpu_history.len() as f64)
    }

    pub fn predict_memory_usage(&self) -> Result<f64, BearDogError> {
        if self.memory_history.is_empty() {
            return Err(BearDogError::invalid_input("No memory history available"));
        }

        let sum: f64 = self.memory_history.iter().sum();
        Ok(sum / self.memory_history.len() as f64)
    }

    pub fn predict_network_latency(&self) -> Result<f64, BearDogError> {
        if self.network_history.is_empty() {
            return Err(BearDogError::invalid_input("No network history available"));
        }

        let sum: f64 = self.network_history.iter().sum();
        Ok(sum / self.network_history.len() as f64)
    }

    /// Gets trend
    /// Gets trend
    pub fn get_trend(&self, resource_type: &str) -> Result<f64, BearDogError> {
        let history = match resource_type {
            "cpu" => &self.cpu_history,
            "memory" => &self.memory_history,
            "network" => &self.network_history,
            _ => {
                return Err(BearDogError::invalid_input(&format!(
                    "Unknown resource type: {resource_type}"
                )))
            }
        };

        if history.len() < 2 {
            return Ok(0.0); // No trend available
        }

        let recent = history.iter().rev().take(10).collect::<Vec<_>>();
        let older = history.iter().take(10).collect::<Vec<_>>();

        if recent.is_empty() || older.is_empty() {
            return Ok(0.0);
        }

        let recent_avg: f64 = recent.iter().copied().sum::<f64>() / recent.len() as f64;
        let older_avg: f64 = older.iter().copied().sum::<f64>() / older.len() as f64;

        Ok(recent_avg - older_avg)
    }
}
