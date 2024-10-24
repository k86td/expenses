use chrono::{DateTime, Utc};
use rusqlite::ToSql;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug)]
pub struct Expense {
    pub uuid: Uuid,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub data: Value,
}
