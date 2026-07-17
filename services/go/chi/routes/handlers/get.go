package handlers

import (
	"app"
	"chi/routes"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
)

func GetHandler(state *app.AppState) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id, err := ParseID(w, r)
		if err != nil {
			return
		}

		state.Mu.RLock()
		task, ok := state.Tasks[id]
		state.Mu.RUnlock()

		if !ok {
			slog.Warn("Task not found", "method", r.Method, "path", r.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(w)
			return
		}

		slog.Info(
			"Retrieved task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", r.Method,
		)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		_ = json.NewEncoder(w).Encode(task)
	}
}
