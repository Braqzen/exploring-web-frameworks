package handlers

import (
	"fiber/routes"
	"log/slog"

	"github.com/gofiber/fiber/v3"
)

func InvalidMethodHandler(c fiber.Ctx) error {
	slog.Warn("Invalid method", "method", c.Method(), "path", c.Path())

	return routes.AppErrors.InvalidMethod.Error(c)
}
