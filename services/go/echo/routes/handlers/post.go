package handlers

import (
	"app"
	"log/slog"
	"net/http"
	"strings"

	"github.com/google/uuid"
	"github.com/labstack/echo/v5"
)

func PostHandler(state *app.AppState) echo.HandlerFunc {
	return func(c *echo.Context) error {
		body, err := ReadBody(c)
		if err != nil {
			return err
		}

		task, err := ParseTask(c, body)
		if err != nil {
			return err
		}

		id := uuid.New()

		state.Mu.Lock()
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Inserted new task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request().Method,
		)

		return c.JSON(http.StatusCreated, map[string]string{"id": id.String()})
	}
}
