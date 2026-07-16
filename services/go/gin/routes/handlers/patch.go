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

func PatchHandler(state *app.AppState) gin.HandlerFunc {
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

		patched_task, err := app.ParsePatchedTask(body)
		if err != nil {
			slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.SendError(c, routes.AppErrors.InvalidJsonBody)
			return
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.SendError(c, routes.AppErrors.TaskNotFound)
			return
		}

		previous_operation := task.Operation
		task.Operation = patched_task.Operation
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Patched task",
			"id", id.String(),
			"secret", len(task.Secret),
			"from_operation", strings.ToLower(string(previous_operation)),
			"to_operation", strings.ToLower(string(patched_task.Operation)),
			"method", c.Request.Method,
		)

		c.JSON(http.StatusOK, task)
	}
}
