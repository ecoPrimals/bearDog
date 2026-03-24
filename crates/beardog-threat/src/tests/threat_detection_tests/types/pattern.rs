// SPDX-License-Identifier: AGPL-3.0-only

//! Pattern Matching Test Types

use std::fmt;

/// Threat severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for ThreatSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

/// Threat pattern
#[derive(Debug, Clone)]
pub struct ThreatPattern {
    id: String,
    pattern: String,
    severity: ThreatSeverity,
}

impl ThreatPattern {
    pub fn new(id: &str, pattern: &str, severity: ThreatSeverity) -> Self {
        Self {
            id: id.to_string(),
            pattern: pattern.to_string(),
            severity,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn matches(&self, input: &str) -> bool {
        // Simplified regex matching
        input.contains(&self.pattern.replace(".*", "").replace('\\', ""))
            || self.pattern_match(input)
    }

    fn pattern_match(&self, input: &str) -> bool {
        // Simple pattern matching for common cases
        if self.pattern.contains("SELECT") && input.to_uppercase().contains("SELECT") {
            return true;
        }
        if self.pattern.contains("<script") && input.contains("<script") {
            return true;
        }
        if self.pattern.contains("../") && input.contains("../") {
            return true;
        }
        if self.pattern.contains("exploit") && input.contains("exploit") {
            return true;
        }
        false
    }

    pub fn severity(&self) -> ThreatSeverity {
        self.severity
    }
}

/// Pattern match result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    pattern_id: String,
    severity: ThreatSeverity,
}

impl PatternMatch {
    pub fn new(pattern_id: String, severity: ThreatSeverity) -> Self {
        Self {
            pattern_id,
            severity,
        }
    }

    pub fn pattern_id(&self) -> &str {
        &self.pattern_id
    }

    pub fn severity(&self) -> ThreatSeverity {
        self.severity
    }
}

/// Pattern matcher
pub struct PatternMatcher {
    patterns: Vec<ThreatPattern>,
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: ThreatPattern) {
        self.patterns.push(pattern);
    }

    pub fn remove_pattern(&mut self, pattern_id: &str) {
        self.patterns.retain(|p| p.id() != pattern_id);
    }

    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn find_matches(&self, input: &str) -> Vec<PatternMatch> {
        self.patterns
            .iter()
            .filter(|p| p.matches(input))
            .map(|p| PatternMatch::new(p.id().to_string(), p.severity()))
            .collect()
    }

    pub fn find_highest_severity_match(&self, input: &str) -> Option<PatternMatch> {
        let matches = self.find_matches(input);
        matches.into_iter().max_by_key(PatternMatch::severity)
    }
}
