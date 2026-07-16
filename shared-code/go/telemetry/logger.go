package telemetry

import (
	"context"
	"log/slog"

	"go.opentelemetry.io/contrib/bridges/otelslog"
	"go.opentelemetry.io/otel/exporters/otlp/otlplog/otlploghttp"
	"go.opentelemetry.io/otel/sdk/log"
	"go.opentelemetry.io/otel/sdk/resource"
	semconv "go.opentelemetry.io/otel/semconv/v1.41.0"
)

type Logger struct {
	serviceName string
	logLevel    string
	provider    *log.LoggerProvider
}

func NewLogger(serviceName string, logLevel string) *Logger {
	return &Logger{
		serviceName: serviceName,
		logLevel:    logLevel,
	}
}

// Start bridges slog logs through an OTeL pipeline to an endpoint
func (self *Logger) Start() error {
	if self.provider != nil {
		return nil
	}

	// Convert a string log level into a valid type, error if invalid level
	var level slog.Level
	if err := level.UnmarshalText([]byte(self.logLevel)); err != nil {
		return err
	}

	// Create a type that exports logs to OTEL_EXPORTER_OTLP_ENDPOINT
	exporter, err := otlploghttp.New(context.Background())
	if err != nil {
		return err
	}

	// Export logs in batches instead of individually
	processor := log.NewBatchProcessor(exporter)

	// Create a type with identifiable information such as a name
	res, err := resource.Merge(
		resource.Default(),
		resource.NewWithAttributes(semconv.SchemaURL, semconv.ServiceName(self.serviceName)),
	)
	if err != nil {
		return err
	}

	// Handles batching and wiring to process logs before sending to the exporter.
	self.provider = log.NewLoggerProvider(log.WithResource(res), log.WithProcessor(processor))

	// Bridge slog -> otel and filter by our log level
	handler := newLevelHandler(self.serviceName, level, self.provider)
	slog.SetDefault(slog.New(handler))

	return nil
}

// Shutdown flushes remaining log records and stops the OTel export pipeline.
func (self *Logger) Shutdown(ctx context.Context) error {
	if self.provider == nil {
		return nil
	}
	return self.provider.Shutdown(ctx)
}

type levelHandler struct {
	slog.Handler
	level slog.Level
}

func (self levelHandler) Enabled(ctx context.Context, level slog.Level) bool {
	return level >= self.level && self.Handler.Enabled(ctx, level)
}

func newLevelHandler(serviceName string, logLevel slog.Level, provider *log.LoggerProvider) *levelHandler {
	return &levelHandler{
		Handler: otelslog.NewHandler(
			serviceName,
			otelslog.WithLoggerProvider(provider),
		),
		level: logLevel,
	}
}
