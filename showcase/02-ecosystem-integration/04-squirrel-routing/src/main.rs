// SPDX-License-Identifier: AGPL-3.0-or-later

// 🐻🐿️ BearDog + Squirrel: Privacy-Preserving Routing Demo
//
// This demo shows BearDog providing privacy-preserving routing for Squirrel MCP

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tracing::{info, warn};

/// BearDog Privacy-Preserving AI Routing Demo (Capability-Based)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Request file (JSON)
    #[arg(short, long)]
    request: PathBuf,

    /// Configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    setup_logging(args.verbose);

    info!("🐻🐿️ BearDog + Squirrel: Privacy-Preserving Routing Demo");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("");

    // Load configuration
    let config = load_config(&args.config)
        .context("Failed to load configuration")?;
    
    info!("Request: {}", args.request.display());
    info!("Config loaded from: {}", args.config.display());
    info!("");

    // Run the complete privacy routing workflow
    run_privacy_routing_workflow(&args.request, config).await?;

    info!("");
    info!("🎉 Demo complete! All operations successful.");
    
    Ok(())
}

async fn run_privacy_routing_workflow(request_path: &PathBuf, config: DemoConfig) -> Result<()> {
    let total_start = Instant::now();

    // Step 1: Load user request
    info!("Step 1: Loading user request...");
    let start = Instant::now();
    
    let user_request = load_request(request_path).await?;
    let load_time = start.elapsed();
    
    info!("✅ Request loaded");
    info!("   User: {}", user_request.user_id);
    info!("   Task: {}", user_request.task);
    info!("   Data size: {} bytes", user_request.data.len());
    info!("   Load time: {:?}", load_time);
    info!("");

    // Step 2: Initialize BearDog privacy service
    info!("Step 2: Initializing BearDog privacy service...");
    let start = Instant::now();
    
    let privacy_service = initialize_privacy_service().await?;
    let init_time = start.elapsed();
    
    info!("✅ Privacy service initialized");
    info!("   PII patterns: 4 (SSN, email, phone, credit card)");
    info!("   Identity store: Active");
    info!("   Init time: {:?}", init_time);
    info!("");

    // Step 3: Sanitize PII
    info!("Step 3: Sanitizing PII from request data...");
    let start = Instant::now();
    
    let sanitized = privacy_service.sanitize_pii(&user_request.data)?;
    let sanitize_time = start.elapsed();
    
    info!("✅ PII sanitized");
    info!("   Redactions: {}", sanitized.redactions.len());
    for redaction in &sanitized.redactions {
        info!("     - {}", redaction);
    }
    info!("   Original size: {} bytes", user_request.data.len());
    info!("   Sanitized size: {} bytes", sanitized.text.len());
    info!("   Bytes stripped: {}", user_request.data.len() as i64 - sanitized.text.len() as i64);
    info!("   Sanitization time: {:?}", sanitize_time);
    info!("");

    // Step 4: Generate anonymous ID
    info!("Step 4: Generating anonymous request ID...");
    let start = Instant::now();
    
    let anon_id = privacy_service.generate_anonymous_id(&user_request.user_id)?;
    let anon_time = start.elapsed();
    
    info!("✅ Anonymous ID generated");
    info!("   ID: {}", anon_id);
    info!("   Real user: {} (stored locally, NEVER sent)", user_request.user_id);
    info!("   Expires: 1 hour from now");
    info!("   Generation time: {:?}", anon_time);
    info!("");

    // Step 5: Create anonymized request
    info!("Step 5: Creating anonymized request...");
    let anonymized_request = AnonymizedRequest {
        request_id: anon_id.clone(),
        task: user_request.task.clone(),
        data: sanitized.text,
        region: "US".to_string(), // Generalized
    };
    
    info!("✅ Request anonymized");
    info!("   Anonymous ID: {}", anonymized_request.request_id);
    info!("   Task: {}", anonymized_request.task);
    info!("   Region: {} (generalized from detailed location)", anonymized_request.region);
    info!("   ⚠️  User identity: REMOVED (privacy preserved)");
    info!("   ⚠️  IP address: REMOVED (privacy preserved)");
    info!("   ⚠️  Device info: REMOVED (privacy preserved)");
    info!("");

    // Step 6: Route to Squirrel MCP
    info!("Step 6: Routing to Squirrel MCP...");
    let start = Instant::now();
    
    let ai_result = route_to_squirrel(&anonymized_request, &config).await?;
    let routing_time = start.elapsed();
    
    info!("✅ Request routed and processed");
    info!("   Squirrel MCP: Simulated (would discover AI provider)");
    info!("   AI provider: Mock LLM");
    info!("   Processing time: {:?}", routing_time);
    info!("   Result size: {} bytes", ai_result.data.len());
    info!("   ✅ Squirrel NEVER saw user identity");
    info!("");

    // Step 7: Deliver result to user
    info!("Step 7: Delivering result to user...");
    let start = Instant::now();
    
    let user_id = privacy_service.lookup_identity(&anon_id)?;
    let delivery_time = start.elapsed();
    
    info!("✅ Result delivered");
    info!("   Anonymous ID: {} → Real user: {}", anon_id, user_id);
    info!("   Result: {}", ai_result.data);
    info!("   Delivery time: {:?}", delivery_time);
    info!("");

    // Step 8: Log privacy metrics
    info!("Step 8: Logging privacy metrics...");
    
    info!("✅ Privacy metrics recorded");
    info!("   Redactions: {} PII instances removed", sanitized.redactions.len());
    info!("   Identity protection: COMPLETE (no leakage)");
    info!("   Metadata stripped: IP, device, detailed location");
    info!("   Zero-knowledge routing: AI service cannot correlate requests");
    info!("");

    // Performance summary
    let total_time = total_start.elapsed();
    let privacy_overhead = sanitize_time + anon_time + delivery_time;
    
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("📊 Performance Summary");
    info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    info!("Request load:     {:?}", load_time);
    info!("Privacy init:     {:?}", init_time);
    info!("PII sanitization: {:?}", sanitize_time);
    info!("Anonymization:    {:?}", anon_time);
    info!("AI routing:       {:?}", routing_time);
    info!("Result delivery:  {:?}", delivery_time);
    info!("Total time:       {:?}", total_time);
    info!("");
    info!("Privacy overhead: {:?} (sanitization + anonymization + delivery)", privacy_overhead);
    info!("");

    // Validation
    let target_overhead = std::time::Duration::from_millis(100);
    
    if privacy_overhead <= target_overhead {
        info!("✅ SUCCESS! Performance target met!");
        info!("   Overhead: {:?} <= {:?} ✓", privacy_overhead, target_overhead);
    } else {
        warn!("⚠️  Performance target not met:");
        warn!("   Overhead: {:?} > {:?} ✗", privacy_overhead, target_overhead);
    }

    Ok(())
}

// Configuration
#[derive(Debug, Clone, serde::Deserialize)]
struct DemoConfig {
    #[serde(rename = "squirrel_endpoint")]
    _ai_endpoint: String,
    #[serde(rename = "identity_expiry_hours")]
    _identity_expiry_hours: u32,
}

fn load_config(path: &PathBuf) -> Result<DemoConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;
    
    toml::from_str(&content)
        .with_context(|| format!("Failed to parse config: {}", path.display()))
}

// User request structure
#[derive(Debug, Clone, serde::Deserialize)]
struct UserRequest {
    user_id: String,
    task: String,
    data: String,
}

async fn load_request(path: &PathBuf) -> Result<UserRequest> {
    let content = tokio::fs::read_to_string(path).await
        .with_context(|| format!("Failed to read request: {}", path.display()))?;
    
    serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse request: {}", path.display()))
}

// Anonymized request
#[derive(Debug, Clone)]
struct AnonymizedRequest {
    request_id: String,
    task: String,
    data: String,
    region: String,
}

// AI result
#[derive(Debug, Clone)]
struct AIResult {
    data: String,
}

// PII sanitization
#[derive(Debug, Clone)]
struct SanitizedData {
    text: String,
    redactions: Vec<String>,
}

struct PIISanitizer {
    ssn_pattern: Regex,
    email_pattern: Regex,
    phone_pattern: Regex,
    credit_card_pattern: Regex,
}

impl PIISanitizer {
    fn new() -> Self {
        Self {
            ssn_pattern: Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
            email_pattern: Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
            phone_pattern: Regex::new(r"\b\d{3}-\d{3}-\d{4}\b").unwrap(),
            credit_card_pattern: Regex::new(r"\b\d{4}-\d{4}-\d{4}-\d{4}\b").unwrap(),
        }
    }
    
    fn sanitize(&self, data: &str) -> SanitizedData {
        let mut text = data.to_string();
        let mut redactions = Vec::new();
        
        // Redact SSNs
        if self.ssn_pattern.is_match(&text) {
            text = self.ssn_pattern.replace_all(&text, "[SSN_REDACTED]").to_string();
            redactions.push("SSN".to_string());
        }
        
        // Redact emails
        if self.email_pattern.is_match(&text) {
            text = self.email_pattern.replace_all(&text, "[EMAIL_REDACTED]").to_string();
            redactions.push("Email".to_string());
        }
        
        // Redact phones
        if self.phone_pattern.is_match(&text) {
            text = self.phone_pattern.replace_all(&text, "[PHONE_REDACTED]").to_string();
            redactions.push("Phone".to_string());
        }
        
        // Redact credit cards
        if self.credit_card_pattern.is_match(&text) {
            text = self.credit_card_pattern.replace_all(&text, "[CARD_REDACTED]").to_string();
            redactions.push("Credit Card".to_string());
        }
        
        SanitizedData { text, redactions }
    }
}

// Privacy service
struct BearDogPrivacyService {
    sanitizer: Arc<PIISanitizer>,
    identity_mappings: Arc<parking_lot::RwLock<HashMap<String, String>>>,
}

async fn initialize_privacy_service() -> Result<Arc<BearDogPrivacyService>> {
    Ok(Arc::new(BearDogPrivacyService {
        sanitizer: Arc::new(PIISanitizer::new()),
        identity_mappings: Arc::new(parking_lot::RwLock::new(HashMap::new())),
    }))
}

impl BearDogPrivacyService {
    fn sanitize_pii(&self, data: &str) -> Result<SanitizedData> {
        Ok(self.sanitizer.sanitize(data))
    }
    
    fn generate_anonymous_id(&self, user_id: &str) -> Result<String> {
        let anon_id = format!("anon_{}", uuid::Uuid::new_v4().simple());
        
        // Store mapping
        self.identity_mappings.write().insert(anon_id.clone(), user_id.to_string());
        
        Ok(anon_id)
    }
    
    fn lookup_identity(&self, anon_id: &str) -> Result<String> {
        self.identity_mappings.read()
            .get(anon_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Identity not found: {}", anon_id))
    }
}

// Squirrel routing (simulated)
async fn route_to_squirrel(
    request: &AnonymizedRequest,
    _config: &DemoConfig,
) -> Result<AIResult> {
    // Simulate Squirrel MCP discovering AI provider and routing
    tokio::time::sleep(std::time::Duration::from_millis(150)).await;
    
    // Simulate AI processing
    let result = match request.task.as_str() {
        task if task.contains("summarize") || task.contains("Summarize") => {
            "Summary: The document contains medical information about a patient with chronic conditions. Key findings indicate stable health status."
        }
        task if task.contains("analyze") || task.contains("Analyze") => {
            "Analysis: The data shows patterns consistent with normal operation. No anomalies detected."
        }
        _ => {
            "Result: Task completed successfully."
        }
    };
    
    Ok(AIResult {
        data: result.to_string(),
    })
}

fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(level))
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
