use expenses::{models::Expense, repository::ExpensesRepository, sqlite::SqliteRepository};
use serde_json::json;

#[test]
fn initialize_db_then_crud_operation() -> expenses::Result<()> {
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

#[test]
fn delete_partial_uuid() -> expenses::Result<()> {
    let repo = SqliteRepository::initialize(":memory:")?;

    let now = chrono::Utc::now();
    let uuid = uuid::Uuid::new_v4().to_string();

    repo.create(Expense {
        uuid: uuid.clone(),
        created: now,
        modified: now,
        data: json!(null),
    })?;

    let expenses = repo.get_all(10)?;
    assert_eq!(expenses.len(), 1);

    repo.delete(&format!("%{}%", &uuid[0..6]))?;

    let expenses = repo.get_all(10)?;
    assert_eq!(expenses.len(), 0);

    Ok(())
}
