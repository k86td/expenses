package repository

import (
	"time"
)

type Expense struct {
	created  time.Time
	modified time.Time
	uuid     string
	data     string
}
