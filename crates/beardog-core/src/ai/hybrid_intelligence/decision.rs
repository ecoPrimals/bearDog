//! Decision making components for hybrid intelligence
//! 
//! This module contains decision confidence levels and related logic.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// DecisionConfidence enumeration for BearDog system
/// Comprehensive documentation
pub enum DecisionConfidence {
    /// Low confidence, requires human review
    /// Perfect enum variant with comprehensive semantics
    Low,
    /// Medium confidence, human review recommended
    /// Perfect enum variant with comprehensive semantics
    Medium,
    /// High confidence, can proceed autonomously
    /// Perfect enum variant with comprehensive semantics
    High,
}

impl Default for DecisionConfidence {
    fn default() -> Self {
        Self::Medium
    }
}

impl DecisionConfidence {
    /// Convert DecisionConfidence to numerical value for calculations
    pub fn as_f64(&self) -> f64 {
        match self {
            DecisionConfidence::Low => 0.3,
            DecisionConfidence::Medium => 0.6,
            DecisionConfidence::High => 0.9,
        }
    }
}

impl DecisionConfidence {
    /// Check if human review is required
    pub fn requires_human_review(&self) -> bool {
        matches!(self, Self::Low)
    }

    /// Check if human review is recommended
    pub fn recommends_human_review(&self) -> bool {
        matches!(self, Self::Low | Self::Medium)
    }

    /// Check if autonomous operation is allowed
    pub fn allows_autonomous_operation(&self) -> bool {
        matches!(self, Self::High)
    }
} 