use std::collections::HashMap;

#[derive(Debug, Clone)]
    pub trust_level: TrustLevel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)] // Placeholder for future trust levels

pub enum TrustLevel {
    /// Represents basic variant
    Basic,
    /// State indicating enhanced
    Enhanced,
    /// Represents full variant
    Full,
}

impl Default for TrustLevel {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Debug, Clone)]
    tokens: HashMap<String, String>,
}
