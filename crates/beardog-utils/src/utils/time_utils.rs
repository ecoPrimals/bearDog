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


/// Time utilities for BearDog security operations

use chrono::{DateTime, Utc};
/// Get the current UTC timestamp
///
/// Returns the current time in UTC timezone as a DateTime object.
/// This is used throughout BearDog for consistent timestamping of
/// security events, audit logs, and other time-sensitive operations.
/// # Returns
/// Current UTC timestamp as `DateTime<Utc>`
/// # Example
/// ```rust
/// use beardog::utils::time_utils::current_timestamp;
/// let timestamp = current_timestamp();
/// println!("Current time: {}", timestamp);
/// ```
pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}
