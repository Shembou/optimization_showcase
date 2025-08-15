package graph

import (
	"log"

	"github.com/Shembou/optimization_showcase/backend/configs"
	"github.com/Shembou/optimization_showcase/backend/internal/data"
)

// This file will not be regenerated automatically.
//
// It serves as dependency injection for your app, add any dependencies you require here.

type Application struct {
	Config configs.Config
	Models data.Models
	Logger *log.Logger
}

type Resolver struct {
	App *Application
}
