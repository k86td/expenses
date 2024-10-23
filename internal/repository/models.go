package repository

import (
	"time"
)

type Expense struct {
	Created  time.Time
	Modified time.Time
	Uuid     string
	Data     string
}
