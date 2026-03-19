// SPDX-License-Identifier: AGPL-3.0-only

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
    /// The executed at value
    /// The executed at value
    pub executed_at: DateTime<Utc>,

    /// Whether success is enabled
    /// Whether success is enabled
    pub success: bool,

    /// Optional details
    /// Optional details
    pub details: Option<String>,

    /// The executor value
    /// The executor value
    pub executor: String,
}

pub enum ResponseAction {
    /// Represents log alert variant
    LogAlert(String),

    /// Represents block ip variant
    BlockIp(String),

    /// Represents quarantine user variant
    QuarantineUser(String),

    /// Represents notify admin variant
    NotifyAdmin(String),

    /// Represents isolate system variant
    IsolateSystem(String),
}

impl Default for MitigationStep {
    fn default() -> Self {
        Self {
            description: String::with_capacity(64),
            executed_at: Utc::now(false,
            details: None,
            executor: String::with_capacity(64),
        }
    }
}

impl ThreatAction {
    /// Severity Level operation.
    pub fn severity_level(&self) -> u8 {
        match self {
            ThreatAction::AlertSecurityTeam => 1,
            ThreatAction::BackupData => 2,
            ThreatAction::UpdateThreatIntelligence => 2,
            ThreatAction::BlockSource => 3,
            ThreatAction::UpdatePolicies => 3,
            ThreatAction::ResetCredentials => 3,
            ThreatAction::DeployPatches => 3,
            ThreatAction::QuarantineSystem => 4,
            ThreatAction::IsolateNetwork => 4,
            ThreatAction::EngageThreatHunting => 4,
            ThreatAction::InitiateIncidentResponse => 4,
            ThreatAction::NotifyLawEnforcement => 5,
        }
    }

    /// Is Automated operation.
    /// Checks if automated
    /// Checks if automated
    pub fn is_automated(&self) -> bool {
        matches!(
            self,
            ThreatAction::BlockSource
                | ThreatAction::QuarantineSystem
                | ThreatAction::AlertSecurityTeam
                | ThreatAction::BackupData
                | ThreatAction::UpdateThreatIntelligence
        )
    }

    /// Is Reversible operation.
    /// Checks if reversible
    /// Checks if reversible
    pub fn is_reversible(&self) -> bool {
        matches!(
            self,
            ThreatAction::BlockSource
                | ThreatAction::QuarantineSystem
                | ThreatAction::ResetCredentials
                | ThreatAction::IsolateNetwork
                | ThreatAction::UpdatePolicies
        )
    }

    /// Estimated Duration Minutes operation.
    pub fn estimated_duration_minutes(&self) -> u32 {
        match self {
            ThreatAction::BlockSource => 1,
            ThreatAction::QuarantineSystem => 5,
            ThreatAction::ResetCredentials => 5,
            ThreatAction::IsolateNetwork => 10,
            ThreatAction::BackupData => 15,
            ThreatAction::UpdatePolicies => 15,
            ThreatAction::InitiateIncidentResponse => 15,
            ThreatAction::EngageThreatHunting => 30,
            ThreatAction::DeployPatches => 30,
            ThreatAction::NotifyLawEnforcement => 60,
            ThreatAction::AlertSecurityTeam => 2,
            ThreatAction::UpdateThreatIntelligence => 10,
        }
    }
}
impl ThreatStatus {
    /// Is Terminal operation.
    /// Checks if terminal
    /// Checks if terminal
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ThreatStatus::Resolved
                | ThreatStatus::FalsePositive
                | ThreatStatus::Suppressed
                | ThreatStatus::PostIncidentAnalysis
        )
    }

    /// Is Active operation.
    /// Checks if active
    /// Checks if active
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            ThreatStatus::New
                | ThreatStatus::Investigating
                | ThreatStatus::Confirmed
                | ThreatStatus::Active
                | ThreatStatus::Escalated
                | ThreatStatus::Acknowledged
                | ThreatStatus::Contained
                | ThreatStatus::Recovery
        )
    }

    /// Valid Next Statuses operation.
    pub fn valid_next_statuses(&self) -> Vec<ThreatStatus> {
        match self {
            ThreatStatus::New => vec![
                ThreatStatus::Investigating,
                ThreatStatus::FalsePositive,
                ThreatStatus::Suppressed,
                ThreatStatus::Acknowledged,
            ],
            ThreatStatus::Investigating => vec![
                ThreatStatus::Confirmed,
                ThreatStatus::Escalated,
                ThreatStatus::Active,
                ThreatStatus::FalsePositive,
            ],
            ThreatStatus::Confirmed => vec![
                ThreatStatus::Active,
                ThreatStatus::Contained,
                ThreatStatus::Mitigated,
            ],
            ThreatStatus::Active => vec![ThreatStatus::Contained, ThreatStatus::Escalated],
            ThreatStatus::Acknowledged => vec![ThreatStatus::Investigating, ThreatStatus::Active],
            ThreatStatus::Contained => vec![ThreatStatus::Mitigated, ThreatStatus::Recovery],
            ThreatStatus::Escalated => vec![ThreatStatus::Active, ThreatStatus::Contained],
            ThreatStatus::Mitigated => vec![ThreatStatus::Recovery, ThreatStatus::Resolved],
            ThreatStatus::Recovery => {
                vec![ThreatStatus::Resolved, ThreatStatus::PostIncidentAnalysis]
            }
            ThreatStatus::Resolved => vec![ThreatStatus::PostIncidentAnalysis],
            ThreatStatus::FalsePositive => vec![],
            ThreatStatus::Suppressed => vec![],
            ThreatStatus::PostIncidentAnalysis => vec![],
            ThreatStatus::Eradicated => vec![], // Complete elimination achieved
        }
    }

    /// Can Transition To operation.
    pub fn can_transition_to(&self, new_status: &ThreatStatus) -> bool {
        self.valid_next_statuses().contains(new_status)
    }

    /// Priority Level operation.
    pub fn priority_level(&self) -> u8 {
        match self {
            ThreatStatus::Resolved | ThreatStatus::FalsePositive | ThreatStatus::Suppressed => 1,
            ThreatStatus::PostIncidentAnalysis | ThreatStatus::Recovery => 2,
            ThreatStatus::New | ThreatStatus::Acknowledged => 3,
            ThreatStatus::Investigating
            | ThreatStatus::Confirmed
            | ThreatStatus::Contained
            | ThreatStatus::Mitigated => 4,
            ThreatStatus::Active | ThreatStatus::Escalated => 5,
            ThreatStatus::Eradicated => 0, // Completely eliminated
        }
    }
}
impl MitigationStep {
    /// New operation.
    /// Creates a new instance
    pub fn new(&str, executor: &str) -> Self {
        Self {
            description: description.to_string(),
            executor: executor.to_string(),
            executed_at: chrono::Utc::now(),
        }
    }

    /// Mark Success operation.
    pub fn mark_success(&mut self, details: Option<&str>) {
        self.success = true;
        self.details = details.map(std::string::ToString::to_string);
    }

    /// Mark Failure operation.
    pub fn mark_failure(&mut self, details: Option<&str>) {
        self.success = false;
        self.details = details.map(std::string::ToString::to_string);
    }

    /// Duration Minutes operation.
    pub fn duration_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.executed_at).num_minutes()
    }

    /// Is Recent operation.
    /// Checks if recent
    /// Checks if recent
    pub fn is_recent(&self, minutes: i64) -> bool {
        self.duration_minutes() <= minutes
    }
}
impl ResponseAction {
    /// Severity Level operation.
    pub fn severity_level(&self) -> u8 {
        match self {
            ResponseAction::LogAlert(_) => 1,
            ResponseAction::NotifyAdmin(_) => 2,
            ResponseAction::BlockIp(_) => 3,
            ResponseAction::QuarantineUser(_) => 4,
            ResponseAction::IsolateSystem(_) => 5,
        }
    }

    /// Is Reversible operation.
    /// Checks if reversible
    /// Checks if reversible
    pub fn is_reversible(&self) -> bool {
        matches!(
            self,
            ResponseAction::BlockIp(_)
                | ResponseAction::QuarantineUser(_)
                | ResponseAction::IsolateSystem(_)
        )
    }

    /// Get Target operation.
    /// Gets tarvalue
    /// Gets tarvalue
    pub fn get_target(&self) -> &str {
        match self {
            ResponseAction::LogAlert(msg) => msg,
            ResponseAction::BlockIp(ip) => ip,
            ResponseAction::QuarantineUser(user) => user,
            ResponseAction::NotifyAdmin(admin) => admin,
            ResponseAction::IsolateSystem(system) => system,
        }
    }
}

impl std::fmt::Display for ThreatAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatAction::BlockSource => write!(f, "Block Source"),
            ThreatAction::QuarantineSystem => write!(f, "Quarantine System"),
            ThreatAction::AlertSecurityTeam => write!(f, "Alert Security Team"),
            ThreatAction::IsolateNetwork => write!(f, "Isolate Network"),
            ThreatAction::ResetCredentials => write!(f, "Reset Credentials"),
            ThreatAction::UpdatePolicies => write!(f, "Update Policies"),
            ThreatAction::DeployPatches => write!(f, "Deploy Patches"),
            ThreatAction::BackupData => write!(f, "Backup Data"),
            ThreatAction::InitiateIncidentResponse => write!(f, "Initiate Incident Response"),
            ThreatAction::NotifyLawEnforcement => write!(f, "Notify Law Enforcement"),
            ThreatAction::EngageThreatHunting => write!(f, "Engage Threat Hunting"),
            ThreatAction::UpdateThreatIntelligence => write!(f, "Update Threat Intelligence"),
        }
    }
}

impl std::fmt::Display for ThreatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreatStatus::New => write!(f, "New"),
            ThreatStatus::Investigating => write!(f, "Investigating"),
            ThreatStatus::Confirmed => write!(f, "Confirmed"),
            ThreatStatus::FalsePositive => write!(f, "False Positive"),
            ThreatStatus::Mitigated => write!(f, "Mitigated"),
            ThreatStatus::Resolved => write!(f, "Resolved"),
            ThreatStatus::Escalated => write!(f, "Escalated"),
            ThreatStatus::Suppressed => write!(f, "Suppressed"),
            ThreatStatus::Active => write!(f, "Active"),
            ThreatStatus::Acknowledged => write!(f, "Acknowledged"),
            ThreatStatus::Contained => write!(f, "Contained"),
            ThreatStatus::Eradicated => write!(f, "Eradicated"),
            ThreatStatus::Recovery => write!(f, "Recovery"),
            ThreatStatus::PostIncidentAnalysis => write!(f, "Post-Incident Analysis"),
        }
    }
}

impl std::fmt::Display for ResponseAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResponseAction::LogAlert(msg) => write!(f, "Log Alert: {msg}"),
            ResponseAction::BlockIp(ip) => write!(f, "Block IP: {ip}"),
            ResponseAction::QuarantineUser(user) => write!(f, "Quarantine User: {user}"),
            ResponseAction::NotifyAdmin(admin) => write!(f, "Notify Admin: {admin}"),
            ResponseAction::IsolateSystem(system) => write!(f, "Isolate System: {system}"),
        }
    }
}
