use expenses::{models::Expense, repository::ExpensesRepository, sqlite::SqliteRepository};
use rusqlite::Error;
use serde_json::json;

#[test]
fn initialize_db_then_crud_operation() -> Result<(), Error> {
    let repo = SqliteRepository::initialize(":memory:")?;

    let now = chrono::Utc::now();
    let uuid = uuid::Uuid::new_v4().to_string();

    repo.create(Expense {
        uuid: uuid.clone(),
        created: now,
        modified: now,
        data: json!(null),
    })?;

    repo.update(Expense {
        uuid: uuid.clone(),
        created: now,
        modified: chrono::Utc::now(),
        data: json!({"test": true}),
    })?;

    let all_expenses = repo.get_all(5)?;
    assert_eq!(all_expenses.len(), 1);
    assert_eq!(all_expenses.first().unwrap().data, json!({"test": true}));

    repo.delete(&uuid)?;

    let all_expenses = repo.get_all(5)?;
    assert_eq!(all_expenses.len(), 0);

    repo.close().unwrap();

    let _ = SqliteRepository::open(":memory:");

    Ok(())
}
