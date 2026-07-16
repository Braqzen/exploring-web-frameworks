package handlers

import (
	"gin/routes"
	"log/slog"

	"github.com/gin-gonic/gin"
)

func InternalHandler(c *gin.Context, err any) {
	slog.Error("Internal server error", "method", c.Request.Method, "path", c.Request.URL.Path, "error", err)

	routes.SendError(c, routes.AppErrors.Internal)
}
