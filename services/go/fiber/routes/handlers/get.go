package handlers

import (
	"app"
	"fiber/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gofiber/fiber/v3"
)

func GetHandler(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			slog.Warn("Task not found", "method", c.Method(), "path", c.Path(), "id", id.String())
			return routes.AppErrors.TaskNotFound.Error(c)
		}

		slog.Info(
			"Retrieved task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Method(),
		)

		return c.Status(http.StatusOK).JSON(task)
	}
}
