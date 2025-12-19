# 🎯 BearDog Constraints - Quick Reference

**Create your own constraints in minutes!**

---

## 🚀 Quick Start (5 Minutes)

### 1. Define Your Constraint

```rust
use beardog_types::constraints::{Constraint, ConstraintContext};
use beardog_errors::BearDogError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyConstraint {
    pub threshold: f64,
}
```

### 2. Implement the Trait (4 Methods)

```rust
impl Constraint for MyConstraint {
    fn is_satisfied(&self, context: &ConstraintContext) -> Result<bool, BearDogError> {
        let value = context.environment
            .get("my_sensor")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        Ok(value >= self.threshold)
    }
    
    fn description(&self) -> String {
        format!("Sensor value >= {}", self.threshold)
    }
    
    fn constraint_type(&self) -> &str {
        "my_custom"
    }
    
    fn serialize_json(&self) -> Result<String, BearDogError> {
        serde_json::to_string(self)
            .map_err(|e| BearDogError::serialization(&format!("{}", e)))
    }
}
```

### 3. Use It!

```rust
let constraint = MyConstraint { threshold: 25.0 };
let context = ConstraintContext::new()
    .with_env("my_sensor".to_string(), json!(30.0));

assert!(constraint.is_satisfied(&context)?);
```

**Done!** No changes to BearDog core needed.

---

## 📚 Built-in Constraints (6)

### TimeRangeConstraint
Active during specific hours:
```rust
TimeRangeConstraint {
    start: "9:00".to_string(),
    end: "17:00".to_string(),
}
```

### WeekdayConstraint
Active on specific days:
```rust
WeekdayConstraint {
    allowed_days: vec!["mon", "tue", "wed", "thu", "fri"],
}
```

### CpuQuotaConstraint
Limit CPU usage:
```rust
CpuQuotaConstraint {
    max_percent: 50,
}
```

### MemoryQuotaConstraint
Limit memory usage:
```rust
MemoryQuotaConstraint {
    max_bytes: 8_589_934_592, // 8 GB
}
```

### ExpiryConstraint
Time-based expiration:
```rust
ExpiryConstraint {
    expires_at: "2025-12-31T23:59:59Z".to_string(),
}
```

### CompositeConstraint
Combine with AND/OR/NOT:
```rust
CompositeConstraint::and(vec![
    Box::new(TimeRangeConstraint { /* ... */ }),
    Box::new(CpuQuotaConstraint { /* ... */ }),
])
```

---

## 🎨 Novel Constraint Examples (8)

### ProximityConstraint
GPS-based physical proximity:
```rust
ProximityConstraint {
    other_party: "alice".to_string(),
    max_distance_meters: 100.0,
}
```
**Use**: "Allow access only when within 100m of Alice"

### GeoFenceConstraint
Geographic boundary:
```rust
GeoFenceConstraint {
    center: GeoLocation { lat: 37.7749, lon: -122.4194, ... },
    radius_meters: 50.0,
}
```
**Use**: "Allow key use only inside secure facility"

### EnvironmentalConstraint
Sensor-based conditions:
```rust
EnvironmentalConstraint {
    sensor_id: "temperature".to_string(),
    min_value: Some(18.0),
    max_value: Some(25.0),
}
```
**Use**: "Allow operations only when temperature is safe"

### NetworkSsidConstraint
WiFi network-based:
```rust
NetworkSsidConstraint {
    allowed_ssids: vec!["Home".to_string(), "Office".to_string()],
}
```
**Use**: "Allow access only on trusted networks"

### VpnConstraint
VPN tunnel requirement:
```rust
VpnConstraint {
    required_tunnel: "songbird".to_string(),
}
```
**Use**: "Require secure VPN connection"

### BiometricConstraint
Hardware-based authentication:
```rust
BiometricConstraint {
    biometric_type: BiometricType::Fingerprint,
    device: "solo-v2".to_string(),
}
```
**Use**: "Require biometric verification"

### SystemLoadConstraint
Performance-based access:
```rust
SystemLoadConstraint {
    max_load_1m: 2.0,
}
```
**Use**: "Prevent access when system is overloaded"

### BatteryConstraint
Mobile device power level:
```rust
BatteryConstraint {
    min_percent: 20,
}
```
**Use**: "Require sufficient battery level"

---

## 🔗 Composing Constraints

### AND Logic (All Must Pass)
```rust
CompositeConstraint::and(vec![
    Box::new(TimeRangeConstraint { start: "9:00".to_string(), end: "17:00".to_string() }),
    Box::new(WeekdayConstraint { allowed_days: vec!["mon", "tue", "wed", "thu", "fri"] }),
    Box::new(BiometricConstraint { biometric_type: BiometricType::Fingerprint, device: "solo-v2".to_string() }),
])
```
**Meaning**: "Business hours AND weekdays AND biometric verified"

### OR Logic (At Least One Must Pass)
```rust
CompositeConstraint::or(vec![
    Box::new(NetworkSsidConstraint { allowed_ssids: vec!["Home".to_string()] }),
    Box::new(VpnConstraint { required_tunnel: "songbird".to_string() }),
])
```
**Meaning**: "Either on Home WiFi OR connected via Songbird VPN"

### NOT Logic (Negate)
```rust
CompositeConstraint::not(
    Box::new(WeekdayConstraint { allowed_days: vec!["sat", "sun"] })
)
```
**Meaning**: "NOT on weekends" (i.e., only on weekdays)

### Complex Nested Logic
```rust
CompositeConstraint::and(vec![
    // Must be during business hours
    Box::new(TimeRangeConstraint { start: "9:00".to_string(), end: "17:00".to_string() }),
    
    // AND either on-site OR via VPN
    Box::new(CompositeConstraint::or(vec![
        Box::new(GeoFenceConstraint { center: lab_location, radius_meters: 50.0 }),
        Box::new(VpnConstraint { required_tunnel: "secure".to_string() }),
    ])),
    
    // AND biometric verified
    Box::new(BiometricConstraint { biometric_type: BiometricType::Fingerprint, device: "solo-v2".to_string() }),
])
```
**Meaning**: "Business hours AND (on-site OR VPN) AND biometric verified"

---

## 🛠️ Using with BearDog CLI

### Delegate Key with Constraints

```bash
beardog key delegate \
    --master-key my-key \
    --delegate-to alice \
    --output alice-delegated \
    --time-range "9:00-17:00" \
    --weekdays "mon-fri" \
    --cpu-quota 50 \
    --memory-quota "8GB" \
    --expires-in 30d
```

### Generate Key with Restrictions

```bash
beardog key generate \
    --key-id secure-key \
    --algorithm aes-256-gcm \
    --hsm software \
    --kdf argon2 \
    --usage sign-verify \
    --purpose "secure-lab" \
    --expires-in 90d
```

### Mix Keys

```bash
beardog key mix \
    --key1 alice-key \
    --key2 bob-key \
    --output shared-key \
    --purpose "shared-access"
```

### Check Key Lineage

```bash
beardog key lineage --key-id my-key
```

### Revoke Key

```bash
beardog key revoke --key-id compromised-key --reason "Security incident"
```

---

## 💡 Real-World Scenarios

### Scenario 1: Secure Lab Access

Multi-factor authentication:

```rust
let secure_lab = CompositeConstraint::and(vec![
    Box::new(GeoFenceConstraint { center: lab_location, radius_meters: 50.0 }),
    Box::new(TimeRangeConstraint { start: "8:00".to_string(), end: "18:00".to_string() }),
    Box::new(WeekdayConstraint { allowed_days: vec!["mon", "tue", "wed", "thu", "fri"] }),
    Box::new(BiometricConstraint { biometric_type: BiometricType::Fingerprint, device: "solo-v2".to_string() }),
    Box::new(SystemLoadConstraint { max_load_1m: 2.0 }),
]);
```

### Scenario 2: Tower Sharing

Friend can use your tower during off-peak hours:

```rust
let tower_sharing = CompositeConstraint::and(vec![
    Box::new(TimeRangeConstraint { start: "22:00".to_string(), end: "6:00".to_string() }),
    Box::new(CpuQuotaConstraint { max_percent: 50 }),
    Box::new(MemoryQuotaConstraint { max_bytes: 8_589_934_592 }),
    Box::new(ExpiryConstraint { expires_at: "2025-12-31T23:59:59Z".to_string() }),
]);
```

### Scenario 3: Mobile Context-Aware

Adapt to device state:

```rust
let mobile_safe = CompositeConstraint::and(vec![
    Box::new(BatteryConstraint { min_percent: 30 }),
    Box::new(CompositeConstraint::or(vec![
        Box::new(NetworkSsidConstraint { allowed_ssids: vec!["Home".to_string(), "Work".to_string()] }),
        Box::new(VpnConstraint { required_tunnel: "songbird".to_string() }),
    ])),
]);
```

---

## 📖 ConstraintContext Reference

### Available Fields

```rust
pub struct ConstraintContext {
    pub current_time: DateTime<Utc>,           // Current time
    pub location: Option<GeoLocation>,         // GPS coordinates
    pub system_state: SystemState,             // CPU, memory, load
    pub network_state: NetworkState,           // WiFi, VPN status
    pub user_identity: Option<String>,         // User ID
    pub environment: HashMap<String, Value>,   // EXTENSIBLE!
}
```

### Builder Pattern

```rust
let context = ConstraintContext::new()
    .with_user("alice".to_string())
    .with_location(GeoLocation { lat: 37.7749, lon: -122.4194, altitude: None, accuracy: None })
    .with_env("temperature".to_string(), json!(22.5))
    .with_env("sensor-id".to_string(), json!("room-01"))
    .with_env("battery_percent".to_string(), json!(75));
```

### Environment HashMap (Your Extension Point!)

```rust
// Add ANY custom data
context.environment.insert("moon_phase".to_string(), json!("full"));
context.environment.insert("proximity_alice".to_string(), json!(50.0));
context.environment.insert("biometric_verified".to_string(), json!(true));

// Access in your constraint
let value = context.environment.get("moon_phase")
    .and_then(|v| v.as_str())
    .unwrap_or("unknown");
```

---

## 🧪 Testing Your Constraint

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_constraint() {
        let constraint = MyConstraint { threshold: 25.0 };
        
        // Test satisfied
        let context = ConstraintContext::new()
            .with_env("my_sensor".to_string(), json!(30.0));
        assert!(constraint.is_satisfied(&context).unwrap());
        
        // Test not satisfied
        let context = ConstraintContext::new()
            .with_env("my_sensor".to_string(), json!(20.0));
        assert!(!constraint.is_satisfied(&context).unwrap());
    }
}
```

---

## 📝 Best Practices

### 1. Be Permissive When Data Unavailable

```rust
// ✅ GOOD: Permissive default
let value = context.environment.get("sensor")
    .and_then(|v| v.as_f64())
    .unwrap_or(0.0); // Safe default

// ❌ BAD: Fail when unavailable
let value = context.environment.get("sensor")
    .ok_or_else(|| BearDogError::not_found("Required".to_string()))?;
```

### 2. Use Descriptive Names

```rust
// ✅ GOOD
fn constraint_type(&self) -> &str { "temperature_range" }

// ❌ BAD
fn constraint_type(&self) -> &str { "custom1" }
```

### 3. Provide Clear Descriptions

```rust
// ✅ GOOD
fn description(&self) -> String {
    format!("Temperature between {}°C and {}°C", self.min, self.max)
}

// ❌ BAD
fn description(&self) -> String { "Temp check".to_string() }
```

### 4. Handle Errors Gracefully

```rust
// ✅ GOOD
.map_err(|e| BearDogError::serialization(&format!("Failed: {}", e)))

// ❌ BAD
.unwrap()
```

### 5. Keep Constraints Simple and Composable

```rust
// ✅ GOOD: Simple, focused
TimeRangeConstraint { ... }
WeekdayConstraint { ... }
// Combine with CompositeConstraint

// ❌ BAD: Monolithic
TimeAndWeekdayAndLocationConstraint { ... }
```

---

## 🎓 Learning Path

1. **Start Simple**: Use built-in constraints with CLI
2. **Compose**: Combine constraints with AND/OR/NOT
3. **Create**: Implement your first custom constraint
4. **Test**: Write unit tests for your constraint
5. **Integrate**: Use in delegation or other BearDog features
6. **Share**: Document and share novel constraints!

---

## 📚 Full Documentation

- **Comprehensive Guide**: See `CONSTRAINT_EXTENSIBILITY_GUIDE.md` (~650 lines)
- **Architecture**: See `CONSTRAINT_AGNOSTIC_COMPLETE.md`
- **Examples**: See `crates/beardog-types/src/constraints/novel.rs`

---

## 🤝 Community

**Share your novel constraints!** Users are creating constraints for scenarios we never imagined.

Examples we'd love to see:
- Astronomical (moon phases, solar conditions)
- Environmental (air quality, noise levels)
- Social (multi-party proximity, quorum)
- Temporal (holidays, special events)
- Physical (altitude, speed, acceleration)

---

## 💬 Philosophy

**"Users define their own rules. We provide the framework, not the limits."**

This isn't just a motto - it's embedded in the architecture:
- ✅ Extensible `ConstraintContext`
- ✅ Object-safe `Constraint` trait
- ✅ Composable with logic operators
- ✅ No core changes needed

---

*Quick Reference - December 11, 2025*  
*Constraints: 14 built-in + infinite custom ✅*  
*Create your own in ~5 minutes! 🚀*

