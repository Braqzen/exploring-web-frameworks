package middleware

import (
	"fiber/routes"
	"math/rand/v2"
	"time"

	"github.com/gofiber/fiber/v3"
)

func ChaosMiddleware() fiber.Handler {
	return func(c fiber.Ctx) error {
		if rand.IntN(101) < 5 {
			delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
			time.Sleep(delay)
		}
		if rand.IntN(101) < 5 {
			return routes.AppErrors.Internal.Error(c)
		}

		return c.Next()
	}
}
