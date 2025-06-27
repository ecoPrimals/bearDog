# Songbird-BearDog Integration Deployment Configuration

**Version:** 1.0  
**Date:** December 2024  
**Target:** BearDog Security Manager Deployment  
**Status:** Production Deployment Configuration Guide

## 🚀 Deployment Overview

This document provides comprehensive deployment configuration for integrating BearDog Security Manager with Songbird Orchestrator in production environments.

## 🔧 Environment Configuration

### Environment Variables

**Required Environment Variables:**
```bash
# BearDog Connection
BEARDOG_ENDPOINT=https://beardog.security.internal
BEARDOG_API_KEY=your-beardog-api-key-here
BEARDOG_TIMEOUT_SECONDS=30
BEARDOG_MAX_RETRIES=3

# Security Configuration
SONGBIRD_ENABLE_SECURITY=true
SONGBIRD_SECURITY_PROVIDER=beardog
SONGBIRD_ENABLE_AUDIT_LOGGING=true
SONGBIRD_AUDIT_ENDPOINT=https://beardog.security.internal/api/v1/audit

# Authentication Configuration
SONGBIRD_ENABLE_AUTHENTICATION=true
SONGBIRD_AUTH_PROVIDER=beardog
SONGBIRD_JWT_SECRET=your-jwt-secret-here
SONGBIRD_JWT_EXPIRATION_HOURS=24

# Network Security
SONGBIRD_BIND_ADDRESS=0.0.0.0
SONGBIRD_PORT=8080
SONGBIRD_ENABLE_TLS=true
SONGBIRD_TLS_CERT_PATH=/etc/ssl/certs/songbird.crt
SONGBIRD_TLS_KEY_PATH=/etc/ssl/private/songbird.key

# Hook Configuration
BEARDOG_HOOK_ENABLED=true
BEARDOG_THREAT_CONFIDENCE_THRESHOLD=0.7
BEARDOG_FAIL_CLOSED_ON_ASSESSMENT_ERROR=false
BEARDOG_ENABLE_REAL_TIME_MONITORING=true
BEARDOG_ENABLE_INCIDENT_RESPONSE=true

# Performance Tuning
BEARDOG_AUDIT_BATCH_SIZE=100
BEARDOG_AUDIT_FLUSH_INTERVAL_SECONDS=60
BEARDOG_CACHE_SIZE=1000
BEARDOG_CACHE_TTL_SECONDS=3600
```

**Optional Environment Variables:**
```bash
# Advanced Security
SONGBIRD_ENABLE_OAUTH=true
SONGBIRD_OAUTH_PROVIDER=beardog
SONGBIRD_ENABLE_MFA=false

# Monitoring
SONGBIRD_ENABLE_METRICS=true
SONGBIRD_METRICS_PORT=9090
SONGBIRD_ENABLE_TRACING=true
SONGBIRD_TRACING_ENDPOINT=http://jaeger:14268

# Development/Testing
SONGBIRD_LOG_LEVEL=info
SONGBIRD_ENABLE_DEBUG_HOOKS=false
BEARDOG_MOCK_MODE=false
```

### Configuration Files

**Songbird Configuration (`songbird.toml`):**
```toml
[orchestrator]
bind_address = "0.0.0.0"
port = 8080
enable_tls = true
tls_cert_path = "/etc/ssl/certs/songbird.crt"
tls_key_path = "/etc/ssl/private/songbird.key"

[security]
provider = "beardog"
enable_authentication = true
enable_authorization = true
enable_audit_logging = true
jwt_expiration_hours = 24

[beardog]
endpoint = "https://beardog.security.internal"
api_key_env = "BEARDOG_API_KEY"
timeout_seconds = 30
max_retries = 3

[beardog.audit]
endpoint = "/api/v1/audit"
batch_size = 100
flush_interval_seconds = 60
enable_encryption = true

[beardog.hooks]
enabled = true
threat_confidence_threshold = 0.7
fail_closed_on_assessment_error = false
enable_real_time_monitoring = true
enable_incident_response = true

[beardog.cache]
size = 1000
ttl_seconds = 3600

[logging]
level = "info"
format = "json"
destination = "file"
path = "/var/log/songbird/songbird.log"

[metrics]
enabled = true
port = 9090
path = "/metrics"
```

**BearDog Integration Configuration (`beardog-integration.toml`):**
```toml
[connection]
endpoint = "https://beardog.security.internal"
api_version = "v1"
timeout_seconds = 30
max_retries = 3
retry_delay_ms = 1000
backoff_multiplier = 2.0

[authentication]
method = "api_key"
api_key_header = "Authorization"
api_key_prefix = "Bearer"

[security_provider]
enable_authorization_cache = true
cache_size = 1000
cache_ttl_seconds = 3600
fail_open_on_timeout = false

[audit_logging]
endpoint = "/api/v1/audit"
batch_size = 100
flush_interval_seconds = 60
max_queue_size = 10000
enable_compression = true
enable_encryption = true

[monitoring_hook]
enabled = true
priority = 100
event_filter = [
    "RequestReceived",
    "ServiceRegistering", 
    "ErrorOccurred",
    "ConfigurationChanged",
    "HealthCheckCompleted",
    "ServiceDiscovered"
]

[threat_detection]
confidence_threshold = 0.7
severity_threshold = "medium"
enable_ml_analysis = true
enable_pattern_matching = true

[incident_response]
enabled = true
auto_block_critical_threats = true
auto_block_high_threats = false
notification_webhook = "https://beardog.security.internal/api/v1/incidents"

[compliance]
enable_service_assessment = true
fail_closed_on_non_compliance = false
assessment_cache_ttl_seconds = 1800
```

## 🐳 Docker Deployment

### Dockerfile

```dockerfile
# Songbird with BearDog Integration
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build with BearDog integration features
RUN cargo build --release --features beardog-integration

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false songbird

# Copy binary and configuration
COPY --from=builder /app/target/release/songbird /usr/local/bin/
COPY --from=builder /app/config/ /etc/songbird/

# Create directories
RUN mkdir -p /var/log/songbird /var/lib/songbird && \
    chown -R songbird:songbird /var/log/songbird /var/lib/songbird

USER songbird

EXPOSE 8080 9090

CMD ["songbird", "--config", "/etc/songbird/songbird.toml"]
```

### Docker Compose

```yaml
version: '3.8'
services:
  songbird:
    build: .
    ports:
      - "8080:8080"
      - "9090:9090"
    environment:
      - BEARDOG_ENDPOINT=https://beardog.security.internal
      - BEARDOG_API_KEY=${BEARDOG_API_KEY}
      - SONGBIRD_ENABLE_SECURITY=true
      - SONGBIRD_SECURITY_PROVIDER=beardog
      - SONGBIRD_ENABLE_TLS=true
    volumes:
      - ./config:/etc/songbird:ro
      - ./ssl:/etc/ssl:ro
      - songbird-logs:/var/log/songbird
      - songbird-data:/var/lib/songbird
    depends_on:
      - beardog-proxy
    networks:
      - songbird-network
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

  beardog-proxy:
    image: nginx:alpine
    ports:
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/ssl:ro
    networks:
      - songbird-network
    restart: unless-stopped

volumes:
  songbird-logs:
  songbird-data:

networks:
  songbird-network:
    driver: bridge
```

## ☸️ Kubernetes Deployment

### Namespace and Service Account

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: songbird-orchestrator
  labels:
    security.policy: strict
---
apiVersion: v1
kind: ServiceAccount
metadata:
  name: songbird
  namespace: songbird-orchestrator
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRole
metadata:
  name: songbird-operator
rules:
- apiGroups: [""]
  resources: ["services", "endpoints", "pods"]
  verbs: ["get", "list", "watch"]
- apiGroups: ["apps"]
  resources: ["deployments", "replicasets"]
  verbs: ["get", "list", "watch"]
---
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: songbird-operator
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: songbird-operator
subjects:
- kind: ServiceAccount
  name: songbird
  namespace: songbird-orchestrator
```

### ConfigMap and Secrets

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: songbird-config
  namespace: songbird-orchestrator
data:
  songbird.toml: |
    [orchestrator]
    bind_address = "0.0.0.0"
    port = 8080
    enable_tls = true
    
    [security]
    provider = "beardog"
    enable_authentication = true
    enable_authorization = true
    enable_audit_logging = true
    
    [beardog]
    endpoint = "https://beardog.security.internal"
    timeout_seconds = 30
    max_retries = 3
    
    [beardog.hooks]
    enabled = true
    threat_confidence_threshold = 0.7
    enable_real_time_monitoring = true
    enable_incident_response = true

---
apiVersion: v1
kind: Secret
metadata:
  name: songbird-secrets
  namespace: songbird-orchestrator
type: Opaque
data:
  beardog-api-key: <base64-encoded-api-key>
  jwt-secret: <base64-encoded-jwt-secret>
  tls.crt: <base64-encoded-certificate>
  tls.key: <base64-encoded-private-key>
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: songbird-orchestrator
  namespace: songbird-orchestrator
  labels:
    app: songbird-orchestrator
    security.policy: beardog-managed
spec:
  replicas: 3
  selector:
    matchLabels:
      app: songbird-orchestrator
  template:
    metadata:
      labels:
        app: songbird-orchestrator
        security.policy: beardog-managed
    spec:
      serviceAccountName: songbird
      securityContext:
        runAsNonRoot: true
        runAsUser: 65534
        fsGroup: 65534
      containers:
      - name: songbird
        image: songbird-orchestrator:latest
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 9090
          name: metrics
        env:
        - name: BEARDOG_ENDPOINT
          value: "https://beardog.security.internal"
        - name: BEARDOG_API_KEY
          valueFrom:
            secretKeyRef:
              name: songbird-secrets
              key: beardog-api-key
        - name: SONGBIRD_JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: songbird-secrets
              key: jwt-secret
        - name: SONGBIRD_ENABLE_SECURITY
          value: "true"
        - name: SONGBIRD_SECURITY_PROVIDER
          value: "beardog"
        volumeMounts:
        - name: config
          mountPath: /etc/songbird
        - name: tls
          mountPath: /etc/ssl
        - name: logs
          mountPath: /var/log/songbird
        resources:
          requests:
            memory: "64Mi"
            cpu: "100m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: songbird-config
      - name: tls
        secret:
          secretName: songbird-secrets
          items:
          - key: tls.crt
            path: certs/songbird.crt
          - key: tls.key
            path: private/songbird.key
      - name: logs
        emptyDir: {}
```

### Service and Ingress

```yaml
apiVersion: v1
kind: Service
metadata:
  name: songbird-orchestrator
  namespace: songbird-orchestrator
  labels:
    app: songbird-orchestrator
spec:
  selector:
    app: songbird-orchestrator
  ports:
  - port: 8080
    targetPort: 8080
    name: http
  - port: 9090
    targetPort: 9090
    name: metrics
  type: ClusterIP
---
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: songbird-orchestrator
  namespace: songbird-orchestrator
  annotations:
    kubernetes.io/ingress.class: nginx
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/backend-protocol: "HTTP"
    cert-manager.io/cluster-issuer: letsencrypt-prod
spec:
  tls:
  - hosts:
    - songbird.example.com
    secretName: songbird-tls
  rules:
  - host: songbird.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: songbird-orchestrator
            port:
              number: 8080
```

## 🔍 Monitoring and Observability

### Prometheus Configuration

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
  namespace: songbird-orchestrator
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
      evaluation_interval: 15s
    
    scrape_configs:
    - job_name: 'songbird-orchestrator'
      static_configs:
      - targets: ['songbird-orchestrator:9090']
      metrics_path: /metrics
      scrape_interval: 10s
      
    - job_name: 'beardog-integration'
      static_configs:
      - targets: ['songbird-orchestrator:9090']
      metrics_path: /beardog/metrics
      scrape_interval: 30s
    
    rule_files:
    - "beardog_alerts.yml"
    
    alerting:
      alertmanagers:
      - static_configs:
        - targets:
          - alertmanager:9093
```

### Alert Rules

```yaml
groups:
- name: beardog_integration
  rules:
  - alert: BearDogConnectionDown
    expr: beardog_client_connection_status == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "BearDog connection is down"
      description: "Songbird cannot connect to BearDog security manager"
  
  - alert: HighThreatDetectionRate
    expr: rate(beardog_threats_detected_total[5m]) > 0.1
    for: 2m
    labels:
      severity: warning
    annotations:
      summary: "High threat detection rate"
      description: "BearDog is detecting threats at {{ $value }} per second"
  
  - alert: SecurityHookFailures
    expr: rate(beardog_hook_failures_total[5m]) > 0.01
    for: 1m
    labels:
      severity: warning
    annotations:
      summary: "BearDog security hook failures"
      description: "Security hooks are failing at {{ $value }} per second"
  
  - alert: AuthorizationLatencyHigh
    expr: histogram_quantile(0.95, beardog_authorization_duration_seconds) > 0.1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High authorization latency"
      description: "95th percentile authorization latency is {{ $value }} seconds"
```

## 🔒 Security Hardening

### Network Security

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: songbird-network-policy
  namespace: songbird-orchestrator
spec:
  podSelector:
    matchLabels:
      app: songbird-orchestrator
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: nginx-ingress
    ports:
    - protocol: TCP
      port: 8080
  - from:
    - namespaceSelector:
        matchLabels:
          name: monitoring
    ports:
    - protocol: TCP
      port: 9090
  egress:
  - to:
    - namespaceSelector: {}
    ports:
    - protocol: TCP
      port: 53
    - protocol: UDP
      port: 53
  - to: []
    ports:
    - protocol: TCP
      port: 443
```

### Pod Security Policy

```yaml
apiVersion: policy/v1beta1
kind: PodSecurityPolicy
metadata:
  name: songbird-psp
spec:
  privileged: false
  allowPrivilegeEscalation: false
  requiredDropCapabilities:
    - ALL
  volumes:
    - 'configMap'
    - 'emptyDir'
    - 'projected'
    - 'secret'
    - 'downwardAPI'
    - 'persistentVolumeClaim'
  runAsUser:
    rule: 'MustRunAsNonRoot'
  seLinux:
    rule: 'RunAsAny'
  supplementalGroups:
    rule: 'MustRunAs'
    ranges:
      - min: 1
        max: 65535
  fsGroup:
    rule: 'MustRunAs'
    ranges:
      - min: 1
        max: 65535
  readOnlyRootFilesystem: false
```

## 📋 Health Checks and Readiness

### Health Check Endpoints

**Songbird Health Checks:**
- `GET /health` - Overall system health
- `GET /health/beardog` - BearDog integration health
- `GET /ready` - Readiness probe
- `GET /metrics` - Prometheus metrics

**BearDog Integration Health:**
```rust
// Health check implementation
pub async fn beardog_health_check() -> Result<HealthStatus> {
    let client = get_beardog_client();
    
    // Test connection
    let connection_ok = client.health_check().await?;
    
    // Check authentication
    let auth_ok = client.test_authentication().await?;
    
    // Check recent response times
    let latency_ok = client.check_response_times().await?;
    
    Ok(HealthStatus {
        healthy: connection_ok && auth_ok && latency_ok,
        details: json!({
            "connection": connection_ok,
            "authentication": auth_ok,
            "latency": latency_ok,
            "last_check": Utc::now()
        })
    })
}
```

## 🚀 Production Deployment Checklist

### Pre-Deployment
- [ ] Environment variables configured
- [ ] TLS certificates generated and installed
- [ ] BearDog API key configured
- [ ] Network policies configured
- [ ] Resource limits set appropriately
- [ ] Health checks configured
- [ ] Monitoring and alerting set up

### Deployment
- [ ] Deploy in staging environment first
- [ ] Validate BearDog connectivity
- [ ] Test authentication and authorization
- [ ] Verify audit logging is working
- [ ] Test security hook functionality
- [ ] Performance testing completed
- [ ] Security scanning completed

### Post-Deployment
- [ ] Monitor application logs
- [ ] Verify metrics collection
- [ ] Test incident response workflows
- [ ] Validate compliance checks
- [ ] Monitor performance metrics
- [ ] Review security alerts

### Rollback Plan
- [ ] Database backup available
- [ ] Configuration backup available
- [ ] Previous version container images available
- [ ] Rollback procedure documented
- [ ] Rollback testing completed

## 📞 Production Support

### Log Locations
- Application logs: `/var/log/songbird/songbird.log`
- Security logs: `/var/log/songbird/security.log`
- Audit logs: `/var/log/songbird/audit.log`
- BearDog integration logs: `/var/log/songbird/beardog.log`

### Key Metrics to Monitor
- `beardog_client_connection_status` - Connection health
- `beardog_authorization_requests_total` - Authorization request rate
- `beardog_authorization_duration_seconds` - Authorization latency
- `beardog_threats_detected_total` - Threat detection rate
- `beardog_incidents_created_total` - Security incident rate
- `beardog_hook_executions_total` - Hook execution rate
- `beardog_hook_failures_total` - Hook failure rate

### Troubleshooting Commands
```bash
# Check BearDog connectivity
curl -H "Authorization: Bearer $BEARDOG_API_KEY" https://beardog.security.internal/api/v1/health

# View security logs
kubectl logs -n songbird-orchestrator deployment/songbird-orchestrator | grep -i security

# Check hook execution
kubectl exec -n songbird-orchestrator deployment/songbird-orchestrator -- songbird hooks list

# Validate configuration
kubectl exec -n songbird-orchestrator deployment/songbird-orchestrator -- songbird config validate
``` 