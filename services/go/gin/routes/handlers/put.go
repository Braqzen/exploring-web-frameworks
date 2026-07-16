package handlers

import (
	"app"
	"errors"
	"gin/routes"
	"io"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

func PutHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := app.ParseUUID(c.Param("id"))
		if err != nil {
			slog.Warn("Invalid path", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.SendError(c, routes.AppErrors.InvalidPath)
			return
		}

		body, err := io.ReadAll(c.Request.Body)
		if err != nil {
			var sizeError *http.MaxBytesError
			if errors.As(err, &sizeError) {
				slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
				routes.SendError(c, routes.AppErrors.InvalidJsonBody)
				return
			}
			slog.Error("Internal server error", "method", c.Request.Method, "path", c.Request.URL.Path, "error", err)
			routes.SendError(c, routes.AppErrors.Internal)
			return
		}

		new_task, err := app.ParseTask(body)
		if err != nil {
			slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.SendError(c, routes.AppErrors.InvalidJsonBody)
			return
		}

		state.Mu.Lock()
		previous_task, ok := state.Tasks[id]
		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.SendError(c, routes.AppErrors.TaskNotFound)
			return
		}
		state.Tasks[id] = new_task
		state.Mu.Unlock()

		slog.Info(
			"Overwrote task",
			"id", id.String(),
			"from_secret", len(previous_task.Secret),
			"to_secret", len(new_task.Secret),
			"from_operation", strings.ToLower(string(previous_task.Operation)),
			"to_operation", strings.ToLower(string(new_task.Operation)),
			"method", c.Request.Method,
		)

		c.JSON(http.StatusOK, new_task)
	}
}
