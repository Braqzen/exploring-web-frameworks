package handlers

import (
	"gin/routes"

	"github.com/gin-gonic/gin"
)

func InvalidPathHandler(c *gin.Context) {
	routes.SendError(c, routes.AppErrors.InvalidPath)
}
