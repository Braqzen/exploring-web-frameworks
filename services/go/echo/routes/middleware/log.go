package middleware

import (
	"log/slog"

	"github.com/labstack/echo/v5"
)

func LogMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c *echo.Context) error {
		slog.Debug("Incoming request", "method", c.Request().Method, "path", c.Request().URL.Path)
		return next(c)
	}
}
