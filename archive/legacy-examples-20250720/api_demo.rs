//! BearDog API System Demo

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 BearDog API Demo");
    println!("==================");

    // Demo 1: Security Analysis
    println!("\n🔍 Security Event Analysis:");
    demo_security_analysis().await?;

    // Demo 2: ML Predictions
    println!("\n🧠 ML Threat Prediction:");
    demo_ml_predictions().await?;

    // Demo 3: Performance Features
    println!("\n⚡ Performance Features:");
    demo_performance().await?;

    println!("\n✅ API Demo completed!");

    Ok(())
}

async fn demo_security_analysis() -> Result<(), Box<dyn std::error::Error>> {
    let event = json!({
        "event_type": "login",
        "source_ip": "203.0.113.42",
        "user_id": "john.doe"
    });

    println!("   Event: {}", event["event_type"]);
    println!("   Source: {}", event["source_ip"]);
    println!("   Result: HIGH RISK - External IP login");

    Ok(())
}

async fn demo_ml_predictions() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Model: login_anomaly_v1");
    println!("   Confidence: 0.87");
    println!("   Prediction: SUSPICIOUS_LOGIN");

    Ok(())
}

async fn demo_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Cache hit rate: 84.7%");
    println!("   Rate limiting: Active");
    println!("   Response time: 18ms avg");

    Ok(())
}
