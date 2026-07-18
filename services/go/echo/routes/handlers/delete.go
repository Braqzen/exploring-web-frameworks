package handlers

import (
	"app"
	"echo/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/labstack/echo/v5"
)

func DeleteHandler(state *app.AppState) echo.HandlerFunc {
	return func(c *echo.Context) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request().Method, "path", c.Request().URL.Path, "id", id.String())
			return routes.AppErrors.TaskNotFound.Error(c)
		}

		delete(state.Tasks, id)

		state.Mu.Unlock()

		slog.Info(
			"Removed task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request().Method,
		)

		return c.NoContent(http.StatusNoContent)
	}
}
