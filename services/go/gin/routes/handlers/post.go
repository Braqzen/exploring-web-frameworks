package handlers

import (
	"app"
	"errors"
	"gin/routes"
	"io"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func PostHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		body, err := io.ReadAll(c.Request.Body)
		if err != nil {
			var sizeError *http.MaxBytesError
			if errors.As(err, &sizeError) {
				routes.SendError(c, routes.AppErrors.InvalidJsonBody)
				return
			}
			routes.SendError(c, routes.AppErrors.Internal)
			return
		}

		task, err := app.ParseTask(body)
		if err != nil {
			routes.SendError(c, routes.AppErrors.InvalidJsonBody)
			return
		}

		id := uuid.New()

		state.Mu.Lock()
		state.Tasks[id] = task
		state.Mu.Unlock()

		c.JSON(http.StatusCreated, gin.H{"id": id.String()})
	}
}
