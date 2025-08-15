package main

import (
	"log"
	"net/http"
	"os"

	"github.com/99designs/gqlgen/graphql/handler"
	"github.com/99designs/gqlgen/graphql/handler/extension"
	"github.com/99designs/gqlgen/graphql/handler/lru"
	"github.com/99designs/gqlgen/graphql/handler/transport"
	"github.com/99designs/gqlgen/graphql/playground"
	"github.com/Shembou/optimization_showcase/backend/cmd/graphql/graph"
	"github.com/Shembou/optimization_showcase/backend/configs"
	"github.com/Shembou/optimization_showcase/backend/internal/data"
	"github.com/vektah/gqlparser/v2/ast"

	_ "github.com/lib/pq"
)

const defaultPort = "8080"

type Application struct {
	Config configs.Config
	Models data.Models
	Logger *log.Logger
}

func main() {
	cfg := &configs.Config{}
	configuration := cfg.ConfigureServer()
	defer cfg.Database.Close()
	defer cfg.Cache.Close()

	app := &graph.Application{
		Config: configuration,
		Logger: configuration.Logger,
		Models: data.NewModels(configuration.Database, configuration.Cache),
	}

	port := os.Getenv("PORT")
	if port == "" {
		port = defaultPort
	}

	srv := handler.NewDefaultServer(graph.NewExecutableSchema(graph.Config{
		Resolvers: &graph.Resolver{App: app},
	}))

	srv.AddTransport(transport.Options{})
	srv.AddTransport(transport.GET{})
	srv.AddTransport(transport.POST{})

	srv.SetQueryCache(lru.New[*ast.QueryDocument](1000))

	srv.Use(extension.Introspection{})
	srv.Use(extension.AutomaticPersistedQuery{
		Cache: lru.New[string](100),
	})

	http.Handle("/", playground.Handler("GraphQL playground", "/query"))
	http.Handle("/query", srv)

	app.Logger.Printf("connect to http://localhost:%s/ for GraphQL playground", port)
	app.Logger.Fatal(http.ListenAndServeTLS(":"+port, "certs/cert.pem", "certs/key.pem", nil))
}
