package main

import (
	"fmt"
	"os"

	repository "k86td/expenses/internal/repository"
)

func main() {
	repo, err := repository.OpenOrCreateDatabase("testing.db")
	if err != nil {
		os.Exit(1)
	}
	defer repo.Db.Close()

	fmt.Printf("Opened DB: %v\n", repo)

	// now := time.Now()
	// err = repo.Create(repository.Expense{
	// 	Created:  now,
	// 	Modified: now,
	// 	Uuid:     uuid.New().String(),
	// 	Data:     "{'testing': true}",
	// })
	// if err != nil {
	// 	fmt.Printf("Got error while inserting: %v", err)
	// }

	exps, err := repo.GetAll(1)
	if err != nil {
		fmt.Printf("Got error while fetching inside DB: %v\n", err)
	}
	fmt.Printf("expenses: %v", exps)
}
