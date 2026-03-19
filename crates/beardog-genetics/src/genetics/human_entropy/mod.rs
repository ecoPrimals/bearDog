// SPDX-License-Identifier: AGPL-3.0-only

// Human Entropy Collection Module
//
// This module provides comprehensive human entropy collection capabilities,
// including interactive keyboard/mouse capture, multi-modal sensors, and
// extensible architecture for future sensor types (DNA, EKG, wearables, etc.)

pub mod interaction_capture; // Interactive keyboard and mouse entropy
pub mod legacy; // Legacy entropy collection (system-based)

// Re-export main types
pub use interaction_capture::{
    InteractionCaptureConfig, InteractionCaptureResult, InteractionEntropyCollector,
    InteractionEvent, InteractionMetrics, InteractionType,
};

pub use legacy::{HumanEntropyConfig, MultiModalHumanEntropyCollector};
