// SPDX-License-Identifier: AGPL-3.0-only

//! Threat Intelligence Test Types

use super::behavior::Threat;
use super::pattern::ThreatSeverity;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IocType {
    IpAddress,
    IPAddress, // Alias for IpAddress
    Domain,
    FileHash,
    Url,
}

#[derive(Debug, Clone)]
pub struct Ioc {
    pub ioc_type: IocType,
    pub value: String,
    pub threat_level: ThreatSeverity,
    pub description: String,
    pub expiry: Option<Instant>,
}

impl Ioc {
    pub fn new(
        ioc_type: IocType,
        value: impl Into<String>,
        threat_level: ThreatSeverity,
        description: impl Into<String>,
    ) -> Self {
        Self {
            ioc_type,
            value: value.into(),
            threat_level,
            description: description.into(),
            expiry: None, // No expiry by default
        }
    }

    pub fn new_with_expiry(
        ioc_type: IocType,
        value: impl Into<String>,
        threat_level: ThreatSeverity,
        description: impl Into<String>,
        expiry: Duration,
    ) -> Self {
        Self {
            ioc_type,
            value: value.into(),
            threat_level,
            description: description.into(),
            expiry: Some(Instant::now() + expiry),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expiry.is_some_and(|exp| Instant::now() > exp)
    }
}

pub struct ThreatIntelligence {
    iocs: HashMap<String, Ioc>,
}

impl ThreatIntelligence {
    pub fn new() -> Self {
        Self {
            iocs: HashMap::new(),
        }
    }

    pub fn add_ioc(&mut self, ioc: Ioc) {
        self.iocs.insert(ioc.value.clone(), ioc);
    }

    pub fn lookup(&self, value: &str) -> Option<LookupResult> {
        self.iocs.get(value).map(|ioc| {
            let is_malicious = matches!(
                ioc.threat_level,
                ThreatSeverity::High | ThreatSeverity::Critical
            );
            LookupResult {
                found: true,
                ioc: ioc.clone(),
                is_malicious,
            }
        })
    }

    pub fn ioc_count(&self) -> usize {
        self.iocs.len()
    }

    pub fn is_malicious(&self, _ioc_type: IocType, value: &str) -> bool {
        self.iocs.contains_key(value)
    }

    pub fn get_reputation_score(&self, _ioc_type: IocType, value: &str) -> f64 {
        self.iocs
            .get(value)
            .map(|ioc| {
                // Convert ThreatSeverity to reputation score (0.0 = worst, 100.0 = best)
                use ThreatSeverity::*;
                match ioc.threat_level {
                    Critical => 0.0,
                    High => 25.0,
                    Medium => 50.0,
                    Low => 75.0,
                    Info => 100.0,
                }
            })
            .unwrap_or(100.0) // Default to good reputation for unknown IPs
    }

    pub fn enrich_threat(&self, threat: &mut Threat) {
        // Check if any metadata values match known IOCs
        let mut matched_iocs = Vec::new();
        let mut max_severity = None;

        // Collect matches first to avoid borrow conflicts
        for (_, value) in threat.metadata().iter() {
            if let Some(ioc) = self.iocs.get(value) {
                matched_iocs.push(value.clone());
                max_severity = Some(match max_severity {
                    Some(current) => {
                        if ioc.threat_level > current {
                            ioc.threat_level
                        } else {
                            current
                        }
                    }
                    None => ioc.threat_level,
                });
            }
        }

        // Now mutate threat
        if let Some(severity) = max_severity {
            threat.elevate_severity(severity);
        }

        if !matched_iocs.is_empty() {
            threat.mark_enriched(matched_iocs);
        }
    }

    pub fn import_from_feed(&mut self, _feed: &ThreatFeed, ioc: Ioc) {
        // In real implementation, would fetch from feed URLs
        // For tests, just add the provided IOC
        self.add_ioc(ioc);
    }

    pub fn cleanup_expired(&mut self) {
        // Remove expired IOCs
        self.iocs.retain(|_, ioc| !ioc.is_expired());
    }

    pub fn bulk_lookup(&self, ioc_type: IocType, values: &[&str]) -> Vec<LookupResult> {
        values
            .iter()
            .map(|v| {
                self.lookup(v).unwrap_or_else(|| {
                    // Create a not-found result with a dummy IOC
                    LookupResult {
                        found: false,
                        ioc: Ioc::new(ioc_type, *v, ThreatSeverity::Info, "Not found"),
                        is_malicious: false,
                    }
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct LookupResult {
    pub found: bool,
    pub ioc: Ioc,
    pub is_malicious: bool,
}

pub struct ThreatFeed {
    name: String,
    feeds: Vec<String>,
    last_updated: Option<Instant>,
    update_interval: Duration,
}

impl ThreatFeed {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            feeds: Vec::new(),
            last_updated: Some(Instant::now()), // Initialize as fresh
            update_interval: Duration::from_secs(3600),
        }
    }

    pub fn add_feed(&mut self, feed_url: String) {
        self.feeds.push(feed_url);
        self.last_updated = Some(Instant::now());
    }

    pub fn feed_count(&self) -> usize {
        self.feeds.len()
    }

    pub fn is_stale(&self, max_age: Duration) -> bool {
        if let Some(last_updated) = self.last_updated {
            Instant::now().duration_since(last_updated) > max_age
        } else {
            true // Never updated = stale
        }
    }
}
