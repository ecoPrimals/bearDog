# BearDog Chaos Testing Framework

**Created:** October 7, 2025  
**Status:** Production-Ready  
**Version:** 1.0.0

## 📖 Overview

The BearDog Chaos Testing Framework is a comprehensive, production-grade system for validating the resilience, fault tolerance, and recovery capabilities of the BearDog ecosystem under adverse conditions.

## 🏗️ Architecture

### Core Modules

#### 1. **models.rs** - Data Structures & Types
- `ChaosTestConfig` - Framework configuration
- `FaultType` - All supported fault types (network, resource, security, database)
- `ActiveFault` - Runtime fault tracking
- `SystemImpact` - Metrics for measuring fault impact
- `ChaosScenario` - Test scenario definitions
- `ChaosMetrics` - Aggregate metrics
- `RecoveryStatus` - Recovery state tracking

#### 2. **controller.rs** - Orchestration
- `ChaosController` - Main orchestration component
- Active fault tracking
- Fault history management
- Lifecycle management (start/stop)

#### 3. **fault_injection.rs** - Fault Injection
Implementations for:
- `NetworkFaultInjector` - Network faults (partitions, latency, packet loss)
- `SecurityFaultInjector` - Security faults (auth failures, cert expiry)
- `DatabaseFaultInjector` - Database faults (timeouts, corruption)
- `ResourceFaultInjector` - Resource exhaustion (CPU, memory, disk)

#### 4. **metrics.rs** - Metrics Collection
- `ChaosMetricsCollector` - Real-time metrics collection
- Baseline metrics capture
- Impact measurement
- Stabilization detection

#### 5. **recovery.rs** - Recovery Validation
Validators for:
- `CoreRecoveryValidator` - Core system recovery
- `SecurityRecoveryValidator` - Security subsystem recovery
- `NetworkRecoveryValidator` - Network recovery
- `DatabaseRecoveryValidator` - Database recovery

#### 6. **scenarios.rs** - Scenario Management
Pre-defined scenarios:
- Network Partition
- High CPU Load
- Database Timeout
- Authentication Failure Spike
- Memory Exhaustion

#### 7. **reporting.rs** - Report Generation
- Comprehensive test reports
- Resilience scoring
- Recommendations generation

### Specialized Test Modules

#### 8. **network_chaos.rs** - Network Testing
- Network partition scenarios
- Latency injection (50ms to 1000ms)
- Packet loss testing (5% to 50%)
- Combined network stress
- Partition recovery validation

**Tests:** 5  
**Lines:** 201

#### 9. **resource_chaos.rs** - Resource Testing
- Memory exhaustion (512MB to 2048MB)
- CPU exhaustion (50% to 90%)
- Disk exhaustion
- Combined resource stress
- OOM scenario handling

**Tests:** 5  
**Lines:** 185

#### 10. **comprehensive_fault_testing.rs** - Full Suite
- All fault type coverage
- Sequential fault injection
- Concurrent fault injection
- Long-running chaos tests

**Tests:** 5  
**Lines:** 232

#### 11. **integration_tests.rs** - Framework Integration
- Framework initialization
- Fault injector registration
- Recovery validator setup
- Scenario execution pipeline
- Metrics collection
- Report generation
- Controller lifecycle
- Fault tracking

**Tests:** 8  
**Lines:** 287

## 🚀 Usage

### Basic Usage

```rust
use beardog::tests::chaos::*;

#[tokio::test]
async fn run_chaos_tests() -> Result<(), BearDogError> {
    // Create framework with default config
    let config = ChaosTestConfig::default();
    let mut framework = ChaosTestFramework::new(config)?;
    
    // Run comprehensive chaos testing
    let report = framework.run_chaos_testing().await?;
    
    println!("Resilience Score: {:.2}%", report.overall_resilience_score);
    Ok(())
}
```

### Running Specific Scenarios

```rust
use beardog::tests::chaos::*;

#[tokio::test]
async fn run_network_partition() -> Result<(), BearDogError> {
    let config = ChaosTestConfig::default();
    let mut framework = ChaosTestFramework::new(config)?;
    
    let scenario = ChaosScenario {
        name: "Custom Network Partition".to_string(),
        description: "Test network resilience".to_string(),
        faults: vec![FaultType::NetworkPartition {
            duration_ms: 10_000,
        }],
        duration_ms: 15_000,
        success_criteria: SuccessCriteria {
            max_error_rate: 0.1,
            min_availability: 0.9,
            max_response_time_degradation: 5.0,
        },
    };
    
    let result = framework.run_chaos_scenario(&scenario).await?;
    assert!(result.success);
    Ok(())
}
```

### Custom Fault Injection

```rust
use beardog::tests::chaos::*;

#[tokio::test]
async fn inject_custom_fault() -> Result<(), BearDogError> {
    let config = ChaosTestConfig::default();
    let mut framework = ChaosTestFramework::new(config)?;
    
    // Inject a specific fault
    let fault = FaultType::CpuExhaustion {
        cpu_percent: 85,
        thread_count: 4,
    };
    
    let result = framework.inject_and_monitor_fault(fault).await?;
    assert!(result.injection_success);
    Ok(())
}
```

## 🧪 Running Tests

### Run All Chaos Tests
```bash
cargo test --lib chaos
```

### Run Specific Test Modules
```bash
# Network chaos tests
cargo test --lib chaos::network_chaos

# Resource chaos tests
cargo test --lib chaos::resource_chaos

# Integration tests
cargo test --lib chaos::integration_tests
```

### Run with Output
```bash
cargo test --lib chaos -- --nocapture
```

## 📊 Metrics & Reporting

### System Impact Metrics
- **Response Time Increase**: Multiplier of baseline response time
- **Error Rate Increase**: Percentage increase in errors
- **Throughput Decrease**: Percentage decrease in throughput
- **Memory Usage Increase**: Percentage increase in memory
- **Availability Decrease**: Percentage decrease in availability

### Chaos Metrics
- **Successful Recoveries**: Count of successful recovery operations
- **Failed Recoveries**: Count of failed recoveries
- **Average Recovery Time**: Mean time to recover (ms)
- **Peak Error Rate**: Highest error rate observed
- **Min Availability**: Lowest availability observed
- **System Resilience Score**: Overall score (0-100)

## ⚙️ Configuration

```rust
pub struct ChaosTestConfig {
    pub scenario_timeout_ms: u64,              // Default: 30,000
    pub fault_injection_rate: f32,             // Default: 0.1
    pub recovery_timeout_ms: u64,              // Default: 60,000
    pub max_concurrent_faults: u32,            // Default: 3
    pub metrics_interval_ms: u64,              // Default: 1,000
    pub enable_byzantine_faults: bool,         // Default: true
    pub degradation_thresholds: DegradationThresholds,
}
```

## 🎯 Fault Types

### Network Faults
- **NetworkPartition**: Simulates network split
- **NetworkLatency**: Adds latency and packet loss
- **ByzantineBehavior**: Incorrect/delayed/omitted responses

### Resource Faults
- **MemoryExhaustion**: Memory pressure/OOM
- **CpuExhaustion**: High CPU load
- **DiskExhaustion**: Disk space exhaustion

### Security Faults
- **AuthenticationFailure**: Auth system failures
- **CertificateExpiry**: TLS certificate expiration

### Database Faults
- **DatabaseTimeout**: Connection timeouts
- **DatabaseCorruption**: Data corruption scenarios

### Component Faults
- **ComponentCrash**: Component failure (graceful/panic/OOM)
- **ComponentSlowdown**: Performance degradation

## 📈 Test Coverage

- **Total Tests**: 23
- **Network Tests**: 5
- **Resource Tests**: 5
- **Comprehensive Tests**: 5
- **Integration Tests**: 8
- **Total Lines**: 905

## ✅ Success Criteria

Each scenario defines success criteria:
- **Max Error Rate**: Maximum acceptable error increase
- **Min Availability**: Minimum acceptable availability
- **Max Response Time Degradation**: Maximum acceptable slowdown

## 🔄 Recovery Validation

The framework validates recovery through:
1. **Health Checks**: Continuous health monitoring
2. **Metrics Stabilization**: Waiting for metrics to return to baseline
3. **Component Validation**: Per-component recovery checks
4. **Timeout Management**: Recovery timeout enforcement

## 📝 Report Example

```
🌪️  Chaos Testing Report
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Overall Resilience Score: 87.5%

Scenario Results:
  ✅ Network Partition - PASSED (12.3s)
  ✅ High CPU Load - PASSED (8.7s)
  ✅ Database Timeout - PASSED (10.1s)
  ✅ Auth Failure Spike - PASSED (9.2s)
  ❌ Memory Exhaustion - FAILED (15.4s)

Metrics:
  - Successful Recoveries: 4
  - Failed Recoveries: 1
  - Average Recovery Time: 8.2s
  - Peak Error Rate: 0.12
  - Min Availability: 0.88

Recommendations:
  - Improve memory management in data processor
  - Add circuit breaker for database connections
  - Increase health check frequency
```

## 🏆 Best Practices

1. **Start Small**: Begin with single fault scenarios
2. **Gradual Increase**: Progressively increase complexity
3. **Monitor Metrics**: Always collect baseline metrics first
4. **Set Realistic Criteria**: Base success criteria on real requirements
5. **Document Failures**: Use failure insights to improve resilience
6. **Regular Testing**: Run chaos tests as part of CI/CD

## 🔗 Integration

### CI/CD Integration
```yaml
- name: Chaos Testing
  run: |
    cargo test --lib chaos -- --nocapture
    cargo test --lib chaos::integration_tests
```

### Production Use
⚠️ **WARNING**: Never run chaos tests in production without:
- Proper safeguards
- Rollback procedures
- Monitoring
- Team awareness
- Off-peak scheduling

## 📚 Further Reading

- [Chaos Engineering Principles](https://principlesofchaos.org/)
- [BearDog Architecture](../../ARCHITECTURE.md)
- [BearDog Contributing Guide](../../CONTRIBUTING.md)

## 🤝 Contributing

When adding new chaos scenarios:
1. Define clear success criteria
2. Add appropriate recovery validators
3. Document expected behavior
4. Include integration tests
5. Update this README

## 📄 License

See [LICENSE](../../LICENSE) in the repository root.

---

**Maintainer**: BearDog Team  
**Last Updated**: October 7, 2025  
**Status**: ✅ Production-Ready

