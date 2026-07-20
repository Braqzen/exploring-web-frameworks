package main

import (
	"app"
	"chi/routes/handlers"
	"chi/routes/middleware"

	"github.com/go-chi/chi/v5"
	chimw "github.com/go-chi/chi/v5/middleware"
)

const BYTES = 1024

type Application struct {
	Engine *chi.Mux
	state  *app.AppState
}

func NewApplication(appConfig app.AppConfig) *Application {
	state := app.NewState(appConfig)

	engine := chi.NewRouter()
	engine.Use(chimw.RequestSize(int64(appConfig.RequestSizeLimit * BYTES)))
	engine.Use(middleware.RecoverMiddleware)
	engine.Use(middleware.LogMiddleware)
	engine.Use(middleware.ChaosMiddleware(state))

	engine.Post("/", handlers.PostHandler(state))
	engine.Get("/{id}", handlers.GetHandler(state))
	engine.Delete("/{id}", handlers.DeleteHandler(state))
	engine.Patch("/{id}", handlers.PatchHandler(state))
	engine.Put("/{id}", handlers.PutHandler(state))

	engine.NotFound(handlers.InvalidPathHandler)
	engine.MethodNotAllowed(handlers.InvalidMethodHandler)

	return &Application{Engine: engine, state: state}
}
