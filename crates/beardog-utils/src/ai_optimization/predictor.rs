// SPDX-License-Identifier: AGPL-3.0-or-later

//! Moving-window averages over CPU, memory, and network telemetry.

use beardog_errors::BearDogError;
use std::collections::VecDeque;

/// Fixed-capacity deques feeding simple mean-based forecasts.
pub struct ResourcePredictor {
    cpu_history: VecDeque<f64>,
    memory_history: VecDeque<f64>,
    network_history: VecDeque<f64>,
    prediction_window: usize,
}

impl ResourcePredictor {
    /// Creates a new instance
    ///
    /// # Errors
    ///
    /// Returns an error if `window_size` is zero.
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

    /// Pushes a correlated triple of readings, trimming to `prediction_window`.
    ///
    /// # Errors
    ///
    /// Currently always returns `Ok(())`.
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

    /// Mean of the CPU deque.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no CPU history yet.
    pub fn predict_cpu_usage(&self) -> Result<f64, BearDogError> {
        if self.cpu_history.is_empty() {
            return Err(BearDogError::invalid_input("No CPU history available"));
        }

        // Simple moving average prediction
        let sum: f64 = self.cpu_history.iter().sum();
        #[expect(
            clippy::cast_precision_loss,
            reason = "deque length as divisor for moving average"
        )]
        let n = self.cpu_history.len() as f64;
        Ok(sum / n)
    }

    /// Mean of the memory deque.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no memory history yet.
    pub fn predict_memory_usage(&self) -> Result<f64, BearDogError> {
        if self.memory_history.is_empty() {
            return Err(BearDogError::invalid_input("No memory history available"));
        }

        let sum: f64 = self.memory_history.iter().sum();
        #[expect(
            clippy::cast_precision_loss,
            reason = "deque length as divisor for moving average"
        )]
        let n = self.memory_history.len() as f64;
        Ok(sum / n)
    }

    /// Mean of the network deque.
    ///
    /// # Errors
    ///
    /// Returns an error if there is no network history yet.
    pub fn predict_network_latency(&self) -> Result<f64, BearDogError> {
        if self.network_history.is_empty() {
            return Err(BearDogError::invalid_input("No network history available"));
        }

        let sum: f64 = self.network_history.iter().sum();
        #[expect(
            clippy::cast_precision_loss,
            reason = "deque length as divisor for moving average"
        )]
        let n = self.network_history.len() as f64;
        Ok(sum / n)
    }

    /// Linear slope estimate over `"cpu"`, `"memory"`, or `"network"` history.
    ///
    /// # Errors
    ///
    /// Returns an error if `resource_type` is not one of the supported strings.
    pub fn get_trend(&self, resource_type: &str) -> Result<f64, BearDogError> {
        let history = match resource_type {
            "cpu" => &self.cpu_history,
            "memory" => &self.memory_history,
            "network" => &self.network_history,
            _ => {
                return Err(BearDogError::invalid_input(&format!(
                    "Unknown resource type: {resource_type}"
                )));
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

        #[expect(
            clippy::cast_precision_loss,
            reason = "recent window size as trend divisor"
        )]
        let recent_n = recent.len() as f64;
        #[expect(
            clippy::cast_precision_loss,
            reason = "older window size as trend divisor"
        )]
        let older_n = older.len() as f64;
        let recent_avg: f64 = recent.iter().copied().sum::<f64>() / recent_n;
        let older_avg: f64 = older.iter().copied().sum::<f64>() / older_n;

        Ok(recent_avg - older_avg)
    }
}
