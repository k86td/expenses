package repository

// CreateExpensesTable constant representing the SQL statement defining the table schema.
const CreateExpensesTable string = `
CREATE TABLE IF NOT EXISTS expenses (
  uuid      TEXT      PRIMARY KEY,
  created   DATETIME  NOT NULL,
  modified  DATETIME  NOT NULL,
  data      TEXT
);
`

// InsertExpense defines the SQL schema to insert an expense inside the expenses table.
const InsertExpense string = `
INSERT INTO expenses (uuid, created, modified, data)
VALUES (?, ?, ?, ?)
`

const UpdateExpense string = `
UPDATE expenses
SET
  modified = ?
  data    = ?
WHERE
  uuid = ?;
`

const DeleteExpense string = `
DELETE FROM expenses
WHERE
  uuid = ?;
`

const GetAllExpenses string = `
SELECT
  uuid, created, modified, data
FROM expenses
ORDER BY
  modified DESC
LIMIT ?
`
