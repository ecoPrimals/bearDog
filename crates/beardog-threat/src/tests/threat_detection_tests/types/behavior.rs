// SPDX-License-Identifier: AGPL-3.0-only

//! Behavioral Analysis Test Types

use super::pattern::ThreatSeverity;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum BehaviorEvent {
    Login {
        timestamp: Instant,
        success: bool,
        source_ip: String,
    },
    AccessDenied {
        timestamp: Instant,
        resource: String,
    },
    DataAccess {
        timestamp: Instant,
        resource: String,
    },
    RateLimitExceeded {
        timestamp: Instant,
    },
    SuspiciousActivity {
        timestamp: Instant,
        description: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ThreatType {
    BruteForce,
    Dos,
    Injection,
    SQLInjection,
    PathTraversal,
    Malware,
    UnusualAccess,
    SuspiciousActivity,
    DataBreach,
    DataExfiltration,
    None,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Threat {
    threat_type: ThreatType,
    timestamp: Instant,
    metadata: HashMap<String, String>,
    confidence: f64,
    severity: ThreatSeverity,
    is_enriched: bool,
    matched_iocs: Vec<String>,
}

impl Threat {
    pub fn new(threat_type: ThreatType, severity: ThreatSeverity) -> Self {
        Self {
            threat_type,
            timestamp: Instant::now(),
            metadata: HashMap::new(),
            confidence: 1.0,
            severity,
            is_enriched: false,
            matched_iocs: Vec::new(),
        }
    }

    pub fn threat_type(&self) -> ThreatType {
        self.threat_type
    }

    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    pub fn severity(&self) -> ThreatSeverity {
        self.severity
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    pub fn enriched(&self) -> bool {
        self.is_enriched
    }

    pub fn ioc_matches(&self) -> &[String] {
        &self.matched_iocs
    }

    pub fn mark_enriched(&mut self, iocs: Vec<String>) {
        self.is_enriched = true;
        self.matched_iocs = iocs;
    }

    pub fn elevate_severity(&mut self, new_severity: ThreatSeverity) {
        if new_severity > self.severity {
            self.severity = new_severity;
        }
    }

    pub fn confidence(&self) -> f64 {
        self.confidence
    }

    pub fn set_confidence(&mut self, confidence: f64) {
        self.confidence = confidence.clamp(0.0, 1.0);
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.metadata.insert("id".to_string(), id.into());
        self
    }
}

pub struct BehaviorAnalyzer {
    events: Vec<BehaviorEvent>,
    profiles: HashMap<String, Vec<BehaviorEvent>>,
}

impl BehaviorAnalyzer {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            profiles: HashMap::new(),
        }
    }

    pub fn record_event(&mut self, _user_id: &str, event: BehaviorEvent) {
        self.events.push(event);
    }

    pub fn analyze(&self) -> Vec<Threat> {
        let mut threats = Vec::new();

        // Count failed logins
        let failed_logins = self
            .events
            .iter()
            .filter(|e| matches!(e, BehaviorEvent::Login { success: false, .. }))
            .count();

        if failed_logins > 3 {
            threats.push(Threat::new(ThreatType::BruteForce, ThreatSeverity::High));
        }

        threats
    }

    pub fn analyze_event(&self, user_id: &str, event: &BehaviorEvent) -> Option<Threat> {
        // Analyze single event for immediate threats
        match event {
            BehaviorEvent::Login { success: false, .. } => {
                // Check recent failed attempts
                let recent_failures = self
                    .events
                    .iter()
                    .rev()
                    .take(10)
                    .filter(|e| matches!(e, BehaviorEvent::Login { success: false, .. }))
                    .count();

                if recent_failures > 5 {
                    Some(Threat::new(ThreatType::BruteForce, ThreatSeverity::High))
                } else {
                    None
                }
            }
            BehaviorEvent::Login {
                success: true,
                source_ip,
                ..
            } => {
                // Check if login from unusual location (if profile exists)
                if let Some(profile) = self.profiles.get(user_id) {
                    let known_ips: Vec<_> = profile
                        .iter()
                        .filter_map(|e| {
                            if let BehaviorEvent::Login { source_ip, .. } = e {
                                Some(source_ip.as_str())
                            } else {
                                None
                            }
                        })
                        .collect();

                    // If this IP hasn't been seen before, flag as unusual access
                    if !known_ips.contains(&source_ip.as_str()) {
                        return Some(Threat::new(
                            ThreatType::UnusualAccess,
                            ThreatSeverity::Medium,
                        ));
                    }
                }
                None
            }
            BehaviorEvent::DataAccess { resource, .. } => {
                // Check if accessing unusual resources (if profile exists)
                if let Some(profile) = self.profiles.get(user_id) {
                    let known_resources: Vec<_> = profile
                        .iter()
                        .filter_map(|e| {
                            if let BehaviorEvent::DataAccess { resource, .. } = e {
                                Some(resource.as_str())
                            } else {
                                None
                            }
                        })
                        .collect();

                    // If accessing a resource not in profile, flag as suspicious
                    if !known_resources.contains(&resource.as_str())
                        && (resource.contains("admin") || resource.contains("sensitive"))
                    {
                        return Some(Threat::new(
                            ThreatType::SuspiciousActivity,
                            ThreatSeverity::High,
                        ));
                    }
                }
                None
            }
            BehaviorEvent::RateLimitExceeded { .. } => {
                Some(Threat::new(ThreatType::Dos, ThreatSeverity::Medium))
            }
            _ => None,
        }
    }

    pub fn build_profile(&mut self, identifier: &str) {
        let profile_events = self.events.clone();
        self.profiles.insert(identifier.to_string(), profile_events);
    }

    pub fn has_profile(&self, identifier: &str) -> bool {
        self.profiles.contains_key(identifier)
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}
