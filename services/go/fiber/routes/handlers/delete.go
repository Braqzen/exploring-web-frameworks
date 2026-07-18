package handlers

import (
	"app"
	"fiber/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gofiber/fiber/v3"
)

func DeleteHandler(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Method(), "path", c.Path(), "id", id.String())
			return routes.AppErrors.TaskNotFound.Error(c)
		}

		delete(state.Tasks, id)

		state.Mu.Unlock()

		slog.Info(
			"Removed task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Method(),
		)

		return c.SendStatus(http.StatusNoContent)
	}
}
