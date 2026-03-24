// SPDX-License-Identifier: AGPL-3.0-only

//! IPC dispatch outcomes and error phase classification.

use std::fmt;

/// Phase of an IPC call where a failure occurred.
/// Allows callers to distinguish transport/protocol issues (retry-worthy)
/// from application-level rejections (not retry-worthy).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum IpcErrorPhase {
    /// Transport layer failure (socket not found, connection refused, timeout).
    Transport,
    /// JSON-RPC protocol error (malformed request, parse error).
    Protocol,
    /// Method dispatch error (method not found, invalid params).
    Dispatch,
    /// Application-level error from the handler (business logic failure).
    Application,
}

impl fmt::Display for IpcErrorPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport => write!(f, "transport"),
            Self::Protocol => write!(f, "protocol"),
            Self::Dispatch => write!(f, "dispatch"),
            Self::Application => write!(f, "application"),
        }
    }
}

/// Outcome of an IPC method dispatch, carrying the phase on failure.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DispatchOutcome<T = serde_json::Value> {
    /// Method executed successfully.
    Success(T),
    /// Method failed at a known phase.
    Failure {
        /// Where the failure occurred (transport, protocol, dispatch, or application).
        phase: IpcErrorPhase,
        /// JSON-RPC-style error code (e.g. -32601 for method not found).
        code: i64,
        /// Human-readable error message.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_outcome_serde_roundtrip_success() {
        let v = DispatchOutcome::Success(serde_json::json!({"a": 1}));
        let s = serde_json::to_string(&v).expect("serialize");
        let back: DispatchOutcome<serde_json::Value> =
            serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back, v);
    }

    #[test]
    fn dispatch_outcome_serde_roundtrip_failure() {
        let v = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Dispatch,
            code: -32601,
            message: "Method not found: x".to_string(),
        };
        let s = serde_json::to_string(&v).expect("serialize");
        let back: DispatchOutcome<serde_json::Value> =
            serde_json::from_str(&s).expect("deserialize");
        assert_eq!(back, v);
    }

    #[test]
    fn ipc_error_phase_display() {
        assert_eq!(format!("{}", IpcErrorPhase::Transport), "transport");
        assert_eq!(format!("{}", IpcErrorPhase::Protocol), "protocol");
        assert_eq!(format!("{}", IpcErrorPhase::Dispatch), "dispatch");
        assert_eq!(format!("{}", IpcErrorPhase::Application), "application");
    }

    #[test]
    fn ipc_error_phase_debug() {
        let s = format!("{:?}", IpcErrorPhase::Dispatch);
        assert!(s.contains("Dispatch"));
    }

    #[test]
    fn dispatch_outcome_failure_eq_distinct_phases() {
        let a = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Transport,
            code: -1,
            message: "a".to_string(),
        };
        let b = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Application,
            code: -1,
            message: "a".to_string(),
        };
        assert_ne!(a, b);
    }

    #[test]
    fn dispatch_outcome_failure_neq_when_message_differs() {
        let a = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Protocol,
            code: -32700,
            message: "parse".to_string(),
        };
        let b = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Protocol,
            code: -32700,
            message: "other".to_string(),
        };
        assert_ne!(a, b);
    }

    #[test]
    fn dispatch_outcome_success_neq_failure() {
        let ok = DispatchOutcome::Success(serde_json::json!(null));
        let bad = DispatchOutcome::<serde_json::Value>::Failure {
            phase: IpcErrorPhase::Dispatch,
            code: 0,
            message: "m".to_string(),
        };
        assert_ne!(ok, bad);
    }

    #[test]
    fn ipc_error_phase_serde_roundtrip() {
        for phase in [
            IpcErrorPhase::Transport,
            IpcErrorPhase::Protocol,
            IpcErrorPhase::Dispatch,
            IpcErrorPhase::Application,
        ] {
            let s = serde_json::to_string(&phase).expect("serialize phase");
            let back: IpcErrorPhase = serde_json::from_str(&s).expect("deserialize phase");
            assert_eq!(back, phase);
        }
    }
}
