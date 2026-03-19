// SPDX-License-Identifier: AGPL-3.0-only

//! Monitoring Test Types

use super::pattern::ThreatSeverity;

use super::alerting::{ThreatAlert, ThreatEvent};

type AlertCallback = Box<dyn Fn(ThreatAlert) + Send + Sync>;

pub struct ThreatMonitor {
    detected_count: usize,
    total_events: usize,
    threat_events: usize,
    benign_events: usize,
    alert_callback: Option<AlertCallback>,
}

impl ThreatMonitor {
    pub fn new() -> Self {
        Self {
            detected_count: 0,
            total_events: 0,
            threat_events: 0,
            benign_events: 0,
            alert_callback: None,
        }
    }

    pub fn record_threat(&mut self, _severity: ThreatSeverity) {
        self.detected_count += 1;
        self.total_events += 1;
        self.threat_events += 1;
    }

    pub fn process_event(&mut self, event: &ThreatEvent) {
        self.total_events += 1;
        if event.is_threat {
            self.detected_count += 1;
            self.threat_events += 1;

            // Create alert for threats and call callback
            if let Some(ref callback) = self.alert_callback {
                let alert = ThreatAlert {
                    event: event.clone(),
                    notified: true,
                };
                callback(alert);
            }
        } else {
            self.benign_events += 1;
        }
    }

    pub fn get_statistics(&self) -> MonitorStatistics {
        MonitorStatistics {
            total_threats: self.detected_count,
            total_events: self.total_events,
            threat_events: self.threat_events,
            benign_events: self.benign_events,
        }
    }

    pub fn set_alert_callback<F>(&mut self, callback: F)
    where
        F: Fn(ThreatAlert) + Send + Sync + 'static,
    {
        self.alert_callback = Some(Box::new(callback));
    }
}

#[derive(Debug, Clone)]
pub struct MonitorStatistics {
    pub total_threats: usize,
    pub total_events: usize,
    pub threat_events: usize,
    pub benign_events: usize,
}
