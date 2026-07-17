package handlers

import (
	"chi/routes"
	"log/slog"
	"net/http"
)

func InvalidMethodHandler(w http.ResponseWriter, r *http.Request) {
	slog.Warn("Invalid method", "method", r.Method, "path", r.URL.Path)

	routes.AppErrors.InvalidMethod.Error(w)
}
