//! Pattern Refinement Test Types

use super::pattern::ThreatPattern;
use std::collections::HashMap;

pub struct PatternRefiner {
    patterns: Vec<ThreatPattern>,
    false_positive_records: HashMap<String, Vec<String>>,
    disabled_patterns: Vec<String>,
}

impl PatternRefiner {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            false_positive_records: HashMap::new(),
            disabled_patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: ThreatPattern) {
        self.patterns.push(pattern);
    }

    pub fn refine_patterns(&mut self) {
        // Simple refinement: remove duplicates
        self.patterns.dedup_by(|a, b| a.id() == b.id());
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn record_false_positive(&mut self, pattern_id: &str, context: &str) {
        self.false_positive_records
            .entry(pattern_id.to_string())
            .or_default()
            .push(context.to_string());
    }

    pub fn false_positive_rate(&self, pattern_id: &str) -> f64 {
        if let Some(records) = self.false_positive_records.get(pattern_id) {
            // Simple heuristic: more records = higher rate
            let count = records.len() as f64;
            (count / (count + 10.0)).min(1.0)
        } else {
            0.0
        }
    }

    pub fn disable_pattern(&mut self, pattern_id: &str) {
        self.disabled_patterns.push(pattern_id.to_string());
    }

    pub fn is_pattern_enabled(&self, pattern_id: &str) -> bool {
        !self.disabled_patterns.contains(&pattern_id.to_string())
    }
}
