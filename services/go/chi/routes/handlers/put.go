package handlers

import (
	"app"
	"chi/routes"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
)

func PutHandler(state *app.AppState) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id, err := ParseID(w, r)
		if err != nil {
			return
		}

		body, err := ReadBody(w, r)
		if err != nil {
			return
		}

		newTask, err := ParseTask(w, r, body)
		if err != nil {
			return
		}

		state.Mu.Lock()
		previousTask, ok := state.Tasks[id]
		if !ok {
			state.Mu.Unlock()
			slog.Warn("Task not found", "method", r.Method, "path", r.URL.Path, "id", id.String())
			routes.AppErrors.TaskNotFound.Error(w)
			return
		}
		state.Tasks[id] = newTask
		state.Mu.Unlock()

		slog.Info(
			"Overwrote task",
			"id", id.String(),
			"from_secret", len(previousTask.Secret),
			"to_secret", len(newTask.Secret),
			"from_operation", strings.ToLower(string(previousTask.Operation)),
			"to_operation", strings.ToLower(string(newTask.Operation)),
			"method", r.Method,
		)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		_ = json.NewEncoder(w).Encode(newTask)
	}
}
