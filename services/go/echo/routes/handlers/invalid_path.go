package handlers

import (
	"echo/routes"
	"log/slog"

	"github.com/labstack/echo/v5"
)

func InvalidPathHandler(c *echo.Context) error {
	slog.Warn("Invalid path", "method", c.Request().Method, "path", c.Request().URL.Path)

	return routes.AppErrors.InvalidPath.Error(c)
}
