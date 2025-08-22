package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/websocket"
)

var upgrader = websocket.Upgrader{
	ReadBufferSize:  1024,
	WriteBufferSize: 1024,
	CheckOrigin: func(r *http.Request) bool {
		return true
	},
}

func (app *application) websocket(w http.ResponseWriter, r *http.Request) {
	conn, err := upgrader.Upgrade(w, r, nil)
	if err != nil {
		app.logger.Println("Failed to initialize websocket connection")
		return
	}
	defer conn.Close()
	for {
		mt, message, err := conn.ReadMessage()
		if err != nil {
			app.logger.Println("read:", err)
			break
		}
		app.logger.Printf("recv: %s", message)

		users, err := app.models.Users.GetAll()
		if err != nil {
			app.logger.Println("Error while getting all users")
		}

		js, err := json.Marshal(users)
		if err != nil {
			app.logger.Println("Error while converting users to JSON")
		}

		err = conn.WriteMessage(mt, js)
		if err != nil {
			log.Println("write:", err)
			break
		}
	}

}

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
