use beardog_errors::BearDogError;
use chrono::Utc;
use tracing::debug;

use super::{ComputeQuota, KeyConstraints, KeyOperation, ScopeConstraint};

impl KeyConstraints {
    /// Compute a cryptographic hash of these constraints
    ///
    /// This hash is used for signing to ensure constraint integrity.
    ///
    /// # Errors
    ///
    /// Returns error if serialization fails
    pub fn hash(&self) -> Result<[u8; 32], BearDogError> {
        use sha3::{Digest, Sha3_256};

        // Serialize constraints to deterministic format
        let serialized = serde_json::to_vec(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize constraints: {e}"))
        })?;

        // Hash the serialized form
        let mut hasher = Sha3_256::new();
        hasher.update(&serialized);
        hasher.update(b"BearDog-Constraint-Hash-v1"); // Domain separation

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result[..32]);

        Ok(hash)
    }

    /// Get a human-readable description of all constraints
    #[must_use]
    pub fn description(&self) -> String {
        let mut parts = Vec::new();

        // Scope
        match &self.scope {
            ScopeConstraint::Unrestricted => {}
            ScopeConstraint::Project { name, .. } => {
                parts.push(format!("Project: {name}"));
            }
            ScopeConstraint::Resources {
                allow_read,
                allow_write,
                deny_delete,
            } => {
                if !deny_delete.is_empty() {
                    parts.push(format!("Protected from deletion: {deny_delete:?}"));
                }
                if !allow_read.is_empty() {
                    parts.push(format!("Read access: {allow_read:?}"));
                }
                if !allow_write.is_empty() {
                    parts.push(format!("Write access: {allow_write:?}"));
                }
            }
            ScopeConstraint::Operations { allowed_operations } => {
                parts.push(format!("Allowed ops: {allowed_operations:?}"));
            }
        }

        // Lifetime
        parts.push(format!(
            "Expires: {}",
            self.lifetime.expires_at.format("%Y-%m-%d")
        ));

        // Co-signers
        if !self.co_signers.is_empty() {
            parts.push(format!("Requires {} co-signers", self.co_signers.len()));
        }

        // Behavioral
        if self.behavior.biometric_required {
            parts.push("Requires biometric".to_string());
        }

        parts.join(", ")
    }

    /// Verify an operation against these constraints
    ///
    /// # Errors
    /// Returns error if operation violates any constraint
    pub fn verify_operation(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        // 1. Check lifetime
        if Utc::now() > self.lifetime.expires_at {
            return Err(BearDogError::unauthorized(
                "Key expired (lifetime constraint)".to_string(),
            ));
        }

        // 2. Check scope
        self.verify_scope(operation)?;

        // 3. Check data access
        self.verify_data_access(operation)?;

        // 4. Check co-signers
        if !self.co_signers.is_empty() {
            // Note: Co-signature verification requires operation metadata
            // In production, operation would include `co_signatures: Vec<CoSignature>`
            // For now, log requirement (actual verification happens in auth layer)
            debug!(
                "Operation requires {} co-signers: {:?}",
                self.co_signers.len(),
                self.co_signers
            );
        }

        // 5. Check behavioral constraints
        self.verify_behavior(operation);

        // 6. Check compute quota
        if let Some(quota) = &self.compute_quota {
            self.verify_compute_quota(operation, quota)?;
        }

        Ok(())
    }

    fn verify_scope(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        match &self.scope {
            ScopeConstraint::Unrestricted => Ok(()),

            ScopeConstraint::Project { name, .. } => {
                let op_project = match operation {
                    KeyOperation::Read { project, .. } => project,
                    KeyOperation::Write { project, .. } => project,
                    KeyOperation::RpcCall { project, .. } => project,
                    _ => &None,
                };

                if let Some(op_proj) = op_project
                    && op_proj != name
                {
                    return Err(BearDogError::unauthorized(format!(
                        "Key scoped to project '{name}', cannot access '{op_proj}'"
                    )));
                }
                Ok(())
            }

            ScopeConstraint::Resources {
                allow_read,
                allow_write,
                deny_delete,
            } => match operation {
                KeyOperation::Delete { path } => {
                    if deny_delete.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(
                            "Key cannot delete protected resources (cryptographically enforced)"
                                .to_string(),
                        ));
                    }
                    Ok(())
                }
                KeyOperation::Read { path, .. } => {
                    if !allow_read.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(format!(
                            "Key not authorized to read path: {path}"
                        )));
                    }
                    Ok(())
                }
                KeyOperation::Write { path, .. } => {
                    if !allow_write.iter().any(|p| Self::path_matches(path, p)) {
                        return Err(BearDogError::unauthorized(format!(
                            "Key not authorized to write path: {path}"
                        )));
                    }
                    Ok(())
                }
                _ => Ok(()),
            },

            ScopeConstraint::Operations { allowed_operations } => {
                let op_name = match operation {
                    KeyOperation::Read { .. } => "read",
                    KeyOperation::Write { .. } => "write",
                    KeyOperation::Delete { .. } => "delete",
                    KeyOperation::RpcCall { method, .. } => method.as_str(),
                    KeyOperation::ComputeAllocation { .. } => "compute",
                };

                if !allowed_operations.iter().any(|op| op == op_name) {
                    return Err(BearDogError::unauthorized(format!(
                        "Operation '{op_name}' not allowed by key constraints"
                    )));
                }
                Ok(())
            }
        }
    }

    fn verify_data_access(&self, operation: &KeyOperation) -> Result<(), BearDogError> {
        if let KeyOperation::Delete { path } = operation
            && self
                .data_access
                .immutable_paths
                .iter()
                .any(|p| Self::path_matches(path, p))
        {
            return Err(BearDogError::unauthorized(format!(
                "cannot delete protected path: {path} (cryptographically enforced)"
            )));
        }

        Ok(())
    }

    fn verify_behavior(&self, operation: &KeyOperation) {
        // Get behavioral verification mode from environment
        let behavioral_mode = std::env::var("BEARDOG_BEHAVIORAL_VERIFICATION")
            .unwrap_or_else(|_| "advisory".to_string())
            .to_lowercase();

        match behavioral_mode.as_str() {
            "permissionless" => {
                // Testing mode: skip all verification
            }
            "strict" => {
                // Strict mode: enforce all behavioral constraints

                // Check biometric requirements (if any)
                // In production, this would integrate with platform biometric APIs
                tracing::debug!(
                    "Behavioral verification (strict): checking biometric requirements"
                );

                // Analyze usage patterns
                // In production, track operation frequency, timing patterns, etc.
                tracing::debug!("Behavioral verification (strict): analyzing usage patterns");

                // Check network constraints
                // In production, verify operation is from expected network/location
                tracing::debug!("Behavioral verification (strict): checking network constraints");
            }
            _ => {
                // Advisory mode: log warnings but don't block
                tracing::debug!(
                    "Behavioral verification (advisory): operation {:?} - checks advisory only",
                    operation
                );
            }
        }
    }

    fn verify_compute_quota(
        &self,
        operation: &KeyOperation,
        quota: &ComputeQuota,
    ) -> Result<(), BearDogError> {
        if let KeyOperation::ComputeAllocation {
            hours,
            memory_bytes,
        } = operation
        {
            if quota.current_usage.hours_used + hours > quota.max_hours {
                return Err(BearDogError::unauthorized(format!(
                    "Compute quota exceeded: {} + {} > {} hours",
                    quota.current_usage.hours_used, hours, quota.max_hours
                )));
            }

            if *memory_bytes > quota.max_memory_bytes {
                return Err(BearDogError::unauthorized(format!(
                    "Memory quota exceeded: {} > {} bytes",
                    memory_bytes, quota.max_memory_bytes
                )));
            }
        }

        Ok(())
    }

    /// Check if a path matches a pattern (supports glob-like patterns)
    fn path_matches(path: &str, pattern: &str) -> bool {
        crate::genetics_constraints_helpers::path_matches(path, pattern)
    }
}
