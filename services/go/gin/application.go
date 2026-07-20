package main

import (
	"app"
	"gin/routes/handlers"
	"gin/routes/middleware"

	"github.com/gin-gonic/gin"
)

type Application struct {
	Engine *gin.Engine
	state  *app.AppState
}

func NewApplication(appConfig app.AppConfig) *Application {
	state := app.NewState(appConfig)

	gin.SetMode(gin.ReleaseMode)

	engine := gin.New()
	engine.HandleMethodNotAllowed = true
	engine.SetTrustedProxies(nil)
	engine.Use(gin.CustomRecovery(handlers.InternalHandler))
	engine.Use(middleware.BodySizeMiddleware())
	engine.Use(middleware.LogMiddleware())
	engine.Use(middleware.ChaosMiddleware(state))

	engine.POST("/", handlers.PostHandler(state))
	engine.GET("/:id", handlers.GetHandler(state))
	engine.DELETE("/:id", handlers.DeleteHandler(state))
	engine.PATCH("/:id", handlers.PatchHandler(state))
	engine.PUT("/:id", handlers.PutHandler(state))

	engine.NoRoute(handlers.InvalidPathHandler)
	engine.NoMethod(handlers.InvalidMethodHandler)

	return &Application{Engine: engine, state: state}
}
