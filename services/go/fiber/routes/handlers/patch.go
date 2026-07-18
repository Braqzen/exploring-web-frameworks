package handlers

import (
	"app"
	"fiber/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gofiber/fiber/v3"
)

func PatchHandler(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		patchedTask, err := ParsePatchedTask(c, c.Body())
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

		previousOperation := task.Operation
		task.Operation = patchedTask.Operation
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Patched task",
			"id", id.String(),
			"secret", len(task.Secret),
			"from_operation", strings.ToLower(string(previousOperation)),
			"to_operation", strings.ToLower(string(patchedTask.Operation)),
			"method", c.Method(),
		)

		return c.Status(http.StatusOK).JSON(task)
	}
}
