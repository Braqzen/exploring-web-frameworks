package handlers

import (
	"app"
	"echo/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/labstack/echo/v5"
)

func GetHandler(state *app.AppState) echo.HandlerFunc {
	return func(c *echo.Context) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			slog.Warn("Task not found", "method", c.Request().Method, "path", c.Request().URL.Path, "id", id.String())
			return routes.AppErrors.TaskNotFound.Error(c)
		}

		slog.Info(
			"Retrieved task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request().Method,
		)

		return c.JSON(http.StatusOK, task)
	}
}
