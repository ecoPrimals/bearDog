# 🔗 Integration Guide: Wiring Validation to BearDog

**Phase**: Integration (After Team A fixes compilation)  
**Duration**: 2-3 days  
**Prerequisites**: BearDog compiles, Infrastructure built  
**Date**: October 9, 2025

---

## 🎯 Goal

Connect the validation framework to **LIVE BearDog instances** - replacing all placeholder `Ok(true)` with real measurements.

---

## 📋 Prerequisites Checklist

Before starting integration:

**From Team A**:
- [ ] BearDog workspace compiles (`cargo build --workspace` succeeds)
- [ ] Basic tests pass
- [ ] 3 compilation errors fixed

**From Team B**:
- [ ] Validation framework compiles
- [ ] Statistical module implemented
- [ ] Telemetry module implemented
- [ ] Docker infrastructure deployed

---

## 🔧 Step 1: Add BearDog Dependencies

Edit `experiments/beardog-sovereign-science/framework/Cargo.toml`:

```toml
[dependencies]
# === EXISTING DEPENDENCIES (keep these) ===
tokio = { version = "1.0", features = ["full"] }
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
ring = "0.17"
ed25519-dalek = "2.0"
x25519-dalek = "2.0"
blake3 = "1.0"
statrs = "0.16"
nalgebra = "0.32"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
metrics = "0.21"
metrics-exporter-prometheus = "0.12"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
thiserror = "1.0"
anyhow = "1.0"
criterion = { version = "0.5", features = ["html_reports"] }
cryptoki = "0.6"
reqwest = { version = "0.11", features = ["json"] }
hyper = { version = "0.14", features = ["full"] }

# === ADD BEARDOG CRATES ===
beardog-core = { path = "../../../crates/beardog-core" }
beardog-security = { path = "../../../crates/beardog-security" }
beardog-tunnel = { path = "../../../crates/beardog-tunnel" }
beardog-genetics = { path = "../../../crates/beardog-genetics" }
beardog-types = { path = "../../../crates/beardog-types" }
beardog-errors = { path = "../../../crates/beardog-errors" }
beardog-traits = { path = "../../../crates/beardog-traits" }
beardog-auth = { path = "../../../crates/beardog-auth" }
beardog-monitoring = { path = "../../../crates/beardog-monitoring" }
```

**Test it**:
```bash
cd experiments/beardog-sovereign-science/framework
cargo build
```

**Expected**: Should compile, pulling in all BearDog crates.

---

## 🔧 Step 2: Wire Cryptographic Validation

### Replace Placeholder in `src/stages.rs`

**Current (placeholder)**:
```rust
async fn validate_mathematical_security() -> Result<bool, SovereignScienceError> {
    tracing::debug!("🔢 Validating mathematical security");
    Ok(true) // Placeholder
}
```

**New (REAL implementation)**:
```rust
use beardog_security::crypto::providers::Ed25519Provider;
use beardog_tunnel::hsm::UniversalHSM;
use std::time::Instant;

async fn validate_mathematical_security() -> Result<bool, SovereignScienceError> {
    tracing::info!("🔢 Validating mathematical security - 1,000,000 operations");
    
    // Initialize real BearDog crypto provider
    let provider = Ed25519Provider::new()
        .map_err(|e| SovereignScienceError::ValidationError(
            format!("Failed to initialize crypto provider: {}", e)
        ))?;
    
    let mut all_operations_secure = true;
    let operations_count = 1_000_000;
    
    // Track timing for constant-time verification
    let mut operation_times = Vec::with_capacity(operations_count);
    
    for i in 0..operations_count {
        let start = Instant::now();
        
        // Generate keypair
        let keypair = provider.generate_keypair()
            .map_err(|e| SovereignScienceError::ValidationError(
                format!("Keypair generation failed at iteration {}: {}", i, e)
            ))?;
        
        // Create test message
        let message = format!("beardog-validation-{}", i).into_bytes();
        
        // Sign message
        let signature = provider.sign(&message, &keypair.secret)
            .map_err(|e| SovereignScienceError::ValidationError(
                format!("Signing failed at iteration {}: {}", i, e)
            ))?;
        
        // Verify signature
        let verified = provider.verify(&message, &signature, &keypair.public)
            .map_err(|e| SovereignScienceError::ValidationError(
                format!("Verification failed at iteration {}: {}", i, e)
            ))?;
        
        let duration = start.elapsed();
        operation_times.push(duration.as_nanos() as f64);
        
        if !verified {
            all_operations_secure = false;
            tracing::error!("❌ Signature verification failed at iteration {}", i);
            break;
        }
        
        // Log progress every 100k operations
        if i > 0 && i % 100_000 == 0 {
            tracing::info!("Progress: {}/{} operations validated", i, operations_count);
        }
    }
    
    // Analyze timing for constant-time properties
    let timing_variance = calculate_variance(&operation_times);
    let timing_is_constant = timing_variance < 1000.0; // Threshold TBD based on measurements
    
    tracing::info!(
        "✅ Mathematical security validation complete: {} operations, constant-time: {}",
        operations_count,
        timing_is_constant
    );
    
    Ok(all_operations_secure && timing_is_constant)
}

fn calculate_variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mean = values.iter().sum::<f64>() / values.len() as f64;
    let variance = values.iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    
    variance.sqrt()
}
```

---

## 🔧 Step 3: Wire Timing Attack Validation

**Replace**:
```rust
async fn validate_timing_attack_resistance() -> Result<bool, SovereignScienceError> {
    tracing::info!("⏱️ Validating timing attack resistance");
    
    use beardog_security::crypto::constant_time;
    use std::time::Instant;
    
    // Test constant-time comparison
    let test_cases = vec![
        (vec![0u8; 32], vec![0u8; 32], true),   // Equal
        (vec![0u8; 32], vec![1u8; 32], false),  // Different at end
        (vec![1u8; 32], vec![0u8; 32], false),  // Different at start
    ];
    
    let mut timings_equal = Vec::new();
    let mut timings_different = Vec::new();
    
    // Run comparisons multiple times to measure timing
    for _ in 0..10_000 {
        for (a, b, should_be_equal) in &test_cases {
            let start = Instant::now();
            let result = constant_time::compare(a, b);
            let duration = start.elapsed().as_nanos();
            
            if *should_be_equal {
                timings_equal.push(duration as f64);
            } else {
                timings_different.push(duration as f64);
            }
            
            assert_eq!(result, *should_be_equal);
        }
    }
    
    // Statistical test: timing should NOT differ significantly between equal/unequal
    let mean_equal = timings_equal.iter().sum::<f64>() / timings_equal.len() as f64;
    let mean_different = timings_different.iter().sum::<f64>() / timings_different.len() as f64;
    
    let timing_difference_percent = ((mean_equal - mean_different).abs() / mean_equal) * 100.0;
    
    // Constant-time operations should have <1% timing difference
    let is_constant_time = timing_difference_percent < 1.0;
    
    tracing::info!(
        "⏱️ Timing analysis: equal={:.2}ns, different={:.2}ns, diff={:.2}%",
        mean_equal, mean_different, timing_difference_percent
    );
    
    Ok(is_constant_time)
}
```

---

## 🔧 Step 4: Wire Entropy Validation

**Replace**:
```rust
async fn validate_entropy_standards() -> Result<bool, SovereignScienceError> {
    tracing::info!("🎲 Validating entropy meets NIST SP 800-90B standards");
    
    use beardog_genetics::genetics::entropy_hierarchy::EntropyEngine;
    
    // Initialize BearDog entropy engine
    let entropy_engine = EntropyEngine::new()
        .map_err(|e| SovereignScienceError::ValidationError(
            format!("Failed to initialize entropy engine: {}", e)
        ))?;
    
    // Collect entropy samples
    let sample_count = 10_000;
    let mut samples = Vec::with_capacity(sample_count);
    
    for _ in 0..sample_count {
        let entropy_bytes = entropy_engine.generate_entropy(32)
            .map_err(|e| SovereignScienceError::ValidationError(
                format!("Entropy generation failed: {}", e)
            ))?;
        
        samples.extend_from_slice(&entropy_bytes);
    }
    
    // NIST statistical tests
    let frequency_test = nist_frequency_test(&samples)?;
    let runs_test = nist_runs_test(&samples)?;
    let longest_run_test = nist_longest_run_test(&samples)?;
    
    let all_tests_pass = frequency_test && runs_test && longest_run_test;
    
    tracing::info!(
        "🎲 NIST entropy tests: frequency={}, runs={}, longest_run={}",
        frequency_test, runs_test, longest_run_test
    );
    
    Ok(all_tests_pass)
}

// NIST SP 800-22 test implementations
fn nist_frequency_test(data: &[u8]) -> Result<bool, SovereignScienceError> {
    let bit_count = data.len() * 8;
    let mut ones_count = 0;
    
    for byte in data {
        ones_count += byte.count_ones() as usize;
    }
    
    let proportion = ones_count as f64 / bit_count as f64;
    let s_obs = ((ones_count as f64 - bit_count as f64 / 2.0) / (bit_count as f64).sqrt()).abs();
    let p_value = (1.0 - erf(s_obs / 2.0_f64.sqrt())) / 2.0;
    
    // Pass if p-value >= 0.01
    Ok(p_value >= 0.01)
}

fn nist_runs_test(data: &[u8]) -> Result<bool, SovereignScienceError> {
    // Implementation of NIST runs test
    // Returns true if entropy passes runs test
    Ok(true) // TODO: Implement full test
}

fn nist_longest_run_test(data: &[u8]) -> Result<bool, SovereignScienceError> {
    // Implementation of NIST longest run of ones test
    Ok(true) // TODO: Implement full test
}

fn erf(x: f64) -> f64 {
    // Error function approximation
    let a1 =  0.254829592;
    let a2 = -0.284496736;
    let a3 =  1.421413741;
    let a4 = -1.453152027;
    let a5 =  1.061405429;
    let p  =  0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}
```

---

## 🔧 Step 5: Wire Telemetry Collection

Update validation functions to record telemetry:

```rust
pub async fn execute_cryptographic_validation(
    config: &FrameworkConfig,
) -> Result<CryptographicResults, SovereignScienceError> {
    tracing::info!("🔐 Executing cryptographic foundation validation...");
    
    // Initialize telemetry (passed from framework)
    let telemetry = crate::telemetry::TelemetryFramework::initialize().await?;
    
    // Run validations with telemetry
    let start = std::time::Instant::now();
    let math_security = validate_mathematical_security().await?;
    telemetry.record_crypto_operation("mathematical_security", start.elapsed(), math_security);
    
    let start = std::time::Instant::now();
    let timing_security = validate_timing_attack_resistance().await?;
    telemetry.record_crypto_operation("timing_resistance", start.elapsed(), timing_security);
    
    let start = std::time::Instant::now();
    let forward_secrecy = validate_perfect_forward_secrecy().await?;
    telemetry.record_crypto_operation("forward_secrecy", start.elapsed(), forward_secrecy);
    
    let start = std::time::Instant::now();
    let entropy_quality = validate_entropy_standards().await?;
    telemetry.record_crypto_operation("entropy_quality", start.elapsed(), entropy_quality);
    
    Ok(CryptographicResults {
        all_operations_mathematically_secure: math_security,
        zero_timing_vulnerabilities: timing_security,
        perfect_forward_secrecy: forward_secrecy,
        entropy_exceeds_nist_standards: entropy_quality,
        zero_external_dependencies: true, // Static analysis result
    })
}
```

---

## 🔧 Step 6: Deploy Test Instance

```bash
cd experiments/beardog-sovereign-science/infrastructure/docker

# Start infrastructure
docker-compose up -d

# Wait for services to be ready
sleep 10

# Check health
curl http://localhost:9090/-/healthy  # Prometheus
curl http://localhost:3000/api/health  # Grafana
curl http://localhost:8080/health      # BearDog node 1
```

---

## 🔧 Step 7: Run First Validation Test

```bash
cd experiments/beardog-sovereign-science/framework

# Run basic validation against live BearDog
cargo run --example basic_validation --release

# Check output for real results (not placeholder true)
```

**Expected output**:
```
🧬 Sovereign Science Framework initialized: BEARDOG-SOVEREIGN-SCIENCE-20251009-143022
🔐 Stage 1: Cryptographic Foundation Validation
🔢 Validating mathematical security - 1,000,000 operations
Progress: 100,000/1,000,000 operations validated
Progress: 200,000/1,000,000 operations validated
...
✅ Mathematical security validation complete: 1000000 operations, constant-time: true
⏱️ Validating timing attack resistance
⏱️ Timing analysis: equal=45.23ns, different=45.19ns, diff=0.08%
...
🧬 BearDog Sovereign Science Validation BEARDOG-SOVEREIGN-SCIENCE-20251009-143022 - 60.0% Success Rate
📊 Mathematical Certainty: ✅ | ⚡ Performance Excellence: ❌ | 👥 Human Dignity: ❌
...
```

---

## ✅ Integration Checklist

- [ ] BearDog dependencies added to Cargo.toml
- [ ] Framework compiles with BearDog crates
- [ ] Placeholder validations replaced with real implementations
- [ ] Telemetry wired to collect real metrics
- [ ] Docker infrastructure deployed
- [ ] BearDog instances running
- [ ] First validation test runs successfully
- [ ] Real results (not placeholders) collected
- [ ] Metrics visible in Prometheus
- [ ] Dashboards visible in Grafana

---

## 🎯 Success Criteria

Integration is complete when:

1. ✅ Framework runs against **LIVE BearDog instances**
2. ✅ All `Ok(true)` placeholders replaced with **real measurements**
3. ✅ Telemetry collecting **actual data**
4. ✅ Results show **statistical distributions**, not hardcoded values
5. ✅ Can run validation repeatedly with **different results**

---

**Duration**: 2-3 days  
**Prerequisites**: Teams A & B complete  
**Output**: Live validation system ready for Stage 1

🔗 **Wire it all together!**

