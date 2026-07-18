package main

import (
	"log/slog"
	"os"
	"telemetry"
)

func main() {
	logLevel, ok := os.LookupEnv("LOG_LEVEL")
	if !ok || logLevel == "" {
		slog.Error("LOG_LEVEL env error")
		os.Exit(1)
	}

	tel := telemetry.NewTelemetry("echo", logLevel)
	err := tel.Start()
	if err != nil {
		slog.Error("Telemetry start failed", "err", err)
		os.Exit(1)
	}

	socket, ok := os.LookupEnv("SOCKET")
	if !ok || socket == "" {
		slog.Error("SOCKET env error", "err", err)
		os.Exit(1)
	}

	server, err := NewServer(socket)
	if err != nil {
		slog.Error("Bad socket", "err", err)
		os.Exit(1)
	}

	err = server.Run(tel)
	if err != nil {
		slog.Error("Server crash", "err", err)
		os.Exit(1)
	}
}
