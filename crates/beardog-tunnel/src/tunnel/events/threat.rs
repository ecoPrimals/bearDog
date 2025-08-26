

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuspiciousActivityType {

    ExcessiveConnections,

    UnusualTrafficPattern,

    FailedAuthentication,

    DataExfiltration,

    PortScanning,

    Unknown,
}

pub enum ThreatMitigationAction {

    IsolateTraffic,

    UpgradeSecurity,

    TerminateSession,

    MonitorClosely,

    RerouteTraffic,

    IncreaseAuthentication,
