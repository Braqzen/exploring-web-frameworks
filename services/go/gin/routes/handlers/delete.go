package handlers

import (
	"app"
	"gin/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

func DeleteHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := ParseID(c)
		if err != nil {
			return
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(c)
			return
		}

		delete(state.Tasks, id)

		state.Mu.Unlock()

		slog.Info(
			"Removed task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request.Method,
		)

		c.Status(http.StatusNoContent)
	}
}
