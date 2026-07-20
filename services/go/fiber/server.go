package main

import (
	"app"
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"os/signal"
	"syscall"
	"telemetry"
	"time"

	"github.com/gofiber/fiber/v3"
)

type Server struct {
	socket string
	app    *Application
}

func NewServer(socket string) (*Server, error) {
	// Check if valid socket format
	_, _, err := net.SplitHostPort(socket)
	if err != nil {
		return nil, fmt.Errorf("socket error: %w", err)
	}

	appConfig, err := app.NewAppConfig()
	if err != nil {
		return nil, err
	}

	return &Server{socket: socket, app: NewApplication(appConfig)}, nil
}

func (self *Server) Run(tel *telemetry.Telemetry) error {
	// Create signals to gracefully shutdown server
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	// Channel used to send errors into to notify need to shutdown
	errCh := make(chan error, 1)

	slog.Info("Starting router", "socket", self.socket)

	// Start a server in the background
	go serveHTTP(self.app.Engine, self.socket, errCh)

	// Block/wait for an error or shutdown signal to exit
	var listenErr error
	select {
	case listenErr = <-errCh:
	case <-ctx.Done():
	}

	stop()

	// Objects have 2 seconds to cleanup before forceful shutdown
	shutdownCtx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	shutdownErr := self.app.Engine.ShutdownWithContext(shutdownCtx)
	telemetryErr := tel.Shutdown(shutdownCtx)

	// Return a single error object
	return errors.Join(listenErr, shutdownErr, telemetryErr)
}

func serveHTTP(app *fiber.App, socket string, errCh chan<- error) {
	err := app.Listen(socket)
	if err != nil {
		errCh <- err
		return
	}
	errCh <- nil
}
