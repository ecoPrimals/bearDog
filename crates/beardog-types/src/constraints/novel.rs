// Novel Constraint Examples
//
// These are example constraints that demonstrate the extensibility
// of the constraint-agnostic system. Users can create similar constraints
// for scenarios we never predicted.
//
// Philosophy: Users define their own rules. We provide the framework.

use super::{Constraint, ConstraintContext, GeoLocation};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

//==============================================================================
// 1. PROXIMITY CONSTRAINT - GPS-Based Physical Proximity
//==============================================================================

/// Proximity constraint - only satisfied when within distance of another party
///
/// # Use Case
/// "Allow tower access only when both Alice and Bob are physically present"
///
/// # Example
/// ```rust,ignore
/// let constraint = ProximityConstraint {
///     other_party: "alice".to_string(),
///     max_distance_meters: 100.0,
/// };
///
/// // Requires ConstraintContext with location data
/// let context = ConstraintContext::new()
///     .with_location(GeoLocation { lat: 37.7749, lon: -122.4194, ... });
///
/// assert!(constraint.is_satisfied(&context)?);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityConstraint {
    /// Other party's identifier (for location discovery)
    pub other_party: String,

    /// Maximum distance in meters
    pub max_distance_meters: f64,
}

impl Constraint for ProximityConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Get our location from context
        let my_location = context.location.ok_or_else(|| {
            BearDogError::invalid_input("Location required for proximity constraint")
        })?;

        // In production, this would discover other party's location via:
        // 1. Query primals advertising "location" or "presence" capability
        // 2. Send location request with party identifier
        // 3. Receive location response
        // For now, we use environment data if available (testing/offline mode)
        let other_location = if let Some(loc_data) = context
            .environment
            .get(&format!("{}_location", self.other_party))
        {
            serde_json::from_value(loc_data.clone()).map_err(|e| {
                BearDogError::serialization(&format!("Invalid location data: {}", e))
            })?
        } else {
            return Err(BearDogError::not_found(format!(
                "Location for '{}' not available. In production, would discover via Songbird.",
                self.other_party
            )));
        };

        // Calculate distance using Haversine formula (already in GeoLocation)
        let distance = my_location.distance_to(&other_location);

        Ok(distance <= self.max_distance_meters)
    }

    fn description(&self) -> String {
        format!(
            "Within {:.0}m of {}",
            self.max_distance_meters, self.other_party
        )
    }

    fn constraint_type(&self) -> &'static str {
        "proximity"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 2. ENVIRONMENTAL CONSTRAINT - Sensor-Based Conditions
//==============================================================================

/// Environmental sensor constraint
///
/// # Use Case
/// "Allow key use only when room temperature is between 18-25°C"
///
/// # Example
/// ```rust,ignore
/// let constraint = EnvironmentalConstraint {
///     sensor_id: "room-temp".to_string(),
///     min_value: Some(18.0),
///     max_value: Some(25.0),
/// };
///
/// let context = ConstraintContext::new()
///     .with_env("room-temp".to_string(), json!(22.5));
///
/// assert!(constraint.is_satisfied(&context)?);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConstraint {
    /// Sensor identifier
    pub sensor_id: String,

    /// Minimum acceptable value (if any)
    pub min_value: Option<f64>,

    /// Maximum acceptable value (if any)
    pub max_value: Option<f64>,
}

impl Constraint for EnvironmentalConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Get sensor value from environment
        let sensor_value = context.environment.get(&self.sensor_id).ok_or_else(|| {
            BearDogError::not_found(format!(
                "Sensor '{}' not available in environment",
                self.sensor_id
            ))
        })?;

        // Parse as number
        let value = sensor_value.as_f64().ok_or_else(|| {
            BearDogError::invalid_input(&format!(
                "Sensor '{}' value is not a number",
                self.sensor_id
            ))
        })?;

        // Check min constraint
        if let Some(min) = self.min_value {
            if value < min {
                return Ok(false);
            }
        }

        // Check max constraint
        if let Some(max) = self.max_value {
            if value > max {
                return Ok(false);
            }
        }

        Ok(true)
    }

    fn description(&self) -> String {
        match (self.min_value, self.max_value) {
            (Some(min), Some(max)) => {
                format!("Sensor {} between {} and {}", self.sensor_id, min, max)
            }
            (Some(min), None) => format!("Sensor {} >= {}", self.sensor_id, min),
            (None, Some(max)) => format!("Sensor {} <= {}", self.sensor_id, max),
            (None, None) => format!("Sensor {} exists", self.sensor_id),
        }
    }

    fn constraint_type(&self) -> &'static str {
        "environmental"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 3. NETWORK CONSTRAINT - WiFi SSID or VPN-Based
//==============================================================================

/// Network SSID constraint
///
/// # Use Case
/// "Allow access only when connected to specific WiFi network"
///
/// # Example
/// ```rust,ignore
/// let constraint = NetworkSsidConstraint {
///     allowed_ssids: vec!["SecureNet".to_string(), "HomeOffice".to_string()],
/// };
///
/// // Satisfied when connected to either SSID
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSsidConstraint {
    /// List of allowed WiFi SSIDs
    pub allowed_ssids: Vec<String>,
}

impl Constraint for NetworkSsidConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Check if connected to allowed SSID
        if let Some(current_ssid) = &context.network_state.wifi_ssid {
            Ok(self
                .allowed_ssids
                .iter()
                .any(|ssid| ssid.eq_ignore_ascii_case(current_ssid)))
        } else {
            // Not connected to WiFi
            Ok(false)
        }
    }

    fn description(&self) -> String {
        format!("WiFi SSID in: {}", self.allowed_ssids.join(", "))
    }

    fn constraint_type(&self) -> &'static str {
        "network_ssid"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

/// VPN constraint
///
/// # Use Case
/// "Allow access only when connected via specific VPN tunnel"
///
/// # Example
/// ```rust,ignore
/// let constraint = VpnConstraint {
///     required_tunnel: "songbird".to_string(),
/// };
///
/// // Satisfied when connected via Songbird VPN
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConstraint {
    /// Required VPN tunnel name
    pub required_tunnel: String,
}

impl Constraint for VpnConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Check if VPN is active
        if !context.network_state.vpn_active {
            return Ok(false);
        }

        // Check if it's the correct VPN
        if let Some(vpn_name) = &context.network_state.vpn_name {
            Ok(vpn_name.eq_ignore_ascii_case(&self.required_tunnel))
        } else {
            Ok(false)
        }
    }

    fn description(&self) -> String {
        format!("VPN tunnel: {}", self.required_tunnel)
    }

    fn constraint_type(&self) -> &'static str {
        "vpn"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 4. BIOMETRIC CONSTRAINT - Hardware-Based Auth
//==============================================================================

/// Biometric type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BiometricType {
    Fingerprint,
    FaceId,
    Iris,
    Voice,
}

impl std::fmt::Display for BiometricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BiometricType::Fingerprint => write!(f, "Fingerprint"),
            BiometricType::FaceId => write!(f, "Face ID"),
            BiometricType::Iris => write!(f, "Iris"),
            BiometricType::Voice => write!(f, "Voice"),
        }
    }
}

/// Biometric constraint
///
/// # Use Case
/// "Allow access only when fingerprint verified on Solo V2 key"
///
/// # Example
/// ```rust,ignore
/// let constraint = BiometricConstraint {
///     biometric_type: BiometricType::Fingerprint,
///     device: "solo-v2".to_string(),
/// };
///
/// // Requires environment data from hardware HSM
/// let context = ConstraintContext::new()
///     .with_env("biometric_verified".to_string(), json!(true))
///     .with_env("biometric_device".to_string(), json!("solo-v2"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricConstraint {
    /// Type of biometric
    pub biometric_type: BiometricType,

    /// Device identifier (e.g., "solo-v2", "iphone-se", "pixel-8a")
    pub device: String,
}

impl Constraint for BiometricConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Check if biometric was verified
        let verified = context
            .environment
            .get("biometric_verified")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false);

        if !verified {
            return Ok(false);
        }

        // Check if it was the correct device
        let device = context
            .environment
            .get("biometric_device")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        Ok(device.eq_ignore_ascii_case(&self.device))
    }

    fn description(&self) -> String {
        format!("{} verified on {}", self.biometric_type, self.device)
    }

    fn constraint_type(&self) -> &'static str {
        "biometric"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 5. SYSTEM LOAD CONSTRAINT - Performance-Based Access
//==============================================================================

/// System load constraint
///
/// # Use Case
/// "Allow delegation only when system load is below threshold"
///
/// # Example
/// ```rust,ignore
/// let constraint = SystemLoadConstraint {
///     max_load_1m: 2.0,  // 1-minute load average
/// };
///
/// // Satisfied when system load < 2.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLoadConstraint {
    /// Maximum 1-minute load average
    pub max_load_1m: f64,
}

impl Constraint for SystemLoadConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        match context.system_state.load_average_1m {
            Some(load) => Ok(load <= self.max_load_1m),
            None => {
                // If load not available, be permissive (sovereignty)
                Ok(true)
            }
        }
    }

    fn description(&self) -> String {
        format!("System load <= {:.2}", self.max_load_1m)
    }

    fn constraint_type(&self) -> &'static str {
        "system_load"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 6. GEOFENCE CONSTRAINT - Geographic Boundary
//==============================================================================

/// Geofence constraint - only satisfied when inside geographic boundary
///
/// # Use Case
/// "Allow key use only when inside secure facility (50m radius)"
///
/// # Example
/// ```rust,ignore
/// let constraint = GeoFenceConstraint {
///     center: GeoLocation { lat: 37.7749, lon: -122.4194, ... },
///     radius_meters: 50.0,
/// };
///
/// // Satisfied when within 50m of center point
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoFenceConstraint {
    /// Center point of geofence
    pub center: GeoLocation,

    /// Radius in meters
    pub radius_meters: f64,
}

impl Constraint for GeoFenceConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Get current location from context
        let current_location = context.location.ok_or_else(|| {
            BearDogError::invalid_input("Location required for geofence constraint")
        })?;

        // Calculate distance from center
        let distance = current_location.distance_to(&self.center);

        Ok(distance <= self.radius_meters)
    }

    fn description(&self) -> String {
        format!(
            "Within {:.0}m of ({:.4}, {:.4})",
            self.radius_meters, self.center.latitude, self.center.longitude
        )
    }

    fn constraint_type(&self) -> &'static str {
        "geofence"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// 7. BATTERY CONSTRAINT - Mobile Device Power Level
//==============================================================================

/// Battery level constraint
///
/// # Use Case
/// "Prevent high-power operations when battery is low"
///
/// # Example
/// ```rust,ignore
/// let constraint = BatteryConstraint {
///     min_percent: 20,
/// };
///
/// // Satisfied when battery >= 20%
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryConstraint {
    /// Minimum battery percentage (0-100)
    pub min_percent: u8,
}

impl Constraint for BatteryConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        let battery_percent = context
            .environment
            .get("battery_percent")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(100); // Default to full battery if unknown

        Ok(battery_percent >= self.min_percent as u64)
    }

    fn description(&self) -> String {
        format!("Battery >= {}%", self.min_percent)
    }

    fn constraint_type(&self) -> &'static str {
        "battery"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}

//==============================================================================
// TESTS
//==============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proximity_constraint() {
        let constraint = ProximityConstraint {
            other_party: "alice".to_string(),
            max_distance_meters: 100.0,
        };

        // Setup context with both locations
        let my_location = GeoLocation {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };

        let alice_location = GeoLocation {
            latitude: 37.7750, // Very close
            longitude: -122.4195,
            altitude: None,
            accuracy: None,
        };

        let context = ConstraintContext::new()
            .with_location(my_location)
            .with_env(
                "alice_location".to_string(),
                serde_json::to_value(alice_location).unwrap(),
            );

        assert!(constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_environmental_constraint() {
        let constraint = EnvironmentalConstraint {
            sensor_id: "temperature".to_string(),
            min_value: Some(18.0),
            max_value: Some(25.0),
        };

        // Test within range
        let context =
            ConstraintContext::new().with_env("temperature".to_string(), serde_json::json!(22.5));
        assert!(constraint.is_satisfied(&context).unwrap());

        // Test outside range (too hot)
        let context =
            ConstraintContext::new().with_env("temperature".to_string(), serde_json::json!(30.0));
        assert!(!constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_network_ssid_constraint() {
        let constraint = NetworkSsidConstraint {
            allowed_ssids: vec!["SecureNet".to_string(), "HomeOffice".to_string()],
        };

        let mut context = ConstraintContext::new();
        context.network_state.wifi_ssid = Some("SecureNet".to_string());

        assert!(constraint.is_satisfied(&context).unwrap());

        // Test wrong SSID
        context.network_state.wifi_ssid = Some("PublicWiFi".to_string());
        assert!(!constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_vpn_constraint() {
        let constraint = VpnConstraint {
            required_tunnel: "songbird".to_string(),
        };

        let mut context = ConstraintContext::new();
        context.network_state.vpn_active = true;
        context.network_state.vpn_name = Some("songbird".to_string());

        assert!(constraint.is_satisfied(&context).unwrap());

        // Test VPN not active
        context.network_state.vpn_active = false;
        assert!(!constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_biometric_constraint() {
        let constraint = BiometricConstraint {
            biometric_type: BiometricType::Fingerprint,
            device: "solo-v2".to_string(),
        };

        let context = ConstraintContext::new()
            .with_env("biometric_verified".to_string(), serde_json::json!(true))
            .with_env("biometric_device".to_string(), serde_json::json!("solo-v2"));

        assert!(constraint.is_satisfied(&context).unwrap());

        // Test wrong device
        let context = ConstraintContext::new()
            .with_env("biometric_verified".to_string(), serde_json::json!(true))
            .with_env(
                "biometric_device".to_string(),
                serde_json::json!("other-device"),
            );

        assert!(!constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_system_load_constraint() {
        let constraint = SystemLoadConstraint { max_load_1m: 2.0 };

        let mut context = ConstraintContext::new();
        context.system_state.load_average_1m = Some(1.5);

        assert!(constraint.is_satisfied(&context).unwrap());

        // Test high load
        context.system_state.load_average_1m = Some(3.0);
        assert!(!constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_geofence_constraint() {
        let center = GeoLocation {
            latitude: 37.7749,
            longitude: -122.4194,
            altitude: None,
            accuracy: None,
        };

        let constraint = GeoFenceConstraint {
            center,
            radius_meters: 50.0,
        };

        // Inside geofence (very close to center)
        let nearby = GeoLocation {
            latitude: 37.7750,
            longitude: -122.4195,
            altitude: None,
            accuracy: None,
        };

        let context = ConstraintContext::new().with_location(nearby);
        assert!(constraint.is_satisfied(&context).unwrap());
    }

    #[test]
    fn test_battery_constraint() {
        let constraint = BatteryConstraint { min_percent: 20 };

        // Good battery
        let context =
            ConstraintContext::new().with_env("battery_percent".to_string(), serde_json::json!(75));
        assert!(constraint.is_satisfied(&context).unwrap());

        // Low battery
        let context =
            ConstraintContext::new().with_env("battery_percent".to_string(), serde_json::json!(10));
        assert!(!constraint.is_satisfied(&context).unwrap());
    }
}
