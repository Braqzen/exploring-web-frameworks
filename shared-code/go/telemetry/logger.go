package telemetry

import (
	"context"
	"log/slog"

	"go.opentelemetry.io/contrib/bridges/otelslog"
	"go.opentelemetry.io/otel/exporters/otlp/otlplog/otlploghttp"
	"go.opentelemetry.io/otel/log/global"
	"go.opentelemetry.io/otel/sdk/log"
	"go.opentelemetry.io/otel/sdk/resource"
	semconv "go.opentelemetry.io/otel/semconv/v1.41.0"
)

type Logger struct {
	ServiceName string
	LogLevel    string
	Provider    *log.LoggerProvider
}

type levelHandler struct {
	slog.Handler
	level slog.Level
}

func (h levelHandler) Enabled(ctx context.Context, level slog.Level) bool {
	return level >= h.level && h.Handler.Enabled(ctx, level)
}

func NewLogger(serviceName string, logLevel string) *Logger {
	return &Logger{
		ServiceName: serviceName,
		LogLevel:    logLevel,
	}
}

func (l *Logger) Start() error {
	if l.Provider != nil {
		return nil
	}

	var level slog.Level
	err := level.UnmarshalText([]byte(l.LogLevel))
	if err != nil {
		return err
	}

	ctx := context.Background()

	exporter, err := otlploghttp.New(ctx)
	if err != nil {
		return err
	}

	res, err := resource.Merge(resource.Default(), resource.NewWithAttributes(semconv.SchemaURL, semconv.ServiceName(l.ServiceName)))
	if err != nil {
		return err
	}

	provider := log.NewLoggerProvider(log.WithResource(res), log.WithProcessor(log.NewBatchProcessor(exporter)))
	l.Provider = provider

	global.SetLoggerProvider(provider)
	handler := levelHandler{
		Handler: otelslog.NewHandler(
			l.ServiceName,
			otelslog.WithLoggerProvider(provider),
		),
		level: level,
	}
	slog.SetDefault(slog.New(handler))

	return nil
}

func (l *Logger) Shutdown(ctx context.Context) error {
	if l.Provider == nil {
		return nil
	}
	return l.Provider.Shutdown(ctx)
}
