package repository

import (
	"database/sql"
	"fmt"

	_ "github.com/ncruces/go-sqlite3/driver"

	_ "github.com/ncruces/go-sqlite3/embed"
)

type ExpenseRepository struct {
	Db *sql.DB
}

// OpenOrCreateDatabase this creates a database if it doesn't already exists, otherwise open the file.
func OpenOrCreateDatabase(dbname string) (ExpenseRepository, error) {
	db, err := sql.Open("sqlite3", fmt.Sprintf("file:%v", dbname))
	if err != nil {
		fmt.Printf("error: %v", err)
		return ExpenseRepository{}, err
	}

	return ExpenseRepository{Db: db}, nil
}

// PrintVersion prints the version of the database inside this repository
func (repo ExpenseRepository) PrintVersion() {
	var version string
	repo.Db.QueryRow("SELECT sqlite_version()").Scan(&version)
	fmt.Printf("Version: %v\n", version)
}

// IsInitialized checks if the database is initalized by
func (repo ExpenseRepository) IsInitialized() bool {
	res := repo.Db.Stats()
	fmt.Printf("Databases: %v\n", res)

	return false
}
