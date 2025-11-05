//! Threat Classification Test Types

use super::behavior::Threat;
use super::composite::ThreatCategory;
use super::pattern::ThreatSeverity;

pub struct ThreatClassifier;

impl ThreatClassifier {
    pub fn new() -> Self {
        Self
    }

    pub fn classify(&self, input: &str) -> Option<ThreatSeverity> {
        if input.contains("CRITICAL") {
            Some(ThreatSeverity::Critical)
        } else if input.contains("HIGH") {
            Some(ThreatSeverity::High)
        } else if input.contains("MEDIUM") {
            Some(ThreatSeverity::Medium)
        } else if input.contains("LOW") {
            Some(ThreatSeverity::Low)
        } else {
            None
        }
    }

    pub fn categorize(&self, threat: &Threat) -> ThreatCategory {
        use super::behavior::ThreatType;
        match threat.threat_type() {
            ThreatType::BruteForce => ThreatCategory::AuthenticationAttack,
            ThreatType::Injection | ThreatType::SQLInjection | ThreatType::PathTraversal => {
                ThreatCategory::InjectionAttack
            }
            ThreatType::Malware => ThreatCategory::MaliciousSoftware,
            ThreatType::Dos | ThreatType::UnusualAccess => ThreatCategory::Network,
            ThreatType::DataBreach | ThreatType::DataExfiltration => ThreatCategory::System,
            _ => ThreatCategory::System, // Default category
        }
    }

    pub fn calculate_score(&self, threat: &Threat) -> f64 {
        match threat.severity() {
            ThreatSeverity::Critical => 100.0,
            ThreatSeverity::High => 75.0,
            ThreatSeverity::Medium => 50.0,
            ThreatSeverity::Low => 25.0,
            ThreatSeverity::Info => 10.0,
        }
    }

    pub fn calculate_composite_score(&self, composite: &super::composite::CompositeThreat) -> f64 {
        if composite.indicators.is_empty() {
            return 0.0;
        }
        let total: f64 = composite
            .indicators
            .iter()
            .map(|t| self.calculate_score(t))
            .sum();
        total / composite.indicators.len() as f64
    }

    pub fn prioritize(&self, threats: Vec<Threat>) -> Vec<Threat> {
        let mut sorted = threats;
        sorted.sort_by_key(|b| std::cmp::Reverse(b.severity()));
        sorted
    }

    pub fn calculate_score_with_confidence(&self, threat: &Threat) -> f64 {
        self.calculate_score(threat) * threat.confidence()
    }
}
