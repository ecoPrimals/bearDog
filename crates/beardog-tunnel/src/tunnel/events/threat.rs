// SPDX-License-Identifier: AGPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of suspicious activity
pub enum SuspiciousActivityType {
    /// Represents excessive connections variant
    ExcessiveConnections,
    /// Represents unusual traffic pattern variant
    UnusualTrafficPattern,
    /// Represents failed authentication variant
    FailedAuthentication,
    /// Represents data exfiltration variant
    DataExfiltration,
    /// Currently portscanning
    PortScanning,
    /// Unknown or undefined state
    Unknown,
}

/// Actions taken to mitigate detected threats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatMitigationAction {
    /// Represents isolate traffic variant
    IsolateTraffic,
    /// Represents upgrade security variant
    UpgradeSecurity,
    /// Represents terminate session variant
    TerminateSession,
    /// Represents monitor closely variant
    MonitorClosely,
    /// Represents reroute traffic variant
    RerouteTraffic,
    /// Represents increase authentication variant
    IncreaseAuthentication,
}
