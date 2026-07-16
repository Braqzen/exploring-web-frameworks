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
		id, err := app.ParseUUID(c.Param("id"))
		if err != nil {
			slog.Warn("Invalid path", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.SendError(c, routes.AppErrors.InvalidPath)
			return
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.SendError(c, routes.AppErrors.TaskNotFound)
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
