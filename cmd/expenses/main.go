package main

import (
	"fmt"
	"os"
	"time"

	repository "k86td/expenses/internal/repository"

	"github.com/google/uuid"
)

func main() {
	repo, err := repository.OpenOrCreateDatabase("testing.db")
	if err != nil {
		os.Exit(1)
	}
	defer repo.Db.Close()

	fmt.Printf("Opened DB: %v\n", repo)

	now := time.Now()
	err = repo.Create(repository.Expense{
		Created:  now,
		Modified: now,
		Uuid:     uuid.New().String(),
		Data:     "{'testing': true}",
	})
	if err != nil {
		fmt.Printf("Got error while inserting: %v", err)
	}
}
