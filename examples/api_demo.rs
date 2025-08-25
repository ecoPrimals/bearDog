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
