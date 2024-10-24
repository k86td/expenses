use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug)]
pub struct Expense {
    pub uuid: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub data: Value,
}
