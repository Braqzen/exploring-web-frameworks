package handlers

import (
	"app"
	"gin/routes"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
)

func PatchHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := ParseID(c)
		if err != nil {
			return
		}

		body, err := ReadBody(c)
		if err != nil {
			return
		}

		patchedTask, err := ParsePatchedTask(c, body)
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

		previousOperation := task.Operation
		task.Operation = patchedTask.Operation
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Patched task",
			"id", id.String(),
			"secret", len(task.Secret),
			"from_operation", strings.ToLower(string(previousOperation)),
			"to_operation", strings.ToLower(string(patchedTask.Operation)),
			"method", c.Request.Method,
		)

		c.JSON(http.StatusOK, task)
	}
}
