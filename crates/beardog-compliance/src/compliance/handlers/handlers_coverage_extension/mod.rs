// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage extension tests for [`crate::ComplianceHandler`].
//!
//! Added December 8, 2025 to increase coverage from 65.91% to 90%+
//! Targets: `generate_recommendations`, edge cases, all compliance standards

#[cfg(test)]
mod evaluation;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod handler_surface;
#[cfg(test)]
mod recommendations;
#[cfg(test)]
mod simulation;
#[cfg(test)]
mod standards;
