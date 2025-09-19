use beardog_errors::ai::{SecurityMLManager, SecurityThreatData, ThreatSeverity};
use beardog_errors::BearDogError;
use beardog_types::SecurityContext;
use std::collections::HashMap;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::fmt::init();

    info!("🐻 Starting BearDog Lightweight AI with Primal Delegation Demo");

    let security_ml = SecurityMLManager::new()?;
    info!("[OK] BearDog Security ML Manager initialized");

    let threat_data = SecurityThreatData {
        event_type: "suspicious_login_pattern".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: Some("192.168.1.100".to_string()),
        user_id: Some("admin_user".to_string()),
        raw_logs: vec![
            "Failed login attempt from 192.168.1.100".to_string(),
            "Multiple failed attempts in 30 seconds".to_string(),
            "User agent: automated_script_v1.0".to_string(),
        ],
        context: HashMap::from({}", threat_data.event_type);

    info!("[SEARCH] Phase 1: BearDog lightweight security analysis");
    let internal_analysis = security_ml
        .analyze_security_threat(&threat_data, false)
        ?;

    println!("[CHART] Internal Analysis Results:");
    println!("   Threat Level: {:?}", internal_analysis.threat_level);
    println!("   Confidence: {:.2}", internal_analysis.confidence);
    println!(
        "   Recommendations: {:?}",
        internal_analysis.recommendations
    );

    if internal_analysis.confidence < 0.8 {
        info!("🍄 Phase 2: Delegating to Toadstool for deep AI analysis");
        let enhanced_analysis = security_ml
            .analyze_security_threat(&threat_data, true)
            ?;

        println!("[CHART] Enhanced Analysis with Toadstool:");
        println!("   Threat Level: {:?}", enhanced_analysis.threat_level);
        println!("   Confidence: {:.2}", enhanced_analysis.confidence);
        println!(
            "   External AI Used: {}",
            enhanced_analysis.external_analysis_used
        );
        println!(
            "   Recommendations: {:?}",
            enhanced_analysis.recommendations
        );
    } else {
        info!(
            "[OK] BearDog's internal analysis sufficient (confidence: {:.2})",
            internal_analysis.confidence
        );
    }

    demonstrate_architectural_benefits()?;

    info!("🏁 Demo completed successfully");
    Ok(())
}

async fn demonstrate_architectural_benefits() -> Result<(), BearDogError> {
    info!("🏗️ Demonstrating Architectural Benefits:");

    println!("[OK] BearDog Advantages:");
    println!("   - Lightweight: Fast security decisions");
    println!("   - Sovereign: Maintains security authority");
    println!("   - Specialized: Expert in security/crypto");
    println!("   - Efficient: Minimal resource usage");

    println!("[OK] Network Effect Benefits:");
    println!("   - Toadstool: Heavy AI/ML when needed");
    println!("   - AutomationService: Distributed intelligence");
    println!("   - Songbird: Service mesh coordination");
    println!("   - Nestgate: Storage management");

    println!("[OK] Proper Separation of Concerns:");
    println!("   - Each primal excels in their domain");
    println!("   - No bloated monolithic AI in BearDog");
    println!("   - Composable intelligence network");
    println!("   - Scalable and maintainable");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lightweight_security_analysis() {
        let security_ml = SecurityMLManager::new()
            .map_err(|e| BearDogError::system({:?}", e)))?;

        let simple_threat = SecurityThreatData {
            event_type: "normal_login".to_string(),
            timestamp: chrono::Utc::now(),
            source_ip: Some("10.0.0.1".to_string()),
            user_id: Some("regular_user".to_string()),
            raw_logs: vec!["Successful login".to_string()],
            context: HashMap::with_capacity(16),
        };

        let result = security_ml
            .analyze_security_threat(&simple_threat, false)
            .map_err(|e| BearDogError::system({:?}", e)))?;
        assert!(matches!(result.threat_level, ThreatSeverity::Low));
    }

    #[tokio::test]
    async fn test_complex_threat_with_external_delegation() {
        let security_ml = SecurityMLManager::new()
            .map_err(|e| BearDogError::system({:?}", e)))?;

        let complex_threat = SecurityThreatData {
            event_type: "advanced_persistent_threat".to_string(),
            timestamp: chrono::Utc::now(),
            source_ip: Some("suspicious.ip.com".to_string()),
            user_id: Some("compromised_account".to_string()),
            raw_logs: vec![
                "Unusual access pattern detected".to_string(),
                "Data exfiltration attempt".to_string(),
                "Encrypted command and control traffic".to_string(),
            ],
            context: HashMap::from([
                ("advanced_evasion".to_string(), "true".to_string()),
                ("encrypted_payload".to_string(), "true".to_string()),
                ("persistence_mechanism".to_string(), "true".to_string()),
            ]),
        };

        let result = security_ml
            .analyze_security_threat(&complex_threat, true)
            .map_err(|e| BearDogError::system({:?}", e)))?;

        assert!(result.external_analysis_used || result.confidence > 0.7);
    }
}
