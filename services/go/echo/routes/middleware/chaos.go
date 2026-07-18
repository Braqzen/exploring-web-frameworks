package middleware

import (
	"echo/routes"
	"math/rand/v2"
	"time"

	"github.com/labstack/echo/v5"
)

func ChaosMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c *echo.Context) error {
		if rand.IntN(101) < 5 {
			delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
			time.Sleep(delay)
		}
		if rand.IntN(101) < 5 {
			return routes.AppErrors.Internal.Error(c)
		}

		return next(c)
	}
}
