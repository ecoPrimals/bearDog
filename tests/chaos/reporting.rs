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


//! Chaos Testing Reporting
//!
//! Report generation and performance recommendations
//! for chaos testing framework.

use super::models::*;
use super::ChaosTestFramework;
use beardog::BearDogResult;

/// Generate comprehensive chaos test report
pub async fn generate_chaos_report(framework: &ChaosTestFramework, scenario_results: Vec<ScenarioResult>) -> BearDogResult<ChaosTestReport> {
    let metrics = framework.metrics_collector.get_metrics().await;
    
    let successful_scenarios = scenario_results.iter().filter(|r| r.success).count();
    let total_scenarios = scenario_results.len();
    
    let overall_resilience_score = if total_scenarios > 0 {
        (successful_scenarios as f64 / total_scenarios as f64) * 100.0
    } else {
        0.0
    };
    
    let recommendations = generate_recommendations(overall_resilience_score).await;
    
    Ok(ChaosTestReport {
        scenario_results,
        overall_resilience_score,
        metrics,
        recommendations,
    })
}

/// Generate performance recommendations based on resilience score
pub async fn generate_recommendations(resilience_score: f64) -> Vec<String> {
    let mut recommendations = Vec::new();
    
    if resilience_score < 70.0 {
        recommendations.push("Critical: System resilience below acceptable threshold. Implement circuit breakers.".to_string());
        recommendations.push("Add redundancy to critical components".to_string());
    }
    
    if resilience_score < 85.0 {
        recommendations.push("Improve error handling and graceful degradation".to_string());
        recommendations.push("Implement better monitoring and alerting".to_string());
    }
    
    if resilience_score >= 95.0 {
        recommendations.push("Excellent resilience! Consider documenting best practices.".to_string());
    }
    
    recommendations
} 