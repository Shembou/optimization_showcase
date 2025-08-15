package main

import (
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/Shembou/optimization_showcase/backend/configs"
	"github.com/Shembou/optimization_showcase/backend/internal/data"
	_ "github.com/lib/pq"
)

const version = "1.0.0"

type application struct {
	config configs.Config
	logger *log.Logger
	models data.Models
}

func main() {
	cfg := &configs.Config{}
	configuration := cfg.ConfigureServer()
	defer cfg.Database.Close()
	defer cfg.Cache.Close()

	app := &application{
		config: configuration,
		logger: configuration.Logger,
		models: data.NewModels(configuration.Database, configuration.Cache),
	}

	addr := fmt.Sprintf(":%d", configuration.Port)

	srv := &http.Server{
		Addr:         addr,
		Handler:      app.route(),
		IdleTimeout:  time.Minute,
		ReadTimeout:  10 * time.Second,
		WriteTimeout: 10 * time.Second,
	}

	app.logger.Printf("starting %s server on %s", configuration.Env, addr)

	err := srv.ListenAndServeTLS("certs/cert.pem", "certs/key.pem")
	if err != nil {
		app.logger.Fatalf("Error while serving at port 4000")
	}
	app.logger.Fatal(err)
}
