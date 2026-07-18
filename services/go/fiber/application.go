package main

import (
	"app"
	"fiber/routes/handlers"
	"fiber/routes/middleware"

	"github.com/gofiber/fiber/v3"
	recoverer "github.com/gofiber/fiber/v3/middleware/recover"
)

const maxBodySize = 64 * 1024

type Application struct {
	Engine *fiber.App
	state  *app.AppState
}

func NewApplication() *Application {
	state := app.NewState()

	engine := fiber.New(fiber.Config{
		BodyLimit:    maxBodySize,
		ErrorHandler: handlers.ErrorHandler,
	})

	engine.Use(recoverer.New())
	engine.Use(middleware.LogMiddleware())
	engine.Use(middleware.ChaosMiddleware())

	engine.Post("/", handlers.PostHandler(state))
	engine.Get("/:id", handlers.GetHandler(state))
	engine.Delete("/:id", handlers.DeleteHandler(state))
	engine.Patch("/:id", handlers.PatchHandler(state))
	engine.Put("/:id", handlers.PutHandler(state))

	return &Application{Engine: engine, state: state}
}
