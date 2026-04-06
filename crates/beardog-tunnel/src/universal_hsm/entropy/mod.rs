// SPDX-License-Identifier: AGPL-3.0-or-later

//! Entropy collection module.
//!
//! Multi-source collection with quality assessment ([`EntropyCollector`]) and
//! live-feed validation ([`LiveFeedValidator`]) for human key creation.

/// Multi-source entropy collector with SHA3-256 mixing.
pub mod collector;
/// Live-feed entropy validation for human key creation.
pub mod live_feed_validator;

pub use collector::EntropyCollector;
pub use live_feed_validator::LiveFeedValidator;
