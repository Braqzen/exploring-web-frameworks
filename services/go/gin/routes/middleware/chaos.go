package middleware

import (
	"gin/routes"
	"math/rand/v2"
	"time"

	"github.com/gin-gonic/gin"
)

func ChaosMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		if rand.IntN(101) < 5 {
			delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
			time.Sleep(delay)
		}
		if rand.IntN(101) < 5 {
			routes.AppErrors.Internal.Error(c)
			c.Abort()
			return
		}

		c.Next()
	}
}
