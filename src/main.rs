use chrono::TimeZone;
use expenses::{repository::ExpensesRepository, sqlite::SqliteRepository};
use serde_json::json;
use uuid::Uuid;

fn main() {
    let repo = SqliteRepository::initialize(":memory:").unwrap();
    let uuid: String = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();

    repo.create(expenses::models::Expense {
        uuid: uuid.clone(),
        created: now,
        modified: now,
        data: json!({}),
    })
    .unwrap();

    repo.update(expenses::models::Expense {
        uuid,
        created: chrono::Utc::now(),
        modified: chrono::Utc::now(),
        data: json!({"newField": true}),
    })
    .unwrap();

    dbg!(repo.get_all(1).unwrap());
}
