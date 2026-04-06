// SPDX-License-Identifier: AGPL-3.0-or-later

//! Human entropy collection: live terminal interaction and legacy multi-modal collectors.

/// Blocking keyboard/mouse capture with timing-derived entropy (no simulated input).
pub mod interaction_capture;
/// System-sourced and multi-modal collectors used where live TTY capture is unavailable.
pub mod legacy;

// Re-export main types
pub use interaction_capture::{
    InteractionCaptureConfig, InteractionCaptureResult, InteractionEntropyCollector,
    InteractionEvent, InteractionMetrics, InteractionType,
};

pub use legacy::{HumanEntropyConfig, MultiModalHumanEntropyCollector};
