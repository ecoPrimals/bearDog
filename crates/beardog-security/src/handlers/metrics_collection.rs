//! Metrics Collection Module
//!
//! Handles security metrics collection, monitoring, and reporting.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Get current comprehensive security metrics
    pub async fn get_current_metrics(&self) -> SecurityProviderMetrics {
        let mut metrics = self.metrics.clone();

        // Update dynamic metrics
        metrics.active_sessions = self.session_store.len() as u64;
        metrics.collected_at = Utc::now();

        // Calculate rates and averages
        self.calculate_performance_metrics(&mut metrics).await;

        metrics
    }

    /// Update authentication metrics
    pub async fn update_auth_metrics(&mut self, success: bool, duration_ms: u64) {
        if success {
            self.metrics.successful_authentications += 1;
        } else {
            self.metrics.failed_authentications += 1;
        }

        // Update average response time
        let total_ops =
            self.metrics.successful_authentications + self.metrics.failed_authentications;
        if total_ops > 0 {
            self.metrics.avg_response_time_ms =
                (self.metrics.avg_response_time_ms * (total_ops - 1) as f64 + duration_ms as f64)
                    / total_ops as f64;
        }

        // Update success rate
        self.metrics.auth_success_rate =
            self.metrics.successful_authentications as f64 / total_ops as f64;
    }

    /// Update authorization metrics
    pub async fn update_authz_metrics(&mut self, granted: bool, risk_level: RiskLevel) {
        if granted {
            self.metrics.successful_authorizations += 1;
        } else {
            self.metrics.failed_authorizations += 1;
        }

        // Track risk level distribution
        match risk_level {
            RiskLevel::Low => self.metrics.low_risk_operations += 1,
            RiskLevel::Medium => self.metrics.medium_risk_operations += 1,
            RiskLevel::High => self.metrics.high_risk_operations += 1,
            RiskLevel::Critical => self.metrics.critical_risk_operations += 1,
        }

        // Update authorization success rate
        let total_authz =
            self.metrics.successful_authorizations + self.metrics.failed_authorizations;
        if total_authz > 0 {
            self.metrics.authz_success_rate =
                self.metrics.successful_authorizations as f64 / total_authz as f64;
        }
    }

    /// Update session metrics
    pub async fn update_session_metrics(&mut self, created: bool) {
        if created {
            self.metrics.total_sessions_created += 1;
            self.metrics.active_sessions += 1;
        }
    }

    /// Update MFA metrics
    pub async fn update_mfa_metrics(&mut self, generated: bool, verified: bool, success: bool) {
        if generated {
            self.metrics.mfa_tokens_generated += 1;
        }

        if verified {
            if success {
                self.metrics.mfa_verifications_successful += 1;
            } else {
                self.metrics.mfa_verifications_failed += 1;
            }
        }
    }

    /// Calculate performance metrics
    async fn calculate_performance_metrics(&self, metrics: &mut SecurityProviderMetrics) {
        // Calculate requests per second (simplified)
        let uptime_seconds = metrics.uptime_seconds.max(1);
        let total_requests = metrics.successful_authentications
            + metrics.failed_authentications
            + metrics.successful_authorizations
            + metrics.failed_authorizations;

        metrics.requests_per_second = total_requests as f64 / uptime_seconds as f64;

        // Calculate error rate
        let total_operations = total_requests;
        let total_errors = metrics.failed_authentications + metrics.failed_authorizations;

        metrics.error_rate = if total_operations > 0 {
            total_errors as f64 / total_operations as f64
        } else {
            0.0
        };
    }

    /// Get security health indicators
    pub async fn get_security_health(&self) -> SecurityHealth {
        let metrics = self.get_current_metrics().await;

        // Determine overall health based on key metrics
        let health_score = self.calculate_health_score(&metrics).await;

        let status = match health_score {
            score if score >= 0.9 => HealthStatus::Healthy,
            score if score >= 0.7 => HealthStatus::Degraded,
            _ => HealthStatus::Unhealthy,
        };

        SecurityHealth {
            overall_status: status,
            health_score,
            auth_success_rate: metrics.auth_success_rate,
            authz_success_rate: metrics.authz_success_rate,
            error_rate: metrics.error_rate,
            active_sessions: metrics.active_sessions,
            high_risk_operations: metrics.high_risk_operations,
            last_check: Utc::now(),
        }
    }

    /// Calculate overall system health score
    async fn calculate_health_score(&self, metrics: &SecurityProviderMetrics) -> f64 {
        let mut score: f64 = 1.0;

        // Penalize high error rates
        if metrics.error_rate > 0.1 {
            score -= 0.3;
        } else if metrics.error_rate > 0.05 {
            score -= 0.1;
        }

        // Penalize low success rates
        if metrics.auth_success_rate < 0.8 {
            score -= 0.2;
        }

        if metrics.authz_success_rate < 0.9 {
            score -= 0.2;
        }

        // Penalize high response times
        if metrics.avg_response_time_ms > 1000.0 {
            score -= 0.2;
        } else if metrics.avg_response_time_ms > 500.0 {
            score -= 0.1;
        }

        // Penalize excessive high-risk operations
        let total_risk_ops = metrics.low_risk_operations
            + metrics.medium_risk_operations
            + metrics.high_risk_operations
            + metrics.critical_risk_operations;

        if total_risk_ops > 0 {
            let high_risk_ratio = (metrics.high_risk_operations + metrics.critical_risk_operations)
                as f64
                / total_risk_ops as f64;
            if high_risk_ratio > 0.2 {
                score -= 0.1;
            }
        }

        score.max(0.0f64)
    }

    /// Reset metrics (admin function)
    pub async fn reset_metrics(&mut self) {
        self.metrics = SecurityProviderMetrics::default();
        self.metrics.collected_at = Utc::now();
    }
}

/// Security health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityHealth {
    pub overall_status: HealthStatus,
    pub health_score: f64, // 0.0 to 1.0
    pub auth_success_rate: f64,
    pub authz_success_rate: f64,
    pub error_rate: f64,
    pub active_sessions: u64,
    pub high_risk_operations: u64,
    pub last_check: chrono::DateTime<Utc>,
}
