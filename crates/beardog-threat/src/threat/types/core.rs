// SPDX-License-Identifier: AGPL-3.0-or-later

// Core Threat Types - Modern Implementation
//
// **MODERNIZED**: Clean, production-ready core types for the BearDog threat detection system.

// Removed unused import - ThreatIntelligenceFeed is defined in mod.rs
use crate::threat::types::IndicatorType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Threat severity levels
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatSeverity {
    /// Represents low variant
    Low,
    /// Represents medium variant
    Medium,
    /// Represents high variant
    High,
    /// Represents critical variant
    Critical,
    /// Unknown or undefined state
    Unknown,
}

/// Threat types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of threat
pub enum ThreatType {
    /// Represents malware variant
    Malware,
    /// Currently phishing
    Phishing,
    /// Represents ransomware variant
    Ransomware,
    /// Represents data breach variant
    DataBreach,
    /// Represents denial of service variant
    DenialOfService,
    /// Represents intrusion variant
    Intrusion,
    /// Represents reconnaissance variant
    Reconnaissance,
    /// Unknown or undefined state
    Unknown,
}

/// Threat status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatStatus {
    /// Active or enabled state
    Active,
    /// State indicating mitigated
    Mitigated,
    /// State indicating resolved
    Resolved,
    /// Currently investigating
    Investigating,
    /// State indicating dismissed
    Dismissed,
}

/// Threat source
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatSource {
    /// Represents external variant
    External,
    /// Represents internal variant
    Internal,
    /// Represents network variant
    Network,
    /// Represents email variant
    Email,
    /// Represents web variant
    Web,
    /// Unknown or undefined state
    Unknown,
}

/// Threat target
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatTarget {
    /// Represents system variant
    System,
    /// Represents user variant
    User,
    /// Represents network variant
    Network,
    /// Represents application variant
    Application,
    /// Represents data variant
    Data,
    /// Unknown or undefined state
    Unknown,
}

/// Detection method
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionMethod {
    /// Represents signature variant
    Signature,
    /// Represents behavioral variant
    Behavioral,
    /// Currently machinelearning
    MachineLearning,
    /// Represents heuristic variant
    Heuristic,
    /// Represents reputation variant
    Reputation,
    /// Unknown or undefined state
    Unknown,
}

/// Threat indicator from intelligence feeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    pub id: String,
    /// The indicator type value
    pub indicator_type: IndicatorType,
    /// The value value
    pub value: String,
    pub confidence: f64,
    /// The severity value
    pub severity: String,
    /// The source value
    pub source: String,
    pub timestamp: DateTime<Utc>,
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Optional expires at
    pub expires_at: Option<DateTime<Utc>>,
}

// Display implementation moved to mod.rs to avoid conflicts

/// Threat indicator from intelligence feeds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIntelligenceIndicator {
    pub id: String,
    /// Type of indicator (IP, domain, hash, etc.)
    /// The indicator type value
    pub indicator_type: IndicatorType,
    /// The actual indicator value
    /// The value value
    pub value: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Severity level
    /// The severity value
    pub severity: String,
    /// Human-readable description
    /// The description value
    pub description: String,
    /// Associated tags
    /// Collection of tags
    pub tags: Vec<String>,
    /// First time this indicator was seen
    /// The first seen value
    pub first_seen: chrono::DateTime<chrono::Utc>,
    /// Last time this indicator was seen
    /// The last seen value
    pub last_seen: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    /// The metadata value
    pub metadata: std::collections::HashMap<String, String>,
}
