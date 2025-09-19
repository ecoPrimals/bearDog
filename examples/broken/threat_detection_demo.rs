use beardog::threat::*;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::init();

    println!("[SEARCH] BearDog Threat Detection Engine Demo");
    println!("========================================");

    let (mut threat_engine, ml_engine) = ThreatAPI::new_with_ml()?;

    println!("[OK] Threat Detection Engine initialized with ML capabilities");

    println!("📋 Demo 1: Rule-based Threat Detection");
    println!("--------------------------------------");

    let suspicious_login = create_suspicious_login_event({}", login_analysis.threats_detected);

    for threat in &login_analysis.detected_threats {
        println!(
            "  - {}: {} ({})",
            threat.threat_type, threat.description, threat.severity
        );
    }

    println!("🧠 Demo 2: ML-powered Anomaly Detection");
    println!("--------------------------------------");

    let data_exfil_event = create_data_exfiltration_event({}", exfil_analysis.threats_detected);

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

    println!("🔥 Demo 3: Brute Force Attack Detection");
    println!("--------------------------------------");

    println!("[SEARCH] Simulating brute force attack...");

    for i in 1..=15 {
        let failed_login = create_failed_login_event({}", threat.threat_type, threat.description);
            }
            break;
        }
    }

    println!("🌐 Demo 4: Threat Intelligence Integration");
    println!("----------------------------------------");

    let threat_feed = create_sample_threat_feed({}", intel_analysis.threats_detected);

    for threat in &intel_analysis.detected_threats {
        println!(
            "  - {}: {} ({})",
            threat.threat_type, threat.description, threat.severity
        );
    }

    println!("👤 Demo 5: Behavioral Analysis");
    println!("-----------------------------");

    let unusual_access = create_unusual_access_event()?;
    println!("[SEARCH] Analyzing unusual access pattern...");

    let behavioral_predictions = ml_engine.analyze_event(&unusual_access)?;
    for prediction in &behavioral_predictions {
        println!(
            "🧠 Behavioral anomaly detected: {:?} (confidence: {:.3})",
            prediction.prediction_type, prediction.confidence_score
        );
        for evidence in &prediction.evidence {
            println!("  - Evidence: {}", evidence);
        }
    }

    println!("[CHART] Demo 6: Threat Statistics");
    println!("---------------------------");

    let stats = threat_engine.get_threat_statistics({}", stats.total_threats);
    println!("High severity threats: {}", stats.high_severity_threats);
    println!("Medium severity threats: {}", stats.medium_severity_threats);
    println!("Low severity threats: {}", stats.low_severity_threats);
    println!("Active incidents: {}", stats.active_incidents);
    println!("Detection rules: {}", stats.detection_rules_count);
    println!("Threat feeds: {}", stats.threat_feeds_count);
    println!("ML models: {}", stats.ml_models_count);

    println!("🛠️ Demo 7: Custom Detection Rule");
    println!("-------------------------------");

    let crypto_mining_rule = ThreatDetectionRule {
        rule_id: "crypto_mining_001".to_string(),
        name: "Cryptocurrency Mining Detection".to_string(),
        description: "Detects potential cryptocurrency mining activity".to_string(ThreatType::ResourceAbuse,
        severity: ThreatLevel::Medium,
        conditions: vec![
            RuleCondition::EventType("process_execution".to_string()),
            RuleCondition::UserAgent(0.10,
        mitre_techniques: vec!["T1496".to_string()], // Resource Hijacking
        response_actions: vec![
            ResponseAction::LogAlert("Cryptocurrency mining detected".to_string()),
            ResponseAction::NotifyAdmin(true,
    };

    threat_engine.add_detection_rule({}", threat.threat_type, threat.description);
        }
    }

    println!("[PARTY] Demo completed successfully!");
    println!("The threat detection engine has demonstrated:");
    println!("[OK] Rule-based detection with multiple conditions");
    println!("[OK] ML-powered anomaly detection and behavioral analysis");
    println!("[OK] Automated incident response and escalation");
    println!("[OK] Threat intelligence integration and matching");
    println!("[OK] Comprehensive threat statistics and reporting");
    println!("[OK] Custom detection rule creation and management");

    Ok(())
}

fn create_suspicious_login_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "login_001".to_string(),
        timestamp: Utc::now(),
        event_type: "login".to_string(),
        source_ip: "203.0.113.42".to_string(), // External IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "john.doe".to_string(),
        user_agent: Some(0.0,
        location: Some(None,
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("timestamp".to_string(), "23"); // 11 PM
            data
        },
    })
}

fn create_data_exfiltration_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "data_transfer_001".to_string(),
        timestamp: Utc::now(),
        event_type: "data_transfer".to_string(),
        source_ip: "192.168.1.50".to_string(),
        destination_ip: "198.51.100.42".to_string(), // External IP
        user_id: "jane.smith".to_string(),
        user_agent: Some(1024.0 * 1024.0 * 750.0, // 750MB
        location: Some("office".to_string()),
        file_hash: Some("a1b2c3d4e5f6".to_string()),
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("protocol".to_string(), "https");
            data.insert("encrypted".to_string(), "true".to_string());
            data
        },
    })
}

fn create_failed_login_event(attempt: i32) -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(format!("failed_login_{:03}", attempt),
        timestamp: Utc::now(),
        event_type: "login_failed ".to_string(),
        source_ip: "203.0.113.666".to_string(), // Suspicious IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "admin".to_string(),
        user_agent: Some(0.0,
        location: Some(None,
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("attempt".to_string(), attempt);
            data.insert("reason".to_string(), "invalid_password");
            data
        },
    })
}

fn create_sample_threat_feed() -> Result<ThreatIntelligenceFeed, Box<dyn std::error::Error>> {
    Ok(ThreatIntelligenceFeed {
        feed_id: "malicious_ips_001".to_string(),
        name: "Known Malicious IPs".to_string(),
        source: "Security Research Labs".to_string(),
        updated_at: Utc::now(),
        indicators: vec![
            ThreatIndicator {
                indicator_type: "ip".to_string(),
                value: "198.51.100.66".to_string(0.95,
                description: "Command and Control Server".to_string(),
                first_seen: Utc::now(),
                last_seen: Utc::now(),
            },
            ThreatIndicator {
                indicator_type: "domain".to_string(),
                value: "malicious-domain.com".to_string(0.90,
                description: "Phishing Domain".to_string(),
                first_seen: Utc::now(),
                last_seen: Utc::now(),
            },
        ],
        metadata: HashMap::with_capacity(16),
    })
}

fn create_malicious_ip_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "malicious_001".to_string(),
        timestamp: Utc::now(),
        event_type: "network_connection".to_string(),
        source_ip: "192.168.1.100".to_string(),
        destination_ip: "198.51.100.66".to_string(), // Known malicious IP
        user_id: "compromised.user".to_string(),
        user_agent: Some(1024.0,
        location: Some(None,
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("protocol".to_string(), "tcp");
            data.insert("port".to_string(), "8080");
            data
        },
    })
}

fn create_unusual_access_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "unusual_access_001".to_string(),
        timestamp: Utc::now(),
        event_type: "login".to_string(),
        source_ip: "203.0.113.199".to_string(), // External IP
        destination_ip: "192.168.1.10".to_string(),
        user_id: "regular.user".to_string(),
        user_agent: Some(0.0,
        location: Some(None,
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("timestamp".to_string(), "3"); // 3 AM
            data.insert("device".to_string(), "unknown");
            data
        },
    })
}

fn create_crypto_mining_event() -> Result<SecurityEvent, Box<dyn std::error::Error>> {
    Ok(SecurityEvent {
        event_id: "mining_001".to_string(),
        timestamp: Utc::now(),
        event_type: "process_execution".to_string(),
        source_ip: "192.168.1.75".to_string(),
        destination_ip: "0.0.0.0".to_string(),
        user_id: "suspicious.user".to_string(),
        user_agent: Some(0.0,
        location: Some("office".to_string()),
        file_hash: Some("mining_hash_123".to_string()),
        additional_data: {
            let mut data = HashMap::with_capacity(16);
            data.insert("process".to_string(), "xmrig");
            data.insert("cpu_usage".to_string(), "95");
            data
        },
    })
}
