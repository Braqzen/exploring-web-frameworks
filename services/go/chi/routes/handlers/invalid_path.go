package handlers

import (
	"chi/routes"
	"log/slog"
	"net/http"
)

func InvalidPathHandler(w http.ResponseWriter, r *http.Request) {
	slog.Warn("Invalid path", "method", r.Method, "path", r.URL.Path)

	routes.AppErrors.InvalidPath.Error(w)
}
