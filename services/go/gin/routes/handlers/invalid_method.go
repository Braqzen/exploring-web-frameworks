package handlers

import (
	"gin/routes"

	"github.com/gin-gonic/gin"
)

func InvalidMethodHandler(c *gin.Context) {
	routes.SendError(c, routes.AppErrors.InvalidMethod)
}
