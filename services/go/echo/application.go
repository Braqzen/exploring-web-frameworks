package main

import (
	"app"
	"echo/routes/handlers"
	"echo/routes/middleware"

	"github.com/labstack/echo/v5"
	echomw "github.com/labstack/echo/v5/middleware"
)

const maxBodySize = 64 * 1024

type Application struct {
	Engine *echo.Echo
	state  *app.AppState
}

func NewApplication() *Application {
	state := app.NewState()

	engine := echo.NewWithConfig(echo.Config{
		Router: echo.NewRouter(echo.RouterConfig{
			NotFoundHandler:         handlers.InvalidPathHandler,
			MethodNotAllowedHandler: handlers.InvalidMethodHandler,
		}),
	})
	engine.Use(echomw.BodyLimit(maxBodySize))
	engine.Use(middleware.RecoverMiddleware)
	engine.Use(middleware.LogMiddleware)
	engine.Use(middleware.ChaosMiddleware)

	engine.POST("/", handlers.PostHandler(state))
	engine.GET("/:id", handlers.GetHandler(state))
	engine.DELETE("/:id", handlers.DeleteHandler(state))
	engine.PATCH("/:id", handlers.PatchHandler(state))
	engine.PUT("/:id", handlers.PutHandler(state))

	return &Application{Engine: engine, state: state}
}
