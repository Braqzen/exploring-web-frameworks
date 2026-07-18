package handlers

import (
	"app"
	"echo/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/labstack/echo/v5"
)

func PutHandler(state *app.AppState) echo.HandlerFunc {
	return func(c *echo.Context) error {
		id, err := ParseID(c)
		if err != nil {
			return err
		}

		body, err := ReadBody(c)
		if err != nil {
			return err
		}

		newTask, err := ParseTask(c, body)
		if err != nil {
			return err
		}

		state.Mu.Lock()
		previousTask, ok := state.Tasks[id]
		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request().Method, "path", c.Request().URL.Path, "id", id.String())
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
			"method", c.Request().Method,
		)

		return c.JSON(http.StatusOK, newTask)
	}
}
