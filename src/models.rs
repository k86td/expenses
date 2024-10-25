use chrono::{DateTime, Utc};
use serde_json::Value;
use tabled::Tabled;

use crate::repository::ExpensesRepository;

#[derive(Debug, Tabled)]
pub struct Expense {
    pub uuid: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub data: Value,
}

pub struct CliContext<'a, R>
where
    R: ExpensesRepository,
{
    pub repo: &'a R,
}
