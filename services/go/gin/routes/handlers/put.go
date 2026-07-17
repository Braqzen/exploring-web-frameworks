package handlers

import (
	"app"
	"gin/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

func PutHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := ParseID(c)
		if err != nil {
			return
		}

		body, err := ReadBody(c)
		if err != nil {
			return
		}

		newTask, err := ParseTask(c, body)
		if err != nil {
			return
		}

		state.Mu.Lock()
		previousTask, ok := state.Tasks[id]
		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", c.Request.Method, "path", c.Request.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(c)
			return
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
			"method", c.Request.Method,
		)

		c.JSON(http.StatusOK, newTask)
	}
}
