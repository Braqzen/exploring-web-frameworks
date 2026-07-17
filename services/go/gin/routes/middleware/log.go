package middleware

import (
	"log/slog"

	"github.com/gin-gonic/gin"
)

func LogMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		slog.Debug("Incoming request", "method", c.Request.Method, "path", c.Request.URL.Path)

		c.Next()
	}
}
