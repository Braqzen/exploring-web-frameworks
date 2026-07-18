package routes

import (
	"net/http"

	"github.com/gofiber/fiber/v3"
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

func (self AppError) Error(c fiber.Ctx) error {
	return c.Status(self.status).JSON(fiber.Map{"error": self.message})
}
