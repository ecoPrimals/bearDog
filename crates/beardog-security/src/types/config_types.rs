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


/// Security Configuration Types - Canonical System
///
/// This module provides unified security configuration types from the canonical system.
/// All security configuration should use these types for consistency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::{BearDogError, BearDogResult};

// Re-export canonical security configuration types
pub use beardog_types::canonical::configuration::security::{
    SecurityConfig as UnifiedSecurityConfig,
    MfaConfig,
    TotpConfig,
    BackupCodesConfig,
    PasswordPolicyConfig,
    RateLimitConfig,
    SessionConfig,
    SessionStorage,
};
