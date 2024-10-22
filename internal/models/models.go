package models

import (
	"time"
)

type Expense struct {
	created  time.Time
	modified time.Time
	value    float64
}
