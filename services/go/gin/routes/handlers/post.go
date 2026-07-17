package handlers

import (
	"app"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func PostHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		body, err := ReadBody(c)
		if err != nil {
			return
		}

		task, err := ParseTask(c, body)
		if err != nil {
			return
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
			"method", c.Request.Method,
		)

		c.JSON(http.StatusCreated, gin.H{"id": id.String()})
	}
}
