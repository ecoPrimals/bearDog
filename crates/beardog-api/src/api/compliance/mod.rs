

use super::*;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub use audit::*;
pub use gdpr::*;
pub use hipaa::*;
pub use models::*;
pub use pci::*;
pub use policies::*;
pub use reporting::*;
pub use risk::*;
pub use sox::*;
pub use status::*;
pub use violations::*;

pub mod audit; // Audit trail management
pub mod gdpr; // GDPR compliance handlers
pub mod hipaa; // HIPAA compliance handlers
pub mod models; // All request/response models
pub mod pci; // PCI DSS compliance handlers
pub mod policies; // Policy management CRUD
pub mod reporting;
pub mod risk; // Risk assessment & mitigation
pub mod sox; // SOX compliance handlers
pub mod status; // Status, overview, and health endpoints
pub mod violations; // Violation detection & remediation // Report generation & analytics

pub fn create_routes() -> Router<AppState> {
    Router::new()

        .route("/status", get(status::get_compliance_status))
        .route("/overview", get(status::get_compliance_overview))
        .route("/health", get(status::get_compliance_health))

        .route("/audit/trail", get(audit::get_audit_trail))
        .route("/audit/events", post(audit::log_audit_event))
        .route("/audit/search", post(audit::search_audit_trail))
        .route("/audit/export", get(audit::export_audit_trail))

        .route("/gdpr/status", get(gdpr::get_gdpr_compliance))
        .route(
            "/gdpr/data-subject-request",
            post(gdpr::handle_data_subject_request),
        )
            "/gdpr/right-to-be-forgotten",
            post(gdpr::handle_right_to_be_forgotten),
        .route("/gdpr/consent-tracking", get(gdpr::get_consent_tracking))
        .route("/hipaa/status", get(hipaa::get_hipaa_compliance))
        .route("/hipaa/phi-access", post(hipaa::log_phi_access))
            "/hipaa/breach-assessment",
            post(hipaa::conduct_breach_assessment),
        .route("/sox/status", get(sox::get_sox_compliance))
        .route("/sox/controls", get(sox::get_sox_controls))
            "/sox/financial-reporting",
            get(sox::get_financial_reporting_compliance),
        .route("/pci/status", get(pci::get_pci_compliance))
            "/pci/cardholder-data",
            get(pci::audit_cardholder_data_handling),

        .route("/policies", get(policies::list_compliance_policies))
        .route("/policies", post(policies::create_compliance_policy))
        .route("/policies/:policy_id", get(policies::get_compliance_policy))
            "/policies/:policy_id",
            put(policies::update_compliance_policy),
            delete(policies::delete_compliance_policy),

        .route("/violations", get(violations::list_compliance_violations))
            "/violations/:violation_id",
            get(violations::get_violation_details),
            "/violations/:violation_id/remediate",
            post(violations::remediate_violation),

        .route("/risk/assessment", get(risk::get_risk_assessment))
        .route("/risk/assessment", post(risk::conduct_risk_assessment))
        .route("/risk/mitigation", get(risk::get_risk_mitigation_plan))

            "/reports/executive",
            get(reporting::generate_executive_report),
            "/reports/detailed",
            get(reporting::generate_detailed_report),
        .route("/reports/custom", post(reporting::generate_custom_report))
        .route("/analytics/trends", get(reporting::get_compliance_trends))
        .route("/analytics/metrics", get(reporting::get_compliance_metrics))
}
