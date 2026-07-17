package handlers

import (
	"app"
	"chi/routes"
	"log/slog"
	"net/http"
	"strings"
)

func DeleteHandler(state *app.AppState) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id, err := ParseID(w, r)
		if err != nil {
			return
		}

		state.Mu.Lock()
		task, ok := state.Tasks[id]

		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", r.Method, "path", r.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(w)
			return
		}

		delete(state.Tasks, id)

		state.Mu.Unlock()

		slog.Info(
			"Removed task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", r.Method,
		)

		w.WriteHeader(http.StatusNoContent)
	}
}
