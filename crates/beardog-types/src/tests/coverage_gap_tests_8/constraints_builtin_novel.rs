// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage gap tests Part 8: Constraints (novel+builtin), `zero_cost`, `genetics_constraints`,
//! canonical/capabilities, canonical/network, constants

// ===========================================================================
// constraints/builtin.rs - 146 uncov
// ===========================================================================
mod builtin_constraints_tests {
    use crate::constraints::builtin::*;
    use crate::constraints::*;

    fn make_context() -> ConstraintContext {
        ConstraintContext::new()
    }

    #[test]
    fn test_time_range_constraint_satisfied() {
        let c = TimeRangeConstraint {
            start: "00:00".to_string(),
            end: "23:59".to_string(),
        };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "time_range");
    }

    #[test]
    fn test_time_range_constraint_serialize() {
        let c = TimeRangeConstraint {
            start: "09:00".to_string(),
            end: "17:00".to_string(),
        };
        let json = c.serialize_json();
        assert!(json.is_ok());
    }

    #[test]
    fn test_weekday_constraint() {
        let c = WeekdayConstraint {
            allowed_days: vec![
                "mon".to_string(),
                "tue".to_string(),
                "wed".to_string(),
                "thu".to_string(),
                "fri".to_string(),
                "sat".to_string(),
                "sun".to_string(),
            ],
        };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "weekday");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_weekday_constraint_empty_days() {
        let c = WeekdayConstraint {
            allowed_days: vec![],
        };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_cpu_quota_constraint() {
        let c = CpuQuotaConstraint { max_percent: 99 };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "cpu_quota");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_cpu_quota_constraint_zero() {
        let c = CpuQuotaConstraint { max_percent: 0 };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_memory_quota_constraint() {
        let c = MemoryQuotaConstraint {
            max_bytes: 16_000_000_000,
        };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "memory_quota");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_expiry_constraint_not_expired() {
        let future = (chrono::Utc::now() + chrono::Duration::days(30)).to_rfc3339();
        let c = ExpiryConstraint { expires_at: future };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "expiry");
    }

    #[test]
    fn test_expiry_constraint_expired() {
        let past = (chrono::Utc::now() - chrono::Duration::days(1)).to_rfc3339();
        let c = ExpiryConstraint { expires_at: past };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_composite_constraint_and() {
        let c1 = Box::new(CpuQuotaConstraint { max_percent: 99 }) as Box<dyn Constraint>;
        let c2 = Box::new(MemoryQuotaConstraint {
            max_bytes: 16_000_000_000,
        }) as Box<dyn Constraint>;
        let composite = CompositeConstraint::and(vec![c1, c2]);
        let ctx = make_context();
        let result = composite.is_satisfied(&ctx);
        assert!(result.is_ok());
        let _ = composite.description();
        assert_eq!(composite.constraint_type(), "composite");
        let _ = composite.serialize_json();
    }

    #[test]
    fn test_composite_constraint_or() {
        let c1 = Box::new(CpuQuotaConstraint { max_percent: 0 }) as Box<dyn Constraint>;
        let c2 = Box::new(MemoryQuotaConstraint {
            max_bytes: 16_000_000_000,
        }) as Box<dyn Constraint>;
        let composite = CompositeConstraint::or(vec![c1, c2]);
        let ctx = make_context();
        let result = composite.is_satisfied(&ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn test_composite_constraint_not() {
        let past = (chrono::Utc::now() - chrono::Duration::days(1)).to_rfc3339();
        let c = Box::new(ExpiryConstraint { expires_at: past }) as Box<dyn Constraint>;
        let composite = CompositeConstraint::not(c);
        let ctx = make_context();
        let result = composite.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap()); // NOT expired=false → true
    }
}

// ===========================================================================
// constraints/novel.rs - 159 uncov
// ===========================================================================
mod novel_constraints_tests {
    use crate::constraints::novel::*;
    use crate::constraints::*;

    fn make_context() -> ConstraintContext {
        ConstraintContext::new()
    }

    #[test]
    fn test_proximity_constraint_no_location() {
        let c = ProximityConstraint {
            other_party: "alice".to_string(),
            max_distance_meters: 100.0,
        };
        let ctx = make_context();
        let result = c.is_satisfied(&ctx);
        assert!(result.is_err());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "proximity");
    }

    #[test]
    fn test_proximity_constraint_with_location() {
        let c = ProximityConstraint {
            other_party: "bob".to_string(),
            max_distance_meters: 1000.0,
        };
        let mut ctx = make_context();
        ctx.location = Some(GeoLocation {
            latitude: 40.7128,
            longitude: -74.0060,
            altitude: None,
            accuracy: None,
        });
        let result = c.is_satisfied(&ctx);
        assert!(result.is_err());
    }

    #[test]
    fn test_proximity_constraint_with_other_location() {
        let c = ProximityConstraint {
            other_party: "bob".to_string(),
            max_distance_meters: 10_000_000.0,
        };
        let mut ctx = make_context();
        ctx.location = Some(GeoLocation {
            latitude: 40.7128,
            longitude: -74.0060,
            altitude: None,
            accuracy: None,
        });
        ctx.environment.insert(
            "bob_location".to_string(),
            serde_json::json!({"latitude": 40.7130, "longitude": -74.0062, "altitude": null, "accuracy": null}),
        );
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_environmental_constraint() {
        let c = EnvironmentalConstraint {
            sensor_id: "temperature".to_string(),
            min_value: Some(-10.0),
            max_value: Some(50.0),
        };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "environmental");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_environmental_constraint_with_data() {
        let c = EnvironmentalConstraint {
            sensor_id: "temp".to_string(),
            min_value: Some(10.0),
            max_value: Some(30.0),
        };
        let mut ctx = make_context();
        ctx.environment
            .insert("temp".to_string(), serde_json::json!(22.5));
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_network_ssid_constraint() {
        let c = NetworkSsidConstraint {
            allowed_ssids: vec!["HomeNetwork".to_string()],
        };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "network_ssid");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_vpn_constraint() {
        let c = VpnConstraint {
            required_tunnel: "wg0".to_string(),
        };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "vpn");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_biometric_constraint() {
        let c = BiometricConstraint {
            biometric_type: BiometricType::Fingerprint,
            device: "test-device".to_string(),
        };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "biometric");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_biometric_types() {
        let types = [
            BiometricType::Fingerprint,
            BiometricType::FaceId,
            BiometricType::Iris,
            BiometricType::Voice,
        ];
        for t in &types {
            let _ = format!("{t:?}");
            let _ = format!("{t}");
        }
    }

    #[test]
    fn test_system_load_constraint() {
        let c = SystemLoadConstraint { max_load_1m: 10.0 };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "system_load");
        let _ = c.serialize_json();
    }

    #[test]
    fn test_geo_fence_constraint() {
        let c = GeoFenceConstraint {
            center: GeoLocation {
                latitude: 40.0,
                longitude: -74.0,
                altitude: None,
                accuracy: None,
            },
            radius_meters: 10000.0,
        };
        let mut ctx = make_context();
        ctx.location = Some(GeoLocation {
            latitude: 40.001,
            longitude: -74.001,
            altitude: None,
            accuracy: None,
        });
        let result = c.is_satisfied(&ctx);
        assert!(result.is_ok());
        assert!(result.unwrap());
        let _ = c.description();
        assert_eq!(c.constraint_type(), "geofence");
    }

    #[test]
    fn test_battery_constraint() {
        let c = BatteryConstraint { min_percent: 10 };
        let ctx = make_context();
        let _ = c.is_satisfied(&ctx);
        let _ = c.description();
        assert_eq!(c.constraint_type(), "battery");
        let _ = c.serialize_json();
    }
}

// ===========================================================================
// zero_cost/memory_safe.rs - 107 uncov
// ===========================================================================
