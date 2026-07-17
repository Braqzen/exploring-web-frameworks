package middleware

import (
	"chi/routes"
	"log/slog"
	"net/http"
)

func RecoverMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		defer recoverPanic(w, r)
		next.ServeHTTP(w, r)
	})
}

func recoverPanic(w http.ResponseWriter, r *http.Request) {
	err := recover()
	if err == nil {
		return
	}
	slog.Error("Internal server error", "method", r.Method, "path", r.URL.Path, "error", err)
	routes.AppErrors.Internal.Error(w)
}
