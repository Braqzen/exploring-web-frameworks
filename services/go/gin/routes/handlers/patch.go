package handlers

import (
	"app"
	"errors"
	"gin/routes"
	"io"
	"net/http"

	"github.com/gin-gonic/gin"
)

func PatchHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := app.ParseUUID(c.Param("id"))
		if err != nil {
			routes.SendError(c, routes.AppErrors.InvalidPath)
			return
		}

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

		patched_task, err := app.ParsePatchedTask(body)
		if err != nil {
			routes.SendError(c, routes.AppErrors.InvalidJsonBody)
			return
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			routes.SendError(c, routes.AppErrors.TaskNotFound)
			return
		}

		task.Operation = patched_task.Operation
		state.Tasks[id] = task
		state.Mu.Unlock()

		c.JSON(http.StatusOK, task)
	}
}
