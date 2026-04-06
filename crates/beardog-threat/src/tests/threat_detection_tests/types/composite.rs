// SPDX-License-Identifier: AGPL-3.0-or-later

//! Composite Threat Test Types

use super::behavior::Threat;
use super::pattern::ThreatSeverity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatCategory {
    Network,
    Application,
    System,
    AuthenticationAttack,
    InjectionAttack,
    MaliciousSoftware,
}

#[derive(Debug, Clone)]
pub struct CompositeThreat {
    pub id: String,
    pub indicators: Vec<Threat>,
}

impl CompositeThreat {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            indicators: Vec::new(),
        }
    }

    pub fn add_indicator(&mut self, indicator: Threat) {
        self.indicators.push(indicator);
    }

    pub fn severity(&self) -> ThreatSeverity {
        // Return highest severity from indicators
        self.indicators
            .iter()
            .map(super::behavior::Threat::severity)
            .max()
            .unwrap_or(ThreatSeverity::Info)
    }
}
