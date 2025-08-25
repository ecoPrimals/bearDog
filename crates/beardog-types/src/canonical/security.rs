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


/// # Canonical Security Types - Minimal Version
///
/// **TEMPORARY MINIMAL IMPLEMENTATION** for build stability
/// This provides essential security types while syntax issues are resolved.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL** Security Context
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityContext {
    pub user_id: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub session_id: Option<String>,
    pub authenticated: bool,
    pub authorization_level: AuthorizationLevel,
}

/// **CANONICAL** Authorization Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorizationLevel {
    None,
    Read,
    Write,
    Admin,
    Root,
}

impl Default for AuthorizationLevel {
    fn default() -> Self {
        Self::None
    }
}

/// **CANONICAL** Security Flags
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityFlags {
    pub encrypted: bool,
    pub authenticated: bool,
    pub authorized: bool,
    pub audited: bool,
}

/// **CANONICAL** Policy Decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Conditional,
}

impl Default for PolicyDecision {
    fn default() -> Self {
        Self::Deny
    }
}

/// **CANONICAL** Risk Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl Default for RiskLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// **CANONICAL** Security Audit Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditEvent {
    pub event_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub user_id: Option<String>,
    pub resource: Option<String>,
    pub action: String,
    pub result: SecurityEventResult,
    pub details: HashMap<String, String>,
}

/// **CANONICAL** Security Event Type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEventType {
    Authentication,
    Authorization,
    DataAccess,
    ConfigurationChange,
    SystemEvent,
}

/// **CANONICAL** Security Event Result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityEventResult {
    Success,
    Failure,
    Warning,
}

/// **CANONICAL** Threat Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for ThreatLevel {
    fn default() -> Self {
        Self::None
    }
}

/// **CANONICAL** Compliance Level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceLevel {
    NonCompliant,
    PartiallyCompliant,
    Compliant,
    FullyCompliant,
}

impl Default for ComplianceLevel {
    fn default() -> Self {
        Self::NonCompliant
    }
}

/// **CANONICAL** Session Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionToken {
    pub token: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub permissions: Vec<String>,
    pub active: bool,
}

impl Default for SessionToken {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            token: String::new(),
            user_id: String::new(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            permissions: Vec::new(),
            active: false,
        }
    }
}

/// **CANONICAL** Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub active: bool,
    pub mfa_verified: bool,
}

impl Default for Session {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            session_id: String::new(),
            user_id: String::new(),
            created_at: now,
            expires_at: now + chrono::Duration::hours(24),
            last_activity: now,
            active: false,
            mfa_verified: false,
        }
    }
}
