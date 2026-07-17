package handlers

import (
	"app"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"

	"github.com/google/uuid"
)

func PostHandler(state *app.AppState) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		body, err := ReadBody(w, r)
		if err != nil {
			return
		}

		task, err := ParseTask(w, r, body)
		if err != nil {
			return
		}

		id := uuid.New()

		state.Mu.Lock()
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Inserted new task",
			"id", id.String(),
			"secret", len(task.Secret),
			"operation", strings.ToLower(string(task.Operation)),
			"method", r.Method,
		)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusCreated)
		_ = json.NewEncoder(w).Encode(map[string]string{"id": id.String()})
	}
}
