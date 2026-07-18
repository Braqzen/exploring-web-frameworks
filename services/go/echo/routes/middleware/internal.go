package middleware

import (
	"echo/routes"
	"log/slog"

	"github.com/labstack/echo/v5"
)

func RecoverMiddleware(next echo.HandlerFunc) echo.HandlerFunc {
	return func(c *echo.Context) (err error) {
		defer recoverPanic(c, &err)
		return next(c)
	}
}

func recoverPanic(c *echo.Context, err *error) {
	r := recover()
	if r == nil {
		return
	}
	slog.Error("Internal server error", "method", c.Request().Method, "path", c.Request().URL.Path, "error", r)
	*err = routes.AppErrors.Internal.Error(c)
}
