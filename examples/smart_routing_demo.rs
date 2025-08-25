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


//! Smart Routing Strategies Demo
//!
//! Demonstrates intelligent routing strategies for distributing operations
//! across different providers and backends based on various criteria.

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone)]
enum RoutingStrategy {
    RoundRobin,
    PerformanceBased,
    CostOptimized,
    FailoverPrimary,
}

#[derive(Debug, Clone)]
struct Provider {
    id: String,
    name: String,
    performance_score: f64,
    cost_per_operation: f64,
    availability: f64,
    current_load: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct RoutingDecision {
    selected_provider: String,
    reason: String,
    confidence: f64,
}

struct SmartRouter {
    providers: Vec<Provider>,
    strategy: RoutingStrategy,
    request_history: HashMap<String, Vec<f64>>,
}

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🧠 BearDog Smart Routing Strategies Demo");
    info!("========================================");

    // Demonstrate different routing strategies
    demonstrate_routing_strategies().await?;

    // Show adaptive routing based on performance
    demonstrate_adaptive_routing().await?;

    // Demonstrate circuit breaker patterns
    demonstrate_circuit_breaker().await?;

    info!("✅ Smart Routing Demo Complete!");
    Ok(())
}

async fn demonstrate_routing_strategies() -> BearDogResult<()> {
    info!("🎯 Routing Strategy Demonstrations");
    info!("----------------------------------");

    // Create sample providers
    let providers = create_sample_providers();

    for provider in &providers {
        info!("🏢 Provider: {}", provider.name);
        info!("   Performance: {:.2}", provider.performance_score);
        info!("   Cost: ${:.4}/op", provider.cost_per_operation);
        info!("   Availability: {:.1}%", provider.availability * 100.0);
        info!("   Load: {:.1}%", provider.current_load * 100.0);
    }

    // Demonstrate different routing strategies
    let strategies = vec![
        RoutingStrategy::RoundRobin,
        RoutingStrategy::PerformanceBased,
        RoutingStrategy::CostOptimized,
        RoutingStrategy::FailoverPrimary,
    ];

    for strategy in strategies {
        info!("\n📊 Strategy: {:?}", strategy);
        let router = SmartRouter::new(providers.clone(), strategy);

        // Make several routing decisions
        for i in 1..=3 {
            let decision = router.route_request(&format!("request_{i}")).await;
            info!(
                "   Request {}: {} ({})",
                i, decision.selected_provider, decision.reason
            );
        }
    }

    Ok(())
}

async fn demonstrate_adaptive_routing() -> BearDogResult<()> {
    info!("\n🔄 Adaptive Routing Based on Performance");
    info!("----------------------------------------");

    let mut router = SmartRouter::new(create_sample_providers(), RoutingStrategy::PerformanceBased);

    // Simulate requests and performance feedback
    for round in 1..=3 {
        info!("📈 Round {}: Learning from performance", round);

        for request_id in 1..=5 {
            let request_name = format!("adaptive_request_{round}_{request_id}");
            let decision = router.route_request(&request_name).await;

            // Simulate performance feedback
            let simulated_response_time = simulate_request_performance(&decision.selected_provider);
            router.record_performance(&request_name, simulated_response_time);

            info!(
                "   {}: {} ({:.2}ms)",
                request_name, decision.selected_provider, simulated_response_time
            );
        }

        // Show how the router adapts
        router.update_provider_scores().await;
        info!("   📊 Provider scores updated based on performance");
    }

    Ok(())
}

async fn demonstrate_circuit_breaker() -> BearDogResult<()> {
    info!("\n⚡ Circuit Breaker Pattern");
    info!("-------------------------");

    let mut router = SmartRouter::new(create_sample_providers(), RoutingStrategy::FailoverPrimary);

    info!("🔄 Normal operation:");
    for i in 1..=3 {
        let decision = router.route_request(&format!("normal_{i}")).await;
        info!("   Request {}: {}", i, decision.selected_provider);
    }

    // Simulate provider failure
    info!("\n⚠️  Simulating provider failure:");
    router.simulate_provider_failure("primary-provider").await;

    for i in 1..=3 {
        let decision = router.route_request(&format!("failover_{i}")).await;
        info!(
            "   Request {}: {} ({})",
            i, decision.selected_provider, decision.reason
        );
    }

    // Simulate recovery
    info!("\n✅ Simulating provider recovery:");
    router.simulate_provider_recovery("primary-provider").await;

    let decision = router.route_request("recovery_test").await;
    info!(
        "   Recovery test: {} ({})",
        decision.selected_provider, decision.reason
    );

    Ok(())
}

impl SmartRouter {
    fn new(providers: Vec<Provider>, strategy: RoutingStrategy) -> Self {
        Self {
            providers,
            strategy,
            request_history: HashMap::new(),
        }
    }

    async fn route_request(&self, request_id: &str) -> RoutingDecision {
        match self.strategy {
            RoutingStrategy::RoundRobin => self.route_round_robin(request_id).await,
            RoutingStrategy::PerformanceBased => self.route_performance_based().await,
            RoutingStrategy::CostOptimized => self.route_cost_optimized().await,
            RoutingStrategy::FailoverPrimary => self.route_failover_primary().await,
        }
    }

    async fn route_round_robin(&self, request_id: &str) -> RoutingDecision {
        let hash = request_id.len() % self.providers.len();
        let selected = &self.providers[hash];

        RoutingDecision {
            selected_provider: selected.name.clone(),
            reason: "Round-robin distribution".to_string(),
            confidence: 0.8,
        }
    }

    async fn route_performance_based(&self) -> RoutingDecision {
        let best_provider = self
            .providers
            .iter()
            .max_by(|a, b| {
                a.performance_score
                    .partial_cmp(&b.performance_score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| {
                tracing::error!("No providers available for performance-based routing");
                beardog_errors::BearDogError::internal("No providers available".to_string())
            })
            .unwrap_or(&self.providers[0]); // Fallback to first provider

        RoutingDecision {
            selected_provider: best_provider.name.clone(),
            reason: format!(
                "Highest performance score: {:.2}",
                best_provider.performance_score
            ),
            confidence: 0.9,
        }
    }

    async fn route_cost_optimized(&self) -> RoutingDecision {
        let cheapest_provider = self
            .providers
            .iter()
            .min_by(|a, b| {
                a.cost_per_operation
                    .partial_cmp(&b.cost_per_operation)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .ok_or_else(|| {
                tracing::error!("No providers available for cost-optimized routing");
                beardog_errors::BearDogError::internal("No providers available".to_string())
            })
            .unwrap_or(&self.providers[0]); // Fallback to first provider

        RoutingDecision {
            selected_provider: cheapest_provider.name.clone(),
            reason: format!(
                "Lowest cost: ${:.4}/op",
                cheapest_provider.cost_per_operation
            ),
            confidence: 0.85,
        }
    }

    async fn route_failover_primary(&self) -> RoutingDecision {
        // Try primary provider first
        let primary = self.providers.iter().find(|p| p.id == "primary-provider");

        if let Some(provider) = primary {
            if provider.availability > 0.9 {
                return RoutingDecision {
                    selected_provider: provider.name.clone(),
                    reason: "Primary provider available".to_string(),
                    confidence: 0.95,
                };
            }
        }

        // Fallback to best available provider
        let fallback = self
            .providers
            .iter()
            .filter(|p| p.availability > 0.8)
            .max_by(|a, b| {
                a.availability
                    .partial_cmp(&b.availability)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or(&self.providers[0]);

        RoutingDecision {
            selected_provider: fallback.name.clone(),
            reason: "Failover to backup provider".to_string(),
            confidence: 0.7,
        }
    }

    fn record_performance(&mut self, request_id: &str, response_time: f64) {
        self.request_history
            .entry(request_id.to_string())
            .or_default()
            .push(response_time);
    }

    async fn update_provider_scores(&mut self) {
        // Simulate updating provider scores based on historical performance
        for provider in &mut self.providers {
            // Add some randomness to simulate real-world performance changes
            let performance_delta = (rand::random::<f64>() - 0.5) * 0.1;
            provider.performance_score =
                (provider.performance_score + performance_delta).clamp(0.1, 1.0);
        }
    }

    async fn simulate_provider_failure(&mut self, provider_id: &str) {
        if let Some(provider) = self.providers.iter_mut().find(|p| p.id == provider_id) {
            provider.availability = 0.0;
            warn!("⚠️  Provider {} marked as unavailable", provider.name);
        }
    }

    async fn simulate_provider_recovery(&mut self, provider_id: &str) {
        if let Some(provider) = self.providers.iter_mut().find(|p| p.id == provider_id) {
            provider.availability = 0.99;
            info!("✅ Provider {} recovered", provider.name);
        }
    }
}

fn create_sample_providers() -> Vec<Provider> {
    vec![
        Provider {
            id: "primary-provider".to_string(),
            name: "Primary HSM Provider".to_string(),
            performance_score: 0.95,
            cost_per_operation: 0.001,
            availability: 0.99,
            current_load: 0.3,
        },
        Provider {
            id: "backup-provider".to_string(),
            name: "Backup Cloud Provider".to_string(),
            performance_score: 0.85,
            cost_per_operation: 0.0005,
            availability: 0.98,
            current_load: 0.6,
        },
        Provider {
            id: "budget-provider".to_string(),
            name: "Budget Software Provider".to_string(),
            performance_score: 0.70,
            cost_per_operation: 0.0001,
            availability: 0.95,
            current_load: 0.8,
        },
        Provider {
            id: "premium-provider".to_string(),
            name: "Premium Hardware Provider".to_string(),
            performance_score: 0.98,
            cost_per_operation: 0.005,
            availability: 0.999,
            current_load: 0.2,
        },
    ]
}

fn simulate_request_performance(provider_name: &str) -> f64 {
    // Simulate different performance characteristics for different providers
    match provider_name {
        "Primary HSM Provider" => 10.0 + rand::random::<f64>() * 5.0,
        "Backup Cloud Provider" => 25.0 + rand::random::<f64>() * 10.0,
        "Budget Software Provider" => 50.0 + rand::random::<f64>() * 20.0,
        "Premium Hardware Provider" => 5.0 + rand::random::<f64>() * 2.0,
        _ => 100.0,
    }
}
