package main

import (
	"fmt"
	"os"

	repository "k86td/expenses/internal/repository"
)

func main() {
	println("Hello from cmd/expenses")
	repo, err := repository.OpenOrCreateDatabase("testing.db")
	if err != nil {
		os.Exit(1)
	}

	defer repo.Db.Close()

	fmt.Printf("Opened DB: %v\n", repo)
	repo.PrintVersion()
}
