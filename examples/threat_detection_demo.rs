// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! # Threat Detection Engine Demo
//!
//! This demo showcases BearDog's advanced threat detection capabilities including:
//! - Rule-based threat detection
//! - ML-powered anomaly detection
//! - Behavioral analysis
//! - Threat intelligence integration
//! - Automated incident response

use beardog::threat::*;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::init();

    println!("🔍 BearDog Threat Detection Engine Demo");
    println!("========================================");

    // Initialize enhanced threat detection engine with ML
    let (mut threat_engine, ml_engine) = ThreatAPI::new_with_ml().await?;

    println!("✅ Threat Detection Engine initialized with ML capabilities");

    // Demo 1: Rule-based threat detection
    println!("\n📋 Demo 1: Rule-based Threat Detection");
    println!("--------------------------------------");

    // Create suspicious login event
    let suspicious_login = create_suspicious_login_event()?;
    println!("🔍 Analyzing suspicious login event...");

    let login_analysis = threat_engine.analyze_event(&suspicious_login).await?;
    println!("🚨 Threats detected: {}", login_analysis.threats_detected);

    for threat in &login_analysis.detected_threats {
        println!(
            "  - {}: {} ({})",
            threat.threat_type, threat.description, threat.severity
        );
    }

    // Demo 2: ML-powered anomaly detection
    println!("\n🧠 Demo 2: ML-powered Anomaly Detection");
    println!("--------------------------------------");

    // Create data exfiltration event
    let data_exfil_event = create_data_exfiltration_event()?;
    println!("🔍 Analyzing large data transfer event...");

    let exfil_analysis = threat_engine.analyze_event(&data_exfil_event).await?;
    println!("🚨 Threats detected: {}", exfil_analysis.threats_detected);

    for threat in &exfil_analysis.detected_threats {
        println!(
            "  - {}: {} ({})",
            threat.threat_type, threat.description, threat.severity
        );
    }

    for prediction in &exfil_analysis.ml_predictions {
        println!(
            "  - ML Prediction: {:?} (confidence: {:.3})",
            prediction.prediction_type, prediction.confidence_score
        );
    }

    // Demo 3: Brute force attack detection
    println!("\n🔥 Demo 3: Brute Force Attack Detection");
    println!("--------------------------------------");

    // Simulate multiple failed login attempts
    println!("🔍 Simulating brute force attack...");

    for i in 1..=15 {
        let failed_login = create_failed_login_event(i)?;
        let analysis = threat_engine.analyze_event(&failed_login).await?;

        if analysis.threats_detected > 0 {
            println!("🚨 Brute force attack detected after {} attempts!", i);
            for threat in &analysis.detected_threats {
                println!("  - {}: {}", threat.threat_type, threat.description);
            }
            break;
        }
    }

    // Demo 4: Threat intelligence integration
    println!("\n🌐 Demo 4: Threat Intelligence Integration");
    println!("----------------------------------------");

    // Add threat intelligence feed
    let threat_feed = create_sample_threat_feed()?;
    threat_engine.update_threat_feed(threat_feed).await?;
    println!("✅ Threat intelligence feed updated");

    // Create event with known malicious IP
    let malicious_event = create_malicious_ip_event()?;
    println!("🔍 Analyzing event with known malicious IP...");

    let intel_analysis = threat_engine.analyze_event(&malicious_event).await?;
    println!("🚨 Threats detected: {}", intel_analysis.threats_detected);

    for threat in &intel_analysis.detected_threats {
        println!(
            "  - {}: {} ({})",
            threat.threat_type, threat.description, threat.severity
        );
    }

    // Demo 5: Behavioral analysis
    println!("\n👤 Demo 5: Behavioral Analysis");
    println!("-----------------------------");

    // Create unusual access pattern
    let unusual_access = create_unusual_access_event()?;
    println!("🔍 Analyzing unusual access pattern...");

    // Use ML engine directly for behavioral analysis
    let behavioral_predictions = ml_engine.analyze_event(&unusual_access).await?;
    for prediction in &behavioral_predictions {
        println!(
            "🧠 Behavioral anomaly detected: {:?} (confidence: {:.3})",
            prediction.prediction_type, prediction.confidence_score
        );
        for evidence in &prediction.evidence {
            println!("  - Evidence: {}", evidence);
        }
    }

    // Demo 6: Comprehensive threat statistics
    println!("\n📊 Demo 6: Threat Statistics");
    println!("---------------------------");

    let stats = threat_engine.get_threat_statistics().await?;
    println!("Total threats detected: {}", stats.total_threats);
    println!("High severity threats: {}", stats.high_severity_threats);
    println!("Medium severity threats: {}", stats.medium_severity_threats);
    println!("Low severity threats: {}", stats.low_severity_threats);
    println!("Active incidents: {}", stats.active_incidents);
    println!("Detection rules: {}", stats.detection_rules_count);
    println!("Threat feeds: {}", stats.threat_feeds_count);
    println!("ML models: {}", stats.ml_models_count);

    // Demo 7: Custom detection rule
    println!("\n🛠️ Demo 7: Custom Detection Rule");
    println!("-------------------------------");

    // Add custom rule for cryptocurrency mining
    let crypto_mining_rule = ThreatDetectionRule {
        rule_id: "crypto_mining_001".to_string(),
        name: "Cryptocurrency Mining Detection".to_string(),
        description: "Detects potential cryptocurrency mining activity".to_string(),
        threat_type: ThreatType::ResourceAbuse,
        severity: ThreatLevel::Medium,
        conditions: vec![
            RuleCondition::EventType("process_execution".to_string()),
            RuleCondition::UserAgent("mining".to_string()),
        ],
        false_positive_rate: 0.10,
        mitre_techniques: vec!["T1496".to_string()], // Resource Hijacking
        response_actions: vec![
            ResponseAction::LogAlert("Cryptocurrency mining detected".to_string()),
            ResponseAction::NotifyAdmin("Suspicious resource usage detected".to_string()),
        ],
        enabled: true,
    };

    threat_engine.add_detection_rule(crypto_mining_rule);
    println!("✅ Custom detection rule added");

    // Test custom rule
    let mining_event = create_crypto_mining_event()?;
    let mining_analysis = threat_engine.analyze_event(&mining_event).await?;

    if mining_analysis.threats_detected > 0 {
        println!("🚨 Custom rule triggered!");
        for threat in &mining_analysis.detected_threats {
            println!("  - {}: {}", threat.threat_type, threat.description);
        }
    }

    println!("\n🎉 Demo completed successfully!");
    println!("The threat detection engine has demonstrated:");
    println!("✅ Rule-based detection with multiple conditions");
    println!("✅ ML-powered anomaly detection and behavioral analysis");
    println!("✅ Automated incident response and escalation");
    println!("✅ Threat intelligence integration and matching");
    println!("✅ Comprehensive threat statistics and reporting");
    println!("✅ Custom detection rule creation and management");

    Ok(())
}

/// Create a suspicious login event (off-hours)
fn create_suspicious_login_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "login_001".to_string(),
        timestamp: Utc::now(),
        event_type: "login".to_string(),
        source_ip: "203.0.113.42".to_string(), // External IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "john.doe".to_string(),
        user_agent: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string()),
        data_size: 0.0,
        location: Some("Unknown".to_string()),
        file_hash: None,
        additional_data: {
            let mut data = HashMap::new();
            data.insert("timestamp".to_string(), "23".to_string()); // 11 PM
            data
        },
    })
}

/// Create a data exfiltration event
fn create_data_exfiltration_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "data_transfer_001".to_string(),
        timestamp: Utc::now(),
        event_type: "data_transfer".to_string(),
        source_ip: "192.168.1.50".to_string(),
        destination_ip: "198.51.100.42".to_string(), // External IP
        user_id: "jane.smith".to_string(),
        user_agent: Some("curl/7.68.0".to_string()),
        data_size: 1024.0 * 1024.0 * 750.0, // 750MB
        location: Some("office".to_string()),
        file_hash: Some("a1b2c3d4e5f6".to_string()),
        additional_data: {
            let mut data = HashMap::new();
            data.insert("protocol".to_string(), "https".to_string());
            data.insert("encrypted".to_string(), "true".to_string());
            data
        },
    })
}

/// Create a failed login event for brute force simulation
fn create_failed_login_event(attempt: i32) -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: format!("failed_login_{:03}", attempt),
        timestamp: Utc::now(),
        event_type: "login_failed".to_string(),
        source_ip: "203.0.113.666".to_string(), // Suspicious IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "admin".to_string(),
        user_agent: Some("AttackTool/1.0".to_string()),
        data_size: 0.0,
        location: Some("Unknown".to_string()),
        file_hash: None,
        additional_data: {
            let mut data = HashMap::new();
            data.insert("attempt".to_string(), attempt.to_string());
            data.insert("reason".to_string(), "invalid_password".to_string());
            data
        },
    })
}

/// Create a sample threat intelligence feed
fn create_sample_threat_feed() -> Result<ThreatIntelligenceFeed, Box<dyn std::error::Error>> {
    Ok(ThreatIntelligenceFeed {
        feed_id: "malicious_ips_001".to_string(),
        name: "Known Malicious IPs".to_string(),
        source: "Security Research Labs".to_string(),
        updated_at: Utc::now(),
        indicators: vec![
            ThreatIndicator {
                indicator_type: "ip".to_string(),
                value: "198.51.100.66".to_string(),
                confidence: 0.95,
                description: "Command and Control Server".to_string(),
                first_seen: Utc::now(),
                last_seen: Utc::now(),
            },
            ThreatIndicator {
                indicator_type: "domain".to_string(),
                value: "malicious-domain.com".to_string(),
                confidence: 0.90,
                description: "Phishing Domain".to_string(),
                first_seen: Utc::now(),
                last_seen: Utc::now(),
            },
        ],
        metadata: HashMap::new(),
    })
}

/// Create an event with known malicious IP
fn create_malicious_ip_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "malicious_001".to_string(),
        timestamp: Utc::now(),
        event_type: "network_connection".to_string(),
        source_ip: "192.168.1.100".to_string(),
        destination_ip: "198.51.100.66".to_string(), // Known malicious IP
        user_id: "compromised.user".to_string(),
        user_agent: Some("Malware/1.0".to_string()),
        data_size: 1024.0,
        location: Some("office".to_string()),
        file_hash: None,
        additional_data: {
            let mut data = HashMap::new();
            data.insert("protocol".to_string(), "tcp".to_string());
            data.insert("port".to_string(), "8080".to_string());
            data
        },
    })
}

/// Create an unusual access event for behavioral analysis
fn create_unusual_access_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "unusual_access_001".to_string(),
        timestamp: Utc::now(),
        event_type: "login".to_string(),
        source_ip: "203.0.113.199".to_string(), // External IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "regular.user".to_string(),
        user_agent: Some("Mozilla/5.0 (Unknown OS)".to_string()),
        data_size: 0.0,
        location: Some("Tokyo".to_string()), // Unusual location
        file_hash: None,
        additional_data: {
            let mut data = HashMap::new();
            data.insert("timestamp".to_string(), "3".to_string()); // 3 AM
            data.insert("device".to_string(), "unknown".to_string());
            data
        },
    })
}

/// Create a cryptocurrency mining event
fn create_crypto_mining_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "mining_001".to_string(),
        timestamp: Utc::now(),
        event_type: "process_execution".to_string(),
        source_ip: "192.168.1.75".to_string(),
        destination_ip: "0.0.0.0".to_string(),
        user_id: "suspicious.user".to_string(),
        user_agent: Some("mining-bot/2.1".to_string()),
        data_size: 0.0,
        location: Some("office".to_string()),
        file_hash: Some("mining_hash_123".to_string()),
        additional_data: {
            let mut data = HashMap::new();
            data.insert("process".to_string(), "xmrig".to_string());
            data.insert("cpu_usage".to_string(), "95".to_string());
            data
        },
    })
}
