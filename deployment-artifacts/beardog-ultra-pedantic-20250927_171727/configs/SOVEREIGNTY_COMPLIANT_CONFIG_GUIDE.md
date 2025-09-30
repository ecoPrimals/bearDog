# 🏆 BearDog Sovereignty-Compliant Configuration Guide

## 🎯 **MISSION: ZERO HARDCODED CONFIGURATION**

This guide ensures **perfect primal sovereignty** by eliminating ALL hardcoded configurations. Each primal discovers its environment dynamically through capability-based patterns and environment variables.

---

## 🌟 **SOVEREIGNTY PRINCIPLES**

### ✅ **1. Zero Hardcoded Knowledge**
- No hardcoded IP addresses, ports, or service names
- All endpoints discovered via environment variables or capability discovery
- Each primal only knows its own configuration requirements

### ✅ **2. Environment-Based Discovery**
- All external dependencies configured via environment variables
- Graceful fallbacks to capability-based discovery when env vars missing
- Dynamic service discovery preferred over static configuration

### ✅ **3. Capability-Based Integration**
- Services discovered by capability type, not hardcoded names
- Universal adapter handles all inter-primal communication
- Zero vendor lock-in through abstracted interfaces

---

## 🔧 **SOVEREIGNTY-COMPLIANT CONFIGURATION PATTERNS**

### **❌ VIOLATIONS (Old Hardcoded Patterns)**
```toml
# ❌ SOVEREIGNTY VIOLATION: Hardcoded primal names
songbird_endpoint = "http://songbird:8080"
toadstool_url = "http://toadstool.cluster:9090" 
squirrel_api = "https://squirrel.ai.internal:443"

# ❌ SOVEREIGNTY VIOLATION: Hardcoded network assumptions
api_host = "127.0.0.1"
database_url = "postgresql://user:pass@localhost:5432/db"
redis_host = "redis.cluster.local"
```

### **✅ COMPLIANCE (New Capability-Based Patterns)**
```toml
# ✅ SOVEREIGNTY COMPLIANT: Capability-based discovery
service_mesh_endpoint = "${SERVICE_MESH_ENDPOINT}"
compute_service_url = "${COMPUTE_SERVICE_ENDPOINT}"
ai_service_api = "${AI_SERVICE_ENDPOINT}"

# ✅ SOVEREIGNTY COMPLIANT: Environment-based configuration
api_host = "${BEARDOG_API_HOST:-0.0.0.0}"
database_url = "${DATABASE_URL}"
cache_endpoint = "${CACHE_SERVICE_ENDPOINT}"

# ✅ SOVEREIGNTY COMPLIANT: Capability discovery fallbacks
[capability_discovery]
enabled = true
timeout_ms = "${DISCOVERY_TIMEOUT_MS:-5000}"
retry_attempts = "${DISCOVERY_RETRIES:-3}"
```

---

## 🌍 **ENVIRONMENT VARIABLE STANDARDS**

### **Core Service Discovery**
```bash
# Universal adapter configuration
export UNIVERSAL_ADAPTER_ENABLED=true
export CAPABILITY_DISCOVERY_TIMEOUT=5000
export SERVICE_DISCOVERY_ENDPOINT="http://discovery.ecosystem.internal:8080"

# Service mesh capability
export SERVICE_MESH_ENDPOINT="http://mesh.ecosystem.internal:8080"
export SERVICE_MESH_TIMEOUT=10000

# Compute intelligence capability  
export COMPUTE_SERVICE_ENDPOINT="http://compute.ecosystem.internal:9090"
export COMPUTE_SERVICE_TIMEOUT=30000

# AI/ML capability
export AI_SERVICE_ENDPOINT="http://ai.ecosystem.internal:8443"
export AI_SERVICE_TIMEOUT=60000

# Data storage capability
export STORAGE_SERVICE_ENDPOINT="http://storage.ecosystem.internal:5432"
export STORAGE_SERVICE_TIMEOUT=15000

# Monitoring capability
export MONITORING_ENDPOINT="http://monitoring.ecosystem.internal:3000"
export METRICS_ENDPOINT="http://metrics.ecosystem.internal:9091"
```

### **Network & Infrastructure**
```bash
# BearDog core configuration
export BEARDOG_API_HOST="0.0.0.0"
export BEARDOG_API_PORT=8080
export BEARDOG_ADMIN_PORT=9999

# Database configuration (no hardcoded credentials!)
export DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_POOL_SIZE=10
export DATABASE_TIMEOUT=30000

# Cache configuration
export CACHE_SERVICE_ENDPOINT="redis://${CACHE_HOST}:${CACHE_PORT}"
export CACHE_TTL_SECONDS=3600

# Security configuration
export JWT_SECRET="${JWT_SECRET}"  # Must be provided
export ENCRYPTION_KEY="${ENCRYPTION_KEY}"  # Must be provided
export TLS_CERT_PATH="${TLS_CERT_PATH}"
export TLS_KEY_PATH="${TLS_KEY_PATH}"
```

### **Ecosystem Integration**
```bash
# Capability discovery configuration
export ECOSYSTEM_DISCOVERY_ENABLED=true
export ECOSYSTEM_DISCOVERY_INTERVAL=30000
export ECOSYSTEM_HEALTH_CHECK_INTERVAL=10000

# Cross-primal communication
export INTER_PRIMAL_TIMEOUT=15000
export INTER_PRIMAL_RETRIES=3
export INTER_PRIMAL_CIRCUIT_BREAKER=true

# Vendor-agnostic integrations
export CONTAINER_ORCHESTRATOR="${ORCHESTRATOR_TYPE:-kubernetes}"
export CLOUD_PROVIDER="${CLOUD_PROVIDER:-generic}"
export MESSAGE_QUEUE_TYPE="${QUEUE_TYPE:-universal}"
```

---

## 📋 **DEPLOYMENT ENVIRONMENT TEMPLATES**

### **Development Environment**
```bash
#!/bin/bash
# dev-environment.sh - Development sovereignty-compliant configuration

# Core services (local development)
export BEARDOG_API_HOST="127.0.0.1"
export BEARDOG_API_PORT=8080

# Capability discovery (local)
export SERVICE_MESH_ENDPOINT="http://localhost:8080"
export COMPUTE_SERVICE_ENDPOINT="http://localhost:9090"
export AI_SERVICE_ENDPOINT="http://localhost:8443"
export STORAGE_SERVICE_ENDPOINT="postgresql://dev:dev@localhost:5432/beardog_dev"
export MONITORING_ENDPOINT="http://localhost:3000"

# Universal adapter (enabled for development)
export UNIVERSAL_ADAPTER_ENABLED=true
export CAPABILITY_DISCOVERY_TIMEOUT=5000

echo "✅ Development environment configured with sovereignty compliance"
```

### **Production Environment**
```bash
#!/bin/bash  
# prod-environment.sh - Production sovereignty-compliant configuration

# Core services (production)
export BEARDOG_API_HOST="0.0.0.0"
export BEARDOG_API_PORT=8080

# Capability discovery (production)
export SERVICE_MESH_ENDPOINT="${SERVICE_MESH_ENDPOINT}"
export COMPUTE_SERVICE_ENDPOINT="${COMPUTE_SERVICE_ENDPOINT}"
export AI_SERVICE_ENDPOINT="${AI_SERVICE_ENDPOINT}"
export STORAGE_SERVICE_ENDPOINT="${DATABASE_URL}"
export MONITORING_ENDPOINT="${MONITORING_ENDPOINT}"

# Universal adapter (production settings)
export UNIVERSAL_ADAPTER_ENABLED=true
export CAPABILITY_DISCOVERY_TIMEOUT=10000
export ECOSYSTEM_DISCOVERY_ENABLED=true

# Validate required environment variables
required_vars=(
    "SERVICE_MESH_ENDPOINT"
    "COMPUTE_SERVICE_ENDPOINT"
    "AI_SERVICE_ENDPOINT"
    "DATABASE_URL"
    "MONITORING_ENDPOINT"
    "JWT_SECRET"
    "ENCRYPTION_KEY"
)

for var in "${required_vars[@]}"; do
    if [[ -z "${!var}" ]]; then
        echo "❌ ERROR: Required environment variable $var is not set"
        exit 1
    fi
done

echo "✅ Production environment configured with perfect sovereignty compliance"
```

### **Kubernetes Environment**
```yaml
# k8s-sovereignty-config.yaml - Kubernetes sovereignty-compliant configuration
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-sovereignty-config
  namespace: beardog-ecosystem
data:
  # Universal adapter configuration
  UNIVERSAL_ADAPTER_ENABLED: "true"
  CAPABILITY_DISCOVERY_TIMEOUT: "10000"
  ECOSYSTEM_DISCOVERY_ENABLED: "true"
  
  # Service discovery (capability-based)
  SERVICE_MESH_ENDPOINT: "http://mesh-service.beardog-ecosystem.svc.cluster.local:8080"
  COMPUTE_SERVICE_ENDPOINT: "http://compute-service.beardog-ecosystem.svc.cluster.local:9090"
  AI_SERVICE_ENDPOINT: "http://ai-service.beardog-ecosystem.svc.cluster.local:8443"
  MONITORING_ENDPOINT: "http://monitoring-service.beardog-ecosystem.svc.cluster.local:3000"
  
  # Network configuration
  BEARDOG_API_HOST: "0.0.0.0"
  BEARDOG_API_PORT: "8080"
  BEARDOG_ADMIN_PORT: "9999"
  
  # Timeouts and retries
  INTER_PRIMAL_TIMEOUT: "15000"
  INTER_PRIMAL_RETRIES: "3"
  ECOSYSTEM_HEALTH_CHECK_INTERVAL: "30000"
```

---

## 🔍 **CONFIGURATION VALIDATION**

### **Sovereignty Compliance Checker**
```bash
#!/bin/bash
# sovereignty-config-validator.sh - Validate configuration compliance

check_hardcoded_violations() {
    echo "🔍 Checking for sovereignty violations..."
    
    # Check for hardcoded primal names
    violations=0
    
    if grep -r "songbird\|toadstool\|squirrel\|nestgate" configs/ --exclude="*.md" 2>/dev/null; then
        echo "❌ VIOLATION: Hardcoded primal names found in configuration"
        violations=$((violations + 1))
    fi
    
    # Check for hardcoded localhost (except templates)
    if grep -r "127\.0\.0\.1\|localhost" configs/ --exclude="*template*" --exclude="*.md" 2>/dev/null; then
        echo "⚠️  WARNING: Hardcoded localhost found - ensure environment variables are used"
    fi
    
    if [[ $violations -eq 0 ]]; then
        echo "✅ Configuration sovereignty compliance: PERFECT"
        return 0
    else
        echo "❌ Configuration sovereignty violations: $violations found"
        return 1
    fi
}

validate_environment() {
    echo "🌍 Validating environment configuration..."
    
    required_capabilities=(
        "SERVICE_MESH_ENDPOINT"
        "COMPUTE_SERVICE_ENDPOINT" 
        "AI_SERVICE_ENDPOINT"
        "MONITORING_ENDPOINT"
    )
    
    missing=0
    for capability in "${required_capabilities[@]}"; do
        if [[ -z "${!capability}" ]]; then
            echo "⚠️  Missing capability endpoint: $capability"
            missing=$((missing + 1))
        else
            echo "✅ Capability configured: $capability = ${!capability}"
        fi
    done
    
    if [[ $missing -eq 0 ]]; then
        echo "✅ Environment sovereignty compliance: PERFECT"
        return 0
    else
        echo "⚠️  Environment configuration: $missing capabilities need configuration"
        return 1
    fi
}

# Run validation
echo "🏆 BearDog Sovereignty Configuration Validator"
echo "============================================="

check_hardcoded_violations
config_result=$?

validate_environment  
env_result=$?

if [[ $config_result -eq 0 && $env_result -eq 0 ]]; then
    echo ""
    echo "🌟 SOVEREIGNTY COMPLIANCE: PERFECT! 👑"
    echo "✅ Zero hardcoded violations detected"
    echo "✅ All capability endpoints configured"
    echo "✅ Ready for sovereign primal deployment"
    exit 0
else
    echo ""
    echo "🚨 SOVEREIGNTY COMPLIANCE: NEEDS ATTENTION"
    echo "Please fix violations before deployment"
    exit 1
fi
```

---

## 🚀 **DEPLOYMENT CHECKLIST**

### **Pre-Deployment Sovereignty Validation**
- [ ] ✅ **Zero Hardcoded Primal Names**: No songbird/toadstool/squirrel/nestgate references
- [ ] ✅ **Environment Variables Set**: All required capability endpoints configured
- [ ] ✅ **Universal Adapter Enabled**: Capability-based discovery active
- [ ] ✅ **Dynamic Service Discovery**: Ecosystem discovery enabled
- [ ] ✅ **Graceful Fallbacks**: Timeout and retry configurations set
- [ ] ✅ **Security Compliance**: JWT secrets and encryption keys configured
- [ ] ✅ **Monitoring Integration**: Capability-based monitoring endpoint set
- [ ] ✅ **Database Sovereignty**: No hardcoded database credentials
- [ ] ✅ **Network Sovereignty**: No hardcoded IP addresses or ports
- [ ] ✅ **Vendor Independence**: No cloud provider or orchestrator hardcoding

### **Post-Deployment Verification**
- [ ] ✅ **Capability Discovery Working**: Services discovered dynamically
- [ ] ✅ **Universal Adapter Operational**: Inter-primal communication working
- [ ] ✅ **Health Checks Passing**: All discovered services responding
- [ ] ✅ **Sovereignty Score**: Achieve 1.0/1.0 perfect compliance
- [ ] ✅ **Scalability Test**: Add new primal without configuration changes
- [ ] ✅ **Failover Test**: Service discovery handles provider failures

---

## 🏆 **SOVEREIGNTY ACHIEVEMENT**

Following this guide achieves:

- **🌱 Perfect Infant Discovery**: Zero hardcoded ecosystem knowledge
- **🔍 Dynamic Configuration**: All services discovered via capabilities
- **🔌 Universal Adaptation**: Single configuration pattern for all environments
- **♾️ Infinite Scalability**: Add unlimited services without config changes
- **👑 True Sovereignty**: Each primal only knows its own requirements
- **🌍 Vendor Freedom**: Works with any provider or orchestrator

**Result: 1.0/1.0 Perfect Sovereignty Score! 🌟**

---

*This configuration guide ensures the BearDog ecosystem achieves perfect primal sovereignty where each component starts with zero hardcoded knowledge and discovers its environment dynamically.* 