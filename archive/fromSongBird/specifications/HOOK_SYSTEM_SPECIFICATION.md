# Songbird Hook System Specification - BearDog Integration

**Version:** 1.0  
**Date:** December 2024  
**Target:** BearDog Security Manager Hook Integration  
**Status:** Production-Ready Hook System - 12 Event Types Available

## 🎯 Overview

Songbird's hook system provides comprehensive event-driven extensibility for security managers. The system supports 12 different event types with configurable filtering, priority-based execution, and comprehensive error handling.

## 🪝 Core Hook System Architecture

### Universal Event Hook Interface

```rust
#[async_trait]
pub trait EventHook: Send + Sync {
    /// Hook name for identification
    fn name(&self) -> &str;
    
    /// Hook version for compatibility
    fn version(&self) -> &str;
    
    /// Hook priority for ordering (lower = earlier)
    fn priority(&self) -> u32;
    
    /// Check if hook is enabled
    fn is_enabled(&self) -> bool;
    
    /// Initialize the hook
    async fn initialize(&mut self, context: &HookContext) -> Result<()>;
    
    /// Handle an orchestrator event
    async fn handle_event(&self, event: &OrchestratorEvent) -> Result<HookResult>;
    
    /// Cleanup hook resources
    async fn cleanup(&self) -> Result<()>;
    
    /// Get hook configuration
    fn get_config(&self) -> HookConfig;
}
```

### Hook Execution Context

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookContext {
    /// Orchestrator instance ID
    pub orchestrator_id: String,
    
    /// Hook configuration
    pub config: HashMap<String, serde_json::Value>,
    
    /// Environment variables
    pub environment: HashMap<String, String>,
    
    /// Shared context between hooks
    pub shared_context: HashMap<String, serde_json::Value>,
}
```

### Hook Execution Result

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResult {
    /// Whether the hook execution was successful
    pub success: bool,
    
    /// Whether to continue processing other hooks
    pub continue_chain: bool,
    
    /// Whether to allow the original operation to continue
    pub allow_operation: bool,
    
    /// Optional modifications to the event/context
    pub modifications: Option<HashMap<String, serde_json::Value>>,
    
    /// Log messages from the hook
    pub log_messages: Vec<String>,
    
    /// Execution duration
    pub execution_time_ms: u64,
    
    /// Error message if execution failed
    pub error: Option<String>,
}
```

## 📊 Event Types Specification

### 1. Service Lifecycle Events

**ServiceRegistering**
```rust
ServiceRegistering {
    service_info: ServiceInfo,
    timestamp: DateTime<Utc>,
}
```
- **When:** Before a service is registered with the orchestrator
- **Use Case:** Security compliance validation, service approval workflows
- **Can Block:** Yes - return `allow_operation: false` to reject registration

**ServiceRegistered**
```rust
ServiceRegistered {
    service_id: String,
    service_info: ServiceInfo,
    timestamp: DateTime<Utc>,
}
```
- **When:** After successful service registration
- **Use Case:** Audit logging, notification systems, monitoring setup
- **Can Block:** No - informational event

**ServiceStarting**
```rust
ServiceStarting {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** Before a service starts execution
- **Use Case:** Security checks, resource allocation, dependency validation
- **Can Block:** Yes - return `allow_operation: false` to prevent startup

**ServiceStarted**
```rust
ServiceStarted {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** After successful service startup
- **Use Case:** Health monitoring initialization, load balancer registration
- **Can Block:** No - informational event

**ServiceStopping**
```rust
ServiceStopping {
    service_id: String,
    reason: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** Before a service stops execution
- **Use Case:** Graceful shutdown procedures, data persistence, cleanup warnings
- **Can Block:** Yes - return `allow_operation: false` to prevent shutdown

**ServiceStopped**
```rust
ServiceStopped {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** After service has stopped
- **Use Case:** Cleanup operations, audit logging, resource deallocation
- **Can Block:** No - informational event

**ServiceUnregistering**
```rust
ServiceUnregistering {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** Before a service is unregistered from the orchestrator
- **Use Case:** Final security checks, data export, dependency cleanup
- **Can Block:** Yes - return `allow_operation: false` to prevent unregistration

**ServiceUnregistered**
```rust
ServiceUnregistered {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** After service unregistration
- **Use Case:** Final audit logging, monitoring cleanup
- **Can Block:** No - informational event

### 2. Request Lifecycle Events

**RequestReceived**
```rust
RequestReceived {
    service_id: String,
    request: ServiceRequest,
    timestamp: DateTime<Utc>,
}
```
- **When:** When a request is received by a service
- **Use Case:** Authentication, authorization, rate limiting, threat detection
- **Can Block:** Yes - return `allow_operation: false` to reject request

**RequestProcessing**
```rust
RequestProcessing {
    service_id: String,
    request_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** During request processing
- **Use Case:** Real-time monitoring, performance tracking
- **Can Block:** No - informational event

**RequestCompleted**
```rust
RequestCompleted {
    service_id: String,
    request_id: String,
    response: ServiceResponse,
    duration_ms: u64,
    timestamp: DateTime<Utc>,
}
```
- **When:** After successful request completion
- **Use Case:** Performance metrics, audit logging, response validation
- **Can Block:** No - informational event

**RequestFailed**
```rust
RequestFailed {
    service_id: String,
    request_id: String,
    error: String,
    duration_ms: u64,
    timestamp: DateTime<Utc>,
}
```
- **When:** When request processing fails
- **Use Case:** Error tracking, incident detection, failure analysis
- **Can Block:** No - informational event

### 3. Health and Monitoring Events

**HealthCheckStarted**
```rust
HealthCheckStarted {
    service_id: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** Before health check execution
- **Use Case:** Health monitoring coordination, dependency checks
- **Can Block:** Yes - return `allow_operation: false` to skip health check

**HealthCheckCompleted**
```rust
HealthCheckCompleted {
    service_id: String,
    healthy: bool,
    details: HashMap<String, serde_json::Value>,
    timestamp: DateTime<Utc>,
}
```
- **When:** After health check completion
- **Use Case:** Health status tracking, alerting, load balancer updates
- **Can Block:** No - informational event

**MetricsCollected**
```rust
MetricsCollected {
    service_id: Option<String>,
    metrics: HashMap<String, f64>,
    timestamp: DateTime<Utc>,
}
```
- **When:** When metrics are collected
- **Use Case:** Performance monitoring, capacity planning, alerting
- **Can Block:** No - informational event

### 4. Discovery and Configuration Events

**ServiceDiscovered**
```rust
ServiceDiscovered {
    service_info: ServiceInfo,
    discovery_source: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** When a service is discovered by service discovery
- **Use Case:** Dynamic service registration, security validation
- **Can Block:** No - informational event

**ServiceLost**
```rust
ServiceLost {
    service_id: String,
    discovery_source: String,
    timestamp: DateTime<Utc>,
}
```
- **When:** When a service is no longer discoverable
- **Use Case:** Cleanup operations, failure detection, alerting
- **Can Block:** No - informational event

**ConfigurationChanged**
```rust
ConfigurationChanged {
    config_section: String,
    old_config: serde_json::Value,
    new_config: serde_json::Value,
    timestamp: DateTime<Utc>,
}
```
- **When:** When configuration changes occur
- **Use Case:** Security validation, audit logging, policy enforcement
- **Can Block:** Yes - return `allow_operation: false` to reject config changes

### 5. Error and Custom Events

**ErrorOccurred**
```rust
ErrorOccurred {
    error_type: String,
    error_message: String,
    service_id: Option<String>,
    context: HashMap<String, serde_json::Value>,
    timestamp: DateTime<Utc>,
}
```
- **When:** When errors occur in the system
- **Use Case:** Incident detection, security monitoring, error analysis
- **Can Block:** No - informational event

**Custom**
```rust
Custom {
    event_type: String,
    data: HashMap<String, serde_json::Value>,
    timestamp: DateTime<Utc>,
}
```
- **When:** Custom events triggered by services or hooks
- **Use Case:** Application-specific monitoring, custom workflows
- **Can Block:** Depends on hook implementation

## 🔧 Hook Configuration System

### Hook Configuration Structure

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HookConfig {
    /// Hook-specific settings
    pub settings: HashMap<String, serde_json::Value>,
    
    /// Event filter - which events this hook cares about
    pub event_filter: EventFilter,
    
    /// Execution settings
    pub execution: ExecutionConfig,
    
    /// Retry settings
    pub retry: RetryConfig,
}
```

### Event Filtering

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventFilter {
    /// Event types to process (empty = all)
    pub event_types: Vec<String>,
    
    /// Service IDs to process (empty = all)
    pub service_ids: Vec<String>,
    
    /// Custom filter conditions
    pub conditions: Vec<FilterCondition>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilterCondition {
    pub field: String,
    pub operator: FilterOperator,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
    In,
    NotIn,
}
```

### Execution Configuration

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExecutionConfig {
    /// Whether to execute asynchronously
    pub async_execution: bool,
    
    /// Maximum execution time
    pub timeout_ms: u64,
    
    /// Whether to log execution details
    pub log_execution: bool,
    
    /// Whether to measure performance
    pub measure_performance: bool,
}
```

### Retry Configuration

```rust
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryConfig {
    /// Enable retries on failure
    pub enabled: bool,
    
    /// Maximum retry attempts
    pub max_attempts: u32,
    
    /// Delay between retries
    pub retry_delay_ms: u64,
    
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
}
```

## 🚀 Hook Manager Interface

```rust
#[async_trait]
pub trait HookManager: Send + Sync {
    /// Register a new hook
    async fn register_hook(&mut self, hook: Box<dyn EventHook>) -> Result<()>;
    
    /// Unregister a hook by name
    async fn unregister_hook(&mut self, hook_name: &str) -> Result<()>;
    
    /// Get list of registered hooks
    fn list_hooks(&self) -> Vec<HookInfo>;
    
    /// Execute hooks for an event
    async fn execute_hooks(&self, event: &OrchestratorEvent) -> Result<Vec<HookResult>>;
    
    /// Enable/disable a hook
    async fn set_hook_enabled(&mut self, hook_name: &str, enabled: bool) -> Result<()>;
    
    /// Get hook statistics
    async fn get_hook_stats(&self) -> Result<HashMap<String, HookStats>>;
    
    /// Cleanup all hooks
    async fn cleanup_all(&self) -> Result<()>;
}
```

### Hook Statistics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_execution_time_ms: f64,
    pub total_execution_time_ms: u64,
    pub last_execution: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
}
```

## 📋 BearDog Integration Examples

### Security Monitoring Hook

```rust
pub struct BearDogSecurityHook {
    name: String,
    config: BearDogHookConfig,
    beardog_client: Arc<BearDogClient>,
}

#[async_trait]
impl EventHook for BearDogSecurityHook {
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { "1.0.0" }
    fn priority(&self) -> u32 { 100 } // High priority for security
    fn is_enabled(&self) -> bool { self.config.enabled }
    
    async fn initialize(&mut self, context: &HookContext) -> Result<()> {
        // Initialize BearDog connection
        self.beardog_client.connect().await?;
        
        // Register with BearDog monitoring
        self.beardog_client.register_songbird_instance(
            &context.orchestrator_id
        ).await?;
        
        Ok(())
    }
    
    async fn handle_event(&self, event: &OrchestratorEvent) -> Result<HookResult> {
        let start_time = Instant::now();
        
        match event {
            OrchestratorEvent::RequestReceived { service_id, request, timestamp } => {
                // Extract security context
                let security_context = SecurityContext {
                    service_id: service_id.clone(),
                    request_id: request.id.clone(),
                    client_ip: request.headers.get("X-Real-IP").cloned(),
                    user_agent: request.headers.get("User-Agent").cloned(),
                    timestamp: *timestamp,
                };
                
                // Send to BearDog for analysis
                let threat_level = self.beardog_client
                    .analyze_request(&security_context)
                    .await?;
                
                // Handle threats
                match threat_level {
                    ThreatLevel::High | ThreatLevel::Critical => {
                        self.beardog_client.trigger_incident(
                            &security_context,
                            threat_level
                        ).await?;
                        
                        Ok(HookResult {
                            success: true,
                            continue_chain: false,
                            allow_operation: false, // Block high-threat requests
                            log_messages: vec![
                                format!("Blocked high-threat request: {:?}", threat_level)
                            ],
                            execution_time_ms: start_time.elapsed().as_millis() as u64,
                            ..Default::default()
                        })
                    },
                    _ => {
                        // Log the request but allow it
                        self.beardog_client.log_request(&security_context).await?;
                        
                        Ok(HookResult {
                            success: true,
                            continue_chain: true,
                            allow_operation: true,
                            execution_time_ms: start_time.elapsed().as_millis() as u64,
                            ..Default::default()
                        })
                    }
                }
            },
            
            OrchestratorEvent::ServiceRegistering { service_info, timestamp } => {
                // Validate service security compliance
                let compliance_result = self.beardog_client
                    .check_service_compliance(service_info)
                    .await?;
                
                if !compliance_result.is_compliant() {
                    Ok(HookResult {
                        success: true,
                        continue_chain: true,
                        allow_operation: false,
                        log_messages: vec![
                            format!("Service failed compliance: {:?}", 
                                compliance_result.violations)
                        ],
                        execution_time_ms: start_time.elapsed().as_millis() as u64,
                        ..Default::default()
                    })
                } else {
                    Ok(HookResult::allow_continue())
                }
            },
            
            _ => Ok(HookResult::allow_continue()),
        }
    }
    
    async fn cleanup(&self) -> Result<()> {
        self.beardog_client.disconnect().await
    }
    
    fn get_config(&self) -> HookConfig {
        HookConfig {
            settings: self.config.to_settings(),
            event_filter: EventFilter {
                event_types: vec![
                    "RequestReceived".to_string(),
                    "ServiceRegistering".to_string(),
                    "ErrorOccurred".to_string(),
                ],
                service_ids: vec![], // Monitor all services
                conditions: vec![],
            },
            execution: ExecutionConfig {
                async_execution: true,
                timeout_ms: 5000, // 5 second timeout
                log_execution: true,
                measure_performance: true,
            },
            retry: RetryConfig {
                enabled: true,
                max_attempts: 3,
                retry_delay_ms: 1000,
                backoff_multiplier: 2.0,
            },
        }
    }
}
```

## 🎯 Best Practices for BearDog Integration

### 1. Performance Considerations

- **Async Execution:** Use `async_execution: true` for non-blocking operations
- **Timeouts:** Set reasonable timeouts (5-10 seconds) for external calls
- **Batching:** Batch audit events to reduce network overhead
- **Caching:** Cache security decisions to improve performance

### 2. Error Handling

- **Graceful Degradation:** Continue operation even if BearDog is unavailable
- **Retry Logic:** Implement exponential backoff for transient failures
- **Circuit Breaker:** Use circuit breaker pattern for BearDog connectivity
- **Logging:** Comprehensive logging for debugging and monitoring

### 3. Security

- **Secure Communication:** Always use TLS for BearDog communication
- **Authentication:** Implement proper API key management
- **Input Validation:** Validate all data before sending to BearDog
- **Rate Limiting:** Implement rate limiting to prevent DoS

### 4. Monitoring

- **Hook Statistics:** Monitor hook execution times and success rates
- **Health Checks:** Implement health checks for BearDog connectivity
- **Alerting:** Set up alerts for hook failures or security incidents
- **Metrics:** Export hook metrics to monitoring systems

## 📊 Production Deployment Checklist

### Pre-Deployment
- [ ] Implement all required hook interfaces
- [ ] Test hook registration and execution
- [ ] Validate event filtering and configuration
- [ ] Performance test with expected load
- [ ] Security review of hook implementation

### Deployment
- [ ] Deploy hooks with proper configuration
- [ ] Monitor hook execution and performance
- [ ] Validate security event processing
- [ ] Test incident response workflows
- [ ] Verify audit log completeness

### Post-Deployment
- [ ] Monitor hook statistics and performance
- [ ] Validate security monitoring effectiveness
- [ ] Review and tune hook configurations
- [ ] Document operational procedures
- [ ] Train operations team on hook system

## 📚 Reference Files

- **Hook Traits:** `src/traits/hooks.rs` (449 lines)
- **Hook Manager:** Implementation in core orchestrator
- **Example Hooks:** `examples/` directory contains working examples
- **Test Cases:** `tests/` directory contains comprehensive hook tests 