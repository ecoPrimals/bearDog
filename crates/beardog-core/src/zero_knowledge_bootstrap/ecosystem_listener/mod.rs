// SPDX-License-Identifier: AGPL-3.0-only

// Ecosystem Listener
//
// This module implements passive listening for ecosystem announcements from other primals.
// It enables discovery of other primals without hardcoded knowledge, following the
// "infant learning" pattern where we listen and learn from the ecosystem.

mod discovery;
mod env;
mod listener;
mod types;

pub use env::EcosystemListenerEnvInputs;
pub use listener::EcosystemListener;
pub use types::{EcosystemEvent, EcosystemListenerMetrics, PrimalAnnouncement};
