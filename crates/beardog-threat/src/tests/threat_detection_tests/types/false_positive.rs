//! False Positive Handling Test Types

use std::collections::{HashMap, HashSet};

pub struct FalsePositiveHandler {
    whitelist: HashMap<String, String>, // key -> value mapping
    known_false_positives: HashSet<String>,
}

impl FalsePositiveHandler {
    pub fn new() -> Self {
        Self {
            whitelist: HashMap::new(),
            known_false_positives: HashSet::new(),
        }
    }

    pub fn add_to_whitelist(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.whitelist.insert(key.into(), value.into());
    }

    pub fn is_whitelisted(&self, key: &str, value: &str) -> bool {
        self.whitelist.get(key).is_some_and(|v| v == value)
    }

    pub fn whitelist_size(&self) -> usize {
        self.whitelist.len()
    }

    pub fn mark_as_false_positive(&mut self, threat_id: &str) {
        self.known_false_positives.insert(threat_id.to_string());
    }

    pub fn is_known_false_positive(&self, threat_id: &str) -> bool {
        self.known_false_positives.contains(threat_id)
    }

    pub fn false_positive_count(&self) -> usize {
        self.known_false_positives.len()
    }

    pub fn should_suppress(&self, threat: &super::behavior::Threat) -> bool {
        // Check if threat metadata matches whitelist or is known false positive
        let threat_id = threat
            .metadata()
            .get("id")
            .map(|s| s.as_str())
            .unwrap_or("");

        if self.is_known_false_positive(threat_id) {
            return true;
        }

        // Check if any metadata values match any whitelisted values
        for threat_value in threat.metadata().values() {
            for whitelist_value in self.whitelist.values() {
                if threat_value == whitelist_value {
                    return true;
                }
            }
        }

        false
    }
}
