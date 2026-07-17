package handlers

import (
	"app"
	"chi/routes"
	"errors"
	"io"
	"log/slog"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/google/uuid"
)

func ParseID(w http.ResponseWriter, r *http.Request) (uuid.UUID, error) {
	id, err := app.ParseUUID(chi.URLParam(r, "id"))
	if err != nil {
		slog.Warn("Invalid path", "method", r.Method, "path", r.URL.Path)
		routes.AppErrors.InvalidPath.Error(w)
		return uuid.Nil, err
	}

	return id, nil
}

func ReadBody(w http.ResponseWriter, r *http.Request) ([]byte, error) {
	body, err := io.ReadAll(r.Body)
	if err != nil {
		var sizeError *http.MaxBytesError
		if errors.As(err, &sizeError) {
			slog.Warn("Invalid body JSON", "method", r.Method, "path", r.URL.Path)
			routes.AppErrors.InvalidJsonBody.Error(w)
			return nil, err
		}
		slog.Error("Internal server error", "method", r.Method, "path", r.URL.Path, "error", err)
		routes.AppErrors.Internal.Error(w)
		return nil, err
	}

	return body, nil
}

func ParsePatchedTask(w http.ResponseWriter, r *http.Request, body []byte) (app.PatchedTask, error) {
	patched_task, err := app.ParsePatchedTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", r.Method, "path", r.URL.Path)
		routes.AppErrors.InvalidJsonBody.Error(w)
		return app.PatchedTask{}, err
	}

	return patched_task, nil
}

func ParseTask(w http.ResponseWriter, r *http.Request, body []byte) (app.Task, error) {
	task, err := app.ParseTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", r.Method, "path", r.URL.Path)
		routes.AppErrors.InvalidJsonBody.Error(w)
		return app.Task{}, err
	}

	return task, nil
}
