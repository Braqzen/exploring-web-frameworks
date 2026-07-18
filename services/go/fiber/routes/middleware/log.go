package middleware

import (
	"log/slog"

	"github.com/gofiber/fiber/v3"
)

func LogMiddleware() fiber.Handler {
	return func(c fiber.Ctx) error {
		slog.Debug("Incoming request", "method", c.Method(), "path", c.Path())

		return c.Next()
	}
}
