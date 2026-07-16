package handlers

import (
	"gin/routes"
	"log/slog"

	"github.com/gin-gonic/gin"
)

func InvalidMethodHandler(c *gin.Context) {
	slog.Warn("Invalid method", "method", c.Request.Method, "path", c.Request.URL.Path)

	routes.SendError(c, routes.AppErrors.InvalidMethod)
}
