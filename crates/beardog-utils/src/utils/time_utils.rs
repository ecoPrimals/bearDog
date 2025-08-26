

use chrono::{DateTime, Utc};

pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}
