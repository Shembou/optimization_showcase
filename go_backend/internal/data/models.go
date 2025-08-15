package data

import (
	"database/sql"

	"github.com/dgraph-io/ristretto/v2"
)

type Models struct {
	Users UserModel
}

func NewModels(db *sql.DB, cache *ristretto.Cache[string, string]) Models {
	return Models{
		Users: UserModel{
			DB:    db,
			Cache: cache,
		},
	}
}
