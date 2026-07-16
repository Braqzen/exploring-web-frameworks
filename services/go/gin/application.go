package main

import (
	"app"
	"gin/routes/handlers"
	"gin/routes/middleware"

	"github.com/gin-gonic/gin"
)

type Application struct {
	Engine *gin.Engine
	State  *app.AppState
}

func NewApplication() *Application {
	state := app.NewState()

	gin.SetMode(gin.ReleaseMode)

	engine := gin.New()
	engine.HandleMethodNotAllowed = true
	engine.Use(gin.CustomRecovery(handlers.InternalHandler))
	engine.Use(middleware.BodySizeMiddleware())
	engine.Use(middleware.LogMiddleware())
	engine.SetTrustedProxies(nil)

	engine.POST("/", handlers.PostHandler(state))
	engine.GET("/:id", handlers.GetHandler(state))
	engine.DELETE("/:id", handlers.DeleteHandler(state))
	engine.PATCH("/:id", handlers.PatchHandler(state))
	engine.PUT("/:id", handlers.PutHandler(state))

	engine.NoRoute(handlers.InvalidPathHandler)
	engine.NoMethod(handlers.InvalidMethodHandler)

	return &Application{Engine: engine, State: state}
}
