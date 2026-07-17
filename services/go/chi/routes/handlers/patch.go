package handlers

import (
	"app"
	"chi/routes"
	"encoding/json"
	"log/slog"
	"net/http"
	"strings"
)

func PatchHandler(state *app.AppState) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		id, err := ParseID(w, r)
		if err != nil {
			return
		}

		body, err := ReadBody(w, r)
		if err != nil {
			return
		}

		patchedTask, err := ParsePatchedTask(w, r, body)
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

		previousOperation := task.Operation
		task.Operation = patchedTask.Operation
		state.Tasks[id] = task
		state.Mu.Unlock()

		slog.Info(
			"Patched task",
			"id", id.String(),
			"secret", len(task.Secret),
			"from_operation", strings.ToLower(string(previousOperation)),
			"to_operation", strings.ToLower(string(patchedTask.Operation)),
			"method", r.Method,
		)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		_ = json.NewEncoder(w).Encode(task)
	}
}
