// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Benchmark handler modules

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
