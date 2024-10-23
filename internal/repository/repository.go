package repository

import (
	"database/sql"
	"errors"
	"fmt"
	"os"

	_ "github.com/ncruces/go-sqlite3/driver"

	_ "github.com/ncruces/go-sqlite3/embed"
)

type ExpenseRepository struct {
	Db *sql.DB
}

// InitializeDatabase initialize the database with the required tables.
func InitializeDatabase(db *sql.DB) error {
	_, err := db.Exec(CreateExpensesTable)
	if err != nil {
		return err
	}

	return nil
}

// OpenOrCreateDatabase creates a new repository. If the underlying database doesn't exist
// it will be created, otherwise use the already existing one. If the database file doesn't
// exist it will be initialized. See [InitializeDatabase]. The database is opening using
// [sql.Db].
func OpenOrCreateDatabase(dbname string) (*ExpenseRepository, error) {
	var needToInitialize bool

	if _, err := os.Stat(dbname); errors.Is(err, os.ErrNotExist) {
		fmt.Println("Database needs to be initialized")
		needToInitialize = true
	} else {
		needToInitialize = false
	}

	db, err := sql.Open("sqlite3", fmt.Sprintf("file:%v", dbname))
	if err != nil {
		fmt.Printf("error: %v", err)
		return nil, err
	} else if needToInitialize {
		fmt.Println("Initializing database")

		err := InitializeDatabase(db)
		if err != nil {
			fmt.Printf("Got error while init DB: %v\n", err)
		}
	}

	return &ExpenseRepository{Db: db}, nil
}

// PrintVersion prints the version of the database inside this repository.
// This is mostly for debugging purposes.
func (repo ExpenseRepository) PrintVersion() {
	var version string
	repo.Db.QueryRow("SELECT sqlite_version();").Scan(&version)
	fmt.Printf("Version: %v\n", version)
}

// Create creates a new expense using the repository database.
func (repo ExpenseRepository) Create(exp Expense) error {
	stmt, err := repo.Db.Prepare(InsertExpense)
	if err != nil {
		return err
	}

	_, err = stmt.Exec(exp.Uuid, exp.Created, exp.Modified, exp.Data)
	if err != nil {
		return err
	}

	return nil
}

// Update updates an expense using its stored Uuid.
func (repo ExpenseRepository) Update(exp Expense) error {
	stmt, err := repo.Db.Prepare(UpdateExpense)
	if err != nil {
		return err
	}

	_, err = stmt.Exec(
		exp.Modified,
		exp.Data,
		exp.Uuid,
	)
	if err != nil {
		return err
	}

	return nil
}

// Update delete an expense using its stored Uuid.
func (repo ExpenseRepository) Delete(exp Expense) error {
	stmt, err := repo.Db.Prepare(DeleteExpense)
	if err != nil {
		return err
	}

	_, err = stmt.Exec(exp.Uuid)
	if err != nil {
		return err
	}

	return nil
}

// GetAll returns all the expenses ordered descending by modified date and with
// a limit on the selected result.
func (repo ExpenseRepository) GetAll(limit int) ([]Expense, error) {
	stmt, err := repo.Db.Prepare(GetAllExpenses)
	if err != nil {
		return nil, err
	}

	rows, err := stmt.Query(limit)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var exps []Expense
	for rows.Next() {
		var exp Expense
		if err := rows.Scan(&exp.Uuid, &exp.Created, &exp.Modified, &exp.Data); err != nil {
			return exps, err
		}
		exps = append(exps, exp)
	}

	if err = rows.Err(); err != nil {
		return exps, err
	}

	return exps, nil
}
