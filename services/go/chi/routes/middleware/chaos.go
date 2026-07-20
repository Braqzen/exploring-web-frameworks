package middleware

import (
	"app"
	"chi/routes"
	"math/rand/v2"
	"net/http"
	"time"
)

func ChaosMiddleware(state *app.AppState) func(http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			latencyEnabled := state.Config.Latency.Enabled
			latencyRate := state.Config.Latency.Rate
			errorEnabled := state.Config.Error.Enabled
			errorRate := state.Config.Error.Rate

			if latencyEnabled && rand.IntN(101) < int(latencyRate) {
				delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
				time.Sleep(delay)
			}
			if errorEnabled && rand.IntN(101) < int(errorRate) {
				routes.AppErrors.Internal.Error(w)
				return
			}

			next.ServeHTTP(w, r)
		})
	}
}
