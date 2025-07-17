//! Threat action and response types
//!
//! This module contains types for threat actions, status tracking, and mitigation steps.
//!
//! ## Features
//! - Threat action enumeration
//! - Status tracking for threat lifecycle
//! - Mitigation step recording
//! - Response action types
//! - Status transitions and validation
//!
//! ## Example
//! ```rust
//! use beardog::threat::types::{ThreatAction, ThreatStatus, MitigationStep};
//! use chrono::Utc;
//!
//! let action = ThreatAction::BlockSource;
//! let status = ThreatStatus::Investigating;
//!
//! let mitigation = MitigationStep {
//!     description: "Blocked malicious IP address".to_string(),
//!     executed_at: Utc::now(),
//!     success: true,
//!     executor: "security-system".to_string(),
//!     ..Default::default()
//! };
//! ```

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Threat action enumeration
///
/// Defines the various actions that can be taken in response to
/// detected threats, from blocking to notification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreatAction {
    /// Block the source
    ///
    /// Prevents further communication from the threat source.
    BlockSource,

    /// Quarantine affected systems
    ///
    /// Isolates compromised systems to prevent lateral movement.
    QuarantineSystem,

    /// Alert security team
    ///
    /// Notifies security personnel about the threat.
    AlertSecurityTeam,

    /// Isolate network segment
    ///
    /// Isolates network segments to contain the threat.
    IsolateNetwork,

    /// Reset user credentials
    ///
    /// Forces password reset for potentially compromised accounts.
    ResetCredentials,

    /// Update security policies
    ///
    /// Modifies security policies to address the threat.
    UpdatePolicies,

    /// Deploy patches
    ///
    /// Applies security patches to vulnerable systems.
    DeployPatches,

    /// Backup critical data
    ///
    /// Creates backups of critical data before remediation.
    BackupData,

    /// Initiate incident response
    ///
    /// Triggers formal incident response procedures.
    InitiateIncidentResponse,

    /// Notify law enforcement
    ///
    /// Contacts appropriate law enforcement agencies.
    NotifyLawEnforcement,

    /// Engage threat hunting team
    ///
    /// Activates threat hunting activities.
    EngageThreatHunting,

    /// Update threat intelligence
    ///
    /// Updates threat intelligence feeds with new indicators.
    UpdateThreatIntelligence,
}

/// Threat status enumeration
///
/// Tracks the current status of a threat through its lifecycle
/// from detection to resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ThreatStatus {
    /// New threat detected and awaiting initial assessment
    #[default]
    New,

    /// Threat under active investigation
    Investigating,

    /// Threat confirmed as genuine security issue
    Confirmed,

    /// Threat determined to be false positive
    FalsePositive,

    /// Threat has been mitigated
    Mitigated,

    /// Threat has been fully resolved
    Resolved,

    /// Threat escalated to higher security level
    Escalated,

    /// Threat alerts suppressed due to noise
    Suppressed,

    /// Threat is currently active and ongoing
    Active,

    /// Threat acknowledged by security team
    Acknowledged,

    /// Threat contained but not yet eliminated
    Contained,

    /// Threat fully eradicated from systems
    Eradicated,

    /// System recovery in progress after threat
    Recovery,

    /// Post-incident analysis phase
    PostIncidentAnalysis,
}

/// Mitigation step structure
///
/// Represents a single step taken to mitigate a threat,
/// including execution details and success status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStep {
    /// Step description
    pub description: String,

    /// Execution timestamp
    pub executed_at: DateTime<Utc>,

    /// Success status
    pub success: bool,

    /// Additional details
    pub details: Option<String>,

    /// Executing system/user
    pub executor: String,
}

/// Response action types
///
/// Defines specific response actions that can be taken
/// during automated threat response.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResponseAction {
    /// Log alert message
    LogAlert(String),

    /// Block IP address
    BlockIp(String),

    /// Quarantine user account
    QuarantineUser(String),

    /// Notify administrator
    NotifyAdmin(String),

    /// Isolate system
    IsolateSystem(String),
}

// Default implementations

impl Default for MitigationStep {
    fn default() -> Self {
        Self {
            description: String::new(),
            executed_at: Utc::now(),
            success: false,
            details: None,
            executor: String::new(),
        }
    }
}

// Utility implementations

impl ThreatAction {
    /// Get the severity level of this action
    ///
    /// # Returns
    /// Severity level (1-5, where 5 is most severe)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatAction;
    ///
    /// assert_eq!(ThreatAction::BlockSource.severity_level(), 3);
    /// assert_eq!(ThreatAction::NotifyLawEnforcement.severity_level(), 5);
    /// ```
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

    /// Check if action is automated
    ///
    /// # Returns
    /// `true` if action can be performed automatically
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatAction;
    ///
    /// assert!(ThreatAction::BlockSource.is_automated());
    /// assert!(!ThreatAction::NotifyLawEnforcement.is_automated());
    /// ```
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

    /// Check if action is reversible
    ///
    /// # Returns
    /// `true` if action can be easily reversed
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatAction;
    ///
    /// assert!(ThreatAction::BlockSource.is_reversible());
    /// assert!(!ThreatAction::NotifyLawEnforcement.is_reversible());
    /// ```
    pub fn is_reversible(&self) -> bool {
        matches!(
            self,
            ThreatAction::BlockSource
                | ThreatAction::QuarantineSystem
                | ThreatAction::IsolateNetwork
                | ThreatAction::UpdatePolicies
        )
    }

    /// Get estimated execution time in minutes
    ///
    /// # Returns
    /// Estimated time to complete the action
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatAction;
    ///
    /// assert_eq!(ThreatAction::BlockSource.estimated_duration_minutes(), 1);
    /// assert_eq!(ThreatAction::DeployPatches.estimated_duration_minutes(), 30);
    /// ```
    pub fn estimated_duration_minutes(&self) -> u32 {
        match self {
            ThreatAction::BlockSource => 1,
            ThreatAction::AlertSecurityTeam => 1,
            ThreatAction::UpdateThreatIntelligence => 2,
            ThreatAction::QuarantineSystem => 5,
            ThreatAction::ResetCredentials => 5,
            ThreatAction::IsolateNetwork => 10,
            ThreatAction::BackupData => 15,
            ThreatAction::UpdatePolicies => 15,
            ThreatAction::InitiateIncidentResponse => 15,
            ThreatAction::EngageThreatHunting => 30,
            ThreatAction::DeployPatches => 30,
            ThreatAction::NotifyLawEnforcement => 60,
        }
    }
}

impl ThreatStatus {
    /// Check if status is terminal (final)
    ///
    /// # Returns
    /// `true` if status represents a final state
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatus;
    ///
    /// assert!(ThreatStatus::Resolved.is_terminal());
    /// assert!(!ThreatStatus::Investigating.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ThreatStatus::Resolved
                | ThreatStatus::FalsePositive
                | ThreatStatus::Suppressed
                | ThreatStatus::PostIncidentAnalysis
        )
    }

    /// Check if status is active (requires attention)
    ///
    /// # Returns
    /// `true` if status requires active attention
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatus;
    ///
    /// assert!(ThreatStatus::Active.is_active());
    /// assert!(!ThreatStatus::Resolved.is_active());
    /// ```
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

    /// Get valid next statuses
    ///
    /// # Returns
    /// Vector of valid next status transitions
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatus;
    ///
    /// let next_statuses = ThreatStatus::New.valid_next_statuses();
    /// assert!(next_statuses.contains(&ThreatStatus::Investigating));
    /// assert!(next_statuses.contains(&ThreatStatus::FalsePositive));
    /// ```
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
                ThreatStatus::FalsePositive,
                ThreatStatus::Escalated,
                ThreatStatus::Active,
            ],
            ThreatStatus::Confirmed => vec![
                ThreatStatus::Active,
                ThreatStatus::Escalated,
                ThreatStatus::Contained,
                ThreatStatus::Mitigated,
            ],
            ThreatStatus::Active => vec![
                ThreatStatus::Contained,
                ThreatStatus::Mitigated,
                ThreatStatus::Escalated,
                ThreatStatus::Eradicated,
            ],
            ThreatStatus::Acknowledged => vec![
                ThreatStatus::Investigating,
                ThreatStatus::Active,
                ThreatStatus::Suppressed,
            ],
            ThreatStatus::Contained => vec![
                ThreatStatus::Mitigated,
                ThreatStatus::Eradicated,
                ThreatStatus::Recovery,
            ],
            ThreatStatus::Mitigated => vec![
                ThreatStatus::Resolved,
                ThreatStatus::Recovery,
                ThreatStatus::Eradicated,
            ],
            ThreatStatus::Eradicated => vec![
                ThreatStatus::Recovery,
                ThreatStatus::Resolved,
                ThreatStatus::PostIncidentAnalysis,
            ],
            ThreatStatus::Recovery => {
                vec![ThreatStatus::Resolved, ThreatStatus::PostIncidentAnalysis]
            }
            ThreatStatus::Escalated => vec![
                ThreatStatus::Active,
                ThreatStatus::Contained,
                ThreatStatus::Mitigated,
            ],
            // Terminal states
            ThreatStatus::Resolved
            | ThreatStatus::FalsePositive
            | ThreatStatus::Suppressed
            | ThreatStatus::PostIncidentAnalysis => vec![],
        }
    }

    /// Check if transition to new status is valid
    ///
    /// # Arguments
    /// * `new_status` - Target status for transition
    ///
    /// # Returns
    /// `true` if transition is valid
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatus;
    ///
    /// let current = ThreatStatus::New;
    /// assert!(current.can_transition_to(&ThreatStatus::Investigating));
    /// assert!(!current.can_transition_to(&ThreatStatus::Resolved));
    /// ```
    pub fn can_transition_to(&self, new_status: &ThreatStatus) -> bool {
        self.valid_next_statuses().contains(new_status)
    }

    /// Get priority level for this status
    ///
    /// # Returns
    /// Priority level (1-5, where 5 is highest priority)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatStatus;
    ///
    /// assert_eq!(ThreatStatus::Active.priority_level(), 5);
    /// assert_eq!(ThreatStatus::Resolved.priority_level(), 1);
    /// ```
    pub fn priority_level(&self) -> u8 {
        match self {
            ThreatStatus::Resolved | ThreatStatus::FalsePositive | ThreatStatus::Suppressed => 1,
            ThreatStatus::PostIncidentAnalysis | ThreatStatus::Recovery => 2,
            ThreatStatus::New | ThreatStatus::Acknowledged => 3,
            ThreatStatus::Investigating
            | ThreatStatus::Confirmed
            | ThreatStatus::Contained
            | ThreatStatus::Mitigated
            | ThreatStatus::Eradicated => 4,
            ThreatStatus::Active | ThreatStatus::Escalated => 5,
        }
    }
}

impl MitigationStep {
    /// Create a new mitigation step
    ///
    /// # Arguments
    /// * `description` - Description of the mitigation step
    /// * `executor` - Who or what executed the step
    ///
    /// # Returns
    /// A new `MitigationStep` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MitigationStep;
    ///
    /// let step = MitigationStep::new(
    ///     "Blocked malicious IP".to_string(),
    ///     "firewall-system".to_string()
    /// );
    /// ```
    pub fn new(description: String, executor: String) -> Self {
        Self {
            description,
            executed_at: Utc::now(),
            success: false,
            details: None,
            executor,
        }
    }

    /// Mark step as successful
    ///
    /// # Arguments
    /// * `details` - Optional additional details
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MitigationStep;
    ///
    /// let mut step = MitigationStep::new(
    ///     "Block IP".to_string(),
    ///     "firewall".to_string()
    /// );
    /// step.mark_success(Some("IP 1.2.3.4 blocked".to_string()));
    ///
    /// assert!(step.success);
    /// ```
    pub fn mark_success(&mut self, details: Option<String>) {
        self.success = true;
        self.details = details;
    }

    /// Mark step as failed
    ///
    /// # Arguments
    /// * `details` - Optional failure details
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MitigationStep;
    ///
    /// let mut step = MitigationStep::new(
    ///     "Block IP".to_string(),
    ///     "firewall".to_string()
    /// );
    /// step.mark_failure(Some("Firewall unreachable".to_string()));
    ///
    /// assert!(!step.success);
    /// ```
    pub fn mark_failure(&mut self, details: Option<String>) {
        self.success = false;
        self.details = details;
    }

    /// Get step duration in minutes
    ///
    /// # Returns
    /// Duration from execution to now in minutes
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MitigationStep;
    /// use chrono::{Utc, Duration};
    ///
    /// let mut step = MitigationStep::new(
    ///     "Block IP".to_string(),
    ///     "firewall".to_string()
    /// );
    /// step.executed_at = Utc::now() - Duration::minutes(15);
    ///
    /// assert_eq!(step.duration_minutes(), 15);
    /// ```
    pub fn duration_minutes(&self) -> i64 {
        let now = Utc::now();
        (now - self.executed_at).num_minutes()
    }

    /// Check if step is recent
    ///
    /// # Arguments
    /// * `minutes` - Maximum age in minutes to consider recent
    ///
    /// # Returns
    /// `true` if step was executed within the specified time
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::MitigationStep;
    ///
    /// let step = MitigationStep::new(
    ///     "Block IP".to_string(),
    ///     "firewall".to_string()
    /// );
    ///
    /// assert!(step.is_recent(60)); // Within last hour
    /// ```
    pub fn is_recent(&self, minutes: i64) -> bool {
        self.duration_minutes() <= minutes
    }
}

impl ResponseAction {
    /// Get the severity level of this response action
    ///
    /// # Returns
    /// Severity level (1-5, where 5 is most severe)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ResponseAction;
    ///
    /// let action = ResponseAction::BlockIp("1.2.3.4".to_string());
    /// assert_eq!(action.severity_level(), 3);
    /// ```
    pub fn severity_level(&self) -> u8 {
        match self {
            ResponseAction::LogAlert(_) => 1,
            ResponseAction::NotifyAdmin(_) => 2,
            ResponseAction::BlockIp(_) => 3,
            ResponseAction::QuarantineUser(_) => 4,
            ResponseAction::IsolateSystem(_) => 5,
        }
    }

    /// Check if action is reversible
    ///
    /// # Returns
    /// `true` if action can be easily reversed
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ResponseAction;
    ///
    /// let action = ResponseAction::BlockIp("1.2.3.4".to_string());
    /// assert!(action.is_reversible());
    /// ```
    pub fn is_reversible(&self) -> bool {
        matches!(
            self,
            ResponseAction::BlockIp(_)
                | ResponseAction::QuarantineUser(_)
                | ResponseAction::IsolateSystem(_)
        )
    }

    /// Get the target of this action
    ///
    /// # Returns
    /// Target identifier (IP, user, system, etc.)
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ResponseAction;
    ///
    /// let action = ResponseAction::BlockIp("1.2.3.4".to_string());
    /// assert_eq!(action.get_target(), "1.2.3.4");
    /// ```
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

// Display implementations

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
