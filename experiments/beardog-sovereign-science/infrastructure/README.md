# 🏗️ BearDog Sovereign Science - Infrastructure

**Purpose**: Production-ready monitoring and validation infrastructure  
**Status**: ✅ Ready for deployment  
**Last Updated**: October 9, 2025  

---

## 📋 Overview

This infrastructure provides comprehensive monitoring, alerting, and observability for the BearDog Sovereign Science validation framework.

### **Components**:

1. **Prometheus** - Metrics collection and storage
2. **Grafana** - Visualization and dashboards
3. **Alert Manager** - Alert routing and management
4. **Node Exporter** - System metrics

---

## 🚀 Quick Start

### **Prerequisites**:
- Docker (v20.10+)
- Docker Compose (v2.0+)
- 4GB RAM minimum
- 10GB disk space

### **Start the Stack**:

```bash
cd infrastructure/docker
docker-compose up -d
```

### **Verify Deployment**:

```bash
# Check all services are running
docker-compose ps

# View logs
docker-compose logs -f
```

### **Access Services**:

- **Prometheus**: http://localhost:9090
- **Grafana**: http://localhost:3000 (admin/beardog-admin)
- **Alert Manager**: http://localhost:9093
- **Node Exporter**: http://localhost:9100/metrics

---

## 📊 Service Details

### **Prometheus** (Port 9090)

**Purpose**: Metrics collection, storage, and querying

**Key Features**:
- 15-second scrape interval
- 30-day data retention
- Alert rule evaluation
- Service discovery

**Configuration**: `prometheus/prometheus.yml`

**Useful Queries**:
```promql
# Validation success rate
rate(beardog_validation_stages_total{success="true"}[5m])

# Cryptographic operation latency
histogram_quantile(0.95, rate(beardog_crypto_duration_ns_bucket[5m]))

# Human dignity check pass rate
rate(beardog_dignity_checks_total{passed="true"}[5m])
```

### **Grafana** (Port 3000)

**Purpose**: Metrics visualization and dashboards

**Default Credentials**:
- Username: `admin`
- Password: `beardog-admin`

**Key Features**:
- Pre-configured Prometheus datasource
- Auto-provisioned dashboards (when available)
- Custom query builder
- Alert visualization

**Dashboards** (coming soon):
- Validation Overview
- Cryptographic Operations
- Human Dignity Metrics
- System Performance
- Security Events

### **Alert Manager** (Port 9093)

**Purpose**: Alert routing, grouping, and silencing

**Key Features**:
- Intelligent alert grouping
- Severity-based routing
- Duplicate suppression
- Inhibition rules

**Alert Severities**:
- **Critical**: Immediate action required (1h repeat)
- **Warning**: Action needed soon (24h repeat)
- **Info**: Informational only (48h repeat)

**Configuration**: `alertmanager/alertmanager.yml`

### **Node Exporter** (Port 9100)

**Purpose**: Host system metrics

**Metrics Collected**:
- CPU usage and load
- Memory utilization
- Disk space and I/O
- Network statistics
- System uptime

---

## 🎯 Alert Rules

### **Validation Alerts**:

1. **HighValidationFailureRate**
   - Trigger: >0.1 failures/sec for 2 minutes
   - Severity: Critical

2. **CryptographicOperationFailures**
   - Trigger: Any crypto failures detected
   - Severity: Critical

3. **HumanDignityCheckFailures**
   - Trigger: Dignity checks failing
   - Severity: Warning

4. **LongRunningValidationStage**
   - Trigger: Stage running >300 seconds
   - Severity: Warning

### **System Alerts**:

1. **HighCPUUsage** - >80% for 5 minutes
2. **HighMemoryUsage** - >85% for 5 minutes
3. **LowDiskSpace** - >85% used
4. **PrometheusScrapeFailure** - Target down >2 minutes

### **Security Alerts**:

1. **CriticalSecurityEvent** - Any critical security event
2. **SecurityAnomalies** - Pattern detection (future)

---

## 🔧 Configuration

### **Prometheus Configuration**:

**File**: `prometheus/prometheus.yml`

**Key Settings**:
```yaml
global:
  scrape_interval: 15s      # How often to scrape targets
  evaluation_interval: 15s  # How often to evaluate rules
  
scrape_configs:
  - job_name: 'validation-framework'
    static_configs:
      - targets: ['validation-framework:9091']
```

### **Alert Rules**:

**File**: `prometheus/alerts.yml`

**Add Custom Rules**:
```yaml
- alert: MyCustomAlert
  expr: my_metric > threshold
  for: 5m
  labels:
    severity: warning
  annotations:
    summary: "Alert summary"
    description: "Alert description"
```

### **AlertManager Configuration**:

**File**: `alertmanager/alertmanager.yml`

**Configure Receivers**:
```yaml
receivers:
  - name: 'email-alerts'
    email_configs:
      - to: 'team@example.com'
        subject: '[Alert] {{ .GroupLabels.alertname }}'
```

### **Grafana Datasources**:

**File**: `grafana/datasources/prometheus.yml`

Pre-configured to connect to Prometheus automatically.

---

## 📈 Monitoring Best Practices

### **1. Dashboard Organization**:
- **Overview Dashboard**: High-level system health
- **Component Dashboards**: Detailed metrics per component
- **Troubleshooting Dashboards**: Deep-dive for issues

### **2. Alert Management**:
- Use inhibition rules to prevent alert storms
- Group related alerts together
- Set appropriate repeat intervals
- Configure multiple notification channels

### **3. Retention Policies**:
- Short-term: Prometheus (30 days)
- Long-term: External storage (optional)
- Archive validation results separately

### **4. Performance Tuning**:
- Adjust scrape intervals based on needs
- Use recording rules for expensive queries
- Monitor Prometheus resource usage
- Scale horizontally if needed

---

## 🛠️ Operations

### **Start Services**:
```bash
docker-compose up -d
```

### **Stop Services**:
```bash
docker-compose down
```

### **Stop and Remove Data**:
```bash
docker-compose down -v  # Removes volumes
```

### **View Logs**:
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f prometheus
docker-compose logs -f grafana
```

### **Restart Service**:
```bash
docker-compose restart prometheus
```

### **Update Configuration**:
```bash
# Edit configuration file, then:
docker-compose restart prometheus  # or other service
```

### **Reload Prometheus Config** (without restart):
```bash
curl -X POST http://localhost:9090/-/reload
```

---

## 🧪 Testing

### **Test Prometheus**:
```bash
# Check targets
curl http://localhost:9090/api/v1/targets

# Query metrics
curl 'http://localhost:9090/api/v1/query?query=up'
```

### **Test Grafana**:
```bash
# Check health
curl http://localhost:3000/api/health
```

### **Test AlertManager**:
```bash
# Check status
curl http://localhost:9093/api/v1/status

# Send test alert
curl -X POST http://localhost:9093/api/v1/alerts -d '[{
  "labels": {"alertname": "TestAlert", "severity": "info"},
  "annotations": {"summary": "Test alert"}
}]'
```

---

## 🔍 Troubleshooting

### **Services Won't Start**:

**Check logs**:
```bash
docker-compose logs [service-name]
```

**Common issues**:
- Ports already in use (check with `netstat -tulpn`)
- Insufficient resources (increase Docker limits)
- Configuration errors (validate YAML syntax)

### **Prometheus Not Scraping**:

**Check targets**:
Visit http://localhost:9090/targets

**Verify connectivity**:
```bash
docker-compose exec prometheus wget -O- http://node-exporter:9100/metrics
```

### **Grafana Can't Connect to Prometheus**:

**Check datasource**:
Grafana → Configuration → Data Sources → Prometheus

**Verify network**:
```bash
docker-compose exec grafana ping prometheus
```

### **Alerts Not Firing**:

**Check alert rules**:
Visit http://localhost:9090/alerts

**Verify AlertManager**:
Visit http://localhost:9093

---

## 📦 Data Persistence

### **Volumes**:
- `prometheus-data`: Prometheus time-series data
- `grafana-data`: Grafana dashboards and settings
- `alertmanager-data`: AlertManager state

### **Backup**:
```bash
# Backup volumes
docker run --rm -v beardog-prometheus-data:/data -v $(pwd):/backup \
  alpine tar czf /backup/prometheus-backup.tar.gz /data

# Restore volumes
docker run --rm -v beardog-prometheus-data:/data -v $(pwd):/backup \
  alpine tar xzf /backup/prometheus-backup.tar.gz
```

---

## 🚀 Integration with Validation Framework

### **When BearDog Integration is Complete**:

1. **Uncomment validation service** in `docker-compose.yml`
2. **Add validation scrape config** in `prometheus/prometheus.yml`
3. **Deploy validation framework**:
   ```bash
   docker-compose up -d validation-framework
   ```

4. **Verify metrics**:
   Visit http://localhost:9091/metrics

---

## 📊 Metrics Reference

### **Validation Metrics**:
- `beardog_validation_active` - Active validation runs
- `beardog_validation_runs_total` - Total validation runs
- `beardog_validation_stages_total` - Stage completions
- `beardog_validation_failures_total` - Validation failures
- `beardog_stage_duration_seconds` - Stage durations

### **Cryptographic Metrics**:
- `beardog_crypto_operations_total` - Crypto operations count
- `beardog_crypto_duration_ns` - Operation latencies
- `beardog_crypto_failures_total` - Crypto failures

### **Human Dignity Metrics**:
- `beardog_dignity_checks_total` - Dignity check count
- `beardog_dignity_score` - Dignity check scores
- `beardog_dignity_failures_total` - Failed dignity checks

### **Performance Metrics**:
- `beardog_performance_metrics` - Performance measurements
- `beardog_test_duration_ms` - Test execution times

---

## ✅ Health Checks

### **System Health**:
```bash
# All services up
docker-compose ps | grep "Up"

# Prometheus healthy
curl http://localhost:9090/-/healthy

# Grafana healthy
curl http://localhost:3000/api/health

# AlertManager healthy
curl http://localhost:9093/-/healthy
```

---

## 📚 Resources

### **Documentation**:
- [Prometheus Documentation](https://prometheus.io/docs/)
- [Grafana Documentation](https://grafana.com/docs/)
- [AlertManager Documentation](https://prometheus.io/docs/alerting/latest/alertmanager/)

### **Query Language**:
- [PromQL Guide](https://prometheus.io/docs/prometheus/latest/querying/basics/)
- [PromQL Examples](https://prometheus.io/docs/prometheus/latest/querying/examples/)

---

## 🎯 Status

**Infrastructure**: ✅ Ready for deployment  
**Configuration**: ✅ Production-ready  
**Documentation**: ✅ Complete  
**Integration**: ⏳ Pending (Week 2)  

**Next Steps**:
1. Deploy stack: `docker-compose up -d`
2. Verify all services running
3. Access Grafana and explore
4. Wait for validation framework integration (Week 2)

---

**Created**: October 9, 2025  
**Version**: 1.0.0  
**Status**: Production-ready  

🌍🔐 **Infrastructure humans can observe and trust!** 📊

