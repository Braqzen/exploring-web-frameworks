package telemetry

import (
	"context"
	"errors"
	"log/slog"
)

type Telemetry struct {
	logger   *Logger
	profiler *Profiler
}

func NewTelemetry(serviceName string, logLevel string) *Telemetry {
	return &Telemetry{
		logger:   NewLogger(serviceName, logLevel),
		profiler: NewProfiler(serviceName),
	}
}

func (t *Telemetry) Start() error {
	logErr := t.logger.Start()
	if logErr != nil {
		return logErr
	}

	profErr := t.profiler.Start()
	if profErr != nil {
		return profErr
	}

	return nil
}

func (t *Telemetry) Shutdown(ctx context.Context) error {
	var shutdownErr error

	profErr := t.profiler.Shutdown()
	if profErr != nil {
		slog.Error("Profiler shutdown failed", "err", profErr)
		shutdownErr = errors.Join(shutdownErr, profErr)
	}

	logErr := t.logger.Shutdown(ctx)
	if logErr != nil {
		slog.Error("Logger shutdown failed", "err", logErr)
		shutdownErr = errors.Join(shutdownErr, logErr)
	}

	return shutdownErr
}
