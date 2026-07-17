package handlers

import (
	"app"
	"gin/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

func GetHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := ParseID(c)
		if err != nil {
			return
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(c)
			return
		}

		slog.Info(
			"Retrieved task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request.Method,
		)

		c.JSON(http.StatusOK, task)
	}
}
