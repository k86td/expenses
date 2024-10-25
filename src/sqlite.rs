use rusqlite::{Connection, OpenFlags};

use crate::{models::Expense, repository::ExpensesRepository, Result};

const CREATE_TABLE_EXPENSES: &str = "
CREATE TABLE IF NOT EXISTS expenses (
  uuid      TEXT      PRIMARY KEY,
  created   DATETIME  NOT NULL,
  modified  DATETIME  NOT NULL,
  data      TEXT
);
";

const CREATE_EXPENSE: &str = "
INSERT INTO expenses
    (uuid, created, modified, data)
VALUES
    (?, ?, ?, ?);
";

const UPDATE_EXPENSE: &str = "
UPDATE expenses
SET
    modified    = ?,
    data        = ?
WHERE
    uuid = ?;
";

const DELETE_EXPENSE: &str = "
DELETE FROM expenses
WHERE
    uuid LIKE ?;
";

const GET_ALL_EXPENSES: &str = "
SELECT uuid, created, modified, data
FROM expenses
ORDER BY
    modified DESC
LIMIT ?;
";

#[derive(Debug)]
pub struct SqliteRepository {
    db: Connection,
}

impl SqliteRepository {
    /// Open a database at `db_path`. The database must exist, otherwise it will return an error
    pub fn open(db_path: &str) -> Result<Self> {
        Ok(SqliteRepository {
            db: Connection::open_with_flags(
                db_path,
                OpenFlags::SQLITE_OPEN_READ_WRITE
                    | OpenFlags::SQLITE_OPEN_URI
                    | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )?,
        })
    }

    /// Initialize the database at `db_path`. If it encounters an error during the creation an
    /// error will be returned.
    pub fn initialize(db_path: &str) -> Result<Self> {
        let db = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        db.execute(CREATE_TABLE_EXPENSES, [])?;

        Ok(SqliteRepository { db })
    }

    /// Closes the database.
    // FIX: should this also be using expenses::Result instead of core::result::Result ?
    pub fn close(self) -> core::result::Result<(), (Connection, rusqlite::Error)> {
        self.db.close()
    }
}

impl ExpensesRepository for SqliteRepository {
    fn create(&self, expense: Expense) -> Result<usize> {
        let changed_rows = self.db.execute(
            CREATE_EXPENSE,
            (
                expense.uuid,
                expense.created,
                expense.modified,
                expense.data,
            ),
        )?;

        Ok(changed_rows)
    }

    fn update(&self, expense: Expense) -> Result<usize> {
        let changed_rows = self.db.execute(
            UPDATE_EXPENSE,
            (expense.modified, expense.data, expense.uuid.to_string()),
        )?;

        Ok(changed_rows)
    }

    fn delete(&self, uuid: &str) -> Result<usize> {
        let changed_rows = self.db.execute(DELETE_EXPENSE, [uuid])?;

        Ok(changed_rows)
    }

    fn get_all(&self, limit: u32) -> Result<Vec<Expense>> {
        let mut stmt = self.db.prepare(GET_ALL_EXPENSES)?;
        let expense_iter = stmt.query_map([limit], |row| {
            Ok(Expense {
                uuid: row.get(0)?,
                created: row.get(1)?,
                modified: row.get(2)?,
                data: row.get(3)?,
            })
        })?;

        let mut res: Vec<Expense> = Vec::new();
        for exp in expense_iter {
            res.push(exp?);
        }

        Ok(res)
    }
}
