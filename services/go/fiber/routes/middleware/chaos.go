package middleware

import (
	"app"
	"fiber/routes"
	"math/rand/v2"
	"time"

	"github.com/gofiber/fiber/v3"
)

func ChaosMiddleware(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		latencyEnabled := state.Config.Latency.Enabled
		latencyRate := state.Config.Latency.Rate
		errorEnabled := state.Config.Error.Enabled
		errorRate := state.Config.Error.Rate

		if latencyEnabled && rand.IntN(101) < int(latencyRate) {
			delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
			time.Sleep(delay)
		}
		if errorEnabled && rand.IntN(101) < int(errorRate) {
			return routes.AppErrors.Internal.Error(c)
		}

		return c.Next()
	}
}
