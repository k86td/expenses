use chrono::TimeZone;
use expenses::{repository::ExpensesRepository, sqlite::SqliteRepository};
use serde_json::json;
use uuid::Uuid;

fn main() {
    let mut repo = SqliteRepository::open("testing.db").unwrap();
    repo.create(expenses::models::Expense {
        uuid: Uuid::new_v4(),
        created: chrono::Utc::now(),
        modified: chrono::Utc::now(),
        data: json!({}),
    })
    .unwrap();
}
