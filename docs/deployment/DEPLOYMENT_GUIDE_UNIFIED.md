# BearDog Unified Deployment Guide

**Version**: 3.0.0 - Canonical Modernization Complete  
**Date**: January 2025  
**Status**: 🚀 **PRODUCTION READY**

## 🎯 Overview

This guide demonstrates how to deploy BearDog using the new **unified configuration system** that eliminates duplication and provides environment-driven configuration management.

## 🏗️ Unified Configuration Architecture

### **Key Innovation: Single Template + Environment Variables**

Instead of maintaining multiple configuration files, BearDog now uses:
- **One unified template**: `configs/beardog-config-template.toml`
- **Environment-specific variables**: `configs/environments/*.env`
- **Zero duplication**: All configuration consolidated

## 🚀 Quick Start Deployment

### **Development Deployment**

```bash
# 1. Clone and setup
git clone <repository>
cd beardog

# 2. Load development environment
source configs/environments/development.env

# 3. Use unified configuration
export BEARDOG_CONFIG=configs/beardog-config-template.toml

# 4. Start development server
cargo run
```

### **Production Deployment**

```bash
# 1. Setup production environment
source configs/environments/production.env

# 2. Set production-specific secrets
export BEARDOG_HSM_PIN="your-secure-hsm-pin"
export BEARDOG_SMTP_SERVER="smtp.your-company.com"
export BEARDOG_CERT_PATH="/etc/ssl/certs/your-cert.crt"
export BEARDOG_KEY_PATH="/etc/ssl/private/your-key.key"

# 3. Use unified configuration
export BEARDOG_CONFIG=configs/beardog-config-template.toml

# 4. Start production server
./target/release/beardog-server
```

## 🔧 Configuration Customization

### **Environment Variables Reference**

#### **Application Settings**
```bash
export BEARDOG_APP_NAME="Your-BearDog-Instance"
export BEARDOG_ENVIRONMENT="production"  # development|staging|production
export BEARDOG_DEBUG="false"
export BEARDOG_STANDALONE_MODE="false"
```

#### **Network Configuration**
```bash
# HTTP Settings
export BEARDOG_HTTP_ENABLED="true"
export BEARDOG_HTTP_BIND="0.0.0.0:8080"
export BEARDOG_HTTP_PORT="8080"

# HTTPS Settings (Production)
export BEARDOG_HTTPS_ENABLED="true"
export BEARDOG_HTTPS_BIND="0.0.0.0:8443"
export BEARDOG_CERT_PATH="/path/to/your/cert.crt"
export BEARDOG_KEY_PATH="/path/to/your/key.key"

# Security Settings
export BEARDOG_RATE_LIMIT_RPM="1000"
export BEARDOG_CORS_ORIGINS="https://your-dashboard.com"
```

#### **Security Configuration**
```bash
export BEARDOG_SECURITY_LEVEL="High"        # Standard|High|Maximum
export BEARDOG_HSM_ENABLED="true"
export BEARDOG_TOKEN_EXPIRATION="3600"      # seconds
export BEARDOG_MFA_REQUIRED="true"
export BEARDOG_PASSWORD_MIN_LENGTH="16"
```

#### **Database & Storage**
```bash
export BEARDOG_STORAGE_DISCOVERY="songbird"  # local|songbird|kubernetes
export BEARDOG_ZFS_ENCRYPTION="true"
export BEARDOG_SECURITY_RETENTION_DAYS="2555"  # 7 years for compliance
export BEARDOG_AUDIT_RETENTION_DAYS="2555"
```

## 🐳 Docker Deployment

### **Dockerfile**
```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/beardog-server /usr/local/bin/
COPY --from=builder /app/configs /etc/beardog/configs/

# Use unified configuration
ENV BEARDOG_CONFIG=/etc/beardog/configs/beardog-config-template.toml

EXPOSE 8080 8443
CMD ["beardog-server"]
```

### **Docker Compose**
```yaml
version: '3.8'
services:
  beardog:
    build: .
    ports:
      - "8080:8080"
      - "8443:8443"
    environment:
      # Load from environment file
      - BEARDOG_ENVIRONMENT=production
      - BEARDOG_HTTPS_ENABLED=true
      - BEARDOG_SECURITY_LEVEL=High
      - BEARDOG_HSM_ENABLED=true
    env_file:
      - configs/environments/production.env
    volumes:
      - /etc/ssl/certs:/etc/ssl/certs:ro
      - /var/log/beardog:/var/log/beardog
    restart: unless-stopped
```

## ☸️ Kubernetes Deployment

### **ConfigMap**
```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: beardog-config
data:
  BEARDOG_ENVIRONMENT: "production"
  BEARDOG_SECURITY_LEVEL: "High"
  BEARDOG_HSM_ENABLED: "true"
  BEARDOG_HTTPS_ENABLED: "true"
  BEARDOG_RATE_LIMIT_RPM: "1000"
  BEARDOG_ZFS_ENCRYPTION: "true"
```

### **Secret**
```yaml
apiVersion: v1
kind: Secret
metadata:
  name: beardog-secrets
type: Opaque
stringData:
  BEARDOG_HSM_PIN: "your-secure-hsm-pin"
  BEARDOG_SMTP_SERVER: "smtp.your-company.com"
```

### **Deployment**
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
        ports:
        - containerPort: 8080
        - containerPort: 8443
        envFrom:
        - configMapRef:
            name: beardog-config
        - secretRef:
            name: beardog-secrets
        env:
        - name: BEARDOG_CONFIG
          value: "/etc/beardog/configs/beardog-config-template.toml"
        volumeMounts:
        - name: ssl-certs
          mountPath: /etc/ssl/certs
          readOnly: true
      volumes:
      - name: ssl-certs
        secret:
          secretName: ssl-certificates
```

## 🔐 Security Hardening

### **Production Security Checklist**

- [ ] **HSM Configuration**
  ```bash
  export BEARDOG_HSM_ENABLED="true"
  export BEARDOG_HSM_PROVIDER="PKCS11"
  export BEARDOG_HSM_PIN="<secure-pin>"
  ```

- [ ] **SSL/TLS Setup**
  ```bash
  export BEARDOG_HTTPS_ENABLED="true"
  export BEARDOG_CLIENT_CERT_VERIFICATION="true"
  export BEARDOG_CERT_PATH="/etc/ssl/certs/beardog.crt"
  export BEARDOG_KEY_PATH="/etc/ssl/private/beardog.key"
  ```

- [ ] **Authentication Hardening**
  ```bash
  export BEARDOG_MFA_REQUIRED="true"
  export BEARDOG_PASSWORD_MIN_LENGTH="16"
  export BEARDOG_MAX_FAILED_LOGINS="3"
  export BEARDOG_SESSION_TIMEOUT="1h"
  ```

- [ ] **Network Security**
  ```bash
  export BEARDOG_RATE_LIMIT_RPM="1000"
  export BEARDOG_CORS_ORIGINS="https://trusted-domain.com"
  export BEARDOG_TRUSTED_IP_RANGES="10.0.0.0/8,172.16.0.0/12"
  ```

## 📊 Monitoring & Observability

### **Metrics Configuration**
```bash
export BEARDOG_FEATURE_MONITORING="true"
export BEARDOG_METRICS_INTERVAL="60"
export BEARDOG_HEALTH_CHECK_INTERVAL="30"
export BEARDOG_PROMETHEUS_ENABLED="true"
```

### **Logging Configuration**
```bash
export BEARDOG_LOG_LEVEL="INFO"  # DEBUG|INFO|WARN|ERROR
export BEARDOG_MAX_LOG_FILES="10"
export BEARDOG_MAX_LOG_FILE_SIZE="10485760"  # 10MB
```

## 🚀 Performance Optimization

### **High-Performance Settings**
```bash
# Threading
export BEARDOG_WORKER_THREADS="8"
export BEARDOG_QUEUE_SIZE="2000"

# Caching
export BEARDOG_FEATURE_CACHING="true"
export BEARDOG_CACHE_SIZE="10000"

# Network
export BEARDOG_PERF_MAX_CONNECTIONS="1000"
export BEARDOG_KEEPALIVE_TIMEOUT="300"
```

### **Resource Limits**
```bash
export BEARDOG_MAX_MEMORY_MB="4096"  # 4GB
export BEARDOG_MAX_CPU_CORES="8"
export BEARDOG_REQUEST_TIMEOUT="30"
```

## 🔄 Migration from Legacy Configs

### **Automatic Migration Script**
```bash
# Run the migration helper
./scripts/migrate-config.sh configs/old-production-config.toml

# This will generate appropriate environment variables
# and show you what to set for your deployment
```

### **Manual Migration Steps**
1. **Identify your current config values**
2. **Map them to environment variables** using the reference above
3. **Test in development** with the new unified system
4. **Deploy to production** using environment-specific files

## 🎯 Deployment Scenarios

### **Scenario 1: Single Server**
```bash
# Simple single-server deployment
source configs/environments/production.env
export BEARDOG_STANDALONE_MODE="true"
export BEARDOG_FEATURE_DISTRIBUTED="false"
./beardog-server
```

### **Scenario 2: High Availability Cluster**
```bash
# Multi-node cluster deployment
source configs/environments/production.env
export BEARDOG_STANDALONE_MODE="false"
export BEARDOG_FEATURE_DISTRIBUTED="true"
export BEARDOG_CLUSTER_NODE_ID="node-1"
./beardog-server
```

### **Scenario 3: Edge Deployment**
```bash
# Edge/IoT deployment
source configs/environments/edge.env  # Create custom edge env
export BEARDOG_SECURITY_LEVEL="Standard"
export BEARDOG_HSM_ENABLED="false"
export BEARDOG_FEATURE_GENETIC="false"
./beardog-server
```

## 🔍 Troubleshooting

### **Common Issues**

#### **Configuration Not Loading**
```bash
# Verify environment variables are set
env | grep BEARDOG_

# Check config template path
ls -la $BEARDOG_CONFIG

# Validate template syntax
cargo run --bin config-validator
```

#### **HSM Connection Issues**
```bash
# Test HSM connectivity
export BEARDOG_HSM_TEST_MODE="true"
cargo run --bin hsm-test

# Check HSM library path
ls -la $BEARDOG_HSM_LIBRARY_PATH
```

#### **SSL Certificate Problems**
```bash
# Verify certificate files
openssl x509 -in $BEARDOG_CERT_PATH -text -noout
openssl rsa -in $BEARDOG_KEY_PATH -check
```

### **Debug Mode**
```bash
export BEARDOG_DEBUG="true"
export BEARDOG_LOG_LEVEL="DEBUG"
./beardog-server
```

## 📚 Additional Resources

- **Configuration Reference**: See `configs/beardog-config-template.toml` for all available options
- **Environment Examples**: Check `configs/environments/` for pre-configured scenarios  
- **Security Guide**: Refer to `docs/security/` for detailed security configurations
- **Performance Tuning**: See `docs/performance/` for optimization guidelines

## 🎉 Success!

You now have a **unified, scalable, and maintainable** BearDog deployment using the modern configuration system. The new approach eliminates configuration duplication and provides clear environment-driven deployment patterns.

---

**🏆 Deployment Status**: Production Ready with Unified Configuration  
**🚀 Architecture**: World-Class Rust Ecosystem  
**⚡ Performance**: Optimized for High-Scale Production Deployment 