# BearDog Environment Variables Reference

**Version**: 3.0  
**Date**: November 21, 2025  
**Status**: Production Ready

## Overview

Complete reference for all environment variables supported by BearDog.

## Quick Reference

| Variable | Default | Type | Purpose |
|----------|---------|------|---------|
| `BEARDOG_API_PORT` | 8080 | u16 | Main API server port |
| `BEARDOG_DISCOVERY_PORT` | 9090 | u16 | Service discovery port |
| `BEARDOG_ADMIN_PORT` | 9091 | u16 | Admin interface port |
| `BEARDOG_HTTPS_PORT` | 8443 | u16 | HTTPS API port |
| `BEARDOG_METRICS_PORT` | 9100 | u16 | Metrics/monitoring port |
| `BEARDOG_HEALTH_PORT` | 8081 | u16 | Health check port |

## Network Ports

### `BEARDOG_API_PORT`

**Default**: `8080`  
**Type**: `u16`  
**Range**: `1024-65535`

Main HTTP API endpoint port.

```bash
export BEARDOG_API_PORT=8080
```

**Usage**:
```rust
use beardog_config::global::api_port;
let port = api_port();
```

---

### `BEARDOG_DISCOVERY_PORT`

**Default**: `9090`  
**Type**: `u16`  
**Range**: `1024-65535`

Service discovery and registration port.

```bash
export BEARDOG_DISCOVERY_PORT=9090
```

---

### `BEARDOG_ADMIN_PORT`

**Default**: `9091`  
**Type**: `u16`  
**Range**: `1024-65535`

Administrative operations and management port.

```bash
export BEARDOG_ADMIN_PORT=9091
```

---

### `BEARDOG_HTTPS_PORT`

**Default**: `8443`  
**Type**: `u16`  
**Range**: `1024-65535`

Secure HTTPS API endpoint port.

```bash
export BEARDOG_HTTPS_PORT=8443
```

**Usage**:
```rust
use beardog_config::global::https_port;
let port = https_port();
```

---

### `BEARDOG_METRICS_PORT`

**Default**: `9100`  
**Type**: `u16`  
**Range**: `1024-65535`

Prometheus metrics and monitoring port.

```bash
export BEARDOG_METRICS_PORT=9100
```

**Usage**:
```rust
use beardog_config::global::metrics_port;
let port = metrics_port();
```

---

### `BEARDOG_HEALTH_PORT`

**Default**: `8081`  
**Type**: `u16`  
**Range**: `1024-65535`

Health and liveness probe endpoint port.

```bash
export BEARDOG_HEALTH_PORT=8081
```

**Usage**:
```rust
use beardog_config::global::health_port;
let port = health_port();
```

---

## Network Addresses

### `BEARDOG_API_BIND_ADDRESS`

**Default**: `127.0.0.1`  
**Type**: IP Address  

API server bind address.

```bash
export BEARDOG_API_BIND_ADDRESS=0.0.0.0  # Listen on all interfaces
export BEARDOG_API_BIND_ADDRESS=127.0.0.1  # Localhost only (secure default)
```

---

### `BEARDOG_PROMETHEUS_ENDPOINT`

**Default**: `http://127.0.0.1:9100`  
**Type**: URL  

Prometheus metrics endpoint.

```bash
export BEARDOG_PROMETHEUS_ENDPOINT=http://prometheus:9090
```

---

## Configuration Hierarchy

BearDog uses the following priority order (highest to lowest):

1. **Command-line arguments** (if provided)
2. **Environment variables** (`BEARDOG_*`)
3. **Configuration file** (`config.toml`)
4. **Platform defaults**
5. **Secure fallback defaults**

## Examples

### Development Environment

```bash
# Development with default ports
export BEARDOG_API_PORT=8080
export BEARDOG_METRICS_PORT=9100
export BEARDOG_HEALTH_PORT=8081
export BEARDOG_API_BIND_ADDRESS=127.0.0.1
```

### Production Environment

```bash
# Production with custom ports
export BEARDOG_API_PORT=80
export BEARDOG_HTTPS_PORT=443
export BEARDOG_METRICS_PORT=9100
export BEARDOG_HEALTH_PORT=8081
export BEARDOG_API_BIND_ADDRESS=0.0.0.0
```

### Docker Environment

```dockerfile
FROM beardog:latest

ENV BEARDOG_API_PORT=8080
ENV BEARDOG_METRICS_PORT=9100
ENV BEARDOG_HEALTH_PORT=8081
ENV BEARDOG_API_BIND_ADDRESS=0.0.0.0

EXPOSE 8080 9100 8081
```

### Kubernetes Environment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
spec:
  template:
    spec:
      containers:
      - name: beardog
        image: beardog:latest
        env:
        - name: BEARDOG_API_PORT
          value: "8080"
        - name: BEARDOG_METRICS_PORT
          value: "9100"
        - name: BEARDOG_HEALTH_PORT
          value: "8081"
        ports:
        - containerPort: 8080
          name: api
        - containerPort: 9100
          name: metrics
        - containerPort: 8081
          name: health
```

## Validation

All environment variables are validated at startup:

- **Ports**: Must be in range 1024-65535 (non-privileged)
- **Ports**: Must be unique (no conflicts)
- **IP Addresses**: Must be valid IPv4 or IPv6
- **URLs**: Must be valid HTTP/HTTPS URLs

### Validation Errors

**Invalid Port**:
```
Error: Port configuration validation failed: api_port (80) should be non-privileged (>1024)
```

**Port Conflict**:
```
Error: Port configuration validation failed: Port conflict detected: 8080
```

## Best Practices

### ✅ DO

- Use non-privileged ports (>1024) when possible
- Set `BEARDOG_API_BIND_ADDRESS=127.0.0.1` in development
- Set `BEARDOG_API_BIND_ADDRESS=0.0.0.0` in production (behind firewall)
- Document all environment variables in deployment

### ❌ DON'T

- Use privileged ports (<1024) without sudo/capabilities
- Bind to `0.0.0.0` in untrusted environments
- Use the same port for multiple services
- Hardcode values instead of using environment variables

## Troubleshooting

### Port Already in Use

```bash
# Check what's using the port
lsof -i :8080

# Use a different port
export BEARDOG_API_PORT=8081
```

### Permission Denied

```bash
# Using privileged port (<1024) without permission
Error: Permission denied (os error 13)

# Solution: Use non-privileged port
export BEARDOG_API_PORT=8080  # >1024
```

### Configuration Not Loading

```bash
# Verify environment variable is set
echo $BEARDOG_API_PORT

# Check spelling (case-sensitive)
export BEARDOG_API_PORT=8080  # ✅ Correct
export BEARDOG_api_port=8080  # ❌ Wrong (case)
export API_PORT=8080          # ❌ Wrong (missing BEARDOG_ prefix)
```

## Testing

### Unit Tests

```rust
#[test]
fn test_custom_port() {
    std::env::set_var("BEARDOG_API_PORT", "9999");
    use beardog_config::NetworkPortsConfig;
    
    let config = NetworkPortsConfig::from_env();
    assert_eq!(config.api_port, 9999);
}
```

### Integration Tests

```bash
# Set test environment
export BEARDOG_API_PORT=8888
export BEARDOG_METRICS_PORT=9888
export BEARDOG_HEALTH_PORT=8889

# Run tests
cargo test
```

## See Also

- [Network Ports Migration Guide](./NETWORK_PORTS_MIGRATION_GUIDE.md)
- [Configuration Guide](../../configs/README.md)
- [Production Deployment Guide](../PRODUCTION_DEPLOYMENT_GUIDE.md)

## Version History

- **3.0** (Nov 21, 2025): Added centralized port configuration
- **2.0**: Added validation and error handling
- **1.0**: Initial environment variable support
