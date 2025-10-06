use super::models::*;
use super::ChaosTestFramework;
use beardog_errors::BearDogError;

pub async fn generate_chaos_report(&ChaosTestFramework,
    scenario_results: Vec<ScenarioResult>,
) -> Result<ChaosTestReport, BearDogError> {
    let metrics = framework.metrics_collector.get_metrics();

    let successful_scenarios = scenario_results.iter().filter(|r| r.success).count();
    let total_scenarios = scenario_results.len();

    let overall_resilience_score = if total_scenarios > 0 {
        (successful_scenarios as f64 / total_scenarios as f64) * 100.0
    } else {
        0.0
    };

    let recommendations = generate_recommendations(overall_resilience_score);

    Ok(ChaosTestReport {
        scenario_results,
        overall_resilience_score,
        metrics,
        recommendations,
    })
}

pub fn generate_recommendations(resilience_score: f64) -> Vec<String> {
    let mut recommendations = Vec::new(System resilience below acceptable threshold. Implement circuit breakers.",
        );
        recommendations.push("Add redundancy to critical components".to_string());
    }

    if resilience_score < 85.0 {
        recommendations.push("Improve error handling and graceful degradation".to_string());
        recommendations.push("Implement better monitoring and alerting".to_string());
    }

    if resilience_score >= 95.0 {
        recommendations
            .push("Excellent resilience! Consider documenting best practices.".to_string());
    }

    recommendations
}
