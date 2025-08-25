// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Threat intelligence indicators
///
/// This module contains types and functionality for managing threat indicators
/// (indicators of compromise - IOCs) used in threat intelligence feeds.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::threat::types::core::{ThreatSeverity, ThreatType};
/// Threat indicator structure
/// Represents an indicator of compromise (IOC) with
/// metadata, confidence, and attribution information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatIndicator {
    /// Indicator type
    pub indicator_type: IndicatorType,
    /// Indicator value
    pub value: String,
    /// Confidence level (0.0 to 1.0)
    pub confidence: f64,
    /// Confidence score (alias for compatibility)
    pub confidence_score: f64,
    /// First seen timestamp
    pub first_seen: DateTime<Utc>,
    /// Last seen timestamp
    pub last_seen: DateTime<Utc>,
    /// Associated threat types
    pub threat_types: Vec<ThreatType>,
    /// Primary threat type for compatibility
    pub threat_type: ThreatType,
    /// Threat severity
    pub severity: ThreatSeverity,
    /// Indicator description
    pub description: String,
    /// Associated threat actor
    pub threat_actor: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}
/// Indicator type enumeration
/// Categorizes different types of indicators of compromise
/// that can be tracked and detected.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndicatorType {
    /// IP address indicator
    IpAddress,
    /// Domain name indicator
    DomainName,
    /// URL indicator
    Url,
    /// File hash indicator
    FileHash,
    /// Email address indicator
    EmailAddress,
    /// Windows registry key indicator
    RegistryKey,
    /// Process mutex indicator
    Mutex,
    /// User agent string indicator
    UserAgent,
    /// Digital certificate indicator
    Certificate,
    /// Process name indicator
    ProcessName,
    /// Network traffic pattern indicator
    NetworkPattern,
    /// Behavioral pattern indicator
    BehaviorPattern,
    /// Custom indicator type
    Custom(String),}


impl Default for ThreatIndicator {}


    fn default() -> Self {
        Self {
            indicator_type: IndicatorType::IpAddress,
            value: String::new(),
            confidence: 0.5,
            confidence_score: 0.5,
            first_seen: Utc::now(),
            last_seen: Utc::now(),
            threat_types: vec![],
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Medium,
            description: String::new(),
            threat_actor: None,
            tags: vec![],
        }
    }
impl ThreatIndicator {
    /// Create a new threat indicator
    ///
    /// # Arguments
    /// * `indicator_type` - Type of indicator
    /// * `value` - Indicator value
    /// * `confidence` - Confidence level (0.0 to 1.0)
    /// # Returns
    /// A new `ThreatIndicator` instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::{ThreatIndicator, IndicatorType};
    /// let indicator = ThreatIndicator::new(
    ///     IndicatorType::IpAddress,
    ///     "192.168.1.100".to_string(),
    ///     0.9
    /// );
    /// ```
    pub fn new(indicator_type: IndicatorType, value: String, confidence: f64) -> Self {
            indicator_type,
            value,
            confidence,
            confidence_score: confidence,
    /// Add threat type association
    /// * `threat_type` - Threat type to associate
    /// use beardog::threat::types::{ThreatIndicator, IndicatorType, ThreatType};
    /// let mut indicator = ThreatIndicator::new(
    /// indicator.add_threat_type(ThreatType::Malware);
    /// assert_eq!(indicator.threat_types.len(), 1);
    pub fn add_threat_type(&mut self, threat_type: ThreatType) {
        if !self.threat_types.contains(&threat_type) {
            self.threat_types.push(threat_type);
    /// Add tag
    /// * `tag` - Tag to add
    /// indicator.add_tag("malware".to_string());
    /// indicator.add_tag("botnet".to_string());
    /// assert_eq!(indicator.tags.len(), 2);}


    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
    /// Update last seen timestamp
    /// use chrono::Utc;
    /// let old_timestamp = indicator.last_seen;
    /// indicator.update_last_seen();
    /// assert!(indicator.last_seen > old_timestamp);
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    /// Check if indicator is high confidence
    /// `true` if confidence is above 0.7
    /// let high_confidence = ThreatIndicator::new(
    /// let low_confidence = ThreatIndicator::new(
    ///     "192.168.1.101".to_string(),
    ///     0.3
    /// assert!(high_confidence.is_high_confidence());
    /// assert!(!low_confidence.is_high_confidence());}


    pub fn is_high_confidence(&self) -> bool {
        self.confidence > 0.7
    /// Check if indicator is recent
    /// * `hours` - Number of hours to consider as recent
    /// `true` if last seen within the specified hours
    /// assert!(indicator.is_recent(24)); // Within last 24 hours
    pub fn is_recent(&self, hours: i64) -> bool {
        let now = Utc::now();
        (now - self.last_seen).num_hours() <= hours
    /// Get indicator age in hours
    /// Hours since first seen
    /// let age = indicator.age_hours();
    /// assert!(age >= 0);}


    pub fn age_hours(&self) -> i64 {
        (now - self.first_seen).num_hours()
impl IndicatorType {
    /// Check if indicator type is network-related
    /// `true` if indicator type relates to network activity
    /// use beardog::threat::types::IndicatorType;
    /// assert!(IndicatorType::IpAddress.is_network_related());
    /// assert!(IndicatorType::DomainName.is_network_related());
    /// assert!(!IndicatorType::FileHash.is_network_related());}


    pub fn is_network_related(&self) -> bool {
        matches!(
            self,
            IndicatorType::IpAddress
                | IndicatorType::DomainName
                | IndicatorType::Url
                | IndicatorType::NetworkPattern
        )
    /// Check if indicator type is file-related
    /// `true` if indicator type relates to files or processes
    /// assert!(IndicatorType::FileHash.is_file_related());
    /// assert!(IndicatorType::ProcessName.is_file_related());
    /// assert!(!IndicatorType::IpAddress.is_file_related());}


    pub fn is_file_related(&self) -> bool {
            IndicatorType::FileHash
                | IndicatorType::ProcessName
                | IndicatorType::RegistryKey
                | IndicatorType::Mutex
    /// Get validation regex pattern
    /// Regex pattern for validating indicator values
    /// let pattern = IndicatorType::IpAddress.validation_pattern();
    /// // Returns IPv4 regex pattern
    pub fn validation_pattern(&self) -> &'static str {
        match self {
            IndicatorType::IpAddress => r"^(?:[0-9]{1,3}\.){3}[0-9]{1,3}$",
            IndicatorType::DomainName => {
                r"^[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9\-]{0,61}[a-zA-Z0-9])?)*$"
            }
            IndicatorType::Url => r"^https?://[^\s/$.?#].[^\s]*$",
            IndicatorType::FileHash => r"^[a-fA-F0-9]{32}$|^[a-fA-F0-9]{40}$|^[a-fA-F0-9]{64}$",
            IndicatorType::EmailAddress => r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$",
            _ => r".*", // Default: accept anything
impl std::fmt::Display for IndicatorType {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            IndicatorType::IpAddress => write!(f, "IP Address"),
            IndicatorType::DomainName => write!(f, "Domain Name"),
            IndicatorType::Url => write!(f, "URL"),
            IndicatorType::FileHash => write!(f, "File Hash"),
            IndicatorType::EmailAddress => write!(f, "Email Address"),
            IndicatorType::RegistryKey => write!(f, "Registry Key"),
            IndicatorType::Mutex => write!(f, "Mutex"),
            IndicatorType::UserAgent => write!(f, "User Agent"),
            IndicatorType::Certificate => write!(f, "Certificate"),
            IndicatorType::ProcessName => write!(f, "Process Name"),
            IndicatorType::NetworkPattern => write!(f, "Network Pattern"),
            IndicatorType::BehaviorPattern => write!(f, "Behavior Pattern"),
            IndicatorType::Custom(custom) => write!(f, "Custom: {custom}"),
