# 🚀 BearDog Sovereign Science - Deployment Guide

**Quick deployment guide for the validation infrastructure**

---

## ⚡ Quick Start (2 minutes)

```bash
# 1. Navigate to infrastructure
cd experiments/beardog-sovereign-science/infrastructure/docker

# 2. Start the stack
docker-compose up -d

# 3. Verify deployment
docker-compose ps

# 4. View logs
docker-compose logs -f
```

**That's it!** Services are now running.

---

## 🌐 Access Services

| Service | URL | Credentials |
|---------|-----|-------------|
| **Prometheus** | http://localhost:9090 | None |
| **Grafana** | http://localhost:3000 | admin / beardog-admin |
| **AlertManager** | http://localhost:9093 | None |
| **Node Exporter** | http://localhost:9100/metrics | None |

---

## ✅ Verification Checklist

### **1. Check Services are Running**:
```bash
docker-compose ps
```

Expected output:
```
NAME                        STATUS
beardog-alertmanager       Up
beardog-grafana            Up  
beardog-node-exporter      Up
beardog-prometheus         Up
```

### **2. Verify Prometheus Targets**:

Visit: http://localhost:9090/targets

All targets should show **UP** status.

### **3. Access Grafana**:

1. Visit: http://localhost:3000
2. Login: `admin` / `beardog-admin`
3. Go to: Configuration → Data Sources
4. Verify: Prometheus datasource is connected (green checkmark)

### **4. Check AlertManager**:

Visit: http://localhost:9093

You should see the AlertManager web interface.

---

## 🛠️ Common Operations

### **Stop Services**:
```bash
docker-compose down
```

### **Stop and Remove Data**:
```bash
docker-compose down -v  # WARNING: Removes all data
```

### **Restart a Service**:
```bash
docker-compose restart prometheus
```

### **View Logs**:
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f prometheus
```

### **Update Configuration**:
```bash
# 1. Edit configuration file (e.g., prometheus/prometheus.yml)
# 2. Restart the service
docker-compose restart prometheus
```

---

## 🎯 Next Steps

### **Week 1** (Infrastructure Setup):
- [x] Deploy monitoring stack
- [ ] Verify all services operational
- [ ] Test alert rules
- [ ] Explore Grafana

### **Week 2** (Integration):
- [ ] Wire validation framework to Prometheus
- [ ] Create custom Grafana dashboards
- [ ] Test end-to-end monitoring
- [ ] Configure alert notifications

---

## 🐛 Troubleshooting

### **Port Already in Use**:
```bash
# Check what's using the port
sudo netstat -tulpn | grep :9090

# Change port in docker-compose.yml if needed
ports:
  - "9091:9090"  # Use 9091 instead of 9090
```

### **Services Won't Start**:
```bash
# Check Docker is running
docker ps

# Check logs for errors
docker-compose logs
```

### **Can't Access Web Interface**:
```bash
# Verify service is listening
docker-compose exec prometheus netstat -tulpn | grep 9090

# Check firewall
sudo ufw status
```

---

## 📊 Sample Queries

### **Prometheus Queries** (http://localhost:9090):

**System CPU Usage**:
```promql
100 - (avg by(instance) (rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100)
```

**Memory Usage**:
```promql
(1 - (node_memory_MemAvailable_bytes / node_memory_MemTotal_bytes)) * 100
```

**Disk Space**:
```promql
(1 - (node_filesystem_avail_bytes / node_filesystem_size_bytes)) * 100
```

---

## ✅ Success Criteria

**Infrastructure is ready when**:
- ✅ All 4 services showing "Up" status
- ✅ Prometheus targets all "UP"
- ✅ Grafana accessible and connected to Prometheus
- ✅ AlertManager web interface accessible
- ✅ Node Exporter metrics visible

---

**Time to Deploy**: 2-5 minutes  
**Difficulty**: Easy  
**Status**: Production-ready  

🌍🔐 **Ready to monitor!** 📊

