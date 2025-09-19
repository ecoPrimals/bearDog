# 🐻 BearDog Production Deployment Guide

## 📋 Table of Contents
- [Prerequisites](#prerequisites)
- [Environment Setup](#environment-setup)
- [Security Configuration](#security-configuration)
- [Database Setup](#database-setup)
- [Kubernetes Deployment](#kubernetes-deployment)
- [Monitoring & Alerting](#monitoring--alerting)
- [Backup & Recovery](#backup--recovery)
- [Troubleshooting](#troubleshooting)
- [Performance Tuning](#performance-tuning)

## Prerequisites

### System Requirements
- **Kubernetes Cluster**: v1.25+ with RBAC enabled
- **Container Runtime**: Docker 20.10+ or containerd 1.6+
- **Database**: PostgreSQL 14+ (recommended) or compatible
- **Memory**: Minimum 4GB per pod, 8GB recommended
- **CPU**: Minimum 2 cores per pod, 4 cores recommended
- **Storage**: 100GB+ for data persistence

### Required Tools
```bash
# Install required CLI tools
curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl"
curl -fsSL https://get.docker.com -o get-docker.sh && sh get-docker.sh
curl https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 | bash
```

### Security Tools (Optional but Recommended)
```bash
# Container security scanning
curl -sfL https://raw.githubusercontent.com/aquasecurity/trivy/main/contrib/install.sh | sh -s -- -b /usr/local/bin
```

## Environment Setup

### 1. Environment Variables
Create a `.env` file with required configurations:

```bash
# Core Configuration
BEARDOG_ENV=production
BEARDOG_LOG_LEVEL=info
BEARDOG_CONFIG_PATH=/app/configs/production.toml

# Database Configuration
BEARDOG_DATABASE_URL=postgresql://beardog_user:secure_password@postgres:5432/beardog_prod
BEARDOG_DATABASE_POOL_SIZE=20
BEARDOG_DATABASE_TIMEOUT=30

# Security Configuration
BEARDOG_SECRET_KEY=your-256-bit-secret-key-here
BEARDOG_JWT_SECRET=your-jwt-signing-secret
BEARDOG_ENCRYPTION_KEY=your-aes-256-encryption-key

# HSM Configuration (if using hardware security modules)
BEARDOG_HSM_PROVIDER=auto
BEARDOG_HSM_SLOT_ID=0
BEARDOG_HSM_PIN=your-hsm-pin

# Monitoring Configuration
BEARDOG_METRICS_ENABLED=true
BEARDOG_METRICS_PORT=9090
BEARDOG_HEALTH_CHECK_PORT=8081

# Performance Configuration
BEARDOG_WORKER_THREADS=0  # Auto-detect
BEARDOG_MAX_CONNECTIONS=1000
BEARDOG_REQUEST_TIMEOUT=30s

# Compliance Configuration
BEARDOG_AUDIT_LOG_ENABLED=true
BEARDOG_DATA_RETENTION_DAYS=2555  # 7 years
BEARDOG_GDPR_COMPLIANCE=true
```

### 2. Generate Secure Secrets
```bash
# Generate secure random secrets
openssl rand -hex 32  # For BEARDOG_SECRET_KEY
openssl rand -base64 32  # For BEARDOG_JWT_SECRET
openssl rand -hex 32  # For BEARDOG_ENCRYPTION_KEY
```

## Security Configuration

### 1. TLS Certificates
```bash
# Generate self-signed certificates for development
openssl req -x509 -newkey rsa:4096 -keyout server.key -out server.crt -days 365 -nodes

# For production, use Let's Encrypt or your CA
certbot certonly --standalone -d your-domain.com
```

### 2. Kubernetes Secrets
```bash
# Create namespace
kubectl create namespace beardog-production

# Create TLS secret
kubectl create secret tls beardog-tls \
  --cert=server.crt \
  --key=server.key \
  -n beardog-production

# Create application secrets
kubectl create secret generic beardog-secrets \
  --from-env-file=.env \
  -n beardog-production
```

### 3. Network Policies
```yaml
# k8s/network-policies.yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: beardog-network-policy
  namespace: beardog-production
spec:
  podSelector:
    matchLabels:
      app: beardog
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          name: ingress-nginx
    ports:
    - protocol: TCP
      port: 8080
    - protocol: TCP
      port: 8443
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: database
    ports:
    - protocol: TCP
      port: 5432
```

## Database Setup

### 1. PostgreSQL Configuration
```sql
-- Create database and user
CREATE DATABASE beardog_prod;
CREATE USER beardog_user WITH PASSWORD 'secure_password';
GRANT ALL PRIVILEGES ON DATABASE beardog_prod TO beardog_user;

-- Enable required extensions
\c beardog_prod;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";
```

### 2. Database Migrations
```bash
# Run database migrations
kubectl exec -it deployment/beardog -n beardog-production -- \
  ./bin/beardog migrate --config /app/configs/production.toml
```

### 3. Database Backup Configuration
```bash
# Set up automated backups
cat > backup-cronjob.yaml << EOF
apiVersion: batch/v1
kind: CronJob
metadata:
  name: beardog-db-backup
  namespace: beardog-production
spec:
  schedule: "0 2 * * *"  # Daily at 2 AM
  jobTemplate:
    spec:
      template:
        spec:
          containers:
          - name: postgres-backup
            image: postgres:14
            env:
            - name: PGPASSWORD
              valueFrom:
                secretKeyRef:
                  name: beardog-secrets
                  key: BEARDOG_DATABASE_PASSWORD
            command:
            - /bin/bash
            - -c
            - |
              pg_dump -h postgres -U beardog_user beardog_prod | \
              gzip > /backup/beardog-backup-$(date +%Y%m%d-%H%M%S).sql.gz
            volumeMounts:
            - name: backup-storage
              mountPath: /backup
          volumes:
          - name: backup-storage
            persistentVolumeClaim:
              claimName: backup-pvc
          restartPolicy: OnFailure
EOF
```

## Kubernetes Deployment

### 1. Deploy BearDog Application
```bash
# Apply all Kubernetes manifests
kubectl apply -f k8s/beardog-production.yaml -n beardog-production

# Wait for deployment to be ready
kubectl rollout status deployment/beardog -n beardog-production --timeout=300s
```

### 2. Production Deployment Manifest
```yaml
# k8s/beardog-production.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
  namespace: beardog-production
  labels:
    app: beardog
    version: v3.0.0
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxSurge: 1
      maxUnavailable: 0
  selector:
    matchLabels:
      app: beardog
  template:
    metadata:
      labels:
        app: beardog
        version: v3.0.0
    spec:
      serviceAccountName: beardog-service-account
      securityContext:
        runAsNonRoot: true
        runAsUser: 1000
        fsGroup: 1000
      containers:
      - name: beardog
        image: beardog:latest
        imagePullPolicy: Always
        ports:
        - containerPort: 8080
          name: http
        - containerPort: 8443
          name: https
        - containerPort: 9090
          name: metrics
        env:
        - name: RUST_LOG
          value: "info"
        - name: BEARDOG_CONFIG_PATH
          value: "/app/configs/production.toml"
        envFrom:
        - secretRef:
            name: beardog-secrets
        resources:
          requests:
            memory: "2Gi"
            cpu: "1000m"
          limits:
            memory: "4Gi"
            cpu: "2000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
          timeoutSeconds: 5
          failureThreshold: 3
        readinessProbe:
          httpGet:
            path: /ready
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
        volumeMounts:
        - name: config-volume
          mountPath: /app/configs
        - name: tls-certs
          mountPath: /app/certs
        - name: data-volume
          mountPath: /app/data
      volumes:
      - name: config-volume
        configMap:
          name: beardog-config
      - name: tls-certs
        secret:
          secretName: beardog-tls
      - name: data-volume
        persistentVolumeClaim:
          claimName: beardog-data-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: beardog-service
  namespace: beardog-production
spec:
  selector:
    app: beardog
  ports:
  - name: http
    port: 80
    targetPort: 8080
  - name: https
    port: 443
    targetPort: 8443
  - name: metrics
    port: 9090
    targetPort: 9090
  type: LoadBalancer
```

### 3. Horizontal Pod Autoscaling
```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: beardog-hpa
  namespace: beardog-production
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: beardog
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
  - type: Resource
    resource:
      name: memory
      target:
        type: Utilization
        averageUtilization: 80
```

## Monitoring & Alerting

### 1. Prometheus Configuration
```yaml
# monitoring/prometheus-config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: prometheus-config
  namespace: monitoring
data:
  prometheus.yml: |
    global:
      scrape_interval: 15s
      evaluation_interval: 15s
    
    rule_files:
      - "beardog_alerts.yml"
    
    scrape_configs:
    - job_name: 'beardog'
      static_configs:
      - targets: ['beardog-service.beardog-production:9090']
      scrape_interval: 5s
      metrics_path: /metrics
      
    - job_name: 'beardog-health'
      static_configs:
      - targets: ['beardog-service.beardog-production:8080']
      scrape_interval: 10s
      metrics_path: /health/metrics
```

### 2. Grafana Dashboard
```json
{
  "dashboard": {
    "title": "BearDog Production Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(beardog_http_requests_total[5m])",
            "legendFormat": "{{method}} {{status}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(beardog_http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      },
      {
        "title": "Memory Usage",
        "type": "graph",
        "targets": [
          {
            "expr": "beardog_memory_usage_bytes / 1024 / 1024",
            "legendFormat": "Memory MB"
          }
        ]
      }
    ]
  }
}
```

### 3. Alert Rules
```yaml
# monitoring/beardog-alerts.yml
groups:
- name: beardog.rules
  rules:
  - alert: BearDogHighErrorRate
    expr: rate(beardog_http_requests_total{status=~"5.."}[5m]) > 0.1
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: "High error rate detected"
      description: "Error rate is {{ $value }} errors per second"
      
  - alert: BearDogHighMemoryUsage
    expr: beardog_memory_usage_bytes / beardog_memory_limit_bytes > 0.9
    for: 2m
    labels:
      severity: warning
    annotations:
      summary: "High memory usage"
      description: "Memory usage is {{ $value | humanizePercentage }}"
      
  - alert: BearDogDatabaseConnectionFailure
    expr: beardog_database_connections_failed_total > 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "Database connection failure"
      description: "Failed to connect to database"
```

## Performance Tuning

### 1. JVM/Runtime Tuning (if applicable)
```bash
# Rust-specific optimizations are built into the binary
# Focus on system-level optimizations

# Kernel parameters for high-performance networking
echo 'net.core.rmem_max = 16777216' >> /etc/sysctl.conf
echo 'net.core.wmem_max = 16777216' >> /etc/sysctl.conf
echo 'net.ipv4.tcp_rmem = 4096 87380 16777216' >> /etc/sysctl.conf
echo 'net.ipv4.tcp_wmem = 4096 65536 16777216' >> /etc/sysctl.conf
sysctl -p
```

### 2. Database Performance Tuning
```sql
-- PostgreSQL performance tuning
ALTER SYSTEM SET shared_buffers = '2GB';
ALTER SYSTEM SET effective_cache_size = '6GB';
ALTER SYSTEM SET maintenance_work_mem = '512MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;
SELECT pg_reload_conf();
```

### 3. Load Testing
```bash
# Install k6 for load testing
curl https://github.com/grafana/k6/releases/download/v0.45.0/k6-v0.45.0-linux-amd64.tar.gz -L | tar xvz --strip-components 1

# Run load test
k6 run - <<EOF
import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  stages: [
    { duration: '2m', target: 100 },
    { duration: '5m', target: 100 },
    { duration: '2m', target: 200 },
    { duration: '5m', target: 200 },
    { duration: '2m', target: 0 },
  ],
};

export default function () {
  let response = http.get('http://beardog-service.beardog-production/health');
  check(response, { 'status was 200': (r) => r.status == 200 });
  sleep(1);
}
EOF
```

## Troubleshooting

### Common Issues

#### 1. Pod Startup Failures
```bash
# Check pod logs
kubectl logs -f deployment/beardog -n beardog-production

# Check pod events
kubectl describe pod <pod-name> -n beardog-production

# Check resource constraints
kubectl top pods -n beardog-production
```

#### 2. Database Connection Issues
```bash
# Test database connectivity
kubectl exec -it deployment/beardog -n beardog-production -- \
  psql $BEARDOG_DATABASE_URL -c "SELECT 1;"

# Check database logs
kubectl logs -f deployment/postgres -n database
```

#### 3. Performance Issues
```bash
# Check CPU and memory usage
kubectl top pods -n beardog-production

# Check application metrics
curl http://beardog-service.beardog-production:9090/metrics

# Check database performance
kubectl exec -it deployment/postgres -n database -- \
  psql -d beardog_prod -c "SELECT * FROM pg_stat_activity;"
```

### Emergency Procedures

#### 1. Emergency Rollback
```bash
# Rollback to previous version
kubectl rollout undo deployment/beardog -n beardog-production

# Check rollout status
kubectl rollout status deployment/beardog -n beardog-production
```

#### 2. Scale Down for Maintenance
```bash
# Scale to 0 replicas
kubectl scale deployment beardog --replicas=0 -n beardog-production

# Scale back up
kubectl scale deployment beardog --replicas=3 -n beardog-production
```

#### 3. Emergency Database Restore
```bash
# Restore from backup
kubectl exec -it deployment/postgres -n database -- \
  psql -d beardog_prod < /backup/beardog-backup-latest.sql
```

## Security Checklist

- [ ] TLS certificates configured and valid
- [ ] All secrets stored in Kubernetes secrets (not ConfigMaps)
- [ ] Network policies implemented and tested
- [ ] RBAC configured with least privilege
- [ ] Container images scanned for vulnerabilities
- [ ] Audit logging enabled and configured
- [ ] Database encryption at rest enabled
- [ ] Regular security updates scheduled
- [ ] Backup encryption configured
- [ ] Monitoring and alerting for security events

## Maintenance Schedule

### Daily
- [ ] Check application health and metrics
- [ ] Review error logs and alerts
- [ ] Verify backup completion

### Weekly
- [ ] Review security alerts and patches
- [ ] Analyze performance metrics and trends
- [ ] Test disaster recovery procedures

### Monthly
- [ ] Update dependencies and security patches
- [ ] Review and rotate secrets/certificates
- [ ] Conduct load testing
- [ ] Review and update documentation

---

## Support and Escalation

For production issues:
1. **P0 (Critical)**: Immediate response required
2. **P1 (High)**: Response within 4 hours
3. **P2 (Medium)**: Response within 24 hours
4. **P3 (Low)**: Response within 72 hours

**Emergency Contact**: [Your escalation procedures here]

---

*This guide should be kept up-to-date with any changes to the production environment.* 