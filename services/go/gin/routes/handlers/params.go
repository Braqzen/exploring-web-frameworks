package handlers

import (
	"app"
	"errors"
	"gin/routes"
	"io"
	"log/slog"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
)

func ParseID(c *gin.Context) (uuid.UUID, error) {
	id, err := app.ParseUUID(c.Param("id"))
	if err != nil {
		slog.Warn("Invalid path", "method", c.Request.Method, "path", c.Request.URL.Path)
		routes.AppErrors.InvalidPath.Error(c)
		return uuid.Nil, err
	}

	return id, nil
}

func ReadBody(c *gin.Context) ([]byte, error) {
	body, err := io.ReadAll(c.Request.Body)
	if err != nil {
		var sizeError *http.MaxBytesError
		if errors.As(err, &sizeError) {
			slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
			routes.AppErrors.InvalidJsonBody.Error(c)
			return nil, err
		}
		slog.Error("Internal server error", "method", c.Request.Method, "path", c.Request.URL.Path, "error", err)
		routes.AppErrors.Internal.Error(c)
		return nil, err
	}

	return body, nil
}

func ParsePatchedTask(c *gin.Context, body []byte) (app.PatchedTask, error) {
	patched_task, err := app.ParsePatchedTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
		routes.AppErrors.InvalidJsonBody.Error(c)
		return app.PatchedTask{}, err
	}

	return patched_task, nil
}

func ParseTask(c *gin.Context, body []byte) (app.Task, error) {
	task, err := app.ParseTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Request.Method, "path", c.Request.URL.Path)
		routes.AppErrors.InvalidJsonBody.Error(c)
		return app.Task{}, err
	}

	return task, nil
}
