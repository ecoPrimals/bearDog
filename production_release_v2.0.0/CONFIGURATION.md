# BearDog Configuration Guide

## Overview

BearDog supports comprehensive configuration through environment variables, configuration files, and runtime parameters. This guide covers all configurable aspects of the system.

## Environment Variables

### 🌐 Network Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_API_BIND_ADDRESS` | `0.0.0.0:8080` | API server bind address |
| `BEARDOG_API_PORT` | `8080` | API server port |
| `BEARDOG_BIND_HOST` | `0.0.0.0` | Host to bind all services |
| `BEARDOG_HTTPS_PORT` | `8443` | HTTPS server port |
| `BEARDOG_ADMIN_PORT` | `9092` | Admin interface port |
| `BEARDOG_METRICS_PORT` | `9091` | Metrics collection port |
| `BEARDOG_ENABLE_TLS` | `true` | Enable TLS/HTTPS |

### 🗄️ Database Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_DATABASE_URL` | `postgresql://localhost/beardog` | Full database connection URL |
| `BEARDOG_DB_HOST` | `beardog-db` | Database host (Docker/K8s service name) |
| `BEARDOG_DB_PORT` | `5432` | Database port |
| `BEARDOG_DB_NAME` | `beardog` | Database name |
| `BEARDOG_DB_USER` | `beardog` | Database username |
| `BEARDOG_DB_PASSWORD` | - | Database password (required) |

### 🔗 External Services

| Variable | Default | Description |
|----------|---------|-------------|
| `PROMETHEUS_ENDPOINT` | `http://prometheus.ecosystem.internal:9090` | Prometheus metrics endpoint |
| `GRAFANA_ENDPOINT` | `http://grafana.ecosystem.internal:3000` | Grafana dashboard endpoint |
| `SONGBIRD_ENDPOINT` | `https://songbird.ecosystem.internal:8443` | SongBird service discovery |
| `NESTGATE_ENDPOINT` | `https://nestgate.ecosystem.internal:8443` | NestGate storage service |
| `SQUIRREL_ENDPOINT` | `https://squirrel.ecosystem.internal:8443` | Squirrel plugin system |
| `TOADSTOOL_ENDPOINT` | `https://toadstool.ecosystem.internal:8443` | ToadStool compute provider |

### 🛡️ Security Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_JWT_SECRET` | - | JWT signing secret (required for production) |
| `BEARDOG_ENCRYPTION_KEY` | - | Data encryption key (required for production) |
| `BEARDOG_API_KEY` | - | API authentication key |
| `BEARDOG_FORCE_HTTPS` | `true` | Redirect HTTP to HTTPS |
| `BEARDOG_CORS_ORIGINS` | `*` | Allowed CORS origins (comma-separated) |

### 📊 Monitoring & Logging

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_LOG_LEVEL` | `info` | Logging level (trace, debug, info, warn, error) |
| `BEARDOG_ENABLE_METRICS` | `true` | Enable metrics collection |
| `BEARDOG_METRICS_INTERVAL` | `30` | Metrics collection interval (seconds) |
| `SPLUNK_HEC_ENDPOINT` | `https://splunk.ecosystem.internal:8088` | Splunk HEC endpoint |
| `SPLUNK_HEC_TOKEN` | - | Splunk HEC authentication token |

### 🧬 Genetic System Configuration

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_ENABLE_GENETICS` | `true` | Enable genetic spawning system |
| `BEARDOG_GENETIC_MUTATION_RATE` | `0.1` | Mutation rate for genetic operations |
| `BEARDOG_GENETIC_FITNESS_THRESHOLD` | `0.7` | Minimum fitness threshold |
| `BEARDOG_MAX_SPAWNS_PER_NODE` | `10` | Maximum genetic spawns per node |

### 🔧 Performance Tuning

| Variable | Default | Description |
|----------|---------|-------------|
| `BEARDOG_MAX_CONNECTIONS` | `1000` | Maximum concurrent connections |
| `BEARDOG_WORKER_THREADS` | `(CPU cores)` | Number of worker threads |
| `BEARDOG_CONNECTION_TIMEOUT` | `30` | Connection timeout (seconds) |
| `BEARDOG_REQUEST_TIMEOUT` | `30` | Request timeout (seconds) |
| `BEARDOG_ENABLE_COMPRESSION` | `true` | Enable response compression |

## Configuration File

BearDog can load configuration from TOML files specified by `BEARDOG_CONFIG_FILE`:

```toml
[network]
bind_host = "0.0.0.0"
api_port = 8080
https_port = 8443
enable_tls = true

[database]
host = "postgres.internal"
port = 5432
database = "beardog_production"
max_connections = 100

[security]
force_https = true
jwt_expiry_hours = 24
enable_rate_limiting = true

[monitoring]
log_level = "info"
enable_metrics = true
metrics_interval = 30

[genetics]
enable_genetics = true
mutation_rate = 0.1
fitness_threshold = 0.7

[endpoints]
songbird_endpoint = "https://songbird.internal:8443"
nestgate_endpoint = "https://nestgate.internal:8443"
```

## Docker Configuration

### Docker Compose

```yaml
version: '3.8'
services:
  beardog:
    image: beardog:latest
    environment:
      - BEARDOG_DATABASE_URL=postgresql://postgres:password@db:5432/beardog
      - BEARDOG_API_BIND_ADDRESS=0.0.0.0:8080
      - BEARDOG_ENABLE_TLS=false
      - BEARDOG_LOG_LEVEL=info
      - PROMETHEUS_ENDPOINT=http://prometheus:9090
      - GRAFANA_ENDPOINT=http://grafana:3000
    ports:
      - "8080:8080"
      - "9091:9091"
    depends_on:
      - db
      - prometheus
    
  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=beardog
      - POSTGRES_USER=beardog
      - POSTGRES_PASSWORD=secure_password
    volumes:
      - beardog_data:/var/lib/postgresql/data
```

## Kubernetes Configuration

### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-config
data:
  BEARDOG_API_BIND_ADDRESS: "0.0.0.0:8080"
  BEARDOG_LOG_LEVEL: "info"
  BEARDOG_ENABLE_METRICS: "true"
  PROMETHEUS_ENDPOINT: "http://prometheus.monitoring.svc.cluster.local:9090"
  GRAFANA_ENDPOINT: "http://grafana.monitoring.svc.cluster.local:3000"
  SONGBIRD_ENDPOINT: "https://songbird.ecosystem.svc.cluster.local:8443"
  NESTGATE_ENDPOINT: "https://nestgate.ecosystem.svc.cluster.local:8443"
```

### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
spec:
  replicas: 3
  selector:
    matchLabels:
      app: beardog
  template:
    metadata:
      labels:
        app: beardog
    spec:
      containers:
      - name: beardog
        image: beardog:latest
        envFrom:
        - configMapRef:
            name: beardog-config
        - secretRef:
            name: beardog-secrets
        ports:
        - containerPort: 8080
          name: api
        - containerPort: 9091
          name: metrics
        resources:
          requests:
            memory: "256Mi"
            cpu: "100m"
          limits:
            memory: "1Gi"
            cpu: "500m"
```

## Production Deployment Best Practices

### 🔒 Security

1. **Always use HTTPS in production** - Set `BEARDOG_ENABLE_TLS=true`
2. **Use strong secrets** - Generate cryptographically secure values for all secret keys
3. **Restrict CORS origins** - Set `BEARDOG_CORS_ORIGINS` to specific domains
4. **Enable rate limiting** - Configure appropriate rate limits for your use case
5. **Use service mesh** - Deploy with internal service discovery names

### 📈 Performance

1. **Tune connection pools** - Set appropriate database connection limits
2. **Configure worker threads** - Match to your system's CPU capabilities  
3. **Enable compression** - Reduces bandwidth usage
4. **Use CDN** - For static assets and API responses
5. **Monitor metrics** - Set up comprehensive monitoring

### 🏗️ High Availability

1. **Run multiple replicas** - Deploy at least 3 instances
2. **Use external databases** - Separate database infrastructure
3. **Configure health checks** - Enable Kubernetes/Docker health probes
4. **Set resource limits** - Prevent resource exhaustion
5. **Use persistent volumes** - For stateful components

### 📊 Monitoring

1. **Enable all metrics** - Use Prometheus for comprehensive monitoring
2. **Set up alerting** - Configure alerts for critical conditions
3. **Use structured logging** - Enable JSON logging for better parsing
4. **Monitor genetic operations** - Track spawning success rates
5. **Performance dashboards** - Create comprehensive Grafana dashboards

## Configuration Validation

BearDog automatically validates configuration on startup:

- ✅ **Port conflicts** - Ensures no port conflicts between services
- ✅ **Database connectivity** - Validates database connection on startup  
- ✅ **TLS certificates** - Validates certificate files if TLS is enabled
- ✅ **External services** - Health checks for external service endpoints
- ✅ **Genetic parameters** - Validates genetic algorithm parameters
- ✅ **Security settings** - Ensures production security requirements

## Troubleshooting

### Common Issues

1. **Port binding errors** - Check for port conflicts and permissions
2. **Database connection failures** - Verify database URL and credentials
3. **TLS certificate errors** - Check certificate file paths and permissions
4. **Memory issues** - Adjust worker thread and connection limits
5. **Network timeouts** - Increase timeout values for slow networks

### Debug Configuration

Enable debug logging to see configuration loading:

```bash
BEARDOG_LOG_LEVEL=debug ./beardog
```

### Configuration Validation

Test configuration without starting services:

```bash
./beardog --validate-config --config-file /etc/beardog/config.toml
```

## Migration Guide

### From v1.0 to v2.0

- `BEARDOG_HOST` → `BEARDOG_BIND_HOST`
- `BEARDOG_PORT` → `BEARDOG_API_PORT`
- `DATABASE_URL` → `BEARDOG_DATABASE_URL`
- `ENABLE_SSL` → `BEARDOG_ENABLE_TLS`

See [CHANGELOG.md](CHANGELOG.md) for complete migration details. 