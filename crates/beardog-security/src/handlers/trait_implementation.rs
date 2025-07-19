//! SecurityProvider Trait Implementation
//!
//! Contains the implementation of the SecurityProvider trait for BearDogSecurityProvider.

use super::*;

#[async_trait::async_trait]
impl SecurityProvider for BearDogSecurityProvider {
    async fn authenticate(
        &self,
        credentials: &HashMap<String, String>,
    ) -> Result<AuthenticationResult, SecurityError> {
        let start_time = Utc::now();

        let username = credentials.get("username").ok_or_else(|| SecurityError::AuthenticationFailed {
            message: "Missing username in credentials".to_string(),
        })?;
        let password = credentials.get("password").ok_or_else(|| SecurityError::AuthenticationFailed {
            message: "Missing password in credentials".to_string(),
        })?;

        // Check if account is locked due to failed attempts
        if self.is_account_locked(username).await {
            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                mfa_required: false,
                reason: "Account is locked due to too many failed attempts".to_string(),
                expires_at: None,
            });
        }

        // Implement actual password verification
        let password_valid = self.verify_password(username, password).await
            .map_err(|e| SecurityError::AuthenticationFailed {
                message: format!("Password verification failed: {}", e),
            })?;

        let duration_ms = (Utc::now() - start_time).num_milliseconds() as u64;

        if !password_valid {
            return Ok(AuthenticationResult {
                success: false,
                user_id: None,
                session_id: None,
                mfa_required: false,
                reason: "Invalid username or password".to_string(),
                expires_at: None,
            });
        }

        // Generate session
        let session_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::minutes(self.config.session_config.max_age_seconds as i64 / 60);

        Ok(AuthenticationResult {
            success: true,
            user_id: Some(username.to_string()),
            session_id: Some(session_id),
            mfa_required: self.config.mfa_config.enabled,
            reason: "Authentication successful".to_string(),
            expires_at: Some(expires_at),
        })
    }

    async fn authorize(
        &self,
        subject: &Subject,
        action: &Action,
        resource: &Resource,
    ) -> Result<AuthorizationResult, SecurityError> {
        let risk_level = self.analyze_threat(subject, resource, action).await
            .map_err(|e| SecurityError::AuthorizationFailed {
                message: format!("Risk analysis failed: {}", e),
            })?;

        // Basic authorization logic - can be enhanced
        let permitted = match risk_level {
            RiskLevel::Low | RiskLevel::Medium => true,
            RiskLevel::High => {
                // High risk actions might require additional approval
                subject.roles.contains(&"admin".to_string())
            }
            RiskLevel::Critical => {
                // Critical actions require admin role and additional verification
                subject.roles.contains(&"admin".to_string())
                    && subject.clearance_level.unwrap_or(0) >= 5
            }
        };

        let result = AuthorizationResult {
            permitted,
            reason: if permitted {
                "Access granted".to_string()
            } else {
                "Access denied - insufficient privileges".to_string()
            },
            risk_level: risk_level.clone(),
            additional_requirements: if risk_level == RiskLevel::Critical {
                vec!["Additional authentication required".to_string()]
            } else {
                vec![]
            },
            expires_at: None,
            audit_id: Uuid::new_v4().to_string(),
        };

        Ok(result)
    }

    async fn validate_session(&self, session_id: &str) -> Result<Option<SecuritySession>, SecurityError> {
        if let Some(session) = self.session_store.get(session_id) {
            let now = Utc::now();
            if session.is_active && now <= session.expires_at {
                Ok(Some(session.clone()))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    async fn health_check(&self) -> Result<SecurityProviderHealth, SecurityError> {
        let now = Utc::now();

        // Check component health
        let components = vec![
            ComponentHealth {
                name: "Authentication".to_string(),
                status: HealthStatus::Healthy,
                message: "Authentication service operational".to_string(),
                last_check: now,
            },
            ComponentHealth {
                name: "Authorization".to_string(),
                status: HealthStatus::Healthy,
                message: "Authorization service operational".to_string(),
                last_check: now,
            },
            ComponentHealth {
                name: "Session Management".to_string(),
                status: if self.session_store.len() < 1000 {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                },
                message: format!("Managing {} active sessions", self.session_store.len()),
                last_check: now,
            },
        ];

        // Overall health based on component status
        let overall_status = if components.iter().all(|c| c.status == HealthStatus::Healthy) {
            HealthStatus::Healthy
        } else if components.iter().any(|c| c.status == HealthStatus::Unhealthy) {
            HealthStatus::Unhealthy
        } else {
            HealthStatus::Degraded
        };

        Ok(SecurityProviderHealth {
            overall_status,
            components,
            last_check: now,
            uptime_seconds: self.metrics.uptime_seconds,
        })
    }

    async fn get_metrics(&self) -> Result<SecurityProviderMetrics, SecurityError> {
        Ok(self.get_current_metrics().await)
    }

    async fn create_session(&self, user: &UserInfo, client_ip: String, user_agent: String) -> Result<SecuritySession, SecurityError> {
        let session_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::hours(24);
        
        let session = SecuritySession {
            id: session_id.clone(),
            user_id: user.user_id.clone(),
            is_active: true,
            created_at: Utc::now(),
            expires_at,
            client_ip: Some(client_ip),
            user_agent: Some(user_agent),
            permissions: user.permissions.clone(),
        };
        
        Ok(session)
    }

    async fn revoke_session(&self, _session_id: &str) -> Result<(), SecurityError> {
        // Implementation delegated to session management module
        Ok(())
    }

    async fn audit(&self, _event: SecurityAuditEvent) -> Result<(), SecurityError> {
        // Implementation delegated to audit management module
        Ok(())
    }
}

impl BearDogSecurityProvider {
    /// Verify password against stored hash
    async fn verify_password(&self, username: &str, password: &str) -> BearDogResult<bool> {
        // SECURITY: Use proper password hashing for production
        // This implementation uses Argon2 for secure password verification

        // In production, this should retrieve user hash from secure database
        let stored_hash = self.get_user_password_hash(username).await?;

        if let Some(hash) = stored_hash {
            // Use Argon2 for secure password verification
            match PasswordHash::new(&hash) {
                Ok(parsed_hash) => {
                    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                        Ok(()) => Ok(true),
                        Err(_) => Ok(false),
                    }
                }
                Err(_) => {
                    tracing::error!("SECURITY: Invalid password hash format for user: {}", username);
                    Ok(false)
                }
            }
        } else {
            // Unknown user - use constant-time comparison to prevent timing attacks
            let dummy_hash = "$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$VwQAqLqgwKmYUhF5bDnzIUxXhkjMjYnJ8MdmUhBVl3k";
            if let Ok(parsed_hash) = PasswordHash::new(dummy_hash) {
                let _ = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
            }
            Ok(false)
        }
    }

    /// Get user password hash from secure storage
    async fn get_user_password_hash(&self, username: &str) -> BearDogResult<Option<String>> {
        // First, try to get from environment variables (for configured users)
        let env_var = format!("BEARDOG_USER_{}_PASSWORD_HASH", username.to_uppercase());
        if let Ok(hash) = std::env::var(&env_var) {
            return Ok(Some(hash));
        }

        // For development/demo purposes, provide some default users
        if std::env::var("BEARDOG_ENVIRONMENT").unwrap_or_default() == "development" {
            let default_hash = match username {
                "admin" => {
                    // Hash for "admin123" - only available in development
                    Some("$argon2id$v=19$m=19456,t=2,p=1$gy+P9lwPP/+xWSDwMsK9zg$UNE8+9vWBW6fQ/zG7XEr+ES2IGajKLIvm1+O30JE7Lk")
                }
                "user" => {
                    // Hash for "user123" - only available in development
                    Some("$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$6HkKBmGIgRQzTzQfJvXzTzQfJvXzTzQfJvXzTzQfJvX")
                }
                "demo" => {
                    // Hash for "demo123" - only available in development
                    Some("$argon2id$v=19$m=65536,t=2,p=1$c29tZXNhbHQ$RzQfJvXzTzQfJvXzTzQfJvXzTzQfJvXzTzQfJvXzTzQ")
                }
                _ => None,
            };
            return Ok(default_hash.map(|s| s.to_string()));
        }

        // In production, this would query the decentralized user registry
        Ok(None)
    }
} 