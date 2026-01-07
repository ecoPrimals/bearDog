# 🔧 Environment Variables Reference

**Version**: 0.15.0  
**Date**: January 6, 2026  
**Status**: ✅ Complete Reference

---

## 📚 Overview

BearDog follows **primal sovereignty** principles where all configuration is environment-driven:
- ✅ Zero hardcoded IPs, ports, or service names
- ✅ Runtime discovery of other primals
- ✅ Self-knowledge only (no hardcoded primal names)

---

## 🎯 Quick Start

### Minimal Configuration

```bash
# Required
export FAMILY_ID=nat0
export NODE_ID=tower1

# Optional (with defaults)
export BEARDOG_HSM_MODE=software
export RUST_LOG=info

# Start server
./beardog-server
```

**Result**: BearDog starts with:
- Unix socket IPC: `/tmp/beardog-nat0-tower1.sock`
- HTTP API: Random port (no conflicts)
- Software HSM: No hardware required

---

## 📋 Complete Reference

### Core Identity (REQUIRED)

#### `FAMILY_ID`

**Purpose**: Genetic family identifier defining trust boundaries  
**Required**: ✅ **YES**  
**Example**: `nat0`, `prod`, `dev-team-alpha`  
**Used For**:
- Trust evaluation (same family = trusted)
- Unix socket path construction
- Genetic lineage calculations

```bash
export FAMILY_ID=nat0
```

---

#### `NODE_ID`

**Purpose**: Unique node identifier within the family  
**Required**: ✅ **YES**  
**Example**: `tower1`, `tower2`, `dev-laptop`  
**Used For**:
- Node identification in trust evaluation
- Unix socket path construction
- Metrics and logging

```bash
export NODE_ID=tower1
```

---

### HSM Configuration

#### `BEARDOG_HSM_MODE`

**Purpose**: Hardware Security Module mode  
**Required**: No  
**Default**: `software`  
**Options**: `software` | `yubikey` | `tpm` | `android_strongbox` | `ios_secure_enclave`  
**Example**:

```bash
# Development (no hardware)
export BEARDOG_HSM_MODE=software

# Production (YubiKey)
export BEARDOG_HSM_MODE=yubikey

# Production (TPM 2.0)
export BEARDOG_HSM_MODE=tpm
```

---

#### `BEARDOG_YUBIKEY_SERIAL`

**Purpose**: Specific YubiKey serial number (optional)  
**Required**: No  
**Default**: Auto-detect first available  
**Example**:

```bash
export BEARDOG_YUBIKEY_SERIAL=12345678
```

---

#### `BEARDOG_TPM_PATH`

**Purpose**: TPM device path (optional)  
**Required**: No  
**Default**: `/dev/tpm0` or `/dev/tpmrm0`  
**Example**:

```bash
export BEARDOG_TPM_PATH=/dev/tpmrm0
```

---

### Network Configuration

#### `BEARDOG_BIND_ADDR`

**Purpose**: HTTP API bind address (OPTIONAL - Unix socket is primary)  
**Required**: No  
**Default**: `0.0.0.0:0` (random port to avoid conflicts)  
**Special**: Set to empty string to disable HTTP entirely  
**Example**:

```bash
# Random port (recommended)
export BEARDOG_BIND_ADDR=0.0.0.0:0

# Specific port (development)
export BEARDOG_BIND_ADDR=127.0.0.1:9000

# Disable HTTP entirely (production)
export BEARDOG_BIND_ADDR=
```

**Note**: Port `0` means the OS assigns a random available port, preventing conflicts when running multiple instances.

---

#### `BEARDOG_ENDPOINT`

**Purpose**: Full endpoint URL for service registration  
**Required**: No  
**Default**: Constructed from `BEARDOG_HOST` and port  
**Example**:

```bash
export BEARDOG_ENDPOINT=http://server.example.com:9000
```

---

#### `BEARDOG_HOST`

**Purpose**: Hostname for endpoint construction  
**Required**: No  
**Default**: `localhost`  
**Used When**: `BEARDOG_ENDPOINT` is not set  
**Example**:

```bash
export BEARDOG_HOST=server.example.com
```

---

#### `BEARDOG_API_PORT`

**Purpose**: API port for endpoint construction  
**Required**: No  
**Default**: `9000`  
**Example**:

```bash
export BEARDOG_API_PORT=8443
```

---

### Unix Socket IPC (PRIMARY)

Unix socket paths are **automatically constructed**:

```
/tmp/beardog-${FAMILY_ID}-${NODE_ID}.sock
```

**Examples**:
- `FAMILY_ID=nat0 NODE_ID=tower1` → `/tmp/beardog-nat0-tower1.sock`
- `FAMILY_ID=prod NODE_ID=tower2` → `/tmp/beardog-prod-tower2.sock`

**Benefits**:
- ✅ No port conflicts (ever)
- ✅ Multiple instances on same machine
- ✅ Secure (Unix file permissions)
- ✅ Fast (no network stack overhead)

---

### Registry & Discovery

#### `PRIMAL_REGISTRY_SOCKET`

**Purpose**: Primal registry Unix socket path  
**Required**: No  
**Default**: `/tmp/primal-registry-${FAMILY_ID}.sock`  
**Example**:

```bash
export PRIMAL_REGISTRY_SOCKET=/var/run/primal-registry.sock
```

---

#### `BEARDOG_UPA_URL`

**Purpose**: Universal Primal Adapter (UPA) service URL  
**Required**: No  
**Default**: `https://localhost:8080`  
**Example**:

```bash
export BEARDOG_UPA_URL=https://upa.example.com:8080
```

---

### Service Configuration

#### `BEARDOG_SERVICE_NAME`

**Purpose**: Service name for registry/discovery  
**Required**: No  
**Default**: `beardog-security-provider`  
**Example**:

```bash
export BEARDOG_SERVICE_NAME=beardog-prod-us-west
```

---

#### `BEARDOG_HEARTBEAT_INTERVAL`

**Purpose**: Heartbeat interval in seconds  
**Required**: No  
**Default**: `30`  
**Example**:

```bash
export BEARDOG_HEARTBEAT_INTERVAL=60
```

---

### Logging & Monitoring

#### `RUST_LOG`

**Purpose**: Rust logging level  
**Required**: No  
**Default**: `info`  
**Options**: `error` | `warn` | `info` | `debug` | `trace`  
**Example**:

```bash
# Simple
export RUST_LOG=debug

# Module-specific
export RUST_LOG=beardog=debug,beardog_tunnel=trace

# Production
export RUST_LOG=info
```

---

#### `BEARDOG_LOG_FORMAT`

**Purpose**: Log output format  
**Required**: No  
**Default**: `pretty` (human-readable)  
**Options**: `pretty` | `json`  
**Example**:

```bash
# Production (structured logging)
export BEARDOG_LOG_FORMAT=json

# Development (human-readable)
export BEARDOG_LOG_FORMAT=pretty
```

---

#### `BEARDOG_LOG_FILE`

**Purpose**: Log file path  
**Required**: No  
**Default**: stdout  
**Example**:

```bash
export BEARDOG_LOG_FILE=/var/log/beardog/beardog.log
```

---

### Security & Trust

#### `BEARDOG_TRUST_MODE`

**Purpose**: Trust evaluation strictness  
**Required**: No  
**Default**: `strict`  
**Options**: `strict` | `lenient` | `permissive`  
**Example**:

```bash
# Production
export BEARDOG_TRUST_MODE=strict

# Testing
export BEARDOG_TRUST_MODE=lenient
```

---

#### `BEARDOG_MTLS_ENABLED`

**Purpose**: Enable mutual TLS for HTTP API  
**Required**: No  
**Default**: `false`  
**Example**:

```bash
export BEARDOG_MTLS_ENABLED=true
export BEARDOG_TLS_CERT=/etc/beardog/certs/server.crt
export BEARDOG_TLS_KEY=/etc/beardog/certs/server.key
```

---

### Performance Tuning

#### `BEARDOG_WORKER_THREADS`

**Purpose**: Tokio worker thread count  
**Required**: No  
**Default**: CPU count  
**Example**:

```bash
export BEARDOG_WORKER_THREADS=8
```

---

#### `BEARDOG_CONNECTION_POOL_SIZE`

**Purpose**: Connection pool size  
**Required**: No  
**Default**: `100`  
**Example**:

```bash
export BEARDOG_CONNECTION_POOL_SIZE=200
```

---

#### `BEARDOG_REQUEST_TIMEOUT`

**Purpose**: Request timeout in seconds  
**Required**: No  
**Default**: `30`  
**Example**:

```bash
export BEARDOG_REQUEST_TIMEOUT=60
```

---

### Development & Testing

#### `BEARDOG_DEV_MODE`

**Purpose**: Enable development mode  
**Required**: No  
**Default**: `false`  
**Effect**: Relaxed security, verbose logging  
**Example**:

```bash
export BEARDOG_DEV_MODE=true
```

---

#### `BEARDOG_API_DOCS_ENABLED`

**Purpose**: Enable API documentation endpoint  
**Required**: No  
**Default**: `false`  
**Example**:

```bash
export BEARDOG_API_DOCS_ENABLED=true
```

---

#### `BEARDOG_METRICS_ENABLED`

**Purpose**: Enable metrics endpoint  
**Required**: No  
**Default**: `true`  
**Example**:

```bash
export BEARDOG_METRICS_ENABLED=false
```

---

## 🎭 Example Configurations

### Development (Single Instance)

```bash
export FAMILY_ID=dev
export NODE_ID=dev-1
export BEARDOG_HSM_MODE=software
export BEARDOG_BIND_ADDR=127.0.0.1:9000
export RUST_LOG=debug
export BEARDOG_DEV_MODE=true

./beardog-server
```

---

### Production (Primary Tower)

```bash
export FAMILY_ID=prod
export NODE_ID=tower1
export BEARDOG_HSM_MODE=yubikey
export BEARDOG_BIND_ADDR=  # Unix socket only
export BEARDOG_TRUST_MODE=strict
export BEARDOG_MTLS_ENABLED=true
export BEARDOG_TLS_CERT=/etc/beardog/certs/server.crt
export BEARDOG_TLS_KEY=/etc/beardog/certs/server.key
export RUST_LOG=info
export BEARDOG_LOG_FORMAT=json
export BEARDOG_LOG_FILE=/var/log/beardog/tower1.log

./beardog-server
```

---

### Production (Secondary Tower)

```bash
export FAMILY_ID=prod
export NODE_ID=tower2
export BEARDOG_HSM_MODE=tpm
export BEARDOG_BIND_ADDR=  # Unix socket only
export BEARDOG_TRUST_MODE=strict
export RUST_LOG=info
export BEARDOG_LOG_FORMAT=json
export BEARDOG_LOG_FILE=/var/log/beardog/tower2.log

./beardog-server
```

---

### Dual Towers on Same Machine (Testing)

**Terminal 1**:
```bash
export FAMILY_ID=nat0
export NODE_ID=tower1
export BEARDOG_HSM_MODE=software
export RUST_LOG=info

./beardog-server
# Creates: /tmp/beardog-nat0-tower1.sock
```

**Terminal 2**:
```bash
export FAMILY_ID=nat0
export NODE_ID=tower2
export BEARDOG_HSM_MODE=software
export RUST_LOG=info

./beardog-server
# Creates: /tmp/beardog-nat0-tower2.sock
```

**Result**: Both instances run without conflicts!

---

## 🚀 Best Practices

### ✅ DO: Use Unix Sockets for IPC

```bash
# Primary: Unix socket IPC
export BEARDOG_BIND_ADDR=  # Empty = Unix socket only
```

**Benefits**: Secure, fast, no port conflicts

---

### ✅ DO: Use Port 0 for HTTP (if needed)

```bash
# HTTP with random port
export BEARDOG_BIND_ADDR=0.0.0.0:0
```

**Benefits**: No port conflicts, multi-instance friendly

---

### ✅ DO: Use Environment Files

```bash
# Load from .env
source .env
./beardog-server

# Or use systemd
EnvironmentFile=/etc/beardog/beardog.env
```

---

### ❌ DON'T: Hardcode in Scripts

```bash
# ❌ BAD
./beardog-server --port 9000 --host localhost

# ✅ GOOD
export BEARDOG_BIND_ADDR=0.0.0.0:9000
./beardog-server
```

---

## 📚 Related Documents

- `.env.example` - Example environment file (copy to `.env`)
- `README.md` - Project overview
- `HARDCODING_AUDIT_JAN_6_2026.md` - Hardcoding audit results
- `JAN_6_2026_SESSION_COMPLETE.md` - Session summary

---

## 🎯 Summary

| Category | Variables | Status |
|----------|-----------|--------|
| **Core Identity** | 2 required | ✅ |
| **HSM** | 3 optional | ✅ |
| **Network** | 4 optional | ✅ |
| **Registry** | 2 optional | ✅ |
| **Logging** | 3 optional | ✅ |
| **Security** | 4 optional | ✅ |
| **Performance** | 3 optional | ✅ |
| **Development** | 3 optional | ✅ |

**Total**: 24 environment variables (2 required, 22 optional)

---

_Last Updated: January 6, 2026_  
_Version: 0.15.0_  
_Status: Production Ready_

