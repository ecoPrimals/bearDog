# 🎨 Constraint Extensibility Guide

**How Users Can Create Novel Constraints Without Modifying BearDog Core**

---

## Philosophy

**"Users define their own rules. We provide the framework, not the limits."**

BearDog's constraint-agnostic architecture enables you to create constraints for scenarios we never imagined. This guide shows you how.

---

## Table of Contents

1. [Quick Start](#quick-start)
2. [The Constraint Trait](#the-constraint-trait)
3. [Using ConstraintContext](#using-constraintcontext)
4. [Built-in Constraint Examples](#built-in-constraint-examples)
5. [Novel Constraint Examples](#novel-constraint-examples)
6. [Step-by-Step: Creating Your Own](#step-by-step-creating-your-own)
7. [Composing Constraints](#composing-constraints)
8. [Integration with BearDog](#integration-with-beardog)
9. [Testing Your Constraints](#testing-your-constraints)
10. [Real-World Scenarios](#real-world-scenarios)

---

## Quick Start

```rust
use beardog_types::constraints::{Constraint, ConstraintContext};
use beardog_errors::BearDogError;
use serde::{Serialize, Deserialize};

// 1. Define your constraint struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyCustomConstraint {
    pub my_field: String,
}

// 2. Implement the Constraint trait (4 methods)
impl Constraint for MyCustomConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Your logic here
        Ok(true)
    }
    
    fn description(&self) -> String {
        format!("My custom constraint: {}", self.my_field)
    }
    
    fn constraint_type(&self) -> &str {
        "my_custom"
    }
    
    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Failed: {}", e)))
    }
}

// 3. Use it!
let constraint = MyCustomConstraint { my_field: "value".to_string() };
let context = ConstraintContext::new();
assert!(constraint.is_satisfied(&context)?);
```

**That's it!** No changes to BearDog core needed.

---

## The Constraint Trait

The universal `Constraint` trait is your foundation:

```rust
pub trait Constraint: Send + Sync + fmt::Debug {
    /// Check if constraint is currently satisfied
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError>;
    
    /// Human-readable description
    fn description(&self) -> String;
    
    /// Constraint type identifier
    fn constraint_type(&self) -> &str;
    
    /// Serialize for storage/transmission
    fn serialize_json(&self) -> Result<String, BearDogError>;
}
```

### Key Features

- **Object-Safe**: Can use `Box<dyn Constraint>` for runtime polymorphism
- **Send + Sync**: Safe to use across threads
- **Result-Based**: No panics, comprehensive error handling
- **Simple**: Only 4 methods to implement

---

## Using ConstraintContext

The `ConstraintContext` provides all runtime information your constraint needs:

```rust
pub struct ConstraintContext {
    pub current_time: DateTime<Utc>,
    pub location: Option<GeoLocation>,
    pub system_state: SystemState,
    pub network_state: NetworkState,
    pub user_identity: Option<String>,
    pub environment: HashMap<String, serde_json::Value>,  // Extensible!
}
```

### Builder Pattern

```rust
let context = ConstraintContext::new()
    .with_user("alice".to_string())
    .with_location(GeoLocation { lat: 37.7749, lon: -122.4194, ... })
    .with_env("temperature".to_string(), json!(22.5))
    .with_env("sensor-id".to_string(), json!("room-temp-01"));
```

### Extensibility via Environment HashMap

The `environment` field is your extension point for novel data:

```rust
// Add any custom data
context.environment.insert("moon_phase".to_string(), json!("full"));
context.environment.insert("proximity_alice".to_string(), json!(50.0)); // meters
context.environment.insert("co2_ppm".to_string(), json!(450));

// Access in your constraint
let moon_phase = context.environment.get("moon_phase")
    .and_then(|v| v.as_str())
    .unwrap_or("unknown");
```

---

## Built-in Constraint Examples

BearDog includes 6 built-in constraints as reference:

### 1. TimeRangeConstraint

Active hours (HH:MM - HH:MM):

```rust
TimeRangeConstraint {
    start: "9:00".to_string(),
    end: "17:00".to_string(),
}
```

### 2. WeekdayConstraint

Active days of the week:

```rust
WeekdayConstraint {
    allowed_days: vec!["mon", "tue", "wed", "thu", "fri"],
}
```

### 3. CpuQuotaConstraint

CPU usage limit (0-100%):

```rust
CpuQuotaConstraint {
    max_percent: 50,
}
```

### 4. MemoryQuotaConstraint

Memory usage limit (bytes):

```rust
MemoryQuotaConstraint {
    max_bytes: 8_589_934_592, // 8 GB
}
```

### 5. ExpiryConstraint

Time-based expiry:

```rust
ExpiryConstraint {
    expires_at: "2025-12-31T23:59:59Z".to_string(),
}
```

### 6. CompositeConstraint

Combine constraints with logic (AND/OR/NOT):

```rust
CompositeConstraint::and(vec![
    Box::new(TimeRangeConstraint { ... }),
    Box::new(CpuQuotaConstraint { ... }),
])
```

---

## Novel Constraint Examples

BearDog provides 8 novel constraint examples to inspire you:

### 1. ProximityConstraint

GPS-based physical proximity:

```rust
ProximityConstraint {
    other_party: "alice".to_string(),
    max_distance_meters: 100.0,
}
```

**Use Case**: "Allow tower access only when both Alice and Bob are within 100m"

### 2. EnvironmentalConstraint

Sensor-based conditions:

```rust
EnvironmentalConstraint {
    sensor_id: "room-temp".to_string(),
    min_value: Some(18.0),
    max_value: Some(25.0),
}
```

**Use Case**: "Allow key use only when room temperature is 18-25°C"

### 3. NetworkSsidConstraint

WiFi network-based:

```rust
NetworkSsidConstraint {
    allowed_ssids: vec!["SecureNet".to_string(), "HomeOffice".to_string()],
}
```

**Use Case**: "Allow access only on trusted WiFi networks"

### 4. VpnConstraint

VPN tunnel requirement:

```rust
VpnConstraint {
    required_tunnel: "songbird".to_string(),
}
```

**Use Case**: "Allow access only when connected via Songbird VPN"

### 5. BiometricConstraint

Hardware-based authentication:

```rust
BiometricConstraint {
    biometric_type: BiometricType::Fingerprint,
    device: "solo-v2".to_string(),
}
```

**Use Case**: "Allow access only when fingerprint verified on Solo V2 key"

### 6. SystemLoadConstraint

Performance-based access:

```rust
SystemLoadConstraint {
    max_load_1m: 2.0,
}
```

**Use Case**: "Prevent delegation when system is overloaded"

### 7. GeoFenceConstraint

Geographic boundary:

```rust
GeoFenceConstraint {
    center: GeoLocation { lat: 37.7749, lon: -122.4194, ... },
    radius_meters: 50.0,
}
```

**Use Case**: "Allow key use only inside secure facility (50m radius)"

### 8. BatteryConstraint

Mobile device power level:

```rust
BatteryConstraint {
    min_percent: 20,
}
```

**Use Case**: "Prevent high-power operations when battery is low"

---

## Step-by-Step: Creating Your Own

Let's create a **Moon Phase Constraint** that only allows access during full moons.

### Step 1: Define the Struct

```rust
use beardog_types::constraints::{Constraint, ConstraintContext};
use beardog_errors::BearDogError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonPhaseConstraint {
    /// Required moon phase ("new", "waxing", "full", "waning")
    pub required_phase: String,
}
```

### Step 2: Implement the Trait

```rust
impl Constraint for MoonPhaseConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        // Get moon phase from environment
        let current_phase = context
            .environment
            .get("moon_phase")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::not_found(
                "Moon phase not available in environment".to_string()
            ))?;

        // Check if it matches
        Ok(current_phase.eq_ignore_ascii_case(&self.required_phase))
    }

    fn description(&self) -> String {
        format!("Moon phase must be: {}", self.required_phase)
    }

    fn constraint_type(&self) -> &str {
        "moon_phase"
    }

    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("Serialization failed: {}", e)))
    }
}
```

### Step 3: Use It

```rust
// Create the constraint
let constraint = MoonPhaseConstraint {
    required_phase: "full".to_string(),
};

// Create context with moon phase data
let context = ConstraintContext::new()
    .with_env("moon_phase".to_string(), json!("full"));

// Evaluate
assert!(constraint.is_satisfied(&context)?);
```

### Step 4: Test It

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_phase_constraint() {
        let constraint = MoonPhaseConstraint {
            required_phase: "full".to_string(),
        };

        // Test satisfied (full moon)
        let context = ConstraintContext::new()
            .with_env("moon_phase".to_string(), json!("full"));
        assert!(constraint.is_satisfied(&context).unwrap());

        // Test not satisfied (new moon)
        let context = ConstraintContext::new()
            .with_env("moon_phase".to_string(), json!("new"));
        assert!(!constraint.is_satisfied(&context).unwrap());
    }
}
```

**Done!** You've created a novel constraint without touching BearDog core.

---

## Composing Constraints

Use `CompositeConstraint` to combine constraints with logical operators:

### AND Logic

All constraints must be satisfied:

```rust
use beardog_types::constraints::builtin::CompositeConstraint;

let constraint = CompositeConstraint::and(vec![
    Box::new(TimeRangeConstraint { start: "9:00".to_string(), end: "17:00".to_string() }),
    Box::new(WeekdayConstraint { allowed_days: vec!["mon", "tue", "wed", "thu", "fri"] }),
    Box::new(MoonPhaseConstraint { required_phase: "full".to_string() }),
]);
```

**Meaning**: "Only during business hours, on weekdays, and during full moons"

### OR Logic

At least one constraint must be satisfied:

```rust
let constraint = CompositeConstraint::or(vec![
    Box::new(NetworkSsidConstraint { allowed_ssids: vec!["HomeOffice".to_string()] }),
    Box::new(VpnConstraint { required_tunnel: "songbird".to_string() }),
]);
```

**Meaning**: "Either on HomeOffice WiFi OR connected via Songbird VPN"

### NOT Logic

Negate a constraint:

```rust
let constraint = CompositeConstraint::not(
    Box::new(WeekdayConstraint { allowed_days: vec!["sat", "sun"] })
);
```

**Meaning**: "NOT on weekends" (i.e., only on weekdays)

### Complex Logic

Nest composites for complex conditions:

```rust
let constraint = CompositeConstraint::and(vec![
    // Must be during business hours
    Box::new(TimeRangeConstraint { start: "9:00".to_string(), end: "17:00".to_string() }),
    
    // AND either on-site OR via VPN
    Box::new(CompositeConstraint::or(vec![
        Box::new(GeoFenceConstraint { center: lab_location, radius_meters: 50.0 }),
        Box::new(VpnConstraint { required_tunnel: "secure".to_string() }),
    ])),
    
    // AND biometric verified
    Box::new(BiometricConstraint { biometric_type: BiometricType::Fingerprint, device: "solo-v2".to_string() }),
]);
```

**Meaning**: "Business hours AND (on-site OR VPN) AND biometric verified"

---

## Integration with BearDog

### CLI Integration

To add your constraint to the CLI, extend the delegation handler:

```rust
// In key_delegate.rs
impl DelegationConstraints {
    pub fn to_constraints(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints = Vec::new();
        
        // ... existing constraints ...
        
        // Add your custom constraint
        if let Some(moon_phase) = &self.moon_phase {
            constraints.push(Box::new(MoonPhaseConstraint {
                required_phase: moon_phase.clone(),
            }));
        }
        
        constraints
    }
}
```

### Runtime Evaluation

Constraints are evaluated at runtime:

```rust
let constraints = delegation.to_constraints();
let context = ConstraintContext::new()
    .with_user("alice".to_string())
    .with_env("moon_phase".to_string(), json!("full"));

for constraint in &constraints {
    if !constraint.is_satisfied(&context)? {
        return Err(BearDogError::business(format!(
            "Constraint not satisfied: {}",
            constraint.description()
        )));
    }
}
```

---

## Testing Your Constraints

### Unit Tests

Test your constraint in isolation:

```rust
#[test]
fn test_my_constraint() {
    let constraint = MyCustomConstraint { /* ... */ };
    let context = ConstraintContext::new()
        .with_env("my_data".to_string(), json!("value"));
    
    assert!(constraint.is_satisfied(&context).unwrap());
}
```

### Integration Tests

Test with other constraints:

```rust
#[test]
fn test_composite_with_my_constraint() {
    let composite = CompositeConstraint::and(vec![
        Box::new(TimeRangeConstraint { /* ... */ }),
        Box::new(MyCustomConstraint { /* ... */ }),
    ]);
    
    let context = ConstraintContext::new()
        .with_env("my_data".to_string(), json!("value"));
    
    assert!(composite.is_satisfied(&context).unwrap());
}
```

### End-to-End Tests

Test via CLI:

```bash
beardog key delegate \
    --master-key my-key \
    --delegate-to alice \
    --output alice-delegated \
    --moon-phase full \
    --expires-in 30d
```

---

## Real-World Scenarios

### Scenario 1: Secure Lab Access

Multi-factor authentication for high-security facility:

```rust
let secure_lab = CompositeConstraint::and(vec![
    // Physical proximity (on-site)
    Box::new(GeoFenceConstraint {
        center: GeoLocation { lat: 37.7749, lon: -122.4194, ... },
        radius_meters: 50.0,
    }),
    
    // Business hours
    Box::new(TimeRangeConstraint {
        start: "8:00".to_string(),
        end: "18:00".to_string(),
    }),
    
    // Weekdays only
    Box::new(WeekdayConstraint {
        allowed_days: vec!["mon", "tue", "wed", "thu", "fri"],
    }),
    
    // Biometric verification
    Box::new(BiometricConstraint {
        biometric_type: BiometricType::Fingerprint,
        device: "solo-v2".to_string(),
    }),
    
    // System not overloaded
    Box::new(SystemLoadConstraint {
        max_load_1m: 2.0,
    }),
]);
```

### Scenario 2: Tower Sharing

Friend can use your tower for compute during specific hours:

```rust
let tower_sharing = CompositeConstraint::and(vec![
    // Only during off-peak hours
    Box::new(TimeRangeConstraint {
        start: "22:00".to_string(),
        end: "6:00".to_string(),
    }),
    
    // CPU quota
    Box::new(CpuQuotaConstraint {
        max_percent: 50,
    }),
    
    // Memory quota
    Box::new(MemoryQuotaConstraint {
        max_bytes: 8_589_934_592, // 8 GB
    }),
    
    // Time-limited
    Box::new(ExpiryConstraint {
        expires_at: "2025-12-31T23:59:59Z".to_string(),
    }),
]);
```

### Scenario 3: Environmental Data Center

Temperature and power-sensitive operations:

```rust
let data_center = CompositeConstraint::and(vec![
    // Temperature range
    Box::new(EnvironmentalConstraint {
        sensor_id: "dc-temp".to_string(),
        min_value: Some(18.0),
        max_value: Some(27.0),
    }),
    
    // Humidity range
    Box::new(EnvironmentalConstraint {
        sensor_id: "dc-humidity".to_string(),
        min_value: Some(40.0),
        max_value: Some(60.0),
    }),
    
    // UPS power available
    Box::new(BatteryConstraint {
        min_percent: 80,
    }),
    
    // Low system load
    Box::new(SystemLoadConstraint {
        max_load_1m: 1.5,
    }),
]);
```

### Scenario 4: Mobile Device Context

Adapt to device state:

```rust
let mobile_context = CompositeConstraint::and(vec![
    // Good battery
    Box::new(BatteryConstraint {
        min_percent: 30,
    }),
    
    // Trusted network
    Box::new(CompositeConstraint::or(vec![
        Box::new(NetworkSsidConstraint {
            allowed_ssids: vec!["Home".to_string(), "Work".to_string()],
        }),
        Box::new(VpnConstraint {
            required_tunnel: "songbird".to_string(),
        }),
    ])),
    
    // Biometric verified
    Box::new(BiometricConstraint {
        biometric_type: BiometricType::FaceId,
        device: "pixel-8a".to_string(),
    }),
]);
```

---

## Best Practices

### 1. **Be Permissive When Data Unavailable**

Follow sovereignty principles - don't fail closed:

```rust
// ✅ GOOD: Permissive when data unavailable
let value = context.environment.get("sensor")
    .and_then(|v| v.as_f64())
    .unwrap_or(0.0); // Default to safe value

// ❌ BAD: Fail when data unavailable (violates sovereignty)
let value = context.environment.get("sensor")
    .ok_or_else(|| BearDogError::not_found("Sensor required".to_string()))?;
```

### 2. **Use Descriptive Constraint Types**

```rust
// ✅ GOOD
fn constraint_type(&self) -> &str {
    "moon_phase"
}

// ❌ BAD
fn constraint_type(&self) -> &str {
    "custom1"
}
```

### 3. **Provide Meaningful Descriptions**

```rust
// ✅ GOOD
fn description(&self) -> String {
    format!("Within {}m of {}", self.max_distance, self.other_party)
}

// ❌ BAD
fn description(&self) -> String {
    "Proximity check".to_string()
}
```

### 4. **Handle Errors Gracefully**

```rust
// ✅ GOOD
.map_err(|e| BearDogError::serialization(&format!("Failed to serialize: {}", e)))

// ❌ BAD
.unwrap()
```

### 5. **Make Constraints Composable**

Design simple, single-purpose constraints that can be combined:

```rust
// ✅ GOOD: Simple, focused constraints
TimeRangeConstraint { ... }
WeekdayConstraint { ... }
// Combine with CompositeConstraint::and()

// ❌ BAD: Monolithic constraint with all logic
TimeAndWeekdayAndLocationConstraint { ... }
```

---

## Conclusion

**You now have everything you need to create novel constraints!**

### Key Takeaways

1. **Simple Interface**: Just 4 methods to implement
2. **Extensible Context**: Use `environment` HashMap for any data
3. **Composable**: Combine with AND/OR/NOT logic
4. **No Core Changes**: Add constraints without modifying BearDog
5. **Testable**: Easy to unit test and integrate

### Next Steps

1. Identify a scenario unique to your use case
2. Define a constraint struct
3. Implement the `Constraint` trait
4. Test it
5. Use it with BearDog delegation

**Remember**: "Users define their own rules. We provide the framework, not the limits."

---

*Constraint Extensibility Guide - December 11, 2025*  
*Framework: COMPLETE ✅*  
*Examples: 14 constraints*  
*Tests: PASSING ✅*  
*Philosophy: SOVEREIGNTY ✅*

