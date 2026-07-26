// SPDX-License-Identifier: AGPL-3.0-or-later

// Built-in Constraints - Refactored from hardcoded implementations
//
// These are the constraints we shipped with (time, CPU, memory, weekday),
// now refactored to use the universal Constraint trait.

use super::{Constraint, ConstraintContext, LogicOperation};
use beardog_errors::BearDogError;
use chrono::{Datelike, Timelike};
use serde::{Deserialize, Serialize};

/// Time range constraint (HH:MM - HH:MM)
///
/// Satisfied when current time is within the specified range.
///
/// # Example
/// ```rust,ignore
/// let constraint = TimeRangeConstraint {
///     start: "09:00".to_string(),
///     end: "17:00".to_string(),
/// };
///
/// // Satisfied between 9 AM and 5 PM
/// assert!(constraint.is_satisfied(&context)?);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRangeConstraint {
    /// Inclusive start time as `HH:MM` (24-hour)
    pub start: String, // HH:MM
    /// Inclusive end time as `HH:MM` (24-hour)
    pub end: String, // HH:MM
}

impl Constraint for TimeRangeConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        let current = format!(
            "{:02}:{:02}",
            context.current_time.hour(),
            context.current_time.minute()
        );
        Ok(current >= self.start && current <= self.end)
    }

    fn description(&self) -> String {
        format!("Active hours: {} - {}", self.start, self.end)
    }

    fn constraint_type(&self) -> &'static str {
        "time_range"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

/// Weekday constraint
///
/// Satisfied when current day is in the allowed list.
///
/// # Example
/// ```rust,ignore
/// let constraint = WeekdayConstraint {
///     allowed_days: vec!["mon".to_string(), "tue".to_string(), "wed".to_string()],
/// };
///
/// // Satisfied on Monday, Tuesday, Wednesday
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeekdayConstraint {
    /// Lowercase weekday names allowed (e.g. `mon`, `tue`, …)
    pub allowed_days: Vec<String>, // ["mon", "tue", "wed", "thu", "fri", "sat", "sun"]
}

impl Constraint for WeekdayConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        let weekday = match context.current_time.weekday() {
            chrono::Weekday::Mon => "mon",
            chrono::Weekday::Tue => "tue",
            chrono::Weekday::Wed => "wed",
            chrono::Weekday::Thu => "thu",
            chrono::Weekday::Fri => "fri",
            chrono::Weekday::Sat => "sat",
            chrono::Weekday::Sun => "sun",
        };

        Ok(self
            .allowed_days
            .iter()
            .any(|d| d.eq_ignore_ascii_case(weekday)))
    }

    fn description(&self) -> String {
        format!("Active days: {}", self.allowed_days.join(", "))
    }

    fn constraint_type(&self) -> &'static str {
        "weekday"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

/// CPU quota constraint (0-100%)
///
/// Satisfied when CPU usage is below the specified percentage.
///
/// # Example
/// ```rust,ignore
/// let constraint = CpuQuotaConstraint {
///     max_percent: 50,
/// };
///
/// // Satisfied when CPU usage <= 50%
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuQuotaConstraint {
    /// Maximum allowed CPU usage percent (0–100)
    pub max_percent: u8,
}

impl Constraint for CpuQuotaConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        match context.system_state.cpu_usage_percent {
            Some(usage) => Ok(usage <= f64::from(self.max_percent)),
            None => {
                // If CPU usage not available, be conservative and allow
                // (sovereignty: default to permissive when data unavailable)
                Ok(true)
            }
        }
    }

    fn description(&self) -> String {
        format!("CPU limit: {}%", self.max_percent)
    }

    fn constraint_type(&self) -> &'static str {
        "cpu_quota"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

/// Memory quota constraint (bytes)
///
/// Satisfied when memory usage is below the specified bytes.
///
/// # Example
/// ```rust,ignore
/// let constraint = MemoryQuotaConstraint {
///     max_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
/// };
///
/// // Satisfied when memory usage <= 8 GB
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryQuotaConstraint {
    /// Maximum allowed resident memory usage in bytes
    pub max_bytes: u64,
}

impl Constraint for MemoryQuotaConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        match context.system_state.memory_used_bytes {
            Some(used) => Ok(used <= self.max_bytes),
            None => {
                // If memory usage not available, be conservative and allow
                Ok(true)
            }
        }
    }

    fn description(&self) -> String {
        format!("Memory limit: {}", format_bytes(self.max_bytes))
    }

    fn constraint_type(&self) -> &'static str {
        "memory_quota"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

/// Expiry constraint (timestamp)
///
/// Satisfied when current time is before the expiry timestamp.
///
/// # Example
/// ```rust,ignore
/// let constraint = ExpiryConstraint {
///     expires_at: "2025-12-31T23:59:59Z".to_string(),
/// };
///
/// // Satisfied before Dec 31, 2025
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiryConstraint {
    /// Expiration instant as RFC 3339 / ISO 8601 timestamp string
    pub expires_at: String, // ISO 8601 timestamp
}

impl Constraint for ExpiryConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        let expiry = chrono::DateTime::parse_from_rfc3339(&self.expires_at)
            .map_err(|e| BearDogError::validation(&format!("Invalid expiry format: {e}")))?;

        Ok(context.current_time < expiry)
    }

    fn description(&self) -> String {
        format!("Expires: {}", self.expires_at)
    }

    fn constraint_type(&self) -> &'static str {
        "expiry"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

/// Composite constraint (AND/OR/NOT logic)
///
/// Combines multiple constraints with logical operations.
///
/// # Example
/// ```rust,ignore
/// let constraint = CompositeConstraint {
///     operation: LogicOperation::And,
///     constraints: vec![
///         Box::new(TimeRangeConstraint { start: "9:00", end: "17:00" }),
///         Box::new(WeekdayConstraint { allowed_days: vec!["mon", "tue", "wed"] }),
///     ],
/// };
///
/// // Satisfied when both constraints are satisfied
/// ```
#[derive(Debug)]
pub struct CompositeConstraint {
    /// How child constraints are combined (`And`, `Or`, `Not`)
    pub operation: LogicOperation,
    /// Child constraints evaluated according to [`LogicOperation`]
    pub constraints: Vec<Box<dyn Constraint>>,
}

impl Constraint for CompositeConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        match self.operation {
            LogicOperation::And => {
                for constraint in &self.constraints {
                    if !constraint.is_satisfied(context)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            LogicOperation::Or => {
                for constraint in &self.constraints {
                    if constraint.is_satisfied(context)? {
                        return Ok(true);
                    }
                }
                Ok(false)
            }
            LogicOperation::Not => {
                // NOT applies to first constraint only
                if self.constraints.is_empty() {
                    return Err(BearDogError::validation(
                        "NOT operation requires at least one constraint",
                    ));
                }
                Ok(!self.constraints[0].is_satisfied(context)?)
            }
        }
    }

    fn description(&self) -> String {
        let desc_list: Vec<String> = self.constraints.iter().map(|c| c.description()).collect();
        format!("{} ({})", self.operation, desc_list.join(" "))
    }

    fn constraint_type(&self) -> &'static str {
        "composite"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        // Composite constraints serialize their operation and component types
        let component_types: Vec<String> = self
            .constraints
            .iter()
            .map(|c| c.constraint_type().to_string())
            .collect();

        let json = serde_json::json!({
            "type": "composite",
            "operation": self.operation,
            "components": component_types,
        });

        serde_json::to_string(&json)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {e}")))
    }
}

impl CompositeConstraint {
    /// Create AND composite constraint
    #[must_use]
    pub fn and(constraints: Vec<Box<dyn Constraint>>) -> Self {
        Self {
            operation: LogicOperation::And,
            constraints,
        }
    }

    /// Create OR composite constraint
    #[must_use]
    pub fn or(constraints: Vec<Box<dyn Constraint>>) -> Self {
        Self {
            operation: LogicOperation::Or,
            constraints,
        }
    }

    /// Create NOT composite constraint
    #[must_use]
    pub fn not(constraint: Box<dyn Constraint>) -> Self {
        Self {
            operation: LogicOperation::Not,
            constraints: vec![constraint],
        }
    }
}

/// Helper function to format bytes
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    #[expect(
        clippy::cast_precision_loss,
        reason = "display/metric conversion, precision loss acceptable"
    )]
    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_time_range_constraint() {
        let constraint = TimeRangeConstraint {
            start: "09:00".to_string(),
            end: "17:00".to_string(),
        };

        // Create context with specific time (12:00)
        let mut context = ConstraintContext::new();
        let noon = Utc::now()
            .date_naive()
            .and_hms_opt(12, 0, 0)
            .expect("valid noon time in test")
            .and_utc();
        context.current_time = noon;

        assert!(
            constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );
        assert_eq!(constraint.constraint_type(), "time_range");
    }

    #[test]
    fn test_cpu_quota_constraint() {
        let constraint = CpuQuotaConstraint { max_percent: 50 };

        let mut context = ConstraintContext::new();
        context.system_state.cpu_usage_percent = Some(30.0);

        assert!(
            constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );

        context.system_state.cpu_usage_percent = Some(70.0);
        assert!(
            !constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );
    }

    #[test]
    fn test_memory_quota_constraint() {
        let constraint = MemoryQuotaConstraint {
            max_bytes: 8 * 1024 * 1024 * 1024, // 8 GB
        };

        let mut context = ConstraintContext::new();
        context.system_state.memory_used_bytes = Some(4 * 1024 * 1024 * 1024); // 4 GB

        assert!(
            constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );
    }

    #[test]
    fn test_composite_and_constraint() {
        let constraint = CompositeConstraint::and(vec![
            Box::new(CpuQuotaConstraint { max_percent: 50 }),
            Box::new(MemoryQuotaConstraint {
                max_bytes: 8 * 1024 * 1024 * 1024,
            }),
        ]);

        let mut context = ConstraintContext::new();
        context.system_state.cpu_usage_percent = Some(30.0);
        context.system_state.memory_used_bytes = Some(4 * 1024 * 1024 * 1024);

        assert!(
            constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );

        // Fail CPU check
        context.system_state.cpu_usage_percent = Some(70.0);
        assert!(
            !constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );
    }

    #[test]
    fn test_composite_or_constraint() {
        let constraint = CompositeConstraint::or(vec![
            Box::new(CpuQuotaConstraint { max_percent: 50 }),
            Box::new(MemoryQuotaConstraint {
                max_bytes: 8 * 1024 * 1024 * 1024,
            }),
        ]);

        let mut context = ConstraintContext::new();
        context.system_state.cpu_usage_percent = Some(70.0); // Fails
        context.system_state.memory_used_bytes = Some(4 * 1024 * 1024 * 1024); // Passes

        // Should pass because memory constraint passes
        assert!(
            constraint
                .is_satisfied(&context)
                .expect("is_satisfied in test")
        );
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(8 * 1024 * 1024 * 1024), "8.00 GB");
    }

    #[test]
    fn test_time_range_constraint_serialize_json_roundtrip() {
        let c = TimeRangeConstraint {
            start: "08:00".to_string(),
            end: "18:00".to_string(),
        };
        let s = c.serialize_json().expect("serialize");
        let back: TimeRangeConstraint = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back.start, "08:00");
        assert_eq!(c.constraint_type(), "time_range");
    }

    #[test]
    fn test_weekday_constraint_description() {
        let c = WeekdayConstraint {
            allowed_days: vec!["mon".to_string(), "fri".to_string()],
        };
        assert!(c.description().contains("mon"));
        assert_eq!(c.constraint_type(), "weekday");
    }
}
