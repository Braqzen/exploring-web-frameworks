package handlers

import (
	"gin/routes"
	"log/slog"

	"github.com/gin-gonic/gin"
)

func InvalidPathHandler(c *gin.Context) {
	slog.Warn("Invalid path", "method", c.Request.Method, "path", c.Request.URL.Path)

	routes.SendError(c, routes.AppErrors.InvalidPath)
}
