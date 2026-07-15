package main

import (
	"context"
	"fmt"
	"net"
	"net/http"
	"os/signal"
	"syscall"
	"time"
)

type Server struct {
	Socket string
	App    Application
}

func NewServer(socket string) (*Server, error) {
	_, _, err := net.SplitHostPort(socket)
	if err != nil {
		return nil, fmt.Errorf("socket error: %w", err)
	}

	return &Server{Socket: socket, App: *NewApplication()}, nil
}

func (s *Server) Run() error {
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	srv := &http.Server{
		Addr:    s.Socket,
		Handler: s.App.Engine,
	}

	errCh := make(chan error, 1)
	go serveHTTP(srv, errCh)

	var listenErr error
	select {
	case listenErr = <-errCh:
	case <-ctx.Done():
		stop()
	}

	shutdownCtx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	shutdownErr := srv.Shutdown(shutdownCtx)
	// TODO: telemetry shutdown here later

	if listenErr != nil {
		return listenErr
	}
	return shutdownErr
}

func serveHTTP(srv *http.Server, errCh chan<- error) {
	err := srv.ListenAndServe()
	if err != nil && err != http.ErrServerClosed {
		errCh <- err
		return
	}
	errCh <- nil
}
