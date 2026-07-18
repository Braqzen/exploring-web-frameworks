package handlers

import (
	"fiber/routes"
	"log/slog"

	"github.com/gofiber/fiber/v3"
)

func InternalHandler(c fiber.Ctx, err any) error {
	slog.Error("Internal server error", "method", c.Method(), "path", c.Path(), "error", err)

	return routes.AppErrors.Internal.Error(c)
}
