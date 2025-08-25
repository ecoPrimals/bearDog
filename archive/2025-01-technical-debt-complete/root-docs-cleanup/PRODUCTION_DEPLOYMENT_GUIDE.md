# 🚀 BearDog Production Deployment Guide

**Version**: 1.0.0  
**Date**: January 2025  
**Status**: Production Ready  

---

## 🎯 **Overview**

This guide provides comprehensive instructions for deploying BearDog in production environments. BearDog is now **production-ready** with perfect ecosystem compliance, hybrid AI architecture, and enterprise-grade security.

---

## 📋 **Prerequisites**

### **System Requirements**
- **OS**: Linux (Ubuntu 20.04+ recommended)
- **CPU**: 4+ cores (8+ recommended for production)
- **Memory**: 4GB minimum (8GB+ recommended)
- **Storage**: 50GB available space
- **Network**: Outbound HTTPS access for ecosystem integration

### **Software Dependencies**
- **Rust**: 1.70+ (latest stable recommended)
- **Docker**: 20.10+ (optional, for containerized deployment)
- **systemd**: For service management
- **curl**: For health checks and API testing

### **Security Requirements**
- **TLS Certificates**: Valid certificates for HTTPS endpoints
- **Firewall**: Configured for secure network access
- **HSM**: Hardware Security Module (optional but recommended)
- **Monitoring**: External monitoring system integration

---

## 🔧 **Environment Configuration**

### **Required Environment Variables**

```bash
# Core Configuration
export BEARDOG_ADMIN_PASSWORD="your_secure_admin_password"
export BEARDOG_DATABASE_URL="postgresql://user:pass@localhost:5432/beardog"
export BEARDOG_API_BIND_ADDRESS="0.0.0.0:8080"

# Universal Adapter Configuration
export BEARDOG_UNIVERSAL_ADAPTER_ENDPOINT="https://ecosystem.adapter.local"
export BEARDOG_ENABLE_TLS="true"
export BEARDOG_TLS_CERT_PATH="/opt/beardog/certs/beardog.crt"
export BEARDOG_TLS_KEY_PATH="/opt/beardog/certs/beardog.key"

# CORS Configuration
export BEARDOG_CORS_ORIGINS="https://admin.beardog.local,https://api.ecosystem.local"

# Hybrid AI Configuration
export BEARDOG_AI_ML_MODELS_PATH="/opt/beardog/models"
export BEARDOG_AI_EXTERNAL_ROUTING="true"

# HSM Configuration (if using hardware HSM)
export BEARDOG_HSM_TYPE="hardware"
export BEARDOG_HSM_DEVICE_PATH="/dev/hsm0"
export BEARDOG_HSM_KEY_STORAGE="/opt/beardog/secure/keys"

# Monitoring Configuration
export BEARDOG_METRICS_ENABLED="true"
export BEARDOG_METRICS_PORT="9090"
export BEARDOG_LOG_LEVEL="info"
```

### **Optional Environment Variables**

```bash
# Performance Tuning
export BEARDOG_WORKER_THREADS="8"
export BEARDOG_MAX_CONNECTIONS="1000"
export BEARDOG_REQUEST_TIMEOUT="30s"

# Security Enhancements
export BEARDOG_RATE_LIMITING="100/min"
export BEARDOG_AUDIT_LOG_ENABLED="true"
export BEARDOG_THREAT_DETECTION_SENSITIVITY="high"

# Ecosystem Integration
export BEARDOG_SERVICE_DISCOVERY_INTERVAL="30s"
export BEARDOG_HEALTH_CHECK_INTERVAL="10s"
export BEARDOG_CAPABILITY_REFRESH_INTERVAL="300s"
```

---

## 🚀 **Deployment Options**

### **Option 1: Automated Deployment (Recommended)**

```bash
# Clone the repository
git clone https://github.com/ecoprimal/beardog.git
cd beardog

# Set environment variables
source deployment/production.env

# Run automated deployment
./scripts/production_deployment.sh deploy
```

### **Option 2: Manual Deployment**

#### **Step 1: Build for Production**
```bash
# Clean and build with optimizations
cargo clean
RUSTFLAGS="-C target-cpu=native -C opt-level=3" cargo build --release --workspace

# Run tests to verify build
BEARDOG_ADMIN_PASSWORD="$BEARDOG_ADMIN_PASSWORD" cargo test --release --workspace
```

#### **Step 2: Create Deployment Structure**
```bash
# Create directories
sudo mkdir -p /opt/beardog/{bin,config,logs,data,certs,models}

# Copy binaries
sudo cp target/release/beardog* /opt/beardog/bin/

# Copy configuration
sudo cp beardog-config.toml /opt/beardog/config/
sudo cp scripts/*.sh /opt/beardog/scripts/

# Set permissions
sudo useradd -r -s /bin/false -d /opt/beardog beardog
sudo chown -R beardog:beardog /opt/beardog
```

#### **Step 3: Configure Service**
```bash
# Create systemd service
sudo cp deployment/beardog.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable beardog
```

#### **Step 4: Start and Validate**
```bash
# Start service
sudo systemctl start beardog

# Check status
sudo systemctl status beardog

# Validate health
curl -f "http://localhost:8080/health"
```

### **Option 3: Container Deployment**

#### **Build Container**
```bash
# Build Docker image
docker build -t beardog:1.0.0 .

# Or use pre-built image
docker pull ecoprimal/beardog:1.0.0
```

#### **Deploy with Docker Compose**
```yaml
# docker-compose.yml
version: '3.8'
services:
  beardog:
    image: beardog:1.0.0
    restart: unless-stopped
    ports:
      - "8080:8080"
      - "8443:8443"
      - "9090:9090"
    environment:
      - BEARDOG_ADMIN_PASSWORD=${BEARDOG_ADMIN_PASSWORD}
      - BEARDOG_DATABASE_URL=${BEARDOG_DATABASE_URL}
      - BEARDOG_API_BIND_ADDRESS=0.0.0.0:8080
    volumes:
      - beardog_data:/opt/beardog/data
      - beardog_logs:/opt/beardog/logs
      - ./certs:/opt/beardog/certs:ro
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3

volumes:
  beardog_data:
  beardog_logs:
```

---

## 🔒 **Security Configuration**

### **TLS/HTTPS Setup**
```bash
# Generate self-signed certificates (development)
openssl req -x509 -newkey rsa:4096 -keyout /opt/beardog/certs/beardog.key \
    -out /opt/beardog/certs/beardog.crt -days 365 -nodes \
    -subj "/CN=beardog.local"

# Or use Let's Encrypt (production)
certbot certonly --standalone -d beardog.yourdomain.com
cp /etc/letsencrypt/live/beardog.yourdomain.com/fullchain.pem /opt/beardog/certs/beardog.crt
cp /etc/letsencrypt/live/beardog.yourdomain.com/privkey.pem /opt/beardog/certs/beardog.key
```

### **Firewall Configuration**
```bash
# Configure UFW
sudo ufw allow ssh
sudo ufw allow 8080/tcp comment "BearDog API"
sudo ufw allow 8443/tcp comment "BearDog HTTPS"
sudo ufw allow 9090/tcp comment "BearDog Metrics"
sudo ufw enable
```

### **HSM Integration**
```bash
# Configure hardware HSM (if available)
echo "hsm_type = \"hardware\"" >> /opt/beardog/config/hsm.conf
echo "key_storage_path = \"/opt/beardog/secure/keys\"" >> /opt/beardog/config/hsm.conf
echo "attestation_required = true" >> /opt/beardog/config/hsm.conf

# Set secure permissions
sudo chmod 600 /opt/beardog/config/hsm.conf
sudo chown beardog:beardog /opt/beardog/config/hsm.conf
```

---

## 📊 **Monitoring & Observability**

### **Health Monitoring**
```bash
# Health check endpoint
curl -f "http://localhost:8080/health"

# Detailed health with authentication
curl -H "Authorization: Bearer $AUTH_TOKEN" \
     "http://localhost:8080/health/detailed"
```

### **Metrics Collection**
```bash
# Prometheus metrics endpoint
curl "http://localhost:9090/metrics"

# Key metrics to monitor:
# - beardog_requests_total
# - beardog_request_duration_seconds
# - beardog_threat_detections_total
# - beardog_ml_processing_duration_seconds
# - beardog_universal_adapter_requests_total
```

### **Log Management**
```bash
# Service logs
sudo journalctl -u beardog -f

# Application logs
tail -f /opt/beardog/logs/beardog.log

# Security audit logs
tail -f /opt/beardog/logs/security_audit.log
```

### **External Monitoring Integration**

#### **Prometheus Configuration**
```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'beardog'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 15s
    metrics_path: /metrics
```

#### **Grafana Dashboard**
```json
{
  "dashboard": {
    "title": "BearDog Security Primal",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(beardog_requests_total[5m])"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph", 
        "targets": [
          {
            "expr": "histogram_quantile(0.95, beardog_request_duration_seconds_bucket)"
          }
        ]
      },
      {
        "title": "Threat Detections",
        "type": "stat",
        "targets": [
          {
            "expr": "increase(beardog_threat_detections_total[1h])"
          }
        ]
      }
    ]
  }
}
```

---

## 🌐 **Ecosystem Integration**

### **Universal Adapter Configuration**
```toml
# /opt/beardog/config/universal_adapter.toml
[universal_adapter]
endpoint = "https://ecosystem.adapter.local"
timeout = "30s"
retry_attempts = 3
health_check_interval = "60s"

[capabilities]
ai_intelligence = true
compute_orchestration = true
service_mesh = true
storage_services = true
system_integration = true

[routing]
discovery_method = "capability_based"
load_balancing = "round_robin"
circuit_breaker_enabled = true
```

### **Service Registration**
BearDog automatically registers with the ecosystem on startup:

1. **Capability Advertisement**: Publishes security capabilities
2. **Service Discovery**: Discovers other primal capabilities
3. **Health Registration**: Registers health check endpoints
4. **Load Balancer Integration**: Configures load balancing

---

## 🔧 **Operational Tasks**

### **Service Management**
```bash
# Start/Stop/Restart
sudo systemctl start beardog
sudo systemctl stop beardog
sudo systemctl restart beardog

# Status and logs
sudo systemctl status beardog
sudo journalctl -u beardog --no-pager -n 50

# Configuration reload
sudo systemctl reload beardog
```

### **Configuration Updates**
```bash
# Edit main configuration
sudo nano /opt/beardog/config/beardog-config.toml

# Edit service configuration
sudo systemctl edit beardog

# Apply changes
sudo systemctl daemon-reload
sudo systemctl restart beardog
```

### **Backup and Recovery**
```bash
# Backup configuration and data
sudo tar -czf beardog_backup_$(date +%Y%m%d).tar.gz \
    /opt/beardog/config \
    /opt/beardog/data

# Restore from backup
sudo tar -xzf beardog_backup_YYYYMMDD.tar.gz -C /
sudo chown -R beardog:beardog /opt/beardog
sudo systemctl restart beardog
```

### **Performance Tuning**
```bash
# Run performance benchmarks
./scripts/performance_benchmark.sh

# Monitor resource usage
htop
iotop
nethogs

# Tune based on workload
# Edit /opt/beardog/config/performance.toml
```

---

## 🚨 **Troubleshooting**

### **Common Issues**

#### **Service Won't Start**
```bash
# Check logs
sudo journalctl -u beardog --no-pager -n 50

# Verify configuration
sudo -u beardog /opt/beardog/bin/beardog --config-check

# Check permissions
ls -la /opt/beardog/
```

#### **High Memory Usage**
```bash
# Check memory usage
ps aux | grep beardog
sudo systemctl status beardog

# Tune memory settings
echo "BEARDOG_MAX_MEMORY=2GB" >> /opt/beardog/config/environment
sudo systemctl restart beardog
```

#### **Universal Adapter Connection Issues**
```bash
# Test adapter connectivity
curl -f "$BEARDOG_UNIVERSAL_ADAPTER_ENDPOINT/health"

# Check routing configuration
cat /opt/beardog/config/universal_adapter.toml

# Verify network access
netstat -tlnp | grep beardog
```

### **Performance Issues**
```bash
# Run diagnostics
./scripts/performance_benchmark.sh quick

# Check resource limits
ulimit -a
cat /proc/$(pgrep beardog)/limits

# Monitor in real-time
sudo perf top -p $(pgrep beardog)
```

---

## ✅ **Validation Checklist**

### **Pre-Deployment**
- [ ] Environment variables configured
- [ ] TLS certificates installed
- [ ] Firewall rules configured
- [ ] HSM (if applicable) configured
- [ ] Database connectivity verified
- [ ] Universal adapter endpoint accessible

### **Post-Deployment**
- [ ] Service starts successfully
- [ ] Health check passes
- [ ] API endpoints responsive
- [ ] Metrics collection working
- [ ] Log files being written
- [ ] Universal adapter registration successful
- [ ] Performance benchmarks meet targets

### **Production Readiness**
- [ ] External monitoring configured
- [ ] Backup procedures tested
- [ ] Security audit completed
- [ ] Load testing passed
- [ ] Disaster recovery plan validated
- [ ] Documentation updated

---

## 📞 **Support**

### **Getting Help**
- **Documentation**: https://beardog.docs.ecoprimal.local
- **Issues**: https://github.com/ecoprimal/beardog/issues
- **Security**: security@ecoprimal.local
- **Support**: support@ecoprimal.local

### **Emergency Contacts**
- **On-call Engineer**: +1-555-BEARDOG
- **Security Team**: security-emergency@ecoprimal.local
- **Incident Response**: incident@ecoprimal.local

---

**Status**: ✅ **PRODUCTION READY**  
**Last Updated**: January 2025  
**Version**: 1.0.0 