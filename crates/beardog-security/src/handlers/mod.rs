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


/// Security Handlers Module
///
/// **CANONICAL SECURITY HANDLERS** - Unified security operation handlers
/// This module provides the core security handlers that implement the canonical
/// security traits and provide unified security operations across BearDog.

use beardog_errors::{BearDogError, BearDogResult};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Import security types
use super::types::*;
// Core handler modules that exist
pub mod metrics_collection;
pub mod trait_implementation;
// MIGRATION COMPLETE: Removed references to deleted modules
// - account_management: Functionality moved to canonical types
// - audit_management: Functionality moved to canonical types
// - maintenance: Functionality moved to canonical types
// - mfa_handling: Functionality moved to canonical types
// - rate_limiting: Functionality moved to canonical types
// - session_management: Functionality moved to canonical types
// - threat_analysis: Functionality moved to canonical types
// Re-exports
pub use trait_implementation::*;
