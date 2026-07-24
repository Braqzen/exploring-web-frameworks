package handlers

import (
	"app"
	"fiber/routes"
	"log/slog"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"
)

func ParseID(c fiber.Ctx) (uuid.UUID, error) {
	id, err := app.ParseUUID(c.Params("id"))
	if err != nil {
		slog.Warn("Invalid path", "method", c.Method(), "path", c.Path())
		response_err := routes.AppErrors.InvalidPath.Error(c)
		if response_err != nil {
			return uuid.Nil, response_err
		}
		return uuid.Nil, errResponseHandled
	}

	return id, nil
}

func ParsePatchedTask(c fiber.Ctx, body []byte) (app.PatchedTask, error) {
	patched_task, err := app.ParsePatchedTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Method(), "path", c.Path())
		response_err := routes.AppErrors.InvalidJsonBody.Error(c)
		if response_err != nil {
			return app.PatchedTask{}, response_err
		}
		return app.PatchedTask{}, errResponseHandled
	}

	return patched_task, nil
}

func ParseTask(c fiber.Ctx, body []byte) (app.Task, error) {
	task, err := app.ParseTask(body)
	if err != nil {
		slog.Warn("Invalid body JSON", "method", c.Method(), "path", c.Path())
		response_err := routes.AppErrors.InvalidJsonBody.Error(c)
		if response_err != nil {
			return app.Task{}, response_err
		}
		return app.Task{}, errResponseHandled
	}

	return task, nil
}
