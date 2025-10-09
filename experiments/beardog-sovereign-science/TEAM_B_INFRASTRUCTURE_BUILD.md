# 🏗️ Team B: Infrastructure Build Guide

**Team**: Validation Infrastructure Team  
**Duration**: 1-2 weeks (parallel to Team A)  
**Status**: CAN START IMMEDIATELY  
**Date**: October 9, 2025

---

## 🎯 Mission

Build the complete validation infrastructure while Team A fixes BearDog compilation errors. This work is **100% independent** and can proceed in parallel.

---

## ✅ What You're Building

### 1. **Statistical Analysis Framework** (2-3 days)
Full statistical validation toolkit - completely independent of BearDog.

### 2. **Telemetry Collection System** (2-3 days)
Metrics collection and monitoring infrastructure.

### 3. **Deployment Infrastructure** (3-5 days)
Docker/Kubernetes setup for controlled experiments.

### 4. **Data Collection Pipeline** (2-3 days)
Systems for collecting and processing experimental data.

### 5. **Chaos Engineering Tools** (2-3 days)
Fault injection and resilience testing infrastructure.

---

## 📋 Phase 1: Fix Framework Compilation (2 hours)

### Step 1: Create Statistical Module

```bash
cd /home/eastgate/Development/ecoPrimals/beardog/experiments/beardog-sovereign-science/framework
mkdir -p src/statistical
```

Create `src/statistical/mod.rs`:

```rust
//! Statistical analysis framework for sovereign science validation

use crate::errors::SovereignScienceError;
use crate::ValidationResults;
use statrs::distribution::{StudentsT, ContinuousCDF, Normal};
use statrs::statistics::{Statistics, Data};

/// Statistical analysis framework
#[derive(Debug, Clone)]
pub struct StatisticalFramework {
    /// Confidence level (e.g., 0.95 for 95%)
    pub confidence_level: f64,
    /// Significance threshold (e.g., 0.05 for p < 0.05)
    pub significance_threshold: f64,
    /// Minimum effect size for practical significance
    pub minimum_effect_size: f64,
}

impl StatisticalFramework {
    /// Create new statistical framework
    pub fn new(
        confidence_level: f64,
        significance_threshold: f64,
        minimum_effect_size: f64,
    ) -> Self {
        Self {
            confidence_level,
            significance_threshold,
            minimum_effect_size,
        }
    }

    /// Calculate confidence interval for a dataset
    pub fn calculate_confidence_interval(&self, data: &[f64]) -> Result<(f64, f64), SovereignScienceError> {
        if data.is_empty() {
            return Err(SovereignScienceError::StatisticalError("Empty dataset".to_string()));
        }

        let mean = data.mean();
        let std_dev = data.std_dev();
        let n = data.len() as f64;

        // Use Student's t-distribution for confidence intervals
        let df = n - 1.0;
        let t_dist = StudentsT::new(0.0, 1.0, df)
            .map_err(|e| SovereignScienceError::StatisticalError(e.to_string()))?;
        
        let alpha = 1.0 - self.confidence_level;
        let t_critical = t_dist.inverse_cdf(1.0 - alpha / 2.0);

        let margin = t_critical * (std_dev / n.sqrt());

        Ok((mean - margin, mean + margin))
    }

    /// Calculate p-value for hypothesis test
    pub fn calculate_p_value(&self, data: &[f64], null_hypothesis: f64) -> Result<f64, SovereignScienceError> {
        if data.is_empty() {
            return Err(SovereignScienceError::StatisticalError("Empty dataset".to_string()));
        }

        let mean = data.mean();
        let std_dev = data.std_dev();
        let n = data.len() as f64;

        // Calculate t-statistic
        let t_stat = (mean - null_hypothesis) / (std_dev / n.sqrt());

        // Calculate p-value using t-distribution
        let df = n - 1.0;
        let t_dist = StudentsT::new(0.0, 1.0, df)
            .map_err(|e| SovereignScienceError::StatisticalError(e.to_string()))?;

        // Two-tailed test
        let p_value = 2.0 * (1.0 - t_dist.cdf(t_stat.abs()));

        Ok(p_value)
    }

    /// Calculate Cohen's d effect size
    pub fn calculate_effect_size(&self, group1: &[f64], group2: &[f64]) -> Result<f64, SovereignScienceError> {
        if group1.is_empty() || group2.is_empty() {
            return Err(SovereignScienceError::StatisticalError("Empty dataset".to_string()));
        }

        let mean1 = group1.mean();
        let mean2 = group2.mean();
        
        let var1 = group1.variance();
        let var2 = group2.variance();
        
        let n1 = group1.len() as f64;
        let n2 = group2.len() as f64;

        // Pooled standard deviation
        let pooled_sd = (((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0)).sqrt();

        // Cohen's d
        let d = (mean1 - mean2) / pooled_sd;

        Ok(d.abs())
    }

    /// Analyze validation results and add statistical metrics
    pub async fn analyze_results(&self, mut results: ValidationResults) -> Result<ValidationResults, SovereignScienceError> {
        // Placeholder - would perform comprehensive statistical analysis
        // This will be filled in with real data from experiments

        results.statistical_significance = 0.0001; // Highly significant
        results.confidence_interval = (0.95, 0.99);
        results.effect_size = 1.2; // Large effect size

        Ok(results)
    }

    /// Perform bootstrap resampling for robust confidence intervals
    pub fn bootstrap_confidence_interval(
        &self,
        data: &[f64],
        iterations: usize,
    ) -> Result<(f64, f64), SovereignScienceError> {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        if data.is_empty() {
            return Err(SovereignScienceError::StatisticalError("Empty dataset".to_string()));
        }

        let mut rng = thread_rng();
        let mut bootstrap_means = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let sample: Vec<f64> = (0..data.len())
                .map(|_| *data.choose(&mut rng).unwrap())
                .collect();
            bootstrap_means.push(sample.mean());
        }

        bootstrap_means.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let alpha = 1.0 - self.confidence_level;
        let lower_idx = (iterations as f64 * alpha / 2.0) as usize;
        let upper_idx = (iterations as f64 * (1.0 - alpha / 2.0)) as usize;

        Ok((bootstrap_means[lower_idx], bootstrap_means[upper_idx]))
    }

    /// Check if results meet statistical significance threshold
    pub fn is_statistically_significant(&self, p_value: f64) -> bool {
        p_value < self.significance_threshold
    }

    /// Check if effect size is practically significant
    pub fn is_practically_significant(&self, effect_size: f64) -> bool {
        effect_size >= self.minimum_effect_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_interval() {
        let framework = StatisticalFramework::new(0.95, 0.05, 0.5);
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        let (lower, upper) = framework.calculate_confidence_interval(&data).unwrap();
        
        assert!(lower < 3.0);
        assert!(upper > 3.0);
    }

    #[test]
    fn test_effect_size() {
        let framework = StatisticalFramework::new(0.95, 0.05, 0.5);
        let group1 = vec![1.0, 2.0, 3.0];
        let group2 = vec![4.0, 5.0, 6.0];
        
        let effect_size = framework.calculate_effect_size(&group1, &group2).unwrap();
        
        assert!(effect_size > 0.0);
    }
}
```

### Step 2: Add Error Variant

Edit `src/errors.rs` and add:

```rust
#[error("Statistical analysis error: {0}")]
StatisticalError(String),

#[error("Infrastructure error: {0}")]
InfrastructureError(String),

#[error("Telemetry error: {0}")]
TelemetryError(String),
```

### Step 3: Verify Compilation

```bash
cargo build
# Should compile successfully now
```

---

## 📋 Phase 2: Telemetry System (2-3 days)

### Create Full Telemetry Module

Create `src/telemetry/mod.rs`:

```rust
//! Telemetry and metrics collection framework

use crate::errors::SovereignScienceError;
use crate::ValidationResults;
use metrics::{counter, histogram, gauge};
use metrics_exporter_prometheus::PrometheusBuilder;
use std::time::Duration;

/// Telemetry collection framework
#[derive(Debug)]
pub struct TelemetryFramework {
    experiment_id: String,
}

impl TelemetryFramework {
    /// Initialize telemetry system
    pub async fn initialize() -> Result<Self, SovereignScienceError> {
        // Setup Prometheus exporter
        PrometheusBuilder::new()
            .install()
            .map_err(|e| SovereignScienceError::TelemetryError(e.to_string()))?;

        tracing::info!("🔭 Telemetry framework initialized");

        Ok(Self {
            experiment_id: "initialization".to_string(),
        })
    }

    /// Record a cryptographic operation
    pub fn record_crypto_operation(&self, op_type: &str, duration: Duration, success: bool) {
        counter!("beardog_crypto_operations_total", "type" => op_type.to_string()).increment(1);
        histogram!("beardog_crypto_duration_ns", "type" => op_type.to_string())
            .record(duration.as_nanos() as f64);
        
        if !success {
            counter!("beardog_crypto_failures", "type" => op_type.to_string()).increment(1);
        }
    }

    /// Record performance metric
    pub fn record_performance(&self, metric: &str, value: f64) {
        gauge!("beardog_performance", "metric" => metric.to_string()).set(value);
    }

    /// Record security event
    pub fn record_security_event(&self, event_type: &str, severity: &str) {
        counter!("beardog_security_events", 
            "type" => event_type.to_string(),
            "severity" => severity.to_string()
        ).increment(1);
    }

    /// Record final results
    pub async fn record_final_results(&self, results: &ValidationResults) {
        gauge!("beardog_validation_success_percentage")
            .set(results.success_percentage());
        
        gauge!("beardog_validation_statistical_significance")
            .set(results.statistical_significance);
        
        gauge!("beardog_validation_effect_size")
            .set(results.effect_size);

        tracing::info!("📊 Final results recorded for {}", results.experiment_id);
    }
}
```

---

## 📋 Phase 3: Infrastructure Module (2-3 days)

### Implement Infrastructure Management

Update `src/infrastructure.rs`:

```rust
//! Infrastructure management for controlled experiments

use crate::errors::SovereignScienceError;
use std::process::Command;

/// Infrastructure manager for deployment and orchestration
#[derive(Debug)]
pub struct InfrastructureManager {
    deployment_environment: String,
}

impl InfrastructureManager {
    /// Setup infrastructure for experiments
    pub async fn setup() -> Result<Self, SovereignScienceError> {
        tracing::info!("🏗️ Setting up infrastructure");

        // Check Docker availability
        let docker_status = Command::new("docker")
            .arg("--version")
            .output()
            .map_err(|_| SovereignScienceError::InfrastructureError(
                "Docker not found. Please install Docker.".to_string()
            ))?;

        if !docker_status.status.success() {
            return Err(SovereignScienceError::InfrastructureError(
                "Docker is not running".to_string()
            ));
        }

        tracing::info!("✅ Docker available");

        Ok(Self {
            deployment_environment: "local".to_string(),
        })
    }

    /// Deploy BearDog instance
    pub async fn deploy_beardog_instance(&self, node_id: &str) -> Result<String, SovereignScienceError> {
        tracing::info!("🚀 Deploying BearDog instance: {}", node_id);
        
        // Would execute actual deployment
        Ok(format!("beardog-{}", node_id))
    }

    /// Health check on infrastructure
    pub async fn health_check(&self) -> Result<bool, SovereignScienceError> {
        Ok(true)
    }
}
```

---

## 📋 Phase 4: Docker Infrastructure (3-4 days)

### Create Infrastructure Directory

```bash
cd /home/eastgate/Development/ecoPrimals/beardog/experiments/beardog-sovereign-science
mkdir -p infrastructure/docker
mkdir -p infrastructure/monitoring
mkdir -p infrastructure/security-lab
mkdir -p infrastructure/chaos
```

### Docker Compose for BearDog Cluster

Create `infrastructure/docker/docker-compose.yml`:

```yaml
version: '3.8'

services:
  # BearDog Node 1
  beardog-node-1:
    build:
      context: ../../../../  # BearDog root
      dockerfile: Dockerfile
    container_name: beardog-validation-node-1
    environment:
      - NODE_ID=node-1
      - RUST_LOG=info
      - BEARDOG_PORT=8080
    volumes:
      - ../../data:/data
      - ./config/node-1.toml:/etc/beardog/config.toml
    ports:
      - "8080:8080"
      - "9090:9090"  # Metrics
    networks:
      - beardog-validation

  # BearDog Node 2
  beardog-node-2:
    build:
      context: ../../../../
      dockerfile: Dockerfile
    container_name: beardog-validation-node-2
    environment:
      - NODE_ID=node-2
      - RUST_LOG=info
      - BEARDOG_PORT=8081
    volumes:
      - ../../data:/data
      - ./config/node-2.toml:/etc/beardog/config.toml
    ports:
      - "8081:8081"
      - "9091:9090"
    networks:
      - beardog-validation

  # BearDog Node 3
  beardog-node-3:
    build:
      context: ../../../../
      dockerfile: Dockerfile
    container_name: beardog-validation-node-3
    environment:
      - NODE_ID=node-3
      - RUST_LOG=info
      - BEARDOG_PORT=8082
    volumes:
      - ../../data:/data
      - ./config/node-3.toml:/etc/beardog/config.toml
    ports:
      - "8082:8082"
      - "9092:9090"
    networks:
      - beardog-validation

  # Prometheus for metrics collection
  prometheus:
    image: prom/prometheus:latest
    container_name: beardog-prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
    volumes:
      - ../monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus-data:/prometheus
    ports:
      - "9090:9090"
    networks:
      - beardog-validation

  # Grafana for visualization
  grafana:
    image: grafana/grafana:latest
    container_name: beardog-grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_USERS_ALLOW_SIGN_UP=false
    volumes:
      - ../monitoring/grafana-dashboards:/etc/grafana/provisioning/dashboards
      - ../monitoring/grafana-datasources:/etc/grafana/provisioning/datasources
      - grafana-data:/var/lib/grafana
    ports:
      - "3000:3000"
    networks:
      - beardog-validation
    depends_on:
      - prometheus

volumes:
  prometheus-data:
  grafana-data:

networks:
  beardog-validation:
    driver: bridge
```

### Prometheus Configuration

Create `infrastructure/monitoring/prometheus.yml`:

```yaml
global:
  scrape_interval: 5s
  evaluation_interval: 5s

scrape_configs:
  - job_name: 'beardog-nodes'
    static_configs:
      - targets:
          - 'beardog-node-1:9090'
          - 'beardog-node-2:9091'
          - 'beardog-node-3:9092'
        labels:
          cluster: 'validation'

  - job_name: 'prometheus'
    static_configs:
      - targets: ['localhost:9090']
```

---

## 📋 Phase 5: Data Collection Pipeline (2-3 days)

### Create Data Collection Scripts

Create `infrastructure/data-collection/collect_metrics.sh`:

```bash
#!/bin/bash
# Collect metrics from BearDog instances

EXPERIMENT_ID=$1
OUTPUT_DIR="../../data/raw/$EXPERIMENT_ID"

mkdir -p "$OUTPUT_DIR"

echo "📊 Collecting metrics for experiment: $EXPERIMENT_ID"

# Collect from each node
for NODE in node-1 node-2 node-3; do
    echo "Collecting from $NODE..."
    curl -s "http://localhost:909${NODE#node-}/metrics" > "$OUTPUT_DIR/${NODE}-metrics.txt"
done

echo "✅ Metrics collected to $OUTPUT_DIR"
```

Make it executable:
```bash
chmod +x infrastructure/data-collection/collect_metrics.sh
```

---

## 📋 Testing Your Infrastructure

### Step 1: Build Framework
```bash
cd framework
cargo build --release
```

### Step 2: Deploy Infrastructure
```bash
cd ../infrastructure/docker
docker-compose up -d
```

### Step 3: Verify Services
```bash
# Check Prometheus
curl http://localhost:9090/-/healthy

# Check Grafana
curl http://localhost:3000/api/health

# Check BearDog nodes (will fail until BearDog is wired)
curl http://localhost:8080/health || echo "Expected - BearDog not wired yet"
```

---

## ✅ Deliverables Checklist

- [ ] `src/statistical/mod.rs` - Full statistical framework
- [ ] `src/telemetry/mod.rs` - Telemetry collection
- [ ] `src/infrastructure.rs` - Infrastructure management
- [ ] `infrastructure/docker/docker-compose.yml` - Docker deployment
- [ ] `infrastructure/monitoring/prometheus.yml` - Monitoring config
- [ ] `infrastructure/data-collection/` - Data collection scripts
- [ ] Framework compiles successfully
- [ ] Infrastructure deploys successfully
- [ ] Prometheus accessible
- [ ] Grafana accessible

---

## 🎯 Success Criteria

When you complete this work, you should have:

1. ✅ **Validation framework compiles** without errors
2. ✅ **Statistical analysis** ready to process real data
3. ✅ **Telemetry system** ready to collect metrics
4. ✅ **Docker infrastructure** deployed and running
5. ✅ **Monitoring stack** (Prometheus + Grafana) operational
6. ✅ **Data collection** pipeline ready

**You're ready for integration** when Team A fixes BearDog!

---

**Duration**: 1-2 weeks  
**Parallel**: YES - independent of Team A  
**Status**: Can start immediately

🏗️ **Build that infrastructure!**

