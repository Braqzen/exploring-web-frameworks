package handlers

import (
	"errors"

	"github.com/gofiber/fiber/v3"
)

var errResponseHandled = errors.New("response handled")

func ErrorHandler(c fiber.Ctx, err error) error {
	if errors.Is(err, errResponseHandled) {
		return nil
	}
	if errors.Is(err, fiber.ErrNotFound) {
		return InvalidPathHandler(c)
	}
	if errors.Is(err, fiber.ErrMethodNotAllowed) {
		return InvalidMethodHandler(c)
	}
	return InternalHandler(c, err)
}
