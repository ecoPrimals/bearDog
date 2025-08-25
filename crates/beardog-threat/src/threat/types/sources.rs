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


/// Threat source and target types
///
/// This module contains types for representing threat sources, targets, and their
/// classifications and attributes.
/// ## Features
/// - Threat source information and classification
/// - Target asset information and criticality
/// - Geographic location data
/// - Asset protection levels
/// - Source reputation and threat actor information
/// ## Example
/// ```rust
/// use beardog::threat::types::{ThreatSource, ThreatTarget, GeoLocation, SourceClassification};
/// let source = ThreatSource {
///     ip_address: Some("192.168.1.100".to_string()),
///     hostname: Some("malicious-host".to_string()),
///     classification: SourceClassification::Malicious,
///     reputation_score: 0.1,
///     threat_actor: Some("APT29".to_string()),
///     ..Default::default()
/// };
/// let target = ThreatTarget {
///     resource_id: "web-server-01".to_string(),
///     resource_type: "web-server".to_string(),
///     criticality: AssetCriticality::High,
/// ```
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Threat source information
/// Represents the source of a threat, including network information,
/// geographic data, and reputation scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatSource {
    /// Source identifier (missing field added)
    pub id: String,
    /// Source IP address
    pub ip_address: Option<String>,
    /// Source hostname
    pub hostname: Option<String>,
    /// Geographic location
    pub geolocation: Option<GeoLocation>,
    /// User agent information
    pub user_agent: Option<String>,
    /// Source reputation score (0.0 = malicious, 1.0 = trusted)
    pub reputation_score: f64,
    /// Known threat actor
    pub threat_actor: Option<String>,
    /// Source classification
    pub classification: SourceClassification,
    /// Confidence score (missing field added)
    pub confidence_score: f64,
    /// First time this source was seen (missing field added)  
    pub first_seen: Option<DateTime<Utc>>,
    /// Last time this source was seen (missing field added)
    pub last_seen: Option<DateTime<Utc>>,
    /// Threat score (missing field added)
    pub threat_score: f64,
}
/// Threat target information
/// Represents the target of a threat, including resource identification,
/// criticality assessment, and protection level.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ThreatTarget {
    /// Target identifier
    /// Target resource ID
    pub resource_id: String,
    /// Resource type
    pub resource_type: String,
    /// Target node ID
    pub node_id: Option<String>,
    /// Target user account
    pub user_account: Option<String>,
    /// Asset criticality
    pub criticality: AssetCriticality,
    /// Protection level
    pub protection_level: ProtectionLevel,
    /// Target IP address (missing field added)
    /// Target hostname (missing field added)
    /// Target service name (missing field added)
    pub service: Option<String>,
    /// Target port number (missing field added)
    pub port: Option<u16>,
    /// Target protocol (missing field added)
    pub protocol: Option<String>,
/// Geographic location information
/// Provides geographic context for threat sources, including
/// detection of anonymization services.
pub struct GeoLocation {
    /// Country name or code
    pub country: String,
    /// State or region within the country
    pub region: Option<String>,
    /// City name
    pub city: Option<String>,
    /// Latitude coordinate
    pub latitude: Option<f64>,
    /// Longitude coordinate
    pub longitude: Option<f64>,
    /// Whether this is a Tor exit node
    pub is_tor_exit: bool,
    /// Whether this is a VPN endpoint
    pub is_vpn: bool,
    /// Whether this is a proxy server
    pub is_proxy: bool,
/// Source classification levels
/// Categorizes threat sources based on their reputation and
/// threat intelligence information.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum SourceClassification {
    /// Trusted source with high confidence
    Trusted,
    /// Neutral source with no specific reputation
    Neutral,
    /// Suspicious source requiring monitoring
    Suspicious,
    /// Malicious source confirmed as threat
    Malicious,
    /// Known malicious source from threat intelligence
    KnownMalicious,
    /// Blacklisted source blocked by security policies
    Blacklisted,
    /// Unknown source with no classification data
    #[default]
    Unknown,
/// Asset criticality levels
/// Defines the business criticality of assets to prioritize
/// protection and response efforts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Default)]}


pub enum AssetCriticality {
    /// Low criticality - minimal impact if compromised
    Low,
    /// Medium criticality - moderate impact if compromised
    Medium,
    /// High criticality - significant impact if compromised
    High,
    /// Critical assets - severe impact if compromised
    Critical,
    /// Mission-critical assets - catastrophic impact if compromised
    Mission,
/// Protection level applied to assets
/// Indicates the level of security protection applied to
/// a particular asset or resource.
pub enum ProtectionLevel {
    /// No protection applied
    None,
    /// Basic protection with minimal security measures
    Basic,
    /// Standard protection with typical security controls
    Standard,
    /// Enhanced protection with additional security layers
    Enhanced,
    /// Maximum protection with all available security measures
    Maximum,
// Default implementations}


impl Default for ThreatSource {}


    fn default() -> Self {
        Self {
            id: String::new(),
            ip_address: None,
            hostname: None,
            geolocation: None,
            user_agent: None,
            reputation_score: 0.5,
            threat_actor: None,
            classification: SourceClassification::default(),
            confidence_score: 0.5,
            first_seen: None,
            last_seen: None,
            threat_score: 0.0,
        }
    }
// Utility implementations
impl ThreatSource {
    /// Create a new threat source with minimal information
    ///
    /// # Arguments
    /// * `ip_address` - Optional IP address of the source
    /// # Returns
    /// A new `ThreatSource` instance
    /// # Example
    /// ```rust
    /// use beardog::threat::types::ThreatSource;
    /// let source = ThreatSource::new(Some("192.168.1.100".to_string()));
    /// assert_eq!(source.ip_address, Some("192.168.1.100".to_string()));
    /// ```}


    pub fn new(ip_address: Option<String>) -> Self {
            ip_address,
            ..Default::default()
    /// Check if source is considered trustworthy
    /// `true` if source is trusted or neutral with good reputation
    /// use beardog::threat::types::{ThreatSource, SourceClassification};
    /// let mut source = ThreatSource::default();
    /// source.classification = SourceClassification::Trusted;
    /// source.reputation_score = 0.9;
    /// assert!(source.is_trustworthy());
    pub fn is_trustworthy(&self) -> bool {
        matches!(self.classification, SourceClassification::Trusted)
            || (matches!(self.classification, SourceClassification::Neutral)
                && self.reputation_score > 0.7)
    /// Check if source is malicious
    /// `true` if source is classified as malicious or blacklisted
    /// source.classification = SourceClassification::Malicious;
    /// assert!(source.is_malicious());}


    pub fn is_malicious(&self) -> bool {
        matches!(
            self.classification,
            SourceClassification::Malicious | SourceClassification::Blacklisted
        )
    /// Check if source uses anonymization services
    /// `true` if source uses Tor, VPN, or proxy services
    /// use beardog::threat::types::{ThreatSource, GeoLocation};
    /// source.geolocation = Some(GeoLocation {
    ///     country: "US".to_string(),
    ///     is_tor_exit: true,
    ///     ..Default::default()
    /// });
    /// assert!(source.is_anonymized());
    pub fn is_anonymized(&self) -> bool {
        self.geolocation
            .as_ref()
            .is_some_and(|geo| geo.is_tor_exit || geo.is_vpn || geo.is_proxy)
impl ThreatTarget {
    /// Create a new threat target
    /// * `resource_id` - Unique identifier for the target resource
    /// * `resource_type` - Type of resource being targeted
    /// A new `ThreatTarget` instance
    /// use beardog::threat::types::ThreatTarget;
    /// let target = ThreatTarget::new("web-server-01".to_string(), "web-server".to_string());
    /// assert_eq!(target.resource_id, "web-server-01");
    /// assert_eq!(target.resource_type, "web-server");}


    pub fn new(resource_id: String, resource_type: String) -> Self {
            id: String::new(), // New id
            resource_id,
            resource_type,
            node_id: None,
            user_account: None,
            criticality: AssetCriticality::Medium,
            protection_level: ProtectionLevel::Standard,
            service: None,
            port: None,
            protocol: None,
    /// Check if target is high-value
    /// `true` if target has high or critical criticality
    /// use beardog::threat::types::{ThreatTarget, AssetCriticality};
    /// let mut target = ThreatTarget::default();
    /// target.criticality = AssetCriticality::High;
    /// assert!(target.is_high_value());
    pub fn is_high_value(&self) -> bool {
            self.criticality,
            AssetCriticality::High | AssetCriticality::Critical | AssetCriticality::Mission
    /// Check if target is well-protected
    /// `true` if target has enhanced or maximum protection
    /// use beardog::threat::types::{ThreatTarget, ProtectionLevel};
    /// target.protection_level = ProtectionLevel::Enhanced;
    /// assert!(target.is_well_protected());
    pub fn is_well_protected(&self) -> bool {
            self.protection_level,
            ProtectionLevel::Enhanced | ProtectionLevel::Maximum
    /// Get risk score for this target
    /// Calculates risk based on criticality and protection level.
    /// Risk score (0.0 to 1.0)
    /// use beardog::threat::types::{ThreatTarget, AssetCriticality, ProtectionLevel};
    /// target.criticality = AssetCriticality::Critical;
    /// target.protection_level = ProtectionLevel::Basic;
    /// assert!(target.risk_score() > 0.7);
    pub fn risk_score(&self) -> f64 {
        let criticality_score = match self.criticality {
            AssetCriticality::Low => 0.2,
            AssetCriticality::Medium => 0.4,
            AssetCriticality::High => 0.6,
            AssetCriticality::Critical => 0.8,
            AssetCriticality::Mission => 1.0,
        };
        let protection_modifier = match self.protection_level {
            ProtectionLevel::None => 1.0,
            ProtectionLevel::Basic => 0.8,
            ProtectionLevel::Standard => 0.6,
            ProtectionLevel::Enhanced => 0.4,
            ProtectionLevel::Maximum => 0.2,
        criticality_score * protection_modifier}


impl GeoLocation {
    /// Create a new geographic location
    /// * `country` - Country name or code
    /// A new `GeoLocation` instance
    /// use beardog::threat::types::GeoLocation;
    /// let geo = GeoLocation::new("US".to_string());
    /// assert_eq!(geo.country, "US");}


    pub fn new(country: String) -> Self {
            country,
    /// Check if location indicates high-risk geography
    /// `true` if location is from a high-risk country
    /// let geo = GeoLocation::new("CN".to_string());
    /// // Implementation would check against threat intelligence}


    pub fn is_high_risk_country(&self) -> bool {
        // This would typically check against threat intelligence feeds
        // For now, we'll use a simple list of commonly flagged countries
        let high_risk_countries = ["CN", "RU", "KP", "IR", "SY", "PK", "BD", "ID", "VN", "IN"];
        high_risk_countries.contains(&self.country.as_str())
impl SourceClassification {
    /// Get numeric reputation score for classification
    /// Reputation score (0.0 to 1.0)
    /// use beardog::threat::types::SourceClassification;
    /// assert_eq!(SourceClassification::Trusted.reputation_score(), 1.0);
    /// assert_eq!(SourceClassification::Malicious.reputation_score(), 0.0);}


    pub fn reputation_score(&self) -> f64 {
        match self {
            SourceClassification::Trusted => 1.0,
            SourceClassification::Neutral => 0.5,
            SourceClassification::Suspicious => 0.3,
            SourceClassification::Malicious => 0.0,
            SourceClassification::KnownMalicious => 0.0,
            SourceClassification::Blacklisted => 0.0,
            SourceClassification::Unknown => 0.5,}


impl AssetCriticality {
    /// Get numeric criticality score
    /// Criticality score (0.0 to 1.0)
    /// use beardog::threat::types::AssetCriticality;
    /// assert_eq!(AssetCriticality::Mission.criticality_score(), 1.0);
    /// assert_eq!(AssetCriticality::Low.criticality_score(), 0.2);}


    pub fn criticality_score(&self) -> f64 {
impl ProtectionLevel {
    /// Get numeric protection score
    /// Protection score (0.0 to 1.0)
    /// use beardog::threat::types::ProtectionLevel;
    /// assert_eq!(ProtectionLevel::Maximum.protection_score(), 1.0);
    /// assert_eq!(ProtectionLevel::None.protection_score(), 0.0);}


    pub fn protection_score(&self) -> f64 {
            ProtectionLevel::None => 0.0,
            ProtectionLevel::Basic => 0.25,
            ProtectionLevel::Standard => 0.5,
            ProtectionLevel::Enhanced => 0.75,
            ProtectionLevel::Maximum => 1.0,
// Display implementations
impl std::fmt::Display for SourceClassification {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            SourceClassification::Trusted => write!(f, "Trusted"),
            SourceClassification::Neutral => write!(f, "Neutral"),
            SourceClassification::Suspicious => write!(f, "Suspicious"),
            SourceClassification::Malicious => write!(f, "Malicious"),
            SourceClassification::KnownMalicious => write!(f, "Known Malicious"),
            SourceClassification::Blacklisted => write!(f, "Blacklisted"),
            SourceClassification::Unknown => write!(f, "Unknown"),
impl std::fmt::Display for AssetCriticality {
            AssetCriticality::Low => write!(f, "Low"),
            AssetCriticality::Medium => write!(f, "Medium"),
            AssetCriticality::High => write!(f, "High"),
            AssetCriticality::Critical => write!(f, "Critical"),
            AssetCriticality::Mission => write!(f, "Mission-Critical"),}


impl std::fmt::Display for ProtectionLevel {
            ProtectionLevel::None => write!(f, "None"),
            ProtectionLevel::Basic => write!(f, "Basic"),
            ProtectionLevel::Standard => write!(f, "Standard"),
            ProtectionLevel::Enhanced => write!(f, "Enhanced"),
            ProtectionLevel::Maximum => write!(f, "Maximum"),
