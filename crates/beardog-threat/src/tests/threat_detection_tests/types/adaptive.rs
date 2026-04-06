// SPDX-License-Identifier: AGPL-3.0-or-later

//! Adaptive Detection Test Types

use super::pattern::ThreatPattern;

#[allow(dead_code)]
pub struct AdaptiveDetector {
    patterns: Vec<ThreatPattern>,
    learning_enabled: bool,
    false_positive_count: usize,
    true_positive_count: usize,
    threshold: f64,
}

#[allow(dead_code)]
impl AdaptiveDetector {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            learning_enabled: true,
            false_positive_count: 0,
            true_positive_count: 0,
            threshold: 0.5,
        }
    }

    pub fn enable_learning(&mut self) {
        self.learning_enabled = true;
    }

    pub fn disable_learning(&mut self) {
        self.learning_enabled = false;
    }

    pub fn learn_pattern(&mut self, pattern: ThreatPattern) {
        if self.learning_enabled {
            self.patterns.push(pattern);
        }
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn report_false_positive(&mut self) {
        self.false_positive_count += 1;
    }

    pub fn report_true_positive(&mut self) {
        self.true_positive_count += 1;
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn adjust_threshold(&mut self) {
        let total = self.false_positive_count + self.true_positive_count;
        if total > 0 {
            let fp_rate = self.false_positive_count as f64 / total as f64;
            // Adjust threshold based on false positive rate
            if fp_rate > 0.2 {
                self.threshold += 0.1;
            } else if fp_rate < 0.05 {
                self.threshold -= 0.05;
            }
            self.threshold = self.threshold.clamp(0.1, 0.9);
        }
    }

    pub fn threshold(&self) -> f64 {
        self.threshold
    }

    pub fn set_threshold(&mut self, threshold: f64) {
        self.threshold = threshold.clamp(0.0, 1.0);
    }
}
