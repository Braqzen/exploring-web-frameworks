package routes

import (
	"encoding/json"
	"net/http"
)

type AppError struct {
	status  int
	message string
}

var AppErrors = struct {
	TaskNotFound    AppError
	InvalidPath     AppError
	InvalidMethod   AppError
	InvalidJsonBody AppError
	Internal        AppError
}{
	TaskNotFound:    AppError{http.StatusNotFound, "Task not found"},
	InvalidPath:     AppError{http.StatusNotFound, "Invalid path"},
	InvalidMethod:   AppError{http.StatusMethodNotAllowed, "Invalid method"},
	InvalidJsonBody: AppError{http.StatusUnprocessableEntity, "Invalid body JSON"},
	Internal:        AppError{http.StatusInternalServerError, "Internal server error"},
}

func (self AppError) Error(w http.ResponseWriter) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(self.status)
	_ = json.NewEncoder(w).Encode(map[string]string{"error": self.message})
}
