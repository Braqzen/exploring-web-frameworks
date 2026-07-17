package main

import (
	"app"
	"chi/routes/handlers"
	"chi/routes/middleware"

	"github.com/go-chi/chi/v5"
	chimw "github.com/go-chi/chi/v5/middleware"
)

const maxBodySize = 64 * 1024

type Application struct {
	Engine *chi.Mux
	state  *app.AppState
}

func NewApplication() *Application {
	state := app.NewState()

	engine := chi.NewRouter()
	engine.Use(chimw.RequestSize(maxBodySize))
	engine.Use(middleware.RecoverMiddleware)
	engine.Use(middleware.LogMiddleware)
	engine.Use(middleware.ChaosMiddleware)

	engine.Post("/", handlers.PostHandler(state))
	engine.Get("/{id}", handlers.GetHandler(state))
	engine.Delete("/{id}", handlers.DeleteHandler(state))
	engine.Patch("/{id}", handlers.PatchHandler(state))
	engine.Put("/{id}", handlers.PutHandler(state))

	engine.NotFound(handlers.InvalidPathHandler)
	engine.MethodNotAllowed(handlers.InvalidMethodHandler)

	return &Application{Engine: engine, state: state}
}
