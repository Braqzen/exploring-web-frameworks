package main

import (
	"context"
	"log/slog"
	"os"
	"telemetry"
)

func main() {
	logLevel, ok := os.LookupEnv("LOG_LEVEL")
	if !ok || logLevel == "" {
		slog.Error("Missing log level")
		os.Exit(1)
	}

	logger := telemetry.NewLogger("gin", logLevel)
	err := logger.Start()
	if err != nil {
		slog.Error("Logger start failed", "err", err)
		os.Exit(1)
	}

	socket, ok := os.LookupEnv("SOCKET")
	if !ok || socket == "" {
		slog.Error("Socket error")
		os.Exit(1)
	}

	server, err := NewServer(socket)
	if err != nil {
		slog.Error("Bad socket")
		os.Exit(1)
	}

	err = server.Run()
	shutdownErr := logger.Shutdown(context.Background())
	if err != nil {
		slog.Error("Server crash")
		os.Exit(1)
	}
	if shutdownErr != nil {
		slog.Error("Logger shutdown failed")
		os.Exit(1)
	}
}
