//! BearDog Security Manager - Main Entry Point
//!
//! **Democratizing Enterprise-Grade Security for Everyone**

use clap::{Arg, Command};
use std::sync::Arc;
use tokio::signal;
use tracing::{error, info, warn};
use tracing_subscriber::{fmt, EnvFilter};

use beardog_adapters::adapters::nestgate::{
    FileOperation, FileOperationRequest, NestGateAdapter, OperationPriority,
};
use beardog_api::api::server::BearDogApiServer;
use beardog_compliance::{ComplianceEngine, ComplianceEvent};
use beardog_config::{
    constants::version::MISSION, constants::version::VERSION, core::BearDogConfig,
};
use beardog_core::core::BearDogCore;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_threat::{threat::types::SecurityEvent, ThreatDetectionEngine};
use beardog_workflows::workflows::{
    ApprovalDecision, ApprovalSubmission, WorkflowPriority, WorkflowRequest, WorkflowTarget,
    WorkflowType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("beardog=info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();

    // Parse command line arguments
    let matches = Command::new("beardog")
        .version(VERSION)
        .about("Enterprise-grade security manager for everyone")
        .long_about("BearDog democratizes Fortune 500-grade security capabilities, \
                     making enterprise security accessible to developers and organizations of all sizes.")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Configuration file path")
                .required(false),
        )
        .arg(
            Arg::new("demo")
                .long("demo")
                .help("Run in demo mode with example data")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("api-only")
                .long("api-only")
                .help("Run only the API server")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Load configuration
    let config = if let Some(config_path) = matches.get_one::<String>("config") {
        info!("Loading configuration from: {}", config_path);
        BearDogConfig::from_file(config_path)?
    } else {
        info!("Loading configuration from environment");
        BearDogConfig::from_env()?
    };

    // Initialize BearDog core
    info!("🐕 Initializing BearDog Security Manager v{}", VERSION);
    info!("Mission: {}", MISSION);

    // Create BearDog core with production-ready initialization
    info!("🔧 Creating BearDog core instance...");
    let beardog_core: Arc<BearDogCore> = Arc::new(BearDogCore::new(config).await?);

    // Start BearDog core
    beardog_core.start().await?;

    if matches.get_flag("api-only") {
        // Run API server only
        run_api_server(beardog_core).await?;
    } else {
        // Run full BearDog with all components
        if matches.get_flag("demo") {
            run_demo_mode(beardog_core).await?;
        } else {
            run_production_mode(beardog_core).await?;
        }
    }

    Ok(())
}

/// Run API server only
async fn run_api_server(beardog_core: Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🚀 Starting BearDog API Server");

    let api_server = BearDogApiServer::new(beardog_core.clone())?;

    // Start API server
    tokio::select! {
        result = api_server.start(&beardog_core.config().api.http.bind_address) => {
            if let Err(e) = result {
                error!("API server failed: {}", e);
                return Err(e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    Ok(())
}

/// Run in demo mode with example data
async fn run_demo_mode(beardog_core: Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🎯 Starting BearDog in Demo Mode - Showcasing Sprint 2 Features");

    // Initialize Sprint 2 components
    let api_server = BearDogApiServer::new(beardog_core.clone())?;
    let threat_detection = Arc::new(tokio::sync::RwLock::new(
        ThreatDetectionEngine::new(ConfigExt::into(
            beardog_core.config().threat_detection.clone(),
        ))
        .await?,
    ));
    let nestgate_adapter = Arc::new(
        NestGateAdapter::new(
            beardog_core.clone(),
            ConfigExt::into(
                beardog_core
                    .config()
                    .adapters
                    .external_systems
                    .rust_ecosystem
                    .nestgate
                    .clone(),
            ),
        )
        .await
        .map_err(|e| BearDogError::internal(format!("Failed to create NestGate adapter: {e}")))?,
    );
    let compliance_engine = Arc::new(
        ComplianceEngine::new(ConfigExt::into(beardog_core.config().compliance.clone())).await?,
    );

    // Initialize threat detection engine
    info!("🔍 Threat detection engine initialized");

    // Start API server in background
    let api_server_handle = {
        let api_server = api_server;
        let bind_address = beardog_core.config().api.http.bind_address.clone();
        tokio::spawn(async move {
            if let Err(e) = api_server.start(&bind_address).await {
                error!("API server failed: {}", e);
            }
        })
    };

    // Run demo scenarios
    info!("🎪 Running Demo Scenarios");

    // Demo 1: Threat Detection
    info!("📊 Demo 1: Threat Detection Engine");
    let demo_event = SecurityEvent {
        event_id: "demo-event-001".to_string(),
        event_type: "FileAccess".to_string(),
        timestamp: chrono::Utc::now(),
        source_ip: "192.168.1.100".to_string(),
        destination_ip: "10.0.0.1".to_string(),
        user_id: "demo-user".to_string(),
        user_agent: Some("Demo Agent".to_string()),
        data_size: 1024.0,
        location: Some("US".to_string()),
        file_hash: Some("demo123".to_string()),
        additional_data: std::collections::HashMap::from([(
            "resource".to_string(),
            "/etc/passwd".to_string(),
        )]),
    };

    // Convert SecurityEvent to HashMap for analyze_event
    let mut event_data =
        std::collections::HashMap::with_capacity(5 + demo_event.additional_data.len());
    event_data.insert("event_id".to_string(), demo_event.event_id);
    event_data.insert("event_type".to_string(), demo_event.event_type);
    event_data.insert("source_ip".to_string(), demo_event.source_ip);
    event_data.insert("destination_ip".to_string(), demo_event.destination_ip);
    event_data.insert("user_id".to_string(), demo_event.user_id);
    event_data.extend(demo_event.additional_data);

    match threat_detection
        .write()
        .await
        .analyze_event(&event_data)
        .await
    {
        Ok(results) => {
            info!("✅ Threat analysis completed:");
            info!("   Detected {} potential threats", results.len());
            for result in results.iter().take(3) {
                info!(
                    "   Threat: {} (Score: {})",
                    result.threat_type, result.score
                );
            }
        }
        Err(e) => warn!("⚠️  Threat analysis failed: {}", e),
    }

    // Demo 2: NestGate Integration
    info!("📊 Demo 2: NestGate ZFS Integration");
    match nestgate_adapter.generate_master_key("demo-owner").await {
        Ok(key) => {
            info!("✅ Master key generated:");
            info!("   Key ID: {}", key.id);
            info!("   Algorithm: {}", key.algorithm);
            info!("   Owner: {}", key.owner_id);
        }
        Err(e) => warn!("⚠️  Key generation failed: {}", e),
    }

    // Demo file operation
    let file_op = FileOperationRequest {
        operation: FileOperation::Read,
        source_path: "/data/demo-file.txt".into(),
        destination_path: None,
        user_id: "demo-user".to_string(),
        metadata: std::collections::HashMap::new(),
        priority: OperationPriority::Normal,
        provider_id: "demo-provider".to_string(),
    };

    match nestgate_adapter.perform_file_operation(file_op).await {
        Ok(result) => {
            info!("✅ File operation completed:");
            info!("   Operation ID: {}", result.operation_id);
            info!("   Success: {}", result.success);
        }
        Err(e) => warn!("⚠️  File operation failed: {}", e),
    }

    // Demo 3: Multi-Party Workflow Engine
    info!("📊 Demo 3: Multi-Party Workflow Engine");

    // Demo workflow: Key rotation requiring approval
    let workflow_request = WorkflowRequest {
        workflow_type: WorkflowType::KeyRotation,
        initiator: "demo-user".to_string(),
        target: WorkflowTarget::Key {
            key_id: "demo-key-001".to_string(),
        },
        parameters: {
            let mut params = std::collections::HashMap::new();
            params.insert("key_id".to_string(), serde_json::json!("demo-key-001"));
            params.insert(
                "reason".to_string(),
                serde_json::json!("Scheduled rotation"),
            );
            params
        },
        reason: "Quarterly key rotation for enhanced security".to_string(),
        priority: WorkflowPriority::Normal,
        metadata: std::collections::HashMap::new(),
    };

    match beardog_core
        .workflow_engine()
        .initiate_workflow(workflow_request)
        .await
    {
        Ok(workflow_response) => {
            info!("✅ Workflow initiated:");
            info!("   Workflow ID: {}", workflow_response.workflow_id);
            info!("   Status: {:?}", workflow_response.status);
            info!(
                "   Required Approvals: {}",
                workflow_response.required_approvals.required_approvals
            );
            info!(
                "   Pending Approvers: {:?}",
                workflow_response.pending_approvers
            );

            // Demo approval by admin1
            let approval_submission = ApprovalSubmission {
                workflow_id: workflow_response.workflow_id.clone(),
                approver: "admin1".to_string(),
                decision: ApprovalDecision::Approved,
                reason: Some("Approved for scheduled rotation".to_string()),
                signature: None,
                metadata: std::collections::HashMap::new(),
            };

            match beardog_core
                .workflow_engine()
                .submit_approval(approval_submission)
                .await
            {
                Ok(approval_response) => {
                    info!("✅ First approval submitted:");
                    info!("   Success: {}", approval_response.success);
                    info!("   Message: {}", approval_response.message);
                    info!(
                        "   Remaining Approvals: {}",
                        approval_response.remaining_approvals
                    );
                    info!(
                        "   Workflow Status: {:?}",
                        approval_response.workflow_status
                    );

                    // Demo second approval by admin2 if needed
                    if approval_response.remaining_approvals > 0 {
                        let second_approval = ApprovalSubmission {
                            workflow_id: workflow_response.workflow_id.clone(),
                            approver: "admin2".to_string(),
                            decision: ApprovalDecision::Approved,
                            reason: Some("Second approval for key rotation".to_string()),
                            signature: None,
                            metadata: std::collections::HashMap::new(),
                        };

                        match beardog_core
                            .workflow_engine()
                            .submit_approval(second_approval)
                            .await
                        {
                            Ok(final_approval) => {
                                info!("✅ Final approval submitted:");
                                info!("   Workflow Status: {:?}", final_approval.workflow_status);
                                if final_approval.remaining_approvals == 0 {
                                    info!("🎉 Workflow fully approved and executing!");
                                }
                            }
                            Err(e) => warn!("⚠️  Second approval failed: {}", e),
                        }
                    }
                }
                Err(e) => warn!("⚠️  Approval submission failed: {}", e),
            }
        }
        Err(e) => warn!("⚠️  Workflow initiation failed: {}", e),
    }

    // Demo 4: Compliance Monitoring
    info!("📊 Demo 4: Compliance Dashboard");
    let compliance_event = ComplianceEvent {
        id: "compliance-demo-001".to_string(),
        event_type: "data_access".to_string(),
        user_id: Some("demo-user".to_string()),
        resource: Some("personal_data".to_string()),
        data: std::collections::HashMap::new(),
        timestamp: chrono::Utc::now(),
        metadata: std::collections::HashMap::new(),
    };

    match compliance_engine.monitor_event(compliance_event).await {
        Ok(result) => {
            info!("✅ Compliance monitoring completed:");
            info!("   Event Compliant: {}", result.compliance_score > 0.5);
            info!("   Violations: {}", result.violations.len());
            info!("   Warnings: {}", result.warnings.len());
        }
        Err(e) => warn!("⚠️  Compliance monitoring failed: {}", e),
    }

    // Get compliance dashboard
    match compliance_engine.get_dashboard_data().await {
        Ok(dashboard) => {
            info!("✅ Compliance Dashboard Data:");
            info!(
                "   Overall Status: {:?}",
                dashboard.compliance_status.overall_status
            );
            info!(
                "   Compliance %: {:.1}%",
                dashboard.compliance_status.compliance_percentage
            );
            info!(
                "   Active Violations: {}",
                dashboard.compliance_status.active_violations
            );
            info!("   Recommendations: {}", dashboard.recommendations.len());
        }
        Err(e) => warn!("⚠️  Dashboard data retrieval failed: {}", e),
    }

    // Demo 4: API Health Check
    info!("📊 Demo 4: API Server Health");
    match beardog_core.health_check().await {
        Ok(health) => {
            info!("✅ System Health Check:");
            info!("   Status: {:?}", health.status);
            info!("   Components: {}", health.components.len());
            info!("   Uptime: {:?}", health.uptime);
        }
        Err(e) => warn!("⚠️  Health check failed: {}", e),
    }

    // Display API endpoints
    info!(
        "🌐 BearDog API Server is running on: {}",
        beardog_core.config().api.http.bind_address
    );
    info!("📋 Available API Endpoints:");
    info!("   GET  /health           - Health check");
    info!("   GET  /status           - Detailed system status");
    info!("   POST /keys             - Generate encryption key");
    info!("   POST /keys/:id/encrypt - Encrypt data");
    info!("   POST /keys/:id/decrypt - Decrypt data");
    info!("   POST /encrypt          - Encrypt with default key");
    info!("   POST /decrypt          - Decrypt with default key");
    info!("   GET  /auth/health      - Authentication health");

    info!("🎯 Demo Mode Complete - BearDog Sprint 2 Features Demonstrated!");
    info!("💡 Try these curl commands:");
    info!(
        "   curl http://{}/health",
        beardog_core.config().api.http.bind_address
    );
    info!(
        "   curl http://{}/status",
        beardog_core.config().api.http.bind_address
    );

    // Wait for shutdown signal
    info!("Press Ctrl+C to shutdown...");
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
        _ = api_server_handle => {
            warn!("API server stopped unexpectedly");
        }
    }

    Ok(())
}

/// Run in production mode
async fn run_production_mode(beardog_core: Arc<BearDogCore>) -> BearDogResult<()> {
    info!("🏭 Starting BearDog in Production Mode");

    // Initialize production components
    let api_server = BearDogApiServer::new(beardog_core.clone())?;
    let _threat_detection = Arc::new(tokio::sync::RwLock::new(
        ThreatDetectionEngine::new(ConfigExt::into(
            beardog_core.config().threat_detection.clone(),
        ))
        .await?,
    ));
    let compliance_engine = Arc::new(
        ComplianceEngine::new(ConfigExt::into(beardog_core.config().compliance.clone())).await?,
    );

    // Initialize monitoring services
    info!("🔍 Monitoring services initialized");

    // Start compliance monitoring
    info!("🔍 Starting compliance monitoring service...");
    let compliance_handle = {
        let compliance_engine = compliance_engine.clone();
        tokio::spawn(async move {
            loop {
                // Monitor compliance continuously
                if let Err(e) = compliance_engine.perform_periodic_check().await {
                    warn!("Compliance check failed: {}", e);
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
                // 5 minutes
            }
        })
    };

    info!("🌐 Starting API server...");
    // Start API server
    tokio::select! {
        result = api_server.start(&beardog_core.config().api.http.bind_address) => {
            if let Err(e) = result {
                error!("API server failed: {}", e);
                return Err(e);
            }
        }
        _ = signal::ctrl_c() => {
            info!("Received shutdown signal");
            compliance_handle.abort(); // Clean shutdown
        }
    }

    // Graceful shutdown
    info!("🛑 Shutting down BearDog gracefully...");
    beardog_core.stop().await?;
    info!("✅ BearDog stopped successfully");

    Ok(())
}

/// Extension trait to convert config types
trait ConfigExt {
    type Output;
    fn into(self) -> Self::Output;
}

impl ConfigExt for beardog_config::ThreatDetectionConfig {
    type Output = beardog_threat::threat::types::ThreatDetectionConfig;

    fn into(self) -> Self::Output {
        beardog_threat::threat::types::ThreatDetectionConfig {
            enabled: self.enabled,
            rules_path: "/etc/beardog/rules".to_string(),
            monitor_paths: vec![
                "/etc".into(),
                "/var/log".into(),
                beardog_config::constants::storage::DEFAULT_CACHE_DIR.into(),
            ],
            alert_threshold: beardog_config::constants::threat_detection::DEFAULT_ANOMALY_THRESHOLD,
            cache_size: beardog_config::constants::performance::DEFAULT_CACHE_SIZE,
            monitoring_interval: 60,
            real_time_detection: true,
            threat_threshold: 70,
            automated_response: true,
            max_alerts_per_minute: 100,
            ml_enhancement: true,
            threat_feeds: vec!["https://feeds.example.com/threats".to_string()],
            auto_quarantine: true,
            notification_endpoints: vec!["admin@example.com".to_string()],
        }
    }
}

impl ConfigExt for beardog_config::ComplianceConfig {
    type Output = beardog_compliance::ComplianceConfig;

    fn into(self) -> Self::Output {
        beardog_compliance::ComplianceConfig {
            enabled_standards: self
                .enabled_standards
                .into_iter()
                .filter_map(|s| match s.as_str() {
                    "GDPR" => Some(beardog_compliance::ComplianceStandard::GDPR),
                    "HIPAA" => Some(beardog_compliance::ComplianceStandard::HIPAA),
                    "SOX" => Some(beardog_compliance::ComplianceStandard::SOX),
                    "PCI_DSS" => Some(beardog_compliance::ComplianceStandard::PCI_DSS),
                    "FedRAMP" => Some(beardog_compliance::ComplianceStandard::FedRAMP),
                    _ => None,
                })
                .collect(),
            monitoring_interval: chrono::Duration::from_std(self.monitoring_interval)
                .unwrap_or(chrono::Duration::minutes(5)),
            audit_retention: chrono::Duration::from_std(self.audit_retention)
                .unwrap_or(chrono::Duration::days(365)),
            dashboard_refresh_interval: chrono::Duration::minutes(1),
            reporting: beardog_compliance::ReportingConfig {
                auto_generate: true,
                generation_interval: chrono::Duration::days(30),
                storage_path: "/var/log/beardog/compliance".to_string(),
                formats: vec![
                    beardog_compliance::ReportFormat::PDF,
                    beardog_compliance::ReportFormat::JSON,
                ],
            },
        }
    }
}

impl ConfigExt for Option<beardog_config::RustProjectConfig> {
    type Output = beardog_adapters::adapters::nestgate::NestGateConfig;

    fn into(self) -> Self::Output {
        use beardog_adapters::adapters::nestgate::{
            AccessLevel, AuditConfig, AuditStorageBackend, AuthConfig, AuthMethod, PolicyConfig,
            ZfsConfig,
        };

        match self {
            Some(config) => beardog_adapters::adapters::nestgate::NestGateConfig {
                enabled: config.enabled,
                api_endpoint: config.endpoint,
                provider_name: "beardog".to_string(),
                auth: AuthConfig {
                    method: AuthMethod::ApiKey,
                    api_key: config.auth.and_then(|a| a.secret).unwrap_or_default(),
                    client_cert_path: None,
                    client_key_path: None,
                    ca_cert_path: None,
                    token_refresh_interval: 3600,
                },
                zfs: ZfsConfig {
                    default_algorithm: "AES-256-GCM".to_string(),
                    wrap_algorithm: "AES-KW".to_string(),
                    pool_name: "beardog".to_string(),
                    dataset_prefix: "beardog/data".to_string(),
                    compression: "lz4".to_string(),
                    deduplication: true,
                    record_size: "128K".to_string(),
                },
                policies: PolicyConfig {
                    enabled_policies: vec!["default".to_string()],
                    default_access_level: AccessLevel::ReadWrite,
                    require_approval: vec!["delete".to_string(), "move".to_string()],
                    refresh_interval: 300,
                    external_provider: None,
                },
                audit: AuditConfig {
                    enabled: true,
                    retention_days: 365,
                    log_all_operations: true,
                    storage_backend: AuditStorageBackend::Local,
                    encrypt_logs: true,
                },
                capabilities: vec![
                    "file_operations".to_string(),
                    "key_management".to_string(),
                    "zfs_integration".to_string(),
                    "policy_enforcement".to_string(),
                    "audit_logging".to_string(),
                    "beardog_specific".to_string(),
                ],
            },
            None => beardog_adapters::adapters::nestgate::NestGateConfig {
                enabled: false,
                api_endpoint: "https://nestgate.internal:8443".to_string(),
                provider_name: "beardog".to_string(),
                auth: AuthConfig {
                    method: AuthMethod::ApiKey,
                    api_key: String::new(),
                    client_cert_path: None,
                    client_key_path: None,
                    ca_cert_path: None,
                    token_refresh_interval: 3600,
                },
                zfs: ZfsConfig {
                    default_algorithm: "AES-256-GCM".to_string(),
                    wrap_algorithm: "AES-KW".to_string(),
                    pool_name: "beardog".to_string(),
                    dataset_prefix: "beardog/data".to_string(),
                    compression: "lz4".to_string(),
                    deduplication: true,
                    record_size: "128K".to_string(),
                },
                policies: PolicyConfig {
                    enabled_policies: vec!["default".to_string()],
                    default_access_level: AccessLevel::ReadWrite,
                    require_approval: vec!["delete".to_string(), "move".to_string()],
                    refresh_interval: 300,
                    external_provider: None,
                },
                audit: AuditConfig {
                    enabled: false,
                    retention_days: 365,
                    log_all_operations: false,
                    storage_backend: AuditStorageBackend::Local,
                    encrypt_logs: false,
                },
                capabilities: vec![
                    "file_operations".to_string(),
                    "key_management".to_string(),
                    "zfs_integration".to_string(),
                    "policy_enforcement".to_string(),
                    "audit_logging".to_string(),
                    "beardog_specific".to_string(),
                ],
            },
        }
    }
}
