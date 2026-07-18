package handlers

import (
	"app"
	"echo/routes"
	"io"
	"log/slog"

	"github.com/google/uuid"
	"github.com/labstack/echo/v5"
)

func ParseID(c *echo.Context) (uuid.UUID, error) {
	id, err := app.ParseUUID(c.Param("id"))
	if err != nil {
		slog.Warn("Invalid path", "method", c.Request().Method, "path", c.Request().URL.Path)
		return uuid.Nil, routes.AppErrors.InvalidPath.Error(c)
	}

	return id, nil
}

func ReadBody(c *echo.Context) ([]byte, error) {
	body, err := io.ReadAll(c.Request().Body)
	if err != nil {
		slog.Error("Internal server error", "method", c.Request().Method, "path", c.Request().URL.Path, "error", err)
		return nil, routes.AppErrors.Internal.Error(c)
	}

	return body, nil
}

func ParsePatchedTask(c *echo.Context, body []byte) (app.PatchedTask, error) {
	patched_task, err := app.ParsePatchedTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Request().Method, "path", c.Request().URL.Path)
		return app.PatchedTask{}, routes.AppErrors.InvalidJsonBody.Error(c)
	}

	return patched_task, nil
}

func ParseTask(c *echo.Context, body []byte) (app.Task, error) {
	task, err := app.ParseTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Request().Method, "path", c.Request().URL.Path)
		return app.Task{}, routes.AppErrors.InvalidJsonBody.Error(c)
	}

	return task, nil
}
