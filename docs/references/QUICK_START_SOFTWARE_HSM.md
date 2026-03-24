# 🚀 Quick Start: BearDog with Software HSM

**For biomeOS Team** - Copy/Paste Solution

---

## ✅ The Problem
```
Error: No HSM providers available
```

## ✅ The Solution (5 Minutes)

### **Option 1: Command Line (Quickest)**

```bash
# Set environment variable
export BEARDOG_HSM_MODE=software

# Start BearDog (it will auto-initialize software HSM)
./beardog server

# Or with cargo
BEARDOG_HSM_MODE=software cargo run --release --bin beardog -- server
```

**Done!** BearDog will auto-detect the env var and initialize software HSM.

---

### **Option 2: Systemd Service (For Production)**

**File**: `/etc/systemd/system/beardog.service`

```ini
[Unit]
Description=BearDog Sovereign Security Service
After=network.target

[Service]
Type=simple
User=beardog
Group=beardog
WorkingDirectory=/opt/beardog

# Software HSM mode (no hardware required)
Environment="BEARDOG_HSM_MODE=software"
Environment="BEARDOG_API_BIND_ADDR=0.0.0.0:9000"

ExecStart=/usr/local/bin/beardog server
Restart=on-failure

# Security hardening
NoNewPrivileges=true
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

**Enable and start**:
```bash
sudo systemctl daemon-reload
sudo systemctl enable beardog
sudo systemctl start beardog
sudo systemctl status beardog
```

---

### **Option 3: Docker/Container (For biomeOS)**

**Dockerfile**:
```dockerfile
FROM rust:1.85 AS builder
WORKDIR /build
COPY . .
RUN cargo build --release --bin beardog

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/beardog /usr/local/bin/

# Software HSM by default (no hardware required)
ENV BEARDOG_HSM_MODE=software
ENV BEARDOG_API_BIND_ADDR=0.0.0.0:9000

EXPOSE 9000
CMD ["beardog", "server"]
```

**Build and run**:
```bash
docker build -t beardog:latest .
docker run -p 9000:9000 -e BEARDOG_HSM_MODE=software beardog:latest
```

---

### **Option 4: Kubernetes (For biomeOS Production)**

**beardog-deployment.yaml**:
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: beardog
  labels:
    app: beardog
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
        - containerPort: 9000
          name: api
        env:
        - name: BEARDOG_HSM_MODE
          value: "software"
        - name: BEARDOG_API_BIND_ADDR
          value: "0.0.0.0:9000"
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 9000
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 9000
          initialDelaySeconds: 5
          periodSeconds: 10
---
apiVersion: v1
kind: Service
metadata:
  name: beardog
spec:
  selector:
    app: beardog
  ports:
  - protocol: TCP
    port: 9000
    targetPort: 9000
  type: ClusterIP
```

**Deploy**:
```bash
kubectl apply -f beardog-deployment.yaml
kubectl get pods -l app=beardog
kubectl logs -l app=beardog
```

---

## 🧪 Test It Works

```bash
# Health check
curl http://localhost:9000/api/v1/health
# Expected: {"status":"healthy","version":"0.9.0","capabilities":["btsp","genesis","birdsong","lineage"]}

# Create lineage
curl -X POST http://localhost:9000/api/v1/lineage/create \
  -H "Content-Type: application/json" \
  -d '{"service_type": "tower", "metadata": null}'

# Expected: {"success":true,"data":{"lineage_id":"lineage:tower:...","created_at":...}}
```

---

## 📦 Code Implementation (If You Need It)

**The auto-initialization code is already in BearDog!**

**Location**: `crates/beardog-tunnel/src/api/server.rs`

**How it works**:
```rust
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use beardog_tunnel::tunnel::hsm::software_hsm::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::{HsmTier, SoftwareHsmConfig};

// Check environment variable
let hsm_mode = std::env::var("BEARDOG_HSM_MODE")
    .unwrap_or_else(|_| "software".to_string());

// Initialize HSM manager
let mut hsm = HsmManager::new();

if hsm_mode == "software" {
    // Auto-register software HSM
    let config = SoftwareHsmConfig::default();
    let software_hsm = RustSoftwareHsm::new(config).await?;
    hsm.register_hsm_provider(HsmTier::Software, Arc::new(software_hsm))?;
    info!("✅ Software HSM initialized");
}

let hsm = Arc::new(hsm);

// Continue with server initialization...
```

**This is already implemented in the BTSP provider creation flow!**

---

## 🎯 For biomeOS Packaging

### **Minimal Package Requirements**:

1. **Binary**: `beardog` (run `beardog server` for the API service)
2. **Environment Variable**: `BEARDOG_HSM_MODE=software`
3. **Port**: `9000` (or configure with `BEARDOG_API_BIND_ADDR`)

**That's it!** No hardware, no configuration files, no manual initialization.

### **biomeOS Script Example**:

```bash
#!/bin/bash
# start-beardog.sh

set -e

# Set software HSM mode
export BEARDOG_HSM_MODE=software
export BEARDOG_API_BIND_ADDR=0.0.0.0:9000

# Optional: Set log level
export RUST_LOG=info

# Start BearDog
exec /usr/local/bin/beardog server
```

---

## 🔧 Troubleshooting

### **Still seeing HSM error?**

**Check**:
```bash
echo $BEARDOG_HSM_MODE  # Should print "software"
```

**Force it**:
```bash
BEARDOG_HSM_MODE=software ./beardog server
```

### **Permission denied?**

```bash
sudo chown -R beardog:beardog /opt/beardog
sudo chmod +x /usr/local/bin/beardog
```

### **Port already in use?**

```bash
# Use different port
BEARDOG_HSM_MODE=software BEARDOG_API_BIND_ADDR=0.0.0.0:9001 ./beardog server
```

---

## 📊 Entropy Sources (Current vs Future)

| Phase | Entropy Source | Trust | Requirement |
|-------|----------------|-------|-------------|
| **Now** | `/dev/urandom` | ⭐⭐⭐ | None (auto) |
| **Next** | SoloKey/YubiKey | ⭐⭐⭐⭐ | USB device |
| **Future** | Android StrongBox | ⭐⭐⭐⭐⭐ | Android 8.1+ |

**For basic biomeOS deployment**: `/dev/urandom` (software HSM) is production-ready!

---

## ✅ Summary

### **Problem**: HSM not initialized
### **Solution**: `export BEARDOG_HSM_MODE=software`
### **Result**: BearDog starts immediately, no hardware required

### **For biomeOS**:
1. Add `BEARDOG_HSM_MODE=software` to deployment
2. Start the BearDog server (`beardog server`)
3. That's it!

**Later**, when you want hardware HSM:
1. Plug in SoloKey/YubiKey
2. Change to `BEARDOG_HSM_MODE=hardware`
3. BearDog will auto-detect and use it

---

🐻 **Software HSM: Zero Config. Zero Hardware. Production Ready.** 🐻

**Any questions? We're here to help!**

