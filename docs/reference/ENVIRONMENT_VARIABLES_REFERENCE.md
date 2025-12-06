# 🌐 Environment Variables Reference - BearDog

**Version**: 1.0  
**Date**: November 27, 2025  
**Status**: Complete reference for all BearDog environment variables

---

## 📋 TABLE OF CONTENTS

- [Network Configuration](#network-configuration)
- [Service Discovery](#service-discovery)
- [External Integrations](#external-integrations)
- [Database Configuration](#database-configuration)
- [Performance & Limits](#performance--limits)
- [Monitoring & Logging](#monitoring--logging)
- [Security](#security)
- [HSM Configuration](#hsm-configuration)
- [Feature Flags](#feature-flags)
- [Container Orchestration](#container-orchestration)

---

## 🌐 NETWORK CONFIGURATION

### Port Configuration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_API_PORT` | 8080 | Main HTTP API port | No |
| `BEARDOG_DISCOVERY_PORT` | 9090 | Service discovery port | No |
| `BEARDOG_ADMIN_PORT` | 9091 | Admin operations port | No |
| `BEARDOG_HTTPS_PORT` | 8443 | HTTPS API port | No |
| `BEARDOG_METRICS_PORT` | 9100 | Prometheus metrics port | No |
| `BEARDOG_HEALTH_PORT` | 8081 | Health check endpoint port | No |
| `BEARDOG_WEBSOCKET_PORT` | 8082 | WebSocket connections port | No |

**Example**:
```bash
export BEARDOG_API_PORT=8080
export BEARDOG_HTTPS_PORT=8443
export BEARDOG_METRICS_PORT=9100
```

### Address Configuration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_API_HOST` | 127.0.0.1 | API server hostname | No |
| `BEARDOG_BIND_ADDRESS` | 127.0.0.1 | Server bind address | No |
| `BEARDOG_EXTERNAL_HOST` | localhost | Public-facing hostname | No |
| `BEARDOG_MULTICAST_ADDRESS` | 239.255.0.1 | Multicast for discovery | No |

**Example**:
```bash
# Development
export BEARDOG_API_HOST="127.0.0.1"
export BEARDOG_BIND_ADDRESS="127.0.0.1"

# Production
export BEARDOG_API_HOST="0.0.0.0"  # Bind to all interfaces
export BEARDOG_EXTERNAL_HOST="beardog.yourdomain.com"
```

---

## 🔍 SERVICE DISCOVERY

### Discovery Configuration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `ECOSYSTEM_DISCOVERY_URL` | - | Primary discovery endpoint | No |
| `ECOSYSTEM_DISCOVERY_PORTS` | 8080,8081,8082,8083 | Ports to scan for services | No |
| `ECOSYSTEM_HOST` | auto-detect | Base host for discovery | No |
| `BEARDOG_LOCAL_HOST` | localhost | Local host fallback | No |

**Example**:
```bash
export ECOSYSTEM_DISCOVERY_URL="http://discovery.cluster.local:8080"
export ECOSYSTEM_DISCOVERY_PORTS="8080,8081,8082"
export ECOSYSTEM_HOST="beardog-discovery.default.svc.cluster.local"
```

### Service Mesh

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_MESH_DISCOVERY_TIMEOUT_SECS` | 30 | Discovery timeout | No |
| `BEARDOG_MESH_HEALTH_CHECK_INTERVAL_SECS` | 60 | Health check interval | No |
| `BEARDOG_SERVICE_NAME` | beardog-discovery | Service name for mesh | No |
| `BEARDOG_NAMESPACE` | default | Namespace for service | No |

**Example**:
```bash
export BEARDOG_MESH_DISCOVERY_TIMEOUT_SECS=30
export BEARDOG_MESH_HEALTH_CHECK_INTERVAL_SECS=60
export BEARDOG_SERVICE_NAME="beardog-production"
export BEARDOG_NAMESPACE="production"
```

---

## 🔗 EXTERNAL INTEGRATIONS

### HashiCorp Vault

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `VAULT_ADDR` | - | Vault server address | Yes (if using Vault) |
| `BEARDOG_VAULT_URL` | http://localhost:8200 | Backup Vault URL | No |
| `VAULT_TOKEN` | - | Vault authentication token | Yes (if using Vault) |
| `VAULT_NAMESPACE` | - | Vault namespace | No |

**Example**:
```bash
export VAULT_ADDR="https://vault.production.com:8200"
export VAULT_TOKEN="s.xxxxxxxxxxxxxx"
export VAULT_NAMESPACE="beardog-production"
```

### Songbird Integration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `SONGBIRD_ENDPOINT` | - | Songbird service endpoint | Yes (if using Songbird) |
| `BEARDOG_SONGBIRD_ENDPOINT` | localhost:8443 | Backup endpoint | No |
| `SONGBIRD_HOST` | localhost | Songbird hostname | No |
| `SONGBIRD_PORT` | 8443 | Songbird port | No |

**Example**:
```bash
export SONGBIRD_ENDPOINT="songbird.cluster.local:8443"
# OR construct from components:
export SONGBIRD_HOST="songbird.cluster.local"
export SONGBIRD_PORT=8443
```

---

## 🗄️ DATABASE CONFIGURATION

### Connection Settings

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `DATABASE_URL` | - | Full database connection string | Yes (if using DB) |
| `DATABASE_TIMEOUT_SECONDS` | 30 | Connection timeout | No |
| `DATABASE_SSL` | false | Enable SSL connections | No |
| `DATABASE_MAX_CONNECTIONS` | 100 | Connection pool size | No |
| `DATABASE_MIN_CONNECTIONS` | 10 | Minimum pool size | No |

**Example**:
```bash
# PostgreSQL
export DATABASE_URL="postgresql://user:pass@db.local:5432/beardog"
export DATABASE_SSL=true
export DATABASE_MAX_CONNECTIONS=100

# With SSL mode
export DATABASE_URL="postgresql://user:pass@db.local:5432/beardog?sslmode=require"
```

---

## ⚡ PERFORMANCE & LIMITS

### Timeouts

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_OPERATION_TIMEOUT_SECS` | 30 | General operation timeout | No |
| `BEARDOG_REQUEST_TIMEOUT_SECS` | 30 | HTTP request timeout | No |
| `BEARDOG_SHUTDOWN_TIMEOUT_SECS` | 30 | Graceful shutdown timeout | No |

**Example**:
```bash
export BEARDOG_OPERATION_TIMEOUT_SECS=30
export BEARDOG_REQUEST_TIMEOUT_SECS=60
export BEARDOG_SHUTDOWN_TIMEOUT_SECS=30
```

### Connection & Resource Limits

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_MAX_RETRIES` | 3 | Maximum retry attempts | No |
| `BEARDOG_MAX_CONNECTIONS` | 1000 | Maximum concurrent connections | No |
| `BEARDOG_BUFFER_SIZE` | 8192 | Buffer size in bytes | No |
| `BEARDOG_WORKER_THREADS` | CPU cores | Number of worker threads | No |

**Example**:
```bash
export BEARDOG_MAX_RETRIES=3
export BEARDOG_MAX_CONNECTIONS=1000
export BEARDOG_BUFFER_SIZE=8192
export BEARDOG_WORKER_THREADS=8
```

---

## 📊 MONITORING & LOGGING

### Logging Configuration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_LOG_LEVEL` | info | Log level (trace, debug, info, warn, error) | No |
| `RUST_LOG` | - | Rust logging directives | No |
| `BEARDOG_LOG_FORMAT` | json | Log format (json, text) | No |
| `BEARDOG_LOG_OUTPUT` | stdout | Output destination (stdout, file) | No |

**Example**:
```bash
# Production
export BEARDOG_LOG_LEVEL=info
export RUST_LOG=beardog=info,beardog_core=debug
export BEARDOG_LOG_FORMAT=json

# Development
export BEARDOG_LOG_LEVEL=debug
export RUST_LOG=beardog=trace
export BEARDOG_LOG_FORMAT=text
```

### Metrics & Telemetry

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_METRICS_ENABLED` | true | Enable Prometheus metrics | No |
| `BEARDOG_TELEMETRY_ENABLED` | true | Enable telemetry collection | No |
| `BEARDOG_TRACING_ENABLED` | false | Enable distributed tracing | No |
| `JAEGER_ENDPOINT` | - | Jaeger collector endpoint | No |

**Example**:
```bash
export BEARDOG_METRICS_ENABLED=true
export BEARDOG_TELEMETRY_ENABLED=true
export BEARDOG_TRACING_ENABLED=true
export JAEGER_ENDPOINT="http://jaeger.cluster.local:14268"
```

---

## 🔒 SECURITY

### Authentication & Authorization

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_ENABLE_AUTHENTICATION` | true | Require authentication | No |
| `BEARDOG_JWT_SECRET` | - | JWT signing secret | Yes (if auth enabled) |
| `BEARDOG_TOKEN_EXPIRY_HOURS` | 24 | Token expiration time | No |

**Example**:
```bash
export BEARDOG_ENABLE_AUTHENTICATION=true
export BEARDOG_JWT_SECRET="your-secret-key-here"
export BEARDOG_TOKEN_EXPIRY_HOURS=24
```

### Rate Limiting

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_ENABLE_RATE_LIMITING` | true | Enable rate limiting | No |
| `BEARDOG_RATE_LIMIT_REQUESTS_PER_MINUTE` | 1000 | Max requests per minute | No |
| `BEARDOG_RATE_LIMIT_BURST` | 100 | Burst capacity | No |

**Example**:
```bash
export BEARDOG_ENABLE_RATE_LIMITING=true
export BEARDOG_RATE_LIMIT_REQUESTS_PER_MINUTE=1000
export BEARDOG_RATE_LIMIT_BURST=100
```

---

## 🔐 HSM CONFIGURATION

### Hardware Security Module

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_HSM_PREFER_HARDWARE` | true | Prefer hardware HSM | No |
| `SOFTHSM2_CONF` | /etc/softhsm2/softhsm2.conf | SoftHSM config path | No |
| `PKCS11_LIBRARY` | auto-detect | PKCS#11 library path | No |

**Example**:
```bash
# Use hardware HSM if available
export BEARDOG_HSM_PREFER_HARDWARE=true

# SoftHSM configuration
export SOFTHSM2_CONF=/etc/softhsm2/softhsm2.conf

# Specific PKCS#11 library
export PKCS11_LIBRARY=/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so
```

---

## 🚩 FEATURE FLAGS

### Experimental Features

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `BEARDOG_ENABLE_EXPERIMENTAL_FEATURES` | false | Enable experimental features | No |
| `BEARDOG_ENABLE_DEBUG_ENDPOINTS` | false | Enable debug endpoints | No |
| `BEARDOG_ENABLE_PROFILING` | false | Enable performance profiling | No |

**Example**:
```bash
# Staging/Development
export BEARDOG_ENABLE_EXPERIMENTAL_FEATURES=true
export BEARDOG_ENABLE_DEBUG_ENDPOINTS=true
export BEARDOG_ENABLE_PROFILING=true

# Production
export BEARDOG_ENABLE_EXPERIMENTAL_FEATURES=false
export BEARDOG_ENABLE_DEBUG_ENDPOINTS=false
export BEARDOG_ENABLE_PROFILING=false
```

---

## ☸️ CONTAINER ORCHESTRATION

### Kubernetes Integration

| Variable | Default | Description | Required |
|----------|---------|-------------|----------|
| `KUBERNETES_SERVICE_HOST` | - | K8s API server host | Auto-set by K8s |
| `CONTAINER_ORCHESTRATION_HOST` | - | Generic orchestration host | No |
| `BEARDOG_ORCHESTRATION_SERVICE_PORT` | 8080 | Service port in orchestration | No |
| `ORCHESTRATION_NAMESPACE` | default | Namespace in orchestration | No |

**Example**:
```bash
# Usually auto-set by Kubernetes, but can override:
export BEARDOG_ORCHESTRATION_SERVICE_PORT=8080
export ORCHESTRATION_NAMESPACE=production
export BEARDOG_SERVICE_NAME=beardog-api
```

---

## 📝 CONFIGURATION HIERARCHY

BearDog uses the following configuration hierarchy (highest priority first):

1. **Command-line Arguments** (highest priority)
2. **Environment Variables** (`BEARDOG_*`)
3. **Config File** (`/etc/beardog/config.toml` or `~/.config/beardog/config.toml`)
4. **Platform Defaults** (auto-detected based on OS/environment)
5. **Fallback Constants** (lowest priority)

**Example**:
```bash
# This will override config file settings:
export BEARDOG_API_PORT=9000

# This will be used if BEARDOG_API_PORT is not set:
# config.toml: api_port = 8080

# This will be used if neither env var nor config file is set:
# Default: 8080
```

---

## 🎯 QUICK START TEMPLATES

### Development Environment
```bash
#!/bin/bash
# dev-env.sh

export BEARDOG_LOG_LEVEL=debug
export RUST_LOG=beardog=trace
export BEARDOG_API_HOST="127.0.0.1"
export BEARDOG_BIND_ADDRESS="127.0.0.1"
export BEARDOG_ENABLE_DEBUG_ENDPOINTS=true
export BEARDOG_HSM_PREFER_HARDWARE=false
```

### Staging Environment
```bash
#!/bin/bash
# staging-env.sh

export BEARDOG_LOG_LEVEL=info
export RUST_LOG=beardog=info
export BEARDOG_API_HOST="0.0.0.0"
export BEARDOG_EXTERNAL_HOST="staging.beardog.com"
export BEARDOG_ENABLE_DEBUG_ENDPOINTS=true
export BEARDOG_METRICS_ENABLED=true
export BEARDOG_TELEMETRY_ENABLED=true
```

### Production Environment
```bash
#!/bin/bash
# production-env.sh

export BEARDOG_LOG_LEVEL=info
export RUST_LOG=beardog=info,beardog_core=info
export BEARDOG_API_HOST="0.0.0.0"
export BEARDOG_EXTERNAL_HOST="beardog.production.com"
export BEARDOG_ENABLE_DEBUG_ENDPOINTS=false
export BEARDOG_METRICS_ENABLED=true
export BEARDOG_TELEMETRY_ENABLED=true
export BEARDOG_ENABLE_RATE_LIMITING=true
export BEARDOG_HSM_PREFER_HARDWARE=true
export DATABASE_SSL=true
```

---

## 🔍 VALIDATION

### Check Configuration
```bash
# Verify environment variables are set
env | grep BEARDOG_ | sort

# Test configuration
beardog --check-config

# Show effective configuration
beardog --show-config
```

### Common Issues

**Issue**: Service won't start
```bash
# Check required variables
echo "API_PORT: $BEARDOG_API_PORT"
echo "LOG_LEVEL: $BEARDOG_LOG_LEVEL"
```

**Issue**: Database connection fails
```bash
# Verify DATABASE_URL
echo "$DATABASE_URL"
psql "$DATABASE_URL" -c "SELECT 1"
```

**Issue**: External services unreachable
```bash
# Check endpoints
echo "Vault: $VAULT_ADDR"
echo "Songbird: $SONGBIRD_ENDPOINT"
curl -I "$VAULT_ADDR/v1/sys/health"
```

---

## 📚 ADDITIONAL RESOURCES

- [Configuration Guide](docs/guides/CONFIGURATION_GUIDE.md)
- [Deployment Checklist](STAGING_DEPLOYMENT_CHECKLIST.md)
- [Troubleshooting Guide](docs/guides/TROUBLESHOOTING_GUIDE.md)
- [Security Best Practices](SECURITY.md)

---

**Last Updated**: November 27, 2025  
**Maintained By**: BearDog Development Team  
**Version**: 1.0

🐻 **BearDog: Fully Configurable, Zero Hardcoding!**

