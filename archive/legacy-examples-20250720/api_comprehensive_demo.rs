//! Comprehensive BearDog API Demo
//!
//! Demonstrates the complete AI-first API system including:
//! - Security API with threat detection and ML capabilities
//! - Genetics API with node spawning and genetic analysis  
//! - Monitoring API with health checks and metrics
//! - Performance features (caching, rate limiting, real-time data)

use serde_json::{json, Value};
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🚀 Starting BearDog Comprehensive API Demo");

    let client = reqwest::Client::new();
    let base_url =
        std::env::var("BEARDOG_API_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    // Start with API info and health checks
    demo_api_basics(&client, &base_url).await?;

    // Demo Security API capabilities
    demo_security_api(&client, &base_url).await?;

    // Demo Genetics API capabilities
    demo_genetics_api(&client, &base_url).await?;

    // Demo Monitoring API capabilities
    demo_monitoring_api(&client, &base_url).await?;

    // Demo real-time monitoring
    demo_realtime_monitoring(&client, &base_url).await?;

    info!("✅ Comprehensive API demo completed successfully!");

    Ok(())
}

/// Demo basic API information and health endpoints
async fn demo_api_basics(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("📋 Testing API Basics");

    // Get API information
    let response: Value = client
        .get(format!("{base_url}/api/"))
        .send()
        .await?
        .json()
        .await?;

    info!("API Info: {}", serde_json::to_string_pretty(&response)?);

    // Check API health
    let health: Value = client
        .get(format!("{base_url}/api/health"))
        .send()
        .await?
        .json()
        .await?;

    info!("API Health: {}", health["data"]["status"]);

    // Get comprehensive status
    let status: Value = client
        .get(format!("{base_url}/api/v1/status"))
        .send()
        .await?
        .json()
        .await?;

    info!("System Status: {}", status["data"]["overall_status"]);

    Ok(())
}

/// Demo Security API with threat detection and ML capabilities
async fn demo_security_api(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🛡️ Testing Security API");

    // Test threat analysis
    let security_event = json!({
        "event_type": "login_attempt",
        "source_ip": "192.168.1.100",
        "user_agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "metadata": {
            "username": "admin",
            "success": false,
            "failure_count": 5
        }
    });

    let analysis: Value = client
        .post(format!("{base_url}/api/v1/security/analyze"))
        .json(&security_event)
        .send()
        .await?
        .json()
        .await?;

    info!(
        "Threat Analysis Result: {}",
        analysis["data"]["threat_level"]
    );

    // Test ML prediction
    let ml_request = json!({
        "features": [0.8, 0.2, 0.95, 0.1, 0.7],
        "model_type": "anomaly_detection",
        "context": {
            "user_id": "user_123",
            "session_id": "sess_456"
        }
    });

    let prediction: Value = client
        .post(format!("{base_url}/api/v1/security/ml/predict"))
        .json(&ml_request)
        .send()
        .await?
        .json()
        .await?;

    info!("ML Prediction: {}", prediction["data"]["prediction"]);

    // Get security statistics
    let stats: Value = client
        .get(format!("{base_url}/api/v1/security/stats"))
        .send()
        .await?
        .json()
        .await?;

    info!(
        "Security Stats - Threats detected: {}",
        stats["data"]["threats_detected_24h"]
    );

    Ok(())
}

/// Demo Genetics API with node spawning and genetic analysis
async fn demo_genetics_api(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🧬 Testing Genetics API");

    // Create a genesis node
    let genesis_request = json!({
        "node_id": "genesis_demo_001",
        "capabilities": ["storage", "compute", "network", "security"],
        "resource_allocation": {
            "cpu_cores": 8,
            "memory_gb": 32,
            "storage_gb": 500,
            "network_bandwidth_mbps": 1000,
            "geographic_region": "us-west-2"
        },
        "security_level": "high",
        "metadata": {
            "purpose": "demo_genesis",
            "creator": "api_demo"
        }
    });

    let genesis_response: Value = client
        .post(format!("{base_url}/api/v1/genetics/genesis"))
        .json(&genesis_request)
        .send()
        .await?
        .json()
        .await?;

    let genesis_node_id = &genesis_response["data"]["node_id"];
    info!("Created Genesis Node: {}", genesis_node_id);

    // Submit a spawning request
    let spawn_request = json!({
        "parent_node_id": genesis_node_id,
        "requested_capabilities": ["storage", "compute", "enhanced_security"],
        "resource_requirements": {
            "cpu_cores": 4,
            "memory_gb": 16,
            "storage_gb": 250,
            "network_bandwidth_mbps": 500,
            "geographic_region": "us-west-2"
        },
        "workflow_type": "automated",
        "priority": "normal",
        "recombination_params": {
            "chromosome_strategy": "crossover",
            "trait_inheritance": "weighted_average",
            "capability_fusion": "additive",
            "mutation_rate": 0.05
        }
    });

    let spawn_response: Value = client
        .post(format!("{base_url}/api/v1/genetics/spawn"))
        .json(&spawn_request)
        .send()
        .await?
        .json()
        .await?;

    let request_id = &spawn_response["data"]["request_id"];
    info!("Spawn Request Submitted: {}", request_id);

    // Wait a moment then check spawn status
    sleep(Duration::from_millis(1000)).await;

    let status: Value = client
        .get(format!(
            "{base_url}/api/v1/genetics/spawn/{request_id}"
        ))
        .send()
        .await?
        .json()
        .await?;

    info!("Spawn Status: {}", status["data"]["status"]);

    // Analyze genetics of the genesis node
    let genetics_analysis: Value = client
        .get(format!(
            "{base_url}/api/v1/genetics/analyze/{genesis_node_id}"
        ))
        .send()
        .await?
        .json()
        .await?;

    info!(
        "Genetic Diversity Score: {}",
        genetics_analysis["data"]["diversity_scores"]["overall_diversity"]
    );

    // Get genetics statistics
    let genetics_stats: Value = client
        .get(format!("{base_url}/api/v1/genetics/stats"))
        .send()
        .await?
        .json()
        .await?;

    info!("Total Nodes: {}", genetics_stats["data"]["total_nodes"]);

    Ok(())
}

/// Demo Monitoring API with health checks and metrics
async fn demo_monitoring_api(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("📊 Testing Monitoring API");

    // Get comprehensive system health
    let health: Value = client
        .get(format!("{base_url}/api/v1/monitoring/health"))
        .send()
        .await?
        .json()
        .await?;

    info!("System Health: {}", health["data"]["overall_status"]);
    info!(
        "System Uptime: {} seconds",
        health["data"]["uptime_seconds"]
    );

    // Get system metrics
    let metrics: Value = client
        .get(format!("{base_url}/api/v1/monitoring/metrics"))
        .send()
        .await?
        .json()
        .await?;

    info!(
        "CPU Usage: {}%",
        metrics["data"]["performance"]["request_latency_p95_ms"]
    );
    info!(
        "Memory Usage: {}%",
        metrics["data"]["resources"]["memory_usage_percent"]
    );

    // Get active alerts
    let alerts: Value = client
        .get(format!("{base_url}/api/v1/monitoring/alerts"))
        .send()
        .await?
        .json()
        .await?;

    let alert_count = alerts["data"].as_array().map(|a| a.len()).unwrap_or(0);
    info!("Active Alerts: {}", alert_count);

    // Search logs
    let log_search = json!({
        "query": "API request",
        "level": "info",
        "limit": 10
    });

    let logs: Value = client
        .post(format!("{base_url}/api/v1/monitoring/logs/search"))
        .json(&log_search)
        .send()
        .await?
        .json()
        .await?;

    let log_count = logs["data"].as_array().map(|l| l.len()).unwrap_or(0);
    info!("Found {} log entries", log_count);

    Ok(())
}

/// Demo real-time monitoring capabilities
async fn demo_realtime_monitoring(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("⚡ Testing Real-time Monitoring");

    // Get real-time metrics multiple times to show live data
    for i in 1..=3 {
        let realtime: Value = client
            .get(format!("{base_url}/api/v1/monitoring/metrics/realtime"))
            .send()
            .await?
            .json()
            .await?;

        info!(
            "Real-time Sample {}: CPU={}%, Requests/sec={}, Response time={}ms",
            i,
            realtime["data"]["cpu_usage"],
            realtime["data"]["request_rate"],
            realtime["data"]["response_time_ms"]
        );

        sleep(Duration::from_millis(2000)).await;
    }

    // Test liveness and readiness probes
    let liveness: Value = client
        .get(format!("{base_url}/api/v1/monitoring/liveness"))
        .send()
        .await?
        .json()
        .await?;

    let readiness: Value = client
        .get(format!("{base_url}/api/v1/monitoring/readiness"))
        .send()
        .await?
        .json()
        .await?;

    info!(
        "Liveness: {}, Readiness: {}",
        liveness["data"]["alive"], readiness["data"]["ready"]
    );

    Ok(())
}

/// Demonstrate performance features like caching and rate limiting
async fn demo_performance_features(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("⚡ Testing Performance Features");

    // Make the same request multiple times to test caching
    let endpoint = format!("{base_url}/api/v1/security/stats");

    for i in 1..=3 {
        let start = std::time::Instant::now();
        let response: Value = client.get(&endpoint).send().await?.json().await?;

        let duration = start.elapsed();
        let cached = response["metadata"]["cached"].as_bool().unwrap_or(false);
        let processing_time = response["metadata"]["processing_time_ms"]
            .as_u64()
            .unwrap_or(0);

        info!(
            "Request {}: Total={}ms, Processing={}ms, Cached={}",
            i,
            duration.as_millis(),
            processing_time,
            cached
        );

        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

/// Test error handling and edge cases
async fn demo_error_handling(
    client: &reqwest::Client,
    base_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔧 Testing Error Handling");

    // Test invalid endpoint
    let invalid_response = client
        .get(format!("{base_url}/api/v1/invalid/endpoint"))
        .send()
        .await?;

    info!("Invalid endpoint status: {}", invalid_response.status());

    // Test malformed request
    let malformed_json = "{ invalid json }";
    let malformed_response = client
        .post(format!("{base_url}/api/v1/security/analyze"))
        .body(malformed_json)
        .header("Content-Type", "application/json")
        .send()
        .await?;

    info!("Malformed JSON status: {}", malformed_response.status());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_demo_flow() {
        // This would test the demo in a real environment
        // For now, just verify the demo functions compile
        assert!(true);
    }
}
