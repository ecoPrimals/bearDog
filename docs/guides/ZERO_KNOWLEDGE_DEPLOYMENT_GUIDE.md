# Zero-Knowledge Deployment Guide

**Philosophy**: Each primal starts with ZERO hardcoded knowledge and discovers everything at runtime, like an infant learning about its world.

## Core Principles

1. **Self-Discovery**: Each primal discovers its own identity and capabilities
2. **Dynamic Discovery**: All external services discovered via universal adapter
3. **No Hardcoding**: Zero hardcoded primal names, vendor names, or endpoints
4. **Environment-First**: Configuration via environment variables → discovery → fallbacks
5. **Infant Deployment**: Can start without prior ecosystem knowledge

## Environment Variables for Zero-Knowledge Bootstrap

### Primal Self-Identity

```bash
# Primal Type (self-identification)
PRIMAL_TYPE=beardog          # Who am I? (beardog, songbird, toadstool, squirrel, nestgate, etc.)
SERVICE_TYPE=security        # Alternative: by function (security, network, compute, storage, ai)

# Primal Name (optional - auto-generated if not provided)
PRIMAL_NAME="BearDog Security Service"
SERVICE_NAME="Security Primal"

# Host identification (auto-detected if not provided)
HOSTNAME=secure-node-01      # Hostname for primal ID generation
COMPUTERNAME=SECURENODE01    # Windows alternative
```

### Network Configuration (No Hardcoded Ports)

```bash
# API Configuration
BEARDOG_SERVICE_HOST=0.0.0.0          # Bind address (default: 0.0.0.0)
BEARDOG_API_PORT=8080                  # API port (default: 8080)
BEARDOG_LOCALHOST=127.0.0.1            # Localhost address

# Service Ports (all configurable)
BEARDOG_METRICS_PORT=9090              # Metrics endpoint
BEARDOG_HEALTH_PORT=8081               # Health check port
BEARDOG_ADMIN_PORT=8082                # Admin interface
BEARDOG_DEBUG_PORT=8083                # Debug port
BEARDOG_MESH_PORT=8443                 # Service mesh port

# Network Bind Addresses
BEARDOG_BIND_ADDRESS=0.0.0.0           # Primary bind address
BEARDOG_MESH_BIND_ADDRESS=0.0.0.0      # Mesh bind address
```

### Discovery Configuration (Vendor Agnostic)

```bash
# Discovery Method (determines HOW we find other primals)
DISCOVERY_METHOD=environment   # Options: environment, dns, kubernetes, consul, mdns, manual
SERVICE_DISCOVERY_METHOD=dns   # Alternative name

# Discovery Endpoints (NO hardcoded vendors)
BEARDOG_DISCOVERY_ENDPOINT=http://discovery-service:8081/discover
DISCOVERY_URL=http://discovery.local:8081

# Service-specific Discovery (by capability, not by primal name)
COMPUTE_SERVICE_ENDPOINT=http://compute-provider:8081      # Any compute primal
MESH_SERVICE_ENDPOINT=http://mesh-provider:8082           # Any network primal
STORAGE_SERVICE_ENDPOINT=http://storage-provider:8083     # Any storage primal
AI_SERVICE_ENDPOINT=http://ai-provider:8084              # Any AI primal
SECRETS_SERVICE_ENDPOINT=http://secrets-provider:8200     # Any secrets manager

# Backup Endpoints (comma-separated)
BEARDOG_COMPUTE_BACKUPS=http://backup1:8081,http://backup2:8081
BEARDOG_STORAGE_BACKUPS=http://backup1:8083,http://backup2:8083
```

### Capability Discovery (Not Primal Names)

```bash
# Request capabilities, not specific primals
# ✅ Correct: Request by capability
SECURITY_CAPABILITY_ENDPOINT=http://security-service:8080
COMPUTE_CAPABILITY_ENDPOINT=http://compute-service:8081
NETWORK_CAPABILITY_ENDPOINT=http://network-service:8082

# ❌ Wrong: Request by primal name (DO NOT DO THIS)
# SONGBIRD_ENDPOINT=...  # NEVER hardcode primal names
# TOADSTOOL_ENDPOINT=... # NEVER hardcode primal names
```

### Vendor-Agnostic Configuration

```bash
# Container Orchestration (NO k8s hardcoding)
ORCHESTRATION_SERVICE_ENDPOINT=https://orchestrator:6443   # Could be k8s, nomad, swarm
CONTAINER_ORCHESTRATION_TYPE=kubernetes                    # or: nomad, swarm, ecs

# Service Registry (NO consul hardcoding)
SERVICE_REGISTRY_ENDPOINT=http://registry:8500             # Could be consul, etcd, k8s
SERVICE_REGISTRY_TYPE=consul                               # or: etcd, zookeeper, kubernetes

# Secrets Management (NO vault hardcoding)
SECRETS_MANAGEMENT_ENDPOINT=http://secrets:8200            # Could be vault, aws, azure
SECRETS_MANAGEMENT_TYPE=vault                              # or: aws, azure, gcp

# Database (vendor agnostic)
DATABASE_URL=postgresql://db-host:5432/beardog             # Any PostgreSQL-compatible
BEARDOG_DATABASE_URL=postgresql://db-host:5432/beardog
DB_MAX_CONNECTIONS=20
DB_MIN_CONNECTIONS=2
DB_CONNECTION_TIMEOUT=30
DB_IDLE_TIMEOUT=600
DB_SSL_ENABLED=true
DB_SSL_MODE=require
DB_SSL_CERT=/path/to/cert.pem
DB_SSL_KEY=/path/to/key.pem
```

### Timeouts and Policies (No Hardcoding)

```bash
# Connection Timeouts
BEARDOG_CONNECTION_TIMEOUT_MS=5000
BEARDOG_REQUEST_TIMEOUT_MS=30000

# Retry Policies
BEARDOG_MAX_RETRIES=3
BEARDOG_RETRY_INITIAL_DELAY_MS=100
BEARDOG_RETRY_MAX_DELAY_MS=5000
BEARDOG_RETRY_BACKOFF_MULTIPLIER=2.0

# Circuit Breaker
BEARDOG_CIRCUIT_BREAKER_FAILURE_THRESHOLD=5
BEARDOG_CIRCUIT_BREAKER_SUCCESS_THRESHOLD=3
BEARDOG_CIRCUIT_BREAKER_TIMEOUT_MS=60000

# Health Checks
BEARDOG_HEALTH_CHECK_INTERVAL_MS=30000
BEARDOG_HEALTH_CHECK_TIMEOUT_MS=5000
```

### TLS/Security Configuration

```bash
# TLS Configuration
BEARDOG_TLS_ENABLED=true
BEARDOG_VERIFY_CERTS=true
BEARDOG_CA_CERT=/path/to/ca.pem
BEARDOG_CLIENT_CERT=/path/to/client-cert.pem
BEARDOG_CLIENT_KEY=/path/to/client-key.pem
BEARDOG_MIN_TLS_VERSION=1.2
```

## Zero-Knowledge Deployment Examples

### Example 1: Complete Infant Deployment (BearDog)

```bash
#!/bin/bash
# deploy-beardog-infant.sh - Zero prior knowledge deployment

# Self-identification (only knows itself)
export PRIMAL_TYPE=beardog
export PRIMAL_NAME="BearDog Security Service"

# Network configuration (all from environment)
export BEARDOG_SERVICE_HOST=0.0.0.0
export BEARDOG_API_PORT=8080

# Discovery configuration (finds others at runtime)
export DISCOVERY_METHOD=dns
export BEARDOG_DISCOVERY_ENDPOINT=http://discovery.cluster.local:8081

# That's it! No hardcoded knowledge of other primals or vendors
docker run -e PRIMAL_TYPE -e PRIMAL_NAME \
           -e BEARDOG_SERVICE_HOST -e BEARDOG_API_PORT \
           -e DISCOVERY_METHOD -e BEARDOG_DISCOVERY_ENDPOINT \
           beardog:latest
```

### Example 2: Multi-Primal Ecosystem Bootstrap

```bash
#!/bin/bash
# bootstrap-ecosystem.sh - Start entire ecosystem with zero hardcoding

# Start BearDog (Security Primal)
docker run -d --name beardog \
  -e PRIMAL_TYPE=beardog \
  -e DISCOVERY_METHOD=mdns \
  beardog:latest

# Start Songbird (Network Primal) - NO knowledge of BearDog
docker run -d --name songbird \
  -e PRIMAL_TYPE=songbird \
  -e DISCOVERY_METHOD=mdns \
  songbird:latest

# Start Toadstool (Compute Primal) - NO knowledge of others
docker run -d --name toadstool \
  -e PRIMAL_TYPE=toadstool \
  -e DISCOVERY_METHOD=mdns \
  toadstool:latest

# Start Nestgate (Storage Primal) - NO knowledge of others
docker run -d --name nestgate \
  -e PRIMAL_TYPE=nestgate \
  -e DISCOVERY_METHOD=mdns \
  nestgate:latest

# Start Squirrel (AI Primal) - NO knowledge of others
docker run -d --name squirrel \
  -e PRIMAL_TYPE=squirrel \
  -e DISCOVERY_METHOD=mdns \
  squirrel:latest

# All primals discover each other via mDNS - zero hardcoded connections!
```

### Example 3: Kubernetes Deployment (Vendor Agnostic)

```yaml
# beardog-deployment.yaml - Zero-knowledge Kubernetes deployment

apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: beardog
        image: beardog:latest
        env:
        # Self-identification
        - name: PRIMAL_TYPE
          value: "beardog"
        - name: HOSTNAME
          valueFrom:
            fieldRef:
              fieldPath: metadata.name
        
        # Discovery (uses k8s service discovery)
        - name: DISCOVERY_METHOD
          value: "kubernetes"
        - name: BEARDOG_DISCOVERY_ENDPOINT
          value: "http://discovery-service:8081"
        
        # Network (no hardcoded ports)
        - name: BEARDOG_API_PORT
          value: "8080"
        - name: BEARDOG_SERVICE_HOST
          value: "0.0.0.0"
        
        # NO hardcoded primal names or vendor references!
        # Everything discovered at runtime
```

### Example 4: Multi-Cloud Deployment (Cloud Agnostic)

```bash
#!/bin/bash
# deploy-cloud-agnostic.sh - Works on ANY cloud provider

# Cloud-agnostic configuration
export PRIMAL_TYPE=beardog
export DISCOVERY_METHOD=dns

# Secrets from cloud provider (abstracted)
export SECRETS_MANAGEMENT_ENDPOINT=$CLOUD_SECRETS_ENDPOINT  # AWS/Azure/GCP
export SECRETS_MANAGEMENT_TYPE=$CLOUD_TYPE                  # Detected at runtime

# Service discovery from cloud provider (abstracted)
export SERVICE_REGISTRY_ENDPOINT=$CLOUD_REGISTRY_ENDPOINT   # Any registry
export SERVICE_REGISTRY_TYPE=$CLOUD_REGISTRY_TYPE           # Detected at runtime

# Container orchestration from cloud provider (abstracted)
export ORCHESTRATION_SERVICE_ENDPOINT=$CLOUD_ORCHESTRATOR   # Any orchestrator

# Deploy! Works on AWS, Azure, GCP, on-prem, etc.
./deploy.sh
```

## Testing Zero-Knowledge Deployment

### Test 1: Infant Deployment Test

```bash
#!/bin/bash
# test-infant-deployment.sh

# Start primal with MINIMAL configuration
docker run --rm \
  -e PRIMAL_TYPE=beardog \
  beardog:latest check-infant-deployment

# Expected output:
# ✅ Self-identity discovered: beardog-hostname-uuid
# ✅ Capabilities auto-detected: [Security, KeyManagement, HSM]
# ✅ Endpoints discovered: [http://0.0.0.0:8080, http://0.0.0.0:8443]
# ✅ Zero hardcoded knowledge verified
# ✅ Ready for ecosystem integration
```

### Test 2: Discovery Integration Test

```bash
#!/bin/bash
# test-discovery-integration.sh

# Start discovery service
docker run -d --name discovery -p 8081:8081 discovery-service:latest

# Start beardog (finds discovery service)
docker run -d --name beardog \
  -e PRIMAL_TYPE=beardog \
  -e DISCOVERY_METHOD=dns \
  -e BEARDOG_DISCOVERY_ENDPOINT=http://discovery:8081 \
  beardog:latest

# Verify beardog registered itself
curl http://localhost:8081/services | jq '.services[] | select(.type=="beardog")'

# Expected: beardog listed with self-discovered capabilities
```

### Test 3: Multi-Primal Communication Test

```bash
#!/bin/bash
# test-multi-primal-communication.sh

# Start primals with mDNS discovery
docker run -d --name beardog -e PRIMAL_TYPE=beardog -e DISCOVERY_METHOD=mdns beardog:latest
docker run -d --name songbird -e PRIMAL_TYPE=songbird -e DISCOVERY_METHOD=mdns songbird:latest

# Request network capability from beardog (should find songbird)
docker exec beardog beardog-cli request-capability --type network

# Expected output:
# ✅ Discovered network capability provider: songbird-host-uuid
# ✅ Connected to network primal
# ✅ Communication established
# (NO hardcoded "songbird" in beardog code!)
```

## Migration Checklist

When deploying a primal, ensure:

- [ ] **NO hardcoded primal names** in code or config
- [ ] **NO hardcoded vendor names** (k8s, consul, vault, etc.)
- [ ] **NO hardcoded ports** - all from environment/config
- [ ] **NO hardcoded endpoints** - all discovered
- [ ] `PRIMAL_TYPE` environment variable set (or defaults to "primal")
- [ ] Discovery method configured (environment, dns, kubernetes, consul, mdns)
- [ ] Can start with only self-identification
- [ ] Passes infant deployment test
- [ ] Discovers other primals via capability, not name
- [ ] Works in isolated environment (no assumptions about ecosystem)

## Success Metrics

A successful zero-knowledge deployment should:

1. ✅ Start with ONLY `PRIMAL_TYPE` and `DISCOVERY_METHOD`
2. ✅ Self-discover identity (UUID-based, no hardcoded names)
3. ✅ Auto-detect capabilities (filesystem/runtime introspection)
4. ✅ Announce to discovery service (found via DNS/env/etc.)
5. ✅ Discover peer capabilities dynamically
6. ✅ Establish connections via universal adapter (O(1) complexity)
7. ✅ Operate normally without any hardcoded ecosystem knowledge

## Troubleshooting

### Problem: "No discovery service found"
**Solution**: Set `BEARDOG_DISCOVERY_ENDPOINT` or use `DISCOVERY_METHOD=mdns`

### Problem: "Cannot find capability provider"
**Solution**: Verify other primals are registered in discovery service

### Problem: "Hardcoded primal name error"
**Solution**: Check code for direct primal references, use capability types instead

### Problem: "Port already in use"
**Solution**: Configure unique port via `BEARDOG_API_PORT` environment variable

## References

- `HARDCODING_ELIMINATION_PLAN.md` - Overall elimination strategy
- `UNIVERSAL_ADAPTER_SPECIFICATION.md` - Universal adapter architecture
- `ecosystem-templates/primal-hardcoding-elimination-template.rs` - Code patterns
- `BEARDOG_CODING_STANDARDS.md` - Primal sovereignty principles

