package handlers

import (
	"app"
	"log/slog"
	"net/http"
	"strings"

	"github.com/gofiber/fiber/v3"
	"github.com/google/uuid"
)

func PostHandler(state *app.AppState) fiber.Handler {
	return func(c fiber.Ctx) error {
		task, err := ParseTask(c, c.Body())
		if err != nil {
			return err
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
			"method", c.Method(),
		)

		return c.Status(http.StatusCreated).JSON(fiber.Map{"id": id.String()})
	}
}
