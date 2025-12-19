// Constraint System - Universal, extensible constraint evaluation
//
// Philosophy: Users define their own rules. We provide the framework, not the limits.

pub mod builtin;
pub mod novel;

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// Universal constraint that can be evaluated
///
/// This trait enables constraint-agnostic design where users can create
/// novel constraints for scenarios we can't predict:
/// - Proximity-based constraints
/// - Environmental sensor constraints
/// - Network connectivity constraints
/// - Biometric authentication constraints
/// - Composite logical constraints (AND/OR/NOT)
/// - Any future constraint type
///
/// # Philosophy
/// Sovereignty means users define their own rules.
/// We provide the framework, not the limits.
///
/// # Example
/// ```rust,ignore
/// use beardog_types::constraints::{Constraint, ConstraintContext};
///
/// struct TimeRangeConstraint {
///     start: String,
///     end: String,
/// }
///
/// impl Constraint for TimeRangeConstraint {
///     fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
///         let current = format!("{:02}:{:02}",
///             context.current_time.hour(),
///             context.current_time.minute()
///         );
///         Ok(current >= self.start && current <= self.end)
///     }
///
///     fn description(&self) -> String {
///         format!("Active hours: {} - {}", self.start, self.end)
///     }
///
///     fn constraint_type(&self) -> &str {
///         "time_range"
///     }
/// }
/// ```
pub trait Constraint: Send + Sync + fmt::Debug {
    /// Check if constraint is currently satisfied
    ///
    /// Returns `Ok(true)` if satisfied, `Ok(false)` if not satisfied,
    /// or `Err` if evaluation failed.
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError>;

    /// Human-readable description of the constraint
    fn description(&self) -> String;

    /// Constraint type identifier (e.g., "time_range", "proximity", "cpu_quota")
    ///
    /// Used for constraint discovery and serialization.
    fn constraint_type(&self) -> &str;

    /// Serialize constraint to JSON for storage/transmission
    ///
    /// Each constraint type should implement this to serialize its specific data.
    /// This makes the trait object-safe by not requiring `Self: Serialize`.
    fn serialize_json(&self) -> Result<String, BearDogError>;
}

/// Context provided during constraint evaluation
///
/// This structure is intentionally extensible to support novel constraints.
/// New fields can be added without breaking existing constraints.
///
/// # Extensibility
/// - `current_time`: Always available
/// - `location`: Available if GPS/location primal present
/// - `system_state`: Available if system monitoring enabled
/// - `network_state`: Available if network primal present
/// - `environment`: Extensible HashMap for any context data
///
/// # Example
/// ```rust,ignore
/// let context = ConstraintContext {
///     current_time: Utc::now(),
///     location: Some(GeoLocation { lat: 37.7749, lon: -122.4194 }),
///     system_state: SystemState::default(),
///     network_state: NetworkState::default(),
///     environment: {
///         let mut env = HashMap::new();
///         env.insert("temperature".to_string(), json!(22.5));
///         env.insert("humidity".to_string(), json!(45));
///         env
///     },
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ConstraintContext {
    /// Current time (always available)
    pub current_time: DateTime<Utc>,

    /// Geographic location (if available)
    pub location: Option<GeoLocation>,

    /// System state (CPU, memory, load, etc.)
    pub system_state: SystemState,

    /// Network state (connectivity, SSID, VPN, etc.)
    pub network_state: NetworkState,

    /// User identity
    pub user_identity: Option<String>,

    /// Extensible environment for novel constraint data
    ///
    /// This HashMap allows any primal to provide context data
    /// that we didn't predict. Examples:
    /// - "room-temperature": 22.5
    /// - "light-level": 800
    /// - "motion-detected": true
    /// - "biometric-verified": true
    pub environment: HashMap<String, serde_json::Value>,
}

impl ConstraintContext {
    /// Create new context with current time
    pub fn new() -> Self {
        Self {
            current_time: Utc::now(),
            location: None,
            system_state: SystemState::default(),
            network_state: NetworkState::default(),
            user_identity: None,
            environment: HashMap::new(),
        }
    }

    /// Add environment data
    pub fn with_env(mut self, key: String, value: serde_json::Value) -> Self {
        self.environment.insert(key, value);
        self
    }

    /// Add location
    pub fn with_location(mut self, location: GeoLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Add user identity
    pub fn with_user(mut self, user: String) -> Self {
        self.user_identity = Some(user);
        self
    }
}

impl Default for ConstraintContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Geographic location
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: Option<f64>,
    pub accuracy: Option<f64>, // meters
}

impl GeoLocation {
    /// Calculate distance to another location (Haversine formula)
    pub fn distance_to(&self, other: &GeoLocation) -> f64 {
        const EARTH_RADIUS_M: f64 = 6_371_000.0;

        let lat1 = self.latitude.to_radians();
        let lat2 = other.latitude.to_radians();
        let delta_lat = (other.latitude - self.latitude).to_radians();
        let delta_lon = (other.longitude - self.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        EARTH_RADIUS_M * c
    }
}

/// System state (CPU, memory, load, etc.)
#[derive(Debug, Clone, Default)]
pub struct SystemState {
    pub cpu_usage_percent: Option<f64>,
    pub memory_used_bytes: Option<u64>,
    pub memory_total_bytes: Option<u64>,
    pub load_average_1m: Option<f64>,
    pub load_average_5m: Option<f64>,
    pub load_average_15m: Option<f64>,
    pub uptime_seconds: Option<u64>,
}

impl SystemState {
    /// Get memory usage percentage (0.0 - 100.0)
    pub fn memory_usage_percent(&self) -> Option<f64> {
        match (self.memory_used_bytes, self.memory_total_bytes) {
            // Note: Precision loss acceptable for percentage calculation
            #[allow(clippy::cast_precision_loss)]
            (Some(used), Some(total)) if total > 0 => Some((used as f64 / total as f64) * 100.0),
            _ => None,
        }
    }
}

/// Network state (connectivity, SSID, VPN, etc.)
#[derive(Debug, Clone, Default)]
pub struct NetworkState {
    pub connected: bool,
    pub wifi_ssid: Option<String>,
    pub vpn_active: bool,
    pub vpn_name: Option<String>,
    pub public_ip: Option<String>,
    pub local_ip: Option<String>,
}

/// Logic operations for composite constraints
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LogicOperation {
    /// All constraints must be satisfied (AND)
    And,
    /// At least one constraint must be satisfied (OR)
    Or,
    /// Negate the constraint (NOT)
    Not,
}

impl fmt::Display for LogicOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicOperation::And => write!(f, "AND"),
            LogicOperation::Or => write!(f, "OR"),
            LogicOperation::Not => write!(f, "NOT"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_context_creation() {
        let context = ConstraintContext::new();
        assert!(context.location.is_none());
        assert!(context.environment.is_empty());
    }

    #[test]
    fn test_constraint_context_builder() {
        let context = ConstraintContext::new()
            .with_user("alice".to_string())
            .with_env("temperature".to_string(), serde_json::json!(22.5));

        assert_eq!(context.user_identity, Some("alice".to_string()));
        assert_eq!(
            context.environment.get("temperature"),
            Some(&serde_json::json!(22.5))
        );
    }

    #[test]
    fn test_geolocation_distance() {
        // San Francisco
        let sf = GeoLocation {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };

        // Los Angeles
        let la = GeoLocation {
            latitude: 34.0522,
            longitude: -118.2437,
            altitude: None,
            accuracy: None,
        };

        let distance = sf.distance_to(&la);
        // Should be approximately 559 km = 559,000 meters
        assert!(
            (distance - 559_000.0).abs() < 10_000.0,
            "Distance calculation off: {}",
            distance
        );
    }

    #[test]
    fn test_system_state_memory_percent() {
        let state = SystemState {
            memory_used_bytes: Some(8 * 1024 * 1024 * 1024), // 8 GB
            memory_total_bytes: Some(16 * 1024 * 1024 * 1024), // 16 GB
            ..Default::default()
        };

        let percent = state.memory_usage_percent();
        assert_eq!(percent, Some(50.0));
    }

    #[test]
    fn test_logic_operation_display() {
        assert_eq!(LogicOperation::And.to_string(), "AND");
        assert_eq!(LogicOperation::Or.to_string(), "OR");
        assert_eq!(LogicOperation::Not.to_string(), "NOT");
    }
}
