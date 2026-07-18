package handlers

import (
	"fiber/routes"
	"log/slog"

	"github.com/gofiber/fiber/v3"
)

func InvalidPathHandler(c fiber.Ctx) error {
	slog.Warn("Invalid path", "method", c.Method(), "path", c.Path())

	return routes.AppErrors.InvalidPath.Error(c)
}
