# 🚀 Beardog Production Deployment Guide

## 📋 **Pre-Deployment Checklist**

### ✅ **System Requirements**
- **OS**: Linux (Ubuntu 20.04+ or RHEL 8+)
- **Memory**: Minimum 4GB RAM, Recommended 8GB+
- **Storage**: 20GB available disk space
- **Network**: Outbound HTTPS (443), configurable service ports
- **Rust**: 1.70+ (for compilation)

### ✅ **Build Verification**
```bash
# Verify clean build
cargo build --release --all
cargo test --all
cargo clippy --all-targets --all-features -- -D warnings -A dead-code
```

## 🔧 **Environment Configuration**

### **Required Environment Variables**
```bash
# Core Service Configuration
export BEARDOG_ENDPOINT="https://your-beardog-instance.com"
export BEARDOG_PORT="8080"
export BEARDOG_LOG_LEVEL="info"

# Ecosystem Integration (Sovereignty Compliant)
export COMPUTE_CAPABILITY_ENDPOINT="https://compute.your-ecosystem.com"
export MESH_CAPABILITY_ENDPOINT="https://mesh.your-ecosystem.com"
export AI_CAPABILITY_ENDPOINT="https://ai.your-ecosystem.com"

# Database Configuration
export DATABASE_URL="postgresql://user:password@localhost:5432/beardog_prod"
export DATABASE_MAX_CONNECTIONS="100"

# Security Configuration
export HSM_PROVIDER="software" # or "hardware" for production HSMs
export ENCRYPTION_KEY_PATH="/etc/beardog/keys/"
export TLS_CERT_PATH="/etc/beardog/certs/server.crt"
export TLS_KEY_PATH="/etc/beardog/certs/server.key"

# Monitoring and Observability
export METRICS_ENDPOINT="0.0.0.0:9090"
export JAEGER_ENDPOINT="http://jaeger:14268/api/traces"
export LOG_FORMAT="json" # or "text" for development
```

### **Configuration Files**
Create `/etc/beardog/config.toml`:
```toml
[server]
host = "0.0.0.0"
port = 8080
workers = 4

[database]
url = "${DATABASE_URL}"
max_connections = 100
connect_timeout = 30

[security]
hsm_provider = "${HSM_PROVIDER}"
encryption_key_path = "${ENCRYPTION_KEY_PATH}"
tls_enabled = true

[ecosystem]
capability_discovery_enabled = true
compute_endpoint = "${COMPUTE_CAPABILITY_ENDPOINT}"
mesh_endpoint = "${MESH_CAPABILITY_ENDPOINT}"
ai_endpoint = "${AI_CAPABILITY_ENDPOINT}"

[monitoring]
metrics_enabled = true
tracing_enabled = true
health_check_interval = 30
```

## 🐳 **Docker Deployment**

### **Dockerfile**
```dockerfile
FROM rust:1.70-slim as builder

WORKDIR /app
COPY . .
RUN cargo build --release --all

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/beardog* ./
COPY configs/ ./configs/

EXPOSE 8080 9090
CMD ["./beardog-core"]
```

### **Docker Compose**
```yaml
version: '3.8'
services:
  beardog:
    build: .
    ports:
      - "8080:8080"
      - "9090:9090"
    environment:
      - DATABASE_URL=postgresql://postgres:password@db:5432/beardog
      - BEARDOG_LOG_LEVEL=info
      - HSM_PROVIDER=software
    depends_on:
      - db
      - jaeger
    volumes:
      - ./configs:/app/configs:ro
      - beardog_data:/app/data
    restart: unless-stopped

  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=beardog
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: unless-stopped

  jaeger:
    image: jaegertracing/all-in-one:latest
    ports:
      - "16686:16686"
    restart: unless-stopped

volumes:
  postgres_data:
  beardog_data:
```

## 🚀 **Deployment Steps**

### **1. System Preparation**
```bash
# Create system user
sudo useradd --system --home /opt/beardog --shell /bin/false beardog

# Create directories
sudo mkdir -p /opt/beardog/{bin,config,data,logs}
sudo mkdir -p /etc/beardog/{certs,keys}

# Set permissions
sudo chown -R beardog:beardog /opt/beardog
sudo chmod 750 /etc/beardog/keys
```

### **2. Binary Installation**
```bash
# Copy release binaries
sudo cp target/release/beardog* /opt/beardog/bin/
sudo chown beardog:beardog /opt/beardog/bin/*
sudo chmod 755 /opt/beardog/bin/*

# Copy configuration
sudo cp configs/beardog-config.toml /etc/beardog/config.toml
sudo chown beardog:beardog /etc/beardog/config.toml
sudo chmod 640 /etc/beardog/config.toml
```

### **3. Service Configuration**
Create `/etc/systemd/system/beardog.service`:
```ini
[Unit]
Description=Beardog Ecosystem Service
After=network.target postgresql.service
Requires=postgresql.service

[Service]
Type=simple
User=beardog
Group=beardog
WorkingDirectory=/opt/beardog
ExecStart=/opt/beardog/bin/beardog-core
EnvironmentFile=/etc/beardog/environment
Restart=always
RestartSec=5
StandardOutput=journal
StandardError=journal

# Security hardening
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ReadWritePaths=/opt/beardog/data /opt/beardog/logs

[Install]
WantedBy=multi-user.target
```

### **4. Start Services**
```bash
# Reload systemd
sudo systemctl daemon-reload

# Enable and start service
sudo systemctl enable beardog
sudo systemctl start beardog

# Check status
sudo systemctl status beardog
```

## 📊 **Health Checks**

### **Service Health**
```bash
# Check service status
curl -f http://localhost:8080/health

# Check metrics
curl http://localhost:9090/metrics

# Check logs
sudo journalctl -u beardog -f
```

### **Database Health**
```bash
# Connect to database
psql $DATABASE_URL -c "SELECT 1;"
```

## 🔍 **Monitoring Setup**

### **Prometheus Configuration**
```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'beardog'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 5s
    metrics_path: /metrics
```

### **Grafana Dashboard**
- Import dashboard ID: `beardog-ecosystem-dashboard`
- Configure data source: `http://prometheus:9090`
- Set up alerts for critical metrics

## 🔒 **Security Hardening**

### **TLS Configuration**
```bash
# Generate certificates (production should use proper CA)
openssl req -x509 -newkey rsa:4096 -keyout /etc/beardog/certs/server.key \
  -out /etc/beardog/certs/server.crt -days 365 -nodes \
  -subj "/CN=your-domain.com"

# Set permissions
sudo chmod 600 /etc/beardog/certs/server.key
sudo chmod 644 /etc/beardog/certs/server.crt
```

### **Firewall Rules**
```bash
# Allow service ports
sudo ufw allow 8080/tcp
sudo ufw allow 9090/tcp

# Restrict database access
sudo ufw deny 5432/tcp
```

## 🔄 **Backup and Recovery**

### **Database Backup**
```bash
# Daily backup script
#!/bin/bash
pg_dump $DATABASE_URL | gzip > /opt/beardog/backups/beardog-$(date +%Y%m%d).sql.gz
find /opt/beardog/backups -name "beardog-*.sql.gz" -mtime +7 -delete
```

### **Configuration Backup**
```bash
# Backup configuration and keys
tar -czf /opt/beardog/backups/config-$(date +%Y%m%d).tar.gz \
  /etc/beardog/config.toml \
  /etc/beardog/keys/ \
  /etc/beardog/certs/
```

## 🚨 **Troubleshooting**

### **Common Issues**
1. **Service won't start**: Check logs with `journalctl -u beardog`
2. **Database connection**: Verify `DATABASE_URL` and network connectivity
3. **Permission errors**: Ensure proper ownership and permissions
4. **Memory issues**: Monitor with `htop` and adjust worker count

### **Log Locations**
- **Service logs**: `journalctl -u beardog`
- **Application logs**: `/opt/beardog/logs/`
- **System logs**: `/var/log/syslog`

## 📈 **Performance Tuning**

### **Database Optimization**
```sql
-- Recommended PostgreSQL settings
ALTER SYSTEM SET shared_buffers = '256MB';
ALTER SYSTEM SET effective_cache_size = '1GB';
ALTER SYSTEM SET maintenance_work_mem = '64MB';
SELECT pg_reload_conf();
```

### **System Limits**
```bash
# /etc/security/limits.conf
beardog soft nofile 65536
beardog hard nofile 65536
```

## ✅ **Deployment Verification**

### **Final Checklist**
- [ ] All services running and healthy
- [ ] Database connectivity verified
- [ ] TLS certificates valid
- [ ] Monitoring dashboards operational
- [ ] Backup scripts configured
- [ ] Log rotation configured
- [ ] Security hardening applied
- [ ] Performance baselines established

### **Post-Deployment Tests**
```bash
# Run integration tests against production
cargo test --test integration_tests -- --ignored

# Load testing (optional)
# wrk -t12 -c400 -d30s http://localhost:8080/health
```

---

## 📞 **Support and Maintenance**

### **Regular Maintenance Tasks**
- Monitor disk usage and logs
- Review security updates monthly
- Backup verification weekly
- Performance metrics review

### **Emergency Procedures**
- Service restart: `sudo systemctl restart beardog`
- Rollback: Keep previous release binaries
- Database recovery: Use latest backup

**🎉 Your Beardog ecosystem is now production-ready!** 