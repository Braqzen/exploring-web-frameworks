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
	"github.com/google/uuid"
)

func PostHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
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

		task, err := app.ParseTask(body)
		if err != nil {
			slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.SendError(c, routes.AppErrors.InvalidJsonBody)
			return
		}

		id := uuid.New()

		state.Mu.Lock()
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info("Inserted new task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", c.Request.Method,
		)

		c.JSON(http.StatusCreated, gin.H{"id": id.String()})
	}
}
