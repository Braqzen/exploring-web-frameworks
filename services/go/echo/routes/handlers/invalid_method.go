package handlers

import (
	"echo/routes"
	"log/slog"

	"github.com/labstack/echo/v5"
)

func InvalidMethodHandler(c *echo.Context) error {
	slog.Warn("Invalid method", "method", c.Request().Method, "path", c.Request().URL.Path)

	return routes.AppErrors.InvalidMethod.Error(c)
}
