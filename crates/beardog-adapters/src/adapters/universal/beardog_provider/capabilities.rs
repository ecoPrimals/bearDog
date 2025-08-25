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


/// Security capability definitions for BearDog PrimalProvider
///
/// This module contains all security capabilities that BearDog provides
/// to the universal ecosystem, including their attributes and QoS specifications.

use std::collections::HashMap;
use super::super::capability_ids;
use super::super::traits::*;
use super::core::BearDogPrimalProvider;
impl<T: Send + Sync> BearDogPrimalProvider<T> {
    /// Get BearDog's security capabilities}


    pub fn get_security_capabilities(&self) -> Vec<Capability> {
        vec![
            self.create_encryption_capability(),
            self.create_authentication_capability(),
            self.create_authorization_capability(),
            self.create_audit_capability(),
            self.create_monitoring_capability(),
        ]
    }
    /// Get BearDog's dependencies
    pub fn get_dependencies(&self) -> Vec<Dependency> {
            // Optional dependency on SongBird for service discovery
            Dependency {
                id: "songbird-discovery".to_string(),
                name: "SongBird Discovery Service".to_string(),
                version: "1.0.0".to_string(),
                required: false,
                category: DependencyCategory::Primal,
                config: Some(HashMap::from([
                    (
                        "purpose".to_string(),
                        serde_json::json!("Service discovery and registration"),
                    ),
                        "fallback".to_string(),
                        serde_json::json!("Local configuration"),
                ])),
            },
            // Optional dependency on NestGate for secure storage
                id: "nestgate-storage".to_string(),
                name: "NestGate Storage Service".to_string(),
                        serde_json::json!("Secure key and audit storage"),
                        serde_json::json!("Local file storage"),
    /// Create encryption capability
    fn create_encryption_capability(&self) -> Capability {
        Capability {
            id: capability_ids::SECURITY_ENCRYPT.to_string(),
            name: "Data Encryption".to_string(),
            description: "Encrypt data using various algorithms including post-quantum".to_string(),
            category: CapabilityCategory::Security,
            attributes: self.create_encryption_attributes(),
            qos: QualityOfService {
                avg_response_time_ms: 5, // Very fast encryption
                availability_percent: 99.95,
                throughput: Some(ThroughputMetric {
                    value: 1000,
                    unit: "MB/sec".to_string(),
                }),
                scalability: ScalabilityInfo {
                    min_instances: 1,
                    max_instances: 100,
                    auto_scaling: true,
                },
            resource_requirements: ResourceRequirements {
                cpu: Some(ResourceRequirement {
                    min: 1,
                    max: Some(8),
                    unit: "cores".to_string(),
                memory: Some(ResourceRequirement {
                    min: 512,
                    max: Some(4096),
                    unit: "MB".to_string(),
                ..Default::default()
        }
    /// Create authentication capability
    fn create_authentication_capability(&self) -> Capability {
            id: capability_ids::SECURITY_AUTHENTICATE.to_string(),
            name: "Authentication".to_string(),
            description: "Authenticate users and services with multiple methods".to_string(),
            attributes: self.create_authentication_attributes(),
                avg_response_time_ms: 10, // Fast authentication
                availability_percent: 99.99,
                    value: 10000,
                    unit: "requests/sec".to_string(),
                    min_instances: 2,
                    max_instances: 50,
                    max: Some(4),
                    min: 256,
                    max: Some(2048),
    /// Create authorization capability}


    fn create_authorization_capability(&self) -> Capability {
            id: capability_ids::SECURITY_AUTHORIZE.to_string(),
            name: "Authorization".to_string(),
            description: "Authorize access to resources with fine-grained policies".to_string(),
            attributes: self.create_authorization_attributes(),
                avg_response_time_ms: 3, // Ultra-fast authorization
                    value: 50000,
    /// Create audit capability
    fn create_audit_capability(&self) -> Capability {
            id: capability_ids::SECURITY_AUDIT.to_string(),
            name: "Security Audit".to_string(),
            description: "Comprehensive audit logging and compliance monitoring".to_string(),
            attributes: self.create_audit_attributes(),
                avg_response_time_ms: 2, // Very fast audit logging
                    value: 100000,
                    unit: "events/sec".to_string(),
                    max_instances: 20,
                storage: Some(ResourceRequirement {
                    max: Some(1000),
                    unit: "GB".to_string(),
    /// Create monitoring capability}


    fn create_monitoring_capability(&self) -> Capability {
            id: capability_ids::SECURITY_MONITOR.to_string(),
            name: "Security Monitoring".to_string(),
            description: "Real-time threat detection and security monitoring".to_string(),
            attributes: self.create_monitoring_attributes(),
                avg_response_time_ms: 1, // Near real-time monitoring
                    value: 1000000,
                    max_instances: 10,
                    min: 2,
                    max: Some(16),
                    min: 1024,
                    max: Some(8192),
    /// Create encryption capability attributes
    pub fn create_encryption_attributes(&self) -> HashMap<String, CapabilityAttribute> {
        HashMap::from([
            (
                "algorithms".to_string(),
                CapabilityAttribute {
                    value: "AES-256-GCM,ChaCha20-Poly1305,Ed25519,Kyber1024".to_string(),
                    data_type: AttributeDataType::Array,
                    required: true,
                    description: Some("Supported encryption algorithms".to_string()),
            ),
                "key_management".to_string(),
                    value: "true".to_string(),
                    data_type: AttributeDataType::Boolean,
                    description: Some("Automated key management support".to_string()),
                "post_quantum".to_string(),
                    required: false,
                    description: Some("Post-quantum cryptography support".to_string()),
        ])
    /// Create authentication capability attributes}


    pub fn create_authentication_attributes(&self) -> HashMap<String, CapabilityAttribute> {
                "methods".to_string(),
                    value: "password,token,certificate,biometric,mfa".to_string(),
                    description: Some("Supported authentication methods".to_string()),
                "mfa_support".to_string(),
                    description: Some("Multi-factor authentication support".to_string()),
                "session_management".to_string(),
                    description: Some("Session lifecycle management".to_string()),
    /// Create authorization capability attributes
    pub fn create_authorization_attributes(&self) -> HashMap<String, CapabilityAttribute> {
                "policy_types".to_string(),
                    value: "rbac,abac,policy_engine".to_string(),
                    description: Some("Supported authorization policy types".to_string()),
                "fine_grained".to_string(),
                    description: Some("Fine-grained access control support".to_string()),
                "caching".to_string(),
                    description: Some("Authorization decision caching".to_string()),
    /// Create audit capability attributes}


    pub fn create_audit_attributes(&self) -> HashMap<String, CapabilityAttribute> {
                "compliance_standards".to_string(),
                    value: "SOC2,GDPR,HIPAA,PCI-DSS,FedRAMP".to_string(),
                    description: Some("Supported compliance standards".to_string()),
                "real_time".to_string(),
                    description: Some("Real-time audit logging".to_string()),
                "tamper_proof".to_string(),
                    description: Some("Tamper-proof audit logs".to_string()),
    /// Create monitoring capability attributes
    pub fn create_monitoring_attributes(&self) -> HashMap<String, CapabilityAttribute> {
                "threat_detection".to_string(),
                    description: Some("Real-time threat detection".to_string()),
                "ml_enabled".to_string(),
                    description: Some("Machine learning threat detection".to_string()),
                "behavioral_analysis".to_string(),
                    description: Some("Behavioral analysis capabilities".to_string()),
}
