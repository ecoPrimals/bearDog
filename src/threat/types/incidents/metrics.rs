//! Incident metrics and performance tracking
//!
//! This module contains types and functionality for tracking incident response
//! metrics and performance indicators.

use serde::{Deserialize, Serialize};

/// Incident metrics
///
/// Tracks key metrics for incident response
/// performance and effectiveness.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct IncidentMetrics {
    /// Time to detection (minutes)
    pub time_to_detection: Option<i64>,

    /// Time to containment (minutes)
    pub time_to_containment: Option<i64>,

    /// Time to resolution (minutes)
    pub time_to_resolution: Option<i64>,

    /// Total incident duration (minutes)
    pub total_duration: Option<i64>,

    /// Number of systems affected
    pub systems_affected: u32,

    /// Number of users affected
    pub users_affected: u32,

    /// Estimated business impact cost
    pub estimated_cost: Option<f64>,

    /// Number of team members involved
    pub team_members_involved: u32,
}

impl IncidentMetrics {
    /// Create new incident metrics
    ///
    /// # Returns
    /// A new `IncidentMetrics` instance
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentMetrics;
    ///
    /// let metrics = IncidentMetrics::new();
    /// assert_eq!(metrics.systems_affected, 0);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate total incident cost
    ///
    /// # Returns
    /// Total estimated cost including business impact
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentMetrics;
    ///
    /// let mut metrics = IncidentMetrics::new();
    /// metrics.estimated_cost = Some(10000.0);
    ///
    /// let total = metrics.calculate_total_cost();
    /// assert_eq!(total, 10000.0);
    /// ```
    pub fn calculate_total_cost(&self) -> f64 {
        self.estimated_cost.unwrap_or(0.0)
    }

    /// Check if metrics are complete
    ///
    /// # Returns
    /// `true` if all key metrics are available
    ///
    /// # Example
    /// ```rust
    /// use beardog::threat::types::IncidentMetrics;
    ///
    /// let metrics = IncidentMetrics::new();
    /// assert!(!metrics.is_complete());
    /// ```
    pub fn is_complete(&self) -> bool {
        self.time_to_detection.is_some()
            && self.time_to_containment.is_some()
            && self.time_to_resolution.is_some()
            && self.total_duration.is_some()
    }
} 