// SPDX-License-Identifier: AGPL-3.0-or-later

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
            serde_json::to_value(alice_location).expect("GeoLocation as JSON value"),
        );

    assert!(
        constraint
            .is_satisfied(&context)
            .expect("proximity constraint evaluates")
    );
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
    assert!(
        constraint
            .is_satisfied(&context)
            .expect("environmental constraint in range")
    );

    // Test outside range (too hot)
    let context =
        ConstraintContext::new().with_env("temperature".to_string(), serde_json::json!(30.0));
    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("environmental constraint out of range")
    );
}

#[test]
fn test_network_ssid_constraint() {
    let constraint = NetworkSsidConstraint {
        allowed_ssids: vec!["SecureNet".to_string(), "HomeOffice".to_string()],
    };

    let mut context = ConstraintContext::new();
    context.network_state.wifi_ssid = Some("SecureNet".to_string());

    assert!(
        constraint
            .is_satisfied(&context)
            .expect("network SSID allowed")
    );

    // Test wrong SSID
    context.network_state.wifi_ssid = Some("PublicWiFi".to_string());
    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("network SSID disallowed")
    );
}

#[test]
fn test_vpn_constraint() {
    let constraint = VpnConstraint {
        required_tunnel: "corp-wireguard".to_string(),
    };

    let mut context = ConstraintContext::new();
    context.network_state.vpn_active = true;
    context.network_state.vpn_name = Some("corp-wireguard".to_string());

    assert!(
        constraint
            .is_satisfied(&context)
            .expect("VPN constraint satisfied")
    );

    // Test VPN not active
    context.network_state.vpn_active = false;
    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("VPN constraint not satisfied")
    );
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

    assert!(
        constraint
            .is_satisfied(&context)
            .expect("biometric constraint satisfied")
    );

    // Test wrong device
    let context = ConstraintContext::new()
        .with_env("biometric_verified".to_string(), serde_json::json!(true))
        .with_env(
            "biometric_device".to_string(),
            serde_json::json!("other-device"),
        );

    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("biometric wrong device")
    );
}

#[test]
fn test_system_load_constraint() {
    let constraint = SystemLoadConstraint { max_load_1m: 2.0 };

    let mut context = ConstraintContext::new();
    context.system_state.load_average_1m = Some(1.5);

    assert!(
        constraint
            .is_satisfied(&context)
            .expect("system load within limit")
    );

    // Test high load
    context.system_state.load_average_1m = Some(3.0);
    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("system load over limit")
    );
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
    assert!(
        constraint
            .is_satisfied(&context)
            .expect("geofence satisfied")
    );
}

#[test]
fn test_battery_constraint() {
    let constraint = BatteryConstraint { min_percent: 20 };

    // Good battery
    let context =
        ConstraintContext::new().with_env("battery_percent".to_string(), serde_json::json!(75));
    assert!(
        constraint
            .is_satisfied(&context)
            .expect("battery sufficient")
    );

    // Low battery
    let context =
        ConstraintContext::new().with_env("battery_percent".to_string(), serde_json::json!(10));
    assert!(
        !constraint
            .is_satisfied(&context)
            .expect("battery insufficient")
    );
}

#[test]
fn test_proximity_requires_location() {
    let c = ProximityConstraint {
        other_party: "bob".into(),
        max_distance_meters: 10.0,
    };
    assert!(c.is_satisfied(&ConstraintContext::new()).is_err());
}

#[test]
fn test_proximity_invalid_other_location_json() {
    let c = ProximityConstraint {
        other_party: "bob".into(),
        max_distance_meters: 100.0,
    };
    let my = GeoLocation {
        latitude: 0.0,
        longitude: 0.0,
        altitude: None,
        accuracy: None,
    };
    let ctx = ConstraintContext::new()
        .with_location(my)
        .with_env("bob_location".into(), serde_json::json!("not-a-location"));
    assert!(c.is_satisfied(&ctx).is_err());
}

#[test]
fn test_proximity_description_and_serialize_json() {
    let c = ProximityConstraint {
        other_party: "z".into(),
        max_distance_meters: 12.5,
    };
    assert!(c.description().contains('z'));
    let json = c
        .serialize_json()
        .expect("ProximityConstraint serializes to JSON");
    assert!(json.contains("other_party") && json.contains("max_distance_meters"));
}

#[test]
fn test_battery_constraint_serialize_json_and_type() {
    let c = BatteryConstraint { min_percent: 15 };
    assert_eq!(c.constraint_type(), "battery");
    let s = c.serialize_json().expect("json");
    assert!(s.contains("15"));
    let back: BatteryConstraint = serde_json::from_str(&s).expect("roundtrip");
    assert_eq!(back.min_percent, 15);
}

#[test]
fn test_proximity_too_far_apart() {
    let constraint = ProximityConstraint {
        other_party: "alice".into(),
        max_distance_meters: 100.0,
    };
    let my = GeoLocation {
        latitude: 37.7749,
        longitude: -122.4194,
        altitude: None,
        accuracy: None,
    };
    let alice = GeoLocation {
        latitude: 51.5074,
        longitude: -0.1278,
        altitude: None,
        accuracy: None,
    };
    let ctx = ConstraintContext::new().with_location(my).with_env(
        "alice_location".into(),
        serde_json::to_value(alice).expect("GeoLocation JSON"),
    );
    assert!(
        !constraint
            .is_satisfied(&ctx)
            .expect("proximity evaluates across regions")
    );
}

#[test]
fn test_proximity_constraint_type() {
    let c = ProximityConstraint {
        other_party: "p".into(),
        max_distance_meters: 1.0,
    };
    assert_eq!(c.constraint_type(), "proximity");
}

#[test]
fn test_environmental_sensor_missing_and_non_numeric() {
    let c = EnvironmentalConstraint {
        sensor_id: "sensor-a".into(),
        min_value: None,
        max_value: None,
    };
    assert!(c.is_satisfied(&ConstraintContext::new()).is_err());
    let ctx = ConstraintContext::new().with_env("sensor-a".into(), serde_json::json!("nope"));
    assert!(c.is_satisfied(&ctx).is_err());
}

#[test]
fn test_environmental_description_branches() {
    let both = EnvironmentalConstraint {
        sensor_id: "s".into(),
        min_value: Some(1.0),
        max_value: Some(2.0),
    };
    assert!(both.description().contains("between"));
    let min_only = EnvironmentalConstraint {
        sensor_id: "s".into(),
        min_value: Some(3.0),
        max_value: None,
    };
    assert!(min_only.description().contains(">="));
    let max_only = EnvironmentalConstraint {
        sensor_id: "s".into(),
        min_value: None,
        max_value: Some(4.0),
    };
    assert!(max_only.description().contains("<="));
    let neither = EnvironmentalConstraint {
        sensor_id: "s".into(),
        min_value: None,
        max_value: None,
    };
    assert!(neither.description().contains("exists"));
}

#[test]
fn test_environmental_constraint_type_and_json() {
    let c = EnvironmentalConstraint {
        sensor_id: "env".into(),
        min_value: Some(0.0),
        max_value: None,
    };
    assert_eq!(c.constraint_type(), "environmental");
    let s = c.serialize_json().expect("json");
    assert!(s.contains("env"));
}

#[test]
fn test_network_ssid_not_connected() {
    let c = NetworkSsidConstraint {
        allowed_ssids: vec!["office".into()],
    };
    let mut ctx = ConstraintContext::new();
    ctx.network_state.wifi_ssid = None;
    assert!(!c.is_satisfied(&ctx).expect("disconnected wifi"));
}

#[test]
fn test_network_ssid_constraint_meta() {
    let c = NetworkSsidConstraint {
        allowed_ssids: vec!["a".into(), "b".into()],
    };
    assert_eq!(c.constraint_type(), "network_ssid");
    let s = c.serialize_json().expect("json");
    assert!(s.contains("a") && s.contains("b"));
}

#[test]
fn test_vpn_wrong_tunnel_or_unnamed() {
    let c = VpnConstraint {
        required_tunnel: "corp-wireguard".into(),
    };
    let mut ctx = ConstraintContext::new();
    ctx.network_state.vpn_active = true;
    ctx.network_state.vpn_name = Some("other-tunnel".into());
    assert!(!c.is_satisfied(&ctx).expect("wrong tunnel name"));
    ctx.network_state.vpn_name = None;
    assert!(!c.is_satisfied(&ctx).expect("vpn active but name missing"));
}

#[test]
fn test_vpn_constraint_meta() {
    let c = VpnConstraint {
        required_tunnel: "t".into(),
    };
    assert_eq!(c.constraint_type(), "vpn");
    assert!(c.description().contains('t'));
    let s = c.serialize_json().expect("json");
    assert!(s.contains("required_tunnel"));
}

#[test]
fn test_system_load_missing_is_permissive() {
    let c = SystemLoadConstraint { max_load_1m: 0.5 };
    assert!(
        c.is_satisfied(&ConstraintContext::new())
            .expect("missing load treated as satisfied")
    );
}

#[test]
fn test_system_load_constraint_meta() {
    let c = SystemLoadConstraint { max_load_1m: 1.25 };
    assert_eq!(c.constraint_type(), "system_load");
    let s = c.serialize_json().expect("json");
    assert!(s.contains("1.25"));
}

#[test]
fn test_geofence_requires_location() {
    let center = GeoLocation {
        latitude: 0.0,
        longitude: 0.0,
        altitude: None,
        accuracy: None,
    };
    let c = GeoFenceConstraint {
        center,
        radius_meters: 10.0,
    };
    assert!(c.is_satisfied(&ConstraintContext::new()).is_err());
}

#[test]
fn test_geofence_outside_radius() {
    let center = GeoLocation {
        latitude: 10.0,
        longitude: 10.0,
        altitude: None,
        accuracy: None,
    };
    let c = GeoFenceConstraint {
        center,
        radius_meters: 1.0,
    };
    let far = GeoLocation {
        latitude: 12.0,
        longitude: 12.0,
        altitude: None,
        accuracy: None,
    };
    let ctx = ConstraintContext::new().with_location(far);
    assert!(!c.is_satisfied(&ctx).expect("outside fence"));
}

#[test]
fn test_geofence_constraint_meta() {
    let center = GeoLocation {
        latitude: 1.0,
        longitude: 2.0,
        altitude: None,
        accuracy: None,
    };
    let c = GeoFenceConstraint {
        center,
        radius_meters: 5.0,
    };
    assert_eq!(c.constraint_type(), "geofence");
    assert!(c.description().contains('5'));
    let s = c.serialize_json().expect("json");
    assert!(s.contains("radius_meters"));
}

#[test]
fn test_biometric_unverified() {
    let c = BiometricConstraint {
        biometric_type: BiometricType::FaceId,
        device: "iphone".into(),
    };
    assert!(
        !c.is_satisfied(&ConstraintContext::new())
            .expect("not verified")
    );
}

#[test]
fn test_biometric_type_display_variants() {
    assert_eq!(format!("{}", BiometricType::Fingerprint), "Fingerprint");
    assert_eq!(format!("{}", BiometricType::FaceId), "Face ID");
    assert_eq!(format!("{}", BiometricType::Iris), "Iris");
    assert_eq!(format!("{}", BiometricType::Voice), "Voice");
}

#[test]
fn test_biometric_constraint_meta() {
    let c = BiometricConstraint {
        biometric_type: BiometricType::Voice,
        device: "dev".into(),
    };
    assert_eq!(c.constraint_type(), "biometric");
    assert!(c.description().contains("Voice"));
    let s = c.serialize_json().expect("json");
    assert!(s.contains("biometric_type"));
}
