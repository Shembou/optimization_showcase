package main

import (
	"fmt"
	"net/http"
)

func (app *application) healthcheck(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, http.StatusText(http.StatusMethodNotAllowed), http.StatusMethodNotAllowed)
		return
	}
	fmt.Fprintln(w, "status: available")
	fmt.Fprintf(w, "environment: %s\n", app.config.Env)
	fmt.Fprintf(w, "version: %s\n", version)
	app.logger.Print("Serving data at /v1/healthcheck endpoint")
}

func (app *application) getUsersHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method == http.MethodGet {
		users, err := app.models.Users.GetAll()
		if err != nil {
			http.Error(w, http.StatusText(http.StatusInternalServerError), http.StatusInternalServerError)
			app.logger.Fatalf("Error while getting users. %v", err)
			return
		}
		if err := app.writeJSON(w, http.StatusOK, envelope{"users": users}, nil); err != nil {
			http.Error(w, http.StatusText(http.StatusInternalServerError), http.StatusInternalServerError)
			app.logger.Fatalf("Error while writing JSON. %v", err)
			return
		}
	}
}
