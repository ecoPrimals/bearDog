// SPDX-License-Identifier: AGPL-3.0-or-later

//! Audit logging trait and concrete backend dispatch.

use beardog_errors::BearDogError;
use std::future::Future;

use crate::tunnel::hsm::software_hsm::audit::types::{AuditLogEntry, AuditLogFilter};

/// Trait for audit logging
pub trait AuditLogger: Send + Sync {
    /// Log an operation
    fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send;

    /// Get audit log entries with filter
    fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> impl Future<Output = Result<Vec<AuditLogEntry>, BearDogError>> + Send;
}

/// Audit logger dispatch (replaces `Arc<dyn AuditLogger>`).
pub enum AuditLoggerBackend {
    /// Default persistent audit logger
    Default(crate::tunnel::hsm::software_hsm::audit::logger::DefaultAuditLogger),
}

impl AuditLogger for AuditLoggerBackend {
    fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> impl Future<Output = Result<(), BearDogError>> + Send {
        let op = operation.clone();
        let slf = self;
        async move {
            match slf {
                Self::Default(l) => AuditLogger::log_operation(l, &op).await,
            }
        }
    }

    fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> impl Future<Output = Result<Vec<AuditLogEntry>, BearDogError>> + Send {
        let filter = filter.clone();
        let slf = self;
        async move {
            match slf {
                Self::Default(l) => AuditLogger::get_audit_log(l, &filter).await,
            }
        }
    }
}
