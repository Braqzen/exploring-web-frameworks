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
	serviceName string
	logLevel    string
	provider    *log.LoggerProvider
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
		serviceName: serviceName,
		logLevel:    logLevel,
	}
}

func (l *Logger) Start() error {
	if l.provider != nil {
		return nil
	}

	var level slog.Level
	err := level.UnmarshalText([]byte(l.logLevel))
	if err != nil {
		return err
	}

	ctx := context.Background()

	exporter, err := otlploghttp.New(ctx)
	if err != nil {
		return err
	}

	res, err := resource.Merge(
		resource.Default(),
		resource.NewWithAttributes(semconv.SchemaURL, semconv.ServiceName(l.serviceName)),
	)
	if err != nil {
		return err
	}

	provider := log.NewLoggerProvider(log.WithResource(res), log.WithProcessor(log.NewBatchProcessor(exporter)))
	l.provider = provider

	global.SetLoggerProvider(provider)
	handler := levelHandler{
		Handler: otelslog.NewHandler(
			l.serviceName,
			otelslog.WithLoggerProvider(provider),
		),
		level: level,
	}
	slog.SetDefault(slog.New(handler))

	return nil
}

func (l *Logger) Shutdown(ctx context.Context) error {
	if l.provider == nil {
		return nil
	}
	return l.provider.Shutdown(ctx)
}
