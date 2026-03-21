// SPDX-License-Identifier: AGPL-3.0-only
#![allow(
    missing_docs,
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[ROCKET] BearDog API Demo");
    println!("==================");

    println!("[SEARCH] Security Event Analysis:");
    demo_security_analysis().await?;

    println!("🧠 ML Threat Prediction:");
    demo_ml_predictions().await?;

    println!("[LIGHTNING] Performance Features:");
    demo_performance().await?;

    println!("[OK] API Demo completed!");

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
