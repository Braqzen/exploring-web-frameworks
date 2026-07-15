package handlers

import (
	"gin/routes"

	"github.com/gin-gonic/gin"
)

func InternalHandler(c *gin.Context, _ any) {
	routes.SendError(c, routes.AppErrors.Internal)
}
