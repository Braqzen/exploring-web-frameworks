package main

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net"
	"net/http"
	"os/signal"
	"syscall"
	"telemetry"
	"time"
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

	return &Server{socket: socket, app: NewApplication()}, nil
}

func (self *Server) Run(tel *telemetry.Telemetry) error {
	// Create signals to gracefully shutdown server
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	server := &http.Server{
		Addr:    self.socket,
		Handler: self.app.Engine,
	}

	// Channel used to send errors into to notify need to shutdown
	errCh := make(chan error, 1)

	slog.Info("Starting router", "socket", self.socket)

	// Start a server in the background
	go serveHTTP(server, errCh)

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

	shutdownErr := server.Shutdown(shutdownCtx)
	telemetryErr := tel.Shutdown(shutdownCtx)

	// Return a single error object
	return errors.Join(listenErr, shutdownErr, telemetryErr)
}

func serveHTTP(server *http.Server, errCh chan<- error) {
	err := server.ListenAndServe()
	if err != nil && err != http.ErrServerClosed {
		errCh <- err
		return
	}
	errCh <- nil
}
