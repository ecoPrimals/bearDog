

use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentOptimizationConfig {

    pub enable_ai_optimization: bool,

    pub performance_weight: f64,

    pub cost_weight: f64,

    pub reliability_weight: f64,

    pub max_deployment_time_minutes: u32,

    pub enable_auto_rollback: bool,
}

impl Default for DeploymentOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_ai_optimization: true,
            performance_weight: 0.4,
            cost_weight: 0.3,
            reliability_weight: 0.3,
            max_deployment_time_minutes: 30,
            enable_auto_rollback: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedDeploymentTarget {

    pub platform: String,

    pub architecture: String,

    pub performance_score: f64,

    pub cost_score: f64,

    pub reliability_score: f64,

    pub optimization_score: f64,

    pub estimated_deployment_time: u32,

    pub last_successful_deployment: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct DeploymentOptimizer {

    config: DeploymentOptimizationConfig,

    deployment_history: HashMap<String, Vec<DeploymentMetrics>>,

    platform_profiles: HashMap<String, PlatformProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentMetrics {

    pub timestamp: DateTime<Utc>,

    pub duration_seconds: u64,

    pub success: bool,

    pub performance_metrics: PerformanceMetrics,

    pub cost_metrics: CostMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {

    pub cpu_utilization: f64,

    pub memory_usage: f64,

    pub network_throughput: f64,

    pub avg_response_time_ms: f64,

    pub requests_per_second: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostMetrics {

    pub deployment_cost_usd: f64,

    pub hourly_cost_usd: f64,

    pub storage_cost_usd_per_gb: f64,

    pub network_cost_usd_per_gb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformProfile {

    pub name: String,

    pub architectures: Vec<String>,

    pub avg_performance_score: f64,

    pub avg_cost_score: f64,

    pub reliability_percentage: f64,

    pub typical_deployment_time: u32,
}

impl DeploymentOptimizer {

    pub fn new(config: DeploymentOptimizationConfig) -> Self {
        let mut platform_profiles = HashMap::with_capacity(16);

        platform_profiles.insert("android".to_string(), PlatformProfile {
            name: "Android".to_string(),
            architectures: vec!["arm64-v8a".to_string(), "armeabi-v7a".to_string()],
            avg_performance_score: 0.85,
            avg_cost_score: 0.2, // Lower cost
            reliability_percentage: 0.92,
            typical_deployment_time: 15,
        });
        
        platform_profiles.insert("ios".to_string(), PlatformProfile {
            name: "iOS".to_string(),
            architectures: vec!["arm64".to_string()],
            avg_performance_score: 0.95,
            avg_cost_score: 0.4, // Higher cost but better performance
            reliability_percentage: 0.98,
            typical_deployment_time: 20,
        });
        
        platform_profiles.insert("linux".to_string(), PlatformProfile {
            name: "Linux".to_string(),
            architectures: vec!["x86_64".to_string(), "aarch64".to_string()],
            avg_performance_score: 0.9,
            avg_cost_score: 0.15, // Very cost effective
            reliability_percentage: 0.95,
            typical_deployment_time: 8,
        });
        
        platform_profiles.insert("windows".to_string(), PlatformProfile {
            name: "Windows".to_string(),
            architectures: vec!["x86_64".to_string()],
            avg_performance_score: 0.8,
            avg_cost_score: 0.35,
            reliability_percentage: 0.88,
            typical_deployment_time: 12,
        });
        
        Self {
            config,
            deployment_history: HashMap::with_capacity(16),
            platform_profiles,
        }
    }

    pub async fn optimize_deployment_targets(&self, requirements: &DeploymentRequirements) -> BearDogResult<Vec<OptimizedDeploymentTarget>> {
        info!("🎯 Optimizing deployment targets with AI-driven selection");
        
        let mut optimized_targets = Vec::new();

        for (platform_name, profile) in &self.platform_profiles {

            if let Some(required_arch) = &requirements.target_architecture {
                if !profile.architectures.contains(required_arch) {
                    debug!("❌ Platform {} doesn't support architecture {}", platform_name, required_arch);
                    continue;
                }
            }

            let optimization_score = self.calculate_optimization_score(profile, requirements).await?;

            if optimization_score >= requirements.min_optimization_score {
                let target = OptimizedDeploymentTarget {
                    platform: platform_name.clone(),
                    architecture: profile.architectures[0].clone(), // Use primary architecture
                    performance_score: profile.avg_performance_score,
                    cost_score: profile.avg_cost_score,
                    reliability_score: profile.reliability_percentage,
                    optimization_score,
                    estimated_deployment_time: profile.typical_deployment_time,
                    last_successful_deployment: self.get_last_successful_deployment(platform_name),
                };
                
                optimized_targets.push(target);
                info!("✅ Platform {} optimized with score: {:.3}", platform_name, optimization_score);
            } else {
                debug!("❌ Platform {} doesn't meet minimum optimization score", platform_name);
            }
        }

        optimized_targets.sort_by(|a, b| {
            b.optimization_score.partial_cmp(&a.optimization_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        if let Some(max_targets) = requirements.max_targets {
            optimized_targets.truncate(max_targets);
        }
        
        info!("🎯 Deployment optimization complete. Selected {} targets", optimized_targets.len());
        Ok(optimized_targets)
    }

    async fn calculate_optimization_score(
        &self, 
        profile: &PlatformProfile, 
        requirements: &DeploymentRequirements
    ) -> BearDogResult<f64> {
        debug!("🧮 Calculating optimization score for platform: {}", profile.name);

        let performance_component = profile.avg_performance_score * self.config.performance_weight;
        let cost_component = (1.0 - profile.avg_cost_score) * self.config.cost_weight; // Invert cost (lower is better)
        let reliability_component = profile.reliability_percentage * self.config.reliability_weight;

        let mut score = performance_component + cost_component + reliability_component;

        if let Some(max_time) = requirements.max_deployment_time_minutes {
            if profile.typical_deployment_time > max_time {
                let time_penalty = 0.2 * ((profile.typical_deployment_time - max_time) as f64 / max_time as f64);
                score -= time_penalty.min(0.5); // Cap penalty at 0.5
                debug!("⏰ Applied time penalty: {:.3}", time_penalty);
            }
        }

        if let Some(_last_success) = self.get_last_successful_deployment(&profile.name) {
            score += 0.1; // 10% bonus for recent success
            debug!("✅ Applied recent success bonus");
        }

        if self.config.enable_ai_optimization {
            let ml_adjustment = self.calculate_ml_adjustment(&profile.name).await?;
            score += ml_adjustment;
            debug!("🤖 Applied ML adjustment: {:.3}", ml_adjustment);
        }

        let final_score = score.max(0.0).min(1.0);
        debug!("📊 Final optimization score for {}: {:.3}", profile.name, final_score);
        
        Ok(final_score)
    }

    async fn calculate_ml_adjustment(&self, platform_name: &str) -> BearDogResult<f64> {
        debug!("🤖 Calculating ML adjustment for platform: {}", platform_name);

        if let Some(history) = self.deployment_history.get(platform_name) {
            if history.is_empty() {
                return Ok(0.0);
            }

            let success_count = history.iter().filter(|m| m.success).count();
            let success_rate = success_count as f64 / history.len() as f64;

            let avg_performance: f64 = history.iter()
                .filter(|m| m.success)
                .map(|m| m.performance_metrics.requests_per_second / 1000.0) // Normalize
                .sum::<f64>() / success_count.max(1) as f64;

            let ml_adjustment = (success_rate - 0.5) * 0.2 + (avg_performance - 0.5) * 0.1;
            
            debug!("🧠 ML adjustment: success_rate={:.3}, avg_performance={:.3}, adjustment={:.3}", 
                   success_rate, avg_performance, ml_adjustment);
            
            Ok(ml_adjustment.max(-0.3).min(0.3)) // Cap adjustment
        } else {
            Ok(0.0) // No historical data
        }
    }

    fn get_last_successful_deployment(&self, platform_name: &str) -> Option<DateTime<Utc>> {
        self.deployment_history.get(platform_name)?
            .iter()
            .filter(|m| m.success)
            .map(|m| m.timestamp)
            .max()
    }

    pub fn record_deployment_metrics(&mut self, platform: &str, metrics: DeploymentMetrics) {
        debug!("📊 Recording deployment metrics for platform: {}", platform);
        
        self.deployment_history
            .entry(platform.to_string())
            .or_default()
            .push(metrics);

        if let Some(history) = self.deployment_history.get_mut(platform) {
            if history.len() > 100 {
                history.drain(0..history.len() - 100);
            }
        }
        
        info!("✅ Deployment metrics recorded for {}", platform);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRequirements {

    pub target_architecture: Option<String>,

    pub max_deployment_time_minutes: Option<u32>,

    pub min_optimization_score: f64,

    pub max_targets: Option<usize>,

    pub priority: String,
}

impl Default for DeploymentRequirements {
    fn default() -> Self {
        Self {
            target_architecture: None,
            max_deployment_time_minutes: None,
            min_optimization_score: 0.6,
            max_targets: Some(3),
            priority: "balanced".to_string(),
        }
    }
} 