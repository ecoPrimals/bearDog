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


/// # Integration Policies Configuration - Canonical
///
/// **UNIFIED INTEGRATION POLICIES CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Integration Policies Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IntegrationPoliciesConfig {
    pub circuit_breaker: IntegrationCircuitBreaker,
    pub rate_limiting: IntegrationRateLimiting,
    pub retry_policy: NotificationRetryPolicy,
}


/// **CANONICAL** Integration Circuit Breaker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCircuitBreaker {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout: Duration,
}

impl Default for IntegrationCircuitBreaker {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
        }
    }
}

/// **CANONICAL** Integration Rate Limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationRateLimiting {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub burst_capacity: u32,
}

impl Default for IntegrationRateLimiting {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_minute: 100,
            burst_capacity: 150,
        }
    }
}

/// **CANONICAL** Notification Retry Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationRetryPolicy {
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for NotificationRetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_secs(5),
        }
    }
}
