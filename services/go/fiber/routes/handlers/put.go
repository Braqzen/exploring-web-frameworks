package handlers

import (
	"app"
	"fiber/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gofiber/fiber/v3"
)

func PutHandler(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		newTask, err := ParseTask(c, c.Body())
		if err != nil {
			return err
		}

		state.Mu.Lock()
		previousTask, ok := state.Tasks[id]
		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Method(), "path", c.Path(), "id", id.String())
			return routes.AppErrors.TaskNotFound.Error(c)
		}
		state.Tasks[id] = newTask
		state.Mu.Unlock()

		slog.Info(
			"Overwrote task",
			"id", id.String(),
			"from_secret", len(previousTask.Secret),
			"to_secret", len(newTask.Secret),
			"from_operation", strings.ToLower(string(previousTask.Operation)),
			"to_operation", strings.ToLower(string(newTask.Operation)),
			"method", c.Method(),
		)

		return c.Status(http.StatusOK).JSON(newTask)
	}
}
