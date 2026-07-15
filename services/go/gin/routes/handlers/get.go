package handlers

import (
	"app"
	"gin/routes"
	"net/http"

	"github.com/gin-gonic/gin"
)

func GetHandler(state *app.AppState) gin.HandlerFunc {
	return func(c *gin.Context) {
		id, err := app.ParseUUID(c.Param("id"))
		if err != nil {
			routes.SendError(c, routes.AppErrors.InvalidPath)
			return
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			routes.SendError(c, routes.AppErrors.TaskNotFound)
			return
		}

		c.JSON(http.StatusOK, task)
	}
}
