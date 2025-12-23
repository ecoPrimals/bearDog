# Environment Configuration Template

This file documents all environment variables that can be used to configure BearDog. Copy the relevant sections to your `.env` file.

## Quick Start

For local development, create a `.env` file with:

```bash
# Minimal configuration
BEARDOG_DISCOVERY_HOST=localhost
BEARDOG_DISCOVERY_PORT=8080
BEARDOG_API_PORT=3000
BEARDOG_LOG_LEVEL=info
BEARDOG_HSM_PROVIDER=software
```

## Complete Configuration

See below for all available configuration options.

### Network Configuration

```bash
# Service Discovery
BEARDOG_DISCOVERY_HOST=localhost
BEARDOG_DISCOVERY_PORT=8080

# API Server
BEARDOG_API_HOST=0.0.0.0
BEARDOG_API_PORT=3000

# Database
BEARDOG_DB_HOST=localhost
BEARDOG_DB_PORT=5432
BEARDOG_DB_NAME=beardog
BEARDOG_DB_USER=beardog
BEARDOG_DB_PASSWORD=change_me_in_production
```

### Security Configuration

```bash
# HSM Configuration
BEARDOG_HSM_PROVIDER=software  # Options: software, pkcs11, cloud_kms, android, ios
BEARDOG_HSM_CONFIG_PATH=/etc/beardog/hsm.conf

# Encryption
BEARDOG_ENCRYPTION_KEY_PATH=/etc/beardog/keys
BEARDOG_ENCRYPTION_ALGORITHM=aes-256-gcm

# TLS/SSL
BEARDOG_TLS_ENABLED=true
BEARDOG_TLS_CERT_PATH=/etc/beardog/certs/server.crt
BEARDOG_TLS_KEY_PATH=/etc/beardog/certs/server.key
BEARDOG_TLS_CA_PATH=/etc/beardog/certs/ca.crt
```

### Monitoring & Observability

```bash
# Metrics
BEARDOG_METRICS_ENABLED=true
BEARDOG_METRICS_PORT=9090

# Logging
BEARDOG_LOG_LEVEL=info  # Options: trace, debug, info, warn, error
BEARDOG_LOG_FORMAT=json  # Options: json, text

# Tracing
BEARDOG_TRACING_ENABLED=false
BEARDOG_TRACING_ENDPOINT=http://localhost:4317
```

### Ecosystem Integration

```bash
# SongBird (Network Layer)
SONGBIRD_ENDPOINT=http://localhost:8081
SONGBIRD_API_KEY=your_songbird_api_key

# NestGate (Storage Layer)
NESTGATE_ENDPOINT=http://localhost:8082
NESTGATE_API_KEY=your_nestgate_api_key

# ToadStool (Compute Layer)
TOADSTOOL_ENDPOINT=http://localhost:8083
TOADSTOOL_API_KEY=your_toadstool_api_key

# BiomeOS Integration
BIOMEOS_ENABLED=false
BIOMEOS_CONFIG_PATH=/etc/beardog/biomeos.yaml
```

### Performance & Scalability

```bash
# Thread Pool
BEARDOG_WORKER_THREADS=4
BEARDOG_MAX_BLOCKING_THREADS=512

# Connection Pool
BEARDOG_DB_POOL_SIZE=10
BEARDOG_DB_POOL_TIMEOUT=30

# Timeouts (seconds)
BEARDOG_REQUEST_TIMEOUT=30
BEARDOG_CONNECTION_TIMEOUT=10
BEARDOG_IDLE_TIMEOUT=60
```

### Testing & Development

```bash
# Development Mode
BEARDOG_DEV_MODE=false
BEARDOG_MOCK_HSM=false
BEARDOG_MOCK_NETWORK=false

# Testing
BEARDOG_TEST_DB_HOST=localhost
BEARDOG_TEST_DB_PORT=5433
```

### Production Settings

```bash
# Deployment
BEARDOG_ENVIRONMENT=development  # Options: development, staging, production
BEARDOG_REGION=us-west-2

# Health Checks
BEARDOG_HEALTH_CHECK_ENABLED=true
BEARDOG_HEALTH_CHECK_INTERVAL=30

# Graceful Shutdown
BEARDOG_SHUTDOWN_TIMEOUT=30
```

## Security Notes

⚠️ **IMPORTANT:**
- NEVER commit `.env` files with real credentials to version control
- Use secrets management (e.g., HashiCorp Vault) in production
- Rotate credentials regularly
- Enable TLS in production
- Use strong, unique passwords

## References

- [PRODUCTION_READY_CHECKLIST.md](PRODUCTION_READY_CHECKLIST.md)
- [configs/README.md](configs/README.md)
- [BEARDOG_CODING_STANDARDS.md](BEARDOG_CODING_STANDARDS.md)

