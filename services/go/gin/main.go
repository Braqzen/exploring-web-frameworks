package main

import (
	"log/slog"
	"os"
)

func main() {
	logLevel, ok := os.LookupEnv("LOG_LEVEL")
	if !ok || logLevel == "" {
		slog.Error("Missing log level")
		os.Exit(1)
	}

	// TODO: move into telemetry
	var level slog.Level
	err := level.UnmarshalText([]byte(logLevel))
	if err != nil {
		slog.Error("Bad log level")
		os.Exit(1)
	}
	slog.SetDefault(slog.New(slog.NewJSONHandler(os.Stdout, &slog.HandlerOptions{
		Level: level,
	})))

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
	if err != nil {
		slog.Error("Server crash")
		os.Exit(1)
	}
}
