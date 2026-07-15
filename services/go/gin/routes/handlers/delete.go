package handlers

import (
	"app"
	"gin/routes"
	"net/http"

	"github.com/gin-gonic/gin"
)

func DeleteHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := app.ParseUUID(c.Param("id"))
		if err != nil {
			routes.SendError(c, routes.AppErrors.InvalidPath)
			return
		}

		state.Mu.Lock()

		if _, ok := state.Tasks[id]; !ok {
			state.Mu.Unlock()
			routes.SendError(c, routes.AppErrors.TaskNotFound)
			return
		}

		delete(state.Tasks, id)

		state.Mu.Unlock()

		c.Status(http.StatusNoContent)
	}
}
