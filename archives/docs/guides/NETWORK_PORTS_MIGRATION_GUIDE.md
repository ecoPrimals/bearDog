# Network Ports Migration Guide

**Version**: 1.0  
**Date**: November 21, 2025  
**Status**: Production Ready

## Overview

This guide helps migrate from hardcoded network ports to the new centralized `NetworkPortsConfig` system.

## What Changed?

### Before (Hardcoded)

```rust
// ❌ OLD: Hardcoded ports
let api_url = "http://localhost:8080";
let metrics_url = "http://localhost:9090/metrics";
let health_url = "http://localhost:8081/health";
```

### After (Configurable)

```rust
// ✅ NEW: Centralized configuration
use beardog_config::global::{BEARDOG_CONFIG, metrics_port, health_port};

let api_port = BEARDOG_CONFIG.network.ports.api_port;
let metrics_url = format!("http://localhost:{}/metrics", metrics_port());
let health_url = format!("http://localhost:{}/health", health_port());
```

## Migration Paths

### Option 1: Global Config (Recommended)

**Best for**: Most use cases, production code

```rust
use beardog_config::global::BEARDOG_CONFIG;

fn start_api_server() {
    let port = BEARDOG_CONFIG.network.ports.api_port;
    println!("Starting API on port {}", port);
}
```

### Option 2: Convenience Functions

**Best for**: Quick access, cleaner code

```rust
use beardog_config::global::{api_port, metrics_port, health_port};

fn server_info() {
    println!("API: {}", api_port());
    println!("Metrics: {}", metrics_port());
    println!("Health: {}", health_port());
}
```

### Option 3: Load Custom Config

**Best for**: Testing, special scenarios

```rust
use beardog_config::NetworkPortsConfig;

fn test_custom_ports() {
    std::env::set_var("BEARDOG_METRICS_PORT", "9999");
    let ports = NetworkPortsConfig::from_env();
    assert_eq!(ports.metrics_port, 9999);
}
```

## Available Ports

| Port Field | Default | Environment Variable | Purpose |
|------------|---------|---------------------|---------|
| `api_port` | 8080 | `BEARDOG_API_PORT` | Main API server |
| `discovery_port` | 9090 | `BEARDOG_DISCOVERY_PORT` | Service discovery |
| `admin_port` | 9091 | `BEARDOG_ADMIN_PORT` | Admin operations |
| `https_port` | 8443 | `BEARDOG_HTTPS_PORT` | HTTPS API |
| `metrics_port` | 9100 | `BEARDOG_METRICS_PORT` | Metrics/monitoring |
| `health_port` | 8081 | `BEARDOG_HEALTH_PORT` | Health checks |

## Configuration Methods

### 1. Environment Variables (Highest Priority)

```bash
export BEARDOG_API_PORT=9000
export BEARDOG_METRICS_PORT=9100
export BEARDOG_HEALTH_PORT=9101

# Start your application
./beardog-server
```

### 2. Config File

**`config.toml`**:
```toml
[network.ports]
api_port = 9000
discovery_port = 9090
admin_port = 9091
https_port = 8443
metrics_port = 9100
health_port = 9101
```

### 3. Runtime Configuration

```rust
use beardog_config::NetworkPortsConfig;

let mut ports = NetworkPortsConfig::default();
ports.api_port = 9000;
ports.metrics_port = 9100;

// Validate before use
ports.validate().expect("Invalid port configuration");
```

## Common Migration Scenarios

### Scenario 1: Replace Hardcoded Constant

**Before**:
```rust
const API_PORT: u16 = 8080;

fn connect() {
    let url = format!("http://localhost:{}", API_PORT);
}
```

**After**:
```rust
use beardog_config::global::BEARDOG_CONFIG;

fn connect() {
    let port = BEARDOG_CONFIG.network.ports.api_port;
    let url = format!("http://localhost:{}", port);
}
```

### Scenario 2: Replace Environment Variable Fallback

**Before**:
```rust
let port = std::env::var("API_PORT")
    .ok()
    .and_then(|s| s.parse().ok())
    .unwrap_or(8080);
```

**After**:
```rust
// The config system handles this automatically
use beardog_config::global::api_port;
let port = api_port();
```

### Scenario 3: Testing with Custom Ports

**Before**:
```rust
#[test]
fn test_api() {
    let port = 8888; // Test port
    test_server_on_port(port);
}
```

**After**:
```rust
#[test]
fn test_api() {
    std::env::set_var("BEARDOG_API_PORT", "8888");
    use beardog_config::global::api_port;
    test_server_on_port(api_port());
}
```

## Validation

The config system includes automatic validation:

```rust
use beardog_config::NetworkPortsConfig;

let ports = NetworkPortsConfig::default();

// Validates:
// - All ports > 1024 (non-privileged)
// - No port conflicts
// - All ports < 65535
match ports.validate() {
    Ok(_) => println!("✅ Configuration valid"),
    Err(e) => eprintln!("❌ Configuration error: {}", e),
}
```

## Backward Compatibility

### Legacy API (Still Supported)

The old individual port configurations still work during migration:

```rust
use beardog_config::global::BEARDOG_CONFIG;

// ⚠️ LEGACY (still works, but deprecated path)
let api_port = BEARDOG_CONFIG.network.api.port;
let discovery_port = BEARDOG_CONFIG.network.discovery.port;
let admin_port = BEARDOG_CONFIG.network.admin.port;

// ✅ NEW (preferred)
let api_port = BEARDOG_CONFIG.network.ports.api_port;
let metrics_port = BEARDOG_CONFIG.network.ports.metrics_port;
let health_port = BEARDOG_CONFIG.network.ports.health_port;
```

## Docker/Kubernetes Configuration

### Docker Compose

```yaml
version: '3.8'
services:
  beardog:
    image: beardog:latest
    environment:
      - BEARDOG_API_PORT=8080
      - BEARDOG_METRICS_PORT=9100
      - BEARDOG_HEALTH_PORT=8081
    ports:
      - "8080:8080"
      - "9100:9100"
      - "8081:8081"
```

### Kubernetes ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-ports
data:
  BEARDOG_API_PORT: "8080"
  BEARDOG_METRICS_PORT: "9100"
  BEARDOG_HEALTH_PORT: "8081"
  BEARDOG_ADMIN_PORT: "9091"
```

## Benefits

### 1. **Zero Hardcoding**
- All ports configurable without code changes
- Runtime configuration via environment

### 2. **Validation**
- Automatic conflict detection
- Range validation (>1024, <65535)
- Clear error messages

### 3. **Consistency**
- Single source of truth
- Centralized configuration
- No scattered constants

### 4. **Testability**
- Easy to override in tests
- No test pollution
- Isolated test environments

## Troubleshooting

### Issue: Port Already in Use

**Error**: `Address already in use (os error 48)`

**Solution**:
```bash
# Check what's using the port
lsof -i :8080

# Change port via environment
export BEARDOG_API_PORT=8081
```

### Issue: Port Validation Fails

**Error**: `Port configuration validation failed: api_port (80) should be non-privileged (>1024)`

**Solution**:
```bash
# Use non-privileged port (>1024)
export BEARDOG_API_PORT=8080
```

### Issue: Port Conflicts

**Error**: `Port conflict detected: 8080`

**Solution**: Ensure all ports are unique:
```bash
export BEARDOG_API_PORT=8080
export BEARDOG_METRICS_PORT=9100  # Different from API
export BEARDOG_HEALTH_PORT=8081   # Different from both
```

## Best Practices

### ✅ DO

- Use `NetworkPortsConfig` for all new code
- Use convenience functions (`api_port()`) for cleaner code
- Validate configuration before use
- Document port requirements

### ❌ DON'T

- Hardcode port numbers in production code
- Mix old and new configuration styles
- Use privileged ports (<1024) without reason
- Skip validation in critical paths

## Examples

### Complete Server Setup

```rust
use beardog_config::global::BEARDOG_CONFIG;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load and validate configuration
    let ports = &BEARDOG_CONFIG.network.ports;
    ports.validate()
        .map_err(|e| format!("Configuration error: {}", e))?;
    
    // Start services on configured ports
    let api_addr = format!("0.0.0.0:{}", ports.api_port);
    let metrics_addr = format!("0.0.0.0:{}", ports.metrics_port);
    let health_addr = format!("0.0.0.0:{}", ports.health_port);
    
    println!("🚀 BearDog Server Starting");
    println!("   API:     {}", api_addr);
    println!("   Metrics: {}", metrics_addr);
    println!("   Health:  {}", health_addr);
    
    // Start your servers...
    Ok(())
}
```

## Support

- **Documentation**: `/docs/guides/`
- **Config Reference**: `beardog-config/src/domains/network_ports.rs`
- **Examples**: `/examples/`

## Version History

- **1.0** (Nov 21, 2025): Initial release with `NetworkPortsConfig`
- All ports configurable via environment or config file
- Comprehensive validation and error messages
- Backward compatible with legacy configuration

