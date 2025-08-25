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


/// Threat intelligence enumeration types
///
/// This module contains enumeration types used throughout the threat intelligence
/// system, including feed types, update frequencies, and status indicators.
use serde::{Deserialize, Serialize};

/// Feed type enumeration
/// Categorizes different types of threat intelligence feeds
/// based on the type of data they provide.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedType {
    /// IP address reputation feed
    IpReputation,
    /// Domain reputation feed
    DomainReputation,
    /// Known malicious file hashes
    FileHashes,
    /// URL blacklist feed
    UrlBlacklist,
    /// Malware signature definitions
    MalwareSignatures,
    /// Attack pattern descriptions
    AttackPatterns,
    /// Known threat actor profiles
    ThreatActors,
    /// Vulnerability information feed
    Vulnerabilities,
}
/// Update frequency enumeration
/// Defines how frequently a threat intelligence feed
/// should be updated with new data.
pub enum UpdateFrequency {
    /// Real-time continuous updates
    RealTime,
    /// Updated every hour
    Hourly,
    /// Updated daily
    Daily,
    /// Updated weekly
    Weekly,
    /// Updated monthly
    Monthly,
/// Feed status enumeration
/// Tracks the current operational status of a
/// threat intelligence feed.}


pub enum FeedStatus {
    /// Feed is active and operational
    Active,
    /// Feed is inactive or disabled
    Inactive,
    /// Feed encountered an error
    Error,
    /// Feed is currently being updated
    Updating,
impl std::fmt::Display for FeedType {}


    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeedType::IpReputation => write!(f, "IP Reputation"),
            FeedType::DomainReputation => write!(f, "Domain Reputation"),
            FeedType::FileHashes => write!(f, "File Hashes"),
            FeedType::UrlBlacklist => write!(f, "URL Blacklist"),
            FeedType::MalwareSignatures => write!(f, "Malware Signatures"),
            FeedType::AttackPatterns => write!(f, "Attack Patterns"),
            FeedType::ThreatActors => write!(f, "Threat Actors"),
            FeedType::Vulnerabilities => write!(f, "Vulnerabilities"),
        }
    }
impl std::fmt::Display for UpdateFrequency {
            UpdateFrequency::RealTime => write!(f, "Real-time"),
            UpdateFrequency::Hourly => write!(f, "Hourly"),
            UpdateFrequency::Daily => write!(f, "Daily"),
            UpdateFrequency::Weekly => write!(f, "Weekly"),
            UpdateFrequency::Monthly => write!(f, "Monthly"),}


impl std::fmt::Display for FeedStatus {
            FeedStatus::Active => write!(f, "Active"),
            FeedStatus::Inactive => write!(f, "Inactive"),
            FeedStatus::Error => write!(f, "Error"),
            FeedStatus::Updating => write!(f, "Updating"),
