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


/// Health and status monitoring for AI interface

use serde::{Deserialize, Serialize};
/// AI Health Status Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIHealthStatus {
    pub status: String,
    pub timestamp: String,
    pub components: Vec<AIHealthCheck>,
}
/// AI Health Check Component
pub struct AIHealthCheck {
    pub name: String,
    pub message: String,
/// AI Metrics Response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AIMetricsResponse {
    pub system: AISystemMetrics,
    pub security: AISecurityMetrics,
    pub genetics: AIGeneticsMetrics,
    pub tunnel: AITunnelMetrics,
/// AI System Metrics
pub struct AISystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub uptime: u64,
/// AI Security Metrics
pub struct AISecurityMetrics {
    pub encryptions_per_second: f64,
    pub signatures_per_second: f64,
    pub verifications_per_second: f64,
/// AI Genetics Metrics
pub struct AIGeneticsMetrics {
    pub active_nodes: u32,
    pub spawns_per_hour: f64,
    pub evolution_rate: f64,
/// AI Tunnel Metrics
pub struct AITunnelMetrics {
    pub active_connections: u32,
    pub throughput_mbps: f64,
    pub latency_ms: f64,}


impl Default for AIHealthStatus {}


    fn default() -> Self {
        Self {
            status: "healthy".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            components: vec![
                AIHealthCheck {
                    name: "core".to_string(),
                    status: "healthy".to_string(),
                    message: "Core systems operational".to_string(),
                },
                    name: "hsm".to_string(),
                    message: "HSM tiers available".to_string(),
            ],
        }
    }
impl Default for AISystemMetrics {
            cpu_usage: 0.0,
            memory_usage: 0,
            uptime: 0,}


impl Default for AISecurityMetrics {
            encryptions_per_second: 0.0,
            signatures_per_second: 0.0,
            verifications_per_second: 0.0,
impl Default for AIGeneticsMetrics {
            active_nodes: 0,
            spawns_per_hour: 0.0,
            evolution_rate: 0.0,}


impl Default for AITunnelMetrics {
            active_connections: 0,
            throughput_mbps: 0.0,
            latency_ms: 0.0,
