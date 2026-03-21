// SPDX-License-Identifier: AGPL-3.0-only

//! Delegated keys with optional time, weekday, CPU, and memory constraints.

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use beardog_types::constraints::{
    Constraint, ConstraintContext,
    builtin::{
        CompositeConstraint, CpuQuotaConstraint, ExpiryConstraint, MemoryQuotaConstraint,
        TimeRangeConstraint, WeekdayConstraint,
    },
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Delegation constraints (simplified for storage)
///
/// This struct is used for CLI parsing and storage.
/// At runtime, it's converted to `Vec<Box<dyn Constraint>>` for evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationConstraints {
    /// Time range (HH:MM-HH:MM)
    pub time_range: Option<(String, String)>,
    /// Allowed weekdays (mon, tue, wed, thu, fri, sat, sun)
    pub weekdays: Option<Vec<String>>,
    /// CPU quota (0-100%)
    pub cpu_quota: Option<u8>,
    /// Memory quota in bytes
    pub memory_quota: Option<u64>,
    /// Expiry timestamp
    pub expires_at: String,
    /// Delegated to (key ID or identity)
    pub delegated_to: String,
}

impl DelegationConstraints {
    /// Convert to trait-based constraints
    ///
    /// This creates a `Vec<Box<dyn Constraint>>` that can be evaluated
    /// using the universal constraint system.
    #[allow(
        dead_code,
        reason = "Public builder for upcoming delegation constraint evaluation"
    )]
    pub fn to_constraints(&self) -> Vec<Box<dyn Constraint>> {
        let mut constraints: Vec<Box<dyn Constraint>> = Vec::new();

        // Add expiry constraint (always present)
        constraints.push(Box::new(ExpiryConstraint {
            expires_at: self.expires_at.clone(),
        }));

        // Add time range constraint if specified
        if let Some((start, end)) = &self.time_range {
            constraints.push(Box::new(TimeRangeConstraint {
                start: start.clone(),
                end: end.clone(),
            }));
        }

        // Add weekday constraint if specified
        if let Some(days) = &self.weekdays {
            constraints.push(Box::new(WeekdayConstraint {
                allowed_days: days.clone(),
            }));
        }

        // Add CPU quota constraint if specified
        if let Some(cpu) = self.cpu_quota {
            constraints.push(Box::new(CpuQuotaConstraint { max_percent: cpu }));
        }

        // Add memory quota constraint if specified
        if let Some(mem) = self.memory_quota {
            constraints.push(Box::new(MemoryQuotaConstraint { max_bytes: mem }));
        }

        constraints
    }

    /// Check if constraints are currently satisfied
    ///
    /// MODERNIZED: Uses trait-based constraint evaluation
    #[allow(
        dead_code,
        reason = "Public API for future delegation validation before issuance"
    )]
    pub fn is_satisfied(&self) -> Result<bool, BearDogError> {
        let constraints = self.to_constraints();
        let context = ConstraintContext::new().with_user(self.delegated_to.clone());

        // All constraints must be satisfied (AND logic)
        for constraint in &constraints {
            if !constraint.is_satisfied(&context)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get composite constraint (all constraints with AND logic)
    #[allow(
        dead_code,
        reason = "Public API for exporting composite constraints in delegation"
    )]
    pub fn as_composite(&self) -> CompositeConstraint {
        CompositeConstraint::and(self.to_constraints())
    }
}

/// Handle key delegation command
#[expect(
    clippy::too_many_arguments,
    reason = "CLI maps many optional delegation flags into one handler"
)]
pub async fn handle_key_delegate(
    master_key_id: &str,
    delegate_to: &str,
    output_key_id: &str,
    time_range: Option<&str>,
    weekdays: Option<&str>,
    cpu_quota: Option<u8>,
    memory_quota: Option<&str>,
    expires_in: &str,
) -> Result<(), BearDogError> {
    let keys_home = key_store::home_dir_for_keys()?;
    let params = DelegateParams {
        master_key_id,
        delegate_to,
        output_key_id,
        time_range,
        weekdays,
        cpu_quota,
        memory_quota,
        expires_in,
    };
    handle_key_delegate_impl(&params, &keys_home, std::path::Path::new(".")).await
}

/// Parameters for key delegation, avoiding long argument lists.
pub struct DelegateParams<'a> {
    /// Master key ID to delegate from
    pub master_key_id: &'a str,
    /// Identity or key ID being delegated to
    pub delegate_to: &'a str,
    /// Output key ID for the delegated key
    pub output_key_id: &'a str,
    /// Optional time range (HH:MM-HH:MM)
    pub time_range: Option<&'a str>,
    /// Optional allowed weekdays
    pub weekdays: Option<&'a str>,
    /// Optional CPU quota (0-100%)
    pub cpu_quota: Option<u8>,
    /// Optional memory quota string (e.g. "8GB")
    pub memory_quota: Option<&'a str>,
    /// Expiry duration string (e.g. "24h")
    pub expires_in: &'a str,
}

/// Same as [`handle_key_delegate`] but keys and receipts live under `home` (tests / DI).
#[cfg(test)]
pub async fn handle_key_delegate_with_home(
    params: &DelegateParams<'_>,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    let home = home.as_ref();
    handle_key_delegate_impl(params, home, home).await
}

async fn handle_key_delegate_impl(
    params: &DelegateParams<'_>,
    keys_home: &Path,
    receipt_parent: &Path,
) -> Result<(), BearDogError> {
    let master_key_id = params.master_key_id;
    let delegate_to = params.delegate_to;
    let output_key_id = params.output_key_id;
    let time_range = params.time_range;
    let weekdays = params.weekdays;
    let cpu_quota = params.cpu_quota;
    let memory_quota = params.memory_quota;
    let expires_in = params.expires_in;
    println!("🎫 BearDog Key Delegation");
    println!("========================\n");

    // Load master key
    println!("📥 Loading master key: {master_key_id}");
    let master_key = key_store::load_key_from_home(master_key_id, keys_home)?;

    println!("✅ Master key loaded");
    println!("   Algorithm: {}", master_key.algorithm);
    println!("   Generation: {}", master_key.generation);

    // Parse constraints
    println!("\n📋 Parsing delegation constraints...");

    let time_range_tuple = if let Some(tr) = time_range {
        Some(parse_time_range(tr)?)
    } else {
        None
    };

    let weekdays_vec = if let Some(wd) = weekdays {
        Some(parse_weekdays(wd)?)
    } else {
        None
    };

    let memory_bytes = if let Some(mem) = memory_quota {
        Some(parse_memory_quota(mem)?)
    } else {
        None
    };

    let expiry = super::key_derive::parse_duration(expires_in)?;

    // Display constraints
    println!("✅ Constraints:");
    if let Some((start, end)) = &time_range_tuple {
        println!("   ⏰ Time Range: {start} - {end}");
    }
    if let Some(days) = &weekdays_vec {
        println!("   📅 Weekdays: {}", days.join(", "));
    }
    if let Some(cpu) = cpu_quota {
        println!("   💻 CPU Quota: {cpu}%");
    }
    if let Some(mem) = memory_bytes {
        println!("   🧠 Memory Quota: {}", format_bytes(mem));
    }
    println!("   ⏱️  Expires: {}", expiry.format("%Y-%m-%d %H:%M:%S UTC"));

    // Derive delegated key
    println!("\n🔐 Creating delegated key...");

    let master_material = key_store::base64_decode(&master_key.key_material_b64)?;
    let delegation_context = format!(
        "delegate:{}:{}:{}",
        delegate_to,
        time_range.unwrap_or("anytime"),
        expires_in
    );

    // Use HKDF to derive delegated key
    let delegated_material = derive_delegated_key(&master_material, delegation_context.as_bytes())?;

    // Create delegation constraints object
    let constraints = DelegationConstraints {
        time_range: time_range_tuple.clone(),
        weekdays: weekdays_vec.clone(),
        cpu_quota,
        memory_quota: memory_bytes,
        expires_at: expiry.to_rfc3339(),
        delegated_to: delegate_to.to_string(),
    };

    // Serialize constraints to JSON string for storage in purpose field
    let constraints_json = serde_json::to_string(&constraints).map_err(|e| {
        BearDogError::serialization(&format!("Failed to serialize constraints: {e}"))
    })?;

    // Create delegated key
    let parent_depth = master_key.lineage.as_ref().map_or(0, |l| l.depth);

    let delegated_key = StoredKey {
        key_id: output_key_id.to_string(),
        algorithm: master_key.algorithm.clone(),
        hsm_name: format!("delegated-{}", master_key.hsm_name),
        key_material_b64: key_store::base64_encode(&delegated_material),
        created_at: Utc::now().to_rfc3339(),
        generation: master_key.generation + 1,
        parent_key_id: Some(master_key_id.to_string()),
        derivation_purpose: Some(format!("delegated-to-{delegate_to}")),
        children: Vec::new(),
        lineage: Some(key_store::KeyLineageInfo {
            parent_key_id: Some(master_key_id.to_string()),
            depth: parent_depth + 1,
        }),
        expires_at: Some(expiry.to_rfc3339()),
        usage: Some("delegated".to_string()),
        purpose: Some(constraints_json), // Store constraints in purpose field
    };

    // Save delegated key
    key_store::save_key_to_home(&delegated_key, keys_home)?;

    // Update master key
    let mut updated_master = master_key.clone();
    updated_master.children.push(output_key_id.to_string());
    key_store::save_key_to_home(&updated_master, keys_home)?;

    // Generate operation receipt
    use beardog_types::receipt::{KeyInfo, OperationReceipt, generate_receipt_filename};
    use serde_json::json;

    let mut receipt = OperationReceipt::new("key-delegate")
        .with_key_info(KeyInfo {
            key_id: output_key_id.to_string(),
            algorithm: delegated_key.algorithm.clone(),
            generation: delegated_key.generation,
            parent_key_id: Some(master_key_id.to_string()),
            expires_at: Some(expiry.to_rfc3339()),
            usage: None,
            purpose: Some(format!("Delegated to {delegate_to}")),
        })
        .with_metadata("master_key_id", json!(master_key_id))
        .with_metadata("delegated_to", json!(delegate_to))
        .with_metadata("expires_at", json!(expiry.to_rfc3339()));

    // Add constraint metadata
    if let Some((start, end)) = &time_range_tuple {
        receipt = receipt.with_metadata("time_range", json!(format!("{}-{}", start, end)));
    }
    if let Some(days) = &weekdays_vec {
        receipt = receipt.with_metadata("weekdays", json!(days));
    }
    if let Some(cpu) = cpu_quota {
        receipt = receipt.with_metadata("cpu_quota", json!(cpu));
    }
    if let Some(mem) = memory_bytes {
        receipt = receipt.with_metadata("memory_quota", json!(mem));
    }

    // Save receipt (`./receipts` in production; under `home` when using `_with_home`)
    let receipt_dir = receipt_parent.join("receipts");
    std::fs::create_dir_all(&receipt_dir)?;
    let receipt_path = receipt_dir.join(generate_receipt_filename("key-delegate"));
    receipt.save_to_file(&receipt_path)?;

    println!("\n✅ Delegated key created successfully!");
    println!("\n📋 Delegated Key Details:");
    println!("   ID: {output_key_id}");
    println!("   Delegated To: {delegate_to}");
    println!(
        "   Generation: {} (delegated from Gen {})",
        delegated_key.generation, master_key.generation
    );
    println!("   Parent: {master_key_id}");

    println!("\n📜 Receipt: {}", receipt_path.display());
    println!("   Receipt ID: {}", receipt.receipt_id);

    println!("\n🔒 Constraints:");
    if let Some((start, end)) = time_range_tuple {
        println!("   ⏰ Active Hours: {start} - {end}");
    }
    if let Some(days) = weekdays_vec {
        println!("   📅 Active Days: {}", days.join(", "));
    }
    if let Some(cpu) = cpu_quota {
        println!("   💻 CPU Limit: {cpu}%");
    }
    if let Some(mem) = memory_bytes {
        println!("   🧠 Memory Limit: {}", format_bytes(mem));
    }
    println!("   ⏱️  Expires: {}", expiry.format("%Y-%m-%d %H:%M:%S UTC"));

    println!("\n🎯 Use Case:");
    println!("   Tower sharing with resource limits and privacy");
    println!("   Delegated party can use resources within constraints");
    println!("   Master retains full control and can revoke");

    println!("\n💡 Next steps:");
    println!("   • Use delegated key: beardog encrypt --key {output_key_id} --input data.txt");
    println!("   • Check constraints: beardog key info --key-id {output_key_id}");

    Ok(())
}

/// Derive delegated key using HKDF
fn derive_delegated_key(master_key: &[u8], context: &[u8]) -> Result<Vec<u8>, BearDogError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(Some(b"beardog_delegation_v1"), master_key);
    let mut okm = vec![0u8; 32];
    hk.expand(context, &mut okm)
        .map_err(|e| BearDogError::crypto_error(format!("HKDF expansion failed: {e}")))?;

    Ok(okm)
}

/// Parse time range string (e.g., "9:00-17:00")
fn parse_time_range(range: &str) -> Result<(String, String), BearDogError> {
    let parts: Vec<&str> = range.split('-').collect();
    if parts.len() != 2 {
        return Err(BearDogError::validation(
            "Invalid time range format. Use 'HH:MM-HH:MM' (e.g., '9:00-17:00')",
        ));
    }

    // Validate time format (basic check)
    for part in &parts {
        if !part.contains(':') {
            return Err(BearDogError::validation(&format!(
                "Invalid time format '{part}'. Use 'HH:MM'"
            )));
        }
    }

    Ok((parts[0].to_string(), parts[1].to_string()))
}

/// Parse weekdays string (e.g., "mon-fri" or "mon,wed,fri")
fn parse_weekdays(weekdays: &str) -> Result<Vec<String>, BearDogError> {
    // Handle ranges like "mon-fri"
    if weekdays.contains('-') && !weekdays.contains(',') {
        let parts: Vec<&str> = weekdays.split('-').collect();
        if parts.len() == 2 {
            return expand_weekday_range(parts[0], parts[1]);
        }
    }

    // Handle comma-separated list like "mon,wed,fri"
    let days: Vec<String> = weekdays
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .collect();

    // Validate each day
    for day in &days {
        if !is_valid_weekday(day) {
            return Err(BearDogError::validation(&format!(
                "Invalid weekday '{day}'. Use mon, tue, wed, thu, fri, sat, sun"
            )));
        }
    }

    Ok(days)
}

/// Expand weekday range (e.g., "mon-fri" -> ["mon", "tue", "wed", "thu", "fri"])
fn expand_weekday_range(start: &str, end: &str) -> Result<Vec<String>, BearDogError> {
    let days_order = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

    let start_lower = start.trim().to_lowercase();
    let end_lower = end.trim().to_lowercase();

    let start_idx = days_order
        .iter()
        .position(|&d| d == start_lower)
        .ok_or_else(|| BearDogError::validation(&format!("Invalid weekday: {start}")))?;

    let end_idx = days_order
        .iter()
        .position(|&d| d == end_lower)
        .ok_or_else(|| BearDogError::validation(&format!("Invalid weekday: {end}")))?;

    if start_idx > end_idx {
        return Err(BearDogError::validation("Start day must be before end day"));
    }

    Ok(days_order[start_idx..=end_idx]
        .iter()
        .map(|&s| s.to_string())
        .collect())
}

/// Check if string is a valid weekday
fn is_valid_weekday(day: &str) -> bool {
    matches!(
        day.to_lowercase().as_str(),
        "mon" | "tue" | "wed" | "thu" | "fri" | "sat" | "sun"
    )
}

/// Parse memory quota string (e.g., "8GB", "512MB", "1024KB")
fn parse_memory_quota(quota: &str) -> Result<u64, BearDogError> {
    let quota = quota.trim().to_uppercase();

    // Extract number and unit
    let (num_str, unit) = if let Some(pos) = quota.find(|c: char| c.is_alphabetic()) {
        (&quota[..pos], &quota[pos..])
    } else {
        // Assume bytes if no unit
        return quota
            .parse::<u64>()
            .map_err(|_| BearDogError::validation("Invalid memory quota number"));
    };

    let num: u64 = num_str
        .trim()
        .parse()
        .map_err(|_| BearDogError::validation("Invalid memory quota number"))?;

    let bytes = match unit {
        "B" | "BYTES" => num,
        "KB" => num * 1024,
        "MB" => num * 1024 * 1024,
        "GB" => num * 1024 * 1024 * 1024,
        "TB" => num * 1024 * 1024 * 1024 * 1024,
        _ => {
            return Err(BearDogError::validation(&format!(
                "Unknown memory unit '{unit}'. Use B, KB, MB, GB, or TB"
            )));
        }
    };

    Ok(bytes)
}

/// Format bytes to human-readable string
#[expect(
    clippy::cast_precision_loss,
    reason = "Human-readable byte units; mantissa loss acceptable for display"
)]
fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    const TB: u64 = GB * 1024;

    if bytes >= TB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::key_store;
    use chrono::Utc;
    use tempfile::TempDir;

    #[test]
    fn test_parse_time_range() {
        let (start, end) = parse_time_range("9:00-17:00").unwrap();
        assert_eq!(start, "9:00");
        assert_eq!(end, "17:00");

        assert!(parse_time_range("invalid").is_err());
        assert!(parse_time_range("9:00").is_err());
    }

    #[test]
    fn test_parse_weekdays_range() {
        let days = parse_weekdays("mon-fri").unwrap();
        assert_eq!(days, vec!["mon", "tue", "wed", "thu", "fri"]);
    }

    #[test]
    fn test_parse_weekdays_list() {
        let days = parse_weekdays("mon,wed,fri").unwrap();
        assert_eq!(days, vec!["mon", "wed", "fri"]);
    }

    #[test]
    fn test_expand_weekday_range() {
        let days = expand_weekday_range("mon", "fri").unwrap();
        assert_eq!(days, vec!["mon", "tue", "wed", "thu", "fri"]);

        let weekend = expand_weekday_range("sat", "sun").unwrap();
        assert_eq!(weekend, vec!["sat", "sun"]);

        // Invalid range
        assert!(expand_weekday_range("fri", "mon").is_err());
    }

    #[test]
    fn test_parse_memory_quota() {
        assert_eq!(parse_memory_quota("8GB").unwrap(), 8 * 1024 * 1024 * 1024);
        assert_eq!(parse_memory_quota("512MB").unwrap(), 512 * 1024 * 1024);
        assert_eq!(parse_memory_quota("1024KB").unwrap(), 1024 * 1024);
        assert_eq!(parse_memory_quota("100").unwrap(), 100); // Bytes

        assert!(parse_memory_quota("invalid").is_err());
        assert!(parse_memory_quota("10XB").is_err());
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(100), "100 bytes");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(format_bytes(8 * 1024 * 1024 * 1024), "8.00 GB");
    }

    #[test]
    fn test_is_valid_weekday() {
        assert!(is_valid_weekday("mon"));
        assert!(is_valid_weekday("MON"));
        assert!(is_valid_weekday("fri"));
        assert!(!is_valid_weekday("invalid"));
        assert!(!is_valid_weekday("monday"));
    }

    #[test]
    fn test_derive_delegated_key_deterministic() {
        let master = [3u8; 32];
        let a = derive_delegated_key(&master, b"delegate:alice").unwrap();
        let b = derive_delegated_key(&master, b"delegate:alice").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.len(), 32);
        assert_ne!(derive_delegated_key(&master, b"delegate:bob").unwrap(), a);
    }

    #[test]
    fn test_parse_time_range_missing_colon_in_part() {
        assert!(parse_time_range("900-17:00").is_err());
    }

    #[test]
    fn test_delegation_constraints_to_constraints_and_composite() {
        let dc = DelegationConstraints {
            time_range: Some(("09:00".to_string(), "17:00".to_string())),
            weekdays: Some(vec!["mon".to_string()]),
            cpu_quota: Some(50),
            memory_quota: Some(1024),
            expires_at: "2099-01-01T00:00:00Z".to_string(),
            delegated_to: "user".to_string(),
        };
        let v = dc.to_constraints();
        assert!(v.len() >= 4);
        let _ = dc.as_composite();
    }

    #[tokio::test]
    async fn test_handle_key_delegate_full_flow() {
        let dir = TempDir::new().unwrap();
        let home = dir.path();

        let master = key_store::StoredKey {
            key_id: "master-delegate".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "hsm-x".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[11u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: Some(key_store::KeyLineageInfo {
                parent_key_id: None,
                depth: 0,
            }),
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&master, home).unwrap();

        let params = DelegateParams {
            master_key_id: "master-delegate",
            delegate_to: "delegatee",
            output_key_id: "delegated-out",
            time_range: Some("9:00-17:00"),
            weekdays: Some("mon,wed"),
            cpu_quota: Some(25),
            memory_quota: Some("512MB"),
            expires_in: "24h",
        };
        handle_key_delegate_with_home(&params, home)
            .await
            .expect("delegate");

        let del = key_store::load_key_from_home("delegated-out", home).expect("delegated key");
        assert_eq!(del.generation, 1);
        assert_eq!(del.parent_key_id.as_deref(), Some("master-delegate"));
    }

    #[test]
    fn test_parse_weekdays_invalid_day_in_list() {
        assert!(parse_weekdays("mon,bad").is_err());
    }

    #[test]
    fn test_parse_memory_quota_bytes_unit() {
        assert_eq!(parse_memory_quota("4096B").unwrap(), 4096);
        assert_eq!(parse_memory_quota("2TB").unwrap(), 2 * 1024_u64.pow(4));
    }

    #[test]
    fn test_format_bytes_terabyte() {
        let tb = 1024_u64.pow(4) * 3;
        let s = format_bytes(tb);
        assert!(s.contains("TB"));
    }

    #[test]
    fn test_delegation_constraints_is_satisfied_expired() {
        let dc = DelegationConstraints {
            time_range: None,
            weekdays: None,
            cpu_quota: None,
            memory_quota: None,
            expires_at: "2000-01-01T00:00:00Z".to_string(),
            delegated_to: "u".to_string(),
        };
        assert_eq!(dc.is_satisfied().unwrap(), false);
    }
}
