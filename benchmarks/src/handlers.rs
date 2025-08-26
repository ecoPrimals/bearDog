

pub mod threat_analysis {
    use crate::types::RiskLevel;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Subject {
        pub subject_id: String,
        pub subject_type: SubjectType,
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum SubjectType {
        User,
        Service,
        System,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Resource {
        pub resource_id: String,
        pub classification: ResourceClassification,
        pub name: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ResourceClassification {
        Internal,
        External,
        Confidential,
        Public,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Action {
        pub action_type: ActionType,
        pub risk_level: RiskLevel,
        pub description: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub enum ActionType {
        Read,
        Write,
        Execute,
        Delete,
    }
}

pub mod session_management {
    use crate::types::AccountStatus;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct UserInfo {
        pub user_id: String,
        pub username: String,
        pub status: AccountStatus,
        pub roles: Vec<String>,
    }
}
