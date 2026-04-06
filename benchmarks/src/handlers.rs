// SPDX-License-Identifier: AGPL-3.0-or-later


pub mod threat_analysis {
    use crate::types::RiskLevel;
    use serde::{Deserialize, Serialize};

    #[derive(String,
        pub subject_type: SubjectType,
        pub name: String,
    }

    #[derive(String,
        pub classification: ResourceClassification,
        pub name: String,
    }

    #[derive(ActionType,
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

    #[derive(String,
        pub username: String,
        pub status: AccountStatus,
        pub roles: Vec<String>,
    }
}
