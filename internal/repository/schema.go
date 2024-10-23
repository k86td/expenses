package repository

const CreateExpensesTable string = `
CREATE TABLE IF NOT EXISTS expenses (
  uuid      TEXT      PRIMARY KEY,
  created   DATETIME  NOT NULL,
  modified  DATETIME  NOT NULL,
  data      TEXT
);
`
