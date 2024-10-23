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

func InitializeDatabase(db *sql.DB) error {
	_, err := db.Exec(CreateExpensesTable)
	if err != nil {
		return err
	}

	return nil
}

// OpenOrCreateDatabase this creates a database if it doesn't already exists, otherwise open the file.
func OpenOrCreateDatabase(dbname string) (ExpenseRepository, error) {
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
		return ExpenseRepository{}, err
	} else if needToInitialize {
		fmt.Println("Initializing database")

		err := InitializeDatabase(db)
		if err != nil {
			fmt.Printf("Got error while init DB: %v\n", err)
		}
	}

	return ExpenseRepository{Db: db}, nil
}

// PrintVersion prints the version of the database inside this repository
func (repo ExpenseRepository) PrintVersion() {
	var version string
	repo.Db.QueryRow("SELECT sqlite_version();").Scan(&version)
	fmt.Printf("Version: %v\n", version)
}
