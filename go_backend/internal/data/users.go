package data

import (
	"database/sql"
	"encoding/json"
	"time"

	"github.com/dgraph-io/ristretto/v2"
)

type User struct {
	ID       int64  `json:"id"`
	Name     string `json:"name"`
	Language string `json:"language"`
	Bio      string `json:"bio"`
	Version  int64  `json:"version"`
}

type UserModel struct {
	DB    *sql.DB
	Cache *ristretto.Cache[string, string]
}

func (u UserModel) GetAll() ([]*User, error) {
	if cached, found := u.Cache.Get("users"); found {
		var users []*User
		if err := json.Unmarshal([]byte(cached), &users); err == nil {
			println("Returning from cache")
			return users, nil
		}
	}

	query := `
		SELECT * FROM users;`
	rows, err := u.DB.Query(query)
	if err != nil {
		println("Query execution failed")
		return nil, err
	}
	println("Finished query execution")
	defer rows.Close()

	users := []*User{}
	for rows.Next() {
		var user User
		err := rows.Scan(
			&user.ID,
			&user.Name,
			&user.Language,
			&user.Bio,
			&user.Version,
		)
		if err != nil {
			return nil, err
		}
		users = append(users, &user)
	}
	if err = rows.Err(); err != nil {
		return nil, err
	}

	js, err := json.MarshalIndent(users, "", "\t")
	if err != nil {
		return nil, err
	}
	if _, found := u.Cache.Get("users"); found {
		return users, nil
	}
	u.Cache.SetWithTTL("users", string(js), 1, 60*time.Second)

	return users, nil
}
