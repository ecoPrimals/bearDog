# BearDog Production Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying BearDog in production environments, including Docker, Kubernetes, bare metal, and cloud deployments.

## Prerequisites

### System Requirements

**Minimum Requirements**:
- CPU: 4 cores (x86_64 or ARM64)
- RAM: 8 GB
- Storage: 100 GB SSD
- Network: 1 Gbps

**Recommended for Production**:
- CPU: 8+ cores (x86_64 recommended)
- RAM: 32 GB
- Storage: 500 GB NVMe SSD
- Network: 10 Gbps
- Load balancer with SSL termination

### Dependencies

- **Database**: PostgreSQL 13+ (recommended) or SQLite for development
- **Redis**: 6.0+ for caching and session storage
- **Prometheus**: For metrics collection
- **Grafana**: For monitoring dashboards

## Docker Deployment

### Single Container

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/beardog /usr/local/bin/beardog
COPY --from=builder /app/target/release/beardog-cli /usr/local/bin/beardog-cli

EXPOSE 8080 9091 9092
USER 1000:1000

CMD ["beardog", "--config", "/etc/beardog/config.toml"]
```

### Docker Compose (Recommended)

```yaml
version: '3.8'

services:
  beardog:
    build: .
    restart: unless-stopped
    ports:
      - "8080:8080"   # API
      - "9091:9091"   # Metrics
      - "9092:9092"   # Admin
    environment:
      - BEARDOG_DATABASE_URL=postgresql://beardog:${DB_PASSWORD}@postgres:5432/beardog
      - BEARDOG_REDIS_URL=redis://redis:6379
      - BEARDOG_LOG_LEVEL=info
      - BEARDOG_ENABLE_TLS=false  # Use load balancer for TLS
      - PROMETHEUS_ENDPOINT=http://prometheus:9090
      - GRAFANA_ENDPOINT=http://grafana:3000
    volumes:
      - ./config:/etc/beardog
      - ./data:/var/lib/beardog
      - ./logs:/var/log/beardog
    depends_on:
      - postgres
      - redis
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '1.0'
        reservations:
          memory: 1G
          cpus: '0.5'

  postgres:
    image: postgres:15
    restart: unless-stopped
    environment:
      - POSTGRES_DB=beardog
      - POSTGRES_USER=beardog
      - POSTGRES_PASSWORD=${DB_PASSWORD}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./init-db.sql:/docker-entrypoint-initdb.d/init-db.sql
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U beardog"]
      interval: 5s
      timeout: 5s
      retries: 5

  redis:
    image: redis:7-alpine
    restart: unless-stopped
    command: redis-server --appendonly yes
    volumes:
      - redis_data:/data
    ports:
      - "6379:6379"
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 5

  prometheus:
    image: prom/prometheus:latest
    restart: unless-stopped
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--web.enable-lifecycle'

  grafana:
    image: grafana/grafana:latest
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana_data:/var/lib/grafana
      - ./grafana-dashboards:/var/lib/grafana/dashboards

  nginx:
    image: nginx:alpine
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - beardog

volumes:
  postgres_data:
  redis_data:
  prometheus_data:
  grafana_data:

networks:
  default:
    name: beardog-network
```

### Environment Variables

Create a `.env` file:

```bash
# Database
DB_PASSWORD=your_secure_database_password

# Grafana
GRAFANA_PASSWORD=your_secure_grafana_password

# BearDog Configuration
BEARDOG_JWT_SECRET=your_64_character_jwt_secret_here_make_it_really_long
BEARDOG_ENCRYPTION_KEY=your_32_character_encryption_key
BEARDOG_API_KEY=your_api_key_for_external_access

# External Services
SONGBIRD_ENDPOINT=https://songbird.your-domain.com:8443
NESTGATE_ENDPOINT=https://nestgate.your-domain.com:8443
```

## Kubernetes Deployment

### Namespace

```yaml
apiVersion: v1
kind: Namespace
metadata:
  name: beardog
  labels:
    name: beardog
```

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-config
  namespace: beardog
data:
  config.toml: |
    [network]
    bind_host = "0.0.0.0"
    api_port = 8080
    https_port = 8443
    enable_tls = false  # TLS terminated by ingress
    
    [database]
    max_connections = 100
    connection_timeout = 30
    
    [security]
    force_https = true
    cors_origins = ["https://app.beardog.com", "https://admin.beardog.com"]
    
    [monitoring]
    log_level = "info"
    enable_metrics = true
    metrics_interval = 30
    
    [genetics]
    enable_genetics = true
    mutation_rate = 0.1
    fitness_threshold = 0.7
```

### Secret

```yaml
apiVersion: v1
kind: Secret
metadata:
  name: beardog-secrets
  namespace: beardog
type: Opaque
data:
  database-url: cG9zdGdyZXNxbDovL2JlYXJkb2c6cGFzc3dvcmRAcG9zdGdyZXM6NTQzMi9iZWFyZG9n  # base64 encoded
  jwt-secret: eW91cl82NF9jaGFyYWN0ZXJfand0X3NlY3JldA==  # base64 encoded
  encryption-key: eW91cl8zMl9jaGFyYWN0ZXJfZW5jcnlwdGlvbl9rZXk=  # base64 encoded
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
  namespace: beardog
  labels:
    app: beardog
spec:
  replicas: 3
  strategy:
    type: RollingUpdate
    rollingUpdate:
      maxUnavailable: 1
      maxSurge: 1
  selector:
    matchLabels:
      app: beardog
  template:
    metadata:
      labels:
        app: beardog
      annotations:
        prometheus.io/scrape: "true"
        prometheus.io/port: "9091"
    spec:
      serviceAccountName: beardog
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
          name: api
        - containerPort: 9091
          name: metrics
        - containerPort: 9092
          name: admin
        env:
        - name: BEARDOG_CONFIG_FILE
          value: "/etc/beardog/config.toml"
        - name: BEARDOG_DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: beardog-secrets
              key: database-url
        - name: BEARDOG_JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: beardog-secrets
              key: jwt-secret
        - name: BEARDOG_ENCRYPTION_KEY
          valueFrom:
            secretKeyRef:
              name: beardog-secrets
              key: encryption-key
        - name: PROMETHEUS_ENDPOINT
          value: "http://prometheus.monitoring.svc.cluster.local:9090"
        - name: GRAFANA_ENDPOINT
          value: "http://grafana.monitoring.svc.cluster.local:3000"
        volumeMounts:
        - name: config
          mountPath: /etc/beardog
        - name: data
          mountPath: /var/lib/beardog
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
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
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
          timeoutSeconds: 3
          failureThreshold: 3
      volumes:
      - name: config
        configMap:
          name: beardog-config
      - name: data
        persistentVolumeClaim:
          claimName: beardog-data
      affinity:
        podAntiAffinity:
          preferredDuringSchedulingIgnoredDuringExecution:
          - weight: 100
            podAffinityTerm:
              labelSelector:
                matchExpressions:
                - key: app
                  operator: In
                  values:
                  - beardog
              topologyKey: kubernetes.io/hostname
```

### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: beardog
  namespace: beardog
  labels:
    app: beardog
spec:
  selector:
    app: beardog
  ports:
  - name: api
    port: 8080
    targetPort: 8080
  - name: metrics
    port: 9091
    targetPort: 9091
  - name: admin
    port: 9092
    targetPort: 9092
  type: ClusterIP
```

### Ingress

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: beardog
  namespace: beardog
  annotations:
    kubernetes.io/ingress.class: "nginx"
    cert-manager.io/cluster-issuer: "letsencrypt-prod"
    nginx.ingress.kubernetes.io/ssl-redirect: "true"
    nginx.ingress.kubernetes.io/force-ssl-redirect: "true"
    nginx.ingress.kubernetes.io/rate-limit: "100"
spec:
  tls:
  - hosts:
    - api.beardog.com
    - admin.beardog.com
    secretName: beardog-tls
  rules:
  - host: api.beardog.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: beardog
            port:
              number: 8080
  - host: admin.beardog.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: beardog
            port:
              number: 9092
```

### HorizontalPodAutoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: beardog
  namespace: beardog
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

## Cloud Deployments

### AWS ECS

```yaml
version: '3.8'
services:
  beardog:
    image: your-account.dkr.ecr.region.amazonaws.com/beardog:latest
    logging:
      driver: awslogs
      options:
        awslogs-group: /ecs/beardog
        awslogs-region: us-west-2
        awslogs-stream-prefix: ecs
    environment:
      - BEARDOG_DATABASE_URL=${DATABASE_URL}
      - BEARDOG_REDIS_URL=${REDIS_URL}
    deploy:
      resources:
        limits:
          memory: 2G
        reservations:
          memory: 1G
```

### AWS Application Load Balancer

```yaml
# ALB Target Group Health Check
HealthCheckPath: /health
HealthCheckIntervalSeconds: 30
HealthCheckTimeoutSeconds: 5
HealthyThresholdCount: 2
UnhealthyThresholdCount: 3
```

### Google Cloud Run

```yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: beardog
  annotations:
    run.googleapis.com/ingress: all
    run.googleapis.com/execution-environment: gen2
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/minScale: "3"
        autoscaling.knative.dev/maxScale: "100"
        run.googleapis.com/cpu-throttling: "false"
    spec:
      containerConcurrency: 100
      timeoutSeconds: 300
      containers:
      - image: gcr.io/your-project/beardog:latest
        ports:
        - containerPort: 8080
        resources:
          limits:
            cpu: "2"
            memory: "2Gi"
        env:
        - name: BEARDOG_DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: beardog-secrets
              key: database-url
```

## Monitoring Setup

### Prometheus Configuration

```yaml
# prometheus.yml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "beardog-rules.yml"

scrape_configs:
  - job_name: 'beardog'
    static_configs:
      - targets: ['beardog:9091']
    metrics_path: /metrics
    scrape_interval: 15s

  - job_name: 'postgres'
    static_configs:
      - targets: ['postgres-exporter:9187']

alerting:
  alertmanagers:
    - static_configs:
        - targets:
          - alertmanager:9093
```

### Alerting Rules

```yaml
# beardog-rules.yml
groups:
- name: beardog
  rules:
  - alert: BearDogDown
    expr: up{job="beardog"} == 0
    for: 1m
    labels:
      severity: critical
    annotations:
      summary: "BearDog instance is down"
      description: "BearDog instance {{ $labels.instance }} has been down for more than 1 minute."

  - alert: HighResponseTime
    expr: beardog_http_request_duration_seconds{quantile="0.95"} > 1
    for: 5m
    labels:
      severity: warning
    annotations:
      summary: "High response time"
      description: "95th percentile response time is {{ $value }} seconds"

  - alert: HighErrorRate
    expr: rate(beardog_http_requests_total{status=~"5.."}[5m]) > 0.01
    for: 2m
    labels:
      severity: critical
    annotations:
      summary: "High error rate"
      description: "Error rate is {{ $value }} errors per second"
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "BearDog Monitoring",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(beardog_http_requests_total[5m])",
            "legendFormat": "{{ method }} {{ endpoint }}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "beardog_http_request_duration_seconds",
            "legendFormat": "{{ quantile }}"
          }
        ]
      },
      {
        "title": "Genetic Spawns",
        "type": "stat",
        "targets": [
          {
            "expr": "beardog_genetic_spawns_total",
            "legendFormat": "Total Spawns"
          }
        ]
      }
    ]
  }
}
```

## Security Hardening

### SSL/TLS Configuration

```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name api.beardog.com;

    ssl_certificate /etc/ssl/certs/beardog.crt;
    ssl_certificate_key /etc/ssl/private/beardog.key;
    
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384;
    ssl_prefer_server_ciphers off;
    
    add_header Strict-Transport-Security "max-age=31536000" always;
    add_header X-Frame-Options DENY always;
    add_header X-Content-Type-Options nosniff always;

    location / {
        proxy_pass http://beardog:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Network Policies (Kubernetes)

```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: beardog
  namespace: beardog
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
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          name: postgres
    ports:
    - protocol: TCP
      port: 5432
  - to: []
    ports:
    - protocol: TCP
      port: 443
    - protocol: TCP
      port: 53
    - protocol: UDP
      port: 53
```

## Backup and Recovery

### Database Backup

```bash
#!/bin/bash
# backup-database.sh
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="beardog_backup_$TIMESTAMP.sql"

pg_dump -h postgres -U beardog -d beardog > /backups/$BACKUP_FILE

# Upload to S3
aws s3 cp /backups/$BACKUP_FILE s3://beardog-backups/database/

# Cleanup old backups (keep 30 days)
find /backups -name "beardog_backup_*.sql" -mtime +30 -delete
```

### Disaster Recovery

```yaml
# disaster-recovery.yml
apiVersion: v1
kind: ConfigMap
metadata:
  name: disaster-recovery
data:
  recovery.sh: |
    #!/bin/bash
    set -e
    
    echo "Starting disaster recovery..."
    
    # Restore database
    LATEST_BACKUP=$(aws s3 ls s3://beardog-backups/database/ | sort | tail -n 1 | awk '{print $4}')
    aws s3 cp s3://beardog-backups/database/$LATEST_BACKUP /tmp/restore.sql
    psql -h postgres -U beardog -d beardog < /tmp/restore.sql
    
    # Verify services
    kubectl rollout status deployment/beardog -n beardog
    
    echo "Disaster recovery complete!"
```

## Performance Tuning

### Resource Allocation

```yaml
resources:
  requests:
    memory: "2Gi"
    cpu: "1000m"
  limits:
    memory: "4Gi"
    cpu: "2000m"
```

### Database Optimization

```sql
-- PostgreSQL optimizations
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
ALTER SYSTEM SET checkpoint_completion_target = 0.9;
ALTER SYSTEM SET wal_buffers = '16MB';
ALTER SYSTEM SET default_statistics_target = 100;
ALTER SYSTEM SET random_page_cost = 1.1;
SELECT pg_reload_conf();
```

### Application Tuning

```toml
[performance]
worker_threads = 8
max_concurrent_connections = 10000
request_timeout_seconds = 30
enable_compression = true
compression_level = 6

[database]
max_connections = 100
connection_timeout = 30
query_timeout = 30
```

## Troubleshooting

### Common Issues

1. **High Memory Usage**
   ```bash
   # Check memory usage
   kubectl top pods -n beardog
   
   # Adjust memory limits
   kubectl patch deployment beardog -n beardog -p '{"spec":{"template":{"spec":{"containers":[{"name":"beardog","resources":{"limits":{"memory":"4Gi"}}}]}}}}'
   ```

2. **Database Connection Issues**
   ```bash
   # Test database connectivity
   kubectl exec -it beardog-pod -n beardog -- psql $BEARDOG_DATABASE_URL -c "SELECT 1;"
   ```

3. **TLS Certificate Issues**
   ```bash
   # Check certificate expiry
   openssl x509 -in /etc/ssl/certs/beardog.crt -text -noout | grep "Not After"
   ```

### Health Checks

```bash
# API Health
curl -f http://localhost:8080/health

# Metrics Health  
curl http://localhost:9091/metrics | grep beardog_up

# Database Health
psql $DATABASE_URL -c "SELECT 1;"

# Redis Health
redis-cli ping
```

This deployment guide provides comprehensive instructions for production BearDog deployments across various platforms and environments. 