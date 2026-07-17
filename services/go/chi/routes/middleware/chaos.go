package middleware

import (
	"chi/routes"
	"math/rand/v2"
	"net/http"
	"time"
)

func ChaosMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if rand.IntN(101) < 5 {
			delay := time.Duration(500+rand.IntN(1001)) * time.Microsecond
			time.Sleep(delay)
		}
		if rand.IntN(101) < 5 {
			routes.AppErrors.Internal.Error(w)
			return
		}
		next.ServeHTTP(w, r)
	})
}
