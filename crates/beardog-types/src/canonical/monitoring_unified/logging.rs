// SPDX-License-Identifier: AGPL-3.0-or-later

// Logging Configuration (DEPRECATED - use canonical system config)
//
// **MIGRATION**: Use `crate::canonical::config::domains::system::LoggingConfig` instead.
//
// This type alias will be removed in v3.3.0.

#[deprecated(
    since = "3.1.0",
    note = "Use crate::canonical::config::domains::system::LoggingConfig instead"
)]
pub type LoggingConfig = crate::canonical::config::domains::system::LoggingConfig;
